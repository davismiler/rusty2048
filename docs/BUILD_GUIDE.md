# Rusty2048 Build Guide

## 🚀 Quick Start

After cloning the repository, you can build and run all versions from the root directory:

```bash
# Build all versions
./build.sh

# Run any version
./run.sh cli      # CLI version
./run.sh web      # Web version
./run.sh desktop  # Desktop version
```

## 📦 Build Scripts

### Root Build Script (`./build.sh`)

The main build script supports building all versions from the root directory:

```bash
# Build all versions (default)
./build.sh

# Build specific versions
./build.sh cli      # CLI version only
./build.sh web      # Web version only
./build.sh desktop  # Desktop version only
./build.sh help     # Show help
```

**Features:**
- ✅ Automatic dependency installation (wasm-pack, tauri-cli)
- ✅ Colored output with status indicators
- ✅ Error handling and validation
- ✅ Cross-platform compatibility
- ✅ Directory validation

### Root Run Script (`./run.sh`)

The run script automatically builds and runs any version:

```bash
# Run CLI version (builds if needed)
./run.sh cli

# Run Web version (builds if needed, serves on localhost:8000)
./run.sh web

# Run Desktop version (builds if needed)
./run.sh desktop
./run.sh help      # Show help
```

**Features:**
- ✅ Automatic building if version not found
- ✅ Smart binary detection
- ✅ Web server management
- ✅ Cross-platform desktop app detection

## 🎯 Build Targets

### CLI Version
- **Output**: `target/release/rusty2048-cli`
- **Dependencies**: Rust toolchain
- **Build Command**: `cargo build --release -p rusty2048-cli`

### Web Version
- **Output**: `web/dist/` (HTML, WASM, assets)
- **Dependencies**: wasm-pack, Python 3
- **Build Command**: `wasm-pack build --target web --out-dir pkg`

### Desktop Version
- **Output**: `desktop/target/release/bundle/`
- **Dependencies**: tauri-cli, system-specific tools
- **Build Command**: `cargo tauri build`

## 🔧 Manual Build Commands

If you prefer to build manually or need more control:

### CLI Version
```bash
# Build
cargo build --release -p rusty2048-cli

# Run
cargo run -p rusty2048-cli
```

### Web Version
```bash
# Install wasm-pack (if not installed)
cargo install wasm-pack

# Build
cd web
wasm-pack build --target web --out-dir pkg
mkdir -p dist
cp index.html dist/
cp test-panic.html dist/
cp -r pkg dist/

# Serve
cd dist && python3 -m http.server 8000
```

### Desktop Version
```bash
# Install Tauri CLI (if not installed)
cargo install tauri-cli

# Build
cd desktop
cargo tauri build

# Run in development mode
cargo tauri dev
```

## 🌐 Running Different Versions

### CLI Version
```bash
./run.sh cli
```
- Runs in terminal
- Full keyboard controls
- All features available (AI, replay, charts)

### Web Version
```bash
./run.sh web
```
- Opens in browser at http://localhost:8000
- Works on desktop and mobile
- Touch/swipe support
- Modern UI with themes

### Desktop Version
```bash
./run.sh desktop
```
- Native desktop application
- System integration
- Offline play
- Cross-platform

## 🛠️ Development Workflow

### For Contributors
```bash
# 1. Clone and setup
git clone <repository>
cd rusty2048

# 2. Build all versions
./build.sh

# 3. Test different versions
./run.sh cli      # Test CLI
./run.sh web      # Test Web
./run.sh desktop  # Test Desktop

# 4. Development mode (for specific version)
cd cli && cargo run
cd web && cargo tauri dev
cd desktop && cargo tauri dev
```

### For Users
```bash
# 1. Clone repository
git clone <repository>
cd rusty2048

# 2. Build and run (one command)
./run.sh web      # For web version
./run.sh cli      # For CLI version
./run.sh desktop  # For desktop version
```

## 🔍 Troubleshooting

### Common Issues

**Build script not found:**
```bash
chmod +x build.sh run.sh
```

**Permission denied:**
```bash
sudo chmod +x build.sh run.sh
```

**Dependencies missing:**
The build script will automatically install:
- `wasm-pack` for Web version
- `tauri-cli` for Desktop version

**Wrong directory:**
Make sure you're in the Rusty2048 root directory:
```bash
ls -la
# Should show: Cargo.toml, cli/, web/, desktop/, build.sh, run.sh
```

### Platform-Specific Notes

**macOS:**
- Desktop app: `desktop/target/release/bundle/macos/Rusty2048.app`
- May need to allow in Security & Privacy settings

**Linux:**
- Desktop app: `desktop/target/release/bundle/linux/Rusty2048`
- May need additional dependencies for Tauri

**Windows:**
- Desktop app: `desktop/target/release/bundle/windows/Rusty2048.exe`
- Requires Visual Studio Build Tools

## 📋 Requirements

### System Requirements
- **Rust**: 1.70+ with Cargo
- **Python**: 3.6+ (for web server)
- **Git**: For cloning repository

### Optional Dependencies
- **wasm-pack**: Auto-installed by build script
- **tauri-cli**: Auto-installed by build script
- **Node.js**: For some development tools

### Platform-Specific
- **macOS**: Xcode Command Line Tools
- **Linux**: Build essentials, GTK development libraries
- **Windows**: Visual Studio Build Tools, WebView2

## 🎉 Success Indicators

After successful builds:

### CLI Version
```
✅ CLI version built successfully!
   Binary location: target/release/rusty2048-cli
   To run: cargo run -p rusty2048-cli
```

### Web Version
```
✅ Web version built successfully!
   Files location: web/dist/
   To serve: cd web/dist && python3 -m http.server 8000
```

### Desktop Version
```
✅ Desktop version built successfully!
   Application location: desktop/target/release/bundle/
   To run in dev mode: cd desktop && cargo tauri dev
```
