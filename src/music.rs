//! Accompaniment ("music" layer) for sessions that come from a living practice, so each one can
//! sound the way its practitioners hear it. Everything is synthesised live, except `chant`, which
//! plays group chants rendered at build time (chant/*.ogg, see tools/gen_chants.py).
//!   tanpura     – four-string drone Pa-Sa-Sa-Sa with the jawari's rising shimmer (Indian classical, kirtan)
//!   drone       – shruti box, harmonium, pipe organ or a rubbed singing bowl held under a chant
//!   choir       – men's voices holding a drone: Byzantine ison, organum (root, fifth, octave), hum, "ah"
//!   om          – a group chanting A-U-M, one long breath per cycle
//!   gyuto       – deep Tibetan chant with the overtone "chord" (subharmonic growl, 5th or 10th harmonic)
//!   flute       – slow improvised melody: ney (maqam), shakuhachi (honkyoku), bansuri (raga alap), xiao
//!   guqin, harp – plucked strings: the Chinese seven-string zither, the Celtic harp
//!   daf, damaru – Sufi frame drum with jingles; Tibetan hand drum
//!   rin         – temple or chapel bell marking the stages: three strikes to begin, one for each new
//!                 stage, two at the start of the last
//!   gongbath    – soft rolling gong strokes in slow waves
//!   stream, wind, fire – running water, wind in trees, a crackling fire
//!   chant       – a pre-rendered group chant of a mantra or prayer, looped (one clip, or one per stage)
//! Layers: a name with '|' adds more instruments under the first, each "kind[:variant][*gain]" and
//! sharing the root and interval, e.g. chant "mahamantra|drone:harmonium*0.7" or flute "ney|daf".
use std::f64::consts::TAU;
use std::sync::Arc;

const PI32: f32 = std::f32::consts::PI;
const TAU32: f32 = std::f32::consts::TAU;

/// A decoded chant clip: (name, interleaved stereo samples, sample rate).
pub type Clip = (String, Arc<Vec<f32>>, u32);

// ======================================================================= small DSP parts

#[derive(Clone)]
pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Self { Rng(seed | 1) }
    #[inline]
    pub fn bi(&mut self) -> f32 {
        let mut x = self.0;
        x ^= x << 13; x ^= x >> 7; x ^= x << 17;
        self.0 = x;
        ((x >> 40) as f32 / (1u64 << 24) as f32) * 2.0 - 1.0
    }
    #[inline] pub fn uni(&mut self) -> f32 { self.bi() * 0.5 + 0.5 }
    pub fn range(&mut self, a: f64, b: f64) -> f64 { a + (b - a) * self.uni() as f64 }
    pub fn pick(&mut self, n: usize) -> usize { ((self.uni() * n as f32) as usize).min(n.max(1) - 1) }
    pub fn chance(&mut self, p: f32) -> bool { self.uni() < p }
}

/// Sine oscillator by rotation (cheap: no sin() per sample). Call fix() now and then.
#[derive(Clone, Copy)]
struct Osc { c: f64, s: f64, cw: f64, sw: f64 }
impl Osc {
    fn new(f: f64, sr: f64, phase: f64) -> Self {
        let mut o = Osc { c: phase.cos(), s: phase.sin(), cw: 1.0, sw: 0.0 };
        o.set(f, sr);
        o
    }
    fn set(&mut self, f: f64, sr: f64) { let w = TAU * f / sr; self.cw = w.cos(); self.sw = w.sin(); }
    #[inline]
    fn step(&mut self) -> f32 {
        let c = self.c * self.cw - self.s * self.sw;
        self.s = self.s * self.cw + self.c * self.sw;
        self.c = c;
        self.s as f32
    }
    fn fix(&mut self) {
        let m = (self.c * self.c + self.s * self.s).sqrt();
        if m > 0.0 { self.c /= m; self.s /= m; }
    }
}

/// Two-pole resonator with unity gain at 0 Hz (Klatt): cascaded, they make vowel formants.
#[derive(Clone, Copy, Default)]
struct Reson { a: f32, b: f32, c: f32, y1: f32, y2: f32 }
impl Reson {
    fn set(&mut self, f: f32, bw: f32, sr: f32) {
        let t = 1.0 / sr;
        let c = -(-TAU32 * bw * t).exp();
        let b = 2.0 * (-PI32 * bw * t).exp() * (TAU32 * f.min(sr * 0.45) * t).cos();
        self.a = 1.0 - b - c;
        self.b = b;
        self.c = c;
    }
    #[inline]
    fn run(&mut self, x: f32) -> f32 {
        let y = self.a * x + self.b * self.y1 + self.c * self.y2;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }
}

/// Band-pass biquad (0 dB at the centre).
#[derive(Clone, Copy, Default)]
struct Bp { b0: f32, a1: f32, a2: f32, x1: f32, x2: f32, y1: f32, y2: f32 }
impl Bp {
    fn new(f: f32, q: f32, sr: f32) -> Self { let mut b = Bp::default(); b.set(f, q, sr); b }
    fn set(&mut self, f: f32, q: f32, sr: f32) {
        let w = TAU32 * (f / sr).clamp(1e-4, 0.49);
        let al = w.sin() / (2.0 * q.max(0.1));
        let a0 = 1.0 + al;
        self.b0 = al / a0;
        self.a1 = -2.0 * w.cos() / a0;
        self.a2 = (1.0 - al) / a0;
    }
    #[inline]
    fn run(&mut self, x: f32) -> f32 {
        let y = self.b0 * (x - self.x2) - self.a1 * self.y1 - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }
}

/// One-pole low-pass coefficient for a corner frequency.
fn lp_coef(fc: f64, sr: f64) -> f32 { (1.0 - (-TAU * fc / sr).exp()) as f32 }
/// Per-sample smoothing coefficient for a time constant in seconds.
fn tc(secs: f64, sr: f64) -> f32 { (1.0 - (-1.0 / (secs.max(1e-4) * sr)).exp()) as f32 }

#[inline]
fn polyblep(t: f64, dt: f64) -> f64 {
    if t < dt { let x = t / dt; x + x - x * x - 1.0 }
    else if t > 1.0 - dt { let x = (t - 1.0) / dt; x * x + x + x + 1.0 }
    else { 0.0 }
}

#[inline]
fn pan2(v: f32, pan: f32) -> (f32, f32) { (v * (1.0 - pan).min(1.0), v * (1.0 + pan).min(1.0)) }

/// A bank of decaying sine partials (bells, gongs, harmonics, bubbles).
struct Part { o: Osc, amp: f32, dec: f32, att: f32, env: f32, pl: f32, pr: f32 }
struct Bank { p: Vec<Part>, cap: usize, sr: f64, n: u32 }
impl Bank {
    fn new(sr: f64, cap: usize) -> Self { Bank { p: Vec::with_capacity(cap + 4), cap, sr, n: 0 } }
    /// `t60`: seconds to fall by 60 dB; `att`: attack time in seconds.
    fn add(&mut self, f: f64, amp: f32, t60: f64, att: f64, pan: f32) {
        if !(f > 0.0 && f < self.sr * 0.45) || amp <= 0.0 { return; }
        if self.p.len() >= self.cap {
            let mut k = 0;
            let mut lo = f32::MAX;
            for (i, q) in self.p.iter().enumerate() { if q.amp < lo { lo = q.amp; k = i; } }
            self.p.swap_remove(k);
        }
        let dec = (-6.9078 / (t60.max(0.005) * self.sr)).exp() as f32;
        let att = (1.0 / (att.max(0.0005) * self.sr)) as f32;
        let pan = pan.clamp(-1.0, 1.0);
        self.p.push(Part { o: Osc::new(f, self.sr, 0.0), amp, dec, att, env: 0.0, pl: (1.0 - pan).min(1.0), pr: (1.0 + pan).min(1.0) });
    }
    #[inline]
    fn run(&mut self) -> (f32, f32) {
        let (mut l, mut r) = (0.0f32, 0.0f32);
        for q in self.p.iter_mut() {
            if q.env < 1.0 { q.env = (q.env + q.att).min(1.0); }
            let v = q.o.step() * q.amp * q.env;
            l += v * q.pl;
            r += v * q.pr;
            q.amp *= q.dec;
        }
        self.n = self.n.wrapping_add(1);
        if self.n & 1023 == 0 {
            self.p.retain(|q| q.amp > 2e-5);
            for q in self.p.iter_mut() { q.o.fix(); }
        }
        (l, r)
    }
}

/// Soft-knee peak limiter: untouched below `knee`, rounded off so it never passes `ceil`.
#[inline]
fn soft(x: f32, knee: f32, ceil: f32) -> f32 {
    let a = x.abs();
    if a <= knee { return x; }
    let w = ceil - knee;
    (knee + w * ((a - knee) / w).tanh()).copysign(x)
}

/// Small stereo room / hall (four combs and two all-passes per side, after Freeverb).
struct Comb { buf: Vec<f32>, i: usize, fb: f32, damp: f32, st: f32 }
struct Allp { buf: Vec<f32>, i: usize }
pub struct Reverb { c: [Vec<Comb>; 2], a: [Vec<Allp>; 2] }
impl Reverb {
    fn new(sr: f64, size: f32) -> Self {
        let k = sr / 44100.0;
        let mk = |n: usize| ((n as f64 * k) as usize).max(16);
        const CL: [usize; 4] = [1116, 1188, 1277, 1356];
        const AL: [usize; 2] = [556, 441];
        let fb = 0.7 + 0.27 * size.clamp(0.0, 1.0);
        let side = |sp: usize| -> (Vec<Comb>, Vec<Allp>) {
            (CL.iter().map(|&n| Comb { buf: vec![0.0; mk(n + sp)], i: 0, fb, damp: 0.3, st: 0.0 }).collect(),
             AL.iter().map(|&n| Allp { buf: vec![0.0; mk(n + sp)], i: 0 }).collect())
        };
        let (c0, a0) = side(0);
        let (c1, a1) = side(23);
        Reverb { c: [c0, c1], a: [a0, a1] }
    }
    #[inline]
    fn run(&mut self, l: f32, r: f32) -> (f32, f32) {
        let x = (l + r) * 0.1;
        let mut out = [0.0f32; 2];
        for ch in 0..2 {
            let mut s = 0.0;
            for c in self.c[ch].iter_mut() {
                let y = c.buf[c.i];
                c.st = y * (1.0 - c.damp) + c.st * c.damp;
                c.buf[c.i] = x + c.st * c.fb;
                c.i += 1;
                if c.i == c.buf.len() { c.i = 0; }
                s += y;
            }
            for a in self.a[ch].iter_mut() {
                let b = a.buf[a.i];
                let y = b - s;
                a.buf[a.i] = s + b * 0.5;
                a.i += 1;
                if a.i == a.buf.len() { a.i = 0; }
                s = y;
            }
            out[ch] = s;
        }
        (out[0], out[1])
    }
}

