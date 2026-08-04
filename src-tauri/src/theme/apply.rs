//! Runtime theme application.
//!
//! Loads theme CSS and injects it into Codex through CDP.

use crate::cdp::CdpClient;
use crate::theme::ThemeManager;

pub struct ThemeApplier {
    manager: ThemeManager,
    cdp: CdpClient,
}

impl ThemeApplier {
    pub fn new(manager: ThemeManager, cdp: CdpClient) -> Self {
        Self { manager, cdp }
    }

    pub async fn apply_theme(&self, theme_id: &str) -> Result<(), String> {
        let css = self.manager.load_css(theme_id)?;

        self.cdp.inject_css(css).await?;

        Ok(())
    }
}
