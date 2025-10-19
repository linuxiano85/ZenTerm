# ZenTerm

**Birthday MVP** - Linux-first voice-driven terminal ecosystem

> ⚠️ **Early Development**: This is the Birthday MVP release. Not production-ready but architecturally aligned with the project vision.

## Vision

ZenTerm is an ambitious project to create a modern, voice-driven terminal experience for Linux systems. Inspired by tools like Warp, ZenTerm aims to combine the power of traditional terminal environments with modern UI/UX principles and voice control capabilities.

Our vision includes:
- **Voice-first interaction**: Hands-free terminal control using advanced speech recognition
- **GPU-accelerated rendering**: Smooth, responsive UI built with modern graphics APIs
- **Intelligent command palette**: Context-aware command suggestions and automation
- **Linux-native experience**: Built specifically for Linux workflows and ecosystem
- **Developer-friendly**: Extensible architecture for plugins and customization

## Current Status - Birthday MVP

This release represents our initial functional GUI implementation with core subsystems in place:

✅ **Implemented Features:**
- Functional GUI using egui/eframe with sidebar controls
- GPU usage limiting (25/50/75/100% options)
- Theme switching (light/dark mode)
- Voice engine mock (placeholder for future integration)
- Multi-step setup wizard
- Live log panel with real-time updates
- Configuration persistence with debounced saving
- Event-driven architecture using crossbeam channels
- Comprehensive test coverage (39+ unit tests)

🚧 **Planned Features** (see [Roadmap](#roadmap)):
- Real voice recognition integration (Vosk, Whisper)
- TUI mode for terminal-only environments
- Advanced command palette with fuzzy search
- Real GPU monitoring and control
- Plugin system and extensibility
- Telemetry and analytics backend
- Window state persistence

## How to Run

### Prerequisites

- Rust (stable toolchain)
- Linux system with X11 or Wayland
- Basic development tools (pkg-config, build essentials)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/linuxiano85/ZenTerm.git
cd ZenTerm

# Build the project
cargo build

# Launch GUI mode
cargo run -- --gui

# Run tests
cargo test

# Check code quality
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

### Build System

ZenTerm uses a workspace-based Cargo project:

- `apps/desktop/`: GUI application entry point
- `crates/engine/`: Core engine library with all subsystems
- `tests/`: Integration tests

### System Dependencies

On Ubuntu/Debian:
```bash
sudo apt-get install pkg-config libx11-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev
```

## Roadmap

### Phase 1: Core Stabilization
- **Issue #59**: Voice recognition integration (Vosk/Whisper)
- **Issue #60**: TUI mode implementation  
- **Issue #61**: Advanced command palette
- **Issue #62**: Real GPU monitoring/control
- **Issue #63**: Plugin architecture foundation

### Phase 2: User Experience
- Window state persistence
- Custom keybindings
- Theme customization
- Performance optimizations
- Accessibility improvements

### Phase 3: Advanced Features
- Terminal multiplexer integration
- Remote session management
- AI-powered command suggestions
- Telemetry and analytics
- Package manager integration

### Phase 4: Ecosystem
- Community plugins
- Documentation site
- User guides and tutorials
- Integration with popular Linux tools
- Distribution packaging

## Contributing

We welcome contributions! Please ensure your code meets our quality standards:

### Development Workflow

1. **Format**: `cargo fmt --all`
2. **Lint**: `cargo clippy --all-targets -- -D warnings`  
3. **Test**: `cargo test`
4. **Audit**: `cargo audit` (install with `cargo install cargo-audit`)

### Code Guidelines

- All code must pass rustfmt formatting
- Clippy warnings are treated as errors
- Comprehensive test coverage for new features
- Security vulnerabilities must be addressed
- Document public APIs and complex logic

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Ensure CI passes locally
5. Submit pull request with detailed description

## Architecture

ZenTerm follows a modular, event-driven architecture:

```
apps/desktop/          # GUI frontend (egui/eframe)
├── main.rs           # Entry point, CLI parsing
└── gui/              # GUI components

crates/engine/        # Core engine library
├── config.rs         # Configuration management
├── event_bus.rs      # Inter-component communication
├── command_registry.rs # Command system
├── gpu_mock.rs       # GPU control (mock)
├── theme.rs          # Theme management
├── voice_mock.rs     # Voice engine (mock)
├── shared_state.rs   # Application state coordination
└── wizard.rs         # Setup wizard
```

### Key Design Principles

- **Separation of concerns**: GUI and engine are cleanly separated
- **Event-driven**: Components communicate via message passing
- **Thread-safe**: Shared state uses Arc<Mutex<>> for safe access
- **Testable**: Each component has comprehensive unit tests
- **Extensible**: Plugin-ready architecture from day one

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Build Information

- **Version**: 0.1.0-birthday-mvp
- **Build**: Birthday MVP (not production-ready)
- **Target**: Linux x86_64
- **Rust**: Stable toolchain required
