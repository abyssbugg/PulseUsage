# Migration Guide

Current hardcoded values → replacement tokens → migration progress.

## Status Legend

- ⬜ Not migrated (hardcoded value still in source)
- 🔄 Partially migrated (some occurrences replaced)
- ✅ Migrated (all occurrences use tokens)

## Priority Classification

Each hardcoded value is classified by migration priority. Migration PRs proceed P0 → P1 → P2 → P3, not file-by-file. This maximizes consistency gain per PR and minimizes regression surface.

| Priority | Meaning | Values | Migration PR |
|---|---|---|---|
| **P0** | Blocks consistency — colors and spacing used many times | `#58a6ff` (4×), `#353537` (1×), `text-[10px]` (11×) | PR-B2 (colors), PR-B3 (font sizes) |
| **P1** | Common components — typography outliers and control sizing | `text-[11px]` (2×), `text-[13px]` (2×), `h-[18px]` (4×), `h-[22px]` (2×), `h-5` (3×) | PR-B3 (font sizes), PR-B4 (control heights) |
| **P2** | Rare component values — elevation and motion | `shadow-xl/lg/md/2xl` (4×), `duration-200` (3×) | PR-B5 (elevation), PR-B6 (motion) |
| **P3** | Nice-to-have cleanup — widths and one-off values | `min-w-[2px]` (2×), `max-w-[150px]` (2×), `w-[92%]`/`h-[88%]` (structural) | PR-B7 (widths) |

**Rationale:** P0 values are the highest-impact, most-repeated hardcoded values. Migrating them first delivers the largest consistency improvement with the smallest regression risk. P3 cleanup can be deferred without blocking feature work.

---

## Hardcoded Color Values

### `#58a6ff` (link blue)

