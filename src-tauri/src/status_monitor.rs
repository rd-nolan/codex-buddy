//! Runtime status monitor for Codex Buddy

use std::time::Duration;
use tokio::time::sleep;

use crate::cdp::discovery;
use crate::status_store::StatusStore;

/// Start background monitoring of Codex runtime state.
pub async fn start_monitor(store: StatusStore) {
    tokio::spawn(async move {
        loop {
            let connected = discovery::find_endpoint(9222).await.is_ok();

            if let Ok(mut status) = store.write() {
                status.cdp_connected = connected;
                status.codex_running = connected;
            }

            sleep(Duration::from_secs(3)).await;
        }
    });
}
