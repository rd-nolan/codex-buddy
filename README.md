# Codex Buddy

A Tauri-based companion tool for Codex.

Codex Buddy provides runtime customization for Codex desktop through Chrome DevTools Protocol (CDP), including theme injection, CSS/JS customization, and future extension capabilities.

## Features

- Launch Codex with remote debugging enabled
- Connect to Electron DevTools Protocol (CDP)
- Inject custom CSS/JS at runtime
- Theme management system
- Theme discovery and loading
- Tauri desktop application foundation

## Roadmap

### v0.1 Foundation

- [x] Project initialization
- [x] Tauri application setup
- [x] Default CSS injection design
- [x] CDP architecture design
- [x] Codex launcher module
- [x] CDP endpoint discovery
- [x] WebSocket CDP client
- [x] Runtime CSS injection pipeline

### v0.2 Theme System

- [x] Theme manager
- [x] Theme file structure
- [x] Theme scanner
- [x] Theme API foundation
- [x] Theme selector UI
- [ ] Theme preview
- [ ] One-click theme switching
- [ ] Theme marketplace support

### v0.3 Extension Platform

- [ ] Plugin system
- [ ] JavaScript extension API
- [ ] Custom widgets
- [ ] Community themes

### v1.0 Codex Companion

- [ ] Plugin marketplace
- [ ] AI assistant integration
- [ ] Workflow automation
- [ ] Cross-platform release

## Architecture

```
                 Codex Buddy (Tauri)
                         |
                         v
                 Theme Manager
                         |
                         v
                  CSS / JS Injector
                         |
                         v
              Chrome DevTools Protocol
                         |
                         v
              Codex Electron Renderer
```

## Project Status

Early development stage. The goal is to build an extensible companion ecosystem for Codex rather than only a theme tool.
