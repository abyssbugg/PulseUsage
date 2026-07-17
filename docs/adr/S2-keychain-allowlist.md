# ADR-S2: Keychain Service-Name Allowlist

| Field | Value |
|---|---|
| **Status** | Proposed |
| **Date** | 2026-07-09 |
| **Trigger** | SEC-2026-07-09 finding SEC-005 (VULN-003) |
| **Related** | [SecurityCharter.md](../governance/audits/SecurityCharter.md), [SEC-2026-07-09](../governance/audits/history/SEC-2026-07-09.md) |

## Context

The keychain host API (`ctx.host.keychain.readGenericPassword(service, account?)`) accepts a plugin-controlled `service` parameter. Any plugin with the `keychainRead` capability can read any keychain service the macOS user has access to — not just services related to that plugin's provider.

An external security analysis flagged this as CWE-732 (Incorrect Permission Assignment for Critical Resource) with LOW severity, noting it is defense-in-depth since plugins are currently bundled and trusted.

## Problem

The current trust model assumes all plugins are bundled, reviewed, and trusted. Under this model, a plugin reading an unrelated keychain service is either:
- **Intentional** — the plugin legitimately needs cross-service access
- **A bug** — the plugin has a path bug that reads the wrong service

There is no mechanism to prevent a compromised or buggy plugin from reading credentials for unrelated services (e.g., a Claude plugin bug that accidentally reads a Cursor service entry).

## Decision

**Defer implementation until third-party plugins are supported. Document the decision and the trigger conditions.**

### Rationale

1. **Current model is curated-only.** All 18 plugins are bundled, reviewed, and tested. No third-party plugin loading is supported (though `dev_dir` and `install_dir` exist for local development).
2. **Cost of implementation is non-trivial.** Per-plugin service allowlists require:
   - Manifest schema change (`keychainServices` array in `plugin.json`)
   - Migration of all 18 plugins to declare their service names
   - Runtime enforcement in `keychain.rs` (match service against allowlist)
   - New tests for allowlist enforcement
   - Documentation update for plugin authors
3. **ROI is low under curated model.** The attack requires a compromised or buggy bundled plugin — both are caught by code review and testing before merge.
4. **Trigger for implementation:** When third-party plugins are officially supported (outside `dev_dir`), this becomes CRITICAL and must be implemented before the first third-party plugin ships.

### Trigger Conditions (when to implement)

- Third-party plugin support is officially announced
- A plugin is found reading an unrelated service during review
- A security audit flags this as exploitable (currently defense-in-depth only)

## Alternatives Considered

### Alternative A: Implement now (permissive allowlist)
Add `keychainServices: ["Claude Code-*"]` to each plugin manifest, enforced at runtime.
**Rejected for now.** High implementation cost, low immediate value under curated model. Re-evaluate at third-party-plugin milestone.

### Alternative B: Prefix-based allowlist (automatic)
Automatically derive the allowlist from the plugin ID (e.g., `claude` plugin can only read services starting with `Claude` or `claude`).
**Rejected.** Too rigid — some providers use non-obvious service names (e.g., Copilot uses `github.com` or `VS Code GitHub Copilot`). Would require manual overrides anyway.

### Alternative C: No allowlist, rely on code review
**Status quo.** Accept that bundled plugins are trusted and code review catches wrong-service reads.
**Accepted as current position.** This ADR documents the decision and the trigger for revisiting.

## Consequences

### Positive
- No manifest schema change needed now
- No migration burden on existing plugins
- Code review remains the gate (it already reviews every plugin)

### Negative
- If a bundled plugin is compromised, it can read any keychain service
- If third-party plugins are added without revisiting this ADR, the risk becomes CRITICAL

### Risk
- **Likelihood:** Low (requires compromised or buggy bundled plugin)
- **Impact:** Medium (cross-service credential access)
- **Risk rating:** LOW (acceptable under curated model)

## Open Questions

- What service names do the 18 bundled plugins actually use? (Needs inventory.)
- Are there any plugins that legitimately need cross-service access? (e.g., does any plugin read a shared "PulseUsage" service?)
- If third-party plugins are ever supported, should the allowlist be per-plugin or per-capability?

## Next Steps

1. This ADR is reviewed and accepted/rejected
2. If accepted: document the trigger in ROADMAP.md
3. If third-party plugins are planned: reopen this ADR and implement before the first third-party plugin ships