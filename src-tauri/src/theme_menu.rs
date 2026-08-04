//! Dynamic theme submenu builder for Codex Buddy

use tauri::menu::{MenuItem, Submenu};
use tauri::App;

use crate::theme::scanner::scan_themes;

/// Build theme submenu from themes directory.
///
/// Theme folders are discovered at runtime, allowing users to add themes
/// without modifying Codex Buddy source code.
pub fn build_theme_menu<R: tauri::Runtime>(app: &mut App<R>) -> Result<Submenu<R>, String> {
    let themes = scan_themes();

    let mut items = Vec::new();

    for theme in themes {
        let id = format!("theme_{}", theme.name.to_lowercase());
        let title = theme.name.clone();

        let item = MenuItem::with_id(
            app,
            id,
            title,
            true,
            None::<&str>,
        )?;

        items.push(item);
    }

    let item_refs: Vec<&MenuItem<R>> = items.iter().collect();

    let submenu = Submenu::with_items(
        app,
        "🎨 Themes",
        true,
        &item_refs,
    )?;

    Ok(submenu)
}
