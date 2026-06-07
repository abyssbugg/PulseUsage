# Warp

> Reverse-engineered from Warp's local macOS state. These fields are not a public API and may change without notice.

## Overview

- macOS provider
- Local-only; no network calls
- Primary usage source: `~/Library/Preferences/dev.warp.Warp-Stable.plist`
- Preview/canary fallbacks: `dev.warp.Warp-Preview.plist`, `dev.warp.Warp-Canary.plist`
- Optional plan source: `~/Library/Group Containers/2BBY89MBSN.dev.warp/Library/Application Support/dev.warp.Warp-Stable/warp.sqlite`

## Plugin metrics

- `Base Credits`
  - `used`: `AIRequestLimitInfo.num_requests_used_since_refresh`
  - `limit`: `AIRequestLimitInfo.limit`
  - `resetsAt`: `AIRequestLimitInfo.next_refresh_time`
  - `periodDurationMs`: derived from `AIRequestQuotaInfoSetting.cycle_history[*].end_date` when available
  - current billing metadata can override this with base credit remaining/limit/reset fields
- Fallback key: `AIAssistantRequestLimitInfo`
- `Personal Credits`
  - from billing metadata (example paths):
    - `personal_credits.remaining`
    - `personalCredits.remaining`
    - `add_on_credits.balance`
    - `addOnCredits.balance`
- `Monthly Spend Limit`
  - from billing metadata fields like `add_on_credits.monthly_spend_limit`
- `Auto-reload`
  - from billing metadata fields like `add_on_credits.auto_reload_enabled`
- `Purchased This Month` (detail)
  - from billing metadata (example paths):
    - `add_on_credits.purchased_this_month`
    - `addOnCredits.purchasedThisMonth`

## Plan label

Usage reads `teams.billing_metadata_json` from `warp.sqlite` and uses `tier.name` when present. Current Warp billing may include the plan, base credits, personal/add-on credits, monthly spend limit, and auto-reload values shown in **Settings → Billing and usage**.

## Provider health

Last audited: 2026-06-07.

Live evidence: local Warp Stable plist contained `AIRequestLimitInfo` and `AIRequestQuotaInfoSetting`; `warp.sqlite` contained billing tier metadata with plan `Max`, but no personal/add-on credit balance, monthly spend limit, auto-reload state, or purchased-this-month fields.

Observed live local shape:

```jsonc
{
  "AIRequestLimitInfo": {
    "limit": 18000,
    "num_requests_used_since_refresh": 1004,
    "next_refresh_time": "2026-07-04T20:27:42Z",
    "request_limit_refresh_duration": "Monthly"
  },
  "AIRequestQuotaInfoSetting": {
    "cycle_history": [
      {
        "end_date": "2025-07-29T15:24:38.322857Z",
        "was_quota_exceeded": false
      }
    ]
  },
  "billing_metadata_json": {
    "tier": {
      "name": "Max",
      "description": "Max tier - Build plan with 18,000 monthly credits.",
      "warp_ai_policy": {
        "limit": 18000
      },
      "purchase_add_on_credits_policy": {
        "enabled": true
      }
    }
  }
}
```

Metric classification:

| Metric | Classification | Evidence |
|---|---|---|
| Base Credits | Required | Present in live plist as `AIRequestLimitInfo`; parser returns this line. |
| Personal Credits | Optional | Live billing metadata had no `personal_credits` or `add_on_credits` balance fields. |
| Monthly Spend Limit | Optional | Live billing metadata had no monthly spend limit field. |
| Auto-reload | Optional | Live billing metadata had no auto-reload state field. |
| Purchased This Month | Optional | Live billing metadata had no purchased-this-month field. |

Audit result: parser matched the observed live shape; no parser code fix was applied.

## Notes

- If Warp has not written request-limit data yet, Usage shows `Warp usage data unavailable. Open Warp and try again.`
- Voice and codebase quota fields are intentionally ignored for now. Base credits and personal/add-on credits are the main billing signals.
