# ADR-001: PulseBar v1 Provider and Usage Baseline

| Field | Value |
|-------|-------|
| **Status** | Approved |
| **Date** | 2026-07-02 |
| **Baseline** | [EDR-001](../edr/001-pulsebar-direction.md) |
| **Roadmaps** | [Architecture Roadmap](../imp/003-provider-usage-architecture-roadmap.md), [Implementation Roadmap](../imp/004-provider-usage-implementation-roadmap.md) |

## Context

PulseBar is a local-first macOS menu-bar app for AI provider usage visibility. The existing architecture uses bundled JavaScript plugins executed in isolated QuickJS runtimes, with Rust/Tauri host APIs and React UI rendering provider-owned `MetricLine` output.

The next product evolution adds first-class Ollama Cloud support and prepares for provider-agnostic usage limits. The implementation must stay incremental, reviewable, and safe for existing providers. Current providers must not be forced to emit new metadata or semantic usage data before they are upgraded.

## Decision

Adopt the PulseBar v1 provider and usage baseline below:

1. Provider capability metadata is optional.
2. Usage observations are optional and separate from display lines.
3. Display compatibility is mandatory for existing providers.
4. Ollama Cloud is the first major new provider under this baseline.
5. SQLite persistence is deferred until semantic observations exist and require durable storage.
6. Existing provider semantic upgrades happen before user budgets and alerts.
7. Budgets, alerts, forecasting, and history are later phases, not part of the initial contracts.
8. No provider may claim unsupported account usage, billing, quota, or organization parity.

## Official PR Ordering

| PR | Theme | Status |
|----|-------|--------|
| PR-1 | Provider capability contracts | Complete via PR #28 |
| PR-2 | Ollama provider | Planned |
| PR-3 | Usage observations | Planned |
| PR-4 | SQLite persistence | Deferred until PR-3 proves semantic observation need |
| PR-5 | Provider upgrades | Planned before budgets |
| PR-6 | Budgets and alerts | Deferred |
| PR-7 | Forecasting and history | Deferred |

## Rationale

### Optional Capability Contracts

Capabilities describe what a provider can support, cannot support, or has not documented. They must be optional because existing bundled providers already work through display lines. Requiring capabilities in the first PR would create broad migration risk without user value.

### Ollama Provider Before Usage Infrastructure

Ollama Cloud can provide immediate value through authentication status, cloud model discovery, connectivity diagnostics, and honest capability reporting. Public Ollama documentation does not expose account usage or billing APIs, so the provider must not fake quota support.

### Optional Usage Observations

Usage observations are the future semantic source for persistence, budgets, alerts, and trends. They must not replace `MetricLine` display output. A provider can remain display-only indefinitely.

### Provider Upgrades Before Budgets

Budgets require real semantic metrics. Upgrading Claude, Codex, Antigravity, and later providers before budget UI reduces the risk of building budget behavior around synthetic or unrepresentative data.

### Deferred SQLite

Persistent storage introduces migration, locking, corruption, privacy, and rollback concerns. It should begin only after the semantic observation contract exists and at least one provider or synthetic fixture produces durable observations worth storing.

### Deferred Budgets, Alerts, Forecasting, And Trends

These features are user-facing and can mislead users if based on weak data. They must wait until provider observations and persistence are stable.

## Consequences

### Positive

- Existing providers remain safe.
- New metadata can be added without forcing immediate migrations.
- Ollama can ship without false feature parity.
- Semantic usage work gets a clean contract before persistence and UI.
- Budgets and forecasts will be based on provider-upgraded data, not display labels.

### Negative

- Limits and forecasts arrive later than the first Ollama provider PR.
- Some providers will temporarily remain display-only.
- SQLite architecture will be designed later, after observation semantics are validated.

### Neutral

- `MetricLine` remains the display contract.
- Local HTTP API behavior should not change until a later PR explicitly documents any new fields.
- Existing cache behavior remains unchanged until persistence work begins.

## Non-Goals

- No plugin marketplace.
- No untrusted third-party plugin execution.
- No provider scraping to simulate official APIs.
- No PulseBar proxy or gateway for enforcing hard limits.
- No rewrite from Tauri/Rust/React/QuickJS.
- No project rename work in this provider/usage baseline.

## Security Requirements

- Provider metadata must not contain raw credentials, bearer tokens, emails, account IDs, organization IDs, or raw provider payloads.
- Provider plugins must redact secrets in logs, diagnostics, cacheable output, and local HTTP API output.
- New auth sources require `host_api.rs` redaction tests.
- Ollama support must use documented APIs only.
- Usage observations must store normalized metrics only, not raw request/response bodies.

## Compatibility Requirements

- Missing `capabilities` must be valid.
- Missing `usageObservations` must be valid.
- Existing `lines` rendering must remain unchanged.
- Existing bundled providers must not require code changes for PR-1 or PR-3.
- Invalid optional metadata must not crash the app.

## Exit Criteria For Beginning PR-1

PR-1 may begin when:

1. This ADR is approved, either before PR-1 or as part of the same PR.
2. The implementation roadmap reflects the official PR ordering.
3. Engineering principles are documented.
4. Planning docs consistently state optional metadata and display compatibility.
5. PR-1 scope is limited to provider capability contracts and validation with no behavior changes.

## References

- [EDR-001: PulseBar Product & Architecture Direction](../edr/001-pulsebar-direction.md)
- [IMP-003: Provider Usage Architecture Roadmap](../imp/003-provider-usage-architecture-roadmap.md)
- [IMP-004: Provider Usage Implementation Roadmap](../imp/004-provider-usage-implementation-roadmap.md)
- [Engineering Principles](../engineering-principles.md)
