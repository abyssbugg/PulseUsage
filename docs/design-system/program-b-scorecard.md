# Program B UI Quality Scorecard

Tracks design-system migration progress and quality signals per Program B PR.

| PR | Scope | Components migrated | Hardcoded visual constants removed | Hardcoded visual constants remaining | Token adoption rate | Visual regressions | Accessibility regressions | Bundle size impact | Render performance impact | Notes |
|---|---|---:|---:|---:|---:|---:|---:|---|---|---|
| B1 / #56 | Design token foundation | 0 | 0 | 40 | 0% | 0 | 0 | Neutral | Neutral | Tokens/docs only. |
| Token freeze / #59 | Missing progress-track token | 0 | 0 | 40 | 0% | 0 | 0 | Neutral | Neutral | Added `--progress-track`; token API frozen. |
| B2 | Component system migration | 9 | 38 | 2 | 95% | 0 expected | 0 expected | Minimal | Neutral | Remaining `max-w-[150px]` values are structural chart constraints with no frozen token. |

## Metric Definitions

- **Components migrated:** source components changed to consume design tokens.
- **Hardcoded visual constants removed:** planned migration-guide constants replaced with frozen tokens.
- **Hardcoded visual constants remaining:** planned migration-guide constants not yet replaced.
- **Token adoption rate:** migrated constants divided by total planned constants.
- **Visual regressions:** screenshot/manual review findings.
- **Accessibility regressions:** keyboard, ARIA, focus, contrast, or screen-reader regressions.
- **Bundle size impact:** production bundle delta from `bun run build`.
- **Render performance impact:** expected runtime/rendering impact from changed code.
