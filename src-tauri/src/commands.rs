use crate::cdp::{discovery, CdpClient};
use crate::injector::css;
use crate::launcher::codex;

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
    let endpoint = discovery::find_endpoint(9222).await?;
    let client = CdpClient::new(endpoint);
    client.connect().await?;
    client.inject_css(css::default_css()).await?;
    Ok("theme applied".to_string())
}
