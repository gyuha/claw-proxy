#!/bin/bash
set -e

# Build the Rust core binary
cd "$(dirname "$0")/../.."
cargo build --release -p claw-proxy-core

# Create binaries directory
mkdir -p "apps/desktop/src-tauri/binaries"

# Copy the binary with platform-specific name
ARCH=$(rustc -Vv | grep host | cut -d' ' -f2)
cp "target/release/claw-proxy" "apps/desktop/src-tauri/binaries/claw-proxy-$ARCH"
echo "Built claw-proxy-$ARCH"
