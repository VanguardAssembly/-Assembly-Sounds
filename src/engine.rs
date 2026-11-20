//! Real-time audio engine (cpal). Everything audible is synthesised here:
//! binaural / monaural / isochronic tones, noise beds, and the voice-cue mixer,
//! followed by a hard safety limiter.
use crate::plan::Timeline;
use crate::texture::TexSynth;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample};
use std::f64::consts::TAU;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode { Binaural, Monaural, Isochronic }
impl Mode {
    pub fn parse(s: &str) -> Mode {
        match s { "monaural" => Mode::Monaural, "isochronic" => Mode::Isochronic, _ => Mode::Binaural }
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Noise { None, Pink, Brown, White, Surf }
impl Noise {
    pub fn parse(s: &str) -> Noise {
        match s { "pink" => Noise::Pink, "brown" => Noise::Brown, "white" => Noise::White, "surf" => Noise::Surf, _ => Noise::None }
    }
}

pub const SCOPE_FRAMES: usize = 1024;
/// Absolute ceiling on output amplitude (≈ -1.4 dBFS) — hearing/equipment safety.
const CEILING: f32 = 0.85;

/// State shared between the UI thread and the audio callback.
pub struct Shared {
    // ---- UI → audio ----
    pub timeline: Option<Arc<Timeline>>,
    pub generation: u64,
    pub seek: Option<f64>,
    pub paused: bool,
    pub stop: bool,
    pub vol: f32,
    pub tone: f32,
    pub noise: f32,
    pub voice_vol: f32,
    pub tex: f32,
    pub noise_type: Noise,
    pub mode: Mode,
    pub live: Option<(f64, f64)>,
    pub voice: Option<(Arc<Vec<f32>>, u32)>,
    pub voice_cancel: bool,
    // ---- audio → UI ----
    pub ack_gen: u64,
    pub elapsed: f64,
    pub running: bool,
    pub scope: Vec<f32>,
}
impl Default for Shared {
    fn default() -> Self {
        Shared {
            timeline: None, generation: 0, seek: None, paused: false, stop: false,
            vol: 0.35, tone: 0.6, noise: 0.35, voice_vol: 0.8, tex: 0.7,
            noise_type: Noise::Pink, mode: Mode::Binaural, live: None,
            voice: None, voice_cancel: false,
            ack_gen: 0, elapsed: 0.0, running: false, scope: vec![0.0; SCOPE_FRAMES * 2],
        }
    }
}

#[derive(Default, Clone)]
struct Phase { l: f64, r: f64, c: f64, lfo: f64 }

struct Synth {
    sr: f64,
    tl: Option<Arc<Timeline>>,
    generation: u64,
    pos: f64,
    phases: Vec<Phase>,
    running: bool,
    stopping: bool,
    fade: f32,
    fade_target: f32,
    fade_step: f32,
    paused: bool,
    pause_g: f32,
    // targets + smoothed levels
    vol: f32, tone: f32, noise_lvl: f32, voice_vol: f32,
    s_vol: f32, s_tone: f32, s_noise: f32, duck: f32,
    noise_type: Noise,
    mode: Mode,
    live: Option<(f64, f64)>,
    rng: u64,
    pink: [[f32; 7]; 2],
    brown: [f32; 2],
    lp: [f32; 2],
    surf: f64,
    voice: Option<(Arc<Vec<f32>>, f64, f64)>,
    tex: Option<TexSynth>,
    tex_level: f32,
    s_tex: f32,
    scope: Vec<f32>,
    scope_i: usize,
}

impl Synth {
    fn new(sr: f64) -> Self {
        Synth {
            sr, tl: None, generation: 0, pos: 0.0, phases: vec![], running: false, stopping: false,
            fade: 0.0, fade_target: 0.0, fade_step: 0.0, paused: false, pause_g: 1.0,
            vol: 0.35, tone: 0.6, noise_lvl: 0.35, voice_vol: 0.8,
            s_vol: 0.35, s_tone: 0.6, s_noise: 0.35, duck: 1.0,
            noise_type: Noise::Pink, mode: Mode::Binaural, live: None,
            rng: 0x9E3779B97F4A7C15, pink: [[0.0; 7]; 2], brown: [0.0; 2], lp: [0.0; 2], surf: 0.0,
            voice: None, tex: None, tex_level: 0.7, s_tex: 0.0, scope: vec![0.0; SCOPE_FRAMES * 2], scope_i: 0,
        }
    }

    fn sync(&mut self, s: &mut Shared) {
        if s.generation != self.generation {
            self.generation = s.generation;
            self.tl = s.timeline.clone();
            self.pos = 0.0;
            self.phases = vec![Phase::default(); self.tl.as_ref().map(|t| t.layers()).unwrap_or(0)];
            self.running = self.tl.is_some();
            self.stopping = false;
            self.fade = 0.0;
            self.fade_target = 1.0;
            self.fade_step = 1.0 / (6.0 * self.sr as f32); // 6 s fade-in
            self.voice = None;
            self.pause_g = 1.0;
            self.tex = self.tl.as_ref().and_then(|t| t.texture.as_ref()).and_then(|tx| TexSynth::new(tx, self.sr));
            self.s_tex = 0.0;
        }
        if let Some(t) = s.seek.take() {
            self.pos = t.max(0.0);
        }
        if s.stop {
            s.stop = false;
            if self.running {
                self.stopping = true;
                self.fade_target = 0.0;
                self.fade_step = 1.0 / (2.0 * self.sr as f32); // 2 s fade-out
            }
        }
        self.paused = s.paused;
        self.vol = s.vol.clamp(0.0, 1.0);
        self.tone = s.tone.clamp(0.0, 1.0);
        self.noise_lvl = s.noise.clamp(0.0, 1.0);
        self.voice_vol = s.voice_vol.clamp(0.0, 1.0);
        self.tex_level = s.tex.clamp(0.0, 1.0);
        self.noise_type = s.noise_type;
        self.mode = s.mode;
        self.live = s.live;
        if s.voice_cancel {
            s.voice_cancel = false;
            self.voice = None;
        }
        if let Some((data, vsr)) = s.voice.take() {
            self.voice = Some((data, 0.0, vsr as f64 / self.sr));
        }
        // publish status
        s.ack_gen = self.generation;
        s.elapsed = self.pos;
        s.running = self.running;
        let n = SCOPE_FRAMES * 2;
        for k in 0..n {
            s.scope[k] = self.scope[(self.scope_i * 2 + k) % n];
        }
    }

    fn at(&self, tl: &Timeline, e: f64) -> (Vec<f64>, Vec<f64>) {
        let (mut b, mut c) = tl.at(e);
        if tl.live {
            if let Some((lc, lb)) = self.live {
                if !b.is_empty() { b[0] = lb; c[0] = lc; }
            }
        }
        (b, c)
    }

    fn rand(&mut self) -> f32 {
        let mut x = self.rng;
        x ^= x << 13; x ^= x >> 7; x ^= x << 17;
        self.rng = x;
        ((x >> 40) as f32 / (1u64 << 24) as f32) * 2.0 - 1.0
    }

    fn noise_sample(&mut self, ch: usize, surf_mod: f32) -> f32 {
        let w = self.rand();
        match self.noise_type {
            Noise::None => 0.0,
            Noise::White => w * 0.3,
            Noise::Brown => {
                self.brown[ch] = (self.brown[ch] + 0.02 * w) / 1.02;
                self.brown[ch] * 3.0
            }
            Noise::Pink | Noise::Surf => {
                let b = &mut self.pink[ch];
                b[0] = 0.99886 * b[0] + w * 0.0555179;
                b[1] = 0.99332 * b[1] + w * 0.0750759;
                b[2] = 0.96900 * b[2] + w * 0.1538520;
                b[3] = 0.86650 * b[3] + w * 0.3104856;
                b[4] = 0.55000 * b[4] + w * 0.5329522;
                b[5] = -0.7616 * b[5] - w * 0.0168980;
                let v = (b[0] + b[1] + b[2] + b[3] + b[4] + b[5] + b[6] + w * 0.5362) * 0.1;
                b[6] = w * 0.115926;
                if self.noise_type == Noise::Surf {
                    // slow "wave" swell: moving low-pass + amplitude
                    let fc = 900.0 + 650.0 * surf_mod;
                    let a = 1.0 - (-(TAU as f32) * fc / self.sr as f32).exp();
                    self.lp[ch] += a * (v - self.lp[ch]);
                    self.lp[ch] * 1.6 * (0.55 + 0.45 * surf_mod)
                } else {
                    v
                }
            }
        }
    }

    /// Fill an interleaved f32 stereo buffer (frames × 2).
    fn render(&mut self, out: &mut [f32]) {
        let frames = out.len() / 2;
        let dt = 1.0 / self.sr;
        let advance = if self.paused || !self.running { 0.0 } else { frames as f64 * dt };
        let tl = self.tl.clone();
        let (b0, c0, b1, c1) = match &tl {
            Some(t) => {
                let (b0, c0) = self.at(t, self.pos);
                let (b1, c1) = self.at(t, self.pos + advance);
                (b0, c0, b1, c1)
            }
            None => (vec![], vec![], vec![], vec![]),
        };
        let gains: Vec<f32> = tl.as_ref().map(|t| t.gains.iter().map(|g| *g as f32).collect()).unwrap_or_default();
        let gsum: f32 = gains.iter().sum::<f32>().max(1.0);
        let tone_norm = 0.5 / gsum.sqrt();
        let k_smooth = 1.0 - (-1.0f32 / (0.05 * self.sr as f32)).exp(); // 50 ms parameter smoothing
        let pause_step = 1.0 / (0.08 * self.sr as f32);
        let tanh4 = 4f32.tanh();
        let tex_target = tl.as_ref().map(|t| t.tex_at(self.pos) as f32).unwrap_or(0.0) * self.tex_level;

        for f in 0..frames {
            let t = f as f64 / frames.max(1) as f64;
            self.s_vol += (self.vol - self.s_vol) * k_smooth;
            self.s_tone += (self.tone - self.s_tone) * k_smooth;
            self.s_noise += (self.noise_lvl - self.s_noise) * k_smooth;
            let duck_t = if self.voice.is_some() { 0.6 } else { 1.0 };
            self.duck += (duck_t - self.duck) * k_smooth * 0.2;

            let (mut l, mut r) = (0.0f32, 0.0f32);
            if self.running {
                for i in 0..self.phases.len() {
                    let b = b0[i] + (b1[i] - b0[i]) * t;
                    let c = c0[i] + (c1[i] - c0[i]) * t;
                    let g = gains.get(i).copied().unwrap_or(0.0);
                    let ph = &mut self.phases[i];
                    match self.mode {
                        Mode::Binaural | Mode::Monaural => {
                            ph.l = (ph.l + TAU * (c - b / 2.0) * dt) % TAU;
                            ph.r = (ph.r + TAU * (c + b / 2.0) * dt) % TAU;
                            let (sl, sr) = (ph.l.sin() as f32, ph.r.sin() as f32);
                            if self.mode == Mode::Binaural {
                                l += sl * g;
                                r += sr * g;
                            } else {
                                let m = (sl + sr) * 0.5 * g;
                                l += m;
                                r += m;
                            }
                        }
                        Mode::Isochronic => {
                            ph.c = (ph.c + TAU * c * dt) % TAU;
                            ph.lfo = (ph.lfo + TAU * b * dt) % TAU;
                            let env = 0.5 + 0.5 * (4.0f32 * ph.lfo.sin() as f32).tanh() / tanh4;
                            let m = ph.c.sin() as f32 * env * g;
                            l += m;
                            r += m;
                        }
                    }
                }
                l *= tone_norm;
                r *= tone_norm;
            }
            self.scope[self.scope_i * 2] = l;
            self.scope[self.scope_i * 2 + 1] = r;
            self.scope_i = (self.scope_i + 1) % SCOPE_FRAMES;

            // noise bed
            let (mut nl, mut nr) = (0.0, 0.0);
            if self.running && self.noise_type != Noise::None {
                self.surf = (self.surf + TAU * 0.09 * dt) % TAU;
                let sm = self.surf.sin() as f32;
                nl = self.noise_sample(0, sm);
                nr = self.noise_sample(1, sm);
            }

            // texture layer (drum / bowls / chimes / pulses / rain)
            let (mut xl, mut xr) = (0.0f32, 0.0f32);
            if self.running {
                self.s_tex += (tex_target - self.s_tex) * k_smooth * 0.1;
                let b = b0.first().map(|x| x + (b1[0] - x) * t).unwrap_or(1.0);
                let g = self.s_tex;
                if let Some(tx) = self.tex.as_mut() {
                    (xl, xr) = tx.sample(b, g);
                }
            }

            // fades
            if self.fade < self.fade_target {
                self.fade = (self.fade + self.fade_step).min(self.fade_target);
            } else if self.fade > self.fade_target {
                self.fade = (self.fade - self.fade_step).max(self.fade_target);
            }
            let pt = if self.paused { 0.0 } else { 1.0 };
            if self.pause_g < pt { self.pause_g = (self.pause_g + pause_step).min(1.0) } else if self.pause_g > pt { self.pause_g = (self.pause_g - pause_step).max(0.0) }

            let bed = self.fade * self.pause_g * self.duck;
            let mut ol = (l * self.s_tone + nl * self.s_noise * 0.6 + xl * 0.5) * bed;
            let mut or = (r * self.s_tone + nr * self.s_noise * 0.6 + xr * 0.5) * bed;

            // voice cue (mono, centred) — advances only while playing
            if !self.paused {
                if let Some((data, pos, step)) = &mut self.voice {
                    let i = *pos as usize;
                    if i + 1 < data.len() {
                        let fr = (*pos - i as f64) as f32;
                        let v = (data[i] * (1.0 - fr) + data[i + 1] * fr) * self.voice_vol * self.pause_g;
                        ol += v;
                        or += v;
                        *pos += *step;
                    } else {
                        self.voice = None;
                    }
                }
            }

            out[f * 2] = limit(ol * self.s_vol);
            out[f * 2 + 1] = limit(or * self.s_vol);
        }

        if self.running {
            self.pos += advance;
            if self.pos >= self.tl.as_ref().map(|t| t.total).unwrap_or(0.0) && !self.stopping {
                self.stopping = true;
                self.fade_target = 0.0;
                self.fade_step = 1.0 / (2.0 * self.sr as f32);
            }
            if self.stopping && self.fade <= 0.0 {
                self.running = false;
                self.stopping = false;
                self.tl = None;
                self.phases.clear();
                self.voice = None;
                self.tex = None;
            }
        }
    }
}

/// Soft-knee limiter with a hard ceiling — output can never exceed CEILING.
#[inline]
fn limit(x: f32) -> f32 {
    let knee = 0.6;
    let a = x.abs();
    let y = if a <= knee { a } else { knee + (CEILING - knee) * ((a - knee) / (CEILING - knee)).tanh() };
    (y.min(CEILING)).copysign(x)
}

/// Offline render (self-test / verification): returns interleaved stereo at `sr`.
pub fn render_offline(tl: Arc<Timeline>, mode: Mode, noise: Noise, start: f64, secs: f64, sr: u32) -> Vec<f32> {
    let mut s = Shared { timeline: Some(tl), generation: 1, mode, noise_type: noise, seek: Some(start), ..Default::default() };
    let mut synth = Synth::new(sr as f64);
    let mut out = vec![0.0f32; (secs * sr as f64) as usize * 2];
    for chunk in out.chunks_mut(1024) {
        synth.sync(&mut s);
        synth.render(chunk);
    }
    out
}

pub struct Engine {
    _stream: cpal::Stream,
}

pub fn start(shared: Arc<Mutex<Shared>>) -> Result<Engine, String> {
    let host = cpal::default_host();
    let dev = host.default_output_device().ok_or("No audio output device found")?;
    let sup = dev.default_output_config().map_err(|e| format!("{e:?}"))?;
    let fmt = sup.sample_format();
    let cfg = sup.config();
    let stream = match fmt {
        cpal::SampleFormat::F32 => build::<f32>(&dev, cfg, shared),
        cpal::SampleFormat::I16 => build::<i16>(&dev, cfg, shared),
        cpal::SampleFormat::U16 => build::<u16>(&dev, cfg, shared),
        cpal::SampleFormat::I32 => build::<i32>(&dev, cfg, shared),
        other => Err(format!("Unsupported sample format {other:?}")),
    }?;
    stream.play().map_err(|e| format!("{e:?}"))?;
    Ok(Engine { _stream: stream })
}

fn build<T>(dev: &cpal::Device, cfg: cpal::StreamConfig, shared: Arc<Mutex<Shared>>) -> Result<cpal::Stream, String>
where
    T: SizedSample + FromSample<f32>,
{
    let ch = cfg.channels as usize;
    let mut synth = Synth::new(cfg.sample_rate as f64);
    let mut buf: Vec<f32> = Vec::new();
    dev.build_output_stream::<T, _, _>(
        cfg,
        move |data: &mut [T], _| {
            if let Ok(mut s) = shared.try_lock() {
                synth.sync(&mut s);
            }
            let frames = data.len() / ch.max(1);
            buf.resize(frames * 2, 0.0);
            synth.render(&mut buf);
            for f in 0..frames {
                let (l, r) = (buf[f * 2], buf[f * 2 + 1]);
                for c in 0..ch {
                    let v = match (ch, c) { (1, _) => (l + r) * 0.5, (_, 0) => l, (_, 1) => r, _ => 0.0 };
                    data[f * ch + c] = T::from_sample(v);
                }
            }
        },
        |_e| {},
        None,
    )
    .map_err(|e| format!("{e:?}"))
}
