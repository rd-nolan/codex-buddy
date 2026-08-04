//! Tray status synchronization for Codex Buddy

use std::time::Duration;
use tauri::{AppHandle, Runtime};
use tokio::time::sleep;

use crate::status_store::{snapshot, StatusStore};
use crate::tray;

pub async fn start(store: StatusStore, app: AppHandle) {
    tokio::spawn(async move {
        loop {
            let status = snapshot(&store);

            let text = format!(
                "Codex: {} | CDP: {} | Theme: {}",
                if status.codex_running { "Running" } else { "Offline" },
                if status.cdp_connected { "Connected" } else { "Disconnected" },
                status.current_theme
            );

            tray::update_status(&app, &text);

            sleep(Duration::from_secs(3)).await;
        }
    });
}
