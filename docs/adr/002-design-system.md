# ADR-002: Design System Foundation

| Field | Value |
|-------|-------|
| **Status** | Approved |
| **Date** | 2026-07-07 |
| **Baseline** | [ADR-001](001-pulsebar-v1-provider-usage-baseline.md) |
| **Related** | [Tokens](../design-system/tokens.md), [Component Contract](../design-system/component-contract.md), [Migration Guide](../design-system/migration-guide.md) |

## Context

PulseBar's UI is built with React + Tailwind v4 (CSS-first config) and renders provider-owned `MetricLine` output through a small set of primitives and composed components. Before this ADR, visual values were defined ad-hoc across the component tree:

- Hardcoded hex colors (`#58a6ff`, `#353537`)
- Arbitrary Tailwind font sizes (`text-[10px]`, `text-[11px]`, `text-[13px]`)
- Arbitrary control heights (`h-[18px]`, `h-[22px]`, `h-5`)
- Arbitrary widths and radii
- Ad-hoc elevation (`shadow-xl`, `shadow-lg`, `shadow-md`, `shadow-2xl`)
- Ad-hoc motion (`duration-200`)

An audit identified **40 hardcoded visual values** across `src/components/`. Each was a one-off decision made in isolation at the time the component was written. There was no single source of truth, no theme-aware aliases for semantic concepts (success, warning, error, link, surface), and no contract preventing new hardcoded values from accumulating.

This is the foundation PR for Program B (UI hardening). No visual redesign is being done in this PR. The purpose is to establish the permanent visual foundation that every future UI change will use.

## Problem

1. **Visual drift.** With no central source of truth, two components needing "small text" independently chose `text-[10px]`, `text-[11px]`, and `text-[13px]`. There is no mechanism to keep them consistent.

2. **No semantic layer.** Components reference raw values (`#58a6ff`) instead of purpose (`--link`). When the link color needs to change, every occurrence must be found individually. Dark-mode overrides require per-occurrence `dark:` variants.

3. **No contract against new debt.** A contributor adding a new component can write `text-[12px]` or `shadow-xl` without any signal that this is wrong. The codebase has no rule saying "don't do this."

4. **Migration risk.** A big-bang rewrite replacing all 40 values in one PR would touch many files, produce a large diff, risk visual regressions, and block all other UI work until merged.

5. **Theme coupling.** Without theme-aware tokens, dark-mode behavior is scattered across `dark:` variants in component files rather than centralized in one place.

## Decision

Adopt a **Design System Foundation** with four artifacts and one contract:

1. **Design Tokens** — CSS custom properties in `src/index.css` covering spacing, typography, radius, elevation, motion, transitions, focus rings, opacity, z-index, icon/control sizing, and semantic color aliases. Theme-aware (dark-mode overrides in `:root.dark`).

2. **Component Contract** — The layering rule:
   ```
   Theme
     ↓
   Design Tokens
     ↓
   Primitives
     ↓
   Components
     ↓
   Screens
   ```
   No layer may skip a level. No screen may introduce custom visual constants.

3. **Migration Guide** — All 40 hardcoded values mapped to replacement tokens with file:line locations, classified by priority (P0/P1/P2/P3), and migrated incrementally in PR-B2 through PR-B7.

4. **This ADR** — The permanent architectural rationale.

The contract is enforced through code review (PR template, reviewer checklist) and the migration guide's status tracking. There is no runtime enforcement — the contract is a project convention, not a build-time gate.

## Alternatives Considered

### Alternative A: Big-bang rewrite
Replace all 40 hardcoded values in a single PR.

**Rejected.** A single PR touching every component file would:
- Produce a massive diff that is hard to review
- Risk visual regressions across the entire app
- Block all other UI work until merged
- Mix high-impact changes (link color used 4×) with low-impact cleanup (rare component values)
- Make bisection hard if a visual regression is introduced

### Alternative B: Tailwind theme extension only
Extend Tailwind's `@theme` block with new utility values (e.g., `text-2xs`, `shadow-elev-1`) and use Tailwind utilities everywhere.

**Rejected.** Tailwind utilities are value-oriented, not purpose-oriented. `text-2xs` tells you the size, not the semantic role. We need both: tokens for the source of truth (CSS variables), and Tailwind utilities that reference those tokens. Pure Tailwind theme extension also makes dark-mode overrides harder to centralize.

### Alternative C: CSS-in-JS with a runtime theme provider
Use a runtime theme object (e.g., styled-components, emotion) injected via React context.

**Rejected.** PulseBar is a small local-first Tauri app. Adding a runtime theming layer introduces bundle size, runtime cost, and a new abstraction the team must learn. CSS custom properties already provide theme-aware values with zero runtime cost and native browser support.

### Alternative D: Keep ad-hoc values, add a linter later
Defer the design system until a linter can enforce it.

**Rejected.** The linter is the wrong starting point. Without a token system to lint *against*, a linter can only forbid arbitrary values — it cannot suggest the correct replacement. The token system must exist first.

### Alternative E: Adopt an external design system (shadcn, Radix, etc.)
Pull in a full external design system.

**Rejected.** PulseBar already has its own primitives in `src/components/ui/`. Replacing them with an external system would be a visual redesign, which is explicitly out of scope for Program B. The design system must fit the existing components, not the other way around.

## Consequences

### Positive