/// Note `deg` of a scale (cents per degree, octave repeats) above `root`.
fn note(root: f64, scale: &[f64], deg: i32) -> f64 {
    let n = scale.len() as i32;
    let oct = deg.div_euclid(n);
    let i = deg.rem_euclid(n) as usize;
    root * 2f64.powf((scale[i] + 1200.0 * oct as f64) / 1200.0)
}

// ======================================================================= the instruments

pub enum Inst {
    Tanpura(Tanpura),
    Drone(Drone),
    Choir(Choir),
    Om(OmChant),
    Gyuto(Gyuto),
    Flute(Flute),
    Strings(Strings),
    Daf(Daf),
    Damaru(Damaru),
    Rin(Rin),
    Gong(GongBath),
    Stream(Stream),
    Wind(Wind),
    Fire(Fire),
    Chant(Chant),
    Layer(Vec<(Inst, f32)>),
}

impl Inst {
    /// `kind` and `name` come from a preset's texture/music entry; unknown kinds give None.
    pub fn new(kind: &str, name: &str, root: f64, interval: f64, sr: f64, clips: &[Clip]) -> Option<Inst> {
        if !name.contains('|') { return Inst::one(kind, name, root, interval, sr, clips); }
        let mut parts = name.split('|');
        let mut v = vec![(Inst::one(kind, parts.next().unwrap_or("").trim(), root, interval, sr, clips)?, 1.0f32)];
        for part in parts {
            let (spec, g) = match part.split_once('*') {
                Some((s, g)) => (s, g.trim().parse::<f32>().unwrap_or(1.0).clamp(0.0, 2.0)),
                None => (part, 1.0),
            };
            let (k, n) = spec.split_once(':').unwrap_or((spec, ""));
            if let Some(x) = Inst::one(k.trim(), n.trim(), root, interval, sr, clips) { v.push((x, g)); }
        }
        Some(Inst::Layer(v))
    }

    fn one(kind: &str, name: &str, root: f64, interval: f64, sr: f64, clips: &[Clip]) -> Option<Inst> {
        Some(match kind {
            "tanpura" => Inst::Tanpura(Tanpura::new(root, name, interval, sr)),
            "drone" => Inst::Drone(Drone::new(root, name, sr)),
            "choir" => Inst::Choir(Choir::new(root, name, sr)),
            "om" => Inst::Om(OmChant::new(root, interval, sr)),
            "gyuto" => Inst::Gyuto(Gyuto::new(root, name, sr)),
            "flute" => Inst::Flute(Flute::new(root, name, sr)),
            "guqin" => Inst::Strings(Strings::new(true, root, sr)),
            "harp" => Inst::Strings(Strings::new(false, root, sr)),
            "daf" => Inst::Daf(Daf::new(interval, sr)),
            "damaru" => Inst::Damaru(Damaru::new(interval, sr)),
            "rin" => Inst::Rin(Rin::new(root, name, sr)),
            "gongbath" => Inst::Gong(GongBath::new(root, sr)),
            "stream" => Inst::Stream(Stream::new(sr)),
            "wind" => Inst::Wind(Wind::new(sr)),
            "fire" => Inst::Fire(Fire::new(sr)),
            "chant" => Inst::Chant(Chant::new(name, sr, clips)),
            _ => return None,
        })
    }

    /// Stage index of the session (some instruments follow the stages).
    pub fn set_stage(&mut self, k: usize, n: usize) {
        match self {
            Inst::Rin(x) => x.set_stage(k, n),
            Inst::Chant(x) => x.set_stage(k),
            Inst::Layer(v) => for (x, _) in v.iter_mut() { x.set_stage(k, n) },
            _ => {}
        }
    }

    /// One stereo sample at `gain` (0..1).
    #[inline]
    pub fn sample(&mut self, gain: f32) -> (f32, f32) {
        let (l, r) = match self {
            Inst::Tanpura(x) => x.sample(),
            Inst::Drone(x) => x.sample(),
            Inst::Choir(x) => x.sample(),
            Inst::Om(x) => x.sample(),
            Inst::Gyuto(x) => x.sample(),
            Inst::Flute(x) => x.sample(),
            Inst::Strings(x) => x.sample(),
            Inst::Daf(x) => x.sample(),
            Inst::Damaru(x) => x.sample(),
            Inst::Rin(x) => x.sample(gain),
            Inst::Gong(x) => x.sample(),
            Inst::Stream(x) => x.sample(),
            Inst::Wind(x) => x.sample(),
            Inst::Fire(x) => x.sample(),
            Inst::Chant(x) => x.sample(),
            Inst::Layer(v) => {
                let (mut l, mut r) = (0.0f32, 0.0f32);
                for (x, g) in v.iter_mut() { let (a, b) = x.sample(gain); l += a * *g; r += b * *g; }
                return (l, r);
            }
        };
        // the bell takes the gain at the moment of each strike, so a strike rings out across stages
        if let Inst::Rin(_) = self { (l, r) } else { (l * gain, r * gain) }
    }

    /// Names of the chant clips a kind/name pair needs (for loading before playback).
    pub fn clip_names(kind: &str, name: &str) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        for (i, part) in name.split('|').enumerate() {
            let (k, spec) = if i == 0 { (kind, part) } else {
                let s = part.split('*').next().unwrap_or("");
                s.split_once(':').unwrap_or((s, ""))
            };
            if k.trim() != "chant" { continue; }
            for c in spec.split(',') {
                let n = c.split('@').next().unwrap_or("").trim();
                if !n.is_empty() && !v.iter().any(|x| x == n) { v.push(n.to_string()); }
            }
        }
        v
    }
}

// ---------------------------------------------------------------- tanpura

const TH: usize = 18;
struct TStr { h: Vec<Osc>, a: [f32; TH], da: [f32; TH], t: f64, live: bool, pan: f32 }
pub struct Tanpura { sr: f64, s: Vec<TStr>, next: usize, clock: f64, wait: f64, gap: f64, k: u32, verb: Reverb }
impl Tanpura {
    fn new(root: f64, name: &str, interval: f64, sr: f64) -> Self {
        let root = if root > 0.0 { root } else { 138.59 };
        // first string: Pa (fifth) below Sa by default; Ma or Ni for ragas without Pa
        let first = match name { "ma" => 2.0 / 3.0, "ni" => 15.0 / 16.0, _ => 0.75 };
        let tun = [first, 1.0, 1.0007, 0.5];
        let pan = [-0.3f32, 0.12, 0.3, -0.12];
        let s = (0..4).map(|i| {
            let f = root * tun[i];
            let h = (1..=TH).map(|n| { let n = n as f64; Osc::new(f * n * (1.0 + 1.0e-5 * n * n), sr, 0.37 * n) }).collect();
            TStr { h, a: [0.0; TH], da: [0.0; TH], t: 0.0, live: false, pan: pan[i] }
        }).collect();
        let gap = if interval > 0.0 { interval.clamp(0.6, 4.0) } else { 1.3 };
        Tanpura { sr, s, next: 0, clock: 1e9, wait: 0.0, gap, k: 0, verb: Reverb::new(sr, 0.5) }
    }
    /// Harmonic n (1-based) at t s after the pluck: the jawari lets each higher harmonic bloom a
    /// little later than the one below, so the brightness climbs through every note.
    fn amp(n: usize, t: f64) -> f32 {
        let nf = n as f64;
        let tn = 0.05 + 0.085 * (nf - 1.0);
        let sn = 0.3 + 0.04 * nf;
        let tau = 5.0 / (1.0 + 0.05 * nf);
        let bump = (-((t - tn) / sn).powi(2)).exp();
        ((1.0 / nf.powf(0.75)) * (-t / tau).exp() * (0.22 + 0.78 * bump)) as f32
    }
    fn sample(&mut self) -> (f32, f32) {
        let dt = 1.0 / self.sr;
        self.clock += dt;
        if self.clock >= self.wait {
            let i = self.next;
            self.s[i].t = 0.0;
            self.s[i].live = true;
            self.wait = self.gap * if i == 3 { 1.6 } else { 1.0 };
            self.next = (i + 1) % 4;
            self.clock = 0.0;
        }
        if self.k % 32 == 0 {
            let fix = self.k % 8192 == 0;
            for st in self.s.iter_mut() {
                if fix { for o in st.h.iter_mut() { o.fix(); } }
                if !st.live { continue; }
                let t1 = st.t + 32.0 * dt;
                for n in 0..TH { st.da[n] = (Self::amp(n + 1, t1) - st.a[n]) / 32.0; }
                if st.t > 14.0 { st.live = false; st.a = [0.0; TH]; st.da = [0.0; TH]; }
            }
        }
        self.k = self.k.wrapping_add(1);
        let (mut l, mut r) = (0.0f32, 0.0f32);
        for st in self.s.iter_mut() {
            if !st.live { continue; }
            let mut v = 0.0f32;
            for n in 0..TH { st.a[n] += st.da[n]; v += st.h[n].step() * st.a[n]; }
            st.t += dt;
            let (a, b) = pan2(v, st.pan);
            l += a;
            r += b;
        }
        let (wl, wr) = self.verb.run(l, r);
        ((l + wl * 0.3) * 0.25, (r + wr * 0.3) * 0.25)
    }
}

// ---------------------------------------------------------------- reed / organ / bowl drones

