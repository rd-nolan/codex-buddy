# Codex Buddy

A Tauri-based companion tool for Codex.

## Features

- Launch Codex with remote debugging enabled
- Connect to Electron DevTools Protocol (CDP)
- Inject custom CSS/JS at runtime
- Manage themes

## Roadmap

- [x] Project initialization
- [x] Default CSS injection design
- [ ] CDP websocket client
- [ ] Theme manager
- [ ] Plugin system

## Architecture

```
Codex Buddy (Tauri)
        |
        v
Chrome DevTools Protocol
        |
        v
Codex Electron Renderer
        |
        v
CSS / JS Injection
```
