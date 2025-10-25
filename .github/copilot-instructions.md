# Copilot Instructions for ZenTerm

## Project Overview
ZenTerm is a modular, event-driven terminal emulator for Linux, built with a Rust backend (Tauri) and a React/TypeScript frontend. The architecture is split between:
- `src-tauri/`: Rust backend (Tauri entry, terminal logic, IPC)
- `crates/engine/`: Core engine (event bus, command registry, config, theme, wizard, mocks)
- `src/`: React UI (main: `App.tsx`, entry: `main.tsx`, components/)

## Architecture & Patterns
- **Event Bus**: Central for inter-component comms (`crates/engine/src/event_bus.rs`). Use `AppEvent` for cross-module signaling (e.g., `ThemeToggled`, `ConfigSaveRequested`).
- **Command Registry**: All user actions are registered in `command_registry.rs` (`register_default_commands`).
- **Config**: JSON-based, debounced (50-10000ms), loaded from user config dir (`config.rs`, see `TuiApp`).
- **Wizard**: Multi-step setup for first-run (`wizard.rs`). Steps: Welcome → GPU Config → Theme → Voice → Complete.
- **Theme**: Centralized palette, supports dark/light (`theme.rs`).
- **Frontend**: React components in `src/components/` (e.g., `TerminalView.tsx`, `TabBar.tsx`). Types in `src/types.ts`.

## Developer Workflows
- **Dev build (full stack)**: `npm run tauri dev`
- **Production build**: `npm run tauri build`
- **Rust only**: `cargo build`, `cargo test`, `cargo fmt --all`, `cargo clippy --all-targets -- -D warnings`, `cargo audit`
- **Frontend only**: `npm run dev`, `npm run build`
- **Debug**: `RUST_LOG=debug npm run tauri dev`
- **Integration tests**: `tests/` (Rust)

## Conventions & Examples
- **Commands**: Register in `command_registry.rs` (e.g., `theme.toggle`, `wizard.open`).
- **Events**: Use event bus for all cross-module comms. Prefer structured event names (e.g., `wizard.start`, `settings.change.*`).
- **Config**: Add new options in `config.rs` and ensure load/save in `TuiApp`.
- **Logging**: Use structured event names for logs.
- **Testing**: Rust unit tests in each module, integration in `tests/`.
- **Frontend**: Use TypeScript types from `src/types.ts`.

## Integration Points
- **Tauri IPC**: Rust commands exposed to frontend via `@tauri-apps/api` (`invoke`). See `DEVELOPMENT.md` for API examples.
- **Voice/GPU**: Currently mocked; see `voice_mock.rs`, `gpu_mock.rs`.

## References
- [README.md](../README.md): Features, build, architecture
- [DEVELOPMENT.md](../DEVELOPMENT.md): Setup, workflows, API
- [PULL_REQUEST_TEMPLATE.md](../PULL_REQUEST_TEMPLATE.md): Event bus API, review checklist

---
For unclear or missing conventions, consult `README.md` and `DEVELOPMENT.md` or ask for clarification in your PR.
