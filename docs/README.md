# PulseBar Engineering Documentation Index

This index lists the canonical planning and architecture documents for PulseBar. Application code is not part of this documentation consolidation phase.

## EDRs

Engineering Decision Records define product and engineering direction.

| Document | Status | Purpose |
|----------|--------|---------|
| [EDR-001: PulseBar Product & Architecture Direction](edr/001-pulsebar-direction.md) | Approved | Overall product identity, technical direction, anti-goals, v1.0 direction. |

## ADRs

Architecture Decision Records define durable architecture decisions.

| Document | Status | Purpose |
|----------|--------|---------|
| [ADR-001: PulseBar v1 Provider and Usage Baseline](adr/001-pulsebar-v1-provider-usage-baseline.md) | Approved | Canonical provider capability, Ollama, usage observation, persistence, provider upgrade, budget, and forecast baseline. |

## Implementation Roadmaps

Implementation plans define PR sequencing and execution gates.

| Document | Status | Purpose |
|----------|--------|---------|
| [IMP-001: PulseBar Implementation Master Plan](imp/001-implementation-master-plan.md) | Approved | Broad PulseBar v1 implementation and release plan. |
| [IMP-002: Implementation Readiness Review](imp/002-irr-readiness-review.md) | Approved | Readiness review and amendments for IMP-001. |
| [IMP-004: Provider Usage Implementation Roadmap](imp/004-provider-usage-implementation-roadmap.md) | Approved baseline | Official PR-1 through PR-7 implementation order for provider and usage work. |

## Architecture Roadmaps

Architecture roadmaps define technical sequencing and architectural dependencies.

| Document | Status | Purpose |
|----------|--------|---------|
| [IMP-003: Provider Usage Architecture Roadmap](imp/003-provider-usage-architecture-roadmap.md) | Approved baseline | Architecture layering for provider capabilities, Ollama, usage observations, persistence, provider upgrades, budgets, and forecasting. |
| [App State Architecture](app-state-architecture.md) | Current | Frontend source-of-truth and derived state guardrails. |
| [Provider Health](provider-health.md) | Current | Provider health and metadata expectations. |

## Engineering Principles

| Document | Status | Purpose |
|----------|--------|---------|
| [Engineering Principles](engineering-principles.md) | Approved baseline | Trusted bundled plugins, optional metadata, display compatibility, security-first provider design, no fake parity, incremental evolution. |

## Migration Plans

| Document | Status | Purpose |
|----------|--------|---------|
| [EDR-001 Migration Section](edr/001-pulsebar-direction.md#migration-pulseusage--pulsebar) | Approved | Product identity and data-path migration direction. |
| [IMP-001 Migration Section](imp/001-implementation-master-plan.md#migration-v070) | Approved | v0.7.0 migration execution notes. |

## Plugin And Provider References

| Document | Purpose |
|----------|---------|
| [Plugin Schema](plugins/schema.md) | Plugin manifest, output lines, runtime lifecycle. |
| [Host API Reference](plugins/api.md) | Plugin host APIs exposed through `ctx.host`. |
| [Local HTTP API](local-http-api.md) | Local loopback API behavior. |
| [Provider Docs](providers/) | Provider-specific support and setup documentation. |

## Official Provider Usage PR Order

| PR | Theme |
|----|-------|
| PR-1 | Provider capability contracts |
| PR-2 | Ollama provider |
| PR-3 | Usage observations |
| PR-4 | SQLite persistence |
| PR-5 | Provider upgrades |
| PR-6 | Budgets and alerts |
| PR-7 | Forecasting and history |
