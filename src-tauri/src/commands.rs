use crate::cdp::{discovery, CdpClient};
use crate::injector::css;
use crate::launcher::codex;
use crate::theme::manager::ThemeManager;
use std::path::PathBuf;

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
    apply_theme("default").await
}

#[tauri::command]
pub async fn apply_theme(name: String) -> Result<String, String> {
    let endpoint = discovery::find_endpoint(9222).await?;
    let client = CdpClient::new(endpoint);
    client.connect().await?;

    let manager = ThemeManager::new(PathBuf::from("themes"));
    let theme_css = manager.load_css(&name)?;

    client.inject_css(theme_css).await?;

    Ok(format!("theme applied: {}", name))
}
