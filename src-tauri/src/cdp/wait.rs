use tokio::time::{sleep, Duration};

use super::discovery;

/// Wait until Codex exposes a CDP endpoint.
///
/// Returns an error after timeout seconds.
pub async fn wait_until_ready(port: u16, timeout_seconds: u64) -> Result<String, String> {
    let mut elapsed = 0;

    while elapsed < timeout_seconds {
        if let Ok(endpoint) = discovery::find_endpoint(port).await {
            return Ok(endpoint);
        }

        sleep(Duration::from_secs(1)).await;
        elapsed += 1;
    }

    Err(format!("Codex CDP not ready after {} seconds", timeout_seconds))
}
