# Current Phase

> **Canonical current-phase document.** The exact milestone and immediate next steps.
> Last updated: 2026-07-06

## Current Milestone

**PulseBar v0.8.0 — Program A: Product Identity**

Core Platform Modernization is complete. v0.7.0-rc.1 has been published, `release/v0.7.0` is frozen except for verified RC fixes, and the PulseBar Governance Lock has merged. The project is now ready to begin Program A after this baseline update is approved.

## Active Program

| Field | Value |
|---|---|
| Product phase | PulseBar v0.8.0 |
| Active program | Program A |
| Program name | Product Identity |
| GitHub milestone | v0.8.0 |
| Required branch | `feat/pulsebar-identity-foundation` |
| Protected branches | `main`, `release/v0.7.0` |
| Release branch policy | `release/v0.7.0` accepts verified RC bug fixes only |

## What Is Happening Now

1. ✅ **v0.7.0-rc.1 published** — tag and GitHub prerelease exist with validated DMG artifact.
2. ✅ **Core Platform Modernization complete** — Program 1, Program 2, and Program 2.5 are merged.
3. ✅ **PulseBar Governance Lock merged** — PR #52 added the Program Charter, Product Decision Record, dependency diagram, PR sequencing, risk register, and Definitions of Done.
4. ✅ **v0.8.0 milestone created** — GitHub milestone #1 tracks the PulseBar product program.
5. 🔄 **v0.8.0 baseline update** — this docs-only update aligns the control-center docs with the new phase.
6. ⬜ **Program A PR-A1** — Product Identity foundation, to begin only after this baseline report is approved.

## Locked Decisions for Program A

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

## Immediate Next Steps

### Step 1 — Baseline approval
Review this baseline update and confirm the repository is aligned for PulseBar v0.8.0 Program A.

### Step 2 — Start Program A PR-A1
Create a separate worktree/branch:

```text
feat/pulsebar-identity-foundation
```

PR-A1 scope is identity foundation only. No UI redesign, no Ollama, no statistics engine, no bundle identifier change, no repository rename.

### Step 3 — Program A sequencing
Follow `docs/governance/pulsebar/PRSequencing.md`:

1. A1 Identity foundation
2. A2 User-facing string rename
3. A3 Artifact naming validation
4. A4 Docs and release notes update
5. A5 Screenshots and visual assets

## What Is NOT Happening

- ❌ No work on `release/v0.7.0` except verified RC bug fixes.
- ❌ No bundle identifier change.
- ❌ No Application Support directory migration.
- ❌ No GitHub repository rename.
- ❌ No schema v1 compatibility removal.
- ❌ No UI redesign rewrite.
- ❌ No Ollama implementation until its workstream is explicitly started.
- ❌ No statistics engine until its workstream is explicitly started.

## Authorization Model

- **Main Engineering Agent**: sole agent authorized to commit, push, merge, release, or modify the repository.
- **All other sessions**: research-only. May audit, research, plan, or benchmark. Must not commit, push, merge, rename, release, or modify.

## Completion Criteria for This Phase

This baseline phase is complete when:

- [x] PR #52 Governance Lock merged.
- [x] GitHub milestone `v0.8.0` exists.
- [x] `CURRENT_PHASE.md` reflects PulseBar v0.8.0 Program A.
- [x] `PROJECT_STATUS.md` reflects v0.7.0-rc.1 publication and Program A readiness.
- [x] Branch strategy verified.
- [x] Stale feature branches identified.
- [ ] Baseline report approved.

After approval, begin **Program A PR-A1: Product Identity foundation**.
