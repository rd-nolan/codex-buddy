//! Runtime handlers for tray actions.

use crate::tray_actions::TrayAction;
use crate::theme::ThemeApplier;

pub struct TrayRuntime {
    pub theme_applier: ThemeApplier,
}

impl TrayRuntime {
    pub fn new(theme_applier: ThemeApplier) -> Self {
        Self { theme_applier }
    }

    pub async fn handle(&self, action: TrayAction) -> Result<(), String> {
        match action {
            TrayAction::SelectTheme(theme) => {
                self.theme_applier.apply_theme(&theme).await?;
            }
            TrayAction::OpenCodex => {
                // handled by launcher integration
            }
            TrayAction::ApplyTheme => {
                // handled by current theme integration
            }
            TrayAction::Quit => {}
        }

        Ok(())
    }
}
