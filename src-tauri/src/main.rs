#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cdp;
mod commands;
mod injector;
mod launcher;
mod settings;
mod startup;
mod status;
mod status_store;
mod theme;
mod tray;
mod tray_actions;
mod tray_monitor;

#[tokio::main]
async fn main() {
    if let Err(error) = startup::initialize().await {
        eprintln!("startup error: {}", error);
    }

    let status_store = status_store::create_store();

    tauri::Builder::default()
        .manage(status_store.clone())
        .setup(|app| {
            tray::setup_tray(app)?;

            tray_monitor::start(
                status_store.clone(),
                app.handle().clone(),
            );

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::launch_codex,
            commands::check_codex_status,
            commands::apply_default_theme,
            commands::apply_theme,
            commands::current_theme,
            commands::restore_theme,
            commands::get_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running Codex Buddy");
}
