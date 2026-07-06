# Roadmap

> **Canonical project roadmap.** Future agents must verify work aligns with this document.
> Last updated: 2026-07-06

## Current Milestone

**Stabilization Milestone** (pre-v0.7.0 release) — Program 1 complete, Program 2 design approved, documentation synchronization in progress.

## Phase Plan

### Phase A — Immediate Stabilization ✅ COMPLETE

| Task | Status | Effort |
|---|---|---|
| Commit governance documentation | ✅ Complete (commit `e749c72`) | 10 min |
| Establish Project Control Center | ✅ Complete (6 control-center docs) | 30 min |
| Create IMP-005 (PulseBar migration plan) | ✅ Complete | 1 hour |
| Triage issue #26 (add labels, assign) | ✅ Complete | 5 min |
| Rebase PR #18 (`@types/node` age gate) | ✅ Superseded by PR #29 (manual bump to 26.0.1) | — |

### Phase B — Security Hardening + macOS 27 Compatibility ✅ COMPLETE

All 8 hardening items resolved by PR #29 (merged 2026-07-05):
- ✅ `dangerouslyIgnoreTls` localhost-only guard — `host_api/http.rs`
- ✅ `inject_plist` plugin_id allowlist — `host_api/mod.rs` orchestrator (`PLIST_ALLOWED`)
- ✅ `inject_sqlite.exec` plugin_id allowlist — `host_api/sqlite.rs` (`SQLITE_WRITE_ALLOWED`)
- ✅ 2 SVG `currentColor` fixes (antigravity, copilot)
- ✅ `tauri-nspanel` fork + pin by rev
- ✅ `cargo clippy --fix` (21 warnings resolved)
- ✅ macOS 27: `keychain_add_generic_password_args` add `-a account` — `host_api/keychain.rs`
- ⬜ `cargo-audit` CI step (deferred — not blocking)

### Phase C — v0.7.0 Architecture Evolution ✅ MOSTLY COMPLETE

| Task | Status | Notes |
|---|---|---|
| npm deps refresh (batch) | ✅ Complete | PR #30 (safe bumps) + PR #31 (major bumps) |
| host_api modularization | ✅ Complete | Program 1 — PRs #32–#45. 4,816-LOC monolith → 13 modules |
| Plugin capability manifest enforcement | ⬜ Design approved | Program 2 — 6 PRs, ~9 hours. Not started |
| `deleteGenericPassword` host implementation | ⬜ Pending | Will be part of Program 2 |
| Perplexity `Agentic Research` classification | ⬜ Deferred | Needs research — no evidence of response shape |

### Phase D — v0.7.0 Release ⬜ PENDING

- Version bump to `0.7.0` (4 files + CHANGELOG + release-readiness doc)
- Tag `v0.7.0` + build DMG + publish release
- See [RELEASE_PLAN.md](./RELEASE_PLAN.md)
- **Prerequisite:** Program 2 (capability enforcement) must be complete

### Phase E — PulseBar Migration ⬜ FORBIDDEN

- Execute [IMP-005](./docs/imp/005-pulsebar-migration-plan.md) (PulseUsage → PulseBar)
- Release as v0.8.0 (or v1.0.0 if combined with notarization)
- **Not started** — forbidden until v0.7.0 stabilizes

### Phase F — v1.0 (conditional — only if triggered)

- Process-exec native crates (`plist`, `rusqlite`, `security-framework`) — if energy complaints or plugin count >25
- Notarization — if macOS 27 enforces or user base >10
- App Sandbox — if macOS 27 enforces
- **Do NOT implement until triggered** (AGENTS.md: simplicity first)

## Program 1 Completion Details

| Metric | Value |
|---|---|
| PRs merged | #30–#45 (16 PRs) |
| Original `host_api.rs` | 4,816 LOC |
| Final `host_api/mod.rs` | 1,736 LOC (112 production + 1,624 tests) |
| Modules extracted | 13 |
| Modularization percentage | 97.7% |
| Behavior changes | 0 |
| Public API changes | 0 |

### Final module layout
```
src-tauri/src/plugin_engine/host_api/
├── mod.rs         1,736 LOC (orchestration + tests)
├── redaction.rs      ~200 LOC  (redaction regexes)
├── shared.rs         ~100 LOC  (expand_path, ProbeDeadline, etc.)
├── logging.rs         48 LOC  (ctx.host.log)
├── fs.rs              90 LOC  (ctx.host.fs)
├── plist.rs           50 LOC  (ctx.host.plist)
├── crypto.rs         155 LOC  (ctx.host.crypto + AES-GCM)
├── env.rs            200 LOC  (ctx.host.env + env resolution)
├── sqlite.rs         125 LOC  (ctx.host.sqlite — exec gated to cursor)
├── ls.rs             411 LOC  (ctx.host.ls + ls_* helpers + tests)
├── http.rs           222 LOC  (ctx.host.http + TLS guard)
├── keychain.rs       501 LOC  (ctx.host.keychain + macOS 27 fix)
├── ccusage.rs        857 LOC  (ctx.host.ccusage + all runner logic)
└── utils.rs          289 LOC  (ctx.line/format/base64/jwt)
```

## Approved Decision Records

| EDR | Title | Status | Document |
|---|---|---|---|
| EDR-001 | PulseBar Product & Architecture Direction | Approved | [docs/edr/001-pulsebar-direction.md](./docs/edr/001-pulsebar-direction.md) |

## Approved Implementation Plans

| IMP | Title | Status | Document |
|---|---|---|---|
| IMP-001 | Implementation Master Plan | Approved (IRR amendments integrated) | [docs/imp/001-implementation-master-plan.md](./docs/imp/001-implementation-master-plan.md) |
| IMP-002 | Implementation Readiness Review | Approved | [docs/imp/002-irr-readiness-review.md](./docs/imp/002-irr-readiness-review.md) |
| IMP-003 | Provider Usage Architecture Roadmap | Approved baseline | [docs/imp/003-provider-usage-architecture-roadmap.md](./docs/imp/003-provider-usage-architecture-roadmap.md) |
| IMP-004 | Provider Usage Implementation Roadmap | Approved baseline | [docs/imp/004-provider-usage-implementation-roadmap.md](./docs/imp/004-provider-usage-implementation-roadmap.md) |
| IMP-005 | PulseUsage to PulseBar Migration Plan | Approved (planning only — no execution) | [docs/imp/005-pulsebar-migration-plan.md](./docs/imp/005-pulsebar-migration-plan.md) |

## Completed Milestones

| Milestone | Date | Release |
|---|---|---|
| Program 1 — host_api modularization (PRs #30–#45) | 2026-07-06 | On main (not released) |
| Maintenance baseline (PR #29) | 2026-07-05 | On main (not released) |
| PR-1 — Provider capability contracts | 2026-07-03 | Merged via PR #28 (not released) |
| v0.6.28 — Provider diagnostics, metadata hardening, keychain fix | 2026-07-02 | [v0.6.28](https://github.com/abyssbugg/PulseUsage/releases/tag/v0.6.28) |
| v0.6.27 — Pre-release-candidate state | 2026-06-06 | v0.6.27 |
| v0.6.26 — Repo production-readiness audit | 2026-06-05 | v0.6.26 |

## Roadmap Discipline

- **No feature work without an IMP.** Every multi-PR feature needs a plan.
- **No roadmap changes without an EDR.**
- **Mark roadmap items complete** via `docs: mark PR-N roadmap complete` commit.
- **Quarterly alignment audit** — verify all branches align with this roadmap.
- See [docs/governance/RoadmapManagement.md](./docs/governance/RoadmapManagement.md) for full process.
