#!/bin/bash

# Build script for Laravel Blade Enhanced Zed Extension

set -e  # Exit on error

echo "🔨 Building Laravel Blade Enhanced extension..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust is not installed"
    echo "Install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check if wasm32-wasip1 target is installed
if ! rustup target list --installed | grep -q "wasm32-wasip1"; then
    echo "📦 Installing wasm32-wasip1 target..."
    rustup target add wasm32-wasip1
fi

# Build the extension
echo "🔧 Compiling extension to WASM..."
cargo build --target wasm32-wasip1 --release

# Check if build succeeded
if [ -f "target/wasm32-wasip1/release/zed_blade_enhanced.wasm" ]; then
    SIZE=$(du -h target/wasm32-wasip1/release/zed_blade_enhanced.wasm | cut -f1)
    echo ""
    echo "✅ Build successful!"
    echo "📦 Extension size: $SIZE"
    echo "📁 Location: target/wasm32-wasip1/release/zed_blade_enhanced.wasm"
    echo ""
    echo "📋 Next steps:"
    echo "  1. Open Zed Editor"
    echo "  2. Press Cmd+Shift+P (Mac) or Ctrl+Shift+P (Linux/Windows)"
    echo "  3. Type 'install dev extension'"
    echo "  4. Select this directory"
    echo ""
else
    echo ""
    echo "❌ Build failed!"
    exit 1
fi
