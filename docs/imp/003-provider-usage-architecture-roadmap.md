# IMP-003: Provider Usage Architecture Roadmap

| Field | Value |
|-------|-------|
| **Status** | Approved baseline |
| **Date** | 2026-07-02 |
| **ADR** | [ADR-001](../adr/001-pulsebar-v1-provider-usage-baseline.md) |
| **Implementation Roadmap** | [IMP-004](004-provider-usage-implementation-roadmap.md) |

## Executive Summary

PulseBar will evolve provider and usage architecture through optional, backward-compatible contracts. The architecture deliberately separates display lines from semantic usage. Ollama Cloud ships first as an honest provider integration, while budgets, alerts, forecasting, and historical trends are deferred until observations and provider upgrades provide reliable data.

## Architecture Baseline

| Area | Baseline |
|------|----------|
| Provider runtime | Bundled JS plugins in isolated QuickJS runtimes. |
| Display contract | `MetricLine[]` remains authoritative for UI display. |
| Provider metadata | Optional capability metadata only. |
| Usage semantics | Optional `usageObservations`, introduced separately from capabilities. |
| Persistence | Deferred SQLite store after observations exist. |
| Provider migration | Existing providers migrate incrementally after observation contract. |
| Limits | Deferred until upgraded providers emit stable semantic metrics. |
| Forecasting/history | Deferred until persisted observations and budgets exist. |

## Architecture Principles

1. Keep existing provider display behavior stable.
2. Add metadata as optional extension fields.
3. Do not infer business semantics from display labels.
4. Do not claim unsupported provider parity.
5. Prefer provider-documented APIs over reverse-engineered endpoints.
6. Isolate persistence from live provider display.
7. Defer user-facing limits until data semantics are trustworthy.

## Capability Metadata Architecture

Provider capabilities describe supported, unsupported, partial, planned, or undocumented provider surfaces. They are not usage data.

Examples:

- Account usage API support
- Billing API support
- Model discovery support
- Rate-limit detail support
- Organization support

Capability metadata must be optional and non-sensitive.

## Ollama Provider Architecture

Ollama is the first major new provider under this baseline.

Initial Ollama scope:

- API key authentication through approved host APIs.
- Cloud model discovery through documented Ollama APIs.
- Diagnostics for missing key, unauthorized, rate-limited, unavailable, timeout, and malformed response states.
- Documentation that account usage and billing APIs are not publicly documented.
- Capability metadata indicating supported and undocumented surfaces.

Out of scope for initial Ollama provider:

- Account usage polling.
- Billing polling.
- Provider quota progress bars.
- Organization/team API support.
- Screen scraping or undocumented account endpoints.

## Usage Observations Architecture

Usage observations are normalized provider-emitted semantic records. They are introduced after capability metadata.

Observation examples:

- Provider usage percentage with reset window.
- Token totals from local usage tooling.
- Credit balance or spend if a provider exposes it through documented sources.

Usage observations must include enough metadata for later persistence and budgets:

- Stable ID.
- Provider ID.
- Metric kind and unit.
- Used value.
- Optional limit and remaining value.
- Optional window.
- Source classification.
- Confidence classification.

Usage observations must not contain raw provider payloads, credentials, or account identifiers.

## Persistence Architecture

SQLite persistence is deferred until semantic observations exist. The store should persist normalized observations only.

Persistence responsibilities when introduced:

- Schema migrations.
- Observation ingestion.
- Deduplication.
- Reset and delta handling.
- Isolation from live provider display.
- No raw provider response storage.

Persistence must not be required for provider cards to render.

## Provider Upgrade Architecture

Provider upgrades move before budgets. Each provider upgrade should add observations and capabilities without changing existing display lines.

Initial upgrade targets:

1. Claude
2. Codex
3. Antigravity
4. Future providers as they become stable

Provider upgrades should be split by provider when review size grows.

## Budgets And Alerts Architecture

Budgets and alerts are deferred until semantic provider upgrades exist. Budget logic must use observation metric IDs, not display labels.

Planned support:

- Hourly limits.
- Daily limits.
- Weekly limits.
- Monthly limits.
- Rolling windows where observations support them.
- Soft warnings.
- Hard warnings as warning-only, not enforcement.
- Menu bar warning summary.

## Forecasting And History Architecture

Forecasting and history are deferred until persistence and provider upgrades provide enough data.

Forecasts must be suppressed when data quality is weak. History views must use normalized observations or rollups only.

## Official Architecture Sequence

| PR | Architecture Layer | Reason For Position |
|----|--------------------|---------------------|
| PR-1 | Provider capabilities | Small optional contract; no behavior changes. |
| PR-2 | Ollama provider | Immediate provider value with honest capability reporting. |
| PR-3 | Usage observations | Semantic contract after capability baseline. |
| PR-4 | SQLite persistence | Store observations only after semantic data exists. |
| PR-5 | Provider upgrades | Real provider data before budgets. |
| PR-6 | Budgets and alerts | User-facing limits after upgraded providers. |
| PR-7 | Forecasting and history | Analytics after persisted usage and budgets. |

## Architecture Exit Criteria

The architecture baseline is accepted when:

- Existing providers remain display-compatible.
- Optional metadata is documented.
- Ollama scope does not include unsupported usage or billing claims.
- SQLite is explicitly deferred.
- Provider upgrades occur before budgets and forecasting.
- Implementation roadmap matches PR-1 through PR-7 ordering.
