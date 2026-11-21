// Desktop entry for `cargo tauri dev`; on Android the library entry point is used instead.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    focus_mobile_lib::run()
}
