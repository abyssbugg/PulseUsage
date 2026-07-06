# PulseBar v0.8.0 Program Charter

## Status

Locked for planning. Implementation may not begin until this governance lock is reviewed and approved.

## Purpose

PulseBar v0.8.0 is a separate product program that begins after the v0.7.0 release candidate. Its purpose is to evolve PulseUsage into the PulseBar product identity without risking the stable core platform created by Program 1, Program 2, and Program 2.5.

This charter is documentation only. It does not authorize production code changes, branding changes, repository renames, bundle identifier changes, UI redesign, Ollama integration, or release publication.

## Locked Product Decisions

| Decision | Locked Position | Rationale |
|---|---|---|
| Product display name | PulseBar | PulseBar is the future user-facing product identity. |
| Bundle identifier | Keep `com.abyssbugg.pulseusage` for v0.8.x | Preserves settings, app support data, Launch Services identity, plugin data, logs, and user trust. |
| Application Support directory | Keep existing PulseUsage path for v0.8.x | Avoids data migration risk and support burden. |
| Plugin compatibility | Maintain schema v1 compatibility | Third-party plugins may still depend on v1 inference. The v1 map is a compatibility contract. |
| GitHub repository | Remain `PulseUsage` for v0.8.0 | Avoids release URL, issue, automation, and bookmark churn during product rename. |
| Statistics policy | Evidence-backed only | Usage, quota, forecast, billing, and budget claims require provider evidence or must be omitted. |
| Ollama policy | No fake quota, billing, or usage metrics | Initial Ollama scope is auth, model discovery, diagnostics, and honest capability metadata only. |
| UI strategy | Incremental refinement, not rewrite | Preserve the stable React/Tauri app and improve quality one concern per PR. |
| Branch strategy | One workstream per branch/worktree | Prevents cross-contamination and protects the release branch. |

## Non-Goals

- Do not rename the GitHub repository for v0.8.0.
- Do not change the bundle identifier for v0.8.x.
- Do not move app support data for v0.8.x.
- Do not remove schema v1 compatibility.
- Do not start a full UI rewrite.
- Do not add Ollama quota, billing, usage, progress, or budget metrics without documented public APIs.
- Do not merge multiple independent programs into one PR.
- Do not use `release/v0.7.0` for PulseBar work.

## Program Goals

1. Establish PulseBar as the user-facing product identity.
2. Preserve all existing PulseUsage user data and plugin compatibility.
3. Improve visual and interaction quality without destabilizing the app.
4. Prepare a credible provider expansion path, including Ollama only within evidence-backed constraints.
5. Keep release engineering repeatable and auditable.

## Success Criteria

PulseBar v0.8.0 is successful when:

- Existing PulseUsage users keep settings, plugin state, cache, and logs.
- The app displays PulseBar consistently in user-facing surfaces.
- v1 and v2 plugins continue to run.
- UI changes are incremental, reviewed with screenshots, and accessible by keyboard.
- Ollama, if included, makes only truthful documented claims.
- Release artifacts, screenshots, docs, and changelog align with the chosen identity strategy.

## Program Boundaries

PulseBar v0.8.0 contains multiple workstreams. Each workstream has an independent branch, worktree, PR sequence, risk profile, and Definition of Done. No implementation PR may cross workstream boundaries unless the dependency is explicitly documented in `WorkstreamDependencies.md` and approved before work begins.

## Release Branch Protection

`release/v0.7.0` is frozen except for verified release-candidate bug fixes. PulseBar work starts from `main` after v0.7.0 RC publication and must not be based on the release branch.
