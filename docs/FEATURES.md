# Rusty2048 Features

## ✨ Core Features

- **High Performance**: Built with Rust, zero-cost abstractions
- **Cross-Platform**: Supports CLI, Web, and Desktop versions
- **Multi-language Support**: English and Chinese localization (CLI, Web, Desktop)
- **Modern UI**: Smooth animations and beautiful interfaces
- **Configurable**: Customizable board size, target score, and more
- **Replay System**: Record and replay game sessions (CLI version)
- **AI Mode**: Three AI algorithms with auto-play (CLI and Web versions)
- **Statistics Charts**: Comprehensive game analytics and visualizations (CLI version)
- **Theme System**: 5 beautiful themes (Classic, Dark, Neon, Retro, Pastel)
- **Real-time Statistics**: Display current score, best score, moves, and game duration
- **Game Over Handling**: Show detailed statistics when no moves are possible
- **Victory Notification**: Display victory message when reaching 2048
- **Score Animation**: Score flashes when tiles merge
- **Sound Feedback**: Play bell sound when score increases

## 🎮 Controls

### CLI Version
- **Arrow Keys** or **WASD**: Move tiles
- **R**: Restart game
- **U**: Undo last move
- **T**: Cycle through themes
- **1-5**: Select theme directly (1=Classic, 2=Dark, 3=Neon, 4=Retro, 5=Pastel)
- **H**: Toggle theme help
- **L**: Switch language (English ↔ Chinese)
- **P**: Enter replay mode
- **C**: Toggle statistics charts
- **I**: Toggle AI mode
- **Q** or **ESC**: Quit game

**Replay Mode Controls:**
- **1**: Start recording new game
- **2**: Load and play replay
- **3**: List saved replays
- **4**: Back to main menu
- **Space**: Play/Pause replay
- **Left/Right**: Step through replay
- **+/-**: Adjust replay speed
- **S**: Stop recording

**AI Mode Controls:**
- **O**: Toggle auto-play
- **[ ]**: Switch between AI algorithms (Greedy ↔ Expectimax ↔ MCTS)
- **+/-**: Adjust AI speed (100ms-2000ms)
- **Q/ESC**: Exit immediately (even during auto-play)

**Charts Controls:**
- **Left/Right**: Navigate between chart modes
- **C**: Toggle charts display

### Desktop Version
- **Arrow Keys** or **WASD**: Move tiles
- **Mouse**: Click buttons for New Game, Undo
- **Language Button**: Click to switch language (English ↔ Chinese)
- **Theme Buttons**: Click to switch themes

### Web Version
- **Arrow Keys** or **WASD**: Move tiles
- **Mouse/Touch**: Click buttons for New Game, Undo
- **Language Button**: Click to switch language (English ↔ Chinese)
- **Theme Buttons**: Click to switch themes
- **Mobile**: Swipe gestures supported

## 🎬 Replay System

The CLI version includes a comprehensive replay system that allows you to:

### Features
- **Record Games**: Automatically record all moves during gameplay
- **Save Replays**: Save completed games with metadata
- **Play Back**: Watch replays with full playback controls
- **Speed Control**: Adjust playback speed (0.5x to 4x)
- **Step Through**: Move forward/backward one move at a time
- **File Management**: Organized storage in `cli/replays/` folder

### File Format
Replay files are saved as JSON and include:
- Complete game configuration
- Initial board state
- All moves with timestamps
- Final statistics and metadata
- Player information and notes

### Usage
1. Press **P** during gameplay to enter replay mode
2. Choose **1** to start recording a new game
3. Play normally - all moves are automatically recorded
4. Press **S** to stop recording and save
5. Choose **2** to load and play back saved replays

## 🤖 AI Mode

The CLI and Web versions include an advanced AI system that can play the game automatically:

### AI Algorithms
- **Greedy**: Simple algorithm that chooses the move with highest immediate score
- **Expectimax**: Advanced search algorithm that considers future moves and random tile placements
- **MCTS**: Monte Carlo Tree Search with UCB1 formula for optimal decision making

### Features
- **Auto-play**: Watch AI play the game automatically
- **Speed Control**: Adjust AI move speed from 100ms to 2000ms
- **Algorithm Switching**: Switch between different AI algorithms in real-time
- **Real-time Status**: Display current algorithm, auto-play state, and speed
- **Non-blocking**: AI runs smoothly without blocking user input

