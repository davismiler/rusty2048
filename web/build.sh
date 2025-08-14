#!/bin/bash

# Build script for Rusty2048 Web version

set -e

echo "🚀 Building Rusty2048 Web version..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Installing..."
    cargo install wasm-pack
fi

# Build the WASM module
echo "📦 Building WASM module..."
wasm-pack build --target web --out-dir pkg

# Create dist directory if it doesn't exist
mkdir -p dist

# Copy HTML and assets to dist
echo "📁 Copying files to dist..."
cp index.html dist/
cp test-panic.html dist/
cp -r pkg dist/

echo "✅ Build complete! Files are in the 'dist' directory."
echo "🌐 To serve the web version, run:"
echo "   cd dist && python3 -m http.server 8000"
echo "   Then open http://localhost:8000 in your browser"
