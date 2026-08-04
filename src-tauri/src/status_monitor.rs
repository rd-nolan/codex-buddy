//! Runtime status monitor for Codex Buddy

use std::time::Duration;
use tokio::time::sleep;

use crate::cdp::discovery;
use crate::status_store::StatusStore;

/// Start background monitoring of Codex runtime state.
pub async fn start_monitor(_store: StatusStore) {
    tokio::spawn(async move {
        loop {
            // CDP availability is the most reliable runtime signal currently.
            let _connected = discovery::find_endpoint(9222).await.is_ok();

            // Future versions will update StatusStore and refresh tray items here.
            sleep(Duration::from_secs(3)).await;
        }
    });
}
