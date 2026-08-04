//! System tray support for Codex Buddy

use tauri::App;

pub fn setup_tray<R: tauri::Runtime>(_app: &mut App<R>) -> Result<(), String> {
    // Tray menu will be added here.
    // Reserved for:
    // - Open Codex
    // - Current theme
    // - Apply theme
    // - Quit
    Ok(())
}
