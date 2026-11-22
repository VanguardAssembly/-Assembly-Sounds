//! Natural-voice cues: neural (Kokoro-82M) recordings rendered at build time,
//! embedded in the exe as Ogg Vorbis and decoded here on demand.
use include_dir::{include_dir, Dir};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

static VOICE: Dir = include_dir!("$CARGO_MANIFEST_DIR/voice");
static CHANT: Dir = include_dir!("$CARGO_MANIFEST_DIR/chant");

pub fn available() -> Vec<String> {
    let mut v: Vec<String> = VOICE
        .dirs()
        .filter_map(|d| d.path().file_name().map(|n| n.to_string_lossy().to_string()))
        .collect();
    v.sort();
    v
}

#[derive(Default)]
pub struct Cache(HashMap<String, (Arc<Vec<f32>>, u32)>);

impl Cache {
    pub fn get(&mut self, voice: &str, id: &str) -> Option<(Arc<Vec<f32>>, u32)> {
        if id.is_empty() || voice.contains(['/', '\\', '.']) { return None; }
        let key = format!("{voice}/{id}.ogg");
        if let Some(c) = self.0.get(&key) { return Some(c.clone()); }
        let file = VOICE.get_file(&key)?;
        let mut rd = lewton::inside_ogg::OggStreamReader::new(Cursor::new(file.contents())).ok()?;
        let sr = rd.ident_hdr.audio_sample_rate;
        let ch = (rd.ident_hdr.audio_channels as usize).max(1);
        let mut out = Vec::new();
        while let Ok(Some(pk)) = rd.read_dec_packet_itl() {
            for frame in pk.chunks(ch) {
                out.push(frame.iter().map(|&s| s as f32).sum::<f32>() / (ch as f32 * 32768.0));
            }
        }
        let clip = (Arc::new(out), sr);
        self.0.insert(key, clip.clone());
        Some(clip)
    }
}

/// Group-chant clips (chant/<name>.ogg, stereo loops), decoded on demand.
pub fn chant_clip(name: &str) -> Option<(Arc<Vec<f32>>, u32)> {
    if name.is_empty() || name.contains(['/', '\\', '.']) { return None; }
    let file = CHANT.get_file(format!("{name}.ogg"))?;
    let mut rd = lewton::inside_ogg::OggStreamReader::new(Cursor::new(file.contents())).ok()?;
    let sr = rd.ident_hdr.audio_sample_rate;
    let ch = (rd.ident_hdr.audio_channels as usize).max(1);
    let mut out = Vec::new();
    while let Ok(Some(pk)) = rd.read_dec_packet_itl() {
        for frame in pk.chunks(ch) {
            let l = frame[0] as f32 / 32768.0;
            let r = if ch > 1 { frame[1] as f32 / 32768.0 } else { l };
            out.push(l);
            out.push(r);
        }
    }
    Some((Arc::new(out), sr))
}

impl Cache {
    /// A chant clip, decoded once and kept.
    pub fn chant(&mut self, name: &str) -> Option<(Arc<Vec<f32>>, u32)> {
        let key = format!("chant/{name}");
        if let Some(c) = self.0.get(&key) { return Some(c.clone()); }
        let c = chant_clip(name)?;
        self.0.insert(key, c.clone());
        Some(c)
    }
    /// Every chant clip a preset's texture and music need, ready for the engine.
    pub fn clips_for(&mut self, p: &crate::plan::Preset) -> Vec<crate::music::Clip> {
        let mut v = Vec::new();
        for t in [p.texture.as_ref(), p.music.as_ref()].into_iter().flatten() {
            for n in crate::music::Inst::clip_names(&t.kind, &t.name) {
                if let Some((d, sr)) = self.chant(&n) { v.push((n, d, sr)); }
            }
        }
        v
    }
}
