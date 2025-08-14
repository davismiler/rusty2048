# Rusty2048 Web Version 🌐

This is the web version of Rusty2048, built with Rust and WebAssembly (WASM).

## Features

- **Full Game Logic**: Complete 2048 game implementation in Rust
- **Beautiful UI**: Modern, responsive web interface
- **Theme System**: 5 different themes (Classic, Dark, Neon, Retro, Pastel)
- **Mobile Support**: Touch/swipe gestures for mobile devices
- **Keyboard Controls**: Arrow keys and WASD support
- **Real-time Statistics**: Score, best score, moves tracking
- **Game State Management**: Win/lose detection and messages

## Quick Start

### Prerequisites

- Rust 1.70+
- wasm-pack (will be installed automatically)
- Python 3 (for local server)

### Build and Run

```bash
# Build the web version
./build.sh

# Serve locally
cd dist && python3 -m http.server 8000

# Open in browser
open http://localhost:8000
```

### Alternative Build Methods

```bash
# Using npm scripts
npm run build
npm run serve

# Manual wasm-pack build
wasm-pack build --target web --out-dir pkg
```

## Project Structure

```
web/
├── src/
│   └── lib.rs          # WASM bindings and game logic
├── index.html          # Main HTML file with UI
├── build.sh            # Build script
├── package.json        # NPM configuration
├── pkg/                # Generated WASM files (after build)
└── dist/               # Distribution files (after build)
```

## Technical Details

### WASM Integration

The web version uses `wasm-bindgen` to expose Rust functions to JavaScript:

- `Rusty2048Web`: Main game struct
- `make_move()`: Perform game moves
- `get_board()`: Get current board state
- `get_score()`: Get score information
- `get_theme()`: Get current theme
- `set_theme()`: Change theme

### UI Components

- **Game Grid**: 4x4 tile grid with smooth animations
- **Statistics Panel**: Score, best score, moves display
- **Control Buttons**: New Game, Undo functionality
- **Theme Selector**: 5 theme buttons with live preview
- **Message Display**: Win/lose notifications

### Responsive Design

- **Desktop**: Full keyboard and mouse support
- **Mobile**: Touch gestures and responsive layout
- **Tablet**: Optimized for medium screens

## Controls

### Desktop
- **Arrow Keys** or **WASD**: Move tiles
- **Mouse**: Click buttons for actions
- **Theme Buttons**: Click to switch themes

### Mobile
- **Swipe**: Up/Down/Left/Right to move tiles
- **Touch**: Tap buttons for actions
- **Theme Buttons**: Tap to switch themes

## Themes

1. **Classic**: Original 2048 colors
2. **Dark**: Modern dark theme
3. **Neon**: Vibrant neon colors
4. **Retro**: Terminal green theme
5. **Pastel**: Soft pastel colors

## Development

### Adding New Features

1. **Rust Side**: Add methods to `Rusty2048Web` in `src/lib.rs`
2. **JavaScript Side**: Update `index.html` to use new features
3. **Rebuild**: Run `./build.sh` to compile changes

### Custom Themes

1. Add theme definition in `shared/src/lib.rs`
2. Update `Theme::all_themes()` method
3. Add theme button in `index.html`
4. Rebuild the project

### Performance

- WASM provides near-native performance
- Optimized for 60fps animations
- Efficient memory usage
- Small bundle size (~200KB gzipped)

## Browser Support

- **Modern Browsers**: Chrome, Firefox, Safari, Edge
- **Mobile Browsers**: iOS Safari, Chrome Mobile
- **Requirements**: WebAssembly support

## Deployment

### Static Hosting

```bash
# Build for production
./build.sh

# Deploy dist/ folder to any static host:
# - Netlify
# - Vercel
# - GitHub Pages
# - AWS S3
```

### Docker

```dockerfile
FROM nginx:alpine
COPY dist/ /usr/share/nginx/html/
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

## Troubleshooting

### Common Issues

1. **Build Fails**: Ensure Rust and wasm-pack are installed
2. **WASM Not Loading**: Check browser console for errors
3. **Performance Issues**: Ensure hardware acceleration is enabled
4. **Mobile Gestures**: Test on actual device, not just emulator

### Debug Mode

```bash
# Build with debug symbols
wasm-pack build --target web --out-dir pkg --debug

# Enable console logging
# Add console.log statements in index.html
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes in both Rust and JavaScript
4. Test thoroughly
5. Submit a pull request

---

*Enjoy playing 2048 in your browser!* 🎮✨
