# Release Readiness Audit Charter (REL)

> Trigger: before every RC tag. Output feeds the versioned readiness doc in `docs/release-readiness/` (see `v0.6.28.md` for the established format). Process: [AuditProcess.md](AuditProcess.md).

## Invariants to verify first

INV-001 (bundle ID), INV-002 (App Support path), INV-006 (release branch policy), INV-009 (repo/release URLs), INV-011 (control-center doc freshness).

## Scope

| Concern | Where to look | What to examine |
|---|---|---|
| Artifact identity | `src-tauri/tauri.conf.json` (`productName`, `version`, `identifier`), built DMG | DMG name, app name, version, and bundle ID all consistent (RiskRegister: "Release artifact naming mismatch" is a tracked Medium) |
| Versioning | `tauri.conf.json`, `package.json`, `CHANGELOG.md`, git tag | Single version everywhere; changelog entry exists and matches milestone scope |
| Branch state | `release/v0.7.0`, `main` | Release branch contains only verified RC fixes since freeze (INV-006); no orphan feature commits |
| CI / reproducibility | `.github/workflows/` | Release workflow green; build reproducible from a clean checkout; publish workflow state matches the notarization decision (debt #17 — disabled is the accepted state) |
| Signing | build output | Ad-hoc signing intact (accepted risk); Gatekeeper bypass documented for users |
| Upgrade path | fresh install + upgrade-over-previous | Settings, plugin order, disabled providers, cache, and logs survive upgrade (INV-002); smoke test per Program A DoD: install, launch, restart, provider probing, config persistence |
| Rollback | previous release artifact | Downgrade path documented; no forward-only state migration ships without a rollback note |
| Docs | `CURRENT_PHASE.md`, `RELEASE_PLAN.md`, `PROJECT_STATUS.md`, release notes | All reflect the release being cut; PulseUsage→PulseBar transition explained in notes (Program A DoD) |

## Method

1. Run the invariant checks — any violation blocks the RC outright.
2. Walk the table above as a checklist; every row gets pass/fail with evidence in the readiness doc.
3. Perform the manual smoke test on the actual built artifact, not a dev build.
4. Cross-check RiskRegister release-blocker rows: each "Yes" blocker must be demonstrably mitigated for this release.

## Out of scope

Feature completeness judgments (milestone/human decision). Notarization and App Sandbox until their debt triggers fire.

## Run history

Completed runs are recorded as individual files in [history/](history/) named `<CHARTER>-<YYYY-MM-DD>.md`.
