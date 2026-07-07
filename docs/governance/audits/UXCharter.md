# UI/UX Audit Charter (UX)

> Trigger: completion of each Program B (Professional UI/UX) phase; batch review of merged `src/components/` changes at phase boundary. Process: [AuditProcess.md](AuditProcess.md).

## Invariants to verify first

INV-004 (identity strings via `src/lib/app-identity.ts`), INV-005 (design-token compliance).

## Scope

| Concern | Where to look | What to examine |
|---|---|---|
| Token compliance | `src/components/` vs `src/index.css` tokens | No new hardcoded visual values (ADR-002 audit found 40; the count must only go down). Check: hex colors, arbitrary `text-[…]`/`h-[…]`/`w-[…]`, ad-hoc `shadow-*`, ad-hoc `duration-*` |
| Component contract | [`docs/design-system/component-contract.md`](../../design-system/component-contract.md) | New/changed components honor the contract; semantic aliases (success/warning/error/link/surface) used over raw palette values |
| Consistency | provider rows, settings, diagnostics surfaces | Same concept → same primitive; spacing/size scale adherence; light/dark parity via semantic tokens (no per-occurrence `dark:` for tokenized colors) |
| Accessibility | interactive components | Keyboard navigation and visible focus (Program B DoD bullet); contrast of token pairs in both themes; hit targets in the menu-bar panel |
| States | data-driven views | Empty, loading, error, and diagnostics states exist and are styled intentionally (Program B DoD) |
| macOS conventions | panel behavior (`src-tauri/src/panel.rs`, `tray.rs`), menus | Menu-bar app idioms: dismiss on outside click, standard menu phrasing, no non-native chrome |
| Copy | user-visible strings | Product name via identity layer; error messages actionable; terminology matches `docs/design-system/` vocabulary |

## Method

1. Invariant checks first — INV-005 violations are the highest-frequency expected finding class; report with exact file:line and the token that should have been used.
2. Verify each Program B DoD bullet against the phase's merged PRs, including that before/after screenshots were actually attached.
3. Run the app and exercise the audited surfaces — a UX audit from source reading alone does not survive refutation.
4. Consistency findings must name both occurrences (the divergent pair), not just one side.

## Out of scope

Visual redesign proposals — starting a full UI redesign requires explicit approval per the [RiskRegister](../pulsebar/RiskRegister.md) escalation list. Provider data semantics.

## Run history

Completed runs are recorded as individual files in [history/](history/) named `<CHARTER>-<YYYY-MM-DD>.md`.
