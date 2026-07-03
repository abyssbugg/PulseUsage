# Current Phase

> **Canonical current-phase document.** The exact milestone and immediate next steps.
> Last updated: 2026-07-03

## Current Milestone

**Repository Stabilization** (pre-v0.7.0)

The repository has transitioned from multi-agent implementation to centralized technical leadership. The Main Engineering Agent is stabilizing the repository before the next major development phase (v0.7.0).

## What Is Happening Now

1. ✅ **Repository Freeze** — verified no active modification, no unpublished work, no pending local changes (other than governance/control-center docs).
2. ✅ **Governance framework committed** — 8 governance documents under `docs/governance/` (commit `e749c72`).
3. ✅ **Project Control Center established** — 6 canonical operational documents (this set).
4. ✅ **IMP-005 (PulseBar migration plan) created** — planning only, no renaming.
5. ⬜ **Dependabot PR triage** — 5 green PRs ready to merge (#19-#23), 1 age-gated (#18).
6. ⬜ **Issue #26 triage** — needs labels and assignment.

## Immediate Next Steps (in order)

### Step 1 — Triage issue #26
Add labels (`hardening`, `antigravity`), assign owner. ~5 min.

### Step 2 — Rebase PR #18
`@dependabot recreate` or `gh pr update-branch 18` to re-trigger CI with aged `@types/node`. ~5 min.

### Step 3 — Merge 5 green dependabot PRs
In risk order: #19 (tauri) → #20 (tauri-build) → #21 (log) → #22 (time) → #23 (uuid). Run `bun run test` + `cargo test` after each. ~50 min.

### Step 4 — Merge PR #18 (after rebase succeeds)
~10 min.

### Step 5 — Security hardening + macOS 27 fix PR
Bundle the 8 hardening items (see [ROADMAP.md](./ROADMAP.md) Phase B). ~3 hours.

### Step 6 — Begin v0.7.0 architecture work
npm deps refresh → `host_api.rs` modularization → capability manifest enforcement. ~1-2 weeks.

## What Is NOT Happening (Forbidden This Phase)

- ❌ PulseBar rename (EDR-001 Approved, IMP-005 planned, but execution forbidden)
- ❌ UI/UX redesign (2 untracked planning docs exist — left untracked, forbidden scope)
- ❌ Ollama integration (PR-2 — not started)
- ❌ OpenUsage / CodexBar imports
- ❌ New release creation (v0.7.0 not ready)
- ❌ Feature development (stabilization only)

## Authorization Model

- **Main Engineering Agent** (this session): sole agent authorized to commit, push, merge, release, or modify the repository.
- **All other sessions**: research-only. May audit, research, plan, benchmark. Must NOT commit, push, merge, rename, release, or modify.

## Completion Criteria for This Phase

The stabilization phase is complete when:
- [x] Governance docs committed
- [x] Project Control Center established
- [x] IMP-005 (PulseBar migration plan) created
- [ ] Issue #26 triaged
- [ ] PR #18 rebased
- [ ] 5 green dependabot PRs merged
- [ ] PR #18 merged (if age gate passes)
- [ ] Security hardening + macOS 27 fix PR created (or deferred to v0.7.0 with documented justification)

Once complete, the repository transitions to **Phase C — v0.7.0 Architecture Evolution**.
