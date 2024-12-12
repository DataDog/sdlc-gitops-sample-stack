#!/bin/bash
set -e

# Map TARGETPLATFORM to Rust target
case "$TARGETPLATFORM" in
    "linux/amd64")
        export RUST_TARGET="x86_64-unknown-linux-gnu"
        ;;
    "linux/arm64")
        export RUST_TARGET="aarch64-unknown-linux-gnu"
        ;;
    *)
        echo "Unsupported platform: $TARGETPLATFORM"
        exit 1
        ;;
esac

echo "Building for $RUST_TARGET"

echo "Fetching dependencies..."
cargo fetch --target "$RUST_TARGET"

# Create a stubbed main to ensure everything builds and is cached
echo "Creating stub main.rs..."
mkdir -p src && echo "fn main() {}" > src/main.rs

# Build the project
echo "Building project..."
cargo build --release --target "$RUST_TARGET"

# Clean up the stub main.rs
echo "Cleaning up..."
rm src/main.rs

echo "Build completed for target: $RUST_TARGET"



