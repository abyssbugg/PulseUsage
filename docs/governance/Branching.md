# Branching Governance

## Branch Naming Conventions

All branches must use lowercase kebab-case with a typed prefix. The following prefixes are canonical:

| Prefix | Purpose | Example |
|---|---|---|
| `feat/` | New feature or capability | `feat/provider-capability-contracts` |
| `fix/` | Bug fix | `fix/codex-fresh-window` |
| `chore/` | Maintenance, deps, version bumps | `chore/bump-version-0.6.27` |
| `ci/` | CI/CD workflow changes | `ci/rust-validation` |
| `docs/` | Documentation-only changes | `docs/release-readiness-v0.6.28` |
| `test/` | Test-only additions or fixes | `test/audit-provider-stability` |
| `hardening/` | Security, diagnostics, or reliability hardening initiatives | `hardening/provider-diagnostics-v0628` |
| `audit/` | Audit or review work | `audit/provider-stability` |
| `restore/` | Restoration of previously-removed features | `restore/warp-provider` |
| `brand/` | Branding, icons, visual identity | `brand/pulseusage-icon-release` |
| `setup/` | Repository setup or distribution changes | `setup/direct-download-distribution` |
| `research/` | Research and planning (docs-only, no production code) | `research/pulsebar-architecture` |
| `dependabot/` | Automated dependency bumps (Dependabot only) | `dependabot/cargo/src-tauri/tauri-2.11.3` |

## Branch Lifetime

### Short-lived branches (default)

Feature, fix, chore, ci, docs, test, hardening, audit, restore, brand, setup branches should be short-lived:
- **Create** when work begins.
- **Push** to origin immediately (enables CI and automated reviewers).
- **Open PR** when ready for review.
- **Merge** when approved and CI is green.
- **Delete** the branch immediately after merge (locally and on origin).

Target lifetime: **under 1 week**. Maximum: **2 weeks**. If a branch lives longer, it should be split into smaller PRs.

### Long-lived branches (exceptions)

- `main` — the production trunk. Lives forever. Never force-pushed.
- `research/*` — research branches. May live longer but must be docs-only. Delete when findings land in main.

### Forbidden patterns

- No `dev`, `develop`, `staging`, `release/*` branches. `main` is the only long-lived integration branch.
- No branch names with uppercase letters, underscores, or spaces.
- No branch names that match a tag name (e.g., avoid `v0.6.28` as a branch).

## Branch Creation

```bash
# Create from an up-to-date main
git checkout main
git pull --ff-only
git checkout -b feat/my-feature

# Push to origin
git push -u origin feat/my-feature
```

## Branch Deletion

### Local branch deletion (after merge)

```bash
# Safe delete (git verifies the branch is merged)
git branch -d feat/my-feature

# The -d flag refuses to delete unmerged branches. Never use -D unless you have
# verified the branch is an ancestor of main via:
git merge-base --is-ancestor feat/my-feature main && echo "safe" || echo "NOT SAFE"
```

### Remote branch deletion (after merge)

```bash
# Delete the remote branch (GitHub usually does this automatically on merge)
git push origin --delete feat/my-feature
```

## Branch Integrity Rules

1. **Never force-push to `main` or any shared branch.** History is immutable.
2. **Never rebase merged branches.** Use merge commits to preserve history.
3. **Never delete a branch without verifying it is an ancestor of main:**
   ```bash
   git merge-base --is-ancestor <branch> main && git branch -d <branch>
   ```
4. **Never leave a branch unmerged if it has unique commits.** Either merge it, or explicitly document why it is abandoned (with a `chore: archive <branch>` commit noting the decision).

## Worktree Conventions

Worktrees are used for parallel work isolation. Conventions:

| Convention | Rule |
|---|---|
| Location | `.worktrees/<branch-name-with-slashes-to-dashes>` |
| One per task | One worktree per active branch. Delete when the task's PR merges. |
| Naming | `.worktrees/research-pulsebar-architecture` (slashes → dashes) |
| Cleanup | `git worktree remove .worktrees/<name>` after merge, then `git worktree prune` |
| No nested worktrees | Worktrees must not create their own worktrees. |

## Stash Conventions

Stashes are temporary. Rules:

1. **Stashes must reference a branch that still exists.** If the base branch is deleted, the stash is likely orphaned.
2. **Stashes must not contain unique work.** If a stash has unique content, commit it to a branch instead.
3. **Drop stashes after the work is merged or abandoned.** `git stash drop stash@{0}` after verifying the content is preserved elsewhere.
4. **Stash messages must be descriptive.** `git stash push -m "wip: keychain fix for PR-002"` not `git stash push`.

## Remote Conventions

| Remote | Purpose | Status |
|---|---|---|
| `origin` | Canonical repository | Permanent — never remove |
| (others) | Forks or upstreams | Remove when stale. Stale = no activity for 30+ days and no relevant unmerged work. |

When removing a remote, also clean orphaned remote-tracking refs:
```bash
git remote remove <name>
for ref in $(git for-each-ref --format='%(refname)' refs/remotes/<name>/); do git update-ref -d "$ref"; done
git pack-refs --all --prune
git gc --prune=now
```