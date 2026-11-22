#![windows_subsystem = "windows"]
//! Focus — Binaural Beats Generator.
//! Native Rust app: audio engine (cpal), session library and voice playback are all Rust.
//! The window is a system WebView2 rendering the bundled UI; it has no network access
//! (strict Content-Security-Policy) and talks to Rust only through a JSON IPC channel.
mod engine;
mod info;
mod plan;
mod presets;
mod texture;
mod voice;

use engine::{Mode, Noise, Shared};
use plan::{Preset, Timeline};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::window::WindowBuilder;
use wry::{http::Request, WebContext, WebViewBuilder};

enum UserEvent {
    Ipc(String),
}

struct App {
    presets: Vec<Preset>,
    timeline: Option<Arc<Timeline>>,
    generation: u64,
    last_stage: usize,
    voice: String,
    cache: voice::Cache,
    /// FOCUS_SELFTEST=<report path>: drive the real UI at zero volume, write a report, exit.
    selftest: Option<(String, Option<Instant>)>,
    /// User settings (voice, levels, last session…) saved by the UI.
    settings_path: std::path::PathBuf,
    quit: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "--dump-cues" {
        dump_cues(&args[2]);
        return;
    }
    // --render <preset_id> <mode> <noise> <start_s> <secs> <out.wav>   (self-test)
    if args.len() >= 8 && args[1] == "--render" {
        if let Some(p) = presets::all().into_iter().find(|p| p.id == args[2]) {
            let tl = Arc::new(Timeline::from_preset(&p));
            let pcm = engine::render_offline(tl, Mode::parse(&args[3]), Noise::parse(&args[4]),
                args[5].parse().unwrap_or(0.0), args[6].parse().unwrap_or(5.0), 48000);
            write_wav(&args[7], &pcm, 48000);
        }
        return;
    }
    // --check-voices <out.txt>: decode every cue in every voice (self-test)
    if args.len() >= 3 && args[1] == "--check-voices" {
        let mut cache = voice::Cache::default();
        let (mut ok, mut bad, mut secs) = (0, Vec::new(), 0.0);
        let mut ids: Vec<String> = presets::all().into_iter().flat_map(|p| p.stages).map(|s| s.cue_id).filter(|i| !i.is_empty()).collect();
        ids.sort();
        ids.dedup();
        for v in voice::available() {
            for id in &ids {
                match cache.get(&v, id) {
                    Some((d, sr)) if !d.is_empty() => { ok += 1; secs += d.len() as f64 / sr as f64; }
                    _ => bad.push(format!("{v}/{id}")),
                }
            }
        }
        let _ = std::fs::write(&args[2], format!("decoded={ok} missing={} total_audio_s={secs:.0} {:?}", bad.len(), bad));
        return;
    }
    if args.len() >= 3 && args[1] == "--dump-presets" {
        let _ = std::fs::write(&args[2], serde_json::to_string(&presets::all()).unwrap());
        return;
    }

    let presets = presets::all();
    let shared = Arc::new(Mutex::new(Shared::default()));
    let engine = engine::start(shared.clone());
    let audio_error = engine.as_ref().err().cloned();

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("Assembly Sounds — Binaural Beats Generator")
        .with_inner_size(LogicalSize::new(1320.0, 900.0))
        .with_min_inner_size(LogicalSize::new(900.0, 600.0))
        .build(&event_loop)
        .expect("window");

    // Keep WebView2's profile in %LOCALAPPDATA% (not next to the exe).
    let data_dir = std::env::var_os("LOCALAPPDATA")
        .map(|p| std::path::PathBuf::from(p).join("FocusBinaural"))
        .unwrap_or_else(std::env::temp_dir);
    let settings_path = data_dir.join("settings.json");
    let mut ctx = WebContext::new(Some(data_dir));
    let webview = WebViewBuilder::new_with_web_context(&mut ctx)
        .with_html(include_str!("../ui/index.html"))
        .with_ipc_handler(move |req: Request<String>| {
            let _ = proxy.send_event(UserEvent::Ipc(req.body().clone()));
        })
        .build(&window)
        .expect("webview");

    let voices = voice::available();
    let mut app = App {
        presets,
        timeline: None,
        generation: 0,
        last_stage: usize::MAX,
        voice: voices.iter().find(|v| *v == "af_heart").or(voices.first()).cloned().unwrap_or_default(),
        cache: voice::Cache::default(),
        selftest: std::env::var("FOCUS_SELFTEST").ok().map(|p| (p, None)),
        settings_path,
        quit: false,
    };
    let mut next_tick = Instant::now();

