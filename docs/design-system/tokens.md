# Design Tokens

Centralized design tokens for PulseBar. Single source of truth for all visual values.

## Location

All tokens are CSS custom properties defined in `src/index.css`:

- **Visual tokens** (colors, radius) — `:root` and `:root.dark` blocks, mapped through `@theme inline` to Tailwind utilities.
- **Foundation tokens** (spacing, typography, elevation, motion, transitions, focus rings, opacity, z-index, icon/control sizing, semantic color aliases) — `:root` "Design System Foundation" block.

## Naming Conventions

| Prefix | Domain | Example |
|---|---|---|
| `--space-*` | Spacing scale | `--space-4` (16px) |
| `--font-*` | Font families | `--font-sans` |
| `--font-size-*` | Font sizes | `--font-size-2xs` (10px) |
| `--leading-*` | Line heights | `--leading-normal` (1.5) |
| `--weight-*` | Font weights | `--weight-semibold` (600) |
| `--radius-*` | Corner radius | `--radius-lg` |
| `--shadow-*` | Elevation | `--shadow-lg` |
| `--duration-*` | Motion durations | `--duration-fast` (100ms) |
| `--ease-*` | Easing curves | `--ease-out` |
| `--transition-*` | Transition presets | `--transition-colors` |
| `--focus-ring-*` | Focus rings | `--focus-ring-width` |
| `--opacity-*` | Opacity scale | `--opacity-60` |
| `--z-*` | Z-index layers | `--z-modal` (50) |
| `--icon-*` | Icon sizing | `--icon-md` (16px) |
| `--control-*` | Control heights | `--control-sm` (28px) |
| `--success/warning/error/info` | Semantic status colors | `--success` |
| `--link` | Link color | `--link` (#58a6ff) |
| `--surface-*` | Semantic surfaces | `--surface-elevated` |

## Token Inventory

### Spacing (4px base)
`--space-0` (0) · `--space-px` (1px) · `--space-half` (2px) · `--space-1` (4px) · `--space-1-and-half` (6px) · `--space-2` (8px) · `--space-2-and-half` (10px) · `--space-3` (12px) · `--space-3-and-half` (14px) · `--space-4` (16px) · `--space-5` (20px) · `--space-6` (24px) · `--space-8` (32px) · `--space-10` (40px) · `--space-12` (48px) · `--space-16` (64px)

### Typography — Font Sizes
`--font-size-2xs` (10px) · `--font-size-xs` (11px) · `--font-size-sm` (12px) · `--font-size-base` (13px) · `--font-size-md` (14px) · `--font-size-lg` (16px) · `--font-size-xl` (20px) · `--font-size-2xl` (24px) · `--font-size-3xl` (30px)

### Typography — Line Heights
`--leading-none` (1) · `--leading-tight` (1.15) · `--leading-snug` (1.3) · `--leading-normal` (1.5) · `--leading-relaxed` (1.625) · `--leading-loose` (2)

### Typography — Weights
`--weight-regular` (400) · `--weight-medium` (500) · `--weight-semibold` (600) · `--weight-bold` (700)

### Elevation
`--shadow-xs` · `--shadow-sm` · `--shadow-md` · `--shadow-lg` · `--shadow-xl` · `--shadow-2xl`

### Motion — Durations
`--duration-instant` (0ms) · `--duration-fast` (100ms) · `--duration-normal` (150ms) · `--duration-default` (200ms) · `--duration-slow` (300ms) · `--duration-slower` (500ms)

### Motion — Easings
`--ease-linear` · `--ease-in` · `--ease-out` · `--ease-in-out` · `--ease-spring`

### Transitions (presets)
`--transition-fast` · `--transition-normal` · `--transition-default` · `--transition-slow` · `--transition-colors` · `--transition-opacity` · `--transition-transform` · `--transition-shadow`

### Focus Rings
`--focus-ring-width` (2px) · `--focus-ring-offset` (2px) · `--focus-ring-color` · `--focus-ring-shadow`

> Note: Focus rings are globally disabled in this menu-bar app. Tokens exist for completeness and future web-view contexts.

### Opacity
`--opacity-0` through `--opacity-100` (step 10)

### Z-Index Layers
`--z-base` (0) · `--z-content` (10) · `--z-sticky` (20) · `--z-dropdown` (30) · `--z-popover` (40) · `--z-modal` (50) · `--z-toast` (60) · `--z-tooltip` (70) · `--z-portal` (80) · `--z-portal-top` (90)

### Icon Sizing
`--icon-2xs` (10px) · `--icon-xs` (12px) · `--icon-sm` (14px) · `--icon-md` (16px) · `--icon-lg` (20px) · `--icon-xl` (24px) · `--icon-2xl` (32px)

### Control Sizing (heights)
`--control-2xs` (18px) · `--control-xs` (22px) · `--control-sm` (28px) · `--control-md` (32px) · `--control-lg` (40px) · `--control-xl` (48px)

### Semantic Colors
`--success` → `--green-500` · `--warning` → `--yellow-500` · `--error` → `--red-500` · `--info` → `--chart-2`
Each has a `-foreground` companion.

### Link Color
`--link` (#58a6ff) · `--link-hover` (lighter variant). Both have dark-mode overrides.

### Semantic Surfaces
`--surface-elevated` → `--card` · `--surface-overlay` → `--popover` · `--surface-sunken` → `--muted`

## When to Introduce New Tokens

1. A visual value is used in **2 or more** components → introduce a token.
2. A value is used once but represents a semantic concept (e.g., "error border") → introduce a token.
3. A value is truly one-off and has no semantic meaning → hardcoding is acceptable.

Do NOT introduce tokens speculatively. Tokens should reflect actual usage, not hypothetical reuse.

## When Reuse Is Required

- **Never** hardcode a value that already exists as a token. Search `src/index.css` before introducing a new constant.
- **Never** copy a token value into a component. Reference the token by `var(--token-name)`.
- **Never** introduce a "close enough" hardcoded value when a token is available. If the token is slightly wrong, adjust the token (not the component) and verify no regressions.

## Validation

Token changes must pass:
- `bun run build` (Tailwind compiles)
- `bun run typecheck`
- `bun run lint`
- Visual regression diff against baseline (see `docs/screenshots/`)
