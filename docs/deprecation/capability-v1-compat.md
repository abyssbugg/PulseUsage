# v1 Capability Compatibility — Deprecation Policy

## Status

**Active — part of the platform compatibility contract.** Not deprecated.

## Purpose

The v1 compatibility map (`infer_v1_capabilities` in `src-tauri/src/plugin_engine/capability.rs`) grants host API capabilities to plugins based on their plugin ID. It exists for plugins that predate schema v2's explicit `hostCapabilities` declaration.

## Removal Criteria

The v1 compatibility layer can be removed **only** when **all** of the following are true:

1. **Bundled providers migrated.** All 18 bundled providers declare explicit `hostCapabilities`. ✅ Complete (Program 2 PR-3, PR-4).
2. **Third-party adoption.** At least 90% of known third-party plugins in the wild have migrated to schema v2. Adoption must be measured via telemetry or a plugin registry. No such measurement exists today.
3. **Deprecation notice shipped.** A release notes the v1 map as deprecated and warns third-party authors to migrate. The structured runtime warning (Program 2.5 Task 2) already surfaces this at runtime; a release-note-level notice is still needed.
4. **Migration tooling available.** A tool that auto-generates an initial `hostCapabilities` block from static analysis of `ctx.host.*` usage (Program 2.5 Task 4, research-only). This reduces the migration cost for third-party authors.
5. **Minimum one full release cycle** between the deprecation notice and removal, so authors have time to migrate.

## Earliest Removal Version

**Not before v0.9.0.** With the current cadence (v0.6.x → v0.7.x → v0.8.x → v0.9.x), and assuming:
- v0.7.0 ships the deprecation notice (structured runtime warning already in place).
- v0.8.0 is the observation cycle (telemetry on third-party adoption).
- v0.9.0 is the earliest removal, contingent on ≥90% adoption.

If adoption metrics fall short, removal is deferred. The map has near-zero runtime cost (a match statement on a string), so keeping it imposes no measurable burden.

## Migration Requirements for Third-Party Authors

1. Audit `ctx.host.*` usage via grep (see [Capabilities Reference](../plugins/capabilities.md#migration-path-v1--v2)).
2. Add `hostCapabilities` to `plugin.json` and bump `schemaVersion` to `2`.
3. Run the provider validator to catch typos.
4. Test the plugin — the Diagnostics panel should show "Capabilities: Explicit (N)".

## Adoption Metrics Required

Before removal, measure:
- **Runtime warning frequency.** The structured warning (`Plugin "<id>" is using legacy capability inference`) is logged on every probe of a v1-inferred plugin. Aggregating this across deployments gives the population of unmigrated plugins.
- **Plugin registry scan.** If a registry exists, scan for plugins still on schema v1.
- **Target: ≥90%** of active plugin deployments on schema v2.

## Recommendation

**Do not remove the v1 compatibility layer until adoption metrics are available and meet the ≥90% threshold.** Treat the map as part of the public platform contract. Removing it silently breaks any unmigrated third-party plugin with no warning — a breaking change that violates the project's compatibility posture.