# PulseBar Product Decision Record

## Status

Locked. This record supersedes older conflicting planning text for v0.8.0 execution.

## Decision 1: Product Display Name

Decision: The user-facing product name for the v0.8.0 program is `PulseBar`.

Rationale:
- PulseBar better represents a professional macOS menu-bar product.
- The future identity was previously approved in PulseBar direction docs.
- Display name can change without immediately changing internal compatibility identifiers.

Implementation implication:
- UI strings, About dialog, README, release notes, screenshots, and future artifact naming may move to PulseBar.
- Internal identifiers remain governed by the decisions below.

## Decision 2: Bundle Identifier

Decision: Keep `com.abyssbugg.pulseusage` for v0.8.x.

Rationale:
- Bundle identifier changes orphan macOS application state.
- Existing settings, plugin data, logs, local cache, and launch behavior are tied to the current identifier.
- The cosmetic value of a matching bundle identifier does not justify the migration risk in v0.8.x.

Implementation implication:
- Do not change `src-tauri/tauri.conf.json` `identifier` during v0.8.x.
- If a future release requires a bundle identifier change, it must be its own migration program with backup, rollback, tests, smoke test, and explicit approval.

## Decision 3: Application Support Directory

Decision: Keep the existing Application Support identity/path for v0.8.x.

Rationale:
- Preserves user settings, plugin install directory, plugin data, local cache, and support files.
- Avoids first-launch migration bugs.
- Aligns with the stable bundle identifier decision.

Implementation implication:
- Do not move or rename app support data as part of the visible PulseBar rename.
- Release notes should explain that existing data remains compatible.

## Decision 4: Plugin Compatibility

Decision: Maintain schema v1 compatibility. Schema v2 remains the current manifest format for bundled and new plugins.

Rationale:
- Third-party plugins may omit `hostCapabilities` and still rely on v1 inference.
- The v1 compatibility map is now a platform compatibility layer, not an implementation detail.
- Removing it requires measured adoption and a future approved deprecation cycle.

Implementation implication:
- Do not remove `infer_v1_capabilities`.
- New bundled plugins should use schema v2 with explicit `hostCapabilities`.
- Diagnostics should continue showing `Explicit` or `Inferred (Legacy)` capability source.

## Decision 5: GitHub Repository

Decision: Keep the GitHub repository as `PulseUsage` for v0.8.0.

Rationale:
- Avoids release URL, documentation, issue, workflow, and bookmark churn while the product rename is being introduced.
- Repository rename can be evaluated after a stable PulseBar release.

Implementation implication:
- Do not rename the repository during Program A.
- Release URL constants may remain pointed at `abyssbugg/PulseUsage` for v0.8.0 unless separately approved.

## Decision 6: Statistics Policy

Decision: Statistics, usage, billing, quota, budget, and forecast claims must be evidence-backed.

Rationale:
- Incorrect usage data is worse than omitted data.
- Provider APIs vary widely in reliability and documentation.
- The product should preserve user trust by showing only data it can justify.

Implementation implication:
- If evidence is missing, suppress the metric or mark capability metadata as `undocumented`.
- Do not infer quota, cost, or reset windows from unrelated fields.

## Decision 7: Ollama Policy

Decision: Ollama may not show fake quota, billing, or usage metrics.

Rationale:
- Existing roadmap analysis indicates public Ollama APIs do not provide account usage/billing parity.
- A provider should not pretend to have quota data where none exists.

Implementation implication:
- Initial Ollama scope is limited to auth, connectivity, model discovery, diagnostics, and honest provider capability metadata.
- `OLLAMA_API_KEY` redaction and env allowlisting require explicit tests before use.

## Decision 8: UI Strategy

Decision: UI work is incremental refinement, not a rewrite.

Rationale:
- The current React/Tauri app is functional and stable.
- Full rewrite would combine design, state, and product risks.
- The app serves a small internal workflow and should remain simple.

Implementation implication:
- One visual concern per PR.
- Visual PRs require before/after screenshots.
- Accessibility fixes and settings IA improvements take priority over novelty.

## Decision 9: Branch and Worktree Strategy

Decision: One workstream per branch and worktree.

Rationale:
- Separates product identity, UI, providers, statistics, platform, and release work.
- Protects release branch integrity.
- Makes rollback and review easier.

Implementation implication:
- PulseBar work starts from `main`, not `release/v0.7.0`.
- Each workstream branch is short-lived and deleted after merge.

## Explicit Conflict Resolution

Older documents that imply a v0.7.0 PulseBar rename, a bundle identifier change, or an immediate repository rename are superseded for v0.8.0 by this Product Decision Record.
