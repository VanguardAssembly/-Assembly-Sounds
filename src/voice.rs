//! Natural-voice cues: neural (Kokoro-82M) recordings rendered at build time,
//! embedded in the exe as Ogg Vorbis and decoded here on demand.
use include_dir::{include_dir, Dir};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

static VOICE: Dir = include_dir!("$CARGO_MANIFEST_DIR/voice");

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