pub struct Drone { sr: f64, t: Vec<(Osc, f32, f32)>, lfo: f64, rate: f64, depth: f32, lp: [f32; 2], lpa: f32, verb: Reverb, wet: f32, norm: f32, k: u32, out: f32 }
impl Drone {
    fn new(root: f64, name: &str, sr: f64) -> Self {
        let mut t: Vec<(Osc, f32, f32)> = Vec::new();
        let mut add = |f: f64, a: f32, pan: f32, ph: f64| t.push((Osc::new(f, sr, ph), a, pan));
        let (rate, depth, lpf, size, wet, out) = match name {
            "organ" => {
                let root = if root > 0.0 { root } else { 146.83 };
                for (i, (m, pan)) in [(1.0, -0.3f32), (1.5, 0.3), (2.0, 0.0)].iter().enumerate() {
                    for (h, a) in [(1.0, 1.0f32), (2.0, 0.45), (3.0, 0.2), (4.0, 0.12), (6.0, 0.05), (8.0, 0.03)] {
                        add(root * m * h, a * if i == 2 { 0.6 } else { 1.0 }, *pan, 0.7 * h + i as f64);
                    }
                }
                (0.0, 0.0, 4200.0, 0.86, 0.55, 0.17)
            }
            "bowl" => {
                let root = if root > 0.0 { root } else { 220.0 };
                for (m, a) in [(1.0, 1.0f32), (2.71, 0.35), (5.33, 0.08)] {
                    add(root * m - 0.6, a * 0.5, -0.35, 0.0);
                    add(root * m + 0.6, a * 0.5, 0.35, 1.3);
                }
                (0.17, 0.35, 9000.0, 0.6, 0.35, 0.29)
            }
            "harmonium" => {
                let root = if root > 0.0 { root } else { 138.59 };
                for (i, (m, pan)) in [(1.0, -0.25f32), (1.5, 0.25), (2.0, 0.0)].iter().enumerate() {
                    for reed in 0..2 {
                        let det = if reed == 1 { 1.0 + 1.1 / (root * m) } else { 1.0 };
                        for h in 1..=12 {
                            let a = (1.0 / (h as f32).powf(0.95)) * if i == 2 { 0.55 } else { 1.0 } * 0.5;
                            add(root * m * h as f64 * det, a, *pan, 0.9 * h as f64 + reed as f64);
                        }
                    }
                }
                (0.09, 0.08, 2600.0, 0.45, 0.25, 0.2)
            }
            _ => { // shruti box
                let root = if root > 0.0 { root } else { 138.59 };
                for (i, (m, pan)) in [(1.0, -0.2f32), (1.5, 0.2), (2.0, 0.0)].iter().enumerate() {
                    for h in 1..=10 {
                        let a = (1.0 / (h as f32).powf(1.15)) * if i == 2 { 0.5 } else { 1.0 };
                        add(root * m * h as f64, a, *pan, 1.1 * h as f64 + i as f64);
                    }
                }
                (0.11, 0.14, 1500.0, 0.5, 0.25, 0.18)
            }
        };
        let norm = 1.0 / t.iter().map(|x| x.1 * x.1).sum::<f32>().sqrt().max(0.1);
        Drone { sr, t, lfo: 0.0, rate, depth, lp: [0.0; 2], lpa: lp_coef(lpf, sr), verb: Reverb::new(sr, size), wet, norm, k: 0, out }
    }
    fn sample(&mut self) -> (f32, f32) {
        self.lfo = (self.lfo + TAU * self.rate / self.sr) % TAU;
        let sw = 1.0 + self.depth * self.lfo.sin() as f32;
        let (mut l, mut r) = (0.0f32, 0.0f32);
        for (o, a, pan) in self.t.iter_mut() {
            let (x, y) = pan2(o.step() * *a, *pan);
            l += x;
            r += y;
        }
        self.k = self.k.wrapping_add(1);
        if self.k % 8192 == 0 { for x in self.t.iter_mut() { x.0.fix(); } }
        self.lp[0] += self.lpa * (l - self.lp[0]);
        self.lp[1] += self.lpa * (r - self.lp[1]);
        let (l, r) = (self.lp[0] * sw * self.norm, self.lp[1] * sw * self.norm);
        let (wl, wr) = self.verb.run(l, r);
        ((l + wl * self.wet) * self.out, (r + wr * self.wet) * self.out)
    }
}

// ---------------------------------------------------------------- singing voices (formant synthesis)

#[derive(Clone, Copy)]
struct Vowel { f: [f32; 5], b: [f32; 5], nas: f32 }
fn vowel(c: char) -> Vowel {
    let (f, b, nas) = match c {
        'a' => ([730.0, 1090.0, 2440.0, 3400.0, 4000.0], [90.0, 110.0, 160.0, 250.0, 300.0], 0.0),
        'o' => ([570.0, 840.0, 2410.0, 3400.0, 4000.0], [80.0, 100.0, 150.0, 250.0, 300.0], 0.0),
        'O' => ([480.0, 860.0, 2450.0, 3400.0, 4000.0], [80.0, 100.0, 150.0, 250.0, 300.0], 0.0),
        'u' => ([300.0, 870.0, 2240.0, 3400.0, 4000.0], [70.0, 100.0, 150.0, 250.0, 300.0], 0.0),
        'i' => ([270.0, 2290.0, 3010.0, 3500.0, 4200.0], [60.0, 100.0, 150.0, 250.0, 300.0], 0.0),
        'e' => ([530.0, 1840.0, 2480.0, 3500.0, 4100.0], [80.0, 100.0, 150.0, 250.0, 300.0], 0.0),
        'n' => ([260.0, 1500.0, 2400.0, 3400.0, 4000.0], [80.0, 250.0, 400.0, 300.0, 300.0], 0.9),
        _ => ([250.0, 1200.0, 2300.0, 3400.0, 4000.0], [80.0, 250.0, 400.0, 300.0, 300.0], 1.0), // 'm'
    };
    Vowel { f, b, nas }
}

struct Singer { ph: f64, f: f64, mul: f64, drift: f64, env: f32, tgt: f32, atk: f32, rel: f32, amp: f32, sub: bool, odd: bool }

/// A group of singers sharing one vocal-tract filter (they sing the same vowel together).
struct Voices {
    sr: f64, s: Vec<Singer>, fr: [Reson; 5], cf: [f32; 5], cb: [f32; 5], tf: [f32; 5], tb: [f32; 5],
    nas: f32, tnas: f32, nlp: f32, nlpa: f32, tlp: f32, tlpa: f32, breath: f32,
    vib: f64, vrate: f64, vdepth: f64, glide: f32, rng: Rng, k: u32, gain: f32, norm: f32,
}
impl Voices {
    fn new(sr: f64, f0s: &[f64], seed: u64, tilt: f64, breath: f32, vrate: f64, vdepth: f64) -> Self {
        let mut rng = Rng::new(seed);
        let s = f0s.iter().map(|&f| Singer {
            ph: rng.uni() as f64, f, mul: 1.0, drift: 1.0 + 0.002 * rng.bi() as f64, env: 0.0, tgt: 0.0,
            atk: tc(0.3, sr), rel: tc(0.45, sr), amp: 1.0, sub: false, odd: false,
        }).collect();
        let v = vowel('a');
        let mut g = Voices {
            sr, s, fr: [Reson::default(); 5], cf: v.f, cb: v.b, tf: v.f, tb: v.b, nas: 0.0, tnas: 0.0,
            nlp: 0.0, nlpa: lp_coef(700.0, sr), tlp: 0.0, tlpa: lp_coef(tilt, sr), breath,
            vib: rng.uni() as f64 * TAU, vrate, vdepth, glide: 0.0, rng, k: 0, gain: 1.0 / (f0s.len().max(1) as f32).sqrt(), norm: 1.0,
        };
        g.glide = (1.0 - (-32.0 / (0.09 * sr)).exp()) as f32; // formants move with ~90 ms smoothing
        g.update();
        g
    }
    fn set(&mut self, v: Vowel) { self.tf = v.f; self.tb = v.b; self.tnas = v.nas; }
    fn update(&mut self) {
        let sr = self.sr as f32;
        for i in 0..5 {
            self.cf[i] += (self.tf[i] - self.cf[i]) * self.glide;
            self.cb[i] += (self.tb[i] - self.cb[i]) * self.glide;
            self.fr[i].set(self.cf[i], self.cb[i], sr);
        }
        self.nas += (self.tnas - self.nas) * self.glide;
        // the cascade's gain at the first formant, so every vowel comes out about equally loud
        let w = (TAU * self.cf[0] as f64 / self.sr) as f32;
        let mut mag = 1.0f32;
        for r in self.fr.iter() {
            let (re, im) = (1.0 - r.b * w.cos() - r.c * (2.0 * w).cos(), r.b * w.sin() + r.c * (2.0 * w).sin());
            mag *= r.a.abs() / (re * re + im * im).sqrt().max(1e-6);
        }
        self.norm = (mag * 300.0 / self.cf[0].max(150.0)).max(0.05).powf(-0.7);
    }
    fn run(&mut self) -> f32 {
        if self.k % 32 == 0 { self.update(); }
        self.k = self.k.wrapping_add(1);
        self.vib = (self.vib + TAU * self.vrate / self.sr) % TAU;
        let vib = 1.0 + self.vdepth * self.vib.sin();
        let Voices { s, rng, sr, breath, .. } = self;
        let mut x = 0.0f32;
        for v in s.iter_mut() {
            let c = if v.tgt > v.env { v.atk } else { v.rel };
            v.env += (v.tgt - v.env) * c;
            let dph = v.f * v.mul * v.drift * vib / *sr;
            v.ph += dph;
            if v.ph >= 1.0 {
                v.ph -= 1.0;
                v.mul = 1.0 + 0.0035 * rng.bi() as f64; // jitter: each period a hair different
                v.drift = (v.drift + 0.0004 * rng.bi() as f64).clamp(0.994, 1.006);
                v.odd = !v.odd;
            }
            if v.env < 1e-4 { continue; }
            let saw = (2.0 * v.ph - 1.0 - polyblep(v.ph, dph)) as f32;
            let a = if v.sub && v.odd { 0.55 } else { 1.0 };
            let open = if v.ph < 0.55 { 1.0 } else { 0.25 };
            x += (saw * a + rng.bi() * *breath * open) * v.env * v.amp;
        }
        self.tlp += self.tlpa * (x - self.tlp);
        let mut y = self.tlp;
        for r in self.fr.iter_mut() { y = r.run(y); }
        self.nlp += self.nlpa * (y - self.nlp);
        (y * (1.0 - self.nas) + self.nlp * self.nas * 1.5) * self.gain * self.norm
    }
}

