#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cdp;
mod commands;
mod injector;
mod launcher;
mod settings;
mod startup;
mod theme;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::launch_codex,
            commands::check_codex_status,
            commands::apply_default_theme,
            commands::apply_theme,
            commands::current_theme,
            commands::restore_theme
        ])
        .run(tauri::generate_context!())
        .expect("error while running Codex Buddy");
}
