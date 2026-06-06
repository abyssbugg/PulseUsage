#!/bin/bash
set -e

cd "$(dirname "$0")/.."

# Bundle plugins into Tauri resources before building. Tauri also runs this via
# beforeBuildCommand, but keeping it here makes direct release builds
# self-checking and avoids stale/empty resource bundles.
bun run bundle:plugins

SOURCE_COUNT=$(find plugins -mindepth 1 -maxdepth 1 -type d ! -name mock | wc -l | tr -d ' ')
BUNDLED_COUNT=$(find src-tauri/resources/bundled_plugins -mindepth 2 -maxdepth 2 -name plugin.json 2>/dev/null | wc -l | tr -d ' ')

if [[ "$SOURCE_COUNT" -lt 1 ]]; then
  echo "Error: No source plugins found under plugins/."
  exit 1
fi

if [[ "$BUNDLED_COUNT" -ne "$SOURCE_COUNT" ]]; then
  echo "Error: Bundled plugin count ($BUNDLED_COUNT) does not match source plugin count ($SOURCE_COUNT)."
  exit 1
fi

for REQUIRED_PLUGIN in factory warp; do
  if [[ -d "plugins/$REQUIRED_PLUGIN" && ! -f "src-tauri/resources/bundled_plugins/$REQUIRED_PLUGIN/plugin.json" ]]; then
    echo "Error: Required plugin '$REQUIRED_PLUGIN' was not bundled."
    exit 1
  fi
done

echo "✓ Bundled $BUNDLED_COUNT plugins"

# Clean previous bundle
rm -rf src-tauri/target/release/bundle

# Build
bun tauri build "$@"

echo ""
echo "✓ Build complete! Output:"
ls -la src-tauri/target/release/bundle/dmg/*.dmg 2>/dev/null || ls -la src-tauri/target/release/bundle/macos/*.app
