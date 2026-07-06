# IMP-005: PulseUsage to PulseBar Migration Plan

| Field | Value |
|-------|-------|
| **Status** | Approved (planning only — execution forbidden until v0.7.0 stabilizes) |
| **Date** | 2026-07-03 |
| **Author** | Main Engineering Agent |
| **Related EDR** | [EDR-001](./001-pulsebar-direction.md) (PulseBar Product & Architecture Direction — Approved) |
| **Prerequisite** | v0.7.0 release complete |

## Goal

Migrate the PulseUsage codebase to the PulseBar identity (per EDR-001) with zero user-visible data loss, zero provider auth loss, and a clean rollback path.

## Scope

### In scope
- All 1,006 occurrences of "PulseUsage" / "pulseusage" across the codebase
- Bundle identifier change (`com.abyssbugg.pulseusage` → `com.abyssbugg.pulsebar`)
- Keychain service name migration (`PulseUsage-copilot` → `PulseBar-copilot`)
- JavaScript global rename (`__pulseusage_plugin` → `__pulsebar_plugin`, `__pulseusage_ctx` → `__pulsebar_ctx`)
- Cargo crate name (`pulseusage` → `pulsebar`)
- npm package name (`pulseusage` → `pulsebar`)
- Documentation, README, CHANGELOG, governance docs
- Release artifacts (DMG name, GitHub release)
- CI/CD (if any references exist — currently none direct)

### Out of scope
- Historical release artifacts (v0.0.1–v0.6.28 DMGs are immutable — keep PulseUsage names)
- Historical git tags (immutable)
- The actual rename execution (this is a plan only — execution requires separate approval post-v0.7.0)

## Complete Migration Inventory

**Total occurrences: 1,006** (excluding `node_modules`, `target`, `.git`)

### Category 1: Configuration & Identity (HIGH impact — rename-critical)

| File | Occurrences | Current Value | Target Value | Complexity |
|---|---|---|---|---|
| `src-tauri/tauri.conf.json` | 3 | `productName: "PulseUsage"`, `identifier: "com.abyssbugg.pulseusage"`, window `title: "PulseUsage"` | `PulseBar`, `com.abyssbugg.pulsebar`, `PulseBar` | **Low** — single file, 3 edits. `identity.rs` sources from `productName`. |
| `src-tauri/Cargo.toml` | 3 | `name = "pulseusage"`, `description`, `name = "pulseusage_lib"` | `pulsebar`, `pulsebar_lib` | **Medium** — crate rename changes artifact names; requires `cargo update` + Cargo.lock regen. |
| `package.json` | 1 | `"name": "pulseusage"` | `"pulsebar"` | **Medium** — npm package name; `bun.lock` regen. |
| `src-tauri/Cargo.lock` | 2 | `name = "pulseusage"`, `version` | Auto-regenerates from Cargo.toml | **Auto** |
| `index.html` | 1 | Page title | `PulseBar` | **Low** |

### Category 2: Rust Source (11 files)

| File | Occurrences | Nature | Complexity |
|---|---|---|---|
| `src-tauri/src/identity.rs` | 2 | `RELEASES_URL` constant (`github.com/abyssbugg/PulseUsage/releases`) + doc comment | **Low** — designed for rename. Update URL + comment. |
| `src-tauri/src/lib.rs` | 1 | `log::info!("PulseUsage v{} starting", version)` | **Low** — use `identity::app_display_name()`. |
| `src-tauri/src/panel.rs` | 4 | `PulseUsagePanel`, `PulseUsagePanelEventHandler` (Rust type names) | **Medium** — type renames; mechanical but touches multiple call sites. |
| `src-tauri/src/plugin_engine/host_api/` (all modules) | (present) | User-Agent strings, log prefixes | **Low** — mostly via `plugin_id` and `app_display_name`. |
| `src-tauri/src/plugin_engine/runtime.rs` | (present) | `__pulseusage_ctx`, `__pulseusage_plugin` global names | **HIGH** — **JS-visible global names**. Changing breaks all 19 plugins. Must coordinate with plugin migration. |
| `src-tauri/src/plugin_engine/diagnostics.rs` | (present) | Redaction patterns | **Low** — follows plugin_id. |
| `src-tauri/src/diagnostics.rs` | (present) | Various | **Low** |
| `src-tauri/src/local_http_api/cache.rs` | (present) | Various | **Low** |
| `src-tauri/src/config.rs` | (present) | Config paths | **Low-Medium** — config dir name may change (user state). |
| `src-tauri/src/log_path.rs` | (present) | Log directory name | **Low-Medium** — log dir may change. |
| `src-tauri/src/main.rs` | (present) | Entry point | **Low** |

