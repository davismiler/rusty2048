#!/bin/bash

# Rusty2048 Cargo Publish Script
# Publishes all packages to crates.io in the correct order

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

# Check if logged in to crates.io
check_login() {
    if ! cargo whoami &>/dev/null; then
        print_error "Not logged in to crates.io"
        print_status "Please run: cargo login"
        print_status "Get your token from: https://crates.io/settings/tokens"
        exit 1
    fi
    
    print_success "Logged in as: $(cargo whoami)"
}

# Check if all changes are committed
check_git_status() {
    if [ -n "$(git status --porcelain)" ]; then
        print_warning "Uncommitted changes detected"
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

# Publish a package
publish_package() {
    local package_name=$1
    local package_path=$2
    
    print_status "Publishing $package_name..."
    
    cd "$package_path"
    
    # Check if package exists on crates.io
    if cargo search "$package_name" --limit 1 | grep -q "^$package_name "; then
        print_warning "$package_name already exists on crates.io"
        read -p "Update version and republish? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_status "Skipping $package_name"
            cd ..
            return 0
        fi
    fi
    
    # Dry run first
    print_status "Running dry run for $package_name..."
    if ! cargo package --allow-dirty; then
        print_error "Dry run failed for $package_name"
        cd ..
        return 1
    fi
    
    # Publish
    print_status "Publishing $package_name to crates.io..."
    if cargo publish --allow-dirty; then
        print_success "$package_name published successfully!"
    else
        print_error "Failed to publish $package_name"
        cd ..
        return 1
    fi
    
    cd ..
}

# Main function
main() {
    echo "🎮 Rusty2048 Cargo Publish Script"
    echo "================================="
    echo ""
    
    # Check prerequisites
    check_login
    check_git_status
    
    print_status "Starting publication process..."
    echo ""
    
    # Publish in dependency order
    publish_package "rusty2048-core" "core"
    publish_package "rusty2048-shared" "shared"
    publish_package "rusty2048-cli" "cli"
    
    echo ""
    print_success "All packages published successfully!"
    print_status "Users can now install with: cargo install rusty2048-cli"
}

main "$@"
