# Technical Debt

> **Canonical technical debt register.** Future agents must check this before adding new work.
> Last updated: 2026-07-06

## Debt Summary

| Priority | Count | Total Effort | Target Release |
|---|---|---|---|
| P1 (immediate) | 0 items | — | — |
| P2 (v0.7.0 hardening) | 0 items | — | — |
| P3 (v0.7.0 architecture) | 3 items | ~1 day | v0.7.0 |
| P4 (conditional v1.0+) | 4 items | ~5 days (conditional) | v1.0 (if triggered) |
| P5 (intentional — not debt) | 3 items | N/A | N/A |
| **Total actionable** | **7 items** | **~1.2 weeks** | |

## P1 — Immediate

**None.** All P1 items resolved.

## P2 — v0.7.0 Security Hardening

**None.** All P2 items resolved by PR #29 (maintenance baseline) and Program 1 (host_api modularization). See "Resolved Debt" section below.

## P3 — v0.7.0 Architecture (~1 day)

| # | Debt | Effort | Risk | Notes |
|---|---|---|---|---|
| 13 | Plugin capability manifest enforcement | 6-8 hours | Medium (schema bump) | Add `"capabilities": [...]` to `plugin.json` (schema v2). Enforce in `inject_host_api_with_deadline` in `host_api/mod.rs`. Design approved in Program Transition document. 6 PRs estimated. |
| 14 | `deleteGenericPassword` not implemented in host | 20 min | Low | `copilot/plugin.js:39` calls it; host has no such function. Copilot catches error silently. Will be implemented as part of Program 2. |
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
| 1 | Issue #26 unlabeled and unassigned | Phase B triage | 2026-07-03 |
| 2 | PR #18 CI failing (Bun age gate) | Superseded by PR #29 (manual `@types/node` bump to 26.0.1) | 2026-07-05 |
| 3 | `dangerouslyIgnoreTls` no-localhost guard | PR #29 — localhost-only guard in `host_api/http.rs` | 2026-07-05 |
| 4 | `inject_plist` no plugin_id allowlist | PR #29 — `PLIST_ALLOWED` gate in `host_api/mod.rs` orchestrator | 2026-07-05 |
| 5 | `inject_sqlite.exec` no plugin_id allowlist | PR #29 — `SQLITE_WRITE_ALLOWED` gate in `host_api/sqlite.rs` | 2026-07-05 |
| 6 | 2 SVG icons missing `currentColor` (antigravity, copilot) | PR #29 — both SVGs now use `currentColor` | 2026-07-05 |
| 7 | `tauri-nspanel` git branch pin | PR #29 — forked to `abyssbugg/tauri-nspanel` + pinned by rev | 2026-07-05 |
| 8 | `cargo clippy --fix` (21 warnings) | PR #29 — clippy auto-fix applied | 2026-07-05 |
| 9 | `cargo-audit` not in CI | (Deferred — not blocking; no known vulnerabilities) | — |
| 10 | macOS 27: `keychain_add_generic_password_args` missing `-a account` | PR #29 — `-a account` added in `host_api/keychain.rs` | 2026-07-05 |
| 11 | npm deps refresh (batch PR) | PR #31 (major bumps) + PR #30 (safe bumps) | 2026-07-05 |
| 12 | `host_api.rs` 4,816-LOC monolith | Program 1 (PRs #32–#45) — split into 13 modules under `host_api/` | 2026-07-06 |
| — | Keychain `readGenericPassword` signature mismatch (1-arg call failed) | PR #14 / commit `24e5bfa` (v0.6.28) | 2026-07-02 |
| — | `host_api.rs` URL/path redaction regex required trailing `"` | commit `2a3a7bd` (v0.6.28) | 2026-06-14 |
| — | `providerLoaded` hardcoded to `true` | commit `2a3a7bd` (v0.6.28) | 2026-06-14 |
| — | `ManifestMismatch` triggered for optional metrics | commit `2a3a7bd` (v0.6.28) | 2026-06-14 |
| — | `validate-provider-metadata.mjs` 641 LOC (exceeded 400 guideline) | commit `2a3a7bd` — split into 347 + 309 LOC | 2026-06-14 |
| — | Stale local branches (9 merged branches) | Phase B cleanup (governance audit) | 2026-07-03 |
| — | Stale remotes (`fork`, `upstream` — 75 orphaned refs) | Phase B cleanup (governance audit) | 2026-07-03 |
| — | Superseded stash (`fix/codex-fresh-window` capabilities schema) | Phase B cleanup — dropped (superseded by PR #28) | 2026-07-03 |
