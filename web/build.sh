#!/bin/bash

# Web-specific build script for Rusty2048
# This script builds the WASM module and web application

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

# Main build function
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
    
    print_success "Web version built successfully!"
    echo "   Files location: dist/"
    echo "   To serve: python3 -m http.server 8000 (from dist directory)"
    echo "   Or use: npm run preview"
}

# Development server function
dev_server() {
    print_status "Starting development server..."
    
    # Check if npm is installed
    if ! command_exists npm; then
        print_error "npm is not installed. Please install Node.js and npm first."
        exit 1
    fi
    
    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        print_status "Installing npm dependencies..."
        npm install
    fi
    
    # Check if WASM module exists, build if not
    if [ ! -d "pkg" ] || [ ! -f "pkg/rusty2048_web.js" ]; then
        print_warning "WASM module not found. Building first..."
        wasm-pack build --target web --out-dir pkg
    fi
    
    print_success "Starting Vite development server..."
    echo "🌐 Web version will be available at: http://localhost:5173"
    echo "   Press Ctrl+C to stop the server"
    echo ""
    npm run dev
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  build     Build production version"
    echo "  dev       Start development server"
    echo "  help      Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 build   # Build production version"
    echo "  $0 dev     # Start development server"
}

# Main script
main() {
    echo "🌐 Rusty2048 Web Build Script"
    echo "============================="
    echo ""
    
    # Check if we're in the web directory
    if [ ! -f "package.json" ] || [ ! -f "vite.config.js" ]; then
        print_error "Please run this script from the web directory"
        exit 1
    fi
    
    # Parse command line arguments
    case "${1:-dev}" in
        "build")
            build_web
            ;;
        "dev")
            dev_server
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
