# Audit Process

> **Canonical audit process.** Defines how independent engineering audits run against this repository,
> when they trigger, and where findings land. Audits are quality gates that complement — never replace —
> the PR-level review flow in [CodeReview.md](../CodeReview.md).
> Last updated: 2026-07-07

## Principles

1. **Audits never implement.** An audit run produces findings and recommendations only. Fixes happen in normal implementation PRs under [DefinitionOfDone](../pulsebar/DefinitionOfDone.md).
2. **One source of truth.** Findings land in the existing canonical registers (below). Audits never create parallel registers.
3. **Independence through model diversity, not personas.** Two independent reviewers with separate contexts (e.g. one Anthropic deep-reasoning pass, one external-peer pass via Codex, both read-only) receive the identical charter and never see each other's output. The orchestrator synthesizes; divergence between reviewers is investigated first — it usually marks the real risk.
4. **Findings are claims until refuted-tested.** Every raw finding goes through a refutation pass — an independent reviewer prompted to disprove it against the actual code — before it may enter a register. Findings that do not survive are discarded, not downgraded.
5. **Invariants first.** Every run starts by verifying the [Invariant Registry](../Invariants.md), then dedups against the registers, and only then searches for new issues.
6. **Evidence or it didn't happen.** Every finding carries: affected files (path:line), root cause, impact, severity, recommendation, estimated fix effort, estimated regression risk.

## Naming

Audits are named by **charter**, not "Program" — the Program namespace (A–F, 1–2.5) is reserved for implementation workstreams in the [Program Charter](../pulsebar/ProgramCharter.md). A run is identified as `<Charter>-<date>`, e.g. `SEC-2026-07-07`.

## Charters and triggers

| Charter | File | Trigger | Scope |
|---|---|---|---|
| SEC — Security | [SecurityCharter.md](SecurityCharter.md) | Any change under `src-tauri/src/plugin_engine/` or `local_http_api/`; new provider auth source; dependency alerts | Plugin isolation, capability enforcement, keychain, redaction, Tauri IPC, supply chain |
| ARC — Architecture | [ArchitectureCharter.md](ArchitectureCharter.md) | Completion of an implementation program phase; new ADR proposed | Module boundaries, coupling, ADR compliance, invariants INV-004/007/008 |
| PERF — Performance | [PerformanceCharter.md](PerformanceCharter.md) | On demand; before an RC when startup/memory-affecting work merged | Startup, memory, rendering, IPC, async, subprocess spawning |
| REL — Release Readiness | [ReleaseReadinessCharter.md](ReleaseReadinessCharter.md) | Before every RC tag | Packaging, versioning, signing, artifact naming, rollback, `docs/release-readiness/` doc |
| UX — UI/UX | [UXCharter.md](UXCharter.md) | Completion of each Program B phase; any PR touching `src/components/` at phase boundary | Design-token compliance (INV-005), accessibility, states, macOS conventions |

Nothing here runs "continuously." An audit outside these triggers is on-demand and must state why.

This table is the canonical audit index. Do not create a separate index document; a second list of charters and triggers would drift from this one.

## Run lifecycle

```
Trigger fires
  → Orchestrator scopes the run (diff / blast radius / full surface per charter)
  → Invariant verification (Invariants.md checks for this charter's scope)
  → Blind parallel review: Reviewer A + Reviewer B, identical charter prompt, read-only, no cross-visibility
  → Synthesis: orchestrator merges, investigates divergence
  → Refutation: each surviving finding sent to an independent refuter with repo access
  → Registration (below)
  → Close: run record written to audits/history/<CHARTER>-<YYYY-MM-DD>.md
```

A run record contains: trigger, scope, invariant-check results, findings (including refuted ones, marked as such), register links, and — for PERF — the measured baseline numbers. The `history/` directory is the only home for run records; charters and registers never accumulate them.

Reviewer count is fixed at two plus one refuter. More layers add cost, not signal (anti-ceremony rule).

## Where findings land (registration)

| Finding type | Destination | Format |
|---|---|---|
| Invariant violation | GitHub issue, label `invariant-violation`, severity per Invariants.md escalation column | Issue links the invariant ID |
| Critical / release-blocking risk | [RiskRegister.md](../pulsebar/RiskRegister.md) row + GitHub issue | Register's existing Critical/High/Medium/Low taxonomy |
| Actionable debt | [TECHNICAL_DEBT.md](../../../TECHNICAL_DEBT.md) row | Existing P1–P5 taxonomy — do not introduce a second severity scale |
| Documentation drift | PR fixing the doc directly (docs-only) | Small, immediate |
| Non-actionable observation | Run record in `history/` only | No register entry — registers hold decisions, not musings |

## Dedup rule

Before registering, check `TECHNICAL_DEBT.md` (including its Resolved and P5-intentional sections), `RiskRegister.md` (including Risk Acceptance), and open `invariant-violation` issues. A finding matching an accepted risk or intentional-debt entry is **not re-reported**; if the auditor believes the acceptance is no longer valid, the finding is "acceptance challenge" and goes to the human, not the register.

## Hard rules

- Reviewers run read-only. No audit run ever gets write access to source.
- Audits never approve. They find or they pass; approval is the merge process's job.
- No busywork: a refactoring recommendation without measurable benefit is discarded at synthesis.
- An audit that finds nothing writes a one-line run record saying so. Empty results are results.

## Framework health

Governance must earn its keep: every artifact here exists to manage a specific decision or risk, and gets simplified or removed when it stops doing so. To make that measurable, every run record tallies four numbers: findings confirmed, findings refuted (false-positive rate), findings that duplicated register entries (dedup failures), and findings resolved from previous runs. After any three consecutive runs of a charter, review the tallies — a charter that keeps producing refuted or duplicate findings gets its scope or trigger adjusted; one that produces nothing actionable gets its trigger loosened or the charter retired. Adjustments to this framework happen only in response to those tallies or a demonstrated gap, never speculatively.
- **Governance documents are production code.** Any change to this process, a charter, [Invariants.md](../Invariants.md), [RiskRegister.md](../pulsebar/RiskRegister.md), or [DefinitionOfDone.md](../pulsebar/DefinitionOfDone.md) gets independent review and evidence validation before merge — every factual claim (paths, identifiers, invariant values) re-verified against the working tree, not trusted from a draft. The registry's own INV-001 typo history is the standing reminder of why.
