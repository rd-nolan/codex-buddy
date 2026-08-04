//! System tray support for Codex Buddy

use crate::tray_actions::TrayAction;
use tauri::menu::{Menu, MenuItem, Submenu};
use tauri::tray::TrayIconBuilder;
use tauri::{App, AppHandle};

pub fn setup_tray<R: tauri::Runtime>(app: &mut App<R>) -> Result<(), String> {
    let open_codex = MenuItem::with_id(app, "open_codex", "🚀 Open Codex", true, None::<&str>)?;
    let apply_theme = MenuItem::with_id(app, "apply_theme", "🎨 Apply Current Theme", true, None::<&str>)?;

    let default_theme = MenuItem::with_id(app, "theme_default", "Default", true, None::<&str>)?;
    let glass_theme = MenuItem::with_id(app, "theme_glass", "Glass", true, None::<&str>)?;
    let midnight_theme = MenuItem::with_id(app, "theme_midnight", "Midnight", true, None::<&str>)?;

    let themes = Submenu::with_items(
        app,
        "🎨 Themes",
        true,
        &[&default_theme, &glass_theme, &midnight_theme],
    )?;

    let status = MenuItem::with_id(app, "status", "🟡 Initializing", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "❌ Quit Codex Buddy", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&status, &themes, &open_codex, &apply_theme, &quit])?;

    TrayIconBuilder::new()
        .id("main")
        .menu(&menu)
        .on_menu_event(|app, event| {
            if let Some(action) = TrayAction::from_id(event.id().as_ref()) {
                match action {
                    TrayAction::Quit => app.exit(0),
                    TrayAction::SelectTheme(theme) => {
                        println!("Selected theme: {}", theme);
                    }
                    _ => {}
                }
            }
        })
        .build(app)?;

    Ok(())
}

pub fn update_status<R: tauri::Runtime>(app: &AppHandle<R>, text: &str) {
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_tooltip(Some(text));
    }
}
