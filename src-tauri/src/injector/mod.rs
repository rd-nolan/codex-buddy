//! Runtime CSS/JS injector
//!
//! This module will communicate with Codex Electron renderer
//! through Chrome DevTools Protocol.

pub struct Injector {
    pub enabled: bool,
}

impl Injector {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub async fn inject_css(&self, css: String) -> Result<(), String> {
        // TODO:
        // 1. Connect CDP websocket
        // 2. Call Runtime.evaluate
        // 3. Append <style id="codex-buddy">
        println!("Inject CSS: {} chars", css.len());
        Ok(())
    }

    pub async fn inject_js(&self, js: String) -> Result<(), String> {
        println!("Inject JS: {} chars", js.len());
        Ok(())
    }
}
