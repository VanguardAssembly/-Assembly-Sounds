//! Non-tonal "texture" layers used by the altered-state sessions:
//!   drum        – frame-drum hits at the beat rate (rhythmic trance; Hove et al. 2016)
//!   bowls       – struck singing bowls whose paired partials beat at the stage beat (Ahn et al. 2023)
//!   kirtan      – 4-note cycle, one note per second (Sa-Ta-Na-Ma; Innes et al. 2017)
//!   tlr         – 3-note lucidity cue every `interval` s (Carr et al. 2023)
//!   breathbell  – soft bell every `interval` s marking a slow breath (Bernardi et al. 2001)
//!   pulses      – 50 ms pink-noise bursts at the beat rate (≈0.8 Hz; Ngo et al. 2013)
//!   rain        – synthesised rain bed with droplets (natural soundscape; Gould van Praag 2017)
//!   mokugyo     – wooden-fish knock every `interval` s (Buddhist chanting: nembutsu, daimoku)
//!   rattle      – gourd-rattle shake every `interval` s (≈210 a minute, as in Goodman's posture trance)
//!   gong        – large gong struck every `interval` s, with a long shimmering decay
//!   heartbeat   – soft lub-dub every `interval` s (≈60 a minute)
//!   kartal      – hand-cymbal pattern ching-ching-chik (kirtan), one stroke every `interval` s
//! …and every accompaniment kind in music.rs (tanpura, chant, flutes, bells, nature…), which can
//! be used as a preset's texture or as its music layer.
use crate::music::{Clip, Inst};
use crate::plan::Texture;
use std::f64::consts::TAU;

#[derive(Clone, Copy, PartialEq)]
enum Kind { Drum, Bowls, Kirtan, Tlr, BreathBell, Pulses, Rain, Mokugyo, Rattle, Gong, Heartbeat, Kartal, Inst }

/// Exponentially decaying sinusoid.
struct Partial { f: f64, ph: f64, amp: f32, decay: f32, attack: f32, env: f32, pl: f32, pr: f32 }

struct Hit { t: f64, amp: f32, pan: f32 }

pub struct TexSynth {
    kind: Kind,
    level: f32,
    root: f64,
    interval: f64,
    sr: f64,
    clock: f64,       // seconds since last event
    step: usize,      // event counter
    partials: Vec<Partial>,
    hits: Vec<Hit>,
    queue: Vec<(f64, f64)>, // (delay s, freq) pending motif notes
    rng: u64,
    pink: [f32; 7],
    lp: [f32; 2],
    burst: f64,       // remaining burst time (pulses)
    next_jitter: f64,
    inst: Option<Inst>,
    now: u64,         // samples since the start
    next_k: u64,      // grid index of the next periodic event
}

impl TexSynth {
    #[allow(dead_code)]
    pub fn new(t: &Texture, sr: f64) -> Option<Self> { Self::with_clips(t, sr, &[]) }

    /// `clips`: decoded chant clips the "chant" kind may need.
    pub fn with_clips(t: &Texture, sr: f64, clips: &[Clip]) -> Option<Self> {
        let mut inst = None;
        let kind = match t.kind.as_str() {
            "drum" => Kind::Drum, "bowls" => Kind::Bowls, "kirtan" => Kind::Kirtan, "tlr" => Kind::Tlr,
            "breathbell" => Kind::BreathBell, "pulses" => Kind::Pulses, "rain" => Kind::Rain,
            "mokugyo" => Kind::Mokugyo, "rattle" => Kind::Rattle, "gong" => Kind::Gong,
            "heartbeat" => Kind::Heartbeat, "kartal" => Kind::Kartal,
            other => { inst = Some(Inst::new(other, &t.name, t.root, t.interval, sr, clips)?); Kind::Inst }
        };
        let root = if t.root > 0.0 { t.root } else {
            match kind {
                Kind::Bowls => 220.0, Kind::Kirtan => 392.0, Kind::Tlr => 660.0, Kind::BreathBell => 528.0,
                Kind::Mokugyo => 460.0, Kind::Gong => 98.0, Kind::Heartbeat => 52.0, Kind::Kartal => 2350.0, _ => 0.0,
            }
        };
        let interval = if t.interval > 0.0 { t.interval } else {
            match kind {
                Kind::Bowls => 9.0, Kind::Tlr => 180.0, Kind::BreathBell => 10.0, Kind::Mokugyo => 0.75,
                Kind::Rattle => 60.0 / 210.0, Kind::Gong => 30.0, Kind::Heartbeat => 1.0, Kind::Kartal => 0.42, _ => 1.0,
            }
        };
        Some(TexSynth {
            kind, level: t.level as f32, root, interval, sr,
            clock: 1e9, step: 0, partials: Vec::new(), hits: Vec::new(), queue: Vec::new(),
            rng: 0x2545F4914F6CDD1D, pink: [0.0; 7], lp: [0.0; 2], burst: 0.0, next_jitter: 0.0, inst,
            now: 0, next_k: 0,
        })
    }

