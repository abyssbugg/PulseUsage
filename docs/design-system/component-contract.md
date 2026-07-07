# Component Contract

The architectural rule governing how visual values flow from theme to screen.

## The Contract

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

**No layer may skip a level.** No screen may introduce custom visual constants.

## Layer Responsibilities

### Theme
The user's OS appearance setting (light/dark). Drives the `:root` vs `:root.dark` selector. Theme defines the *values* of visual tokens.

### Design Tokens
CSS custom properties in `src/index.css`. The single source of truth for all visual values: spacing, typography, elevation, motion, transitions, focus rings, opacity, z-index, icon/control sizing, and semantic color aliases.

Tokens are **theme-aware** (dark-mode overrides exist where needed) and **semantic** (named by purpose, not by value).

### Primitives
Low-level UI building blocks in `src/components/ui/`: `button`, `badge`, `checkbox`, `progress`, `separator`, `skeleton`, `tabs`, `tooltip`, `alert`. Primitives consume tokens directly — they never hardcode visual values.

### Components
Composed UI in `src/components/`: `provider-card`, `side-nav`, `panel-footer`, `about-dialog`, `changelog-dialog`, `provider-diagnostics`, `skeleton-lines`, `plugin-error`, `usage-sparkline`, `global-shortcut-section`. Components are built from primitives and tokens.

### Screens
Top-level views in `src/components/app/`: `app-shell`, `app-content`. Screens compose components. **Screens must not introduce visual constants.** If a screen needs a visual value, that value must already be a token or be promoted to a token.

## Rules

1. **No hardcoded visual values in components or screens.**
   - Colors: use `var(--color-*)` or Tailwind utilities (`bg-card`, `text-muted-foreground`, etc.).
   - Sizes: use `var(--space-*)`, `var(--font-size-*)`, `var(--control-*)`, `var(--icon-*)`.
   - Elevation: use `var(--shadow-*)`.
   - Motion: use `var(--transition-*)`, `var(--duration-*)`, `var(--ease-*)`.
   - Z-index: use `var(--z-*)`.

2. **No arbitrary Tailwind values.**
   - `text-[10px]` → use the token (`var(--font-size-2xs)`) or register a Tailwind utility in `@theme inline`.
   - `h-[18px]` → use `var(--control-2xs)`.
   - `shadow-xl` (Tailwind default) → use `var(--shadow-xl)` (PulseBar scale).
   - `#58a6ff` → use `var(--link)`.

3. **No inline style attributes with visual constants.**
   - `style={{ padding: '12px' }}` → use `var(--space-3)` or the Tailwind `p-3` utility.

4. **Exceptions (allowed hardcoded values):**
   - One-off structural values with no visual meaning (e.g., `flex-1`, `w-full`).
   - Values dictated by external specs (e.g., SVG `viewBox`, icon path data).
   - Test fixtures (`*.test.tsx`).

## Enforcement

- **Code review:** every PR touching `src/components/` is reviewed for token compliance.
- **Lint:** a custom lint rule (future PR) will flag arbitrary Tailwind values.
- **Visual regression:** every Program B PR captures screenshots and diffs against the baseline.

## Migration

See `docs/design-system/migration-guide.md` for the current backlog of hardcoded values and their token replacements.
