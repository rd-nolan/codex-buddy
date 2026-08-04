#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod injector;

#[tauri::command]
fn get_default_css() -> String {
    injector::css::default_css()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_default_css])
        .run(tauri::generate_context!())
        .expect("error while running Codex Buddy");
}
