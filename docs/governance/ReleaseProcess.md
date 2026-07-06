# Release Process

## Release Strategy

PulseUsage uses **manual direct-download distribution**. The `publish.yml` GitHub workflow is intentionally disabled (`if: ${{ false }}`) because the app serves 2-5 internal users and notarization is not justified (AGENTS.md: "Simplicity first").

Releases are built locally, tagged, and published via `gh release create` with DMG artifact upload.

## Versioning

PulseUsage follows **semantic versioning** (`MAJOR.MINOR.PATCH`):
- **PATCH** (`0.6.27` → `0.6.28`): bug fixes, hardening, no new providers
- **MINOR** (`0.6.x` → `0.7.0`): new features, new providers, capability model changes
- **MAJOR** (`0.x` → `1.0.0`): breaking changes, enforced notarization/sandbox (conditional)

## Version Files (4 — must all align)

| File | Field |
|---|---|
| `package.json` | `"version": "X.Y.Z"` |
| `src-tauri/Cargo.toml` | `version = "X.Y.Z"` |
| `src-tauri/tauri.conf.json` | `"version": "X.Y.Z"` |
| `src-tauri/Cargo.lock` | `version = "X.Y.Z"` (under `name = "pulseusage"`) |

## Release Sequence

### Prerequisites

- All feature PRs merged to `main`.
- CI green on `main`.
- Production verification report approved.
- Final merge gate review approved.

### Step 1 — Version bump commit

Create a standalone commit on `main` (not via PR):

```bash
git checkout main
git pull --ff-only

# Edit version in all 4 files:
# package.json, src-tauri/Cargo.toml, src-tauri/tauri.conf.json
# Then regenerate Cargo.lock:
cargo update --manifest-path src-tauri/Cargo.toml --package pulseusage --precise X.Y.Z

# Update CHANGELOG.md with a new version entry
# Update docs/release-readiness/vX.Y.Z.md status from "pending" to "released"

git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json src-tauri/Cargo.lock CHANGELOG.md docs/release-readiness/vX.Y.Z.md
git commit -m "chore: bump version to vX.Y.Z"
git push origin main
```

### Step 2 — Tag

Create an annotated tag pointing to the version bump commit:

```bash
git tag -a vX.Y.Z -m "vX.Y.Z

<Summary of release highlights>

Known limitations:
- <Any known limitations>"
git push origin vX.Y.Z
```

### Step 3 — Build release artifact

```bash
# Kill any running PulseUsage instance (avoids DMG bundling conflicts)
pkill -f "PulseUsage.app" 2>/dev/null

# Clean previous bundle
rm -rf src-tauri/target/release/bundle

# Clear any stale hdiutil mounts (DMG creation fails if /Volumes has stale rw images)
hdiutil info | grep "image-path.*rw\." # if any, detach them

# Build the DMG
bun tauri build --bundles dmg
# Or: ./scripts/build-release.sh
```

The DMG is created at `src-tauri/target/release/bundle/dmg/PulseUsage_X.Y.Z_aarch64.dmg`.

### Step 4 — Verify artifact

```bash
# Verify DMG exists and get SHA256
ls -la src-tauri/target/release/bundle/dmg/PulseUsage_*.dmg
shasum -a 256 src-tauri/target/release/bundle/dmg/PulseUsage_*.dmg

# Mount and verify contents
hdiutil attach src-tauri/target/release/bundle/dmg/PulseUsage_*.dmg -nobrowse -readonly
plutil -p "/Volumes/PulseUsage/PulseUsage.app/Contents/Info.plist" | grep -i "version"
ls "/Volumes/PulseUsage/PulseUsage.app/Contents/Resources/resources/bundled_plugins/" | wc -l  # should be 18
hdiutil detach /Volumes/PulseUsage
```

Verify:
- `CFBundleShortVersionString` = `X.Y.Z`
- `CFBundleVersion` = `X.Y.Z`
- 18 bundled plugins (including required `factory` + `warp`)
- Tray icon present
- Applications symlink present

