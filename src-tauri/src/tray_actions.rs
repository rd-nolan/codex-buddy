//! Tray action handlers for Codex Buddy.
//! Keeps tray callbacks separated from tray construction.

pub enum TrayAction {
    OpenCodex,
    ApplyTheme,
    SelectTheme(String),
    Quit,
}

impl TrayAction {
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "open_codex" => Some(Self::OpenCodex),
            "apply_theme" => Some(Self::ApplyTheme),
            "quit" => Some(Self::Quit),
            value if value.starts_with("theme_") => {
                let theme = value.trim_start_matches("theme_").to_string();
                if theme.is_empty() {
                    None
                } else {
                    Some(Self::SelectTheme(theme))
                }
            }
            _ => None,
        }
    }
}
