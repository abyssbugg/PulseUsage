# Release Plan

> **Canonical release strategy.** Future agents must follow this for all releases.
> Last updated: 2026-07-03

## Current Release

| Field | Value |
|---|---|
| **Latest release** | v0.6.28 |
| **Release date** | 2026-07-02 |
| **Tag** | `v0.6.28` (annotated, points to `8c85d65`) |
| **Merge commit** | `120b0bc` (PR #14) |
| **Version bump commit** | `8c85d65` |
| **DMG artifact** | `PulseUsage_0.6.28_aarch64.dmg` (11.5MB) |
| **SHA256** | `326da0423b25578236fd35abd5ed8f33ee0d3d22851039856e9c2184ed97e0ec` |
| **Release URL** | https://github.com/abyssbugg/PulseUsage/releases/tag/v0.6.28 |
| **Main ahead of tag** | 12 commits (post-release work) |

## Release Strategy

PulseUsage uses **manual direct-download distribution**. The `publish.yml` GitHub workflow is intentionally disabled (`if: ${{ false }}`) — the app serves 2-5 internal users and notarization is not justified (AGENTS.md: simplicity first).

Releases are built locally, tagged, and published via `gh release create` with DMG upload.

Full process: [docs/governance/ReleaseProcess.md](./docs/governance/ReleaseProcess.md)

## Next Release: v0.7.0

### Recommendation: Stabilize first. Do NOT release immediately.

**Rationale:**
1. The 12 post-v0.6.28 commits are NOT independently release-worthy (incremental fixes + 1 feature).
2. 6 dependabot PRs should merge first — releasing with pending deps ships stale dependencies.
3. Issue #26 (Antigravity LS probe hardening) is open and unreleased.
4. v0.7.0 should bundle: dependabot merges + security hardening + macOS 27 fix + npm refresh + (optionally) modularization + capability model.
5. No release-blocking issue exists. v0.6.28 is stable and published.

### v0.7.0 Scope (proposed)

| Item | Effort | Priority |
|---|---|---|
| Merge 6 dependabot PRs (#18-#23) | 1 hour | P1 |
| Security hardening PR (8 items) | 3 hours | P2 |
| macOS 27 keychain write fix | 30 min | P2 |
| npm deps refresh (batch PR) | 2 hours | P3 |
| `host_api.rs` modularization | 4-6 hours | P3 |
| Plugin capability manifest enforcement | 6-8 hours | P3 |
| `deleteGenericPassword` host impl | 20 min | P3 |
| Perplexity `Agentic Research` classification | 1 day (research) | P3 |
| **Total** | ~2 weeks | |

### v0.7.0 Release Blockers

- [ ] 6 dependabot PRs merged
- [ ] Security hardening PR merged
- [ ] macOS 27 keychain write fix verified on macOS 27.0
- [ ] Full test suite green (1,200+ tests)
- [ ] CI green on main
- [ ] Production verification report approved
- [ ] Version bump (4 files aligned)
- [ ] CHANGELOG entry added
- [ ] Release readiness report (docs/release-readiness/v0.7.0.md)

### v0.7.0 Timeline (estimated)

- **Week 1:** Dependabot merges + security hardening + macOS 27 fix + npm refresh
- **Week 2:** Modularization + capability manifest + Perplexity classification
- **End of Week 2:** v0.7.0 release

## Future Releases

### v0.8.0 — PulseBar Migration (post-v0.7.0, when approved)

- Execute [IMP-005](./docs/imp/005-pulsebar-migration-plan.md)
- Rename PulseUsage → PulseBar
- Bundle ID migration with preferences/keychain migration logic
- Release as v0.8.0 (or v1.0.0 if combined with notarization)
- **Recommendation:** Release v0.7.0 first, then migrate to PulseBar in v0.8.0. Do NOT combine the rename with v0.7.0 architecture work — the rename is high-risk and should be isolated.

### v1.0.0 (conditional — only if triggered)

- Notarization (if macOS 27 enforces or user base >10)
- App Sandbox (if macOS 27 enforces)
- Process-exec native crates (if energy complaints or plugin count >25)
- **Do NOT release v1.0 unless a trigger condition is met.**

## Release vs PulseBar Migration: Sequencing Recommendation

**Recommendation: Release v0.7.0 BEFORE the PulseBar migration.**

Reasons:
1. **Isolate risk.** v0.7.0 is architecture work (modularization, capability model). PulseBar is a rename (1,006 occurrences, bundle ID change, keychain service migration). Combining them creates a massive, hard-to-review PR.
2. **Stable baseline for migration.** The PulseBar migration (IMP-005) should start from a clean v0.7.0 baseline, not from a moving main.
3. **User continuity.** v0.7.0 ships as PulseUsage (familiar to users). v0.8.0 ships as PulseBar (with migration logic). Users can choose when to upgrade.
4. **Rollback safety.** If the rename breaks something, v0.7.0 remains a stable fallback.

## Version Alignment Rule

Before any release, verify all 4 version files are aligned:
```bash
grep '"version"' package.json
grep '^version' src-tauri/Cargo.toml
grep '"version"' src-tauri/tauri.conf.json
grep -A1 '^name = "pulseusage"' src-tauri/Cargo.lock | head -2
```
All four must report the same version. See [docs/governance/ReleaseProcess.md](./docs/governance/ReleaseProcess.md).
