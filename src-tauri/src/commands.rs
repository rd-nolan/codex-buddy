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
pub async fn launch_codex(state: State<'_, StatusStore>) -> Result<String, String> {
    codex::launch(9222)?;

    if let Ok(mut status) = state.write() {
        status.codex_running = true;
    }

    Ok("Codex started".to_string())
}

#[tauri::command]
pub async fn check_codex_status(state: State<'_, StatusStore>) -> Result<String, String> {
    match discovery::find_endpoint(9222).await {
        Ok(_) => {
            if let Ok(mut status) = state.write() {
                status.codex_running = true;
                status.cdp_connected = true;
            }
            Ok("connected".to_string())
        }
        Err(_) => {
            if let Ok(mut status) = state.write() {
                status.cdp_connected = false;
            }
            Ok("offline".to_string())
        }
    }
}

#[tauri::command]
pub async fn apply_default_theme(state: State<'_, StatusStore>) -> Result<String, String> {
    apply_theme_inner("default".to_string(), &state).await
}

#[tauri::command]
pub async fn apply_theme(name: String, state: State<'_, StatusStore>) -> Result<String, String> {
    apply_theme_inner(name, &state).await
}

async fn apply_theme_inner(name: String, state: &StatusStore) -> Result<String, String> {
    let endpoint = discovery::find_endpoint(9222).await?;
    let client = CdpClient::new(endpoint);
    client.connect().await?;

    let manager = ThemeManager::new(PathBuf::from("themes"));
    let theme_css = manager.load_css(&name)?;

    client.inject_js(css::managed_css(theme_css)).await?;

    let mut current = settings::load();
    current.current_theme = name.clone();
    settings::save(&current)?;

    if let Ok(mut status) = state.write() {
        status.current_theme = name.clone();
        status.cdp_connected = true;
        status.last_injection = "success".to_string();
    }

    Ok(format!("theme applied: {}", name))
}

#[tauri::command]
pub fn current_theme() -> String {
    settings::load().current_theme
}

#[tauri::command]
pub async fn restore_theme(state: State<'_, StatusStore>) -> Result<String, String> {
    let settings = settings::load();

    if !settings.auto_apply {
        return Ok("auto apply disabled".to_string());
    }

    apply_theme_inner(settings.current_theme, &state).await
}

#[tauri::command]
pub fn get_status(state: State<'_, StatusStore>) -> AppStatus {
    status_store::snapshot(&state)
}
