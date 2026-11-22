//! Builds the mobile front-end from the desktop UI (ui/index.html) so both stay identical.
//! Only two things change: messages go through Tauri's invoke() instead of wry's raw IPC,
//! and the Content-Security-Policy allows Tauri's local IPC endpoint (still no internet).
use std::path::PathBuf;

fn main() {
    let here = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let root = here.parent().unwrap().canonicalize().unwrap();
    // voice.rs embeds <repo>/voice via include_dir!("$FOCUS_ROOT/voice").
    println!("cargo:rustc-env=FOCUS_ROOT={}", root.display());
    println!("cargo:rerun-if-changed=../ui/index.html");
    println!("cargo:rerun-if-changed=../voice");
    println!("cargo:rerun-if-changed=../chant");

    let src = std::fs::read_to_string(root.join("ui/index.html")).expect("read ui/index.html");
    let ipc_old = "window.ipc.postMessage(JSON.stringify(o))";
    let ipc_new = "(window.__TAURI_INTERNALS__?window.__TAURI_INTERNALS__.invoke(\"msg\",{m:JSON.stringify(o)}).catch(()=>{}):setTimeout(()=>send(o),100))";
    let csp_old = "connect-src 'none'";
    let csp_new = "connect-src ipc: http://ipc.localhost";
    assert!(src.contains(ipc_old), "ui/index.html: IPC call not found");
    assert!(src.contains(csp_old), "ui/index.html: CSP connect-src not found");
    let out = src.replace(ipc_old, ipc_new).replace(csp_old, csp_new);
    let dist = here.join("dist");
    std::fs::create_dir_all(&dist).unwrap();
    std::fs::write(dist.join("index.html"), out).unwrap();

    tauri_build::build();
}
