# IMP-001: PulseBar Implementation Master Plan

| Field | Value |
|-------|-------|
| **Status** | Approved (IRR amendments integrated) |
| **Baseline** | [EDR-001](../edr/001-pulsebar-direction.md) |
| **IRR** | [IRR-001](002-irr-readiness-review.md) |

## Executive Summary

Six implementation waves, ~38 focused PRs. Critical path:

```
P0 Housekeeping → P1 v0.6.28 → P2 v0.7.0 Rename → P3 Architecture → P4 Operations → v1.0
```

Estimated effort: 72–105 engineer-days (1 FTE: 15–21 weeks).

## Implementation Principles

1. One concern per PR
2. Phase gates before progression
3. Test before merge
4. Rollback by revert (except rename: forward-fix only)
5. No upstream code paste
6. PR links EDR/IMP epic ID
7. Feature branches from synced `main`
8. Files <~400 LOC per module

## IRR Amendments (integrated)

### A-001: Apple Developer enrollment (P0 admin, parallel)

Enroll in Apple Developer Program and export Developer ID Application cert **before P4 calendar start**.

### A-002: PR-025a — Updater dependency (before PR-030)

- Add `tauri-plugin-updater` to `Cargo.toml`
- Configure `tauri.conf.json` updater section
- Set `createUpdaterArtifacts: true`

### A-003: PR-025b — Signing pipeline (before PR-027)

- Add `APPLE_CERTIFICATE`, `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID` secrets
- Add codesign + notarization to `publish.yml`
- Create `docs/release-setup.md`

### A-004: PR-028 expanded — SWR + UI hydration

- Extend `cache.rs` session-aware semantics
- Add Tauri command `get_cached_snapshots`
- Bootstrap UI `pluginStates` from disk on launch
- Add "Outdated" indicator after ~2 missed cycles

### A-005: Rename PR stack (replaces single PR-007)

| PR | Scope |
|----|-------|
| PR-007a | `tauri.conf.json`, `package.json`, `Cargo.toml` identifiers |
| PR-007b | Rust paths (config, logs) |
| PR-007c | Frontend UI strings |
| PR-007d | 18 plugin globals + `runtime.rs` loader |
| PR-007e | Docs sweep + `targets: ["dmg"]` macOS-only |

### A-006: PR-001 rescoped

`fix(antigravity): align merged Gemini pool with OU v0.7.2` — write failing test first (`retrieveUserQuota` already exists).

### A-007: Worktree cleanup (P0) — complete

### A-008: Coverage in CI (Gate G3)

Add `bun run test:coverage` to `ci.yml`.

### A-009: Dev onboarding

Document: fresh checkout/worktree requires `bun install && bun run bundle:plugins` before `cargo test`.

## Phase Breakdown

| Phase | Version | Theme |
|-------|---------|-------|
| P0 | — | Housekeeping + docs ✅ |
| P1 | v0.6.28 | Provider patch |
| P2 | v0.7.0 | Rename |
| P3 | v0.7.1–0.7.2 | Settings SSOT, host split |
| P4 | v0.7.3–0.7.5 | Signing, updater, SWR |
| P5 | v0.8.x | UX polish |
| P6 | v1.0.0 | Stable gate |

## PR Sequence (amended)

| PR | Title | Phase | Depends |
|----|-------|-------|---------|
| PR-001 | fix(antigravity): align Gemini pool (rescoped) | P1 | P0 |
| PR-002 | fix(codex): fresh-window latency | P1 | P0 |
| PR-003 | fix(cursor): Sonnet 5 spend pricing | P1 | P0 |
| PR-004 | docs(plugins): primaryOrder schema | P1 | P0 |
| PR-005 | docs: EDR + IMP + IRR | P0 | — |
| PR-006 | docs: upstream governance + provider parity | P0 | PR-005 |
| PR-007a–e | Rename stack (5 PRs) | P2 | v0.6.28 tag |
| PR-009–014 | Config migration, env markers, defaultEnabled | P2 | PR-007a |
| PR-015 | refactor: Rust settings SSOT | P3 | v0.7.0 |
| PR-016 | refactor: frontend settings IPC | P3 | PR-015 |
| PR-017 | fix: HTTP API settings consistency | P3 | PR-015 |
| PR-018 | refactor: AppBootstrap | P3 | PR-015 |
| PR-019–024 | host_api split (6 PRs) | P3 | PR-018 |
| PR-025a | feat: tauri-plugin-updater setup | P4 | Gate G3 |
| PR-025b | ci: signing + notarization | P4 | A-001 |
| PR-026 | ci: universal binary + publish.yml | P4 | PR-025b |
| PR-028 | feat: SWR cache + UI hydration | P4 | PR-017 |
| PR-029 | feat: Outdated indicator | P4 | PR-028 |
| PR-030 | feat: Tauri updater wiring | P4 | PR-025a, PR-026 |
| PR-031 | feat: API key settings UI | P4 | PR-016 |
| PR-032 | feat: Reset All Customization | P4 | PR-016 |
| PR-033–035 | UX polish | P5 | Gate G4 |
| PR-036–038 | v1.0 validation + release | P6 | Gate G5 |

## Validation Gates

| Gate | Criteria |
|------|----------|
| G0 | EDR/IMP/IRR committed; `main` synced | ✅ Phase 0 |
| G1 | v0.6.28 provider fixes; no rename |
| G2 | Rename complete; migration tested |
| G3 | Settings SSOT; host split; schema CI; coverage in CI |
| G4 | Signed DMG; updater N→N+1; SWR + UI hydration |
| G4.5 | Apple cert verified in CI |
| G5 | v1.0 checklist; 2-week soak |

## Branch Strategy

| Type | Pattern | Use |
|------|---------|-----|
| Research | `research/pulsebar-architecture` | Planning docs only |
| Feature | `feat/*`, `fix/*`, `chore/*` | Implementation PRs |
| Release | `release/v*` | Optional soak |

**Rules:** No `develop` branch. Branch from synced `main`. Max 3 active worktrees.

## Must NOT overlap

- Rename + provider fixes on same plugin files
- `host_api` split + provider fix PRs
- Settings SSOT + HTTP API features
- Signing first release + large feature merge

## Rollback

- Per-PR: `git revert`
- Rename: forward-fix only
- Publish: disable `publish.yml`
- Keep 2 prior signed DMGs on Releases

## Migration (v0.7.0)

1. Manual install PulseBar DMG
2. First launch copies settings + cache from old app data dir
3. Proxy: `~/.pulseusage/` → `~/.pulsebar/`
4. Port 6736 unchanged

## v1.0 Criteria

See [EDR-001 §v1.0](../edr/001-pulsebar-direction.md).

## Effort Estimate (amended)

| Phase | Days |
|-------|------|
| P0 | 3–5 ✅ |
| P1 | 5–7 |
| P2 | 12–18 (rename expanded) |
| P3 | 18–25 (6 host PRs) |
| P4 | 22–30 (signing + UI cache) |
| P5 | 8–12 |
| P6 | 8–10 |
| **Total** | **76–107** |

## Authorized next step

**Phase 1** after Gate G0 ✅ and explicit Phase 1 approval.