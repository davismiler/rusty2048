#!/bin/bash

# CI Cargo Publish Script for Rusty2048
# Used in GitHub Actions to publish packages to crates.io

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

# Check if CARGO_REGISTRY_TOKEN is set
check_token() {
    if [ -z "$CARGO_REGISTRY_TOKEN" ]; then
        print_error "CARGO_REGISTRY_TOKEN environment variable is not set"
        exit 1
    fi
    print_success "Cargo registry token is configured"
}

# Publish a package with retry mechanism
publish_package() {
    local package_name=$1
    local package_path=$2
    local max_retries=3
    local retry_count=0
    
    print_status "Publishing $package_name..."
    
    cd "$package_path"
    
    while [ $retry_count -lt $max_retries ]; do
        print_status "Attempt $((retry_count + 1)) of $max_retries for $package_name"
        
        # Check if package already exists
        if cargo search "$package_name" --limit 1 | grep -q "^$package_name "; then
            print_warning "$package_name already exists on crates.io"
            print_status "Checking if we need to update version..."
            
            # Get current version from Cargo.toml
            local current_version=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)
            print_status "Current version: $current_version"
            
            # Check if this version already exists
            if cargo search "$package_name" --limit 10 | grep -q "$package_name \"$current_version\""; then
                print_warning "Version $current_version already published, skipping $package_name"
                cd ..
                return 0
            fi
        fi
        
        # Try to publish
        if cargo publish --token "$CARGO_REGISTRY_TOKEN" --allow-dirty; then
            print_success "$package_name published successfully!"
            cd ..
            return 0
        else
            retry_count=$((retry_count + 1))
            if [ $retry_count -lt $max_retries ]; then
                print_warning "Failed to publish $package_name, retrying in 10 seconds..."
                sleep 10
            else
                print_error "Failed to publish $package_name after $max_retries attempts"
                cd ..
                return 1
            fi
        fi
    done
    
    cd ..
}

# Main function
main() {
    echo "🎮 Rusty2048 CI Cargo Publish"
    echo "============================="
    echo ""
    
    # Check prerequisites
    check_token
    
    print_status "Starting publication process..."
    echo ""
    
    # Publish in dependency order with error handling
    if ! publish_package "rusty2048-core" "core"; then
        print_error "Failed to publish core package"
        exit 1
    fi
    
    if ! publish_package "rusty2048-shared" "shared"; then
        print_error "Failed to publish shared package"
        exit 1
    fi
    
    if ! publish_package "rusty2048-cli" "cli"; then
        print_error "Failed to publish CLI package"
        exit 1
    fi
    
    echo ""
    print_success "All packages published to Crates.io successfully!"
    print_status "Users can now install with: cargo install rusty2048-cli"
}

main "$@"
