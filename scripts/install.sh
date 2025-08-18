#!/bin/bash

# Rusty2048 CLI Installation Script
# Supports multiple installation methods

set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

# Detect operating system
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        CYGWIN*|MINGW*|MSYS*) echo "windows";;
        *)          echo "unknown";;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64)     echo "x86_64";;
        aarch64)    echo "aarch64";;
        arm64)      echo "aarch64";;
        *)          echo "unknown";;
    esac
}



# Download pre-compiled binary
download_binary() {
    local os=$(detect_os)
    local arch=$(detect_arch)
    local version=${1:-"latest"}
    
    print_status "Downloading pre-compiled binary..."
    
    # Create temporary directory
    local temp_dir=$(mktemp -d)
    cd "$temp_dir"
    
    # Download binary for the corresponding platform
    local download_url="https://github.com/honkinglin/rusty2048/releases/download/v${version}/rusty2048-cli-${os}-${arch}.tar.gz"
    
    if curl -L -o rusty2048.tar.gz "$download_url"; then
        tar -xzf rusty2048.tar.gz
        chmod +x rusty2048
        
        # Move to system path
        if [ "$os" = "macos" ]; then
            sudo mv rusty2048 /usr/local/bin/
        else
            sudo mv rusty2048 /usr/bin/
        fi
        
        print_success "Installation complete! Use 'rusty2048' command to start the game"
    else
        print_error "Download failed. Please check your network connection or version number"
        return 1
    fi
    
    # Clean up temporary files
    cd - > /dev/null
    rm -rf "$temp_dir"
}

# Install using package manager
install_with_package_manager() {
    local os=$(detect_os)
    
    case "$os" in
        "macos")
            if command -v brew &> /dev/null; then
                print_status "Installing with Homebrew..."
                brew install honkinglin/tap/rusty2048-cli
                print_success "Installation complete!"
                return 0
            fi
            ;;
        "linux")
            if command -v apt &> /dev/null; then
                print_status "Installing with apt..."
                sudo apt update
                sudo apt install rusty2048-cli
                print_success "Installation complete!"
                return 0
            elif command -v yum &> /dev/null; then
                print_status "Installing with yum..."
                sudo yum install rusty2048-cli
                print_success "Installation complete!"
                return 0
            fi
            ;;
    esac
    
    return 1
}

# Main function
main() {
    echo "🎮 Rusty2048 CLI Installer"
    echo "=========================="
    echo ""
    
    # Check if already installed
    if command -v rusty2048 &> /dev/null; then
        print_warning "Rusty2048 CLI is already installed"
        read -p "Reinstall? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 0
        fi
    fi
    
    echo "Choose installation method:"
    echo "1. Download pre-compiled binary (Recommended)"
    echo "2. Use system package manager"
    echo "3. Exit"
    echo ""
    
    read -p "Please choose (1-3): " choice
    
    case $choice in
        1)
            read -p "Enter version number (leave empty for latest): " version
            download_binary "$version"
            ;;
        2)
            if ! install_with_package_manager; then
                print_error "System package manager not supported on this platform"
                exit 1
            fi
            ;;
        3)
            print_status "Exiting installation"
            exit 0
            ;;
        *)
            print_error "Invalid choice"
            exit 1
            ;;
    esac
}

main "$@"
