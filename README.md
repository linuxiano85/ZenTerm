# ZenTerm - Modern Terminal Emulator for Linux

ZenTerm is a modern, feature-rich terminal emulator for Linux that provides a clean interface with intelligent features. Built with Rust and React, it offers a modern alternative to traditional terminal emulators.

## Features

### Current Implementation (Phase 1)
- ✅ **Modern UI**: Clean interface with tab support
- ✅ **Terminal Emulation**: Full terminal emulation using portable-pty
- ✅ **Session Management**: Create, switch, and close terminal sessions
- ✅ **Dark Theme**: Modern Tokyo Night inspired color scheme
- ✅ **Tauri Framework**: Native performance with web technologies

### Planned Features
- 🔄 **Command Blocks**: Visual separation of commands and output
- 🔄 **Syntax Highlighting**: Real-time command syntax highlighting
- 🔄 **Smart Autocompletion**: Context-aware suggestions
- 🔄 **Git Integration**: Branch and status display in prompt
- 🔄 **AI Integration**: Intelligent command suggestions
- 🔄 **Plugin System**: Extensible architecture
- 🔄 **Themes**: Multiple built-in themes and customization
- 🔄 **Split Panes**: Multiple terminals in one window

## Architecture

```
ZenTerm/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── terminal/    # Terminal emulation logic
│   │   │   ├── mod.rs
│   │   │   ├── pty.rs   # PTY management
│   │   │   └── session.rs # Session management
│   │   └── lib.rs
├── src/                 # Frontend (React/TypeScript)
│   ├── components/      # UI components
│   │   ├── TerminalView.tsx
│   │   └── TabBar.tsx
│   ├── types.ts         # TypeScript definitions
│   └── App.tsx          # Main application
└── dist/                # Built frontend assets
```

## Technology Stack

- **Backend**: Rust with Tauri framework
- **Frontend**: React with TypeScript
- **Terminal**: xterm.js with portable-pty
- **Styling**: Custom CSS with modern design
- **Build System**: Vite for frontend, Cargo for backend

## Development

### Prerequisites
- Node.js (v20+)
- Rust (latest stable)
- System dependencies:
  ```bash
  sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
  ```

### Building
```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Project Status
- ✅ **Phase 1 Complete**: Core terminal functionality and modern UI
- 🔄 **Phase 2 In Progress**: Command blocks and syntax highlighting
- 📋 **Phase 3 Planned**: Smart features and Git integration
- 📋 **Phase 4 Planned**: AI integration and plugin system

## License

GPL v3 - See LICENSE file for details.

## Contributing

This is an open-source project. Contributions are welcome!

## Screenshots

*Screenshots will be added once the GUI components are fully functional in a display environment.*