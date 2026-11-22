//! Session data model shared by the preset library, the audio engine and the UI.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stage {
    pub name: String,
    /// Minutes. 0 = open-ended (custom generator).
    pub min: f64,
    /// Beat frequency per layer (Hz).
    pub beats: Vec<f64>,
    /// Optional per-stage carrier override (lets journeys glide between tones).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carriers: Option<Vec<f64>>,
    /// Seconds to glide from the previous stage (default: min(60, half the stage)).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ramp: Option<f64>,
    #[serde(default)]
    pub cue: String,
    #[serde(default)]
    pub cue_id: String,
    #[serde(default)]
    pub breath: bool,
    /// Texture gain for this stage (0 = off, default 1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tex: Option<f64>,
    /// Music (accompaniment) gain for this stage (0 = off, default 1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mus: Option<f64>,
}

/// Non-tonal sound layer synthesised by the engine.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Texture {
    /// "drum" | "bowls" | "kirtan" | "tlr" | "breathbell" | "pulses" | "rain"
    pub kind: String,
    #[serde(default = "d_one")]
    pub level: f64,
    #[serde(default)]
    pub root: f64,
    /// Seconds between events (bowls / tlr / breathbell).
    #[serde(default)]
    pub interval: f64,
    /// Variant or content for the accompaniment kinds (music.rs): a tanpura tuning, a flute's
    /// style and mode ("ney:hijaz"), a bell type, or the chant clip(s) to play ("mani", "lam@1,vam@2").
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
}
fn d_one() -> f64 { 1.0 }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Preset {
    pub id: String,
    pub group: String,
    pub name: String,
    pub short: String,
    pub carriers: Vec<f64>,
    pub gains: Vec<f64>,
    #[serde(default = "d_noise")]
    pub noise: String,
    /// "binaural" | "monaural" | "isochronic"
    #[serde(default = "d_mode")]
    pub mode: String,
    /// "Gateway" | "Research" | "Traditional" | "Speculative"
    #[serde(default)]
    pub evidence: String,
    #[serde(default)]
    pub about: String,
    pub stages: Vec<Stage>,
    #[serde(default)]
    pub live: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<Texture>,
    /// Accompaniment a practitioner would hear (tanpura, chant, flute, bells…), see music.rs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub music: Option<Texture>,
    /// "" | "ganzfeld" | "flicker" | "ganzflicker" (flicker = opt-in Light journey)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub visual: String,
    /// Search keywords (generated in presets::auto_tags).
    #[serde(default)]
    pub tags: Vec<String>,
    /// Spiritual traditions the session belongs to (keys for the Tradition filter).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trad: Vec<String>,
    /// Long-form explanation (filled in by info::fill).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<Info>,
}

/// In-depth background shown in the session's info card.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Info {
    pub what: String,
    pub origin: String,
    pub era: String,
    pub claims: String,
    pub research: String,
}
fn d_noise() -> String { "pink".into() }
fn d_mode() -> String { "binaural".into() }

/// FNV-1a 64 — stable id for a cue text (used for the voice file names).
pub fn cue_id(text: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in text.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", h)
}

/// Timeline model used by the engine (seconds, interpolation).
pub struct Timeline {
    pub base_carriers: Vec<f64>,
    pub gains: Vec<f64>,
    pub stages: Vec<Stage>,
    pub starts: Vec<f64>,
    pub total: f64, // f64::INFINITY when open-ended
    pub live: bool,
    pub texture: Option<Texture>,
    pub music: Option<Texture>,
}

