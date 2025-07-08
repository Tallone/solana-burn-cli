#!/bin/bash

# Test build script for Solana Burn CLI
# This script tests the build process on different platforms

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to test build
test_build() {
    local target=$1
    local description=$2
    
    print_info "Testing build for $description ($target)"
    
    if cargo build --release --target "$target"; then
        print_success "Build successful for $target"
        
        # Check if binary exists
        local binary_name="solana-burn-cli"
        if [[ "$target" == *"windows"* ]]; then
            binary_name="solana-burn-cli.exe"
        fi
        
        local binary_path="target/$target/release/$binary_name"
        if [[ -f "$binary_path" ]]; then
            print_success "Binary found: $binary_path"
            
            # Get binary size
            local size=$(du -h "$binary_path" | cut -f1)
            print_info "Binary size: $size"
            
            # Test help command (if not cross-compiling)
            if [[ "$target" == "$(rustc -vV | grep host | cut -d' ' -f2)" ]]; then
                print_info "Testing help command..."
                if "$binary_path" --help > /dev/null 2>&1; then
                    print_success "Help command works"
                else
                    print_warning "Help command failed (may be expected for cross-compiled binaries)"
                fi
            fi
        else
            print_error "Binary not found: $binary_path"
            return 1
        fi
    else
        print_error "Build failed for $target"
        return 1
    fi
}

# Main function
main() {
    print_info "Solana Burn CLI Build Test"
    print_info "=========================="
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Please install Rust."
        exit 1
    fi
    
    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]]; then
        print_error "Cargo.toml not found. Please run this script from the project root."
        exit 1
    fi
    
    print_info "Rust version: $(rustc --version)"
    print_info "Cargo version: $(cargo --version)"
    
    # Detect host platform
    local host_target=$(rustc -vV | grep host | cut -d' ' -f2)
    print_info "Host target: $host_target"
    
    # Test native build first
    print_info ""
    test_build "$host_target" "Native"
    
    # Test other targets if available
    local targets_to_test=()
    
    case "$host_target" in
        *linux*)
            targets_to_test+=("x86_64-unknown-linux-musl")
            ;;
        *darwin*)
            if [[ "$host_target" == "x86_64-apple-darwin" ]]; then
                targets_to_test+=("aarch64-apple-darwin")
            elif [[ "$host_target" == "aarch64-apple-darwin" ]]; then
                targets_to_test+=("x86_64-apple-darwin")
            fi
            ;;
    esac
    
    # Test additional targets
    for target in "${targets_to_test[@]}"; do
        print_info ""
        if rustup target list --installed | grep -q "$target"; then
            test_build "$target" "Cross-compile"
        else
            print_warning "Target $target not installed, skipping"
            print_info "To install: rustup target add $target"
        fi
    done
    
    print_info ""
    print_success "Build test completed!"
    
    # Show available binaries
    print_info ""
    print_info "Available binaries:"
    find target -name "solana-burn-cli*" -type f -executable 2>/dev/null | while read -r binary; do
        local size=$(du -h "$binary" | cut -f1)
        print_info "  $binary ($size)"
    done
}

# Run main function
main "$@"