    /// Current stage of the session (stage bells and per-stage chants follow it).
    pub fn set_stage(&mut self, k: usize, n: usize) {
        if let Some(i) = self.inst.as_mut() { i.set_stage(k, n); }
    }

    fn rand(&mut self) -> f32 {
        let mut x = self.rng;
        x ^= x << 13; x ^= x >> 7; x ^= x << 17;
        self.rng = x;
        ((x >> 40) as f32 / (1u64 << 24) as f32) * 2.0 - 1.0
    }
    fn pink(&mut self) -> f32 {
        let w = self.rand();
        let b = &mut self.pink;
        b[0] = 0.99886 * b[0] + w * 0.0555179; b[1] = 0.99332 * b[1] + w * 0.0750759;
        b[2] = 0.96900 * b[2] + w * 0.1538520; b[3] = 0.86650 * b[3] + w * 0.3104856;
        b[4] = 0.55000 * b[4] + w * 0.5329522; b[5] = -0.7616 * b[5] - w * 0.0168980;
        let v = (b[0] + b[1] + b[2] + b[3] + b[4] + b[5] + b[6] + w * 0.5362) * 0.11;
        b[6] = w * 0.115926;
        v
    }
    /// Periodic events sit on a fixed grid from the start of the session (k × period), so two
    /// layers with the same period (a wooden fish and a chant) never drift apart. While the layer
    /// is silent its grid points are skipped, not saved up.
    fn due(&mut self, active: bool, period: f64) -> bool {
        let p = (period * self.sr).max(1.0);
        let now = self.now as f64;
        if now + 0.5 < self.next_k as f64 * p { return false; }
        self.next_k = ((now + 0.5) / p).floor() as u64 + 1;
        active
    }
    fn decay_for(&self, secs: f64) -> f32 { (-1.0 / (secs * self.sr)).exp() as f32 }

    fn add(&mut self, f: f64, amp: f32, secs: f64, pan: f32) {
        let decay = self.decay_for(secs);
        let attack = 1.0 / (0.006 * self.sr as f32);
        self.partials.push(Partial { f, ph: 0.0, amp, decay, attack, env: 0.0, pl: (1.0 - pan).min(1.0), pr: (1.0 + pan).min(1.0) });
    }
    fn bell(&mut self, f: f64, amp: f32, secs: f64) {
        self.add(f, amp, secs, 0.0);
        self.add(f * 2.76, amp * 0.22, secs * 0.35, 0.0);
        self.add(f * 5.40, amp * 0.06, secs * 0.15, 0.0);
    }
    fn strike_bowl(&mut self, f0: f64, beat: f64, pan: f32) {
        // Singing-bowl mode ratios with a detuned twin per mode -> acoustic beating at `beat` Hz.
        const MODES: [(f64, f32, f64); 4] = [(1.0, 1.0, 22.0), (2.71, 0.45, 13.0), (5.33, 0.2, 7.0), (8.69, 0.08, 4.0)];
        let b = beat.clamp(0.2, 12.0);
        for (r, a, d) in MODES {
            self.add(f0 * r - b / 2.0, a * 0.5, d, pan);
            self.add(f0 * r + b / 2.0, a * 0.5, d, pan);
        }
        self.partials.retain(|p| p.amp > 1e-4); // cap growth
        if self.partials.len() > 96 { let n = self.partials.len() - 96; self.partials.drain(0..n); }
    }

    /// A queued note: a soft heart sound for the heartbeat, a bell for everything else.
    fn fire(&mut self, f: f64) {
        if self.kind == Kind::Heartbeat {
            self.add(f, 0.9, 0.11, 0.0);
            self.add(f * 2.0, 0.25, 0.05, 0.0);
            self.add(f * 3.0, 0.08, 0.03, 0.0);
            self.burst = 0.006;
        } else {
            self.bell(f, 0.5, 1.2);
        }
    }

