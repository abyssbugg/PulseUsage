# PulseBar Risk Register

## Risk Classification

| Severity | Meaning |
|---|---|
| Critical | Can cause user data loss, broken app launch, auth loss, or incompatible plugin behavior. |
| High | Can block release or create difficult rollback. |
| Medium | Can degrade UX, increase support, or create maintenance debt. |
| Low | Manageable cleanup or documentation issue. |

## Risks

| Risk | Severity | Workstream | Mitigation | Release Blocker? |
|---|---|---|---|---|
| Bundle identifier change orphans user state | Critical | Program A | Keep `com.abyssbugg.pulseusage` for v0.8.x. Any future change requires separate migration program. | Yes |
| App Support path migration loses settings/plugin data | Critical | Program A | Do not move app support path in v0.8.x. | Yes |
| Repository rename breaks release links/workflows | High | Program A/F | Keep repo `PulseUsage` for v0.8.0. Rename later only after stable PulseBar release. | Yes if attempted |
| JS global rename breaks plugins | High | Program A/E | Do not rename JS globals in initial visible rename. If needed later, do atomically with all plugins/tests. | Yes if partial |
| Copilot keychain service rename loses auth | Critical | Program A | Keep keychain service names initially. If changed later, old-read/new-write fallback for one release. | Yes |
| Removing v1 compatibility breaks third-party plugins | Critical | Program E | Do not remove v1 map. Follow deprecation policy requiring adoption metrics. | Yes |
| Ollama overclaims usage/billing/quota | High | Program C | Only documented APIs. Mark unsupported/undocumented when evidence missing. | Yes |
| Statistics engine shows misleading data | High | Program D | Require source/confidence model and evidence-backed metrics. | Yes |
| UI rewrite destabilizes app | Medium | Program B | Incremental refinements only. Visual PRs require screenshots. | No, unless regressions |
| Accessibility regressions | Medium | Program B | Keyboard/focus checks in each visual PR. | Yes for visual release |
| Release artifact naming mismatch | Medium | Program F | Validate DMG name, app name, version, bundle ID before tag. | Yes |
| Docs drift after v0.7.0 RC | Medium | Governance | Reconcile control-center docs before implementation. | No, but should precede v0.8 work |
| Ad-hoc signing user friction | Medium | Program F | Document Gatekeeper bypass; notarization is future decision. | No |
| Perplexity metric remains unclassified | Low | Provider Platform | Keep strict warning; evidence required before classification. | No |

## Required Escalations

Escalate for explicit approval before:

- Changing bundle identifier.
- Changing app support path.
- Renaming GitHub repository.
- Renaming JS plugin globals.
- Removing v1 compatibility.
- Claiming Ollama usage, quota, billing, budget, or progress data.
- Adding persistent statistics storage.
- Starting a full UI redesign.

## Risk Acceptance

The following risks are accepted for v0.8.x:

- Internal bundle ID remains PulseUsage while display name becomes PulseBar.
- GitHub repository remains PulseUsage while product name becomes PulseBar.
- Ad-hoc signing remains unless a separate release-engineering decision approves Developer ID signing/notarization.