impl Preset {
    /// Clamp a (possibly user-supplied) plan into safe, sane bounds.
    pub fn sanitize(mut self) -> Option<Self> {
        if self.carriers.is_empty() || self.stages.is_empty() { return None; }
        self.carriers.truncate(8);
        let n = self.carriers.len();
        for c in self.carriers.iter_mut() { *c = if c.is_finite() { c.clamp(20.0, 1500.0) } else { 200.0 }; }
        self.gains.resize(n, 0.5);
        for g in self.gains.iter_mut() { *g = if g.is_finite() { g.clamp(0.0, 1.0) } else { 0.5 }; }
        self.stages.truncate(64);
        for s in self.stages.iter_mut() {
            s.min = if s.min.is_finite() { s.min.clamp(0.0, 600.0) } else { 0.0 };
            s.beats.resize(n, 0.0);
            for b in s.beats.iter_mut() { *b = if b.is_finite() { b.clamp(0.0, 250.0) } else { 0.0 }; }
            if let Some(cs) = s.carriers.as_mut() {
                cs.resize(n, 200.0);
                for c in cs.iter_mut() { *c = if c.is_finite() { c.clamp(20.0, 1500.0) } else { 200.0 }; }
            }
            s.ramp = s.ramp.map(|r| if r.is_finite() { r.clamp(0.0, 3600.0) } else { 60.0 });
            s.tex = s.tex.map(|x| if x.is_finite() { x.clamp(0.0, 1.0) } else { 1.0 });
            s.mus = s.mus.map(|x| if x.is_finite() { x.clamp(0.0, 1.0) } else { 1.0 });
        }
        for tx in [self.texture.as_mut(), self.music.as_mut()].into_iter().flatten() {
            tx.level = if tx.level.is_finite() { tx.level.clamp(0.0, 1.0) } else { 1.0 };
            tx.root = if tx.root.is_finite() { tx.root.clamp(0.0, 2000.0) } else { 0.0 };
            tx.interval = if tx.interval.is_finite() { tx.interval.clamp(0.0, 600.0) } else { 0.0 };
            tx.name.truncate(200);
        }
        Some(self)
    }
}

impl Timeline {
    pub fn from_preset(p: &Preset) -> Self {
        let mut starts = Vec::new();
        let mut t = 0.0;
        for s in &p.stages {
            starts.push(t);
            t += if s.min <= 0.0 { f64::INFINITY } else { s.min * 60.0 };
        }
        Timeline {
            base_carriers: p.carriers.clone(),
            gains: p.gains.clone(),
            stages: p.stages.clone(),
            starts,
            total: t,
            live: p.live,
            texture: p.texture.clone(),
            music: p.music.clone(),
        }
    }
    pub fn layers(&self) -> usize { self.base_carriers.len() }
    /// Texture gain of the stage at elapsed `e` (default 1).
    pub fn tex_at(&self, e: f64) -> f64 { self.stages[self.stage_at(e)].tex.unwrap_or(1.0) }
    /// Music gain of the stage at elapsed `e` (default 1).
    pub fn mus_at(&self, e: f64) -> f64 { self.stages[self.stage_at(e)].mus.unwrap_or(1.0) }
    fn dur(&self, k: usize) -> f64 {
        let m = self.stages[k].min;
        if m <= 0.0 { f64::INFINITY } else { m * 60.0 }
    }
    pub fn ramp(&self, k: usize) -> f64 {
        if k == 0 { return 0.0; }
        let s = &self.stages[k];
        s.ramp.unwrap_or(60.0).min(self.dur(k) * 0.5)
    }
    pub fn stage_at(&self, e: f64) -> usize {
        let mut k = 0;
        for (i, st) in self.starts.iter().enumerate() {
            if e >= *st { k = i; }
        }
        k
    }
    fn carriers_of(&self, k: usize) -> &[f64] {
        self.stages[k].carriers.as_deref().unwrap_or(&self.base_carriers)
    }
    /// (beats, carriers) at elapsed seconds `e`, gliding linearly between stages.
    pub fn at(&self, e: f64) -> (Vec<f64>, Vec<f64>) {
        let k = self.stage_at(e);
        let s = &self.stages[k];
        let pk = if k > 0 { k - 1 } else { 0 };
        let pb = &self.stages[pk].beats;
        let pc = self.carriers_of(pk);
        let cc = self.carriers_of(k);
        let r = self.ramp(k);
        let f = if r > 0.0 { ((e - self.starts[k]) / r).clamp(0.0, 1.0) } else { 1.0 };
        let n = self.layers();
        let mut beats = Vec::with_capacity(n);
        let mut cars = Vec::with_capacity(n);
        for i in 0..n {
            let b1 = s.beats.get(i).copied().unwrap_or(0.0);
            let b0 = pb.get(i).copied().unwrap_or(b1);
            let c1 = cc.get(i).copied().unwrap_or(200.0);
            let c0 = pc.get(i).copied().unwrap_or(c1);
            beats.push(b0 + (b1 - b0) * f);
            cars.push(c0 + (c1 - c0) * f);
        }
        (beats, cars)
    }
}
