# EDR-001: PulseBar Product & Architecture Direction

| Field | Value |
|-------|-------|
| **Status** | Approved |
| **Date** | 2026-07-02 |
| **Supersedes** | Ad hoc implementation decisions |

## Executive Summary

PulseBar is the long-term identity for the PulseUsage codebase: a **macOS menu-bar app** aggregating AI coding subscription usage via **Tauri/Rust + QuickJS plugins + React**.

**Decision:** Do not rewrite to Swift. Do not chase OpenUsage/CodexBar feature parity. Optimize for **plugin velocity, provider breadth, and operational maturity** by v1.0.

## Product Vision

### What is PulseBar?

Local-first, privacy-preserving menu-bar dashboard. Reads usage from provider-native credentials (keychain, files, APIs). Exposes loopback HTTP API at `127.0.0.1:6736`.

### Problems solved

- Fragmented provider dashboards → one panel
- Reset window mental math → `resetsAt`, pace indicators
- Slow provider addition → JS plugins
- Automation → `/v1/usage` API

### Target user

Primary: internal engineering team (2–5 people). Secondary: technical power users valuing breadth over native polish.

### Core capabilities

1. Multi-provider dashboard (progress, text, badge, barChart)
2. Menu-bar tray (provider icon, donut, multi-bar via `primaryOrder`)
3. Configurable refresh (5/15/30/60 min)
4. Global shortcut, proxy support, bundled JS plugins
5. Local HTTP API on port 6736

### Anti-goals

| Anti-goal | Reason |
|-----------|--------|
| Swift rewrite | Destroys plugin velocity |
| 56-provider monolith | Unmaintainable |
| Plugin marketplace | Security undefined |
| Cloud sync | Privacy violation |
| CLI-first platform | CodexBar owns this |
| WidgetKit | Zero internal demand |
| Cross-platform pretense | macOS-native integrations are core |

### Differentiation

**vs OpenUsage:** Plugin breadth (18+ vs 9), configurable refresh, JS extensibility. Adopt operational patterns (signing, SWR caching), not Swift code.

**vs CodexBar:** Plugin contributor model vs compile-time Swift. HTTP API vs CLI. Adopt descriptor-like manifest metadata, not 56-provider scope.

## Engineering Principles

1. Simplicity before complexity
2. Evidence-driven engineering
3. Stability over feature count
4. Plugin-first architecture
5. Provider neutrality (host knows MetricLine, not provider APIs)
6. Security by default (redaction, no credential storage)
7. Testability (90% coverage gate, per-plugin tests)
8. Maintainability (files <~400 LOC)
9. Minimal regressions (test per bug fix)
10. Incremental evolution (no big-bang rewrites)
11. Fail loud (no silent fallbacks)
12. Traceable decisions (EDR/ADR/issue links)
13. Operational honesty (document macOS-only)
14. Upstream governance (≤2h triage per release)

## Architectural Decisions

| ID | Area | Decision |
|----|------|----------|
| AD-001 | Runtime | Tauri 2 + Rust + React + QuickJS. macOS-only. |
| AD-002 | Plugins | Bundled JS `probe(ctx)`, fresh QuickJS per probe, 4 workers. |
| AD-003 | Providers | Provider = plugin. Auth in JS via `ctx.host.*`. |
| AD-004 | Diagnostics | Tray log levels, redaction. No telemetry until post-v1.0. |
| AD-005 | Settings | Rust SSOT; frontend via IPC; HTTP API reads same store. |
| AD-006 | Frontend state | Zustand source stores + derived hooks (keep). |
| AD-007 | Release | Tag-driven CI, sign, notarize, universal binary. |
| AD-008 | Updates | Tauri 2 updater (not Sparkle). |
| AD-009 | Testing | Vitest 90% + plugin tests + `cargo test --test-threads=1`. |
| AD-010 | Repo layout | `plugins/` → bundled; `docs/edr/`, `docs/imp/` for decisions. |

## Adoption Strategy (patterns only, no code import)

### OpenUsage — Adopt

Session-aware SWR caching, signed/notarized releases, provider logic fixes (as JS ports), dynamic pricing table, Reset All Customization.

### OpenUsage — Reject

Swift runtime, Sparkle, fixed 5-min refresh, Liquid Glass, PostHog (v1.0).

### CodexBar — Adapt

Descriptor metadata in `plugin.json`, optional status polling, API key settings UI, pace semantics.

### CodexBar — Reject

56 providers, per-provider status items, WidgetKit, CLI, WebView scrape host API.

## Migration (PulseUsage → PulseBar)

| Asset | Current | Target |
|-------|---------|--------|
| Product | PulseUsage | PulseBar |
| Bundle ID | `com.abyssbugg.pulseusage` | `com.abyssbugg.pulsebar` |
| Proxy config | `~/.pulseusage/` | `~/.pulsebar/` |
| HTTP API port | 6736 | 6736 (never change) |

Bundle ID change = manual upgrade from 0.6.x. Settings migrate on first launch. Keychain: no migration (provider-native services).

## Roadmap Phases

| Phase | Version | Theme |
|-------|---------|-------|
| 0 | — | Housekeeping + planning docs |
| 1 | v0.6.28 | Provider patch (PulseUsage identity) |
| 2 | v0.7.0 | Rename to PulseBar |
| 3 | v0.7.1–0.7.2 | Settings SSOT, host_api split |
| 4 | v0.7.3–0.7.5 | Signing, updater, SWR cache |
| 5 | v0.8.x | UX polish |
| 6 | v1.0.0 | Stable release gate |

## v1.0 Success Criteria

- 18 plugins probe successfully
- Settings SSOT, no duplicated defaults
- `host_api.rs` split (<400 LOC per module)
- Signed + notarized universal binary
- Tauri auto-update across 3 releases
- API key settings UI, Reset All Customization
- Migration + updater E2E tests

## References

- Implementation plan: [IMP-001](../imp/001-implementation-master-plan.md)
- Readiness review: [IRR-001](../imp/002-irr-readiness-review.md)