use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub current_theme: String,
    pub auto_apply: bool,
    pub auto_start_codex: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            current_theme: "default".to_string(),
            auto_apply: true,
            auto_start_codex: true,
        }
    }
}

pub fn load() -> Settings {
    let path = PathBuf::from("settings.json");
    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Settings::default(),
    }
}

pub fn save(settings: &Settings) -> Result<(), String> {
    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| e.to_string())?;

    fs::write("settings.json", content)
        .map_err(|e| e.to_string())
}
