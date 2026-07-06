# PulseBar PR Sequencing

## Principles

- One logical change per PR.
- No production code in governance-lock PRs.
- Visual PRs require before/after screenshots.
- Provider PRs require provider docs, manifest validation, unit tests, and redaction review when auth is introduced.
- Release branch fixes are separate from PulseBar work.

## Phase 0: Governance and Reconciliation

| PR | Scope | Required Checks |
|---|---|---|
| G1 | PulseBar Governance Lock | Docs-only, no production code. |
| G2 | Reconcile stale control docs after v0.7.0 final | Docs-only, aligns ROADMAP/CURRENT_PHASE/PROJECT_STATUS/RELEASE_PLAN. |

## Program A: PulseBar Migration

| PR | Scope | Notes |
|---|---|---|
| A1 | Identity foundation | Centralize visible product name strategy. No bundle ID change. |
| A2 | User-facing string rename | About, settings, footer, tray labels, README copy. |
| A3 | Artifact naming validation | Confirm DMG/app naming behavior after productName change. |
| A4 | Docs and release notes update | README, changelog, release docs, install docs. |
| A5 | Screenshots and visual assets | Requires before/after screenshots. |

## Program B: Professional UI/UX

| PR | Scope | Notes |
|---|---|---|
| B1 | Accessibility baseline | Restore focus visibility and keyboard paths. |
| B2 | Refresh and error actions | Improve discoverability without layout rewrite. |
| B3 | Empty states | Add clear CTA and provider guidance. |
| B4 | Settings IA | Group settings into clear sections. |
| B5 | Provider card polish | Improve metric hierarchy and readability. |

## Program C: Ollama

| PR | Scope | Notes |
|---|---|---|
| C1 | Ollama API evidence review | Docs-only or research PR; no plugin claims yet. |
| C2 | Auth/redaction prep | Add `OLLAMA_API_KEY` only with redaction tests. |
| C3 | Ollama provider baseline | Auth, connectivity, model discovery, diagnostics only. |
| C4 | Ollama docs/README | Honest capability metadata; no fake quota/billing. |

## Program D: Statistics Engine

| PR | Scope | Notes |
|---|---|---|
| D1 | Statistics data model RFC | Docs-only; define source/confidence/retention. |
| D2 | Snapshot persistence | Only after D1 approval. |
| D3 | Minimal history UI | Only after persisted data proves useful. |
| D4 | Export/debug support | Optional. |

## Program E: Provider Platform

| PR | Scope | Notes |
|---|---|---|
| E1 | Plugin authoring guide | Docs-only. |
| E2 | Capability audit helper research | Research or script prototype if justified. |
| E3 | Validator improvements | No breaking schema v1 compatibility. |
| E4 | Example third-party plugin | Optional, after docs settle. |

## Program F: Release Engineering

| PR | Scope | Notes |
|---|---|---|
| F1 | v0.8.0 readiness template | Docs-only. |
| F2 | PulseBar artifact checklist | Validate productName/DMG naming. |
| F3 | Checksum generation | Optional release convenience. |
| F4 | Signing/notarization decision record | Conditional, not required unless distribution scope changes. |

## Merge Policy

- Merge only after CI passes.
- Use merge commits to preserve program history.
- Delete topic branches/worktrees after merge.
- Keep Program A merged before Program F artifact naming changes.
- Do not merge any PR that changes user data paths without explicit migration tests.
