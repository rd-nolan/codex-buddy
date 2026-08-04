//! System tray support for Codex Buddy

use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{App, AppHandle, Manager};

pub fn setup_tray<R: tauri::Runtime>(app: &mut App<R>) -> Result<(), String> {
    let open_codex = MenuItem::with_id(
        app,
        "open_codex",
        "🚀 Open Codex",
        true,
        None::<&str>,
    )?;

    let apply_theme = MenuItem::with_id(
        app,
        "apply_theme",
        "🎨 Apply Current Theme",
        true,
        None::<&str>,
    )?;

    let status = MenuItem::with_id(
        app,
        "status",
        "🟡 Initializing",
        true,
        None::<&str>,
    )?;

    let quit = MenuItem::with_id(
        app,
        "quit",
        "❌ Quit Codex Buddy",
        true,
        None::<&str>,
    )?;

    let menu = Menu::with_items(
        app,
        &[&open_codex, &apply_theme, &status, &quit],
    )?;

    TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "quit" => {
                    app.exit(0);
                }
                "open_codex" => {
                    // Reuse launcher command from tray action.
                }
                "apply_theme" => {
                    // Reuse theme command from tray action.
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}

/// Update tray status text from runtime state.
/// The caller can invoke this after startup/CDP/theme changes.
pub fn update_status<R: tauri::Runtime>(app: &AppHandle<R>, text: &str) {
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_tooltip(Some(text));
    }
}
