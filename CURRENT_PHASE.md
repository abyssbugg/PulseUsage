# Current Phase

> **Canonical current-phase document.** The exact milestone and immediate next steps.
> Last updated: 2026-07-17

## Current Milestone

**PulseBar v0.8.0 — RC Audit in progress**

Program B is complete (B1-B5 merged). Program S (Security Hardening) merged. Program A (Product Identity) complete. Architecture frozen. RC Audit underway — no new feature work until release candidate is cut.

## Active Programs

| Field | Value |
|---|---|
| Product phase | PulseBar v0.8.0 |
| GitHub milestones | v0.8.0 (#1, Program B), v0.8.1 Hardening (#2, CSP POC), v0.9 Platform (#3, Plugin SDK) |
| Program A — Product Identity | ✅ Complete (PRs #54, #55) |
| Program B — Professional UI/UX | ✅ Complete: B1 ✅, B2 ✅ (#61), B3 ✅ (#75), B4 ✅ (#76), B5 ✅ (#77) |
| Program S — Security Hardening | ✅ Complete (#69-#72 merged, #73-#74 dep bumps merged) |
| Branch strategy | One workstream per branch/worktree, per [PRSequencing](docs/governance/pulsebar/PRSequencing.md) |
| Protected branches | `main`, `release/v0.7.0` |
| Release branch policy | `release/v0.7.0` accepts verified RC bug fixes only |

## What Is Happening Now

1. ✅ **Program A complete** — Product identity foundation and rollout merged (PRs #54, #55).
2. ✅ **Program B B1 complete** — Design system foundation + token freeze merged (PRs #56, #59, ADR-002).
3. ✅ **Program B B2 complete** — Component migration merged (PR #61, 38/40 constants migrated).
4. ✅ **Program S complete** — Security triage, ADRs, CORS/mutex/log-level fixes, plugin isolation audit all merged (PRs #69-#72).
5. ✅ **Repository Recovery complete** — 0 open PRs, 4 open issues all in milestones, 12 stale branches deleted, duplicate files removed.
6. 🔄 **Program B B3** — Layout Refinement is the immediate next step.

## Open Issues (Roadmap Items, Not Blockers)

| Issue | Milestone | Status |
|---|---|---|
| #65 SEC-004 CSP POC | v0.8.1 Hardening | Deferred — needs Tauri WebView POC |
| #66 SEC-005 Keychain allowlist | v0.9 Platform | Deferred — third-party plugin era |
| #67 SEC-006 Filesystem confinement | v0.9 Platform | Deferred — third-party plugin era |
| #26 Antigravity LS hardening | v0.9 Platform | Enhancement, P3 |

None of these block Program B or v0.8.0.

## Locked Decisions

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
| Architecture freeze | No new architectural initiatives until Program B complete |

## Immediate Next Steps

### Step 1 — Program B B3 (Layout Refinement)
Resume Program B at B3. Scope: layout refinement using existing design tokens. No new tokens, no architectural changes. Screenshots required per Definition of Done.

### Step 2 — B4 (Visual Polish)
After B3 merges. Polish pass: spacing, alignment, micro-interactions. Token-only.

### Step 3 — B5 (Accessibility)
After B4 merges. Keyboard navigation, focus visibility, ARIA, contrast verification.

### Step 4 — Release Candidate
After B5. Full production verification, DMG packaging, release notes.

## What Is NOT Happening

- ❌ No new architectural initiatives until Program B is complete.
- ❌ No CSP implementation until v0.8.1 POC validates it.
- ❌ No keychain/filesystem allowlists until v0.9 Plugin SDK.
- ❌ No work on `release/v0.7.0` except verified RC bug fixes.
- ❌ No bundle identifier change.
- ❌ No GitHub repository rename.
- ❌ No schema v1 compatibility removal.
- ❌ No UI redesign rewrite.
- ❌ No Ollama implementation until its workstream is explicitly started.
- ❌ No statistics engine until its workstream is explicitly started.

## Authorization Model

- **Main Engineering Agent**: sole agent authorized to commit, push, merge, release, or modify the repository.
- **All other sessions**: research-only. May audit, research, plan, or benchmark. Must not commit, push, merge, rename, release, or modify.