**Token:** `--link` / `--link-hover`

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/changelog-dialog.tsx` | 86 | `text-[#58a6ff]` | `text-[var(--link)]` | ⬜ |
| `src/components/changelog-dialog.tsx` | 244 | `text-[#58a6ff]` | `text-[var(--link)]` | ⬜ |
| `src/components/changelog-dialog.tsx` | 260 | `text-[#58a6ff]` | `text-[var(--link)]` | ⬜ |
| `src/components/changelog-dialog.tsx` | 274 | `text-[#58a6ff]` | `text-[var(--link)]` | ⬜ |

### `#353537` (progress dark bg)

**Token:** needs alias (currently `dark:bg-[#353537]` — add `--surface-sunken-dark` or reuse `--muted`)

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/ui/progress.tsx` | 45 | `dark:bg-[#353537]` | `dark:bg-[var(--surface-sunken)]` or new token | ⬜ |

### `#ffffff` / `#000000` (side-nav contrast tests)

**Token:** N/A (test assertions, not visual source)

| File | Line | Current | Notes | Status |
|---|---|---|---|---|
| `src/components/side-nav.test.tsx` | — | `#ffffff`, `#000000` | Test fixtures, not source | ✅ N/A |

---

## Arbitrary Font Sizes

### `text-[10px]`

**Token:** `--font-size-2xs`

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/about-dialog.tsx` | 105 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |
| `src/components/provider-diagnostics.tsx` | 111 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |
| `src/components/provider-diagnostics.tsx` | 132 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |
| `src/components/changelog-dialog.tsx` | 230 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |
| `src/components/changelog-dialog.tsx` | 244 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |
| `src/components/changelog-dialog.tsx` | 256 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |
| `src/components/usage-sparkline.tsx` | 69 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |
| `src/components/usage-sparkline.tsx` | 95 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |
| `src/components/usage-sparkline.tsx` | 99 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |
| `src/components/provider-card.tsx` | 79 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |
| `src/components/provider-card.tsx` | 399 | `text-[10px]` | `text-[var(--font-size-2xs)]` | ⬜ |

### `text-[11px]`

**Token:** `--font-size-xs`

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/provider-diagnostics.tsx` | 93 | `text-[11px]` | `text-[var(--font-size-xs)]` | ⬜ |
| `src/components/provider-card.tsx` | 286 | `text-[11px]` | `text-[var(--font-size-xs)]` | ⬜ |

### `text-[13px]`

**Token:** `--font-size-base`

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/changelog-dialog.tsx` | 160 | `text-[13px]` | `text-[var(--font-size-base)]` | ⬜ |
| `src/components/changelog-dialog.tsx` | 168 | `text-[13px]` | `text-[var(--font-size-base)]` | ⬜ |

---

## Arbitrary Control Heights

### `h-[18px]`

**Token:** `--control-2xs` (18px)

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/skeleton-lines.tsx` | 8 | `h-[18px]` | `h-[var(--control-2xs)]` | ⬜ |
| `src/components/skeleton-lines.tsx` | 39 | `h-[18px]` | `h-[var(--control-2xs)]` | ⬜ |
| `src/components/usage-sparkline.tsx` | 48 | `h-[18px]` | `h-[var(--control-2xs)]` | ⬜ |
| `src/components/provider-card.tsx` | 386 | `h-[18px]` | `h-[var(--control-2xs)]` | ⬜ |
| `src/components/skeleton-lines.test.tsx` | 31, 32 | `h-[18px]` | Test assertion | ✅ N/A |

### `h-[22px]`

**Token:** `--control-xs` (22px)

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/skeleton-lines.tsx` | 17 | `h-[22px]` | `h-[var(--control-xs)]` | ⬜ |
| `src/components/provider-card.tsx` | 412 | `h-[22px]` | `h-[var(--control-xs)]` | ⬜ |

### `h-5` (Tailwind default = 20px)

**Token:** `--control-2xs` (18px) is closest; evaluate per-context.

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/about-dialog.tsx` | 105 | `h-5` | `h-[var(--control-2xs)]` | ⬜ |
| `src/components/provider-diagnostics.tsx` | 111 | `h-5` | `h-[var(--control-2xs)]` | ⬜ |
| `src/components/provider-diagnostics.tsx` | 132 | `h-5` | `h-[var(--control-2xs)]` | ⬜ |

---

## Arbitrary Width Values

### `min-w-[2px]`, `max-w-[150px]`, `w-[92%]`, `h-[88%]`

**Token:** `--space-half` (2px) for `min-w-[2px]`; others are structural, evaluate per-context.

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/skeleton-lines.tsx` | 45 | `min-w-[2px]` | `min-w-[var(--space-half)]` | ⬜ |
| `src/components/usage-sparkline.tsx` | 58 | `min-w-[2px]` | `min-w-[var(--space-half)]` | ⬜ |
| `src/components/skeleton-lines.tsx` | 41 | `max-w-[150px]` | structural — keep or token | ⬜ |
| `src/components/usage-sparkline.tsx` | 53 | `max-w-[150px]` | structural — keep or token | ⬜ |
| `src/components/changelog-dialog.tsx` | 197 | `w-[92%]`, `h-[88%]` | structural — keep | ✅ N/A |

---

## Elevation (Tailwind default shadows)

### `shadow-xl`, `shadow-lg`, `shadow-md`, `shadow-2xl`

**Token:** `--shadow-xl`, `--shadow-lg`, `--shadow-md`, `--shadow-2xl`

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/about-dialog.tsx` | 88 | `shadow-xl` | `shadow-[var(--shadow-xl)]` | ⬜ |
| `src/components/changelog-dialog.tsx` | 197 | `shadow-2xl` | `shadow-[var(--shadow-2xl)]` | ⬜ |
| `src/components/app/app-shell.tsx` | 77 | `shadow-lg` | `shadow-[var(--shadow-lg)]` | ⬜ |
| `src/components/ui/tooltip.tsx` | 55 | `shadow-md` | `shadow-[var(--shadow-md)]` | ⬜ |

---

## Motion

### `duration-200`

**Token:** `--duration-default` (200ms)

| File | Line | Current | Replacement | Status |
|---|---|---|---|---|
| `src/components/about-dialog.tsx` | 88 | `duration-200` | `duration-[var(--duration-default)]` | ⬜ |
| `src/components/changelog-dialog.tsx` | 197 | `duration-200` | `duration-[var(--duration-default)]` | ⬜ |
| `src/components/app/app-shell.tsx` | 100 | `duration-200` | `duration-[var(--duration-default)]` | ⬜ |

---

## Migration Progress Summary

| Category | Total | Migrated | Remaining |
|---|---|---|---|
| Link color (`#58a6ff`) | 4 | 0 | 4 |
| Progress dark bg (`#353537`) | 1 | 0 | 1 |
| Font size `text-[10px]` | 11 | 0 | 11 |
| Font size `text-[11px]` | 2 | 0 | 2 |
| Font size `text-[13px]` | 2 | 0 | 2 |
| Control height `h-[18px]` | 4 | 0 | 4 |
| Control height `h-[22px]` | 2 | 0 | 2 |
| Control height `h-5` | 3 | 0 | 3 |
| Width `min-w-[2px]` | 2 | 0 | 2 |
| Width `max-w-[150px]` | 2 | 0 | 2 |
| Elevation | 4 | 0 | 4 |
| Motion `duration-200` | 3 | 0 | 3 |
| **Total** | **40** | **0** | **40** |

## Migration Order (Future PRs)

Migration proceeds by **priority** (P0 → P1 → P2 → P3), not by file. This ordering ensures the highest-impact, most-repeated values are migrated first, maximizing consistency gain per PR and minimizing regression surface.

### P0 — Blocks consistency (PR-B2, PR-B3)

1. **PR-B2:** Color tokens — `#58a6ff` → `--link`, `#353537` → surface token. Lowest risk, highest semantic value. 5 occurrences.
2. **PR-B3:** Font size `text-[10px]` → `--font-size-2xs`. 11 occurrences — highest count, most consistency impact.

### P1 — Common components (PR-B3, PR-B4)

3. **PR-B3:** Font sizes `text-[11px]` → `--font-size-xs`, `text-[13px]` → `--font-size-base`. 4 occurrences.
4. **PR-B4:** Control heights — `h-[18px]` → `--control-2xs`, `h-[22px]` → `--control-xs`, `h-5` → `--control-2xs`. 9 occurrences.

### P2 — Rare component values (PR-B5, PR-B6)

5. **PR-B5:** Elevation — Tailwind shadow utilities → `--shadow-*` tokens. 4 occurrences.
6. **PR-B6:** Motion — `duration-200` → `--duration-default`. 3 occurrences.

### P3 — Nice-to-have cleanup (PR-B7)

7. **PR-B7:** Width values — `min-w-[2px]` → `--space-half`; evaluate `max-w-[150px]` (may be structural, keep or token). 4 occurrences (2 structural, 2 token-eligible).

### Notes

- Each migration PR captures before/after screenshots to verify zero visual regressions.
- A migration PR may be deferred if its priority bucket is low-impact and other Program B work takes precedence.
- The goal is not "100% migrated" — it is "the system is now the obvious way to build UI." P0 and P1 must complete; P2 and P3 may be deferred if remaining debt is low-impact.
