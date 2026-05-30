#!/bin/bash
# Build script for RustyOS

set -e

echo "🔨 Building RustyOS..."
echo ""

# Install required tools
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust toolchain not found. Please install from https://rustup.rs"
    exit 1
fi

# Build bootloader
echo "📦 Building bootimage..."
cargo install bootimage --quiet 2>/dev/null || true

# Build kernel
echo "🔧 Building kernel..."
cargo build --release

# Create bootable image
echo "💿 Creating bootable ISO..."
bootimage build --release

echo ""
echo "✅ Build complete!"
echo ""
echo "📁 Output: target/x86_64-unknown-none/release/boot-image.bin"
echo ""
echo "🚀 To run in QEMU:"
echo "   qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/boot-image.bin"
echo ""
echo "💾 To create ISO:"
echo "   bootimage build --release"
