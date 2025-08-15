#!/bin/bash

# Rusty2048 Run Script
# Quick start script to run different versions of the game

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

# Function to check if binary exists
binary_exists() {
    [ -f "$1" ]
}

# Function to run CLI version
run_cli() {
    print_status "Starting CLI version..."
    
    # Check if CLI binary exists
    if binary_exists "target/release/rusty2048-cli"; then
        print_success "Running CLI version..."
        ./target/release/rusty2048-cli
    else
        print_warning "CLI binary not found. Building first..."
        cargo build --release -p rusty2048-cli
        print_success "Running CLI version..."
        ./target/release/rusty2048-cli
    fi
}

# Function to run Web version
run_web() {
    print_status "Starting Web version..."
    
    # Check if web dist exists
    if [ -d "web/dist" ]; then
        print_success "Serving Web version..."
        echo "🌐 Web version will be available at: http://localhost:8000"
        echo "   Press Ctrl+C to stop the server"
        echo ""
        cd web/dist && python3 -m http.server 8000
    else
        print_warning "Web dist not found. Building first..."
        ./build.sh web
        print_success "Serving Web version..."
        echo "🌐 Web version will be available at: http://localhost:8000"
        echo "   Press Ctrl+C to stop the server"
        echo ""
        cd web/dist && python3 -m http.server 8000
    fi
}

# Function to run Desktop version
run_desktop() {
    print_status "Starting Desktop version..."
    
    # Check if desktop app exists
    if [ -d "desktop/target/release/bundle" ]; then
        print_success "Running Desktop version..."
        # Try to find and run the desktop app
        if [ -f "desktop/target/release/bundle/macos/Rusty2048.app/Contents/MacOS/Rusty2048" ]; then
            ./desktop/target/release/bundle/macos/Rusty2048.app/Contents/MacOS/Rusty2048
        elif [ -f "desktop/target/release/bundle/linux/Rusty2048" ]; then
            ./desktop/target/release/bundle/linux/Rusty2048
        elif [ -f "desktop/target/release/bundle/windows/Rusty2048.exe" ]; then
            ./desktop/target/release/bundle/windows/Rusty2048.exe
        else
            print_warning "Desktop app not found. Building first..."
            ./build.sh desktop
            print_success "Desktop version built. Please run the app manually from:"
            echo "   desktop/target/release/bundle/"
        fi
    else
        print_warning "Desktop app not found. Building first..."
        ./build.sh desktop
        print_success "Desktop version built. Please run the app manually from:"
        echo "   desktop/target/release/bundle/"
    fi
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  cli       Run CLI version"
    echo "  web       Run Web version (serves on localhost:8000)"
    echo "  desktop   Run Desktop version"
    echo "  help      Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 cli      # Run CLI version"
    echo "  $0 web      # Run Web version"
    echo "  $0 desktop  # Run Desktop version"
    echo ""
    echo "Note: If the version is not built, it will be built automatically."
}

# Main script
main() {
    echo "🎮 Rusty2048 Run Script"
    echo "======================="
    echo ""
    
    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ] || [ ! -d "cli" ] || [ ! -d "web" ] || [ ! -d "desktop" ]; then
        print_error "Please run this script from the Rusty2048 root directory"
        exit 1
    fi
    
    # Parse command line arguments
    case "${1:-}" in
        "cli")
            run_cli
            ;;
        "web")
            run_web
            ;;
        "desktop")
            run_desktop
            ;;
        "help"|"-h"|"--help")
            show_usage
            ;;
        "")
            echo "Please specify which version to run:"
            echo ""
            show_usage
            exit 1
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
