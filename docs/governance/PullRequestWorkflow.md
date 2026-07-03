# Pull Request Workflow

## PR Lifecycle

1. **Create branch** from an up-to-date `main` (see [Branching.md](./Branching.md)).
2. **Push to origin** immediately to enable CI.
3. **Open PR** with a clear title and description.
4. **CI runs** automatically (Lint, Type-check, Build, Test, Provider Validator, Release Guards, Cargo Check, Cargo Test).
5. **Automated reviewers** post comments (CodeRabbit, Copilot, cubic-dev-ai, macroscopeapp, augmentcode, sourcery).
6. **Address review feedback** with separate commits (not squash) — preserves review history.
7. **Human review** — at least one human reviewer for non-trivial changes.
8. **Merge** via GitHub merge commit (not squash or rebase).
9. **Delete branch** locally and on origin after merge.

## PR Title

Use Conventional Commits format:
```
<type>(<scope>): <subject>
```

Examples from main history:
- `feat(plugin-engine): optional provider capability contracts (PR-1)`
- `fix(codex): normalize fresh-window 1% readings to 0%`
- `fix: keychain readGenericPassword accepts optional account argument`
- `chore: bump version to v0.6.28`
- `docs: add EDR-001, IMP-001, and IRR-001 planning artifacts`

## PR Description

Must include:
- **Summary** — what changed and why.
- **Validation** — what test suites were run and results.
- **Files changed** — if non-obvious.
- **Known limitations** — if any.

Example (from PR #14 keychain fix):
```markdown
## Summary
Fix keychain `readGenericPassword` signature mismatch that broke Copilot, Cursor, Claude, Codex, and Factory plugins on every launch.

## Validation
- JS: 1,099 pass / 0 fail
- Rust: 120 pass / 0 fail (+1 new contract test)
- Provider validator: 7 pass / 0 fail
- Live: copilot and cursor succeed with HTTP 200

## Files changed
- `src-tauri/src/plugin_engine/host_api.rs` (+54, -3)
- `docs/plugins/schema.md` (+30)
```

## Commit Strategy within a PR

- **Atomic commits.** Each commit is one logical change.
- **Review-feedback fixes are separate commits.** Do not squash fixup commits into the original. This preserves the review trail.
- **Conventional Commit messages for every commit.** Including fixup commits (`fix(plugin-engine): report capability string conversion errors`).

Example from PR #28 (good pattern):
```
feat(plugin-engine): optional provider capability contracts (PR-1)
fix(plugin-engine): report capability string conversion errors
fix(plugin-engine): keep capabilities fail-safe
```

## Merge Strategy

- **Merge commits** (GitHub default "Create a merge commit"). Preserves full history.
- **Never squash-merge.** Squashing loses the atomic-commit structure and review-feedback trail.
- **Never rebase-merge.** Rebasing rewrites history and breaks ancestry verification.

## Merge Gate (Required Before Merge)

Verify ALL of the following before merging:

| Gate | Check |
|---|---|
| CI green | `gh pr checks <PR#>` shows all required checks pass |
| Mergeable | `mergeable: MERGEABLE`, `mergeStateStatus: CLEAN` |
| No conflicts | `gh pr view <PR#> --json mergeable` confirms no conflicts |
| Review feedback addressed | All blocking review comments resolved or responded to |
| No force-push needed | History is clean; no rebase required |
| Version aligned | If version bump, all 4 version files match (`package.json`, `Cargo.toml`, `tauri.conf.json`, `Cargo.lock`) |

## Post-Merge Cleanup

Immediately after merge:

1. **Delete local branch:**
   ```bash
   git branch -d <branch-name>
   ```
2. **Delete remote branch** (GitHub usually does this automatically):
   ```bash
   git push origin --delete <branch-name>
   ```
3. **Remove any worktree** for the branch:
   ```bash
   git worktree remove .worktrees/<worktree-name>
   git worktree prune
   ```
4. **Drop any stash** that referenced the branch (after verifying supersession).
5. **Sync local main:**
   ```bash
   git checkout main
   git pull --ff-only
   ```

## PR Numbering

PRs are numbered sequentially by GitHub. Reference PRs in commit messages and docs as `#N`:
- `feat(plugin-engine): optional provider capability contracts (PR-1)` — internal roadmap PR number
- `Merge pull request #28 from abyssbugg/feat/provider-capability-contracts` — GitHub PR number

When a PR implements a roadmap item, include the roadmap PR number in the commit subject: `(PR-1)`, `(PR-2)`, etc.

## Automated Reviewer Integration

The repository has 6 automated reviewers configured:

| Reviewer | Role | Action |
|---|---|---|
| CodeRabbit | High-level summary + inline findings | Address Critical/Major findings; Low/Minor are optional |
| Copilot | Inline findings | Address; Copilot reviews the latest commit only |
| cubic-dev-ai | P1/P2/P3 severity findings | Address P1/P2; P3 optional |
| macroscopeapp | Severity-tagged findings | Address Medium+; Low optional |
| augmentcode | Severity-tagged findings | Address High; Medium/Low optional |
| sourcery | Reviewer's guide | Reference only |

**No PR is blocked by automated reviewers.** All reviews are `COMMENTED` state, not `REQUEST_CHANGES`. Human judgment decides what to address. See [CodeReview.md](./CodeReview.md) for triage rules.

## Dependabot PRs

Dependabot PRs follow a simplified workflow:

1. **Triage weekly.** Don't let them accumulate.
2. **Merge in risk order:** build tooling patches first (tauri, tauri-build), then transitive (log, time, uuid), then dev deps (types/node).
3. **Run full test suite** after each merge (`bun run test` + `cargo test`).
4. **Close superseded PRs** promptly (Dependabot usually does this automatically when a newer version is available).

## Forbidden PR Patterns

- **No PR without a description.** Even small PRs need a summary.
- **No PR with failing CI.** Fix CI before merging (unless CI is flaking — document the flake).
- **No PR that deletes files without explanation.** Deletions require a justification in the PR description.
- **No PR that renames the project** without an EDR (Engineering Decision Record) approving the rename.
- **No PR that imports functionality from OpenUsage or CodexBar** without explicit approval.