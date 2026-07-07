# Active Branches

> **Canonical branch inventory.** Future agents must verify branch state against this document.
> Last updated: 2026-07-07 (commit `1863846`)

## Local Branches (13)

| Branch | SHA | Purpose | Status | Recommendation |
|---|---|---|---|---|
| `main` | `1863846` | Production trunk (== origin/main) | Active | **Keep** — canonical |
| `release/v0.7.0` | `e6b34c6` | Frozen release branch, verified RC fixes only | Active, protected | **Keep** |
| `docs/governance-audit-framework` | `b874153` | PR #57 — governance & audit framework | Open PR | **Keep until merge** |
| `docs/current-phase-refresh` | — | PR #58 — CURRENT_PHASE.md drift fix | Open PR | **Keep until merge** |
| `docs/active-branches-refresh` | — | This update | Open PR | **Keep until merge** |
| `docs/pulsebar-governance-lock` | `46f986e` | PR #52 — merged | Merged | **Delete** |
| `docs/pulsebar-v0.8-baseline` | `1afea14` | v0.8.0 baseline docs — merged | Merged | **Delete** |
| `feat/program2-5-compat-hardening` | `dccec24` | Program 2.5 — merged | Merged | **Delete** |
| `feat/program2-all-providers-capabilities` | `aa3d170` | Program 2 — merged | Merged | **Delete** |
| `feat/program2-cursor-capabilities` | `4ede1c4` | Program 2 — merged | Merged | **Delete** |
| `feat/program2-orchestrator-enforcement` | `dbb5576` | Program 2 — merged | Merged | **Delete** |
| `feat/pulsebar-identity-foundation` | `e40c580` | PR #54 (A1) — merged | Merged | **Delete** |
| `research/uiux-redesign` | `c37237b` | Research branch; tip is an ancestor of main | Merged, no unique work | **Delete** |

Cleanup command for the merged set (verify each is in the Merged rows above first):

```sh
git branch -d docs/pulsebar-governance-lock docs/pulsebar-v0.8-baseline \
  feat/program2-5-compat-hardening feat/program2-all-providers-capabilities \
  feat/program2-cursor-capabilities feat/program2-orchestrator-enforcement \
  feat/pulsebar-identity-foundation research/uiux-redesign
```

`git branch -d` (not `-D`) refuses to delete anything unmerged — it is the safety check.

## Remote Branches (origin)

| Category | Branches | Status | Recommendation |
|---|---|---|---|
| Canonical | `origin/main`, `origin/release/v0.7.0` | Active | **Keep** |
| Open PRs | `origin/docs/governance-audit-framework` (#57), `origin/docs/current-phase-refresh` (#58), `origin/docs/active-branches-refresh`, `origin/dependabot/cargo/src-tauri/aes-gcm-0.11.0` (#35) | Open | **Keep until merge** |
| Merged, unpruned | 26 branches: `chore/*` (3), `docs/pulsebar-*` + `docs/sync-program-1-completion` (3), `feat/program2-*` + `feat/program-b-design-tokens` + `feat/pulsebar-identity-*` (7), `refactor/*` (13) | All ancestors of `origin/main` | **Prune** — delete on GitHub or enable auto-delete of merged branches in repo settings |

Enabling **Settings → General → Automatically delete head branches** prevents this backlog from re-accumulating.

## Worktrees (1)

| Worktree | Branch | Status | Recommendation |
|---|---|---|---|
| `/Users/datamatics/Desktop/PulseUsage` (main repo) | `main` | Clean (2 untracked UX planning docs) | **Keep** — primary working directory |

Temporary worktrees created for docs-only PR branches are removed after push and are not inventoried.

## Stashes (0)

None. Clean.

## Remotes (1)

| Remote | URL | Status |
|---|---|---|
| `origin` | `https://github.com/abyssbugg/PulseUsage.git` | Active (fetch + push) — canonical |

## Branch Health

| Metric | Value | Target |
|---|---|---|
| Total local branches | 13 (5 keep + 8 deletable — wait for open-PR merges to reach ≤5) | ≤5 |
| Stale branches (merged, not deleted) | 8 local, 26 remote | 0 |
| Diverged branches (ahead of main) | 3 (all open PRs) | Open PRs only |
| Unmerged unique work | Open PRs only | 0 outside PRs |
| Worktrees | 1 | ≤2 per active task |
| Stashes | 0 | 0 |
| Remotes | 1 | 1 (origin only) |

## Post-Merge Cleanup

After merging each PR:
1. Delete the local branch: `git branch -d <branch>`
2. Delete the remote branch (or rely on GitHub auto-delete once enabled)
3. Sync local main: `git checkout main && git pull --ff-only`
4. Run full test suite: `bun run test && cargo test --manifest-path src-tauri/Cargo.toml`

## Branch Naming Conventions

See [docs/governance/Branching.md](./docs/governance/Branching.md) for the full naming convention. Summary:
- `feat/`, `fix/`, `chore/`, `ci/`, `docs/`, `test/` — standard typed prefixes
- `hardening/`, `audit/`, `restore/`, `brand/`, `setup/`, `research/` — project-specific prefixes
- `dependabot/` — automated (Dependabot only)
- All lowercase, kebab-case, no underscores/spaces
