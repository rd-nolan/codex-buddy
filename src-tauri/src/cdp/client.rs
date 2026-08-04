use serde_json::json;

/// Chrome DevTools Protocol client
pub struct CdpClient {
    pub port: u16,
}

impl CdpClient {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    /// Connect to Codex renderer through websocket.
    pub async fn connect(&self) -> Result<(), String> {
        println!("Connecting CDP on port {}", self.port);
        Ok(())
    }

    /// Execute JavaScript in Codex renderer.
    pub async fn evaluate(&self, script: String) -> Result<(), String> {
        let message = json!({
            "method": "Runtime.evaluate",
            "params": {
                "expression": script
            }
        });

        println!("CDP message: {}", message);
        Ok(())
    }

    /// Inject CSS into Codex page.
    pub async fn inject_css(&self, css: String) -> Result<(), String> {
        let js = format!(
            r#"
            (() => {{
                let old = document.querySelector('#codex-buddy-style');
                if (old) old.remove();
                let style = document.createElement('style');
                style.id = 'codex-buddy-style';
                style.innerHTML = `{}`;
                document.head.appendChild(style);
            }})();
            "#,
            css.replace('`', "\\`")
        );

        self.evaluate(js).await
    }
}