    /// One stereo sample. `beat` = current layer-0 beat (Hz), `gain` = texture gain (0..1).
    pub fn sample(&mut self, beat: f64, gain: f32) -> (f32, f32) {
        if let Some(i) = self.inst.as_mut() {
            let (l, r) = i.sample(gain);
            return (l * self.level, r * self.level);
        }
        let dt = 1.0 / self.sr;
        self.clock += dt;
        let (mut l, mut r) = (0.0f32, 0.0f32);
        let active = gain > 1e-4;

        // ---------- event scheduling ----------
        match self.kind {
            Kind::Drum => {
                let period = 1.0 / beat.clamp(0.5, 12.0);
                if active && self.clock >= period + self.next_jitter {
                    self.clock = 0.0;
                    self.step += 1;
                    let accent = if self.step % 4 == 1 { 1.0 } else { 0.8 };
                    let amp = accent * (0.9 + 0.1 * self.rand());
                    let pan = 0.15 * self.rand();
                    self.hits.push(Hit { t: 0.0, amp, pan });
                    self.next_jitter = period * 0.015 * self.rand() as f64; // human feel
                    if self.hits.len() > 8 { self.hits.remove(0); }
                }
            }
            Kind::Bowls => {
                if self.due(active, self.interval) {
                    self.clock = 0.0;
                    const ROOTS: [f64; 4] = [1.0, 1.5, 0.75, 1.335];
                    let f0 = self.root * ROOTS[self.step % 4];
                    let pan = [0.0f32, -0.35, 0.35, 0.0][self.step % 4];
                    self.step += 1;
                    self.strike_bowl(f0, beat, pan);
                }
            }
            Kind::Kirtan => {
                if self.due(active, 1.0) {
                    self.clock = 0.0;
                    const NOTES: [f64; 4] = [1.5, 4.0 / 3.0, 1.25, 1.0]; // Sa Ta Na Ma
                    let f = self.root * NOTES[self.step % 4];
                    self.step += 1;
                    self.bell(f, 0.55, 0.9);
                }
            }
            Kind::Tlr => {
                if self.due(active, self.interval) {
                    self.clock = 0.0;
                    let f = self.root;
                    self.queue.push((0.0, f));
                    self.queue.push((0.28, f * 1.25));
                    self.queue.push((0.56, f * 1.5));
                }
            }
            Kind::BreathBell => {
                if self.due(active, self.interval) {
                    self.clock = 0.0;
                    let f = self.root;
                    self.bell(f, 0.45, 3.5);
                }
            }
            Kind::Pulses => {
                let period = 1.0 / beat.clamp(0.3, 4.0);
                if active && self.clock >= period {
                    self.clock = 0.0;
                    self.burst = 0.05;
                }
            }
            Kind::Rain => {}
            Kind::Mokugyo => {
                if self.due(active, self.interval) {
                    self.clock = 0.0;
                    self.step += 1;
                    let a = (if self.step % 4 == 1 { 1.0 } else { 0.78 }) * (0.92 + 0.08 * self.rand());
                    let f = self.root * (1.0 + 0.004 * self.rand() as f64);
                    self.add(f, 0.62 * a, 0.075, 0.0);
                    self.add(f * 2.71, 0.2 * a, 0.03, 0.0);
                    self.add(f * 0.5, 0.18 * a, 0.05, 0.0);
                    self.burst = 0.004;
                }
            }
            Kind::Rattle => {
                if active && self.clock >= self.interval + self.next_jitter {
                    self.clock = 0.0;
                    self.burst = 0.055;
                    self.next_jitter = self.interval * 0.03 * self.rand() as f64; // human feel
                }
            }
            Kind::Gong => {
                if self.due(active, self.interval) {
                    self.clock = 0.0;
                    let f0 = self.root;
                    // inharmonic gong modes, each with a slightly detuned twin for the shimmer
                    const P: [(f64, f32, f64); 8] = [(1.0, 1.0, 16.0), (1.48, 0.55, 12.0), (2.01, 0.45, 10.0), (2.47, 0.3, 8.0),
                                                     (2.96, 0.26, 6.5), (3.62, 0.18, 5.0), (4.41, 0.12, 4.0), (5.33, 0.08, 3.0)];
                    for (r, a, d) in P {
                        self.add(f0 * r - 0.35, a * 0.25, d, -0.2);
                        self.add(f0 * r + 0.35, a * 0.25, d, 0.2);
                    }
                    if self.partials.len() > 96 { let n = self.partials.len() - 96; self.partials.drain(0..n); }
                }
            }
            Kind::Heartbeat => {
                if self.due(active, self.interval) {
                    self.clock = 0.0;
                    let f = self.root;
                    self.queue.push((0.0, f));
                    self.queue.push((0.3, f * 1.2));
                }
            }
            Kind::Inst => {}
            Kind::Kartal => {
                if self.due(active, self.interval) {
                    self.clock = 0.0;
                    let s = self.step % 4; // ching, ching, chik, rest
                    self.step += 1;
                    if s < 3 {
                        let (secs, amp) = if s == 2 { (0.07, 0.2) } else { (0.6, 0.14) };
                        let f0 = self.root;
                        for (r, a) in [(1.0, 1.0f32), (1.45, 0.7), (2.07, 0.45), (2.65, 0.3)] {
                            self.add(f0 * r - 3.0, amp * a, secs, -0.1);
                            self.add(f0 * r + 3.0, amp * a, secs, 0.1);
                        }
                        self.burst = 0.003;
                    }
                }
            }
        }
        self.now += 1;
        // queued notes (tlr motif, heartbeat)
        if !self.queue.is_empty() {
            let mut k = 0;
            while k < self.queue.len() {
                self.queue[k].0 -= dt;
                if self.queue[k].0 <= 0.0 { let f = self.queue.swap_remove(k).1; self.fire(f); } else { k += 1; }
            }
        }

        // ---------- synthesis ----------
        match self.kind {
            Kind::Drum => {
                let sr = self.sr;
                let mut out = (0.0f32, 0.0f32);
                let mut hits = std::mem::take(&mut self.hits);
                for h in hits.iter_mut() {
                    let t = h.t;
                    // pitch-swept membrane + short skin noise
                    let f = 58.0 + 75.0 * (-t / 0.035).exp();
                    let body = ((TAU * f * t) + 0.0).sin() as f32 * (-t / 0.22).exp() as f32;
                    let skin = self.rand() * (-t / 0.012).exp() as f32 * 0.35;
                    let v = (body + skin) * h.amp;
                    out.0 += v * (1.0 - h.pan);
                    out.1 += v * (1.0 + h.pan);
                    h.t += 1.0 / sr;
                }
                hits.retain(|h| h.t < 1.2);
                self.hits = hits;
                l += out.0 * 0.55;
                r += out.1 * 0.55;
            }
            Kind::Pulses => {
                if self.burst > 0.0 {
                    let edge = (self.burst.min(0.05 - self.burst) / 0.005).clamp(0.0, 1.0) as f32;
                    let v = self.pink() * edge * 1.4;
                    l += v; r += v;
                    self.burst -= dt;
                }
            }
            Kind::Rain => {
                // bed: two decorrelated low-passed noises
                for ch in 0..2 {
                    let w = self.rand();
                    self.lp[ch] += 0.18 * (w - self.lp[ch]);
                    let v = self.lp[ch] * 0.22;
                    if ch == 0 { l += v } else { r += v }
                }
                // droplets: Poisson ~30/s, tiny decaying chirps
                if (self.rand() * 0.5 + 0.5) < (30.0 * dt) as f32 {
                    let f = 1800.0 + 2600.0 * (self.rand() * 0.5 + 0.5) as f64;
                    let pan = self.rand() * 0.8;
                    let amp = 0.08 + 0.12 * (self.rand() * 0.5 + 0.5);
                    self.add(f, amp, 0.012, pan);
                    if self.partials.len() > 64 { self.partials.remove(0); }
                }
            }
            Kind::Rattle => {
                if self.burst > 0.0 {
                    // seeds on gourd: bright, slightly grainy noise with a fast attack and quick decay
                    let x = (self.burst / 0.055) as f32;
                    let env = x * x * (1.0 - (-((0.055 - self.burst) / 0.002) as f32).exp());
                    let (wl, wr) = (self.rand(), self.rand());
                    self.lp[0] += 0.35 * (wl - self.lp[0]);
                    self.lp[1] += 0.35 * (wr - self.lp[1]);
                    l += (wl - self.lp[0]) * env * 0.5;
                    r += (wr - self.lp[1]) * env * 0.5;
                    self.burst -= dt;
                }
            }
            Kind::Mokugyo | Kind::Kartal | Kind::Heartbeat => {
                if self.burst > 0.0 {
                    let v = self.rand() * 0.12; // contact click
                    l += v;
                    r += v;
                    self.burst -= dt;
                }
            }
            _ => {}
        }
        // decaying partials (bowls, bells, droplets)
        for p in self.partials.iter_mut() {
            p.ph = (p.ph + TAU * p.f * dt) % TAU;
            if p.env < 1.0 { p.env = (p.env + p.attack).min(1.0); }
            let v = p.ph.sin() as f32 * p.amp * p.env;
            l += v * p.pl;
            r += v * p.pr;
            p.amp *= p.decay;
        }
        if self.partials.len() > 8 { self.partials.retain(|p| p.amp > 1e-4); }
        (l * gain * self.level, r * gain * self.level)
    }
}
