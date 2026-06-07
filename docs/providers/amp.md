# Amp

## Overview

- **Protocol:** JSON-RPC (`POST /api/internal`)
- **URL:** `https://ampcode.com/api/internal`
- **Auth:** API key from Amp CLI (`~/.local/share/amp/secrets.json`)
- **Tier:** Free (daily replenishing quota) and/or individual credits

## Authentication

### Credential Source

The plugin reads the API key automatically from `~/.local/share/amp/secrets.json`, created by Amp CLI when you sign in. No manual setup required.

```json
{
  "apiKey@https://ampcode.com/": "sgamp_user_..."
}
```

The key is sent as `Authorization: Bearer <key>` to the JSON-RPC API.

## Data Source

### API Endpoint

```
POST https://ampcode.com/api/internal
Authorization: Bearer <api_key>
Content-Type: application/json

{"method": "userDisplayBalanceInfo", "params": {}}
```

### Response

The response contains a `displayText` string whose contents vary by user tier:

**Free tier + credits:**
```
Signed in as <user>
Amp Free: $<remaining>/$<total> remaining (replenishes +$<rate>/hour) [optional: +N% bonus for N more days] - https://ampcode.com/settings#amp-free
Individual credits: $<credits> remaining - https://ampcode.com/settings
```

**Paid credits only:**
```
Signed in as <user>
Individual credits: $<credits> remaining - https://ampcode.com/settings
```
**Workspace credits only:**
```
Signed in as <user>
Workspace <name>: $<credits> remaining (replenishes automatically) - https://ampcode.com/workspaces/<name>
```

The plugin parses the display text with regex to extract:
- **Balance:** `$remaining/$total remaining` → dollar amounts (only if Amp Free enabled)
- **Rate:** `replenishes +$rate/hour` → replenishment speed (only if Amp Free enabled)
- **Bonus:** `[+N% bonus for N more days]` → optional promotional bonus 
- **Credits:** `Individual credits: $N remaining` or `Workspace <name>: $N remaining` → paid/workspace credits balance

## Provider health

Last audited: 2026-06-07.

Live evidence: Amp secrets file existed and the JSON-RPC `userDisplayBalanceInfo` call returned HTTP 200. The live display text used a workspace credits-only shape, not the older `Individual credits` shape.

Observed workspace live response shape:

```jsonc
{
  "ok": true,
  "result": {
    "displayText": "Signed in as <redacted>\nWorkspace <name>: $3.88 remaining (replenishes automatically) - https://ampcode.com/workspaces/<name>"
  }
}
```

Metric classification:

| Metric | Classification | Evidence |
|--------|----------------|----------|
| Credits | Required | Present in live workspace response as `Workspace <name>: $3.88 remaining`; parser now returns this line. |
| Free | Plan-dependent | Live response did not contain `Amp Free`; parser returns this only when free-tier balance text is present. |
| Bonus | Plan-dependent | Live response did not contain a bonus bracket; parser returns this only for active Free-tier promotions. |

Audit result: parser had a proven workspace credits gap. Fixed by parsing `Workspace <name>: $N remaining` as credits.

### Usage Calculation (Free tier only)

- **Used:** `total - remaining` (clamped to 0 minimum)
- **Reset time:** `used / hourlyRate` hours from now (null if nothing used or rate is zero)
- **Period:** 24 hours (fixed)

## Plan Detection

| Condition | Plan |
|-----------|------|
| Free tier present (with or without credits) | `"Free"` |
| Credits only (no free tier) | `"Credits"` |

## Displayed Lines

| Line       | Scope    | Condition                   | Description                            |
|------------|----------|-----------------------------|----------------------------------------|
| Free       | overview | Amp Free enabled            | Dollar amount consumed as progress bar |
| Bonus      | detail   | Amp Free + active promotion | Bonus percentage and duration          |
| Credits    | overview | Credits > $0, or credits-only accounts | Individual credits balance      |

Progress line includes:
- `resetsAt` — ISO timestamp of estimated full replenishment (null if nothing used or rate is zero)
- `periodDurationMs` — 24 hours for pace tracking

## Errors

| Condition              | Message                                                        |
|------------------------|----------------------------------------------------------------|
| Amp not installed      | "Amp not installed. Install Amp Code to get started."          |
| 401/403                | "Session expired. Re-authenticate in Amp Code."               |
| Non-2xx with detail    | Error message from API response                                |
| Non-2xx without detail | "Request failed (HTTP {status}). Try again later."             |
| Unparseable response   | "Could not parse usage data."                                  |
| Network error          | "Request failed. Check your connection."                       |
