# Architecture Audit Charter (ARC)

> Trigger: completion of an implementation program phase (A–F); a new ADR proposed. Process: [AuditProcess.md](AuditProcess.md).

## Invariants to verify first

INV-004 (identity layer), INV-007 (capability model), INV-008 (plugin contract atomicity), INV-011 (control-center doc freshness).

## Scope

| Concern | Where to look | What to examine |
|---|---|---|
| ADR compliance | `docs/adr/` vs implementation | Does merged work match ADR-001 (provider usage baseline) and ADR-002 (design system)? Any de-facto architectural decision made in code without an ADR? |
| Module boundaries | `src-tauri/src/` top-level modules; `src/` (`components`, `hooks`, `stores`, `lib`, `pages`) | Cohesion per module; cross-module reach-ins; does `plugin_engine` leak types into UI-adjacent code? |
| Identity layering | `src-tauri/src/identity.rs`, `src/lib/app-identity.ts` | All rename-sensitive strings behind the layer; no new hardcoded surfaces since last run (Program A DoD) |
| Provider platform | `plugins/`, manifest schema usage | Schema v2 adoption; v1 compatibility surface unchanged; per-provider coupling to host internals |
| State architecture | `docs/app-state-architecture.md` vs `src/stores/` | Doc still describes reality; store boundaries respected |
| Dependency direction | imports across `src-tauri/src/` | No cycles; `lib.rs` as composition root, not logic dump |

## Method

1. Invariant checks, then read the completed program's Definition of Done section and verify each bullet against the merged diffs — DoD bullets are the spec; the audit asks "nothing missing, nothing extra."
2. Map new coupling introduced by the phase: for each new `use`/import edge crossing a module boundary, ask whether it was forced or convenient.
3. ADR gap check: list decisions visible in the phase's diffs that constrain future work; each needs an ADR or an explicit note that it doesn't.
4. Refactoring recommendations require a measurable benefit statement (fewer edges, deleted code, removed duplication) or they are discarded at synthesis.

## Out of scope

Visual/UX consistency (UX charter). Performance characteristics (PERF charter). The PulseBar full rename (IMP-005 — planned, not drift).

## Run history

Completed runs are recorded as individual files in [history/](history/) named `<CHARTER>-<YYYY-MM-DD>.md`.
