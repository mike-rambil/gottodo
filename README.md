# GotTodo

A minimal, terminal-based todo list application built with Rust. GotTodo provides a clean, keyboard-driven interface for managing your tasks without leaving the terminal.

## Features

- ✅ **Add/Delete Tasks** - Create and remove tasks with simple keystrokes
- 🔄 **Toggle Completion** - Mark tasks as done/undone
- 👁️ **Hide/Show Interface** - Toggle visibility to work alongside terminal
- 🆘 **Help System** - Built-in keymap reference
- 💾 **Persistent Storage** - Tasks saved to `todos.json`
- 🐛 **Debug Mode** - Optional logging for troubleshooting

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)

### Build from Source
```bash
git clone <repository-url>
cd gottodo
cargo build --release
```

### Run
```bash
cargo run
```

## Usage

### Basic Controls

| Key | Action |
|-----|--------|
| `h` | Show/hide help popup |
| `a` | Add new task |
| `d` | Delete selected task (with confirmation) |
| `Space` | Toggle task completion |
| `↑/↓` | Navigate between tasks |
| `Ctrl+Space` | Hide/show todo interface |
| `q` | Quit application |

### Task Management

1. **Adding Tasks**: Press `a`, type your task, press `Enter` to save or `Esc` to cancel
2. **Deleting Tasks**: Press `d` on selected task, confirm with `y` or cancel with `n`/`Esc`
3. **Marking Complete**: Use `Space` to toggle between `[ ]` and `[x]`

### Interface Modes

- **Normal Mode**: Standard navigation and task management
- **Adding Task**: Text input for new tasks
- **Confirming Delete**: Y/N prompt for task deletion
- **Help Mode**: Overlay showing all keyboard shortcuts

## Advanced Usage

### Debug Mode
```bash
cargo run -- --debug
```
Debug mode shows real-time logging of:
- Key presses and their codes
- State changes (UI toggles, task modifications)
- Navigation movements
- Mode transitions

### Data Storage
Tasks are automatically saved to `todos.json` in the current directory. The file uses a simple JSON format:
```json
[
  {
    "text": "Complete the project",
    "done": false
  },
  {
    "text": "Write documentation", 
    "done": true
  }
]
```

## Development

### Project Structure
```
gottodo/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Rust dependencies
├── Cargo.lock           # Dependency lockfile
├── todos.json           # Task storage (created on first run)
└── README.md           # This file
```

### Dependencies
- `serde` - JSON serialization
- `crossterm` - Cross-platform terminal manipulation
- `ratatui` - Terminal UI framework

### Building
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run with debug logging
cargo run -- --debug
```

## Screenshots

```
┌TODO (h=help)──────────────────────┐
│[ ] Learn Rust                     │
│[x] Build todo app                 │
│[ ] Write documentation            │
│[ ] Deploy to production           │
└───────────────────────────────────┘
```

## License

This project is open source. Feel free to use, modify, and distribute as needed.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Troubleshooting

### Common Issues

**Help popup won't close**: Press any key to dismiss the help overlay

**UI doesn't toggle**: Try using the exact key combination `Ctrl+Space`

**Tasks not saving**: Ensure write permissions in the current directory

**Key not responding**: Use debug mode (`cargo run -- --debug`) to see what keys are being detected

### Debug Information

When issues arise, run with debug mode to see detailed logging:
```bash
cargo run -- --debug
```

The debug panel shows real-time information about key presses, state changes, and application behavior. 