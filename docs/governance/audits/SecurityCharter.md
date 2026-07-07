# Security Audit Charter (SEC)

> Trigger: any change under `src-tauri/src/plugin_engine/` or `src-tauri/src/local_http_api/`; a new provider auth source; dependency vulnerability alerts. Process: [AuditProcess.md](AuditProcess.md).

## Invariants to verify first

INV-003 (keychain service names), INV-007 (capability least-privilege), INV-008 (plugin global atomicity), INV-010 (secret redaction) — checks in [Invariants.md](../Invariants.md).

## Scope

| Surface | Files | What to examine |
|---|---|---|
| Capability enforcement | `src-tauri/src/plugin_engine/capability.rs`, `manifest.rs` | Schema-v2 `hostCapabilities` declared vs granted; v1 inference (`infer_v1_capabilities_*`) grants nothing broader than documented; deny-by-default holds for unknown capability strings |
| Host API injection | `src-tauri/src/plugin_engine/host_api/` | Every injected function gated by a capability check; no function reachable without one; `inject_host_api_with_deadline` failure modes |
| Keychain access | host_api keychain functions + `plugins/*/plugin.js` call sites | Read/write/delete each separately gated; service names stable (INV-003) |
| Redaction | `src-tauri/src/plugin_engine/redaction.rs`, `diagnostics.rs` | Every auth source (per-provider env vars, keychain values) covered; new providers add redaction + tests per DoD |
| Local HTTP API | `src-tauri/src/local_http_api/` | Bind address, auth on endpoints, request validation, no plugin-reachable escalation |
| Subprocess spawning | process-exec call sites (`plist`, `sqlite`, `security` CLI usage per TECHNICAL_DEBT #16) | Argument injection, path handling via `expand_path` |
| Supply chain | `src-tauri/Cargo.lock`, `bun.lock` | `cargo audit` / advisory state for direct deps; unsafe-Rust inventory (`grep -rn "unsafe" src-tauri/src`) |

## Method

1. Run the invariant checks; any violation is an immediate Critical finding.
2. Diff-scoped pass over the triggering change: what capability, path, or trust boundary did it move?
3. Adversarial pass: for each host API function, write the attack — "a malicious `plugin.js` with manifest X can do Y." A finding without a concrete attack path is an observation, not a security finding.
4. Check the DoD security bullet was actually satisfiable for the triggering PR.

## Out of scope

Notarization and App Sandbox (tracked as conditional debt #17/#18 in TECHNICAL_DEBT.md — do not re-report). Ad-hoc signing (accepted risk in RiskRegister).

## Run history

Completed runs are recorded as individual files in [history/](history/) named `<CHARTER>-<YYYY-MM-DD>.md`.
