use serde::Serialize;

use super::scanner::{scan_themes, ThemeInfo};

#[derive(Debug, Serialize)]
pub struct ThemeList {
    pub themes: Vec<ThemeInfo>,
}

pub fn list_themes() -> ThemeList {
    ThemeList {
        themes: scan_themes(),
    }
}
