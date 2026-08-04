use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;

/// Chrome DevTools Protocol client
pub struct CdpClient {
    pub websocket_url: String,
}

impl CdpClient {
    pub fn new(websocket_url: String) -> Self {
        Self { websocket_url }
    }

    /// Connect to Codex renderer through websocket.
    pub async fn evaluate(&self, script: String) -> Result<(), String> {
        let (mut socket, _) = connect_async(&self.websocket_url)
            .await
            .map_err(|e| e.to_string())?;

        let id = Uuid::new_v4();

        let payload = json!({
            "id": id.to_string(),
            "method": "Runtime.evaluate",
            "params": {
                "expression": script,
                "returnByValue": true
            }
        });

        socket
            .send(Message::Text(payload.to_string()))
            .await
            .map_err(|e| e.to_string())?;

        if let Some(response) = socket.next().await {
            println!("CDP response: {:?}", response);
        }

        Ok(())
    }

    pub async fn inject_css(&self, css: String) -> Result<(), String> {
        let escaped = css.replace('`', "\\`");

        let js = format!(
            r#"
            (() => {{
                let old = document.querySelector('#codex-buddy-style');
                if (old) old.remove();

                const style = document.createElement('style');
                style.id = 'codex-buddy-style';
                style.innerHTML = `{}`;
                document.head.appendChild(style);
            }})();
            "#,
            escaped
        );

        self.evaluate(js).await
    }
}
