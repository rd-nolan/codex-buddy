//! Chrome DevTools Protocol client
//!
//! Responsible for communicating with Codex Electron renderer.

pub mod client;
pub mod discovery;
pub mod protocol;
pub mod wait;

pub use client::CdpClient;
