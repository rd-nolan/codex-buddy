//! System tray support for Codex Buddy

use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::App;

pub fn setup_tray<R: tauri::Runtime>(app: &mut App<R>) -> Result<(), String> {
    let open_codex = MenuItem::with_id(
        app,
        "open_codex",
        "Open Codex",
        true,
        None::<&str>,
    )?;

    let apply_theme = MenuItem::with_id(
        app,
        "apply_theme",
        "Apply Current Theme",
        true,
        None::<&str>,
    )?;

    let quit = MenuItem::with_id(
        app,
        "quit",
        "Quit Codex Buddy",
        true,
        None::<&str>,
    )?;

    let menu = Menu::with_items(
        app,
        &[&open_codex, &apply_theme, &quit],
    )?;

    TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}
