#!/bin/bash
set -e

cd "$(dirname "$0")/.."

# Clean previous bundle
rm -rf src-tauri/target/release/bundle

# Build
bun tauri build "$@"

echo ""
echo "✓ Build complete! Output:"
ls -la src-tauri/target/release/bundle/dmg/*.dmg 2>/dev/null || ls -la src-tauri/target/release/bundle/macos/*.app