### Category 3: React/TypeScript Source (17 files)

| File | Occurrences | Nature | Complexity |
|---|---|---|---|
| `src/components/about-dialog.tsx` | 3 | `<h2>PulseUsage</h2>`, `alt`, GitHub link | **Low** — 3 edits. |
| `src/components/panel-footer.tsx` | 1 | `PulseUsage {version}` | **Low** — use centralized name. |
| `src/pages/settings.tsx` | 1 | `PulseUsage starts when you sign in` | **Low** — string edit. |
| `src/components/side-nav.tsx` | 1 | GitHub issues link | **Low** — URL update. |
| `src/components/changelog-dialog.tsx` | (present) | Changelog title | **Low** |
| `src/hooks/use-changelog.ts`, `use-tray-icon.ts` | (present) | Various | **Low** |
| `src/lib/tray-tooltip.ts`, `provider-diagnostics.ts` | (present) | Tooltip + diagnostics | **Low** |
| Test files (`.test.tsx`, `.test.ts`) | (present) | Mock assertions, `__pulseusage_plugin` global | **Medium** — test fixtures referencing the global name. |

### Category 4: Plugins (41 files — HIGHEST complexity)

| Nature | Occurrences | Complexity | Migration Risk |
|---|---|---|---|
| **Keychain service names** (`PulseUsage-copilot`) | 1 plugin (copilot) | **HIGH** | Keychain items stored under service name. Renaming orphans existing entries. Requires migration logic. |
| **User-Agent headers** (`"User-Agent": "PulseUsage"`) | 6 plugins (codex, grok, perplexity, kimi, factory, kiro) | **Medium** | Provider APIs may log UA; change is safe but visible. |
| **`__pulseusage_plugin` global** | All 19 plugins + test-helpers | **HIGH** | JS global registered by every plugin. Must match `runtime.rs`. Coordinated rename. |
| **`PulseUsage/{version}` UA string** (kiro) | 1 plugin | **Low** | Follows UA pattern. |
| **Log messages** | multiple | **Low** | String edits. |

### Category 5: Documentation (31 files)

| File | Occurrences | Complexity |
|---|---|---|
| `README.md` | 11 | **Low** — string edits (screenshot ref, download link, GitHub link) |
| `CONTRIBUTING.md` | 7 | **Low** |
| `AGENTS.md` | 1 | **Low** |
| `CHANGELOG.md` | 0 | None (no PulseUsage refs) |
| `docs/providers/*.md`, `docs/governance/*.md`, `docs/edr/*.md`, `docs/imp/*.md` | (present) | **Low-Medium** — many docs, mechanical edits |

### Category 6: CI/CD, Scripts, Release Artifacts

| File/Artifact | Occurrences | Complexity |
|---|---|---|
| `.github/workflows/*.yml` | 0 direct refs | None |
| `scripts/build-release-guards.test.sh` | 1 (`PulseUsage.app` path) | **Low** |
| `scripts/validate-provider-metadata.test.mjs` | 2 (`pulseusage-provider-validator-` tempdir, `__pulseusage_plugin`) | **Low-Medium** |
| **Historical release artifacts** (DMGs) | All v0.0.1–v0.6.28 | **None** — immutable. New releases use new name. |
| **Screenshots** | `screenshot.png` | **Low** — re-screenshot after rename (or keep if UI unchanged). |
| **Icons** | `src-tauri/icons/*`, `public/*` | **None** — icons use `currentColor`, no text. |
| **Bundle identifier** | `com.abyssbugg.pulseusage` | **HIGH** — orphans app preferences, keychain items, app state. |

