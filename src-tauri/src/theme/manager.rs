use std::fs;
use std::path::PathBuf;

pub struct ThemeManager {
    root: PathBuf,
}

impl ThemeManager {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn load_css(&self, name: &str) -> Result<String, String> {
        let path = self.root.join(name).join("style.css");
        fs::read_to_string(path)
            .map_err(|e| format!("failed to load theme: {}", e))
    }
}
