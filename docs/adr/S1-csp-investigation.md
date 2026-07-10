# ADR-S1: Content Security Policy for Tauri WebView

| Field | Value |
|---|---|
| **Status** | Proposed — Awaiting POC |
| **Date** | 2026-07-09 |
| **Trigger** | SEC-2026-07-09 finding SEC-004 (VULN-001) |
| **Related** | [SecurityCharter.md](../governance/audits/SecurityCharter.md), [SEC-2026-07-09](../governance/audits/history/SEC-2026-07-09.md) |

## Context

The Tauri WebView currently has `"csp": null` in `src-tauri/tauri.conf.json:26`, meaning no Content Security Policy is enforced. An external security analysis flagged this as CWE-693 (Protection Mechanism Failure) with MEDIUM severity.

CSP is a defense-in-depth layer: even if an XSS vector is introduced in the React frontend, CSP restricts what scripts can execute, what styles can be applied, and what endpoints can be connected to. React's default escaping mitigates most XSS, but CSP protects against future regressions.

## Problem

Tauri WebViews are not standard web applications. A naive CSP can break:

1. **Vite dev mode** — HMR injects inline scripts for hot module replacement
2. **Tailwind CSS** — requires `'unsafe-inline'` for style application
3. **Tauri IPC** — uses `ipc:` and `http://ipc.localhost` for command invocation
4. **Production assets** — scripts and styles loaded from `self`
5. **Data URIs** — icons or images may use `data:` scheme

A CSP that breaks any of these would trade a marginal security gain for a broken UI.

## Decision

**Investigate via proof-of-concept before applying CSP.**

### POC Plan

1. Create a branch `program-s/csp-poc`
2. Apply the recommended CSP:
   ```
   default-src 'self';
   script-src 'self';
   style-src 'self' 'unsafe-inline';
   img-src 'self' data:;
   connect-src 'self' ipc: http://ipc.localhost
   ```
3. Run `bun run dev` — verify HMR works, no console errors
4. Run `bun run build` — verify production build works
5. Manual click-through: open panel, navigate to each view (overview, provider detail, settings, about, changelog), trigger a refresh, verify no CSP violations in console
6. If any view breaks, tune the CSP and re-test
7. Document the validated CSP in this ADR

### Validation Checklist

- [ ] Vite dev mode: HMR works, no inline-script violations
- [ ] Tailwind: styles applied correctly (no `style-src` violations)
- [ ] Tauri IPC: `invoke()` calls succeed (no `connect-src` violations)
- [ ] Production build: all assets load from `self`
- [ ] Data URIs: icons render (no `img-src` violations)
- [ ] All views: overview, provider detail, settings, about, changelog
- [ ] Tray menu: log level control, diagnostics export
- [ ] No new console errors or warnings

## Alternatives Considered

### Alternative A: Keep CSP null (status quo)
**Rejected.** While React's escaping is good defense, CSP is cheap defense-in-depth that protects against future regressions. The POC will confirm whether it's safe to enable.

### Alternative B: Use Tauri's default CSP
**Rejected without testing.** Tauri 2 can generate a default CSP, but it may not account for Tailwind's `'unsafe-inline'` style requirement. The POC approach is more controlled.

## Consequences

### If POC succeeds
- CSP is applied in PR-S4
- WebView has defense-in-depth against XSS
- Future React regressions that introduce inline scripts will be caught

### If POC finds breakage
- CSP is tuned (e.g., add `'unsafe-inline'` to `script-src` if Vite HMR requires it)
- ADR documents the final validated CSP string
- Trade-off documented between security and functionality

### If POC finds fundamental incompatibility
- ADR documents why CSP cannot be applied
- Alternative mitigations considered (e.g., Subresource Integrity, stricter React ESLint rules)
- Finding reclassified

## Open Questions

- Does `bun run dev` inject inline scripts that would require `'unsafe-inline'` on `script-src`?
- Does Tauri 2's IPC use `ipc:` or `http://ipc.localhost` or both?
- Are there any `eval` or `new Function()` calls in the frontend that would require `'unsafe-eval'`?

## Next Steps

1. Create POC branch
2. Run validation checklist
3. Update this ADR with validated CSP string
4. If successful, create PR-S4 with the CSP applied