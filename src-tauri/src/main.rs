#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod injector;

#[tauri::command]
fn inject_default_css() -> String {
    injector::inject_css::default_css()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![inject_default_css])
        .run(tauri::generate_context!())
        .expect("error while running Codex Buddy");
}
