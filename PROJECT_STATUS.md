# Project Status

> **Canonical project state document.** Future agents must read this first.
> Last updated: 2026-07-06 (PulseBar v0.8.0 baseline)

## Current State

| Field | Value |
|---|---|
| **Phase** | PulseBar v0.8.0 — Program A: Product Identity |
| **Main HEAD** | `8f02d0d` (Merge PR #52 — PulseBar Governance Lock) |
| **Origin/main** | In sync after PR #52 merge |
| **Latest published prerelease** | `v0.7.0-rc.1` — published with validated DMG |
| **Latest stable release** | `v0.6.28` |
| **Release branch** | `release/v0.7.0` — frozen except verified RC bug fixes |
| **Active milestone** | GitHub milestone `v0.8.0` (#1) |
| **Next active branch** | `feat/pulsebar-identity-foundation` |
| **Protected branches** | `main`, `release/v0.7.0` |
| **Platform** | macOS 27.0 (Build 26A5368g) — development machine |

## Core Platform Modernization Status

| Program | Status | Evidence |
|---|---|---|
| Program 1 — host_api modularization | ✅ Complete | PRs #30–#45 merged; 13 modules under `host_api/` |
| Program 2 — Capability enforcement | ✅ Complete | PRs #47–#50 merged; all 18 bundled providers use schema v2 `hostCapabilities` |
| Program 2.5 — Compatibility hardening | ✅ Complete | PR #51 merged; v1 compatibility documented as platform contract |
| Stabilization — Documentation sync | ✅ Complete | PR #46 merged |
| Governance Lock — PulseBar v0.8.0 | ✅ Complete | PR #52 merged |

## Release State

| Release | Status | Notes |
|---|---|---|
| v0.7.0-rc.1 | ✅ Published | Tag/release published from `release/v0.7.0` at `e6b34c6`; DMG checksum verified |
| v0.7.0 final | ⬜ Pending | Release branch frozen for RC validation/fixes only |
| v0.8.0 | 🔄 Planning/Program A | PulseBar product program begins after this baseline approval |

## Repository Health

| Dimension | Status | Notes |
|---|---|---|
| Main branch | ✅ Healthy | PR #52 merged, docs-only governance lock in main |
| Release branch | ✅ Protected | `release/v0.7.0` exists and is frozen |
| Worktree health | ✅ Healthy | One primary worktree; create separate worktree for Program A |
| Plugin compatibility | ✅ Healthy | schema v1 compatibility retained; schema v2 current |
| Provider manifests | ✅ Healthy | 18 bundled providers migrated to explicit `hostCapabilities` |
| Documentation | ✅ Strong | PulseBar governance docs added; control-center docs updated by this baseline |
| Release readiness | ✅ RC published | v0.7.0-rc.1 smoke-tested and published |

## Active Work Items

| Item | Type | Status | Priority |
|---|---|---|---|
| PulseBar v0.8.0 baseline update | Docs | 🔄 In progress | P1 |
| Program A PR-A1 Product Identity foundation | Feature program | ⬜ Pending baseline approval | P1 |
| v0.7.0 final release validation | Release | ⬜ Pending RC soak/approval | P1 |
| Issue #26 Antigravity LS probe hardening | Enhancement | ⬜ Open | P3 |
| PR #35 aes-gcm 0.11 upgrade | Dependency | ⬜ Open/failing CI | P3 |

## Branch Strategy

| Branch | Role | Policy |
|---|---|---|
| `main` | Stable development base | Protected; merge via PR only |
| `release/v0.7.0` | v0.7.0 RC/final branch | Frozen except verified RC bug fixes |
| `feat/pulsebar-identity-foundation` | Next Program A branch | Create from `main`; product identity foundation only |

## Stale Branch Risk

Several merged topic branches remain locally/remotely from prior programs. They must not receive new PulseBar work. Program A must start from fresh `main` using `feat/pulsebar-identity-foundation`.

Branches to avoid for Program A:

- `feat/program2-*`
- `docs/pulsebar-governance-lock`
- `docs/sync-program-1-completion`
- `refactor/extract-*`
- `chore/deps-*`
- `chore/v0.7.0-maintenance-baseline`
- `research/uiux-redesign`

## PulseBar Governance Decisions

| Decision | Locked Position |
|---|---|
| Product display name | PulseBar |
| Bundle identifier | Keep `com.abyssbugg.pulseusage` for v0.8.x |
| Application Support directory | Remain unchanged |
| Plugin compatibility | Maintain schema v1 compatibility |
| GitHub repository | Remain PulseUsage for v0.8.0 |
| Statistics policy | Evidence-backed only |
| Ollama policy | No fake quota, billing, or usage metrics |
| UI strategy | Incremental refinement, not rewrite |
| Branch strategy | One workstream per branch/worktree |

## Next Step

After this baseline report is approved, create a new worktree and branch from `main`:

```text
feat/pulsebar-identity-foundation
```

Then begin **Program A PR-A1: Product Identity foundation**. No production rename implementation may happen before that branch/worktree is created and scoped.
