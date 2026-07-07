# Current Phase

> **Canonical current-phase document.** The exact milestone and immediate next steps.
> Last updated: 2026-07-07

## Current Milestone

**PulseBar v0.8.0 — Programs A and B in flight**

Core Platform Modernization is complete. v0.7.0-rc.1 is published and `release/v0.7.0` is frozen except for verified RC fixes. Program A (Product Identity) has merged its first two PRs; Program B (Professional UI/UX) has merged its design-system foundation. The Engineering Governance & Audit Framework is in review.

## Active Programs

| Field | Value |
|---|---|
| Product phase | PulseBar v0.8.0 |
| GitHub milestone | v0.8.0 |
| Program A — Product Identity | In progress: A1 ✅ (PR #54), A2 ✅ (PR #55); A3–A5 remaining |
| Program B — Professional UI/UX | In progress: design-system foundation ✅ (PR #56, ADR-002); B1 next |
| Branch strategy | One workstream per branch/worktree, per [PRSequencing](docs/governance/pulsebar/PRSequencing.md) |
| Protected branches | `main`, `release/v0.7.0` |
| Release branch policy | `release/v0.7.0` accepts verified RC bug fixes only |

## What Is Happening Now

1. ✅ **Program A identity foundation merged** — PR #54 added the `identity.rs` / `app-identity.ts` identity layers (A1).
2. ✅ **Program A identity rollout merged** — PR #55 moved user-visible surfaces to the PulseBar display identity (A2).
3. ✅ **Program B design-system foundation merged** — PR #56 established design tokens, [ADR-002](docs/adr/002-design-system.md), and the component contract.
4. 🔄 **Governance & Audit Framework in review** — PR #57 adds the invariant registry, audit process, and five audit charters under `docs/governance/`.
5. ⬜ **Program B PR-B1** — accessibility baseline (focus visibility, keyboard paths) per PRSequencing.
6. ⬜ **Program A A3–A5** — artifact naming validation, docs/release notes, screenshots and visual assets.

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

## Immediate Next Steps

### Step 1 — Merge the governance framework
Review and merge PR #57. Governance docs follow the same review rigor as code (see `docs/governance/audits/AuditProcess.md` hard rules once merged).

### Step 2 — Program B PR-B1
Create a dedicated branch for the accessibility baseline. Scope per [PRSequencing](docs/governance/pulsebar/PRSequencing.md): restore focus visibility and keyboard paths. Program B Definition of Done applies (screenshots, keyboard/focus verification, state checks).

### Step 3 — First audit pilot
After the next significant Program B milestone, run the first Security Charter (SEC) audit against the plugin engine as a pilot, and use its run record to validate the framework itself before scheduling broader audits.

### Step 4 — Program A completion
A3 (artifact naming validation), A4 (docs and release notes), A5 (screenshots and visual assets), each as its own PR.

## What Is NOT Happening

- ❌ No work on `release/v0.7.0` except verified RC bug fixes.
- ❌ No bundle identifier change.
- ❌ No Application Support directory migration.
- ❌ No GitHub repository rename.
- ❌ No schema v1 compatibility removal.
- ❌ No UI redesign rewrite.
- ❌ No Ollama implementation until its workstream is explicitly started.
- ❌ No statistics engine until its workstream is explicitly started.
