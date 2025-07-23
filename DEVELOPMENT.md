# ZenTerm Development Guide

## Getting Started

ZenTerm is built using Tauri (Rust + Web frontend) to create a modern terminal emulator for Linux.

### Development Environment Setup

1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install Node.js** (v20 or later):
   ```bash
   # Using NodeSource repository
   curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
   sudo apt-get install -y nodejs
   ```

3. **Install System Dependencies**:
   ```bash
   sudo apt update
   sudo apt install -y \
     build-essential \
     curl \
     wget \
     libssl-dev \
     libgtk-3-dev \
     libwebkit2gtk-4.1-dev \
     libappindicator3-dev \
     librsvg2-dev \
     patchelf \
     pkg-config
   ```

4. **Install Tauri CLI**:
   ```bash
   npm install -g @tauri-apps/cli
   ```

### Building and Running

1. **Clone and Setup**:
   ```bash
   git clone <repository-url>
   cd ZenTerm
   npm install
   ```

2. **Development Mode**:
   ```bash
   npm run tauri dev
   ```

3. **Production Build**:
   ```bash
   npm run tauri build
   ```

## Project Structure

```
ZenTerm/
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs              # Application entry point
│   │   ├── lib.rs               # Main application logic
│   │   └── terminal/            # Terminal implementation
│   │       ├── mod.rs           # Module exports
│   │       ├── pty.rs           # PTY management and terminal I/O
│   │       └── session.rs       # Session management
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── src/                         # React frontend
│   ├── components/              # React components
│   │   ├── TerminalView.tsx     # Terminal display component
│   │   └── TabBar.tsx           # Tab management UI
│   ├── types.ts                 # TypeScript definitions
│   ├── App.tsx                  # Main application component
│   └── main.tsx                 # React entry point
├── package.json                 # Node.js dependencies
└── vite.config.ts               # Vite configuration
```

## Key Components

### Backend (Rust)

#### PTY Management (`src-tauri/src/terminal/pty.rs`)
- Handles terminal process spawning using `portable-pty`
- Manages terminal I/O and command execution
- Provides terminal output streaming

#### Session Management (`src-tauri/src/terminal/session.rs`)
- Manages multiple terminal sessions
- Handles session creation, switching, and cleanup
- Maintains session state and metadata

#### Tauri Commands (`src-tauri/src/lib.rs`)
- `create_terminal_session`: Creates new terminal session
- `execute_command`: Executes commands in terminal
- `get_sessions`: Lists all terminal sessions
- `set_active_session`: Switches active session
- `resize_terminal`: Handles terminal resize events
- `close_session`: Closes terminal session

### Frontend (React/TypeScript)

#### TerminalView Component
- Integrates xterm.js for terminal display
- Handles keyboard input and command execution
- Manages terminal themes and styling

#### TabBar Component
- Provides tab interface for multiple sessions
- Handles session switching and creation
- Shows session status and allows closing

## API Reference

### Rust Backend Commands

All commands are exposed via Tauri's IPC mechanism:

```typescript
// Create a new terminal session
const sessionId = await invoke<string>("create_terminal_session", { 
  name: "Terminal 1" 
});

// Execute a command in a session
await invoke("execute_command", {
  sessionId: "session-id",
  command: "ls -la"
});

// Get all sessions
const sessions = await invoke<TerminalSession[]>("get_sessions");

// Switch active session
await invoke("set_active_session", { sessionId: "session-id" });

// Resize terminal
await invoke("resize_terminal", {
  sessionId: "session-id",
  rows: 24,
  cols: 80
});

// Close session
await invoke("close_session", { sessionId: "session-id" });
```

### TypeScript Types

```typescript
interface TerminalSession {
  id: string;
  name: string;
  active: boolean;
  current_directory: string;
  commands: CommandBlock[];
}

interface CommandBlock {
  id: string;
  session_id: string;
  command: string;
  output: TerminalOutput[];
  exit_code?: number;
  start_time: number;
  end_time?: number;
  status: CommandStatus;
}

interface TerminalOutput {
  session_id: string;
  content: string;
  is_error: boolean;
  timestamp: number;
}
```

## Testing

### Manual Testing
1. Start development server: `npm run tauri dev`
2. Test terminal functionality:
   - Create new sessions
   - Execute commands
   - Switch between sessions
   - Close sessions

### Automated Testing (Planned)
- Unit tests for Rust backend
- Integration tests for terminal functionality
- E2E tests for UI components

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and test them
4. Commit with clear messages: `git commit -m "Add feature description"`
5. Push to your fork: `git push origin feature-name`
6. Create a Pull Request

## Troubleshooting

### Common Issues

1. **GTK Dependencies Missing**:
   ```bash
   sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev
   ```

2. **Rust Not Found**:
   ```bash
   source ~/.cargo/env
   ```

3. **Build Failures**:
   - Clear cargo cache: `cargo clean`
   - Update dependencies: `cargo update`
   - Rebuild: `npm run tauri build`

### Debug Mode

Enable debug logging by setting environment variables:
```bash
RUST_LOG=debug npm run tauri dev
```

## Future Development

### Phase 2: Command Blocks
- Visual separation of commands and output
- Command execution status indicators
- Output streaming and buffering

### Phase 3: Smart Features
- Command history with search
- Autocompletion system
- Git integration

### Phase 4: Advanced Features
- AI-powered suggestions
- Plugin system
- Theme customization
- Split panes

## Performance Considerations

- Terminal output streaming for large outputs
- Efficient session management
- Memory management for multiple sessions
- Responsive UI with async operations