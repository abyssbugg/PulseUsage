#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_DIR="$(mktemp -d)"

cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

mkdir -p \
  "$TMP_DIR/bin" \
  "$TMP_DIR/scripts" \
  "$TMP_DIR/plugins/factory" \
  "$TMP_DIR/src-tauri/resources/bundled_plugins"

cp "$ROOT_DIR/scripts/build-release.sh" "$TMP_DIR/scripts/build-release.sh"

cat > "$TMP_DIR/bin/bun" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if [[ "$1" == "run" && "$2" == "bundle:plugins" ]]; then
  mkdir -p src-tauri/resources/bundled_plugins/factory
  printf '{}\n' > src-tauri/resources/bundled_plugins/factory/plugin.json
  exit 0
fi

if [[ "$1" == "tauri" && "$2" == "build" ]]; then
  mkdir -p src-tauri/target/release/bundle/macos/PulseUsage.app
  exit 0
fi

echo "Unexpected bun command: $*" >&2
exit 64
EOF
chmod +x "$TMP_DIR/bin/bun"

OUTPUT_FILE="$TMP_DIR/output.txt"

if PATH="$TMP_DIR/bin:$PATH" "$TMP_DIR/scripts/build-release.sh" --bundles app > "$OUTPUT_FILE" 2>&1; then
  cat "$OUTPUT_FILE"
  echo "Expected build-release.sh to fail when a required source plugin is missing." >&2
  exit 1
fi

if ! grep -q "Required plugin 'warp' not found in source" "$OUTPUT_FILE"; then
  cat "$OUTPUT_FILE"
  echo "Expected missing source plugin error for warp." >&2
  exit 1
fi

echo "✓ build-release.sh fails when required source plugins are missing"
