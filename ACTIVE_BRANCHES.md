# Active Branches

> **Canonical branch inventory.** Future agents must verify branch state against this document.
> Last updated: 2026-07-03 (commit `e749c72`)

## Local Branches (2)

| Branch | SHA | Date | Purpose | Status | Recommendation |
|---|---|---|---|---|---|
| `main` | `e749c72` | 2026-07-03 | Production trunk (== origin/main) | Active | **Keep** — canonical |
| `research/uiux-redesign` | `e749c72` | 2026-07-03 | Identical to main (0/0) | Placeholder (forbidden work — not started) | **Keep for now** — delete if UX redesign is formally abandoned, or use when approved |

## Remote Branches (origin — 7 tracking refs)

| Branch | SHA | Purpose | Status | Recommendation |
|---|---|---|---|---|
| `origin/main` | `e749c72` | Canonical trunk | Active (== local main) | **Keep** |
| `origin/dependabot/cargo/src-tauri/tauri-2.11.3` | `2b25114` | PR #19 — tauri 2.11.2→2.11.3 | Open PR, CI green, MERGEABLE/CLEAN | **Merge** (P1) |
| `origin/dependabot/cargo/src-tauri/tauri-build-2.6.3` | `36869df` | PR #20 — tauri-build 2.6.2→2.6.3 | Open PR, CI green, MERGEABLE/CLEAN | **Merge** (P1) |
| `origin/dependabot/cargo/src-tauri/log-0.4.33` | `66af29c` | PR #21 — log 0.4.32→0.4.33 | Open PR, CI green, MERGEABLE/CLEAN | **Merge** (P1) |
| `origin/dependabot/cargo/src-tauri/time-0.3.51` | `4c62016` | PR #22 — time 0.3.47→0.3.51 | Open PR, CI green, MERGEABLE/CLEAN | **Merge** (P1) |
| `origin/dependabot/cargo/src-tauri/uuid-1.23.4` | `8f43bf2` | PR #23 — uuid 1.23.2→1.23.4 | Open PR, CI green, MERGEABLE/CLEAN | **Merge** (P1) |
| `origin/dependabot/npm_and_yarn/types/node-26.0.0` | `72ec253` | PR #18 — @types/node 25.9.4→26.0.0 | Open PR, CI failing (age gate), UNSTABLE | **Rebase** then merge (P2) |

## Worktrees (1)

| Worktree | Branch | Status | Recommendation |
|---|---|---|---|
| `/Users/datamatics/Desktop/PulseUsage` (main repo) | `main` | Clean (2 untracked UX planning docs — forbidden scope) | **Keep** — primary working directory |

## Stashes (0)

None. Clean.

## Remotes (1)

| Remote | URL | Status |
|---|---|---|
| `origin` | `https://github.com/abyssbugg/PulseUsage.git` | Active (fetch + push) — canonical |

## Branch Health

| Metric | Value | Target |
|---|---|---|
| Total local branches | 2 | ≤5 |
| Stale branches (merged, not deleted) | 0 | 0 |
| Diverged branches (ahead of main) | 0 | 0 |
| Unmerged unique work | 0 | 0 |
| Worktrees | 1 | ≤2 per active task |
| Stashes | 0 | 0 |
| Remotes | 1 | 1 (origin only) |

## Post-Merge Cleanup

After merging each dependabot PR:
1. Delete local branch (Dependabot branches are remote-only — no local cleanup needed unless checked out)
2. Delete remote branch (GitHub auto-deletes on merge by default)
3. Sync local main: `git checkout main && git pull --ff-only`
4. Run full test suite: `bun run test && cargo test --manifest-path src-tauri/Cargo.toml`

## Branch Naming Conventions

See [docs/governance/Branching.md](./docs/governance/Branching.md) for the full naming convention. Summary:
- `feat/`, `fix/`, `chore/`, `ci/`, `docs/`, `test/` — standard typed prefixes
- `hardening/`, `audit/`, `restore/`, `brand/`, `setup/`, `research/` — project-specific prefixes
- `dependabot/` — automated (Dependabot only)
- All lowercase, kebab-case, no underscores/spaces