/// Breathing plan for one singer: sing for a while, breathe, come back in (staggered, so the
/// group's sound never stops).
struct Breath { singing: bool, until: f64 }

pub struct Choir { g: [Voices; 2], plan: [Vec<Breath>; 2], t: f64, next_v: [f64; 2], vowels: Vec<char>, verb: Reverb, wet: f32, rng: Rng, k: u32, sr: f64, level: f32 }
impl Choir {
    fn new(root: f64, name: &str, sr: f64) -> Self {
        let root = if root > 0.0 { root } else { 146.83 };
        let (l, r, vowels, tilt, vr, vd, size, wet, level): (Vec<f64>, Vec<f64>, Vec<char>, f64, f64, f64, f32, f32, f32) = match name {
            "organum" => (vec![root, root * 1.5, root * 2.0], vec![root, root * 1.5, root * 0.5], vec!['a', 'o', 'O', 'a'], 3600.0, 5.2, 0.0035, 0.86, 0.55, 1.0),
            "hum" => (vec![root, root * 1.5, root], vec![root * 2.0, root, root * 0.5], vec!['m', 'm', 'n'], 1900.0, 4.8, 0.002, 0.6, 0.35, 1.3),
            "ah" => (vec![root, root * 1.5, root * 2.0], vec![root * 1.25, root, root * 2.0], vec!['a', 'e', 'a', 'o'], 4500.0, 5.5, 0.004, 0.75, 0.45, 0.9),
            _ => (vec![root, root, root * 0.5], vec![root, root * 1.002, root * 0.5], vec!['O', 'o', 'm', 'a', 'O'], 2600.0, 4.6, 0.0018, 0.8, 0.5, 1.1), // ison
        };
        let g = [Voices::new(sr, &l, 11, tilt, 0.05, vr, vd), Voices::new(sr, &r, 23, tilt, 0.05, vr * 1.07, vd)];
        let mut rng = Rng::new(97);
        let plan = [0usize, 1].map(|gi| (0..g[gi].s.len()).map(|i| Breath { singing: false, until: 0.4 * i as f64 + 0.2 * gi as f64 + rng.range(0.0, 0.6) }).collect());
        Choir { g, plan, t: 0.0, next_v: [0.0, 6.0], vowels, verb: Reverb::new(sr, size), wet, rng, k: 0, sr, level }
    }
    fn control(&mut self) {
        let t = self.t;
        for gi in 0..2 {
            if t >= self.next_v[gi] {
                let v = self.vowels[self.rng.pick(self.vowels.len())];
                self.g[gi].set(vowel(v));
                self.next_v[gi] = t + self.rng.range(14.0, 30.0);
            }
            for (i, b) in self.plan[gi].iter_mut().enumerate() {
                if t >= b.until {
                    b.singing = !b.singing;
                    b.until = t + if b.singing { self.rng.range(7.0, 13.0) } else { self.rng.range(1.2, 2.2) };
                    self.g[gi].s[i].tgt = if b.singing { 1.0 } else { 0.0 };
                }
            }
        }
    }
    fn sample(&mut self) -> (f32, f32) {
        if self.k % 64 == 0 { self.control(); }
        self.k = self.k.wrapping_add(1);
        self.t += 1.0 / self.sr;
        let a = self.g[0].run();
        let b = self.g[1].run();
        let (l, r) = (a * 0.8 + b * 0.25, b * 0.8 + a * 0.25);
        let (wl, wr) = self.verb.run(l, r);
        let k = 0.29 * self.level;
        ((l + wl * self.wet) * k, (r + wr * self.wet) * k)
    }
}

/// A group chanting AUM: inhale, a long open "A", rounding through "U" into a humming "M".
pub struct OmChant { g: [Voices; 2], t: f64, cyc: f64, step: [usize; 2], off: [f64; 2], verb: Reverb, k: u32, sr: f64 }
impl OmChant {
    fn new(root: f64, interval: f64, sr: f64) -> Self {
        let root = if root > 0.0 { root } else { 136.1 };
        let l = [root, root * 1.003, root * 0.5];
        let r = [root * 0.997, root, root * 0.5 * 1.002];
        let mut g = [Voices::new(sr, &l, 31, 2600.0, 0.04, 4.5, 0.0012), Voices::new(sr, &r, 47, 2500.0, 0.04, 4.1, 0.0012)];
        for v in g.iter_mut() { for s in v.s.iter_mut() { s.atk = tc(0.28, sr); s.rel = tc(0.5, sr); } }
        let cyc = if interval > 0.0 { interval.clamp(8.0, 30.0) } else { 12.0 };
        OmChant { g, t: 0.0, cyc, step: [9, 9], off: [0.0, 0.17], verb: Reverb::new(sr, 0.7), k: 0, sr }
    }
    fn control(&mut self) {
        let s = self.cyc / 12.0;
        // (time in a 12 s cycle, action): 0 silent breath, 1 sing "A", 2 "O", 3 "U", 4 "M", 5 release
        const SCHED: [(f64, u8); 6] = [(0.0, 0), (1.4, 1), (4.6, 2), (5.4, 3), (6.2, 4), (10.6, 5)];
        for gi in 0..2 {
            let tc_ = (self.t - self.off[gi]).rem_euclid(self.cyc);
            let mut idx = 0;
            for (i, (at, _)) in SCHED.iter().enumerate() { if tc_ >= at * s { idx = i; } }
            if idx == self.step[gi] { continue; }
            self.step[gi] = idx;
            let g = &mut self.g[gi];
            match SCHED[idx].1 {
                0 => g.set(vowel('a')),
                1 => { for x in g.s.iter_mut() { x.tgt = 1.0; } g.set(vowel('a')); }
                2 => g.set(vowel('O')),
                3 => g.set(vowel('u')),
                4 => { g.set(vowel('m')); for x in g.s.iter_mut() { x.tgt = 0.8; } }
                _ => { for x in g.s.iter_mut() { x.tgt = 0.0; } }
            }
        }
    }
    fn sample(&mut self) -> (f32, f32) {
        if self.k % 64 == 0 { self.control(); }
        self.k = self.k.wrapping_add(1);
        self.t += 1.0 / self.sr;
        let a = self.g[0].run();
        let b = self.g[1].run();
        let (l, r) = (a * 0.85 + b * 0.2, b * 0.85 + a * 0.2);
        let (wl, wr) = self.verb.run(l, r);
        ((l + wl * 0.45) * 0.38, (r + wr * 0.45) * 0.38)
    }
}

/// Deep Tibetan chant: very low voices, half of them with the subharmonic "growl", slow syllables,
/// and a vowel that rings one overtone (the 5th or 10th harmonic) like a second note.
pub struct Gyuto { g: [Voices; 2], plan: [Vec<Breath>; 2], t: f64, next: [f64; 2], harm: f64, root: f64, dip: [f32; 2], rng: Rng, verb: Reverb, k: u32, sr: f64 }
impl Gyuto {
    fn new(root: f64, name: &str, sr: f64) -> Self {
        let root = if root > 0.0 { root } else { 68.0 };
        let harm = if name.contains("10") { 10.0 } else { 5.0 };
        let l = [root, root * 1.002, root * 0.998, root * 2.0];
        let r = [root * 1.001, root * 0.999, root, root];
        let mut g = [Voices::new(sr, &l, 61, 2300.0, 0.06, 0.0, 0.0), Voices::new(sr, &r, 83, 2200.0, 0.06, 0.0, 0.0)];
        for v in g.iter_mut() {
            for (i, s) in v.s.iter_mut().enumerate() { s.sub = i < 2; s.atk = tc(0.35, sr); s.rel = tc(0.5, sr); }
        }
        g[0].s[3].amp = 0.4;
        let mut rng = Rng::new(5);
        let plan = [0usize, 1].map(|gi| (0..4).map(|i| Breath { singing: false, until: 0.3 * i as f64 + 0.25 * gi as f64 + rng.range(0.0, 0.5) }).collect());
        Gyuto { g, plan, t: 0.0, next: [0.0, 0.9], harm, root, dip: [1.0; 2], rng, verb: Reverb::new(sr, 0.78), k: 0, sr }
    }
    fn overtone(&self) -> Vowel {
        let f = (self.root * self.harm) as f32;
        Vowel { f: [f, f * 1.07, 2400.0, 3300.0, 3900.0], b: [35.0, 45.0, 220.0, 300.0, 300.0], nas: 0.0 }
    }
    fn control(&mut self) {
        let t = self.t;
        for gi in 0..2 {
            if t >= self.next[gi] {
                // one syllable: a short dip, then a new vowel; about half the time the overtone vowel
                let v = if self.rng.chance(0.5) { self.overtone() } else { vowel(['o', 'O', 'u', 'a'][self.rng.pick(4)]) };
                self.g[gi].set(v);
                self.dip[gi] = 0.5;
                self.next[gi] = t + self.rng.range(1.4, 2.4);
            }
            for (i, b) in self.plan[gi].iter_mut().enumerate() {
                if t >= b.until {
                    b.singing = !b.singing;
                    b.until = t + if b.singing { self.rng.range(9.0, 14.0) } else { self.rng.range(1.2, 2.0) };
                    self.g[gi].s[i].tgt = if b.singing { 1.0 } else { 0.0 };
                }
            }
        }
    }
    fn sample(&mut self) -> (f32, f32) {
        if self.k % 64 == 0 { self.control(); }
        self.k = self.k.wrapping_add(1);
        self.t += 1.0 / self.sr;
        let rec = tc(0.16, self.sr);
        for d in self.dip.iter_mut() { *d += (1.0 - *d) * rec; }
        let a = self.g[0].run() * self.dip[0];
        let b = self.g[1].run() * self.dip[1];
        let (l, r) = (a * 0.85 + b * 0.25, b * 0.85 + a * 0.25);
        let (wl, wr) = self.verb.run(l, r);
        ((l + wl * 0.5) * 0.46, (r + wr * 0.5) * 0.46)
    }
}

// ---------------------------------------------------------------- flutes

struct FStyle {
    scale: Vec<f64>, lo: i32, hi: i32, durs: Vec<(f64, f32)>, rest: (f64, f64), notes: (i32, i32),
    breath: f32, vib: (f64, f64, f64), glide: f64, scoop: f64, grace: f32, drone: bool, muraiki: f32,
    room: f32, wet: f32, swell: bool, alap: bool,
}

