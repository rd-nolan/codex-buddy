use std::fs;
use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThemeInfo {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
}

fn read_theme_metadata(path: &Path, fallback_name: String) -> ThemeInfo {
    let config = path.join("theme.json");

    if let Ok(content) = fs::read_to_string(config) {
        if let Ok(mut info) = serde_json::from_str::<ThemeInfo>(&content) {
            info.path = path.to_string_lossy().to_string();
            return info;
        }
    }

    ThemeInfo {
        name: fallback_name,
        path: path.to_string_lossy().to_string(),
        description: None,
    }
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
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            read_theme_metadata(&path, name)
        })
        .collect()
}