## Dependency Graph

```
tauri.conf.json (productName, identifier)
    ↓
identity.rs (sources from productName) ← lib.rs, about-dialog, panel-footer, etc.
    ↓
Cargo.toml (crate name) → Cargo.lock (auto-regen)
package.json (npm name) → bun.lock (auto-regen)
    ↓
runtime.rs (__pulseusage_ctx, __pulseusage_plugin globals)
    ↓ (MUST coordinate)
plugins/*.js (__pulseusage_plugin global) + test-helpers.js + plugin.test.js
    ↓
copilot/plugin.js (PulseUsage-copilot keychain service)
    ↓ (MUST migrate keychain entries)
user keychain (existing PulseUsage-copilot entries)
    ↓
bundle identifier (com.abyssbugg.pulseusage)
    ↓ (MUST migrate preferences + app state)
user preferences, app support dir, log dir
```

## Migration Phases

### Phase 1 — Display Name & Identity (LOW risk, ~1 hour)

**What:** Change the user-visible name everywhere it's sourced from `tauri.conf.json` `productName`.

**Files:**
- `src-tauri/tauri.conf.json`: `productName` → `PulseBar`, window `title` → `PulseBar`
- `src-tauri/src/identity.rs`: `RELEASES_URL` → `github.com/abyssbugg/PulseBar/releases`, update doc comment
- Verify `identity::app_display_name()` now returns "PulseBar" everywhere it's used

**Risk:** Low. No bundle ID change yet. App appears as "PulseBar" in menu bar, About dialog, panel footer.

**Test:** Launch app, verify "PulseBar" appears in all UI surfaces. All tests pass.

### Phase 2 — Rust Type Names (MEDIUM risk, ~2 hours)

**What:** Rename Rust types `PulseUsagePanel` → `PulseBarPanel`, `PulseUsagePanelEventHandler` → `PulseBarPanelEventHandler`.

**Files:**
- `src-tauri/src/panel.rs`: type definitions + all call sites
- Any other files referencing these types

**Risk:** Medium. Mechanical but touches multiple files. Compiler catches all misses.

**Test:** `cargo build` + `cargo test` green.

### Phase 3 — React UI Strings & Docs (LOW risk, ~2 hours)

**What:** Update all hardcoded "PulseUsage" strings in React components and documentation.

**Files:**
- `src/components/about-dialog.tsx`, `panel-footer.tsx`, `side-nav.tsx`, `changelog-dialog.tsx`
- `src/pages/settings.tsx`
- `src/hooks/*.ts`, `src/lib/*.ts` (string references)
- `README.md`, `CONTRIBUTING.md`, `AGENTS.md`, `docs/**/*.md`
- Test files (string assertions)

**Risk:** Low. String edits. Tests need assertion updates.

**Test:** `bun run test` green. Manual UI review.

### Phase 4 — JavaScript Globals (HIGH risk, ~3 hours)

**What:** Rename `__pulseusage_plugin` → `__pulsebar_plugin`, `__pulseusage_ctx` → `__pulsebar_ctx` across the runtime and all 19 plugins + test-helpers.

**Files:**
- `src-tauri/src/plugin_engine/runtime.rs`: global registration + patch functions
- `src-tauri/src/plugin_engine/host_api/` (all modules): `__pulseusage_ctx` references
- `plugins/*/plugin.js` (all 19): `globalThis.__pulseusage_plugin` registration
- `plugins/test-helpers.js`: test global setup
- `plugins/*/plugin.test.js` (all): test assertions
- `scripts/validate-provider-metadata.test.mjs`: test global

**Risk:** HIGH. All 19 plugins + runtime + tests must change simultaneously. A partial rename breaks plugins.

