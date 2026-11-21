//! Focus — Android build.
//! Same Rust audio engine, session library, explanations and voices as the desktop app
//! (the source files are shared, not copied). Tauri provides the Android window and WebView;
//! the UI talks to Rust through one `msg` command carrying the same JSON as on desktop.
#![allow(dead_code)]

#[path = "../../src/engine.rs"]
mod engine;
#[path = "../../src/info.rs"]
mod info;
#[path = "../../src/plan.rs"]
mod plan;
#[path = "../../src/presets.rs"]
mod presets;
#[path = "../../src/texture.rs"]
mod texture;
mod voice;

use engine::{Mode, Noise, Shared};
use plan::{Preset, Timeline};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Manager;

struct App {
    presets: Vec<Preset>,
    timeline: Option<Arc<Timeline>>,
    generation: u64,
    last_stage: usize,
    voice: String,
    cache: voice::Cache,
}

struct Core {
    app: Mutex<App>,
    shared: Arc<Mutex<Shared>>,
    voices: Vec<String>,
    audio_error: Option<String>,
    /// <app data dir>/settings.json, known once Tauri has started.
    settings_path: std::sync::OnceLock<std::path::PathBuf>,
}

#[tauri::command]
fn msg(m: String, window: tauri::WebviewWindow, core: tauri::State<'_, Arc<Core>>) {
    let Ok(v) = serde_json::from_str::<Value>(&m) else { return };
    let mut app = core.app.lock().unwrap();
    handle(&v, &mut app, &core, &|js: &str| {
        let _ = window.eval(js);
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let shared = Arc::new(Mutex::new(Shared::default()));

    // The audio stream lives on its own thread for the life of the app.
    let (tx, rx) = std::sync::mpsc::channel::<Option<String>>();
    let sh = shared.clone();
    std::thread::spawn(move || match engine::start(sh) {
        Ok(stream) => {
            let _ = tx.send(None);
            let _keep_alive = stream;
            loop {
                std::thread::park();
            }
        }
        Err(e) => {
            let _ = tx.send(Some(e));
        }
    });
    let audio_error = rx.recv_timeout(Duration::from_secs(5)).unwrap_or(Some("audio start timed out".into()));

    let voices = voice::available();
    let core = Arc::new(Core {
        app: Mutex::new(App {
            presets: presets::all(),
            timeline: None,
            generation: 0,
            last_stage: usize::MAX,
            voice: voices.iter().find(|v| *v == "af_heart").or(voices.first()).cloned().unwrap_or_default(),
            cache: voice::Cache::default(),
        }),
        shared,
        voices,
        audio_error,
        settings_path: std::sync::OnceLock::new(),
    });

    let ticker = core.clone();
    tauri::Builder::default()
        .manage(core)
        .invoke_handler(tauri::generate_handler![msg])
        .setup(move |a| {
            if let Ok(dir) = a.path().app_data_dir() {
                let _ = ticker.settings_path.set(dir.join("settings.json"));
            }
            let h = a.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_millis(33));
                if let Some(w) = h.get_webview_window("main") {
                    let mut app = ticker.app.lock().unwrap();
                    tick(&mut app, &ticker.shared, &|js: &str| {
                        let _ = w.eval(js);
                    });
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Focus");
}

fn f(v: &Value, k: &str, d: f64) -> f64 { v.get(k).and_then(|x| x.as_f64()).filter(|x| x.is_finite()).unwrap_or(d) }
fn s<'a>(v: &'a Value, k: &str) -> &'a str { v.get(k).and_then(|x| x.as_str()).unwrap_or("") }

/// Same commands as the desktop app (src/main.rs), minus the desktop-only self-test.
fn handle(v: &Value, app: &mut App, core: &Core, eval: &dyn Fn(&str)) {
    let mut sh = core.shared.lock().unwrap();
    match s(v, "cmd") {
        "ready" => {
            let settings = core.settings_path.get().map(|p| load_settings(p)).unwrap_or_else(|| json!({}));
            let init = json!({ "presets": app.presets, "voices": core.voices, "voice": app.voice, "audioError": core.audio_error, "settings": settings });
            drop(sh);
            eval(&format!("window.__init({})", init));
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
                if tl.total.is_finite() {
                    sh.seek = Some(f(v, "t", 0.0).clamp(0.0, tl.total - 0.5));
                    sh.voice_cancel = true;
                }
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
            if name == "off" || core.voices.iter().any(|x| x == name) {
                app.voice = name.to_string();
                if name == "off" {
                    sh.voice_cancel = true;
                }
            }
        }
        "speak" => app.last_stage = usize::MAX,
        "save" => {
            if let (Some(path), Some(st)) = (core.settings_path.get(), v.get("settings")) {
                save_settings(path, st);
            }
        }
        _ => {}
    }
}

fn tick(app: &mut App, shared: &Arc<Mutex<Shared>>, eval: &dyn Fn(&str)) {
    let Some(tl) = app.timeline.clone() else { return };
    let (running, paused, elapsed, acked, live, scope) = {
        let sh = shared.lock().unwrap();
        let mut sc = String::with_capacity(4096);
        let n = engine::SCOPE_FRAMES;
        for i in (0..n).step_by(4) {
            if !sc.is_empty() {
                sc.push(',');
            }
            sc.push_str(&format!("{:.3},{:.3}", sh.scope[i * 2], sh.scope[i * 2 + 1]));
        }
        (sh.running, sh.paused, sh.elapsed, sh.ack_gen == app.generation, sh.live, sc)
    };
    if !acked {
        return;
    }
    if !running {
        app.timeline = None;
        eval("window.__ended()");
        return;
    }
    let (mut beats, mut cars) = tl.at(elapsed);
    if tl.live {
        if let Some((c, b)) = live {
            beats[0] = b;
            cars[0] = c;
        }
    }
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
    let state = json!({ "paused": paused, "elapsed": elapsed, "stage": k, "beats": beats, "carriers": cars });
    eval(&format!("window.__state({},[{}])", state, scope));
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
