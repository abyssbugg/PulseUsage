# Repository Governance

This document defines the canonical engineering standards and workflows for the PulseUsage repository. All contributors must follow these standards to maintain a consistent, professional, enterprise-quality codebase.

## Scope

These governance documents apply to:
- Branch creation and naming
- Commit messages and history
- Pull request workflow
- Release process
- Code review
- Architecture review
- Repository maintenance
- Roadmap management

## Governance Documents

| Document | Purpose |
|---|---|
| [Repository.md](./Repository.md) | This document — governance overview and repository standards |
| [Branching.md](./Branching.md) | Branch naming, lifetime, and cleanup conventions |
| [PullRequestWorkflow.md](./PullRequestWorkflow.md) | PR creation, review, merge, and cleanup process |
| [ReleaseProcess.md](./ReleaseProcess.md) | Version bumping, tagging, artifact building, and publishing |
| [CodeReview.md](./CodeReview.md) | Review standards, automated reviewers, and approval gates |
| [ArchitectureReview.md](./ArchitectureReview.md) | Architecture decision records and implementation planning |
| [RepositoryMaintenance.md](./RepositoryMaintenance.md) | Ongoing hygiene — branches, worktrees, stashes, remotes, dependencies |
| [RoadmapManagement.md](./RoadmapManagement.md) | Roadmap alignment, phase tracking, and drift detection |

## Repository Identity

- **Name:** PulseUsage
- **Canonical remote:** `https://github.com/abyssbugg/PulseUsage.git` (origin)
- **Distribution:** Direct download (unsigned DMG, manual release)
- **Users:** 2-5 internal users
- **Platform:** macOS (arm64 primary)
- **Stack:** Tauri 2.x + Rust + React 19 + Vite + Vitest + rquickjs plugin engine

## Core Principles

1. **Simplicity first.** This app is used by 2-5 people internally. Do not over-engineer. Handle only important cases. No enterprise fallbacks. (AGENTS.md)
2. **Fail loud.** Expected issues use explicit result types. Unexpected issues throw + log + toast. Never add silent fallbacks. (AGENTS.md)
3. **Conventional Commits.** All commit messages follow the Conventional Commits specification with scopes. (Verified in main history)
4. **Atomic commits.** One logical change per commit. Review-feedback fixes are separate commits, not squashed, to preserve review history.
5. **Merge commits.** PRs merge via GitHub merge commits (not squash or rebase) to preserve full history.
6. **Verified before deletion.** No branch, worktree, stash, or remote is deleted without ancestry verification (`git merge-base --is-ancestor`).
7. **No force pushes.** History is immutable. Mistakes are fixed with revert commits, not rewrites.
8. **No secrets in commits.** Test fixtures with fake secrets must use obvious placeholders (`key_value_1234567890`, `ghp_abcdefghijklmnopqrstuvwxyz`).

## File Size Limits

- Keep source files under ~400 LOC. Split or refactor as needed. (AGENTS.md)
- The `validate-provider-metadata.mjs` split (v0.6.28) is the precedent: 641 LOC → 347 + 309.

## See Also

- [AGENTS.md](../../AGENTS.md) — project-specific instructions and guardrails
- [CONTRIBUTING.md](../../CONTRIBUTING.md) — contributor onboarding (if present)
- [docs/release-readiness/](../release-readiness/) — per-release readiness reports
- [docs/edr/](../edr/) — engineering decision records
- [docs/imp/](../imp/) — implementation master plans