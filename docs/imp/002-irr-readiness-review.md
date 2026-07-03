# IRR-001: Implementation Readiness Review

| Field | Value |
|-------|-------|
| **Status** | Approved |
| **Baseline** | IMP-001 + EDR-001 |
| **Review date** | 2026-07-02 |
| **Recommendation** | Ready with recommended adjustments |

## Summary

IMP-001 is executable against the repository at `origin/main` @ `4f083ee` (v0.6.27). Three gaps required amendments before Phase 1+:

1. **Repository hygiene** — stale `main` with 79 staged files (Phase 0)
2. **Operational prerequisites** — no updater dependency, no signing pipeline (Phase 4 amendments)
3. **SWR scope** — disk cache serves HTTP API only, not UI launch hydration (Phase 4 amendment)

## Repository Findings (pre-Phase 0)

| Check | Result |
|-------|--------|
| Vitest | 1089 tests pass |
| Cargo test | Requires `bun run bundle:plugins` in fresh worktree |
| `host_api.rs` | 4,617 LOC (split undersized in original IMP) |
| Settings | Dual path: `LazyStore` + raw disk read in `cache.rs` + `tray.rs` |
| Updater | Not in `Cargo.toml`; `use-app-update.ts` is stub |
| `publish.yml` | Disabled; no signing/notarization |
| Planning docs | Absent until Phase 0 |

## Amendments (incorporated into IMP-001)

| ID | Amendment | Class |
|----|-----------|-------|
| A-001 | Apple Developer Program enrollment before P4 calendar | Critical |
| A-002 | PR-025a: `tauri-plugin-updater` before PR-030 | Critical |
| A-003 | PR-025b: signing/notarization in `publish.yml` | Critical |
| A-004 | PR-028: add UI cache hydration IPC (`get_cached_snapshots`) | High |
| A-005 | Rename: 5–7 PRs not single PR-007 (~80 files) | High |
| A-006 | PR-001: rescope Antigravity (retrieveUserQuota already exists) | Medium |
| A-007 | Prune merged worktrees in P0 | Medium |
| A-008 | Add `bun run test:coverage` to CI at Gate G3 | Medium |
| A-009 | Document `bundle:plugins` in dev onboarding | Low |

## Phase Readiness

| Phase | Verdict |
|-------|---------|
| P0 | Executable as planned |
| P1 | Executable after P0; rescope PR-001 |
| P2 | Executable; expand rename PR count |
| P3 | Executable; 6 host split PRs, settings touches 4 files |
| P4 | Not ready without A-001–A-004 |
| P5–P6 | Executable post-G4 |

## Staged State Resolution (Phase 0)

79 staged files on stale `main` @ `c4c73ba`. Comparison to `origin/main`:

- 66 files: identical to merged upstream content
- 13 files: staged regressions (deleted tests/docs vs `origin/main`)

**Action taken:** `git reset --hard origin/main` — no legitimate work lost.

## Worktree Pruning (Phase 0)

Removed (merged into `origin/main`):

- `audit-provider-stability`, `brand-pulseusage-icon-release`, `bump-version-0.6.27`
- `restore-factory-rich-metrics`, `restore-warp-provider`
- `hardening-diagnostics-runtime-ui`, `hardening-provider-metadata-docs`, `hardening-validation-release`

Preserved:

- `research-pulsebar-architecture` (planning)
- `hardening-provider-diagnostics-v0628` (open PR #14)

## References

- [EDR-001](../edr/001-pulsebar-direction.md)
- [IMP-001](001-implementation-master-plan.md)