//! Runtime status center for Codex Buddy

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStatus {
    pub codex_running: bool,
    pub cdp_connected: bool,
    pub current_theme: String,
    pub last_injection: String,
}

impl Default for AppStatus {
    fn default() -> Self {
        Self {
            codex_running: false,
            cdp_connected: false,
            current_theme: "default".to_string(),
            last_injection: "none".to_string(),
        }
    }
}

impl AppStatus {
    pub fn connected() -> Self {
        Self {
            codex_running: true,
            cdp_connected: true,
            ..Default::default()
        }
    }
}
