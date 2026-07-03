# Project Status

> **Canonical project state document.** Future agents must read this first.
> Last updated: 2026-07-03 (commit `e749c72`)

## Current State

| Field | Value |
|---|---|
| **Phase** | Stabilization (pre-v0.7.0) |
| **Main HEAD** | `e749c72` (docs: establish repository governance framework) |
| **Origin/main** | In sync (0 ahead, 0 behind) |
| **Latest release** | v0.6.28 (2026-07-02) — published, DMG uploaded |
| **Commits since v0.6.28** | 12 (post-release: PRs #24, #25, #27, #28 + governance docs) |
| **Version (all 4 files)** | `0.6.28` — aligned |
| **Branches** | 2 local (`main`, `research/uiux-redesign` identical to main) |
| **Worktrees** | 1 (main only) |
| **Stashes** | 0 |
| **Remotes** | 1 (`origin` — `https://github.com/abyssbugg/PulseUsage.git`) |
| **Platform** | macOS 27.0 (Build 26A5368g) — development machine |

## Repository Health

| Dimension | Score | Status |
|---|---|---|
| Repository health | 9.5/10 | 🟢 Excellent |
| Branch health | 10/10 | 🟢 Excellent |
| Worktree health | 10/10 | 🟢 Excellent |
| Git history health | 9.5/10 | 🟢 Excellent |
| Release health | 9/10 | 🟢 Excellent |
| Documentation health | 9/10 | 🟢 Excellent (governance docs committed) |
| Governance maturity | 9/10 | 🟢 Excellent (8 governance docs + 6 control-center docs) |
| CI/CD health | 9/10 | 🟢 Excellent |
| Dependency health | 7/10 | 🟡 Good (5 green PRs, 1 age-gated, npm refresh needed) |
| Issue hygiene | 7/10 | 🟡 Good (1 open issue unlabeled) |
| **Overall** | **8.9/10** | 🟢 **Excellent** |

## Open Work Items

| Item | Type | Status | Priority |
|---|---|---|---|
| Issue #26 | Antigravity LS probe hardening | Open, unlabeled, unassigned | P2 — triage needed |
| PR #18 | `@types/node` 25.9.4→26.0.0 | CI failing (Bun age gate) | P2 — rebase needed |
| PR #19 | tauri 2.11.2→2.11.3 | ✅ CI green, MERGEABLE/CLEAN | P1 — merge first |
| PR #20 | tauri-build 2.6.2→2.6.3 | ✅ CI green, MERGEABLE/CLEAN | P1 — merge second |
| PR #21 | log 0.4.32→0.4.33 | ✅ CI green, MERGEABLE/CLEAN | P1 — merge third |
| PR #22 | time 0.3.47→0.3.51 | ✅ CI green, MERGEABLE/CLEAN | P1 — merge fourth |
| PR #23 | uuid 1.23.2→1.23.4 | ✅ CI green, MERGEABLE/CLEAN | P1 — merge fifth |

## Confirmed Issues (Not Yet Fixed)

| Issue | Severity | Status | Fix Target |
|---|---|---|---|
| macOS 27: `security add-generic-password` requires `-a account` | Medium | Confirmed (keychain write broken, read works) | v0.7.0 hardening PR |
| `dangerouslyIgnoreTls` no-localhost guard | Low | Confirmed (defense-in-depth) | v0.7.0 hardening PR |
| `inject_plist` no plugin_id allowlist | Low | Confirmed | v0.7.0 hardening PR |
| `inject_sqlite.exec` no plugin_id allowlist | Low | Confirmed | v0.7.0 hardening PR |
| 2 SVG icons missing `currentColor` | Low | Confirmed (AGENTS.md compliance) | v0.7.0 hardening PR |
| `deleteGenericPassword` not implemented in host | Low | Confirmed (copilot catches error) | v0.7.0 |
| `host_api.rs` 4,727-LOC monolith | Low | Acknowledged technical debt | v0.7.0 modularization |
| Perplexity `Agentic Research` unclassified | Low | Intentional (no evidence) | v0.7.0 (after evidence) |

## Frozen Work (Explicitly Forbidden This Phase)

- PulseBar rename (EDR-001 Approved but not executing — see [IMP-005](./docs/imp/005-pulsebar-migration-plan.md))
- UI/UX redesign (2 untracked planning docs exist — `docs/ux-audit-phase-0.md`, `docs/uiux-redesign-phase-0-spec.md` — left untracked, forbidden scope)
- Ollama integration (PR-2 — not started)
- OpenUsage / CodexBar imports (explicitly forbidden)
- New release creation (v0.7.0 not yet ready)

## Authoritative References

- [CURRENT_PHASE.md](./CURRENT_PHASE.md) — current milestone and immediate next steps
- [ROADMAP.md](./ROADMAP.md) — project roadmap and phase plan
- [RELEASE_PLAN.md](./RELEASE_PLAN.md) — release strategy and timeline
- [ACTIVE_BRANCHES.md](./ACTIVE_BRANCHES.md) — branch inventory
- [TECHNICAL_DEBT.md](./TECHNICAL_DEBT.md) — debt register
- [docs/governance/](./docs/governance/) — engineering standards (8 documents)
- [docs/edr/](./docs/edr/) — engineering decision records
- [docs/imp/](./docs/imp/) — implementation master plans
- [docs/release-readiness/](./docs/release-readiness/) — per-release readiness reports
