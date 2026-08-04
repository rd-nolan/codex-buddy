use crate::cdp::{discovery, CdpClient};
use crate::injector::css;
use crate::launcher::codex;
use crate::settings;
use crate::status::AppStatus;
use crate::status_store::{self, StatusStore};
use crate::theme::manager::ThemeManager;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn launch_codex() -> Result<String, String> {
    codex::launch(9222)?;
    Ok("Codex started".to_string())
}

#[tauri::command]
pub async fn check_codex_status() -> Result<String, String> {
    match discovery::find_endpoint(9222).await {
        Ok(_) => Ok("connected".to_string()),
        Err(_) => Ok("offline".to_string()),
    }
}

#[tauri::command]
pub async fn apply_default_theme() -> Result<String, String> {
    apply_theme("default".to_string()).await
}

#[tauri::command]
pub async fn apply_theme(name: String) -> Result<String, String> {
    let endpoint = discovery::find_endpoint(9222).await?;
    let client = CdpClient::new(endpoint);
    client.connect().await?;

    let manager = ThemeManager::new(PathBuf::from("themes"));
    let theme_css = manager.load_css(&name)?;

    client.inject_js(css::managed_css(theme_css)).await?;

    let mut current = settings::load();
    current.current_theme = name.clone();
    settings::save(&current)?;

    Ok(format!("theme applied: {}", name))
}

#[tauri::command]
pub fn current_theme() -> String {
    settings::load().current_theme
}

#[tauri::command]
pub async fn restore_theme() -> Result<String, String> {
    let settings = settings::load();

    if !settings.auto_apply {
        return Ok("auto apply disabled".to_string());
    }

    apply_theme(settings.current_theme).await
}

#[tauri::command]
pub fn get_status(state: State<'_, StatusStore>) -> AppStatus {
    status_store::snapshot(&state)
}