- **Single source of truth.** Every visual value has exactly one definition location. Changing `--link` updates every link in the app.
- **Semantic clarity.** Components reference purpose (`--success`, `--warning`, `--link`) not raw values. Intent is self-documenting.
- **Centralized theme.** Dark-mode overrides live in `:root.dark` in one file. Component files no longer need `dark:` variants for token-backed values.
- **Incremental migration.** PR-B2 through PR-B7 can migrate values by priority without blocking other work. The token system is additive — old hardcoded values keep working until explicitly replaced.
- **Forward compatibility.** New components use tokens from day one. The migration guide's status tracking makes remaining debt visible.
- **Zero visual change in this PR.** Tokens are additive CSS variables. No component references them yet, so no visual output changes. PR-B1 is invisible to users.

### Negative

- **Two systems coexist temporarily.** Until migration is complete, both hardcoded values and tokens exist in the codebase. Contributors must know which to use. The component contract and migration guide document this.
- **Token names are a new vocabulary.** Contributors must learn `--space-4`, `--font-size-2xs`, `--shadow-md` (token alias) vs `shadow-md` (Tailwind utility). The tokens.md reference mitigates this.
- **No runtime enforcement.** A contributor can still write `text-[12px]` in a new component. Code review is the only gate. A future PR could add a lint rule if debt recurs.
- **Migration guide maintenance.** The 40-row table must be updated as migrations land. This is a small ongoing cost during Program B.

### Neutral

- **Tailwind v4 `@theme inline` is preserved.** Existing Tailwind utilities (`bg-background`, `text-foreground`, `border-border`) continue to work. The new tokens are an additional layer, not a replacement for Tailwind theme tokens.
- **No new dependencies.** Pure CSS custom properties. No runtime, no build-time cost.
- **No test changes.** Token additions are CSS-only. Existing JS tests, Rust tests, and provider validation are unaffected.

## Migration Strategy

**Incremental, priority-ordered.** Not a big-bang rewrite.

The migration guide classifies all 40 hardcoded values by priority:

| Priority | Meaning | Migration PR |
|---|---|---|
| **P0** | Blocks consistency — colors and spacing used many times | PR-B2 (colors) |
| **P1** | Common components — typography outliers, control sizing | PR-B3 (font sizes), PR-B4 (control heights) |
| **P2** | Rare component values — elevation, motion | PR-B5 (elevation), PR-B6 (motion) |
| **P3** | Nice-to-have cleanup — widths, one-off values | PR-B7 (widths) |

**Ordering rationale:** Migrate the highest-impact, most-repeated values first. This maximizes consistency gain per PR and minimizes regression surface. Lower-priority cleanup can be deferred without blocking feature work.

**PR-B1 scope is strictly additive.** This PR introduces tokens + docs only. No component reads from the new tokens yet. No visual output changes. This guarantees PR-B1 cannot introduce a visual regression.

Each migration PR (B2–B7) is independently mergeable and individually small. If a migration PR introduces a visual regression, it can be reverted without affecting the token foundation or other migrations.

## Long-Term Maintenance

### When to introduce a new token
- The value is used by 2+ components, OR
- The value has a semantic purpose not covered by an existing token, OR
- The value needs dark-mode awareness

Introduce the token in `src/index.css`, document it in `tokens.md`, and reference it from the component. Do not introduce a token for a one-off value used in a single component — that value should use an existing token or be promoted to a primitive.

### When to reuse a token
- A value matches an existing token's purpose → use the token, never hardcode
- A value is "close to" an existing token but not exact → use the token. Visual consistency matters more than pixel-perfect intent.
- A value is genuinely new → introduce a new token (see above), do not hardcode

### When to deprecate a token
- A token is unused after migration → remove it from `src/index.css` and `tokens.md` in a separate cleanup PR
- A token's semantic purpose changes → rename it (add new, migrate references, remove old) rather than repurposing in place

### Token naming conventions
- Semantic aliases: `--success`, `--warning`, `--error`, `--info`, `--link`, `--surface-*` — named by purpose
- Scale tokens: `--space-4`, `--font-size-2xs`, `--shadow-md`, `--duration-fast` — named by position in scale
- Dark-mode overrides: same name, value overridden in `:root.dark` — never create `--*-dark` variants

### Reviewer checklist (for future PRs)
- [ ] No new hardcoded visual values in `src/components/`
- [ ] New components reference tokens, not raw values
- [ ] If a new token was added: it is documented in `tokens.md`, has a semantic purpose, and follows naming conventions
- [ ] If a value was migrated: the migration guide's status for that row is updated

## Non-Goals

- No visual redesign in PR-B1. Tokens are additive; no component references them yet.
- No runtime enforcement of the contract. Code review is the gate.
- No external design system adoption. The token system fits existing primitives.
- No CSS-in-JS runtime theming layer.
- No replacement of Tailwind v4 `@theme inline`. New tokens are an additional layer.
- No removal of existing hardcoded values in this PR. Migration happens in PR-B2 through PR-B7.
- No test changes. Token additions are CSS-only.

## Exit Criteria For Program B

Program B is complete when:
1. All P0 values are migrated (consistency-critical colors and spacing)
2. All P1 values are migrated (common component typography and control sizing)
3. P2 and P3 may be deferred if remaining debt is low-impact
4. The migration guide's status column shows ✅ for all P0 and P1 rows
5. No new hardcoded values have been introduced since PR-B1

The goal is not "100% migrated" — it is "the system is now the obvious way to build UI." Once new components naturally use tokens because that is the path of least resistance, Program B has succeeded.

## References

- [ADR-001: PulseBar v1 Provider and Usage Baseline](001-pulsebar-v1-provider-usage-baseline.md)
- [Design Tokens](../design-system/tokens.md)
- [Component Contract](../design-system/component-contract.md)
- [Migration Guide](../design-system/migration-guide.md)
- [Engineering Principles](../engineering-principles.md)
