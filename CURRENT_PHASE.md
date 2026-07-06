# Current Phase

> **Canonical current-phase document.** The exact milestone and immediate next steps.
> Last updated: 2026-07-06

## Current Milestone

**Stabilization Milestone** (pre-v0.7.0 release)

Program 1 (host_api modularization) is complete. The repository is in a stabilization phase preparing for the v0.7.0 release. Capability enforcement (Program 2) design is approved but not yet implemented.

## What Is Happening Now

1. ✅ **Program 1 complete** — 16 PRs (#30–#45) merged. `host_api.rs` 4,816-LOC monolith decomposed into 13 cohesive modules under `host_api/`.
2. ✅ **Maintenance baseline merged** — PR #29 resolved all P2 security hardening + macOS 27 compatibility items.
3. ✅ **Dependency refresh complete** — PR #30 (safe bumps) + PR #31 (major bumps) merged.
4. ✅ **Program 2 design approved** — Capability enforcement blueprint produced in Program Transition document. 6 PRs estimated, ~9 hours.
5. 🔄 **Documentation synchronization** — This PR. Updating stale docs to reflect Program 1 completion.
6. ⬜ **Repository health audit** — Audit branches, tags, releases for cleanup.
7. ⬜ **Release readiness audit** — Verify v0.7.0 technical readiness.
8. ⬜ **Program 2 readiness review** — Final go/no-go decision.

## Immediate Next Steps (in order)

### Step 1 — Documentation synchronization (this PR)
Update all stale docs that reference `host_api.rs` monolith. ~30 min.

### Step 2 — Repository health audit
Audit branches, tags, releases. Produce cleanup plan (no auto-deletion). ~15 min.

### Step 3 — Release readiness audit
Verify version, changelog, build, CI, packaging for v0.7.0. ~20 min.

### Step 4 — Program 2 readiness review
Re-evaluate whether Program 2 should begin immediately or if another prerequisite emerged. ~10 min.

### Step 5 — Decision point
Based on Task 4 recommendation, either:
- Begin Program 2 (capability enforcement, 6 PRs), OR
- Address any prerequisite that emerged from the audit

### Step 6 — v0.7.0 release
After Program 2 is complete:
- Version bump to `0.7.0` (4 files + CHANGELOG + release-readiness doc)
- Tag `v0.7.0` + build DMG + publish release
- See [RELEASE_PLAN.md](./RELEASE_PLAN.md)

## What Is NOT Happening (Forbidden This Phase)

- ❌ PulseBar rename (EDR-001 Approved, IMP-005 planned, but execution forbidden until v0.7.0 stabilizes)
- ❌ UI/UX redesign (2 untracked planning docs exist — left untracked, forbidden scope)
- ❌ Ollama integration (not sequenced — can be added anytime after v0.7.0)
- ❌ OpenUsage / CodexBar imports
- ❌ New release creation (v0.7.0 not ready — Program 2 pending)
- ❌ Capability enforcement code (design approved, implementation not started)

## Authorization Model

- **Main Engineering Agent** (this session): sole agent authorized to commit, push, merge, release, or modify the repository.
- **All other sessions**: research-only. May audit, research, plan, benchmark. Must NOT commit, push, merge, rename, release, or modify.

## Completion Criteria for This Phase

The stabilization phase is complete when:
- [x] Program 1 (host_api modularization) complete
- [x] Maintenance baseline (PR #29) merged
- [x] Dependency refresh (PR #30, #31) merged
- [x] Program 2 design approved
- [ ] Documentation synchronized (this PR)
- [ ] Repository health audit complete
- [ ] Release readiness audit complete
- [ ] Program 2 readiness review complete
- [ ] Program 2 decision made (begin or defer)

Once complete, the repository transitions to **Program 2** (if approved) or **v0.7.0 release** (if Program 2 is deferred).