    event_loop.run(move |event, _, cf| {
        let _keep_audio_alive = &engine;
        match event {
            Event::UserEvent(UserEvent::Ipc(msg)) => {
                if let Ok(v) = serde_json::from_str::<Value>(&msg) {
                    handle(&v, &mut app, &shared, &webview, &voices, &audio_error);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *cf = ControlFlow::Exit;
                return;
            }
            Event::MainEventsCleared if app.quit => {
                *cf = ControlFlow::Exit;
                return;
            }
            Event::MainEventsCleared => {
                if Instant::now() >= next_tick {
                    tick(&mut app, &shared, &webview);
                    if let Some((_, Some(t0))) = &app.selftest {
                        if t0.elapsed() > Duration::from_secs(5) {
                            app.selftest.as_mut().unwrap().1 = None;
                            let _ = webview.evaluate_script("send({cmd:'report',stName:$('stName').textContent,clock:$('clock').textContent,btn:$('play').textContent,chips:$('chips').textContent,presets:PRESETS.length,voice:$('voiceSel').value})");
                        }
                    }
                    next_tick = Instant::now() + Duration::from_millis(33);
                }
            }
            _ => {}
        }
        *cf = ControlFlow::WaitUntil(next_tick);
    });
}

fn f(v: &Value, k: &str, d: f64) -> f64 { v.get(k).and_then(|x| x.as_f64()).filter(|x| x.is_finite()).unwrap_or(d) }
fn s<'a>(v: &'a Value, k: &str) -> &'a str { v.get(k).and_then(|x| x.as_str()).unwrap_or("") }

fn handle(v: &Value, app: &mut App, shared: &Arc<Mutex<Shared>>, wv: &wry::WebView, voices: &[String], audio_error: &Option<String>) {
    let mut sh = shared.lock().unwrap();
    match s(v, "cmd") {
        "ready" => {
            // The self-test always starts from defaults so it is repeatable.
            let settings = if app.selftest.is_some() { json!({}) } else { load_settings(&app.settings_path) };
            let init = json!({ "presets": app.presets, "voices": voices, "voice": app.voice, "audioError": audio_error, "settings": settings });
            drop(sh);
            let _ = wv.evaluate_script(&format!("window.__init({})", init));
            if let Some(st) = app.selftest.as_mut() {
                st.1 = Some(Instant::now());
                let _ = wv.evaluate_script("$('vol').value=0;sendLevels();$('play').click();");
            }
        }
        "report" if app.selftest.is_some() => {
            let rep = json!({ "ui": v, "engine": { "running": sh.running, "elapsed": sh.elapsed, "vol": sh.vol, "voice_pending_or_playing": app.last_stage != usize::MAX }, "audioError": audio_error, "voices": voices });
            let _ = std::fs::write(&app.selftest.as_ref().unwrap().0, serde_json::to_string_pretty(&rep).unwrap());
            sh.stop = true;
            app.quit = true;
        }
        "start" => {
            let preset = if let Some(p) = v.get("plan") {
                serde_json::from_value::<Preset>(p.clone()).ok().and_then(|p| p.sanitize())
            } else {
                let id = s(v, "id");
                app.presets.iter().find(|p| p.id == id).cloned()
            };
            if let Some(p) = preset {
                let tl = Arc::new(Timeline::from_preset(&p));
                sh.mode = Mode::parse(s(v, "mode"));
                sh.noise_type = Noise::parse(s(v, "noise"));
                sh.live = if p.live { Some((p.carriers[0], p.stages[0].beats[0])) } else { None };
                sh.timeline = Some(tl.clone());
                sh.generation += 1;
                sh.paused = false;
                sh.stop = false;
                sh.seek = None;
                sh.voice = None;
                sh.voice_cancel = true;
                app.generation = sh.generation;
                app.timeline = Some(tl);
                app.last_stage = usize::MAX;
            }
        }
        "stop" => {
            sh.stop = true;
            sh.voice_cancel = true;
        }
        "pause" => sh.paused = true,
        "resume" => sh.paused = false,
        "seek" => {
            if let Some(tl) = &app.timeline {
                if tl.total.is_finite() { sh.seek = Some(f(v, "t", 0.0).clamp(0.0, tl.total - 0.5)); sh.voice_cancel = true; }
            }
        }
        "jump" => {
            if let Some(tl) = &app.timeline {
                let e = sh.elapsed;
                let k = tl.stage_at(e) as i64;
                let dir = f(v, "dir", 1.0);
                let nk = if dir > 0.0 { k + 1 } else if e - tl.starts[k as usize] > 5.0 { k } else { k - 1 };
                let nk = nk.clamp(0, tl.stages.len() as i64 - 1) as usize;
                sh.seek = Some(tl.starts[nk]);
                sh.voice_cancel = true;
            }
        }
        "levels" => {
            sh.vol = f(v, "vol", 0.35) as f32;
            sh.tone = f(v, "tone", 0.6) as f32;
            sh.noise = f(v, "noise", 0.35) as f32;
            sh.voice_vol = f(v, "voiceVol", 0.8) as f32;
            sh.tex = f(v, "tex", 0.7) as f32;
        }
        "noise" => sh.noise_type = Noise::parse(s(v, "type")),
        "mode" => sh.mode = Mode::parse(s(v, "mode")),
        "live" => sh.live = Some((f(v, "carrier", 200.0).clamp(20.0, 1500.0), f(v, "beat", 7.83).clamp(0.0, 250.0))),
        "voice" => {
            let name = s(v, "name");
            if name == "off" || voices.iter().any(|x| x == name) {
                app.voice = name.to_string();
                if name == "off" { sh.voice_cancel = true; }
            }
        }
        "save" => {
            if app.selftest.is_none() {
                if let Some(st) = v.get("settings") { save_settings(&app.settings_path, st); }
            }
        }
        "infra" => {
            let lvl = f(v, "level", 0.0);
            sh.infra = if lvl > 0.0 { Some((f(v, "hz", 10.0).clamp(5.0, 19.0), lvl.clamp(0.0, 1.0) as f32)) } else { None };
        }
        "speak" => {
            // replay current stage cue
            app.last_stage = usize::MAX;
        }
        _ => {}
    }
}

