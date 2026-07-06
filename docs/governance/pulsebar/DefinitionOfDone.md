# PulseBar Definition of Done

## Global Definition of Done

Every PulseBar PR must satisfy:

- Scope matches one workstream and one logical change.
- No accidental changes to `release/v0.7.0`.
- No unrelated formatting churn.
- Tests relevant to the change pass locally.
- CI passes before merge.
- Documentation is updated when behavior, user-facing copy, provider support, or release process changes.
- Security impact is considered for host APIs, auth, redaction, filesystem, keychain, HTTP, and diagnostics.
- Backward compatibility is preserved unless an approved migration plan exists.

## Program A: PulseBar Migration

Done when:

- User-facing product name is PulseBar in approved surfaces.
- Bundle ID remains `com.abyssbugg.pulseusage`.
- Application Support directory remains unchanged.
- Existing settings, plugin order, disabled providers, cache, and logs remain usable.
- Release notes explain the PulseUsage-to-PulseBar transition.
- No JS globals, plugin contracts, or keychain services are partially renamed.
- Manual smoke test confirms install, launch, restart, provider probing, and config persistence.

## Program B: Professional UI/UX

Done when:

- UI change is incremental and isolated.
- Before/after screenshots are attached to the PR.
- Keyboard navigation and focus visibility are verified.
- Empty, loading, error, and diagnostics states are checked.
- Settings changes preserve existing preferences.
- No provider data semantics change unless explicitly in scope.

## Program C: Ollama

Done when:

- Ollama public API evidence is cited in provider docs.
- Provider manifest uses schema v2 and minimal `hostCapabilities`.
- No quota, usage, billing, budget, reset, or forecast metrics are emitted without documented APIs.
- `OLLAMA_API_KEY` or equivalent auth source is redacted in logs/diagnostics and tested.
- Provider tests cover auth missing, auth present, API success, API failure, and malformed response.
- README and provider docs describe limitations honestly.

## Program D: Statistics Engine

Done when:

- Data model documents source, confidence, retention, privacy, and migration behavior.
- Persistence format is versioned.
- No derived metric is shown without source semantics.
- Local storage impact is bounded.
- Export/debug path exists for diagnosing bad data.
- Tests cover schema migration and corrupted persistence.

## Program E: Provider Platform

Done when:

- Third-party plugin compatibility remains intact.
- Schema v1 inference still works unless a separately approved deprecation removal occurs.
- New validation rules are non-breaking or migration-safe.
- Plugin author docs include examples and failure modes.
- Capability docs stay synchronized with `HostCapability` enum.

## Program F: Release Engineering

Done when:

- Version files align.
- Changelog has a complete release entry.
- Release readiness document exists for the target version.
- DMG builds and contains the expected app name, version, bundle ID, icons, and bundled plugins.
- Checksums are generated and verified.
- Smoke test passes on macOS 27.
- Release branch/tag/release flow follows governance docs.

## Release Candidate Definition of Done

A PulseBar release candidate is done when:

- All included workstreams meet their DoD.
- No P0/P1 issues remain.
- Known limitations are documented.
- Release artifacts are built and verified.
- Upgrade path from PulseUsage to PulseBar is tested.
- No unapproved bundle ID, app support, repository, JS global, or plugin compatibility changes are present.
