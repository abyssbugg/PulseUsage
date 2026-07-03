# Repository Maintenance

## Maintenance Cadence

| Task | Cadence | Owner |
|---|---|---|
| Triage open issues | Weekly | Project owner |
| Triage Dependabot PRs | Weekly | Project owner |
| Delete merged branches (local + remote) | After every PR merge | PR author |
| Remove merged worktrees | After every PR merge | PR author |
| Prune Git metadata (`git gc`) | Monthly | Any contributor |
| Audit repository state | Quarterly | Project owner |
| Review stale remotes | Quarterly | Project owner |
| Review roadmap alignment | Quarterly | Project owner |

## Branch Maintenance

### After every PR merge

```bash
# 1. Delete local branch (git verifies it's merged)
git branch -d <branch-name>

# 2. Delete remote branch (if GitHub didn't auto-delete)
git push origin --delete <branch-name>

# 3. Remove any worktree for the branch
git worktree remove .worktrees/<worktree-name> 2>/dev/null
git worktree prune
```

### Quarterly audit

Run a full branch audit (see [Repository Governance Audit process](./Repository.md)):

1. List all local branches with ahead/behind/merged status.
2. Verify every branch is an ancestor of main (or has documented unique work).
3. Delete all merged branches.
4. Document any branches with unique unmerged work (decide: merge, rebase, or archive).

## Worktree Maintenance

### Rules

1. **One worktree per active branch.** No orphaned worktrees.
2. **Delete worktrees when the branch merges.**
3. **Worktrees must not have uncommitted work** when the branch is ready to merge.

### Quarterly audit

```bash
git worktree list
# For each worktree:
git -C .worktrees/<name> status --short
git -C .worktrees/<name> stash list
# If clean and branch is merged, remove:
git worktree remove .worktrees/<name>
git worktree prune
```

## Stash Maintenance

### Rules

1. **Stashes are temporary** — commit to a branch if the work is valuable.
2. **Stashes must not reference deleted branches** — if the base branch is gone, the stash is likely orphaned.
3. **Drop stashes after the work is merged or superseded.**

### Verification before dropping a stash

```bash
# View stash content
git stash show -p stash@{0}

# Verify the content is preserved elsewhere (e.g., in main)
git log --oneline main --grep="<feature>"
# Or check if a merged PR covers the stash content
gh pr list --state merged --search "<feature>"

# Drop if superseded
git stash drop stash@{0}
```

## Remote Maintenance

### Rules

1. **`origin` is permanent** — never remove.
2. **Other remotes (forks, upstreams) should be removed when stale.**
3. **Stale = no activity for 30+ days AND no relevant unmerged work.**

### Removing a remote + cleaning orphaned refs

```bash
git remote remove <name>

# Clean orphaned remote-tracking refs (git doesn't always do this automatically)
for ref in $(git for-each-ref --format='%(refname)' refs/remotes/<name>/); do
  git update-ref -d "$ref"
done

git pack-refs --all --prune
git gc --prune=now
```

### Quarterly remote audit

```bash
git remote -v
# For each remote (except origin):
# 1. Check last activity date
# 2. Check for unique unmerged work
# 3. Remove if stale and no unique work
```

## Git Metadata Maintenance

### Monthly garbage collection

```bash
git reflog expire --expire=now --all
git gc --prune=now
```

This reclaims disk space from loose objects and stale refs.

### After cleanup operations

After deleting branches, worktrees, remotes, or stashes, run:

```bash
git pack-refs --all --prune
git gc --prune=now
```

## Dependency Maintenance

### Dependabot PRs

Dependabot is configured (`.github/dependabot.yml`) for:
- `npm` (weekly, 5 open PRs limit)
- `cargo` in `/src-tauri` (weekly, 5 open PRs limit)

### Triage order (low risk first)

1. **Build tooling patches:** `tauri`, `tauri-build`
2. **Transitive patches:** `log`, `time`, `uuid`
3. **Dev deps:** `@types/node`

### Process

```bash
# Review the PR
gh pr view <PR#> --repo abyssbugg/PulseUsage

# Check CI status
gh pr checks <PR#> --repo abyssbugg/PulseUsage

# If green, merge (merge commit)
gh pr merge <PR#> --repo abyssbugg/PulseUsage --merge

# Run full test suite locally after merge
git checkout main && git pull --ff-only
bun run test
cargo test --manifest-path src-tauri/Cargo.toml
```

### npm dependency refresh (manual, periodic)

Beyond Dependabot patch bumps, periodically refresh major deps:

```bash
bun outdated
# Review the list; batch into a chore PR:
# bun update <package> (for each safe update)
# Test thoroughly before merging
```

## CI Maintenance

### Workflows (4)

| Workflow | File | Cadence | Maintenance |
|---|---|---|---|
| CI | `.github/workflows/ci.yml` | On PR | Verify jobs match local validation |
| Labeler | `.github/workflows/labeler.yml` | On PR | Verify labels config is current |
| Publish | `.github/workflows/publish.yml` | Disabled (`if: ${{ false }}`) | Re-enable only for v1.0 notarization |
| Stale | `.github/workflows/stale.yml` | Daily schedule | Verify stale thresholds (30 days + 7 days) |

### CI health check

```bash
gh run list --repo abyssbugg/PulseUsage --limit 10
# Verify all recent runs are "success"
# If any failed, investigate and fix
```

### CI annotations

GitHub Actions deprecation notices (e.g., Node.js 20 deprecation, macOS runner image migration) are non-blocking but should be addressed quarterly:

- Update `actions/checkout@v4` to newer versions when available
- Monitor `macos-latest` runner image changes (migrated to macOS 26 in June 2026)

## Version Alignment Maintenance

After every release, verify all 4 version files are aligned:

```bash
grep '"version"' package.json
grep '^version' src-tauri/Cargo.toml
grep '"version"' src-tauri/tauri.conf.json
grep -A1 '^name = "pulseusage"' src-tauri/Cargo.lock | head -2
```

All four must report the same version.

## Repository Size Maintenance

```bash
# Check .git size
du -sh .git

# If >100MB, run aggressive gc:
git reflog expire --expire=now --all
git gc --aggressive --prune=now

# Check for large files in history (if size is unexpectedly large)
git rev-list --objects --all | git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' | awk '/^blob/ {print $3, $4}' | sort -n | tail -20
```

## Release Readiness Documentation

Every release must have a readiness report at `docs/release-readiness/vX.Y.Z.md`:

- **Before release:** status "pending", blockers listed, pre-release checklist
- **After release:** status "released", validation results, known limitations, follow-up work

This creates a historical record of each release's readiness state.