fn tick(app: &mut App, shared: &Arc<Mutex<Shared>>, wv: &wry::WebView) {
    let Some(tl) = app.timeline.clone() else { return };
    let (running, paused, elapsed, acked, live, scope) = {
        let sh = shared.lock().unwrap();
        // downsample scope to 256 points
        let mut sc = String::with_capacity(4096);
        let n = engine::SCOPE_FRAMES;
        for i in (0..n).step_by(4) {
            if !sc.is_empty() { sc.push(','); }
            sc.push_str(&format!("{:.3},{:.3}", sh.scope[i * 2], sh.scope[i * 2 + 1]));
        }
        (sh.running, sh.paused, sh.elapsed, sh.ack_gen == app.generation, sh.live, sc)
    };
    if !acked { return; }
    if !running {
        app.timeline = None;
        let _ = wv.evaluate_script("window.__ended()");
        return;
    }
    let (mut beats, mut cars) = tl.at(elapsed);
    if tl.live { if let Some((c, b)) = live { beats[0] = b; cars[0] = c; } }
    let k = tl.stage_at(elapsed);
    if k != app.last_stage && !paused {
        app.last_stage = k;
        if app.voice != "off" {
            let id = tl.stages[k].cue_id.clone();
            if let Some(clip) = app.cache.get(&app.voice, &id) {
                shared.lock().unwrap().voice = Some(clip);
            }
        }
    }
    let state = json!({
        "paused": paused, "elapsed": elapsed, "stage": k,
        "beats": beats, "carriers": cars,
    });
    let _ = wv.evaluate_script(&format!("window.__state({},[{}])", state, scope));
}

fn write_wav(path: &str, pcm: &[f32], sr: u32) {
    let data_len = (pcm.len() * 2) as u32;
    let mut b = Vec::with_capacity(44 + data_len as usize);
    b.extend_from_slice(b"RIFF");
    b.extend_from_slice(&(36 + data_len).to_le_bytes());
    b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&2u16.to_le_bytes());
    b.extend_from_slice(&sr.to_le_bytes());
    b.extend_from_slice(&(sr * 4).to_le_bytes());
    b.extend_from_slice(&4u16.to_le_bytes());
    b.extend_from_slice(&16u16.to_le_bytes());
    b.extend_from_slice(b"data");
    b.extend_from_slice(&data_len.to_le_bytes());
    for x in pcm { b.extend_from_slice(&((x.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes()); }
    let _ = std::fs::write(path, b);
}

fn dump_cues(path: &str) {
    let mut seen = std::collections::BTreeMap::new();
    for p in presets::all() {
        for s in p.stages {
            if !s.cue.is_empty() { seen.insert(s.cue_id, s.cue); }
        }
    }
    let list: Vec<Value> = seen.into_iter().map(|(id, text)| json!({"id": id, "text": text})).collect();
    let _ = std::fs::write(path, serde_json::to_string_pretty(&list).unwrap());
}

/// Reads the saved UI settings; anything missing or malformed just means defaults.
fn load_settings(path: &std::path::Path) -> Value {
    std::fs::read_to_string(path).ok()
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .filter(|v| v.is_object())
        .unwrap_or_else(|| json!({}))
}

/// Saves the UI settings (a small JSON object; anything else or anything oversized is ignored).
fn save_settings(path: &std::path::Path, st: &Value) {
    if !st.is_object() { return; }
    let Ok(text) = serde_json::to_string_pretty(st) else { return };
    if text.len() > 64 * 1024 { return; }
    if let Some(dir) = path.parent() { let _ = std::fs::create_dir_all(dir); }
    let tmp = path.with_extension("json.tmp");
    if std::fs::write(&tmp, text).is_ok() { let _ = std::fs::rename(&tmp, path); }
}
