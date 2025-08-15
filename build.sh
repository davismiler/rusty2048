#!/bin/bash

# Rusty2048 Build Script
# Build all versions (CLI, Web, Desktop) from the root directory

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}🚀${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Function to build CLI version
build_cli() {
    print_status "Building CLI version..."
    
    # Build CLI version
    cargo build --release -p rusty2048-cli
    
    print_success "CLI version built successfully!"
    echo "   Binary location: target/release/rusty2048-cli"
    echo "   To run: cargo run -p rusty2048-cli"
}

# Function to build Web version
build_web() {
    print_status "Building Web version..."
    
    # Check if wasm-pack is installed
    if ! command_exists wasm-pack; then
        print_warning "wasm-pack is not installed. Installing..."
        cargo install wasm-pack
    fi
    
    # Check if npm is installed
    if ! command_exists npm; then
        print_error "npm is not installed. Please install Node.js and npm first."
        exit 1
    fi
    
    # Navigate to web directory
    cd web
    
    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        print_status "Installing npm dependencies..."
        npm install
    fi
    
    # Build the WASM module
    print_status "Building WASM module..."
    wasm-pack build --target web --out-dir pkg
    
    # Build the web application with Vite
    print_status "Building web application with Vite..."
    npm run build
    
    # Create _headers file for Vercel
    print_status "Creating _headers file..."
    cat > dist/_headers << EOF
/*.wasm
  Content-Type: application/wasm
EOF
    
    # Return to root directory
    cd ..
    
    print_success "Web version built successfully!"
    echo "   Files location: web/dist/"
    echo "   To serve: cd web/dist && python3 -m http.server 8000"
    echo "   Or use: npm run preview (from web directory)"
}

# Function to build Desktop version
build_desktop() {
    print_status "Building Desktop version..."
    
    # Check if Tauri CLI is installed
    if ! command_exists tauri; then
        print_warning "Tauri CLI is not installed. Installing..."
        cargo install tauri-cli
    fi
    
    # Navigate to desktop directory
    cd desktop
    
    # Build the desktop application
    print_status "Building desktop application..."
    cargo tauri build
    
    # Return to root directory
    cd ..
    
    print_success "Desktop version built successfully!"
    echo "   Application location: desktop/target/release/bundle/"
    echo "   To run in dev mode: cd desktop && cargo tauri dev"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  cli       Build CLI version only"
    echo "  web       Build Web version only"
    echo "  desktop   Build Desktop version only"
    echo "  all       Build all versions (default)"
    echo "  help      Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0          # Build all versions"
    echo "  $0 cli      # Build CLI version only"
    echo "  $0 web      # Build Web version only"
    echo "  $0 desktop  # Build Desktop version only"
}

# Main script
main() {
    echo "🎮 Rusty2048 Build Script"
    echo "=========================="
    echo ""
    
    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ] || [ ! -d "cli" ] || [ ! -d "web" ] || [ ! -d "desktop" ]; then
        print_error "Please run this script from the Rusty2048 root directory"
        exit 1
    fi
    
    # Parse command line arguments
    case "${1:-all}" in
        "cli")
            build_cli
            ;;
        "web")
            build_web
            ;;
        "desktop")
            build_desktop
            ;;
        "all")
            print_status "Building all versions..."
            echo ""
            build_cli
            echo ""
            build_web
            echo ""
            build_desktop
            echo ""
            print_success "All versions built successfully!"
            ;;
        "help"|"-h"|"--help")
            show_usage
            ;;
        *)
            print_error "Unknown option: $1"
            echo ""
            show_usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
