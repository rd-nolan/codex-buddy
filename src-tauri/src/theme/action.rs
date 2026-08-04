//! Theme tray action parser

/// Parse a tray menu id like `theme_glass` into a theme id.
pub fn parse_theme_action(menu_id: &str) -> Option<String> {
    menu_id
        .strip_prefix("theme_")
        .map(|name| name.to_string())
        .filter(|name| !name.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_theme_id() {
        assert_eq!(parse_theme_action("theme_glass"), Some("glass".into()));
        assert_eq!(parse_theme_action("open_codex"), None);
    }
}
