# ZenTerm - Modern Terminal Application

A Rust-based terminal application inspired by Warp, with clean architecture and modern tooling.

## Project Structure

- `apps/desktop`: Desktop binary with CLI interface, configuration management, and GUI (winit)
- `crates/engine`: Core library for terminal functionality (placeholder for future features)
- `tests/`: Workspace-level smoke tests

## Build

```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run tests
cargo test

# Run the application
cargo run -p app-desktop

# Run with flags (see below)
cargo run -p app-desktop -- --help
```

## Command Line Flags

ZenTerm supports the following command line options:

- `--reset-config`: Reset configuration to default values
- `-v, --verbose`: Enable debug logging  
- `-vv`: Enable trace logging (maximum verbosity)
- `-h, --help`: Show help information

### Examples

```bash
# Run with debug logging
./target/release/app-desktop -v

# Run with trace logging  
./target/release/app-desktop -vv

# Reset configuration to defaults
./target/release/app-desktop --reset-config

# Reset config with debug logging
./target/release/app-desktop --reset-config -v
```

## Logging

ZenTerm uses structured logging with different verbosity levels:

- **Default (INFO)**: Basic application lifecycle events
- **Debug (-v)**: Detailed configuration and internal state information  
- **Trace (-vv)**: Maximum verbosity including all internal operations

Logging output includes timestamps and is designed for both development and production use.

## Configuration

### Config File Location

Configuration is automatically managed in the platform-specific config directory:

- **Linux**: `~/.config/zenterm/config.toml`
- **macOS**: `~/Library/Application Support/zenterm/config.toml`  
- **Windows**: `%APPDATA%\zenterm\config.toml`

### Config File Format

The configuration file uses TOML format:

```toml
version = "0.1.0"

[general]
theme = "dark"
gpu_limit_mb = 1024
telemetry = false
```

### Configuration Management

- **Automatic Creation**: Config file is created with defaults if it doesn't exist
- **Corruption Recovery**: Corrupted config files are automatically detected and replaced with defaults (with warning)
- **Atomic Saves**: Configuration changes are saved atomically using temporary files
- **Reset Function**: Use `--reset-config` flag to restore all settings to defaults

The application never crashes due to missing or corrupted configuration files - it gracefully falls back to sensible defaults.

## Development

### Prerequisites

- Rust (latest stable version)
- Platform-specific dependencies for windowing (handled by winit)

### Architecture

The application follows a clean bootstrap pattern:

1. **CLI Parsing**: Command line arguments processed with clap
2. **Logging Init**: Structured logging setup based on verbosity flags  
3. **Config Load**: Configuration loaded/created with error recovery
4. **Reset Flow**: Optional configuration reset if requested
5. **Application Run**: Main application logic (currently GUI, future TUI)
6. **Config Save**: Configuration persisted before clean shutdown

This architecture provides a solid foundation for the upcoming TUI interface while maintaining clean separation of concerns.

## CI

GitHub Actions workflow provides:
- Code formatting checks (`rustfmt`)
- Linting with Clippy (warnings treated as errors)
- Build and test verification on stable Rust

## Future Development

The current implementation provides the foundation for:
- **TUI Interface**: Terminal-based user interface with ratatui (next PR)
- **Settings Panel**: Interactive configuration management
- **Theme System**: Comprehensive theming support
- **Advanced Terminal Features**: Shell integration, advanced rendering

The modular architecture ensures that these features can be added incrementally without disrupting the core functionality.