### Step 5 — Publish GitHub release

```bash
gh release create vX.Y.Z \
  --repo abyssbugg/PulseUsage \
  --title "vX.Y.Z — <release summary>" \
  --notes-file - <<'NOTES'
## vX.Y.Z

<Release notes — copy from CHANGELOG entry>

### Install
Download `PulseUsage_X.Y.Z_aarch64.dmg` (Apple Silicon / arm64). Open the DMG and drag PulseUsage to Applications. The app is unsigned — right-click → Open on first launch to bypass Gatekeeper.

### Verification
- N tests pass (X JS + Y Rust + Z validator)
- CI green on merge commit and version bump
- DMG SHA256: <sha256>
NOTES

# Upload the DMG
gh release upload vX.Y.Z src-tauri/target/release/bundle/dmg/PulseUsage_*.dmg --repo abyssbugg/PulseUsage

# Verify assets uploaded
gh release view vX.Y.Z --repo abyssbugg/PulseUsage --json assets
```

### Step 6 — Release report

Produce a release report including:
- Merge commit SHA
- Version bump commit SHA
- Release tag
- Release artifact names + SHA256
- Validation summary
- Remaining known limitations
- Follow-up work recommended for next version

## DMG Bundling Troubleshooting

### "No space left on device" during DMG creation

**Cause:** Stale read/write DMG images mounted from previous failed builds.

**Fix:**
```bash
# List stale mounts
hdiutil info | grep "image-path.*rw\."

# Detach each stale mount
hdiutil detach /Volumes/dmg.XXX -force

# Clean stale rw files
rm -f src-tauri/target/release/bundle/macos/rw.*.dmg

# Retry
bun tauri build --bundles dmg
```

### DMG bundling fails transiently

Retry with `bun tauri build --bundles dmg`. The `bundle_dmg.sh` script occasionally fails on first run; a clean retry succeeds.

## Pre-Release Checklist

- [ ] All feature PRs merged to main
- [ ] CI green on main
- [ ] Production verification report approved
- [ ] Final merge gate review approved
- [ ] Version bump commit created (all 4 files aligned)
- [ ] CHANGELOG entry added
- [ ] Release readiness doc updated (status: released)
- [ ] Annotated tag created and pushed
- [ ] DMG built and verified (SHA256, version, plugins)
- [ ] GitHub release created with notes
- [ ] DMG artifact uploaded to release
- [ ] Release report produced

## Post-Release Cleanup

- [ ] Delete merged feature branches (local + remote)
- [ ] Remove merged worktrees
- [ ] Drop superseded stashes
- [ ] Sync local main to origin/main
- [ ] Verify `git status` is clean (only untracked planning docs allowed)

## Release Artifacts (v0.6.28 — current)

| Artifact | Value |
|---|---|
| DMG | `PulseUsage_0.6.28_aarch64.dmg` |
| Size | 11,500,195 bytes (11.5 MB) |
| SHA256 | `326da0423b25578236fd35abd5ed8f33ee0d3d22851039856e9c2184ed97e0ec` |
| Architecture | arm64 (Apple Silicon) |
| Bundle ID | `com.abyssbugg.pulseusage` |
| Version | 0.6.28 |
| Release URL | https://github.com/abyssbugg/PulseUsage/releases/tag/v0.6.28 |

## Future Release Considerations

### v0.7.0
- Merge 6 pending dependabot PRs
- npm deps refresh (batch PR)
- Provider capability model (schema v2)
- host_api modularization (Program 1 complete — 13 modules under `host_api/`)

### v1.0.0 (conditional — only if triggered)
- Notarization (if macOS 27 enforces or user base grows >10)
- App Sandbox (if macOS 27 enforces)
- Process-exec native crates (`plist`, `rusqlite`, `security-framework`)
- Strict-mode provider validation CI-enforced

Do NOT implement these until triggered. AGENTS.md: "Simplicity first: handle only important cases."