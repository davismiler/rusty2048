# Rusty2048 🎮

A modern implementation of the 2048 game written in Rust, supporting multi-platform deployment.

## ✨ Features

- **High Performance**: Built with Rust, zero-cost abstractions
- **Cross-Platform**: Supports CLI, Web, and Desktop versions
- **Modern UI**: Smooth animations and beautiful interfaces
- **Configurable**: Customizable board size, target score, and more
- **Replay System**: Record and replay game sessions
- **AI Mode**: Simple AI algorithm demonstration
- **Theme System**: Multiple color schemes

## 🏗️ Project Architecture

```
rusty2048/
├── core/           # Core game logic library
├── cli/            # Command-line version (TUI)
├── web/            # Web version (WASM)
├── desktop/        # Desktop version (Tauri)
└── shared/         # Shared components
```

## 🚀 Quick Start

### CLI Version

```bash
# Build and run the CLI version
cargo run -p rusty2048-cli
```

### Web Version

```bash
# Build the web version
cd web && ./build.sh

# Serve the web version locally
cd web/dist && python3 -m http.server 8000
# Then open http://localhost:8000 in your browser
```

### Controls

#### CLI Version
- **Arrow Keys** or **WASD**: Move tiles
- **R**: Restart game
- **U**: Undo last move
- **T**: Cycle through themes
- **1-5**: Select theme directly (1=Classic, 2=Dark, 3=Neon, 4=Retro, 5=Pastel)
- **H**: Toggle theme help
- **Q** or **ESC**: Quit game

#### Web Version
- **Arrow Keys** or **WASD**: Move tiles
- **Mouse/Touch**: Click buttons for New Game, Undo
- **Theme Buttons**: Click to switch themes
- **Mobile**: Swipe gestures supported

### Game Features

- **Real-time Statistics**: Display current score, best score, moves, and game duration
- **Game Over Handling**: Show detailed statistics when no moves are possible
- **Victory Notification**: Display victory message when reaching 2048
- **Score Animation**: Score flashes when tiles merge
- **Sound Feedback**: Play bell sound when score increases
- **Theme System**: 5 beautiful themes with different color schemes

## 🛠️ Development

### Requirements

- Rust 1.70+
- Cargo

### Project Structure

- `core/`: Core game logic, including board, moves, scoring, etc.
- `cli/`: Command-line interface using ratatui and crossterm
- `web/`: Web version using wasm-bindgen
- `desktop/`: Desktop version using Tauri

### Testing

```bash
# Run all tests
cargo test

# Run benchmarks
cargo bench

# Run property tests
cargo test --features proptest
```

## 📦 Build Targets

### CLI Version
```bash
cargo build --release -p rusty2048-cli
```

### Web Version
```bash
cd web && ./build.sh
# Files will be generated in web/dist/
```

### Desktop Version (Planned)
```bash
cargo tauri build -p rusty2048-desktop
```

## 🎯 Development Roadmap

- [x] Core game logic
- [x] CLI version basic functionality
- [x] CLI version game over handling
- [x] CLI version score statistics and animations
- [x] CLI version theme system
- [x] Web version (WASM)
- [ ] Desktop version (Tauri)
- [ ] Replay system
- [ ] AI mode
- [ ] Statistics charts
- [ ] Multi-language support

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Enjoy the game!** 🎉
