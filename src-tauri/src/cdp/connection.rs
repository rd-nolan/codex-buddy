use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CdpTarget {
    pub title: Option<String>,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub websocket_url: Option<String>,
}

pub async fn discover(port: u16) -> Result<String, String> {
    let url = format!("http://127.0.0.1:{}/json", port);

    let targets: Vec<CdpTarget> = Client::new()
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    targets
        .into_iter()
        .find_map(|target| {
            if target.title.as_deref() == Some("Codex") {
                target.websocket_url
            } else {
                None
            }
        })
        .ok_or_else(|| "Codex CDP endpoint not found".to_string())
}