**Strategy:** Single coordinated commit. No deprecation period (the globals are internal, not a public API). Rename in one atomic PR.

**Test:** `bun run test` + `cargo test` green. Live provider probes succeed.

### Phase 5 — Keychain Service Migration (HIGH risk, ~2 hours + migration logic)

**What:** Rename `PulseUsage-copilot` → `PulseBar-copilot` keychain service. Migrate existing keychain entries.

**Files:**
- `plugins/copilot/plugin.js`: `KEYCHAIN_SERVICE` constant

**Migration logic (in copilot plugin):**
```javascript
const OLD_KEYCHAIN_SERVICE = "PulseUsage-copilot";
const NEW_KEYCHAIN_SERVICE = "PulseBar-copilot";

function migrateKeychainEntry(ctx) {
  try {
    const oldRaw = ctx.host.keychain.readGenericPassword(OLD_KEYCHAIN_SERVICE);
    if (oldRaw) {
      ctx.host.keychain.writeGenericPassword(NEW_KEYCHAIN_SERVICE, oldRaw);
      // Optionally delete old entry (requires deleteGenericPassword host impl — P3 debt #14)
      ctx.host.log.info("migrated PulseUsage keychain entry to PulseBar");
    }
  } catch (e) {
    // Old entry doesn't exist — no migration needed
  }
}
```

**Risk:** HIGH. If migration fails, users must re-auth with GitHub. Mitigation: keep the read-from-old-name fallback for one release.

**Strategy:** Read old name → write new name → use new name. Don't delete old entry until v0.9.0 (deprecation period).

**Test:** Verify existing Copilot users retain their token after upgrade.

### Phase 6 — Bundle Identifier (HIGH risk, ~1 hour + migration logic)

**What:** Change `com.abyssbugg.pulseusage` → `com.abyssbugg.pulsebar` in `tauri.conf.json`.

**Impact:**
- macOS treats the new ID as a **different app**.
- User preferences (`~/Library/Preferences/com.abyssbugg.pulseusage.plist`) are orphaned.
- App support dir (`~/Library/Application Support/com.abyssbugg.pulseusage/`) is orphaned.
- Keychain items (if any stored under the bundle ID) are orphaned.
- Log dir (`~/Library/Logs/PulseUsage/` or similar) may be orphaned.

**Migration strategy:**
Option A (recommended): **Keep bundle ID stable.** Only change `productName` (display name). The app shows as "PulseBar" but macOS still knows it as `com.abyssbugg.pulseusage`. Zero user state loss. The bundle ID is internal — users never see it.
Option B: **Change bundle ID + migration helper.** Ship a one-time migration that copies preferences from old ID to new ID. Higher risk.

**Recommendation:** Option A. The bundle ID is not user-visible. Changing it provides no benefit and risks user state loss. Keep `com.abyssbugg.pulseusage` as the internal ID; only change the display name to "PulseBar".

**If Option B is chosen:**
- `tauri.conf.json`: `identifier` → `com.abyssbugg.pulsebar`
- Migration helper (Rust, on first launch): copy `~/Library/Preferences/com.abyssbugg.pulseusage.plist` → `com.abyssbugg.pulsebar.plist`, copy app support dir, copy log dir.
- Risk: ~2-3 hours additional work + edge cases (permissions, concurrent instances).

### Phase 7 — Crate & npm Package Name (MEDIUM risk, ~1 hour)

**What:** Rename Cargo crate `pulseusage` → `pulsebar`, npm package `pulseusage` → `pulsebar`.

**Files:**
- `src-tauri/Cargo.toml`: `name`, `name = "pulseusage_lib"` → `pulsebar`, `pulsebar_lib`
- `package.json`: `"name"` → `"pulsebar"`
- Regenerate `Cargo.lock` (`cargo update --package pulsebar --precise <version>`)
- Regenerate `bun.lock` (`bun install`)

**Risk:** Medium. Artifact names change (`pulseusage` binary → `pulsebar` binary). CI/release scripts may reference the old name. Check `scripts/build-release.sh` and `scripts/build-release-guards.test.sh`.