pub struct Flute {
    sr: f64, st: FStyle, root: f64, rng: Rng,
    f: f64, ft: f64, amp: f32, at: f32, atk: f32, rel: f32, o: [Osc; 3], bp: [Bp; 2], hp: f32, chiff: f32,
    vib: f64, vib_on: f32, note_t: f64, bend: f64,
    t: f64, next: f64, deg: i32, left: i32, dur: f64, grace_to: Option<i32>,
    drone: [Osc; 2], verb: Reverb, k: u32, start: f64,
}
impl Flute {
    fn new(root: f64, name: &str, sr: f64) -> Self {
        let (style, var) = name.split_once(':').unwrap_or((name, ""));
        let d = |v: &[(f64, f32)]| v.to_vec();
        let (st, def_root) = match style {
            "shakuhachi" => (FStyle {
                scale: vec![0.0, 300.0, 500.0, 700.0, 1000.0], lo: -1, hi: 7,
                durs: d(&[(2.5, 1.0), (3.5, 1.5), (5.0, 1.2), (7.0, 0.6), (1.2, 0.5)]), rest: (2.5, 6.5), notes: (1, 4),
                breath: 0.55, vib: (3.6, 22.0, 1.4), glide: 0.18, scoop: -70.0, grace: 0.0, drone: false, muraiki: 0.35,
                room: 0.65, wet: 0.4, swell: true, alap: false }, 293.66),
            "bansuri" => {
                let scale = match var {
                    "bhupali" => vec![0.0, 204.0, 386.0, 702.0, 906.0],
                    "bhairav" => vec![0.0, 112.0, 386.0, 498.0, 702.0, 814.0, 1088.0],
                    _ => vec![0.0, 204.0, 386.0, 590.0, 702.0, 906.0, 1088.0], // Yaman
                };
                let n = scale.len() as i32;
                (FStyle { scale, lo: -2, hi: n + 2, durs: d(&[(0.8, 0.8), (1.5, 1.4), (2.5, 1.4), (4.0, 0.8), (0.4, 0.4)]),
                    rest: (1.5, 4.0), notes: (3, 7), breath: 0.32, vib: (5.0, 12.0, 0.8), glide: 0.25, scoop: -25.0, grace: 0.2,
                    drone: false, muraiki: 0.0, room: 0.55, wet: 0.3, swell: false, alap: true }, 220.0)
            }
            "xiao" => (FStyle {
                scale: vec![0.0, 200.0, 400.0, 700.0, 900.0], lo: -2, hi: 6,
                durs: d(&[(1.5, 1.0), (2.5, 1.4), (4.0, 1.0), (0.8, 0.6), (5.5, 0.4)]), rest: (2.0, 5.0), notes: (2, 6),
                breath: 0.6, vib: (4.5, 10.0, 1.0), glide: 0.15, scoop: -20.0, grace: 0.1, drone: false, muraiki: 0.0,
                room: 0.6, wet: 0.35, swell: true, alap: false }, 261.63),
            _ => { // ney, in a maqam
                let scale = match var {
                    "rast" => vec![0.0, 204.0, 351.0, 498.0, 702.0, 906.0, 1053.0],
                    "bayati" => vec![0.0, 151.0, 294.0, 498.0, 702.0, 792.0, 996.0],
                    _ => vec![0.0, 112.0, 386.0, 498.0, 702.0, 814.0, 1018.0], // Hijaz
                };
                (FStyle { scale, lo: -3, hi: 9, durs: d(&[(0.6, 0.8), (1.2, 1.3), (2.0, 1.2), (3.5, 0.8), (0.3, 0.5)]),
                    rest: (2.0, 4.5), notes: (3, 7), breath: 0.45, vib: (5.2, 18.0, 0.6), glide: 0.12, scoop: -30.0, grace: 0.25,
                    drone: true, muraiki: 0.1, room: 0.7, wet: 0.4, swell: false, alap: false }, 293.66)
            }
        };
        let root = if root > 0.0 { root } else { def_root };
        let o = [Osc::new(root, sr, 0.0), Osc::new(root * 2.0, sr, 0.0), Osc::new(root * 3.0, sr, 0.0)];
        let srf = sr as f32;
        let drone = [Osc::new(root * 0.5, sr, 0.0), Osc::new(root * 0.75, sr, 0.0)];
        Flute {
            sr, root, rng: Rng::new(0xF1u64 + root as u64), f: root, ft: root, amp: 0.0, at: 0.0, atk: tc(0.12, sr), rel: tc(0.25, sr),
            o, bp: [Bp::new(root as f32, 9.0, srf), Bp::new(2.0 * root as f32, 9.0, srf)], hp: 0.0, chiff: 0.0,
            vib: 0.0, vib_on: 0.0, note_t: 0.0, bend: 0.0, t: 0.0, next: 1.5, deg: 0, left: 0, dur: 1.0, grace_to: None,
            verb: Reverb::new(sr, st.room), drone, st, k: 0, start: 0.0,
        }
    }
    fn pick_dur(&mut self) -> f64 {
        let tot: f32 = self.st.durs.iter().map(|x| x.1).sum();
        let mut r = self.rng.uni() * tot;
        for (d, w) in self.st.durs.iter() { if r < *w { return *d * self.rng.range(0.85, 1.15); } r -= *w; }
        self.st.durs[0].0
    }
    fn start_note(&mut self, deg: i32, dur: f64) {
        self.deg = deg;
        self.ft = note(self.root, &self.st.scale, deg);
        self.note_t = 0.0;
        self.dur = dur;
        self.at = if dur < 0.5 { 0.75 } else { 1.0 } * (0.85 + 0.15 * self.rng.uni());
        self.vib_on = 0.0;
        if self.amp < 0.05 {
            // a new phrase: start at pitch, with the breath "chiff" and maybe a shakuhachi bend or a ney scoop
            self.f = self.ft;
            self.chiff = 1.0;
            self.bend = if self.rng.chance(0.6) { self.st.scoop } else { 0.0 };
            if self.st.muraiki > 0.0 && self.rng.chance(self.st.muraiki) { self.chiff = 2.5; }
        }
    }
    fn sequence(&mut self) {
        let n = self.st.scale.len() as i32;
        if let Some(d) = self.grace_to.take() { // the grace note resolves to its main note
            let dur = self.pick_dur();
            self.start_note(d, dur);
            self.next = self.t + dur;
            return;
        }
        if self.left <= 0 {
            if self.amp > 0.02 || self.at > 0.0 { // end of phrase: rest
                self.at = 0.0;
                self.next = self.t + self.rng.range(self.st.rest.0, self.st.rest.1);
                self.left = self.rng.range(self.st.notes.0 as f64, self.st.notes.1 as f64 + 0.99) as i32;
                return;
            }
            self.left = self.rng.range(self.st.notes.0 as f64, self.st.notes.1 as f64 + 0.99) as i32;
        }
        // range grows slowly in alap style (begin around Sa, open upward over the first minutes)
        let hi = if self.st.alap { (self.st.lo + 2 + ((self.t - self.start) / 40.0) as i32).min(self.st.hi) } else { self.st.hi };
        let mut d = self.deg + [-2, -1, -1, 1, 1, 2, 0, -1][self.rng.pick(8)];
        if self.left == 1 { // phrase ends on the tonic, the fifth or the fourth
            let homes = [0, n, (n as f64 * 0.57) as i32, (n as f64 * 0.43) as i32];
            d = homes[self.rng.pick(4)].min(hi);
        }
        d = d.clamp(self.st.lo, hi);
        self.left -= 1;
        if self.st.grace > 0.0 && self.rng.chance(self.st.grace) && d != self.deg {
            self.grace_to = Some(d);
            self.start_note(d + 1, 0.12);
            self.next = self.t + 0.12;
            return;
        }
        let dur = self.pick_dur();
        self.start_note(d, dur);
        self.next = self.t + dur;
        let _ = n;
    }
    fn sample(&mut self) -> (f32, f32) {
        let dt = 1.0 / self.sr;
        self.t += dt;
        if self.t >= self.next { self.sequence(); }
        if self.k % 16 == 0 {
            let s = 16.0 * dt;
            self.f += (self.ft - self.f) * (1.0 - (-s / self.st.glide).exp());
            self.bend *= (-s / 0.35).exp();
            self.note_t += s;
            if self.note_t > self.st.vib.2 { self.vib_on = (self.vib_on + (s / 0.6) as f32).min(1.0); }
            self.vib = (self.vib + TAU * self.st.vib.0 * s) % TAU;
            let cents = self.bend + self.st.vib.1 * self.vib_on as f64 * self.vib.sin();
            let f = self.f * 2f64.powf(cents / 1200.0);
            let sr = self.sr;
            for (i, o) in self.o.iter_mut().enumerate() { o.set(f * (i + 1) as f64, sr); }
            self.bp[0].set(f as f32, 9.0, sr as f32);
            self.bp[1].set(2.0 * f as f32, 9.0, sr as f32);
            if self.k % 8192 == 0 { for o in self.o.iter_mut() { o.fix(); } for o in self.drone.iter_mut() { o.fix(); } }
        }
        self.k = self.k.wrapping_add(1);
        // breath swell within long notes (shakuhachi and xiao): rise, then fall away
        let mut tgt = self.at;
        if self.st.swell && self.at > 0.0 && self.dur > 1.5 {
            let x = (self.note_t / self.dur).clamp(0.0, 1.0);
            tgt *= (0.65 + 0.6 * (std::f64::consts::PI * x).sin()) as f32;
        }
        let c = if tgt > self.amp { self.atk } else { self.rel };
        self.amp += (tgt - self.amp) * c;
        let tone = self.o[0].step() + 0.16 * self.o[1].step() + 0.05 * self.o[2].step();
        let n = self.rng.bi();
        self.hp += 0.3 * (n - self.hp);
        let air = self.bp[0].run(n) * 3.2 + self.bp[1].run(n) * 1.4 + (n - self.hp) * 0.06;
        let br = self.st.breath;
        let mut v = self.amp * (tone * (1.0 - br) + air * br);
        if self.chiff > 0.001 { v += n * self.chiff * 0.12 * self.amp.max(0.3); self.chiff *= 0.9985; }
        let mut d = 0.0;
        if self.st.drone { d = (self.drone[0].step() + 0.5 * self.drone[1].step()) * 0.05; }
        let (wl, wr) = self.verb.run(v, v);
        let w = self.st.wet;
        ((v * 0.95 + wl * w + d) * 0.42, (v + wr * w + d) * 0.42)
    }
}

