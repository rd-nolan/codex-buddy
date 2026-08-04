use std::fs;
use std::path::Path;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ThemeInfo {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
}

pub fn scan_themes() -> Vec<ThemeInfo> {
    let root = Path::new("themes");
    if !root.exists() {
        return vec![];
    }

    fs::read_dir(root)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            ThemeInfo {
                name: name.clone(),
                path: entry.path().to_string_lossy().to_string(),
                description: None,
            }
        })
        .collect()
}
