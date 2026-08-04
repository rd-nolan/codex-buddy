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
mod tray_monitor;

use tauri::Listener;

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

            let handle = app.handle().clone();
            let store = status_store.clone();

            app.listen("theme-selected", move |event| {
                if let Some(theme) = event.payload() {
                    let theme = theme.trim_matches('"').to_string();
                    let store = store.clone();
                    tokio::spawn(async move {
                        let _ = commands::apply_theme_runtime(theme, store).await;
                    });
                }
            });

            tray_monitor::start(
                status_store.clone(),
                handle,
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
