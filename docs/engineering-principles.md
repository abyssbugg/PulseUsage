# Engineering Principles

This document is the canonical engineering baseline for PulseBar provider and usage work. It complements [EDR-001](edr/001-pulsebar-direction.md) and [ADR-001](adr/001-pulsebar-v1-provider-usage-baseline.md).

## Trusted Bundled Plugins Only

PulseBar supports bundled plugins shipped with the app. It does not support an untrusted plugin marketplace.

Rules:

- Plugins live in the repository and are reviewed like application code.
- Plugin host APIs must assume plugin code can make mistakes, but not that arbitrary third parties are installing code at runtime.
- New plugin capabilities require tests and documentation.
- Provider additions must include plugin tests, docs, and redaction review.

## Optional Metadata For Backward Compatibility

Provider metadata must be additive.

Rules:

- Missing `capabilities` is valid.
- Missing `usageObservations` is valid.
- Existing display-only providers remain valid.
- Optional metadata must not change provider health unless the specific PR explicitly documents that behavior.
- Invalid optional metadata must fail safely and must not crash the app.

## Display Compatibility

`MetricLine[]` remains the display contract.

Rules:

- Provider cards render from display lines.
- Semantic usage data must not replace display lines.
- Budgets and forecasts must not parse labels from display lines.
- Existing provider visual output should remain stable unless a PR is specifically scoped to change it.

## Security-First Provider Design

Provider integrations handle credentials and account data. Security is part of the contract, not a cleanup task.

Rules:

- Do not log bearer tokens, API keys, refresh tokens, account IDs, organization IDs, emails, or raw provider payloads.
- New auth sources require redaction tests.
- Prefer provider-native credential stores and documented APIs.
- Store normalized usage metrics only when persistence is introduced.
- Do not expose sensitive provider data through diagnostics or local HTTP API.
- Fail loud on unexpected errors; do not silently hide provider failures.

## No Fake Provider Parity

Providers expose different capabilities. PulseBar must be honest about that.

Rules:

- Do not invent quotas when a provider does not expose them.
- Do not scrape account pages to simulate billing or usage APIs.
- Do not present inferred data as exact.
- Use `unsupported`, `undocumented`, `partial`, or equivalent capability states when parity is unavailable.
- For Ollama Cloud specifically, account usage and billing are not supported until public APIs exist.

## Incremental Evolution

Large architecture changes must ship through small, reviewable PRs.

Rules:

- One concern per PR.
- Contracts before providers.
- Providers before persistence when possible.
- Semantic provider upgrades before budgets.
- Persistence before forecasting.
- User-facing analytics only after data quality is good enough.
- Rollback must be clear for each PR.

## Provider-Agnostic Semantics

Semantic usage data must be provider-agnostic without hiding provider-specific truth.

Rules:

- Use stable metric IDs.
- Preserve units explicitly.
- Preserve source and confidence.
- Preserve reset/window semantics when known.
- Do not mix dollars, credits, tokens, requests, and percentages.
- Do not collapse provider-specific uncertainty into precise UI.

## Operational Simplicity

PulseBar is an internal-first tool for a small team.

Rules:

- Prefer simple data models.
- Avoid enterprise policy engines.
- Avoid background services that are not required.
- Keep files focused and small.
- Add tests where behavior can regress.

## Documentation As Contract

Provider and architecture docs are part of the implementation baseline.

Rules:

- ADRs record durable decisions.
- EDRs record engineering direction.
- IMPs record execution roadmaps.
- Provider docs record actual support, not intended support.
- Roadmaps must stay internally consistent with approved PR ordering.
