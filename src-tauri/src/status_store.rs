//! Shared runtime status storage for Codex Buddy

use crate::status::AppStatus;
use std::sync::{Arc, RwLock};

pub type StatusStore = Arc<RwLock<AppStatus>>;

pub fn create_store() -> StatusStore {
    Arc::new(RwLock::new(AppStatus::default()))
}

pub fn snapshot(store: &StatusStore) -> AppStatus {
    store
        .read()
        .map(|status| status.clone())
        .unwrap_or_default()
}