// ---------------------------------------------------------------- plucked strings (Karplus-Strong)

const KS_LEN: usize = 4096;
struct Ks { buf: Vec<f32>, w: usize, d: f64, dt: f64, gl: f64, g: f32, prev: f32, live: bool, pan: f32, age: f64 }
impl Ks {
    fn new() -> Self { Ks { buf: vec![0.0; KS_LEN], w: 0, d: 100.0, dt: 100.0, gl: 0.0, g: 0.99, prev: 0.0, live: false, pan: 0.0, age: 0.0 } }
    fn pluck(&mut self, f: f64, sr: f64, t60: f64, bright: f32, amp: f32, pos: f64, rng: &mut Rng) {
        let d = (sr / f - 0.5).clamp(4.0, (KS_LEN - 8) as f64);
        self.d = d;
        self.dt = d;
        self.g = 10f64.powf(-3.0 / (f * t60)) as f32;
        let n = d.ceil() as usize;
        let mut lp = 0.0f32;
        let mut exc = vec![0.0f32; n];
        for e in exc.iter_mut() { lp += bright * (rng.bi() - lp); *e = lp; }
        let p = ((d * pos) as usize).max(1);
        for i in (p..n).rev() { exc[i] -= exc[i - p]; } // pluck position: removes a few harmonics, as on a real string
        let m = exc.iter().fold(0.0f32, |a, x| a.max(x.abs())).max(1e-6);
        for (i, e) in exc.iter().enumerate() {
            let idx = (self.w + KS_LEN - n + i) % KS_LEN;
            self.buf[idx] = e / m * amp;
        }
        self.prev = 0.0;
        self.live = true;
        self.age = 0.0;
    }
    fn slide(&mut self, f: f64, sr: f64, secs: f64) {
        self.dt = (sr / f - 0.5).clamp(4.0, (KS_LEN - 8) as f64);
        self.gl = 1.0 - (-1.0 / (secs * sr)).exp();
    }
    #[inline]
    fn run(&mut self) -> f32 {
        if !self.live { return 0.0; }
        if self.gl > 0.0 { self.d += (self.dt - self.d) * self.gl; }
        let rp = self.w as f64 + KS_LEN as f64 - self.d;
        let i = rp.floor() as usize;
        let fr = (rp - rp.floor()) as f32;
        let y = self.buf[i % KS_LEN] * (1.0 - fr) + self.buf[(i + 1) % KS_LEN] * fr;
        let s = (y + self.prev) * 0.5 * self.g;
        self.prev = y;
        self.buf[self.w] = s;
        self.w = (self.w + 1) % KS_LEN;
        y
    }
}

/// Guqin (Chinese seven-string zither, pentatonic, slides and harmonics) or Celtic harp (Dorian arpeggios).
pub struct Strings { sr: f64, qin: bool, root: f64, ks: Vec<Ks>, rr: usize, bank: Bank, rng: Rng, t: f64, next: f64, queue: Vec<(f64, u8, i32)>, verb: Reverb, k: u32 }
impl Strings {
    fn new(qin: bool, root: f64, sr: f64) -> Self {
        let root = if root > 0.0 { root } else if qin { 65.41 } else { 73.42 };
        Strings { sr, qin, root, ks: (0..6).map(|_| Ks::new()).collect(), rr: 0, bank: Bank::new(sr, 24), rng: Rng::new(if qin { 7 } else { 13 }),
            t: 0.0, next: 1.0, queue: Vec::new(), verb: Reverb::new(sr, if qin { 0.6 } else { 0.7 }), k: 0 }
    }
    fn scale(&self) -> &'static [f64] {
        if self.qin { &[0.0, 200.0, 400.0, 700.0, 900.0] } else { &[0.0, 200.0, 300.0, 500.0, 700.0, 900.0, 1000.0] }
    }
    fn pluck(&mut self, deg: i32, amp: f32) {
        let f = note(self.root, self.scale(), deg);
        let sr = self.sr;
        let (t60, bright, pos) = if self.qin { (8.0 * (130.0 / f).powf(0.4).clamp(0.4, 1.6), 0.2f32, 0.14) } else { (5.0 * (150.0 / f).powf(0.5).clamp(0.3, 1.5), 0.35, 0.18) };
        let i = self.rr;
        self.rr = (self.rr + 1) % self.ks.len();
        let pan = 0.35 * self.rng.bi();
        let Strings { ks, rng, .. } = self;
        ks[i].pluck(f, sr, t60, bright, amp, pos, rng);
        ks[i].pan = pan;
        ks[i].gl = 0.0;
    }
    fn gesture(&mut self) {
        let t = self.t;
        if self.qin {
            // one slow gesture every few seconds: a note, a note that slides, an interval, or harmonics
            let base = [5, 6, 7, 8, 9, 10, 4, 3][self.rng.pick(8)];
            match self.rng.pick(5) {
                0 | 1 => self.queue.push((t, 0, base)),
                2 => { self.queue.push((t, 0, base)); self.queue.push((t + self.rng.range(0.6, 1.2), 1, base + if self.rng.chance(0.5) { 1 } else { -1 })); }
                3 => { self.queue.push((t, 0, base)); self.queue.push((t + 0.05, 0, base - 3)); }
                _ => { for j in 0..(1 + self.rng.pick(3)) { self.queue.push((t + j as f64 * self.rng.range(0.5, 0.9), 2, base + j as i32)); } }
            }
            self.next = t + self.rng.range(3.0, 8.5);
        } else {
            // a rising or falling arpeggio on i, VII or IV of the Dorian mode, sometimes over a bass note
            let chord = [[0, 2, 4, 7, 9], [6, 8, 10, 13, 15], [3, 5, 7, 10, 12]][self.rng.pick(3)];
            let up = self.rng.chance(0.65);
            let gap = self.rng.range(0.28, 0.5);
            if self.rng.chance(0.5) { self.queue.push((t, 0, chord[0] - 7)); }
            let n = 3 + self.rng.pick(3);
            for j in 0..n {
                let d = if up { chord[j % 5] + 7 * (j / 5) as i32 } else { chord[(n - 1 - j) % 5] };
                self.queue.push((t + 0.05 + j as f64 * gap, 0, d + 7));
            }
            self.next = t + n as f64 * gap + self.rng.range(2.0, 5.0);
        }
    }
    fn sample(&mut self) -> (f32, f32) {
        let dt = 1.0 / self.sr;
        self.t += dt;
        if self.k % 64 == 0 {
            if self.t >= self.next { self.gesture(); }
            let t = self.t;
            let mut i = 0;
            while i < self.queue.len() {
                if self.queue[i].0 <= t {
                    let (_, kind, deg) = self.queue.swap_remove(i);
                    match kind {
                        0 => { let a = 0.8 + 0.2 * self.rng.uni(); self.pluck(deg, a); }
                        1 => { // slide the last string to a new note
                            let f = note(self.root, self.scale(), deg);
                            let j = (self.rr + self.ks.len() - 1) % self.ks.len();
                            let sr = self.sr;
                            self.ks[j].slide(f, sr, 0.18);
                        }
                        _ => { // harmonic: a pure, bell-like overtone
                            let f = note(self.root, self.scale(), deg) * 4.0;
                            let p = 0.3 * self.rng.bi();
                            self.bank.add(f, 0.1, 2.5, 0.002, p);
                            self.bank.add(f * 2.0, 0.02, 1.0, 0.002, p);
                        }
                    }
                } else { i += 1; }
            }
            for s in self.ks.iter_mut() { if s.live { s.age += 64.0 * dt; if s.age > 14.0 { s.live = false; } } }
        }
        self.k = self.k.wrapping_add(1);
        let (mut l, mut r) = (0.0f32, 0.0f32);
        for s in self.ks.iter_mut() {
            let v = s.run();
            let (a, b) = pan2(v, s.pan);
            l += a;
            r += b;
        }
        let (bl, br) = self.bank.run();
        l += bl;
        r += br;
        let (wl, wr) = self.verb.run(l, r);
        let k = if self.qin { 0.84 } else { 0.75 };
        (soft((l + wl * 0.35) * k, 0.45, 0.7), soft((r + wr * 0.35) * k, 0.45, 0.7))
    }
}

// ---------------------------------------------------------------- frame drums

struct DHit { t: f64, amp: f32, kind: u8, pan: f32 }