### Usage (CLI Version)
1. Press **I** to enter AI mode
2. Press **O** to start auto-play
3. Use **[ ]** to switch between algorithms
4. Use **+/-** to adjust speed
5. Press **Q** to exit at any time

### Usage (Web Version)
1. Click **AI Mode** button to enable AI mode
2. Click **Start Auto-play** to begin automatic gameplay
3. Use **Switch Algorithm** to change AI algorithms
4. Use **Speed +/-** buttons to adjust AI speed
5. Click **Make AI Move** for single moves

## 📊 Statistics Charts

The CLI version includes comprehensive statistics and analytics:

### Chart Types
- **Summary**: Overall statistics including games played, win rate, highest scores
- **Score Trend**: Visual chart showing score progression over last 20 games
- **Efficiency Trend**: Chart displaying efficiency (score per move) over time
- **Tile Achievements**: Bar chart showing how often each tile value was achieved
- **Recent Games**: Table of the last 10 games with detailed statistics

### Features
- **Automatic Recording**: All games are automatically recorded when they end
- **Real-time Updates**: Charts update immediately when new data is available
- **Visual Analytics**: ASCII-based charts for terminal display
- **Data Persistence**: Statistics are saved to `cli/stats.json` for long-term tracking
- **Performance Metrics**: Track efficiency, average scores, and improvement trends

### Usage
1. Press **C** to toggle charts display
2. Use **Left/Right** arrow keys to navigate between chart types
3. Charts show alongside the game board for easy comparison
4. Statistics are automatically updated after each game

## 🎨 Theme System

All versions support 5 beautiful themes:

### Available Themes
1. **Classic**: The original 2048 color scheme with warm tones
2. **Dark**: Dark background with light text, perfect for low-light environments
3. **Neon**: Vibrant neon colors on black background for a modern look
4. **Retro**: Warm retro colors with golden accents
5. **Pastel**: Soft pastel colors for a gentle, calming experience

### Theme Features
- **Instant Switching**: Change themes with a single click or keypress
- **Persistent Selection**: Your theme choice is remembered
- **Complete UI Coverage**: All interface elements adapt to the selected theme
- **Smooth Transitions**: Beautiful color transitions when switching themes

## 🌍 Multi-language Support

### Supported Languages
- **English**: Default language with complete localization
- **Chinese (中文)**: Full Chinese translation for all interface elements

### Language Features
- **Automatic Detection**: Web version detects browser language
- **Easy Switching**: Toggle between languages with a single button
- **Complete Translation**: All text, buttons, and messages are translated
- **Consistent Experience**: Language preference is maintained across sessions

## 📱 Platform Features

### CLI Version
- **Terminal UI**: Beautiful text-based interface using ratatui
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Keyboard Controls**: Full keyboard navigation and shortcuts
- **Advanced Features**: Replay system, AI mode, statistics charts

### Web Version
- **WASM-powered**: High-performance WebAssembly implementation
- **Responsive Design**: Works on desktop, tablet, and mobile
- **Touch Support**: Swipe gestures for mobile devices
- **Modern UI**: Clean, modern interface with smooth animations

### Desktop Version
- **Native App**: Cross-platform desktop application using Tauri
- **System Integration**: Native window management and system integration
- **Offline Play**: No internet connection required
- **Easy Distribution**: Simple installation and updates

## 🔧 Technical Features

### Performance
- **Rust Backend**: High-performance game logic written in Rust
- **Zero-cost Abstractions**: Efficient memory usage and fast execution
- **Optimized Algorithms**: Fast AI algorithms and game mechanics
- **Smooth Animations**: 60fps animations and responsive UI

### Architecture
- **Modular Design**: Clean separation between core logic and UI
- **Cross-platform**: Shared core logic across all platforms
- **Extensible**: Easy to add new features and platforms
- **Well-tested**: Comprehensive test coverage for reliability

### Development
- **Modern Toolchain**: Latest Rust, WebAssembly, and Tauri technologies
- **Hot Reload**: Fast development cycle with hot reloading
- **Debugging Support**: Comprehensive debugging tools and error handling
- **Documentation**: Detailed documentation and examples
