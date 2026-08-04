//! Discover Chromium DevTools endpoints

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DebugTarget {
    pub title: Option<String>,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: Option<String>,
}

pub async fn discover(port: u16) -> Result<Option<String>, String> {
    let url = format!("http://127.0.0.1:{}/json", port);

    let targets: Vec<DebugTarget> = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    for target in targets {
        if target
            .title
            .unwrap_or_default()
            .to_lowercase()
            .contains("codex")
        {
            return Ok(target.web_socket_debugger_url);
        }
    }

    Ok(None)
}