/// Daf: Sufi frame drum with jingling rings, a steady dhikr groove (one step every `interval` s).
pub struct Daf { sr: f64, step_t: f64, clock: f64, step: usize, bar: usize, hits: Vec<DHit>, jbp: [Bp; 4], jenv: f32, rng: Rng, verb: Reverb }
impl Daf {
    fn new(interval: f64, sr: f64) -> Self {
        let srf = sr as f32;
        Daf { sr, step_t: if interval > 0.0 { interval.clamp(0.12, 1.5) } else { 0.3 }, clock: 1e9, step: 0, bar: 0, hits: Vec::with_capacity(16),
            jbp: [Bp::new(4800.0, 12.0, srf), Bp::new(6300.0, 14.0, srf), Bp::new(7900.0, 14.0, srf), Bp::new(9600.0, 12.0, srf)],
            jenv: 0.0, rng: Rng::new(41), verb: Reverb::new(sr, 0.5) }
    }
    fn sample(&mut self) -> (f32, f32) {
        let dt = 1.0 / self.sr;
        self.clock += dt;
        if self.clock >= self.step_t {
            self.clock = 0.0;
            const A: &[u8; 8] = b"D.tTD.T.";
            const B: &[u8; 8] = b"D.tTD.TT";
            let pat = if self.bar % 4 == 3 { B } else { A };
            let c = pat[self.step % 8];
            self.step += 1;
            if self.step % 8 == 0 { self.bar += 1; }
            let hum = 0.92 + 0.08 * self.rng.uni();
            let (kind, amp) = match c { b'D' => (0u8, 1.0f32), b'T' => (1, 0.75), b't' => (1, 0.4), _ => (9, 0.0) };
            if kind < 9 {
                self.hits.push(DHit { t: 0.0, amp: amp * hum, kind, pan: 0.1 * self.rng.bi() });
                self.jenv = self.jenv.max(if kind == 0 { 0.5 } else { 0.8 } * amp);
                if self.hits.len() > 12 { self.hits.remove(0); }
            }
        }
        let (mut l, mut r) = (0.0f32, 0.0f32);
        for h in self.hits.iter_mut() {
            let t = h.t;
            let v = if h.kind == 0 {
                let f = 68.0 + 30.0 * (-t / 0.04).exp();
                (TAU * f * t).sin() as f32 * (-t / 0.28).exp() as f32 + self.rng.bi() * 0.15 * (-t / 0.012).exp() as f32
            } else {
                (TAU * 380.0 * t).sin() as f32 * 0.5 * (-t / 0.05).exp() as f32 + self.rng.bi() * 0.45 * (-t / 0.008).exp() as f32
            } * h.amp;
            let (a, b) = pan2(v, h.pan);
            l += a;
            r += b;
            h.t += dt;
        }
        self.hits.retain(|h| h.t < 1.0);
        // the rings inside the frame
        if self.jenv > 1e-4 {
            let n = self.rng.bi() * self.jenv;
            let mut j = 0.0;
            for b in self.jbp.iter_mut() { j += b.run(n); }
            l += j * 0.9;
            r += j * 0.7;
            self.jenv *= (-dt / 0.11).exp() as f32;
        }
        let (wl, wr) = self.verb.run(l, r);
        ((l + wl * 0.25) * 0.5, (r + wr * 0.25) * 0.5)
    }
}

/// Damaru: small Tibetan hourglass drum, rattled in quick bursts ("drr-drr ... drrrr").
pub struct Damaru { sr: f64, per: f64, clock: f64, n: usize, hits: Vec<DHit>, queue: Vec<f64>, rng: Rng, verb: Reverb, t: f64 }
impl Damaru {
    fn new(interval: f64, sr: f64) -> Self {
        Damaru { sr, per: if interval > 0.0 { interval.clamp(0.4, 4.0) } else { 0.9 }, clock: 1e9, n: 0, hits: Vec::with_capacity(32), queue: Vec::new(), rng: Rng::new(77), verb: Reverb::new(sr, 0.6), t: 0.0 }
    }
    fn sample(&mut self) -> (f32, f32) {
        let dt = 1.0 / self.sr;
        self.clock += dt;
        self.t += dt;
        if self.clock >= self.per {
            self.clock = 0.0;
            self.n += 1;
            let strikes = if self.n % 4 == 0 { 10 + self.rng.pick(5) } else { 3 + self.rng.pick(2) };
            let rate = self.rng.range(15.0, 18.0);
            for j in 0..strikes { self.queue.push(self.t + j as f64 / rate); }
        }
        let t = self.t;
        let mut i = 0;
        while i < self.queue.len() {
            if self.queue[i] <= t {
                self.queue.swap_remove(i);
                let side = (self.hits.len() % 2) as u8;
                self.hits.push(DHit { t: 0.0, amp: 0.7 + 0.3 * self.rng.uni(), kind: side, pan: if side == 0 { -0.2 } else { 0.2 } });
                if self.hits.len() > 24 { self.hits.remove(0); }
            } else { i += 1; }
        }
        let (mut l, mut r) = (0.0f32, 0.0f32);
        for h in self.hits.iter_mut() {
            let f = if h.kind == 0 { 330.0 } else { 372.0 };
            let v = ((TAU * f * h.t).sin() as f32 * (-h.t / 0.035).exp() as f32 + self.rng.bi() * 0.3 * (-h.t / 0.004).exp() as f32) * h.amp;
            let (a, b) = pan2(v, h.pan);
            l += a;
            r += b;
            h.t += dt;
        }
        self.hits.retain(|h| h.t < 0.3);
        let (wl, wr) = self.verb.run(l, r);
        ((l + wl * 0.3) * 0.45, (r + wr * 0.3) * 0.45)
    }
}

// ---------------------------------------------------------------- bells and gongs

/// Stage bell: three strikes to open the session, one at each new stage, two at the start of the last.
pub struct Rin { sr: f64, bank: Bank, modes: Vec<(f64, f32, f64)>, root: f64, stage: usize, started: bool, queue: Vec<f64>, t: f64, rng: Rng, verb: Reverb, gain: f32, click: f32, out: f32 }
impl Rin {
    fn new(root: f64, name: &str, sr: f64) -> Self {
        let (modes, def): (Vec<(f64, f32, f64)>, f64) = match name {
            // tuned church bell: hum, prime, tierce (minor third), quint, nominal, superquint, octave nominal
            "church" => (vec![(0.5, 0.55, 14.0), (1.0, 0.5, 10.0), (1.2, 0.42, 8.0), (1.5, 0.22, 6.0), (2.0, 0.8, 7.0), (3.0, 0.28, 4.0), (4.0, 0.18, 3.0)], 261.63),
            // small hand bell (Tibetan drilbu / Theravada bell): high and shimmering
            "bell" => (vec![(1.0, 1.0, 6.0), (2.32, 0.45, 3.0), (4.25, 0.2, 1.6), (6.63, 0.1, 0.9)], 1150.0),
            "qing" => (vec![(1.0, 1.0, 12.0), (2.63, 0.4, 6.0), (4.95, 0.16, 3.0), (7.6, 0.06, 1.6)], 420.0),
            // rin / keisu: Japanese temple bowl bell
            _ => (vec![(1.0, 1.0, 16.0), (2.76, 0.5, 8.0), (5.40, 0.22, 4.0), (8.93, 0.09, 2.2)], 330.0),
        };
        let root = if root > 0.0 { root } else { def };
        let out = match name { "church" => 0.22, "bell" => 0.25, _ => 0.27 };
        Rin { sr, bank: Bank::new(sr, 64), modes, root, stage: usize::MAX, started: false, queue: Vec::new(), t: 0.0, rng: Rng::new(3), verb: Reverb::new(sr, 0.7), gain: 0.0, click: 0.0, out }
    }
    fn set_stage(&mut self, k: usize, n: usize) {
        if self.started && k == self.stage { return; }
        let first = !self.started;
        self.started = true;
        self.stage = k;
        let t = self.t + 0.6;
        let times: &[f64] = if first && k == 0 { &[0.0, 6.5, 13.0] } else if n > 1 && k + 1 == n { &[0.0, 6.0] } else { &[0.0] };
        for x in times { self.queue.push(t + x); }
    }
    fn strike(&mut self, amp: f32) {
        let pan = 0.15 * self.rng.bi();
        let modes = self.modes.clone();
        for (m, a, t60) in modes {
            let f = self.root * m;
            self.bank.add(f - 0.35, a * amp * 0.5, t60, 0.001, pan - 0.15);
            self.bank.add(f + 0.35, a * amp * 0.5, t60, 0.001, pan + 0.15);
        }
        self.click = amp;
    }
    fn sample(&mut self, gain: f32) -> (f32, f32) {
        self.t += 1.0 / self.sr;
        self.gain = gain;
        if !self.queue.is_empty() {
            let t = self.t;
            let mut i = 0;
            while i < self.queue.len() {
                if self.queue[i] <= t { self.queue.swap_remove(i); let g = self.gain; if g > 1e-3 { self.strike(g); } } else { i += 1; }
            }
        }
        let (mut l, mut r) = self.bank.run();
        if self.click > 1e-4 { let n = self.rng.bi() * self.click * 0.15; l += n; r += n; self.click *= 0.995; }
        let (wl, wr) = self.verb.run(l, r);
        ((l + wl * 0.3) * self.out, (r + wr * 0.3) * self.out)
    }
}

/// Gong bath: soft mallet strokes at uneven intervals, swelling and fading in slow waves,
/// the higher modes blooming a moment after each stroke.
pub struct GongBath { sr: f64, bank: Bank, root: f64, t: f64, next: f64, rng: Rng, verb: Reverb }
impl GongBath {
    fn new(root: f64, sr: f64) -> Self {
        GongBath { sr, bank: Bank::new(sr, 96), root: if root > 0.0 { root } else { 98.0 }, t: 0.0, next: 0.5, rng: Rng::new(19), verb: Reverb::new(sr, 0.8) }
    }
    fn sample(&mut self) -> (f32, f32) {
        self.t += 1.0 / self.sr;
        if self.t >= self.next {
            let wave = 0.3 + 0.7 * (0.5 - 0.5 * (TAU * self.t / 240.0).cos()) as f32;
            let amp = wave * (0.6 + 0.4 * self.rng.uni());
            const P: [(f64, f32, f64, f64); 9] = [(1.0, 1.0, 18.0, 0.01), (1.48, 0.55, 14.0, 0.05), (2.01, 0.45, 12.0, 0.15), (2.47, 0.32, 10.0, 0.3),
                (2.96, 0.28, 8.0, 0.5), (3.62, 0.2, 6.5, 0.7), (4.41, 0.14, 5.0, 0.9), (5.33, 0.1, 4.0, 1.1), (6.8, 0.06, 3.0, 1.3)];
            let pan = 0.3 * self.rng.bi();
            let det = 1.0 + 0.004 * self.rng.bi() as f64;
            for (m, a, t60, att) in P {
                let a = a * (0.6 + 0.8 * self.rng.uni());
                let f = self.root * m * det;
                self.bank.add(f - 0.3, a * amp * 0.3, t60, att, pan - 0.2);
                self.bank.add(f + 0.3, a * amp * 0.3, t60, att, pan + 0.2);
            }
            self.next = self.t + self.rng.range(1.3, 3.2);
        }
        let (l, r) = self.bank.run();
        let (wl, wr) = self.verb.run(l, r);
        ((l + wl * 0.4) * 0.9, (r + wr * 0.4) * 0.9)
    }
}

// ---------------------------------------------------------------- nature

