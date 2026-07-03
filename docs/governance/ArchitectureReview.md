# Architecture Review

## Architecture Decision Records (EDR)

Significant architecture decisions are documented as **Engineering Decision Records** under `docs/edr/`.

### When an EDR is required

An EDR is required for:
- New major version (v0.7.0, v1.0.0)
- Breaking changes to the plugin engine or host API
- New host API capabilities (plist, keychain, sqlite, http, etc.)
- Changes to the release strategy (e.g., enabling notarization)
- Project rename or rebrand
- Importing functionality from another project
- Adopting a new major dependency (Tauri 3.x, React 20, etc.)
- Security architecture changes (capability model, sandbox, notarization)

An EDR is **not** required for:
- Bug fixes
- New providers (these follow the plugin manifest schema)
- Dependency patch/minor bumps
- Docs-only changes
- Refactoring that doesn't change behavior

### EDR Format

```markdown
# EDR-NNN: <Decision Title>

**Status:** Proposed | Accepted | Superseded | Deprecated
**Date:** YYYY-MM-DD
**Author:** <author>

## Context

<Why is this decision needed? What problem does it solve?>

## Decision

<What is the decision? Be specific.>

## Consequences

### Positive
- <benefit>

### Negative
- <trade-off>

### Neutral
- <observation>

## Alternatives Considered

### <Alternative 1>
<Why rejected?>

### <Alternative 2>
<Why rejected?>

## References

- <links to related EDRs, IMPs, PRs, issues>
```

### EDR Numbering

EDRs are numbered sequentially (`EDR-001`, `EDR-002`, ...). Once accepted, an EDR is immutable; supersession creates a new EDR that references the old one.

### Current EDRs

| EDR | Title | Status |
|---|---|---|
| EDR-001 | PulseBar architecture direction | (see `docs/edr/001-pulsebar-direction.md`) |

## Implementation Master Plans (IMP)

Significant implementation work is planned as **Implementation Master Plans** under `docs/imp/`.

### When an IMP is required

An IMP is required for:
- Multi-PR features (2+ PRs)
- Features that span multiple releases
- Features that require architectural changes

An IMP is **not** required for:
- Single-PR features
- Bug fixes
- Dependency bumps

### IMP Format

```markdown
# IMP-NNN: <Feature Title>

**Status:** Planned | In Progress | Complete | Abandoned
**Date:** YYYY-MM-DD
**Owner:** <owner>
**Related EDR:** EDR-NNN (if applicable)

## Goal

<What will this feature achieve?>

## Scope

<What is in scope? What is out of scope?>

## Implementation Plan

### PR-1: <title>
- <changes>
- <validation>

### PR-2: <title>
- <changes>
- <validation>

## Risks

- <risk + mitigation>

## Validation

<How will we verify the feature is complete and correct?>

## Rollback

<How to roll back if the feature causes problems?>
```

### Current IMPs

| IMP | Title | Status |
|---|---|---|
| IMP-001 | Implementation master plan | (see `docs/imp/001-implementation-master-plan.md`) |
| IMP-002 | IRR readiness review | (see `docs/imp/002-irr-readiness-review.md`) |
| IMP-003 | Provider usage architecture roadmap | (see `docs/imp/003-provider-usage-architecture-roadmap.md`) |
| IMP-004 | Provider usage implementation roadmap | (see `docs/imp/004-provider-usage-implementation-roadmap.md`) |

## Implementation Readiness Reviews (IRR)

Before starting implementation of an IMP, an **Implementation Readiness Review** (IRR) verifies the plan is executable. IRRs are under `docs/imp/` with the `irr-` prefix or as `002-irr-readiness-review.md`.

### IRR verifies

- The IMP is complete and unambiguous
- Dependencies are available
- The codebase is in a state to accept the changes
- Risks are identified and mitigatable
- Validation approach is defined

## Architecture Review Board (ARB)

For enterprise-scale decisions, an informal ARB review is recommended:

### ARB Composition
- Project owner (decision maker)
- Author of the EDR/IMP
- One independent reviewer (can be an automated reviewer's summary)

### ARB Process
1. Author presents EDR/IMP
2. ARB asks questions, identifies risks
3. ARB approves, rejects, or requests changes
4. Author incorporates feedback
5. ARB gives final approval

### ARB Meeting Cadence

- **As needed** — when an EDR or IMP is proposed
- Not scheduled — PulseUsage is a small project; formal meetings are overhead

## Architectural Principles

### 1. Plugin sandboxing

Plugins run in a QuickJS sandbox (`rquickjs`). Host APIs (`host.*`) are the only bridge to the system. Plugins must not have direct filesystem, network, or keychain access — only through `ctx.host.*`.

### 2. Defense-in-depth security

- Redaction in logs (`redact_body`, `redact_log_message`, `redact_diagnostic_text`)
- Per-plugin diagnostics recorders (auth reads, local reads, HTTP counts)
- `dangerouslyIgnoreTls` restricted to localhost (v0.6.29 hardening — planned)
- Plugin capability manifest (v0.7 — planned)

### 3. Fail-loud error handling

- Expected issues: explicit `Result` types (not throw/catch)
- Unexpected issues: `throw` + `log::error!` + user-facing `toast.error`
- Never silent fallbacks (AGENTS.md)

### 4. Process-exec isolation

Host APIs that shell out (`plutil`, `security`, `sqlite3`) are isolated in named `inject_*` functions. v1.0+ will replace these with Rust-native crates (`plist`, `rusqlite`, `security-framework`) — but only if energy/reliability concerns materialize (AGENTS.md: simplicity first).

### 5. Monolith awareness

`host_api.rs` is 4,727 LOC (v0.6.28). This is acknowledged technical debt. v0.7 will modularize it into `host_api/{log,fs,plist,crypto,env,http,ls,ccusage,keychain,sqlite}.rs` + `redaction.rs`. No behavior change — pure extraction.

## Architecture Decision Log

| Decision | Date | Rationale |
|---|---|---|
| Tauri 2.x + rquickjs plugin engine | Project inception | Cross-platform, sandboxed plugins |
| Direct-download distribution (no notarization) | v0.6.27 | 2-5 internal users; AGENTS.md simplicity |
| Conventional Commits | Project inception | Industry standard, enables changelog automation |
| Merge commits (not squash) | Project inception | Preserves atomic-commit and review trail |
| Manual release process | v0.6.27 | `publish.yml` disabled; `gh release create` manual |
| `Opt<String>` for optional JS params | v0.6.28 | rquickjs `Option<T>` is not optional at JS layer; `Opt<T>` is |

## Future Architecture Considerations

### Under evaluation (do not implement until triggered)

- **Plugin capability manifest** (v0.7) — `"capabilities": [...]` in `plugin.json`, enforced in `inject_host_api_with_deadline`
- **`host_api.rs` modularization** (v0.7) — split 4,727-LOC monolith
- **Process-exec native crates** (v1.0, conditional) — replace subprocess spawning
- **Notarization** (v1.0, conditional) — if macOS 27 enforces
- **App Sandbox** (v1.0, conditional) — if macOS 27 enforces
- **Strict-mode provider validation in CI** (v0.7, after Perplexity evidence gathered)

These are documented for awareness only. Implementation requires an EDR and IMP per the process above.