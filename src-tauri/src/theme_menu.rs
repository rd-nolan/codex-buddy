//! Dynamic theme submenu builder for Codex Buddy

use tauri::menu::{Menu, MenuItem, Submenu};
use tauri::App;

/// Build theme submenu from available themes.
///
/// This is intentionally isolated so theme discovery can later come from
/// ThemeManager instead of hard-coded tray entries.
pub fn build_theme_menu<R: tauri::Runtime>(app: &mut App<R>) -> Result<Submenu<R>, String> {
    let default = MenuItem::with_id(app, "theme_default", "Default", true, None::<&str>)?;
    let glass = MenuItem::with_id(app, "theme_glass", "Glass", true, None::<&str>)?;
    let midnight = MenuItem::with_id(app, "theme_midnight", "Midnight", true, None::<&str>)?;

    let submenu = Submenu::with_items(
        app,
        "🎨 Themes",
        true,
        &[&default, &glass, &midnight],
    )?;

    Ok(submenu)
}
