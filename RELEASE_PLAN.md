# Release Plan

> **Canonical release strategy.** Future agents must follow this for all releases.
> Last updated: 2026-07-06

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
| **Main ahead of tag** | 30+ commits (PR #29 maintenance baseline + Program 1 PRs #30–#45) |

## Release Strategy

PulseUsage uses **manual direct-download distribution**. The `publish.yml` GitHub workflow is intentionally disabled (`if: ${{ false }}`) — the app serves 2-5 internal users and notarization is not justified (AGENTS.md: simplicity first).

Releases are built locally, tagged, and published via `gh release create` with DMG upload.

Full process: [docs/governance/ReleaseProcess.md](./docs/governance/ReleaseProcess.md)

## Next Release: v0.7.0

### Status: Nearly ready — Program 2 (capability enforcement) is the last prerequisite

**Completed for v0.7.0:**
- ✅ Dependency refresh (PR #30 safe bumps + PR #31 major bumps)
- ✅ Security hardening (PR #29 — 8 items resolved)
- ✅ macOS 27 keychain write fix (PR #29)
- ✅ host_api modularization (Program 1 — PRs #32–#45)
- ✅ npm deps refresh (PR #30, #31)

**Remaining for v0.7.0:**
- ⬜ Plugin capability manifest enforcement (Program 2 — design approved, 6 PRs, ~9 hours)
- ⬜ `deleteGenericPassword` host impl (part of Program 2)
- ⬜ Documentation synchronization (this PR)
- ⬜ Version bump (4 files aligned)
- ⬜ CHANGELOG entry
- ⬜ Release readiness report (docs/release-readiness/v0.7.0.md)
- ⬜ Tag + DMG + publish

### v0.7.0 Scope

| Item | Effort | Priority | Status |
|---|---|---|---|
| Merge 6 dependabot PRs (#18-#23) | 1 hour | P1 | ✅ Superseded by PR #29/#30/#31 |
| Security hardening PR (8 items) | 3 hours | P2 | ✅ Complete (PR #29) |
| macOS 27 keychain write fix | 30 min | P2 | ✅ Complete (PR #29) |
| npm deps refresh (batch PR) | 2 hours | P3 | ✅ Complete (PR #30, #31) |
| host_api modularization | 4-6 hours | P3 | ✅ Complete (Program 1) |
| Plugin capability manifest enforcement | 6-8 hours | P3 | ⬜ Design approved (Program 2) |
| `deleteGenericPassword` host impl | 20 min | P3 | ⬜ Part of Program 2 |
| Perplexity `Agentic Research` classification | 1 day (research) | P3 | ⬜ Deferred — needs research |
| **Remaining** | ~1 day | | |

### v0.7.0 Release Blockers

- [x] 6 dependabot PRs merged (superseded by PR #29/#30/#31)
- [x] Security hardening PR merged (PR #29)
- [x] macOS 27 keychain write fix verified on macOS 27.0
- [x] Full test suite green (139 Rust + 1,109 JS = 1,248 tests)
- [x] CI green on main
- [ ] Documentation synchronized (this PR)
- [ ] Program 2 (capability enforcement) complete
- [ ] Version bump (4 files aligned)
- [ ] CHANGELOG entry added
- [ ] Release readiness report (docs/release-readiness/v0.7.0.md)

### v0.7.0 Timeline (estimated)

- **Complete:** Dependabot merges + security hardening + macOS 27 fix + npm refresh + modularization
- **Remaining:** Program 2 (capability enforcement, ~9 hours, 6 PRs) + doc sync + release process (~2 hours)
- **Estimated release:** Within 1-2 working sessions after Program 2 approval

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
