# Code Review Standards

## Review Philosophy

PulseUsage uses a **defense-in-depth review model**: human review + 6 automated reviewers. No single reviewer is a gate; review is a collaborative process that improves quality without blocking progress.

## Automated Reviewers

| Reviewer | Role | Severity Levels | Action Required |
|---|---|---|---|
| **CodeRabbit** | High-level summary + inline findings + walkthrough | Critical, Major, Minor, Potential issue | Address Critical/Major; Minor optional |
| **Copilot** | Inline findings on latest commit | (no severity tags) | Address; Copilot reviews only the latest commit, so comments may be stale if issues were fixed in earlier commits |
| **cubic-dev-ai** | P1/P2/P3 severity with confidence scores | P1 (critical), P2 (medium), P3 (low) | Address P1/P2; P3 optional. cubic self-marks "Addressed in <commit>" when fixed. |
| **macroscopeapp** | Severity-tagged findings | High, Medium, Low | Address Medium+; Low optional. macroscope self-marks "Resolved in <commit>". |
| **augmentcode** | Severity-tagged findings with "Fix This in Augment" links | High, Medium, Low | Address High; Medium/Low optional. augment does NOT self-mark resolutions — verify by checking current code. |
| **sourcery** | Reviewer's guide (summary only, no inline findings) | N/A | Reference only |

## Review Triage Rules

### Rule 1: Verify before addressing

Before addressing any review comment, verify the finding against current code:
- Is the flagged line still present in the latest commit?
- Was the issue already fixed in an earlier commit?
- Is the comment stale (bot reviewed a pre-fix commit)?

**Stale comments are common.** Bots review specific commits; if a fix landed in a later commit, the comment on the earlier commit is stale. Document the staleness in the PR discussion, don't re-fix.

### Rule 2: Severity-based triage

| Severity | Action | Example |
|---|---|---|
| Critical / P1 / High | **Must address before merge** | Security issue, data loss, crash |
| Major / P2 / Medium | **Should address before merge** (justify if deferred) | Logic error, missing edge case |
| Minor / P3 / Low | Optional — address if cheap, defer if expensive | Style, naming, micro-optimization |
| Potential issue | Investigate; address if real, document if false positive | |

### Rule 3: No performative agreement

Do not blindly implement review suggestions. Evaluate each:
- Is the suggestion technically correct?
- Does it align with the project's simplicity principle (AGENTS.md)?
- Does it introduce over-engineering?

If a suggestion is wrong, explain why in the PR discussion. If it's right, implement it. If it's a matter of opinion, defer to the maintainer.

### Rule 4: Review feedback as separate commits

When addressing review feedback, create **separate commits** (not squashed into the original):
```
feat(plugin-engine): optional provider capability contracts (PR-1)
fix(plugin-engine): report capability string conversion errors  ← review feedback
fix(plugin-engine): keep capabilities fail-safe                 ← review feedback
```

This preserves the review trail and shows the evolution of the fix.

## Self-Marked Resolution Verification

Some bots (cubic-dev-ai, macroscopeapp) append "Addressed in <commit>" or "Resolved in <commit>" to their comments when they detect a fix. **Verify these claims** — a bot may claim resolution without the fix actually addressing the issue:

```bash
# Did the referenced commit actually touch the flagged file?
git show <commit> --name-only | grep <flagged-file>

# Did the commit actually fix the flagged line?
git show <commit> -- <flagged-file> | grep <flagged-line-context>
```

## Human Review

### When required

- **All non-trivial changes** (logic changes, new features, security-sensitive code)
- **All changes to `host_api.rs`** (security-critical: keychain, plist, sqlite, http, redaction)
- **All new plugins or plugin manifest changes** (per AGENTS.md: audit redaction lists)
- **All CI/release script changes**

### When optional

- Docs-only changes
- Dependabot patch bumps (CI validates)
- Cosmetic changes (formatting, comments)

### Reviewer count

- **1 human reviewer** minimum for non-trivial PRs.
- **2 human reviewers** for security-critical changes (`host_api.rs`, redaction, auth).

## Review Checklist

For every PR, the reviewer verifies:

### Correctness
- [ ] Code does what the PR description claims
- [ ] Edge cases handled (empty input, null, timeout, error states)
- [ ] No silent fallbacks (AGENTS.md: fail loud)
- [ ] Tests cover the new behavior

### Security
- [ ] No secrets in code or tests (use obvious placeholders for test fixtures)
- [ ] No new `unwrap()`/`expect()` in production paths without justification
- [ ] Host API changes audited for redaction coverage (AGENTS.md)
- [ ] No `dangerouslyIgnoreTls` for non-localhost URLs
- [ ] No unrestricted file read/write without capability gating

### Performance
- [ ] No subprocess spawning in hot paths (use Rust-native crates for v1.0+)
- [ ] No unbounded loops or allocations
- [ ] Probe worker count bounded (current: 4)

### Maintainability
- [ ] Files under ~400 LOC (AGENTS.md) — or justified split plan
- [ ] Conventional Commit message
- [ ] No debug statements (`println!`, `console.log`, `dbg!`) left in
- [ ] No `TODO`/`FIXME`/`HACK` without a linked issue

### AGENTS.md Compliance
- [ ] `brandColor` set to provider's real brand color (for new plugins)
- [ ] SVG icons use `currentColor`
- [ ] README plugin list updated (for new/removed plugins)
- [ ] Redaction lists audited for new plugin response fields

## Approval

- **No `APPROVE` required** from automated reviewers. They only `COMMENT`.
- **Human `APPROVE`** is required for non-trivial PRs (project owner decision).
- **No `REQUEST_CHANGES`** from any reviewer blocks merge — the maintainer decides.

## Merge Gate

Before merging, verify (see [PullRequestWorkflow.md](./PullRequestWorkflow.md#merge-gate-required-before-merge)):
- CI green
- Mergeable / CLEAN
- No conflicts
- Blocking review comments addressed
- Version files aligned (if version bump)