struct Bub { ph: f64, f: f64, df: f64, amp: f32, dec: f32, pan: f32 }
/// Running water: a bed of soft noise and a stream of tiny rising bubble tones (after van den Doel).
pub struct Stream { sr: f64, b: Vec<Bub>, lp: [f32; 2], lp2: [f32; 2], rng: Rng, swell: f64 }
impl Stream {
    fn new(sr: f64) -> Self { Stream { sr, b: Vec::with_capacity(64), lp: [0.0; 2], lp2: [0.0; 2], rng: Rng::new(29), swell: 0.0 } }
    fn sample(&mut self) -> (f32, f32) {
        let dt = 1.0 / self.sr;
        self.swell = (self.swell + TAU * 0.05 * dt) % TAU;
        let rate = 70.0 * (1.0 + 0.3 * self.swell.sin());
        if (self.rng.uni() as f64) < rate * dt && self.b.len() < 48 {
            let f = 300.0 * 2f64.powf(self.rng.uni() as f64 * 2.6);
            let life = 0.006 + 0.02 * self.rng.uni() as f64;
            let amp = 0.05 + 0.25 * self.rng.uni().powi(3);
            self.b.push(Bub { ph: 0.0, f, df: f * 0.3 / (life * self.sr), amp, dec: (-1.0 / (life * self.sr)).exp() as f32, pan: 0.8 * self.rng.bi() });
        }
        let (mut l, mut r) = (0.0f32, 0.0f32);
        for q in self.b.iter_mut() {
            q.ph = (q.ph + TAU * q.f * dt) % TAU;
            q.f += q.df;
            let v = q.ph.sin() as f32 * q.amp;
            q.amp *= q.dec;
            let (a, b) = pan2(v, q.pan);
            l += a;
            r += b;
        }
        self.b.retain(|q| q.amp > 1e-4);
        for ch in 0..2 {
            let w = self.rng.bi();
            self.lp[ch] += 0.12 * (w - self.lp[ch]);
            self.lp2[ch] += 0.5 * (self.lp[ch] - self.lp2[ch]);
            let v = (self.lp[ch] - self.lp2[ch] * 0.6) * 0.35;
            if ch == 0 { l += v } else { r += v }
        }
        (l * 1.15, r * 1.15)
    }
}

/// Wind in trees: band-passed noise in slow gusts, with a leafy rustle on top.
pub struct Wind { sr: f64, bp: [Bp; 2], hp: [f32; 2], gust: f32, gt: f32, cf: f32, ct: f32, rng: Rng, k: u32 }
impl Wind {
    fn new(sr: f64) -> Self {
        let s = sr as f32;
        Wind { sr, bp: [Bp::new(420.0, 0.9, s), Bp::new(470.0, 0.9, s)], hp: [0.0; 2], gust: 0.3, gt: 0.5, cf: 450.0, ct: 450.0, rng: Rng::new(53), k: 0 }
    }
    fn sample(&mut self) -> (f32, f32) {
        if self.k % 256 == 0 {
            if self.rng.chance(0.004) { self.gt = 0.15 + 0.85 * self.rng.uni(); self.ct = 280.0 + 600.0 * self.rng.uni(); }
            let c = 256.0 / (2.5 * self.sr as f32);
            self.gust += (self.gt - self.gust) * c;
            self.cf += (self.ct - self.cf) * c;
            let s = self.sr as f32;
            self.bp[0].set(self.cf, 0.9, s);
            self.bp[1].set(self.cf * 1.12, 0.9, s);
        }
        self.k = self.k.wrapping_add(1);
        let (n0, n1) = (self.rng.bi(), self.rng.bi());
        let lo = [self.bp[0].run(n0), self.bp[1].run(n1)];
        self.hp[0] += 0.5 * (n0 - self.hp[0]);
        self.hp[1] += 0.5 * (n1 - self.hp[1]);
        let rustle = self.gust * self.gust * 0.08;
        let l = lo[0] * self.gust * 0.9 + (n0 - self.hp[0]) * rustle;
        let r = lo[1] * self.gust * 0.9 + (n1 - self.hp[1]) * rustle;
        (l * 1.2, r * 1.2)
    }
}

/// A wood fire: crackles and pops over a soft hiss and a low roar.
pub struct Fire { sr: f64, cr: Vec<(Bp, f32, f32, f32)>, lp: [f32; 2], hp: f32, flick: f32, ft: f32, rng: Rng, k: u32, burst: f32 }
impl Fire {
    fn new(sr: f64) -> Self { Fire { sr, cr: Vec::with_capacity(16), lp: [0.0; 2], hp: 0.0, flick: 0.5, ft: 0.5, rng: Rng::new(67), k: 0, burst: 0.0 } }
    fn sample(&mut self) -> (f32, f32) {
        let dt = 1.0 / self.sr as f32;
        if self.k % 128 == 0 {
            if self.rng.chance(0.02) { self.ft = 0.25 + 0.75 * self.rng.uni(); }
            self.flick += (self.ft - self.flick) * 0.05;
            if self.rng.chance(0.003) { self.burst = 1.0; }
            self.burst *= 0.97;
        }
        self.k = self.k.wrapping_add(1);
        let rate = 4.0 + 14.0 * self.flick + 40.0 * self.burst;
        if self.rng.uni() < rate * dt && self.cr.len() < 12 {
            let pop = self.rng.chance(0.15);
            let f = if pop { 350.0 + 500.0 * self.rng.uni() } else { 1400.0 + 3600.0 * self.rng.uni() };
            let amp = (0.2 + 0.8 * self.rng.uni().powi(2)) * if pop { 1.2 } else { 0.7 };
            self.cr.push((Bp::new(f, if pop { 3.0 } else { 4.5 }, self.sr as f32), amp, if pop { 0.012 } else { 0.003 }, 0.7 * self.rng.bi()));
        }
        let (mut l, mut r) = (0.0f32, 0.0f32);
        for c in self.cr.iter_mut() {
            let v = c.0.run(self.rng.bi() * c.1);
            c.1 *= (-dt / c.2).exp();
            let (a, b) = pan2(v * 2.2, c.3);
            l += a;
            r += b;
        }
        self.cr.retain(|c| c.1 > 1e-3);
        let n = self.rng.bi();
        self.hp += 0.6 * (n - self.hp);
        let hiss = (n - self.hp) * 0.025 * self.flick;
        for ch in 0..2 {
            let w = self.rng.bi();
            self.lp[ch] += 0.004 * (w - self.lp[ch]);
            let roar = self.lp[ch] * 2.2 * (0.6 + 0.4 * self.flick);
            if ch == 0 { l += hiss + roar } else { r += hiss * 0.9 + roar }
        }
        (l * 1.1, r * 1.1)
    }
}

// ---------------------------------------------------------------- pre-rendered group chants

/// Plays group-chant loops rendered at build time. `spec` is "name" (every stage) or a clip per
/// stage, "a@0,b@1-3,c@5.7" (stages counted from 0; ranges with '-', lists with '.'); stages not
/// listed play the clip named without '@', or nothing. Every clip runs on one clock from the
/// start of the session, so a chant stays in step with a wooden fish, cymbals or a drum that has
/// the same period, and a new stage's clip crossfades in at the same point of the cycle.
pub struct Chant { clips: Vec<(Arc<Vec<f32>>, f64)>, map: Vec<(usize, Option<usize>)>, all: Option<usize>, cur: Option<usize>, prev: Option<usize>, x: f32, xs: f32, n: u64 }
impl Chant {
    fn new(spec: &str, sr: f64, clips: &[Clip]) -> Self {
        let mut c: Vec<(Arc<Vec<f32>>, f64)> = Vec::new();
        let mut names: Vec<String> = Vec::new();
        let mut idx = |n: &str, c: &mut Vec<(Arc<Vec<f32>>, f64)>| -> Option<usize> {
            if let Some(i) = names.iter().position(|x| x == n) { return Some(i); }
            let (_, d, r) = clips.iter().find(|x| x.0 == n)?;
            if d.len() < 8 { return None; }
            names.push(n.to_string());
            c.push((d.clone(), *r as f64 / sr));
            Some(c.len() - 1)
        };
        let mut map = Vec::new();
        let mut all = None;
        for part in spec.split(',') {
            let part = part.trim();
            if part.is_empty() { continue; }
            if let Some((n, st)) = part.split_once('@') {
                let i = idx(n.trim(), &mut c);
                for item in st.split('.') {
                    let (a, b) = match item.split_once('-') {
                        Some((a, b)) => (a.trim().parse::<usize>().ok(), b.trim().parse::<usize>().ok()),
                        None => { let v = item.trim().parse::<usize>().ok(); (v, v) }
                    };
                    if let (Some(a), Some(b)) = (a, b) { for s in a..=b.min(a + 99) { map.push((s, i)); } }
                }
            } else {
                all = idx(part, &mut c);
            }
        }
        let cur = map.iter().find(|x| x.0 == 0).map(|x| x.1).unwrap_or(all);
        Chant { clips: c, map, all, cur, prev: None, x: 0.0, xs: (1.0 / (1.5 * sr)) as f32, n: 0 }
    }
    fn set_stage(&mut self, k: usize) {
        if self.map.is_empty() { return; }
        let want = self.map.iter().find(|x| x.0 == k).map(|x| x.1).unwrap_or(self.all);
        if want != self.cur { self.prev = self.cur; self.cur = want; self.x = 0.0; }
    }
    /// Clip `ci` at the shared clock (linear interpolation, wrapping round the loop).
    #[inline]
    fn read(&self, ci: usize) -> (f32, f32) {
        let (d, step) = &self.clips[ci];
        let frames = d.len() / 2;
        if frames < 4 { return (0.0, 0.0); }
        let pos = (self.n as f64 * step) % frames as f64;
        let i = (pos as usize).min(frames - 1);
        let fr = (pos - i as f64) as f32;
        let j = if i + 1 >= frames { 0 } else { i + 1 };
        (d[i * 2] + (d[j * 2] - d[i * 2]) * fr, d[i * 2 + 1] + (d[j * 2 + 1] - d[i * 2 + 1]) * fr)
    }
    fn sample(&mut self) -> (f32, f32) {
        let (mut l, mut r) = (0.0f32, 0.0f32);
        if self.x < 1.0 {
            self.x = (self.x + self.xs).min(1.0);
            if let Some(p) = self.prev { let (a, b) = self.read(p); let g = 1.0 - self.x; l += a * g; r += b * g; }
        }
        if let Some(c) = self.cur { let (a, b) = self.read(c); l += a * self.x; r += b * self.x; }
        self.n += 1;
        (l * 0.9, r * 0.9)
    }
}
