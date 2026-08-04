use crate::cdp::wait;
use crate::commands;
use crate::settings;

pub async fn initialize() -> Result<String, String> {
    let config = settings::load();

    if config.auto_start_codex {
        commands::launch_codex().await?;

        wait::wait_until_ready(9222, 30).await?;
    }

    if config.auto_apply {
        commands::apply_theme(config.current_theme).await?;
    }

    Ok("startup initialized".to_string())
}
