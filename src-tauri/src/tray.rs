//! System tray support for Codex Buddy

use crate::commands;
use crate::settings;
use crate::status_store::StatusStore;
use crate::tray_actions::TrayAction;
use tauri::menu::{Menu, MenuItem, Submenu};
use tauri::tray::TrayIconBuilder;
use tauri::{App, AppHandle, Manager};

pub fn setup_tray<R: tauri::Runtime>(app: &mut App<R>) -> Result<(), String> {
    let open_codex = MenuItem::with_id(app, "open_codex", "🚀 Open Codex", true, None::<&str>)?;
    let apply_theme = MenuItem::with_id(
        app,
        "apply_theme",
        "🎨 Apply Current Theme",
        true,
        None::<&str>,
    )?;

    let default_theme = MenuItem::with_id(app, "theme_default", "Default", true, None::<&str>)?;
    let glass_theme = MenuItem::with_id(app, "theme_glass", "Glass", true, None::<&str>)?;
    let midnight_theme = MenuItem::with_id(app, "theme_midnight", "Midnight", true, None::<&str>)?;

    let themes = Submenu::with_items(
        app,
        "🎨 Themes",
        true,
        &[&default_theme, &glass_theme, &midnight_theme],
    )?;

    let status = MenuItem::with_id(app, "status", "🟡 Initializing", false, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "❌ Quit Codex Buddy", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&status, &themes, &open_codex, &apply_theme, &quit])?;

    TrayIconBuilder::new()
        .id("main")
        .menu(&menu)
        .on_menu_event(|app, event| {
            let Some(action) = TrayAction::from_id(event.id().as_ref()) else {
                return;
            };

            match action {
                TrayAction::Quit => app.exit(0),
                TrayAction::SelectTheme(theme) => {
                    spawn_theme_application(app.clone(), theme);
                }
                TrayAction::ApplyTheme => {
                    spawn_theme_application(app.clone(), settings::load().current_theme);
                }
                TrayAction::OpenCodex => {
                    // Launcher action remains separate from theme application.
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn spawn_theme_application<R: tauri::Runtime>(app: AppHandle<R>, theme: String) {
    let store = app.state::<StatusStore>().inner().clone();

    tauri::async_runtime::spawn(async move {
        if let Err(error) = commands::apply_theme_runtime(theme, &store).await {
            if let Ok(mut status) = store.write() {
                status.last_injection = format!("failed: {}", error);
            }
        }
    });
}

pub fn update_status<R: tauri::Runtime>(app: &AppHandle<R>, text: &str) {
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_tooltip(Some(text));
    }
}
