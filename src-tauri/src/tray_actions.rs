//! Tray action handlers for Codex Buddy.
//! Keeps tray callbacks separated from tray construction.

pub enum TrayAction {
    OpenCodex,
    ApplyTheme,
    Quit,
}

impl TrayAction {
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "open_codex" => Some(Self::OpenCodex),
            "apply_theme" => Some(Self::ApplyTheme),
            "quit" => Some(Self::Quit),
            _ => None,
        }
    }
}
