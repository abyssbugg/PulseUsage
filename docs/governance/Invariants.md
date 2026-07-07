# PulseBar Invariant Registry

> **Canonical invariant registry.** Properties that must hold at every commit on `main`.
> Every audit (see [audits/AuditProcess.md](audits/AuditProcess.md)) verifies these **before** searching for new issues.
> Changing an invariant requires the escalation listed in its row and an update to this file in the same PR.
> Last updated: 2026-07-07

## How to read this table

- **Enforcement** — `test` (a CI test fails on violation), `audit` (checked by an audit charter; no automated guard yet), `process` (guarded by branch protection / PR review).
- **Check** — the exact command or file an auditor uses to verify. An invariant without a concrete check is a wish, not an invariant.
- Every value below was verified against the working tree on 2026-07-07. Do not edit from memory; re-verify.

## Invariants

| ID | Invariant | Enforcement | Check | Escalation |
|---|---|---|---|---|
| INV-001 | Bundle identifier is `com.abyssbugg.pulseusage` and never changes without an approved migration program. | audit | `src-tauri/tauri.conf.json` `identifier` field | [RiskRegister](pulsebar/RiskRegister.md) — Critical, release blocker |
| INV-002 | Application Support directory path never changes. Existing settings, plugin order, disabled providers, cache, and logs remain usable across releases. | audit | `src-tauri/src/config.rs`, `log_path.rs` path derivation | RiskRegister — Critical |
| INV-003 | Keychain service names are never renamed. Any future change requires old-read/new-write fallback for one full release. | audit | `src-tauri/src/plugin_engine/host_api` keychain functions; plugin `plugin.js` call sites | RiskRegister — Critical |
| INV-004 | All user-visible product-name and external-link strings flow through the identity layer: `src-tauri/src/identity.rs` (Rust) and `src/lib/app-identity.ts` (frontend). No hardcoded "PulseBar" strings outside these modules and `tauri.conf.json`. | test + audit | `src/lib/app-identity.test.ts`; grep for hardcoded product strings outside identity modules | Program A charter |
| INV-005 | No new hardcoded visual values in `src/components/`. All colors, font sizes, control heights, radii, elevation, and motion use the design tokens defined in `src/index.css` per [ADR-002](../adr/002-design-system.md) and the [component contract](../design-system/component-contract.md). *Activates when ADR-002 merges with the Program B foundation PR; until then the linked docs exist only on the Program B branch.* | audit | grep `src/components` for hex colors, `text-[`, `h-[`, `shadow-`, `duration-` arbitrary values | ADR-002 |
| INV-006 | `release/v0.7.0` accepts verified RC bug fixes only. `main` and `release/v0.7.0` are protected. | process | `CURRENT_PHASE.md` release branch policy; branch protection settings | CURRENT_PHASE.md |
| INV-007 | Plugin capability enforcement is least-privilege: schema-v2 manifests declare `hostCapabilities`; the host grants nothing undeclared. v1 compatibility inference is never removed without the deprecation policy's adoption-metric gate. | test + audit | `src-tauri/src/plugin_engine/capability.rs` and its tests | RiskRegister — Critical |
| INV-008 | JS plugin globals and plugin contracts are never partially renamed. Any rename is atomic across host, all bundled plugins, and tests. | audit | `plugins/*/plugin.js` global references vs host injection in `plugin_engine/host_api` | RiskRegister — High |
| INV-009 | GitHub repository remains `abyssbugg/PulseUsage`; all release/support URLs flow through `identity.rs` constants. | test | `identity.rs` unit tests; `RELEASES_URL` et al. | RiskRegister — High |
| INV-010 | Secrets (API keys, tokens) never appear in logs or diagnostics. Redaction in `plugin_engine/redaction.rs` covers every new auth source before it ships. | test + audit | `redaction.rs` tests; per-provider DoD in [DefinitionOfDone](pulsebar/DefinitionOfDone.md) | DefinitionOfDone — security bullet |
| INV-011 | Control-center docs (`CURRENT_PHASE.md`, `ACTIVE_BRANCHES.md`, `TECHNICAL_DEBT.md`, `PROJECT_STATUS.md`) reflect reality. Staleness beyond the current program boundary is a Medium finding. | audit | compare doc claims against `git branch -a`, open PRs, milestone state | Governance |

## Adding an invariant

1. Verify the property holds **today** (run the Check yourself — see INV-001's history: a draft of this registry misspelled the bundle ID as `com.abyssbug.pulseusage`).
2. Prefer `test` enforcement; add `audit` only when a test is impractical, and say why.
3. Link the escalation owner. An invariant nobody can waive is an invariant nobody maintains.