**Test:** `cargo build` + `bun run build` green. DMG name becomes `PulseBar_X.Y.Z_aarch64.dmg`.

### Phase 8 — Release Artifacts & Branding (LOW risk, ~30 min)

**What:** New releases use "PulseBar" in DMG name, GitHub release title, release notes.

**Files:**
- `docs/governance/ReleaseProcess.md`: update artifact name examples
- Release notes template: "PulseBar" instead of "PulseUsage"
- Screenshots: re-capture if UI changed (optional — if only the name changed, old screenshots may still work if the UI is identical)

**Historical artifacts:** v0.0.1–v0.6.28 DMGs remain `PulseUsage_*` — they are immutable. Do not re-tag or re-release.

**Risk:** Low. New releases naturally use the new name.

## Rollback Strategy

### If migration breaks something critical:

1. **Revert the migration PR** (or the specific phase commit). Since all phases are atomic commits, revert is clean.
2. **Tag a hotfix release** (e.g., v0.7.1) from the reverted main.
3. **Keychain entries:** If Phase 5 migrated entries to `PulseBar-copilot`, the old `PulseUsage-copilot` entries still exist (we don't delete in the same release). Reverting restores the old read path.
4. **Bundle ID (Option A):** No rollback needed — bundle ID never changed.
5. **Bundle ID (Option B):** Users who migrated to `com.abyssbugg.pulsebar` lose preferences on revert. Mitigation: the migration helper should be bidirectional (or just don't use Option B).

### Rollback test

Before executing the migration, verify that reverting each phase commit restores the previous state cleanly. Test on a clean machine + a machine with existing PulseUsage preferences.

## Compatibility Strategy

### Backward compatibility

- **Keychain:** Read from old `PulseUsage-copilot` as fallback for one release (v0.8.0). Delete old entry only in v0.9.0.
- **Preferences (Option A — recommended):** No change to bundle ID → preferences are automatically compatible.
- **Preferences (Option B):** Migration helper copies old → new on first launch.
- **Plugins:** All plugins update in one atomic PR. No backward-compatible global name period (globals are internal).
- **HTTP API (port 6736):** No change — the local HTTP API doesn't reference the product name in its interface.

### Forward compatibility

- Once migrated, the codebase is PulseBar-only. No dual-name support beyond the keychain fallback period.

## Testing Strategy

### Pre-migration (on v0.7.0 baseline)

1. Full test suite green: `bun run test` + `cargo test` + `bun run test:provider-validator` + `bash scripts/build-release-guards.test.sh`
2. Live provider verification on macOS 27: all providers that worked on v0.7.0 still work.
3. Snapshot baseline: capture screenshots, log output, keychain entries for comparison.

### Per-phase validation

| Phase | Validation |
|---|---|
| Phase 1 (display name) | UI shows "PulseBar" everywhere; `bun run test` green |
| Phase 2 (Rust types) | `cargo build` + `cargo test` green |
| Phase 3 (UI strings + docs) | `bun run test` green; manual UI review; docs review |
| Phase 4 (JS globals) | `bun run test` + `cargo test` green; **all 19 plugins load**; live probes succeed |
| Phase 5 (keychain) | **Existing Copilot users retain token** (migration logic works); new users can auth |
| Phase 6 (bundle ID) | App launches; preferences preserved (Option A) or migrated (Option B); keychain accessible |
| Phase 7 (crate/npm name) | `cargo build` + `bun run build` green; DMG named `PulseBar_*` |
| Phase 8 (release artifacts) | GitHub release title is "PulseBar"; DMG download works |

### Post-migration (full verification)

1. Clean install on fresh macOS 27 machine: app launches, all providers work.
2. Upgrade install (existing PulseUsage v0.7.0 → PulseBar v0.8.0): preferences retained, Copilot token retained.
3. Full test suite green.
4. Production verification report (per [docs/governance/ReleaseProcess.md](../governance/ReleaseProcess.md)).

## Release Strategy

- **Release as v0.8.0** (PulseBar) — separate from v0.7.0 (PulseUsage).
- **Do NOT combine with v0.7.0** — isolate the rename risk.
- **Release notes:** Clearly communicate the rename to users. Explain that preferences and Copilot tokens are retained.
- **DMG:** `PulseBar_0.8.0_aarch64.dmg`
- **Tag:** `v0.8.0` (annotated)
- **GitHub release:** "PulseBar v0.8.0 — formerly PulseUsage"

## Special Attention Items

### Bundle identifier
- **Recommendation: Option A (keep `com.abyssbugg.pulseusage`)** — change only `productName` to "PulseBar". Zero user state loss. The bundle ID is internal.
- If Option B is chosen, implement a migration helper and accept the risk.

### Keychain service migration
- `PulseUsage-copilot` → `PulseBar-copilot` with read-old-write-new fallback for one release.
- Requires `deleteGenericPassword` host implementation (P3 debt #14) to clean up old entries in v0.9.0.

### JavaScript globals
- `__pulseusage_plugin` → `__pulsebar_plugin`, `__pulseusage_ctx` → `__pulsebar_ctx`
- **Single atomic PR** — no partial rename. All 19 plugins + runtime + tests change simultaneously.

### Provider plugins
- User-Agent headers (`"User-Agent": "PulseUsage"`) → `"PulseBar"` in 6 plugins. Safe but visible in provider API logs.
- Log messages in plugins → string edits.

### Cargo package names
- `pulseusage` → `pulsebar`, `pulseusage_lib` → `pulsebar_lib`
- Regenerate `Cargo.lock`. Verify `scripts/build-release.sh` doesn't hardcode the old binary name.

### npm package names
- `pulseusage` → `pulsebar`
- Regenerate `bun.lock`. The npm package is private (`"private": true`) — no registry impact.

### Application identity
- `src-tauri/src/identity.rs` is already designed for this rename. Update `RELEASES_URL` + doc comment.
- `app_display_name()` automatically returns the new `productName`.

### Updater
- No auto-updater implemented (`createUpdaterArtifacts: false`). Users manually download new releases. No updater migration needed.

### Release assets
- Historical DMGs (v0.0.1–v0.6.28) are immutable — keep `PulseUsage_*` names.
- New releases: `PulseBar_X.Y.Z_aarch64.dmg`.

### Documentation
- All docs (README, CONTRIBUTING, AGENTS, governance, EDR, IMP, provider docs) → string edits.
- CHANGELOG: add v0.8.0 entry noting the rename.

### CI/CD
- `.github/workflows/*.yml` have 0 direct `PulseUsage` references. No CI changes needed.
- `scripts/build-release-guards.test.sh` references `PulseUsage.app` — update test fixture.

### Branding
- Icons use `currentColor` — no text in icons. No icon changes needed.
- Screenshots: re-capture if the UI changed; if only the name changed, old screenshots may still work (the name appears in the menu bar and About dialog, which may not be in the screenshot).

## Estimated Total Effort

| Phase | Effort | Risk |
|---|---|---|
| Phase 1 (display name) | 1 hour | Low |
| Phase 2 (Rust types) | 2 hours | Medium |
| Phase 3 (UI strings + docs) | 2 hours | Low |
| Phase 4 (JS globals) | 3 hours | HIGH |
| Phase 5 (keychain) | 2 hours + migration logic | HIGH |
| Phase 6 (bundle ID — Option A) | 0 hours (no change) | None |
| Phase 6 (bundle ID — Option B) | 3 hours + migration logic | HIGH |
| Phase 7 (crate/npm name) | 1 hour | Medium |
| Phase 8 (release artifacts) | 30 min | Low |
| **Total (Option A)** | **~11.5 hours** | |
| **Total (Option B)** | **~14.5 hours** | |

## Approval Gate

This plan is **Approved for planning**. Execution requires:
1. v0.7.0 release complete.
2. Separate explicit approval to begin execution.
3. EDR-001 amendment (if scope changes from this plan).
