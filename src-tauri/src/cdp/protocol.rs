//! Minimal Chrome DevTools Protocol message definitions

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CdpRequest {
    pub id: u32,
    pub method: String,
    pub params: serde_json::Value,
}

impl CdpRequest {
    pub fn evaluate(id: u32, script: String) -> Self {
        Self {
            id,
            method: "Runtime.evaluate".into(),
            params: serde_json::json!({
                "expression": script,
                "returnByValue": true
            }),
        }
    }
}
