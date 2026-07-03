# IMP-004: Provider Usage Implementation Roadmap

| Field | Value |
|-------|-------|
| **Status** | Approved baseline |
| **Date** | 2026-07-02 |
| **ADR** | [ADR-001](../adr/001-pulsebar-v1-provider-usage-baseline.md) |
| **Architecture Roadmap** | [IMP-003](003-provider-usage-architecture-roadmap.md) |

## Executive Summary

This roadmap converts the approved PulseBar provider and usage architecture into seven independently reviewable pull requests. The order prioritizes optional contracts and display compatibility before persistence, budgets, and analytics.

## Official PR Order

| PR | Title | Dependency | Review Complexity | Merge Risk |
|----|-------|------------|-------------------|------------|
| PR-1 | Provider capability contracts | None | Low | Low |
| PR-2 | Ollama provider | PR-1 preferred | Medium | Medium |
| PR-3 | Usage observations | PR-1 | Medium | Medium |
| PR-4 | SQLite persistence | PR-3 and demonstrated observation need | Medium-high | Medium-high |
| PR-5 | Provider upgrades | PR-3, PR-4 preferred | Medium per provider | Medium per provider |
| PR-6 | Budgets and alerts | PR-4, PR-5 | High | High |
| PR-7 | Forecasting and history | PR-4, PR-6 preferred | Medium-high | Medium-high |

## PR-1: Provider Capability Contracts

### Objective

Introduce minimal optional provider capability metadata with runtime validation and no behavior changes.

### Scope

- Optional provider capability contract.
- Shared TypeScript/Rust semantic types where needed.
- Runtime validation of capability statuses.
- Documentation of allowed capability values.
- No UI behavior changes.
- No usage observations.

### Files Expected To Change

- `src-tauri/src/plugin_engine/runtime.rs`
- Rust plugin output DTO files if separate from runtime.
- Frontend shared plugin/provider type files.
- `docs/plugins/schema.md`
- `docs/plugins/api.md` if host-facing examples are needed.
- Runtime and frontend fixture tests.

### Dependencies

None.

### Risks

- Optional metadata accidentally becomes required.
- Legacy providers fail parsing.
- Capability values become a place to leak sensitive account data.

### Validation Requirements

- Missing capabilities parse successfully.
- Valid capabilities parse successfully.
- Invalid capability statuses fail safely.
- Existing bundled provider fixtures remain unchanged.
- Existing provider cards render unchanged.

### Estimated Implementation Size

Small.

### Exit Criteria

- No observable behavior changes.
- Existing providers do not need edits.
- Capability metadata is optional and non-sensitive.

### Rollback Strategy

Revert contract, parser, and docs. No data migration.

## PR-2: Ollama Provider

### Objective

Add Ollama Cloud as the first major new provider under the baseline.

### Scope

- New bundled Ollama plugin.
- Authentication through approved API key sources.
- Model discovery through documented Ollama APIs.
- Diagnostics for common auth/network/API states.
- Provider documentation.
- Tests for auth, model discovery, diagnostics, and redaction.
- Explicit usage/billing unsupported messaging.

### Files Expected To Change

- `plugins/ollama/plugin.json`
- `plugins/ollama/plugin.js`
- `plugins/ollama/icon.svg`
- `plugins/ollama/plugin.test.js`
- `docs/providers/ollama.md`
- `README.md`
- `src-tauri/src/plugin_engine/host_api.rs` if `OLLAMA_API_KEY` env access is added.
- Provider validator fixtures if needed.

### Dependencies

PR-1 preferred. PR-3 not required.

### Risks

- API key leakage.
- False usage or billing parity claims.
- Undocumented endpoint use.
- Ollama API response shape drift.

### Validation Requirements

- Missing key returns setup guidance.
- Valid key sends bearer auth to documented endpoint.
- `401`, `403`, `429`, `502`, timeout, empty model list, and malformed JSON are covered.
- No quota progress lines are emitted for Ollama account usage.
- Secret redaction tests cover `OLLAMA_API_KEY` and bearer headers.
- README supported providers updated.

### Estimated Implementation Size

Medium.

### Exit Criteria

- Ollama provider works independently.
- Existing providers remain unchanged.
- Ollama docs state no public account usage or billing API support.

### Rollback Strategy

Remove Ollama plugin, docs, README entry, and env whitelist if added.

## PR-3: Usage Observations

### Objective

Introduce optional semantic usage observations as a separate contract from display lines.

### Scope

- Optional `usageObservations` contract.
- Runtime validation.
- Shared semantic types.
- Synthetic fixtures.
- Documentation with examples.
- No SQLite persistence.
- No budgets.
- No alerts.
- No forecast/history UI.

### Files Expected To Change

- `src-tauri/src/plugin_engine/runtime.rs`
- Rust plugin output DTO files.
- Frontend shared usage type files.
- `docs/plugins/schema.md`
- Runtime and fixture tests.

### Dependencies

PR-1.

### Risks

- Invalid observations break display lines.
- Semantics accidentally inferred from display labels.
- Observation dimensions leak sensitive values.

### Validation Requirements

- Missing observations accepted.
- Valid observations accepted.
- Invalid values fail safely.
- Existing providers still render using `lines` only.
- Local HTTP API exposure is unchanged unless separately documented.

### Estimated Implementation Size

Medium.

### Exit Criteria

- Observations are optional.
- Display compatibility is preserved.
- No persistence or user-facing budget behavior exists yet.

### Rollback Strategy

Remove observation parser/types/docs. Existing display-only providers continue to work.

## PR-4: SQLite Persistence

### Objective

