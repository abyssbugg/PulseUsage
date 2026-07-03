# Roadmap

> **Canonical project roadmap.** Future agents must verify work aligns with this document.
> Last updated: 2026-07-03

## Current Milestone

**v0.7.0 — Stabilization & Architecture Hardening** (in planning, not started)

## Phase Plan

### Phase A — Immediate Stabilization (current, ~2 hours)

| Task | Status | Effort |
|---|---|---|
| Commit governance documentation | ✅ Complete (commit `e749c72`) | 10 min |
| Establish Project Control Center | ✅ Complete (this document set) | 30 min |
| Create IMP-005 (PulseBar migration plan) | ✅ Complete | 1 hour |
| Triage issue #26 (add labels, assign) | ⬜ Pending | 5 min |
| Rebase PR #18 (`@types/node` age gate) | ⬜ Pending | 5 min |
| Merge 5 green dependabot PRs (#19→#20→#21→#22→#23) | ⬜ Pending | 50 min |

### Phase B — Security Hardening + macOS 27 Compatibility (~3 hours)

Bundle into one hardening PR:
- `dangerouslyIgnoreTls` localhost-only guard
- `inject_plist` plugin_id allowlist
- `inject_sqlite.exec` plugin_id allowlist
- 2 SVG `currentColor` fixes (antigravity, copilot)
- `tauri-nspanel` fork + pin by rev (reproducibility)
- `cargo clippy --fix` (21 warnings, 15 auto-fixable)
- `cargo-audit` CI step
- **macOS 27: `keychain_add_generic_password_args` add `-a account`**

### Phase C — v0.7.0 Architecture Evolution (~1-2 weeks)

- npm deps refresh (batch `@base-ui/react` 1.1→1.6, `lucide-react` 1.7→1.21, tailwind, react, zustand)
- `host_api.rs` modularization (split 4,727-LOC monolith into per-capability modules)
- Plugin capability manifest enforcement (schema v2 with `"capabilities": [...]`)
- `deleteGenericPassword` host implementation
- Perplexity `Agentic Research` classification (evidence-gathering, then strict-mode CI)

### Phase D — v0.7.0 Release

- Version bump to `0.7.0` (4 files + CHANGELOG + release-readiness doc)
- Tag `v0.7.0` + build DMG + publish release
- See [RELEASE_PLAN.md](./RELEASE_PLAN.md)

### Phase E — PulseBar Migration (post-v0.7.0, when approved)

- Execute [IMP-005](./docs/imp/005-pulsebar-migration-plan.md) (PulseUsage → PulseBar)
- Release as v0.8.0 (or v1.0.0 if combined with notarization)
- **Not started** — forbidden until v0.7.0 stabilizes

### Phase F — v1.0 (conditional — only if triggered)

- Process-exec native crates (`plist`, `rusqlite`, `security-framework`) — if energy complaints or plugin count >25
- Notarization — if macOS 27 enforces or user base >10
- App Sandbox — if macOS 27 enforces
- **Do NOT implement until triggered** (AGENTS.md: simplicity first)

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
| v0.6.28 — Provider diagnostics, metadata hardening, keychain fix | 2026-07-02 | [v0.6.28](https://github.com/abyssbugg/PulseUsage/releases/tag/v0.6.28) |
| v0.6.27 — Pre-release-candidate state | 2026-06-06 | v0.6.27 |
| v0.6.26 — Repo production-readiness audit | 2026-06-05 | v0.6.26 |
| PR-1 — Provider capability contracts | 2026-07-03 | Merged via PR #28 (not released — on main) |

## Roadmap Discipline

- **No feature work without an IMP.** Every multi-PR feature needs a plan.
- **No roadmap changes without an EDR.**
- **Mark roadmap items complete** via `docs: mark PR-N roadmap complete` commit.
- **Quarterly alignment audit** — verify all branches align with this roadmap.
- See [docs/governance/RoadmapManagement.md](./docs/governance/RoadmapManagement.md) for full process.
