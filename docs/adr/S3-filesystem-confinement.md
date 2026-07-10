# ADR-S3: Filesystem Path Confinement for Plugin Host API

| Field | Value |
|---|---|
| **Status** | Proposed |
| **Date** | 2026-07-09 |
| **Trigger** | SEC-2026-07-09 finding SEC-006 (VULN-004) |
| **Related** | [SecurityCharter.md](../governance/audits/SecurityCharter.md), [SEC-2026-07-09](../governance/audits/history/SEC-2026-07-09.md) |

## Context

The filesystem host API (`ctx.host.fs.readText`, `writeText`, `listDir`, `exists`) accepts plugin-controlled path strings. The `expand_path` function (`shared.rs:19-31`) performs `~` expansion but applies no path confinement — a plugin with `fsRead` can read any file the macOS user can access.

An external security analysis flagged this as CWE-22 (Path Traversal) with LOW severity, noting it is defense-in-depth since plugins are currently bundled and trusted.

## Problem

The current trust model (same as ADR-S2) assumes all plugins are bundled and trusted. Under this model, a plugin reading an arbitrary file is either:
- **Intentional** — the plugin legitimately needs to read a provider-specific config (e.g., `~/.claude/config.json`, `~/Library/Application Support/Cursor/User/...`)
- **A bug** — the plugin has a path bug that reads the wrong file

There is no mechanism to prevent a compromised or buggy plugin from reading sensitive files (e.g., `~/.ssh/id_rsa`, `~/.aws/credentials`).

## Decision

**Defer implementation until third-party plugins are supported. Document the decision and the trigger conditions.**

### Rationale

1. **Current model is curated-only.** Same as ADR-S2. All 18 plugins are bundled, reviewed, and tested.
2. **Cost of implementation is non-trivial.** Per-plugin path allowlists require:
   - Manifest schema change (`fsAllowedPaths` or `fsAllowedRoots` array in `plugin.json`)
   - Migration of all 18 plugins to declare their path roots
   - Runtime enforcement in `fs.rs` (path canonicalization + prefix match)
   - Handling symlinks, `..` traversal, case sensitivity (macOS APFS is case-insensitive by default)
   - New tests for path enforcement
   - Documentation update for plugin authors
3. **ROI is low under curated model.** Same as ADR-S2 — code review catches wrong-path reads.
4. **Trigger for implementation:** Same as ADR-S2 — when third-party plugins are supported.

### Complexity Notes

Path confinement is harder than keychain service allowlists because:
- Paths need canonicalization (resolve symlinks, `..`, `.`, relative paths)
- macOS APFS is case-insensitive by default but case-preserving
- Paths may contain `~`, environment variables, or user-specific subdirectories
- A allowlist of roots (e.g., `~/.claude/`, `~/Library/Application Support/Cursor/`) is more practical than an exact-path allowlist but still needs canonicalization

### Trigger Conditions (when to implement)

- Third-party plugin support is officially announced
- A plugin is found reading sensitive files during review
- A security audit flags this as exploitable (currently defense-in-depth only)

## Alternatives Considered

### Alternative A: Implement now (root-prefix allowlist)
Add `fsAllowedRoots: ["~/.claude/", "~/Library/Application Support/Cursor/"]` to each plugin manifest, enforced via canonical path prefix match.
**Rejected for now.** Same reasoning as ADR-S2 — high cost, low value under curated model.

### Alternative B: Restrict to app data directory only
Only allow `fsRead`/`fsWrite` within `~/Library/Application Support/com.abyssbugg.pulseusage/`.
**Rejected.** Plugins need to read provider-specific files outside the app data directory (e.g., `~/.claude/config.json`, `~/.codex/auth.json`). This would break every plugin.

### Alternative C: No confinement, rely on code review
**Status quo.** Accept that bundled plugins are trusted and code review catches wrong-path reads.
**Accepted as current position.** This ADR documents the decision and the trigger for revisiting.

## Consequences

### Positive
- No manifest schema change needed now
- No migration burden on existing plugins
- Plugins can read any file they legitimately need without allowlist friction

### Negative
- If a bundled plugin is compromised, it can read any user file
- If third-party plugins are added without revisiting this ADR, the risk becomes CRITICAL

### Risk
- **Likelihood:** Low (requires compromised or buggy bundled plugin)
- **Impact:** High (arbitrary file read — could access SSH keys, AWS credentials, etc.)
- **Risk rating:** LOW-MEDIUM (acceptable under curated model, but higher impact than keychain because file contents are not redacted)

## Open Questions

- What paths do the 18 bundled plugins actually read? (Needs inventory.)
- Should `fsWrite` have stricter confinement than `fsRead`? (Writing is more dangerous — could overwrite sensitive files.)
- If third-party plugins are supported, should the allowlist be roots (prefix match) or exact paths?

## Next Steps

1. This ADR is reviewed and accepted/rejected
2. If accepted: document the trigger in ROADMAP.md
3. If third-party plugins are planned: reopen this ADR and implement before the first third-party plugin ships
4. Consider a separate audit of all `fsRead`/`fsWrite` call sites across bundled plugins to inventory current path patterns (useful regardless of ADR decision)