Persist normalized usage observations after semantic observation need is proven.

### Scope

- App-owned SQLite schema.
- Migrations.
- Observation ingestion.
- Deduplication.
- Basic reset/delta handling only if needed for durable semantics.
- No budgets.
- No alerts.
- No forecasting/history UI.

### Files Expected To Change

- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`
- New `src-tauri/src/usage/*` modules or equivalent.
- Probe result ingestion path.
- Backend tests.

### Dependencies

PR-3 and at least one synthetic or provider observation source worth persisting.

### Risks

- Persistence failure affects live provider display.
- Migration errors.
- Sensitive data stored in DB.
- Duplicate probes inflate history.

### Validation Requirements

- Empty DB migration succeeds.
- Migration is idempotent.
- Synthetic observations persist.
- Duplicate observations are skipped.
- Store failure logs loudly but provider display still works.
- No raw provider payloads or credentials stored.

### Estimated Implementation Size

Medium-large.

### Exit Criteria

- Persistence is isolated from provider rendering.
- Existing display-only providers produce no rows and no errors.
- Rollback can leave the DB file unused.

### Rollback Strategy

Revert store and ingestion code. Leave DB file inert.

## PR-5: Provider Upgrades

### Objective

Upgrade existing providers to emit semantic observations and capabilities before budgets are introduced.

### Scope

- Add observations to selected providers.
- Preserve existing display lines.
- Add provider-specific observation tests.
- Update provider docs with semantic metric support.
- Split by provider if review size grows.

### Recommended Sub-PRs

| Sub-PR | Provider |
|--------|----------|
| PR-5a | Claude |
| PR-5b | Codex |
| PR-5c | Antigravity |
| PR-5d | Future provider template |

### Files Expected To Change

- `plugins/claude/*`
- `plugins/codex/*`
- `plugins/antigravity/*`
- `docs/providers/claude.md`
- `docs/providers/codex.md`
- `docs/providers/antigravity.md`
- `README.md`

### Dependencies

PR-3 required. PR-4 preferred when persisted validation is useful.

### Risks

- Existing provider UI changes.
- Units or windows are mapped incorrectly.
- Sensitive dimensions leak account data.
- Review scope becomes too large.

### Validation Requirements

- Existing display output snapshots remain stable.
- Observation IDs are stable.
- Units do not mix dollars, credits, percent, tokens, or requests.
- Reset windows are mapped accurately where known.
- Redaction audit covers newly exposed fields.

### Estimated Implementation Size

Medium per provider.

### Exit Criteria

- Each provider can be reverted independently.
- Budgets have real provider metrics to target later.
- Display compatibility is preserved.

### Rollback Strategy

Revert one provider's semantic observation changes. Provider falls back to display-only output.

## PR-6: Budgets And Alerts

### Objective

Add user-defined budgets and warning-only alerts using persisted semantic observations from upgraded providers.

### Scope

- Hourly, daily, weekly, and monthly budget policies.
- Rolling windows where supported by observations.
- Soft warning thresholds.
- Hard warning thresholds as warnings only.
- Alert deduplication.
- Menu bar notification summary.
- No provider-side enforcement.

### Files Expected To Change

- Usage store/evaluator modules.
- Tauri command registration.
- Settings UI.
- App hooks for usage state.
- Tray/menu summary code.
- Backend and frontend tests.

### Dependencies

PR-4 and PR-5.

### Risks

- Alert spam.
- Hard warnings misunderstood as enforcement.
- Window math errors.
- Budget creation for providers without semantic metrics.

### Validation Requirements

- Hour/day/week/month tests.
- Fixed, rolling, and calendar window tests.
- DST and month-boundary tests.
- Alert dedupe and escalation tests.
- Providers without observations cannot create budgets.

### Estimated Implementation Size

Large.

### Exit Criteria

- Budgets use observation metric IDs only.
- Alerts are deduplicated.
- Hard warnings are clearly non-enforcing.

### Rollback Strategy

Disable budget UI and evaluator. Leave observations and DB tables unused.

## PR-7: Forecasting And History

### Objective

Add historical usage, trend analysis, and confidence-gated forecasts.

### Scope

- Historical usage queries.
- Hourly/daily rollups.
- Trend analysis.
- Forecasted exhaustion estimates.
- Confidence and stale-data suppression.
- History and forecast UI.

### Files Expected To Change

- Usage analytics modules.
- Tauri commands.
- Frontend usage history and forecast components.
- Provider detail UI.
- Backend and frontend tests.

### Dependencies

PR-4 required. PR-6 preferred.

### Risks

- False precision.
- Poor data quality.
- Date/window bugs.
- Performance issues on long history.

### Validation Requirements

- Forecast suppressed for no limit, no deltas, stale data, weak confidence, and early windows.
- Trend math deterministic.
- Rollups aggregate correctly.
- Empty and unavailable states are explicit.

### Estimated Implementation Size

Medium-large.

### Exit Criteria

- No forecast appears when data quality is insufficient.
- History uses normalized observations only.
- Existing provider cards remain compatible.

### Rollback Strategy

Hide history and forecast UI. Keep persisted observations.

## Global Validation Gates

Every PR must verify:

- Existing bundled providers still run.
- Display line compatibility remains intact.
- Optional metadata remains optional.
- No secrets are emitted in plugin output, cache, logs, diagnostics, or local HTTP API.
- Documentation matches behavior.
- Rollback is possible by reverting the PR, except for intentionally inert DB files after PR-4.

## Authorized Next Step

After this documentation baseline is accepted, begin PR-1 only. Do not start Ollama provider implementation until PR-1 is merged or explicitly waived.
