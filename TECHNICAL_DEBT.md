# Technical Debt

> **Canonical technical debt register.** Future agents must check this before adding new work.
> Last updated: 2026-07-03

## Debt Summary

| Priority | Count | Total Effort | Target Release |
|---|---|---|---|
| P1 (immediate) | 2 items | ~10 min | Now |
| P2 (v0.7.0 hardening) | 8 items | ~3 hours | v0.7.0 |
| P3 (v0.7.0 architecture) | 5 items | ~1.5 days | v0.7.0 |
| P4 (conditional v1.0+) | 4 items | ~5 days (conditional) | v1.0 (if triggered) |
| P5 (intentional — not debt) | 3 items | N/A | N/A |
| **Total actionable** | **19 items** | **~2 weeks** | |

## P1 — Immediate (now, ~10 min)

| # | Debt | Effort | Risk | Notes |
|---|---|---|---|---|
| 1 | Issue #26 unlabeled and unassigned | 5 min | None | Add labels `hardening`, `antigravity`; assign owner |
| 2 | PR #18 CI failing (Bun age gate on `@types/node@^26.0.0`) | 5 min | None | Rebase: `@dependabot recreate` or `gh pr update-branch 18` |

## P2 — v0.7.0 Security Hardening (~3 hours)

| # | Debt | Effort | Risk | Evidence |
|---|---|---|---|---|
| 3 | `dangerouslyIgnoreTls` no-localhost guard | 20 min | Near-zero | `host_api.rs:916-918` — `danger_accept_invalid_certs(true)` when flag set; antigravity uses for localhost only but no guard prevents non-localhost misuse |
| 4 | `inject_plist` no plugin_id allowlist | 30 min | Very low | `host_api.rs:782` — any plugin can `host.plist.read("/any/path.plist")`; only warp uses it |
| 5 | `inject_sqlite.exec` no plugin_id allowlist | 30 min | Low | `host_api.rs:2855` — write-capable SQL, only cursor uses it; no DB path allowlist |
| 6 | 2 SVG icons missing `currentColor` (antigravity, copilot) | 10 min | None | AGENTS.md compliance — icon theming breaks for these 2 providers |
| 7 | `tauri-nspanel` git branch pin (reproducibility) | 30 min | Low | `Cargo.toml` pins `branch = "v2.1"` (mutable); fork to `abyssbugg/tauri-nspanel` + pin by `rev` |
| 8 | `cargo clippy --fix` (21 warnings, 15 auto-fixable) | 10 min | None | 11 collapsible-if, 2 `&PathBuf`→`&Path`, 2 unneeded unit, etc. |
| 9 | `cargo-audit` not in CI | 20 min | None | Install `cargo-audit`, add `cargo audit` step to `ci.yml` |
| 10 | **macOS 27: `keychain_add_generic_password_args` missing `-a account`** | 30 min + verify | Low | `host_api.rs:190` — builds args without `-a`; macOS 27 `security add-generic-password` now requires `-a`. Copilot write fails (read works after v0.6.28 fix). Probe succeeds via gh CLI fallback. |

## P3 — v0.7.0 Architecture (~1.5 days)

| # | Debt | Effort | Risk | Notes |
|---|---|---|---|---|
| 11 | npm deps refresh (batch PR) | 2 hours | Medium | `@base-ui/react` 1.1→1.6 (major), `lucide-react` 1.7→1.21 (major), tailwind 4.1.18→4.3.1, react 19.2.4→19.2.7, zustand 5.0.11→5.0.14, `@tauri-apps/cli` 2.10→2.11.3 |
| 12 | `host_api.rs` 4,727-LOC monolith | 4-6 hours | Low (pure extraction) | Split into `host_api/{log,fs,plist,crypto,env,http,ls,ccusage,keychain,sqlite}.rs` + `redaction.rs`. Violates AGENTS.md 400-LOC guideline. |
| 13 | Plugin capability manifest enforcement | 6-8 hours | Medium (schema bump) | Add `"capabilities": [...]` to `plugin.json` (schema v2). Enforce in `inject_host_api_with_deadline`. PR-1 (capability contracts) is opt-in metadata; enforcement is v0.7.0. |
| 14 | `deleteGenericPassword` not implemented in host | 20 min | Low | `copilot/plugin.js:39` calls it; host has no such function. Copilot catches error silently. |
| 15 | Perplexity `Agentic Research` unclassified | 1 day (research) | None | No evidence of response shape. Strict-mode validator fails by design. Default-mode passes with 1 warning. |

## P4 — Conditional v1.0+ (only if triggered, ~5 days)

| # | Debt | Effort | Trigger | Notes |
|---|---|---|---|---|
| 16 | Process-exec native crates (`plist`, `rusqlite`, `security-framework`) | 2-3 days | Energy complaints OR plugin count >25 | Replace subprocess spawning with Rust-native crates. Medium-high regression risk. |
| 17 | Notarization | 1 day + $99/year | macOS 27 enforces OR user base >10 | Re-enable `publish.yml` with Apple Developer Program secrets. |
| 18 | App Sandbox | 2-3 days | macOS 27 enforces | Add entitlements + `expand_path` audit. High risk — every path needs entitlement. |
| 19 | PulseBar rename (1,006 occurrences) | 1-2 days + migration logic | Post-v0.7.0, when approved | See [IMP-005](./docs/imp/005-pulsebar-migration-plan.md). Bundle ID change orphans user state. |

## P5 — Intentional Design Choices (NOT debt — do not "fix")

| # | Item | Rationale |
|---|---|---|
| 20 | `publish.yml` disabled (`if: ${{ false }}`) | Manual release process. 2-5 internal users. Notarization not justified. AGENTS.md: simplicity first. |
| 21 | App unsigned / not notarized | Direct-download. Users bypass Gatekeeper manually. Intentional. |
| 22 | Strict-mode provider validation not CI-enforced | Intentional migration path. Default-mode enforced in CI. Strict-mode deferred to v0.8.0 (after Perplexity evidence). |

## Debt Resolution Tracking

When a debt item is resolved:
1. Move it to "Resolved" section below with the commit/PR that fixed it.
2. Update [PROJECT_STATUS.md](./PROJECT_STATUS.md) if the resolution changes repository health.

## Resolved Debt (historical)

| # | Debt | Resolved By | Date |
|---|---|---|---|
| — | Keychain `readGenericPassword` signature mismatch (1-arg call failed) | PR #14 / commit `24e5bfa` (v0.6.28) | 2026-07-02 |
| — | `host_api.rs` URL/path redaction regex required trailing `"` | commit `2a3a7bd` (v0.6.28) | 2026-06-14 |
| — | `providerLoaded` hardcoded to `true` | commit `2a3a7bd` (v0.6.28) | 2026-06-14 |
| — | `ManifestMismatch` triggered for optional metrics | commit `2a3a7bd` (v0.6.28) | 2026-06-14 |
| — | `validate-provider-metadata.mjs` 641 LOC (exceeded 400 guideline) | commit `2a3a7bd` — split into 347 + 309 LOC | 2026-06-14 |
| — | Stale local branches (9 merged branches) | Phase B cleanup (governance audit) | 2026-07-03 |
| — | Stale remotes (`fork`, `upstream` — 75 orphaned refs) | Phase B cleanup (governance audit) | 2026-07-03 |
| — | Superseded stash (`fix/codex-fresh-window` capabilities schema) | Phase B cleanup — dropped (superseded by PR #28) | 2026-07-03 |
