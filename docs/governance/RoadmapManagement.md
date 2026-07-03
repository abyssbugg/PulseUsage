# Roadmap Management

## Roadmap Structure

PulseUsage's roadmap is structured as:

1. **EDR (Engineering Decision Records)** — `docs/edr/` — what and why (decisions)
2. **IMP (Implementation Master Plans)** — `docs/imp/` — how and when (implementation)
3. **IRR (Implementation Readiness Reviews)** — within `docs/imp/` — readiness verification
4. **Release Readiness Reports** — `docs/release-readiness/` — per-release state
5. **Roadmap completion markers** — commits like `docs: mark PR-1 roadmap complete` on main

## Roadmap Phases

### Current phase: v0.6.28 released → v0.7.0 development

The v0.6.28 release is complete. The repository is in a clean, governed state. The next phase is v0.7.0 development (not yet started).

### v0.7.0 (planned, not started)

From the approved engineering hardening roadmap:

1. **Dependabot backlog** — merge 6 pending PRs (tauri, tauri-build, log, time, uuid, types/node)
2. **Security hardening PR** — bundle:
   - `dangerouslyIgnoreTls` localhost-only guard
   - `inject_plist` plugin_id allowlist
   - `inject_sqlite.exec` plugin_id allowlist
   - 2 SVG `currentColor` fixes (AGENTS.md compliance)
   - `tauri-nspanel` fork + pin by rev
   - `cargo clippy --fix`
   - `cargo-audit` CI step
3. **`host_api.rs` modularization** — split 4,727-LOC monolith into per-capability modules
4. **Plugin capability manifest** — `"capabilities": [...]` in `plugin.json` (schema v2)
5. **Perplexity `Agentic Research` classification** — gather evidence, enable strict-mode CI
6. **npm deps refresh** — batch `@base-ui/react`, `lucide-react`, tailwind updates

### v1.0.0 (conditional — only if triggered)

- **Notarization** — if macOS 27 enforces or user base grows >10
- **App Sandbox** — if macOS 27 enforces
- **Process-exec native crates** — if energy impact becomes a complaint or plugin count >25
- **Strict-mode provider validation** — after Perplexity evidence gathered
- **macOS 27 compatibility** — `writeGenericPassword -a account` fix

**Do NOT implement v1.0 work until triggered.** AGENTS.md: "Simplicity first: handle only important cases."

## Roadmap Alignment Verification

### Quarterly audit

Compare every active branch against the approved roadmap:

| Branch type | Roadmap classification | Alignment check |
|---|---|---|
| `main` | Current production trunk | ✅ Always aligned |
| `feat/*` | Current roadmap (v0.7 features) | Verify the feature is in the IMP |
| `fix/*` | Current roadmap (stabilization) | Verify the fix is justified by an issue or audit |
| `chore/*` | Current roadmap (maintenance) | Verify the chore is necessary |
| `research/*` | Future roadmap (planning) | Verify an EDR or IMP references the research |
| `hardening/*` | Current roadmap (hardening) | Verify the hardening is in the security roadmap |
| `dependabot/*` | Ongoing maintenance | Always aligned (automated) |

### Drift detection

A branch has **drifted** if:
- It implements a feature not in any IMP or EDR
- It has been open for >2 weeks without progress
- It has conflicts with main that require manual resolution
- It duplicates work already merged

**Drifted branches** must be either:
1. Aligned (add an IMP/EDR referencing it)
2. Closed (with a justification comment)
3. Rebased (if the conflict is the only issue)

## Roadmap Documents

### EDR-001: PulseBar architecture direction

**Status:** Proposed (not yet approved for implementation)
**Scope:** Project rename to PulseBar, architecture evolution

**Constraint:** The user has explicitly stated "Do not begin the PulseBar rename" and "Do not start PulseBar development." EDR-001 is research only. No implementation.

### IMP-001: Implementation master plan

**Status:** Reference document
**Scope:** Overall implementation approach

### IMP-002: IRR readiness review

**Status:** Complete (v0.6.28 readiness verified)
**Scope:** v0.6.28 release readiness

### IMP-003: Provider usage architecture roadmap

**Status:** Proposed
**Scope:** Provider usage architecture evolution

### IMP-004: Provider usage implementation roadmap

**Status:** Proposed
**Scope:** Provider usage implementation plan

**Constraint:** The user has explicitly stated "Do not create a new roadmap." These IMPs exist as planning artifacts; no new roadmap creation.

## Roadmap Completion Tracking

When a roadmap item (PR-N) completes:

```bash
# Create a docs commit marking completion
git checkout main
git pull --ff-only
git commit -m "docs: mark PR-N roadmap complete"
git push origin main
```

Example from main history:
- `c37237b docs: mark PR-1 roadmap complete` (PR-1 = provider capability contracts)

This creates a visible marker in the commit history that the roadmap item is done.

## Roadmap Discipline

### What NOT to do

- **Do not implement features without an IMP.** Every multi-PR feature needs a plan.
- **Do not create a new roadmap without EDR approval.** Roadmap changes are architectural decisions.
- **Do not begin the PulseBar rename.** Explicitly forbidden by the user.
- **Do not start Ollama (PR-2).** Explicitly forbidden by the user.
- **Do not import functionality from OpenUsage or CodexBar.** Explicitly forbidden by the user.
- **Do not begin the UI/UX redesign.** Explicitly forbidden by the user.

### What TO do

- **Document all significant work in EDR/IMP.**
- **Mark roadmap items complete when done.**
- **Verify alignment quarterly.**
- **Close abandoned roadmap items** with a justification.
- **Keep the roadmap honest** — if a planned item is no longer relevant, deprecate it (don't leave it as "planned" forever).

## Roadmap Review Cadence

| Cadence | Action |
|---|---|
| Weekly | Triage issues and PRs; verify no drift |
| Monthly | Review open roadmap items; close stale ones |
| Quarterly | Full roadmap alignment audit (all branches vs roadmap) |
| Per-release | Verify release readiness report is accurate; mark completed items |

## Roadmap Health Metrics

| Metric | Target | Measurement |
|---|---|---|
| Drifted branches | 0 | Quarterly audit |
| Stale roadmap items (open >3 months without progress) | 0 | Monthly review |
| Completed items without completion marker | 0 | `git log --grep="mark PR-.* roadmap complete"` |
| EDRs without IMPs (decisions without plans) | 0 | Quarterly |
| IMPs without IRRs (plans without readiness review) | 0 | Before implementation starts |