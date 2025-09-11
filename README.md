# ZenTerm - Advanced Terminal User Interface

Modern TUI application with advanced features including wizard setup, settings management, and responsive design.

## Features

### Sprint Turbo Implementation
- **Stability & UX**: Panic guard with terminal restoration, quit confirmation, global help overlay, graceful exits
- **Visual Polish**: Centralized dark/light theme palette, responsive layout, accented borders  
- **Performance**: Debounced configuration saving, buffer reuse, optimized rendering

### Core Functionality

#### Wizard Mode
- Interactive setup process for new installations
- GPU memory limit configuration (10-100%)
- Quit confirmation with y/n prompt
- Structured logging: `wizard.start`, `wizard.complete`, `wizard.abort`

#### Runtime Mode  
- Main application interface
- Access to settings and help
- Clean quit functionality

#### Settings Mode
- GPU limit adjustment with ↑/↓ keys
- Theme toggle between dark and light modes
- Ctrl+C graceful exit support
- Debounced saving (configurable 50-10000ms)
- Structured logging: `settings.open`, `settings.close`, `settings.change.*`

#### Help System
- Global help overlay accessible with `?` key
- Comprehensive keybinding documentation
- Close with `?` or `ESC`
- Structured logging: `help.show`, `help.hide`

#### Responsive Design
- Footer hidden when terminal height < 18 rows
- Compact header when terminal height < 12 rows
- Automatic layout adaptation

#### Advanced Features
- **Panic Guard**: Automatic terminal restoration on panic
- **Structured Logging**: Comprehensive event tracking including `gpu.limit.apply`
- **Theme System**: Centralized palette with dark/light mode support
- **Configuration**: JSON-based config with debounced persistence

## Struttura

- `apps/desktop`: TUI application binary with full feature set
- `crates/engine`: Core library with UI components and application logic
- `tests/`: Test suite including unit tests for all major features

## Quick Start

```bash
# Build and test
cargo build --release
cargo test

# Run the application
cargo run -p app-desktop

# Test panic handler (verifies terminal restoration)
cargo run -p app-desktop -- --test-panic

# Configure debounce timing (50-10000ms range)
ZENTERM_SAVE_DEBOUNCE_MS=100 cargo run -p app-desktop
```

## Keybindings

### Global
- `?` - Toggle help overlay
- `ESC` - Close dialogs/help

### Wizard Mode
- `ENTER` - Next step
- `q` - Quit (with confirmation)
- `b` - Back to previous step
- `↑/↓` - Adjust values

### Runtime Mode  
- `s` - Open Settings
- `q` - Quit application

### Settings Mode
- `↑/↓` - Adjust GPU limit
- `t` - Toggle theme
- `Ctrl+C` - Graceful exit
- `ESC/q` - Back to Runtime

## Configuration

Configuration is automatically saved to `~/.config/zenterm/config.json` with the following structure:

```json
{
  "gpu_limit": 80,
  "theme": "dark",
  "debounce_ms": 500
}
```

### Environment Variables
- `ZENTERM_SAVE_DEBOUNCE_MS`: Override save debounce timing (50-10000ms)
- `RUST_LOG`: Control logging level (e.g., `info`, `debug`)

## Testing

The application includes comprehensive testing:

```bash
# Unit tests
cargo test

# Manual testing scenarios
./target/release/app-desktop

# Responsive layout testing
# - Resize terminal to < 18 rows (footer disappears)
# - Resize terminal to < 12 rows (compact header)

# Debounce testing  
# - Change GPU limit repeatedly
# - Observe single save after delay
# - Verify file timestamp updates

# Panic testing
./target/release/app-desktop --test-panic
```

## CI

Workflow GitHub Actions `Rust CI`:
- `fmt` (rustfmt)
- `clippy` (warning = error)  
- build e test su Rust stable
