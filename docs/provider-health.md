# Provider Health

Canonical provider health registry for real providers. `mock` is excluded.

Classification values:
- `required`: expected for a healthy provider shape and fixture-backed.
- `optional`: shown only when an optional source or value exists.
- `planDependent`: tied to tier, entitlement, model set, or provider mode.
- `deprecated`: legacy fallback kept for compatibility.
- `unclassified`: blocked; no docs/tests/live evidence strong enough to classify.

## Amp
- Version: 0.0.1
- Live verified: Yes, 2026-06-07 audit.
- Fixture coverage: All manifest lines covered.
- Risk level: Medium.
- Known evidence gaps: Live audit covered workspace credits; Free and Bonus remain fixture/docs-backed only; Credits can be absent for Free accounts with zero credits.
- Metric classifications: Credits=planDependent; Free=planDependent; Bonus=planDependent.
- Last audit date: 2026-06-07.
- Last validation date: 2026-06-07.
- Notes: Workspace credits gap was fixed before this branch.
- Follow-up actions: Re-audit Free tier and active bonus promotion when available.

## Antigravity
- Version: 0.0.1
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: All manifest lines covered by local/Cloud Code fixtures.
- Risk level: High.
- Known evidence gaps: Dynamic model list; no live account capture in current audit set.
- Metric classifications: Gemini Pro=optional; Gemini Flash=optional; Claude=optional.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Behavior is proven for fixture-backed model groups only.
- Follow-up actions: Capture sanitized live provider shape before promoting any model group to required.

## Claude
- Version: 0.0.1
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: All manifest lines covered.
- Risk level: High.
- Known evidence gaps: Undocumented endpoint shape not live-verified in current audit set.
- Metric classifications: Session=required; Weekly=required; Sonnet=planDependent; Claude Design=planDependent; Extra usage spent=optional; Today=optional; Yesterday=optional; Last 30 Days=optional; Usage Trend=optional.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Local history lines depend on `ccusage`; live usage can be rate-limited.
- Follow-up actions: Add sanitized live audit evidence for current Claude Code accounts.

## Codex
- Version: 0.0.1
- Live verified: Yes, 2026-06-07 Team audit plus local history.
- Fixture coverage: All manifest lines covered.
- Risk level: Medium.
- Known evidence gaps: Spark and review limits were absent in live Team response.
- Metric classifications: Session=required; Weekly=required; Credits=optional; Spark=planDependent; Spark Weekly=planDependent; Reviews=planDependent; Today=optional; Yesterday=optional; Last 30 Days=optional; Usage Trend=optional.
- Last audit date: 2026-06-07.
- Last validation date: 2026-06-07.
- Notes: Parser matched audited Team shape.
- Follow-up actions: Capture sanitized Spark/review-limit responses when available.

## Copilot
- Version: 0.0.1
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: All manifest lines covered for paid/free shapes.
- Risk level: Medium.
- Known evidence gaps: No sanitized live paid/free response in current audit set.
- Metric classifications: Chat=required; Premium=planDependent; Completions=planDependent.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Tests also cover missing snapshot fallback.
- Follow-up actions: Add sanitized live paid and free captures.

## Cursor
- Version: 0.0.1
- Live verified: Partial, 2026-06-07 Free audit; optional Stripe/REST endpoints timed out.
- Fixture coverage: All manifest lines covered.
- Risk level: High.
- Known evidence gaps: Credit/request fallback endpoints lacked successful live capture.
- Metric classifications: Total usage=required; Credits=optional; Auto usage=optional; API usage=optional; Requests=planDependent; On-demand=planDependent.
- Last audit date: 2026-06-07.
- Last validation date: 2026-06-07.
- Notes: Parser matched audited Free shape.
- Follow-up actions: Re-audit team/enterprise and Stripe balance paths.

## Devin
- Version: 0.0.1
- Live verified: No current live audit; provider docs include sanitized observed shape and tests.
- Fixture coverage: All manifest lines covered.
- Risk level: High.
- Known evidence gaps: No current live audit in provider stability set.
- Metric classifications: Weekly quota=required; Daily quota=planDependent; Extra usage balance=optional.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Daily quota is hidden for documented Max-style shape.
- Follow-up actions: Add sanitized live audit for visible daily quota accounts.

## Factory
- Version: 0.0.1
- Live verified: Yes, 2026-06-07 Pro audit.
- Fixture coverage: All manifest lines covered.
- Risk level: Medium.
- Known evidence gaps: Token-rate, Droid Core, Premium, and managed-compute lines were not active in live Pro account.
- Metric classifications: Standard=required; Premium=planDependent; Extra Usage=planDependent; 5-hour usage=planDependent; Weekly usage=planDependent; Monthly usage=planDependent; Droid Core=planDependent; Managed Computers=planDependent.
- Last audit date: 2026-06-07.
- Last validation date: 2026-06-07.
- Notes: Parser matched audited Pro shape.
- Follow-up actions: Capture sanitized Max/Droid Core/managed-compute shapes.

## Grok
- Version: 0.0.1
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: All manifest lines covered.
- Risk level: High.
- Known evidence gaps: No sanitized live billing/settings response in current audit set.
- Metric classifications: Credits used=required; Pay as you go=required.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Pay-as-you-go line is fixture-backed for disabled and enabled states.
- Follow-up actions: Add sanitized live billing capture.

## JetBrains AI Assistant
- Version: 0.0.1
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: All manifest lines covered.
- Risk level: Medium.
- Known evidence gaps: No current live quota XML capture in audit set.
- Metric classifications: Quota=required; Used=required; Remaining=required.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Tests cover raw and normalized quota units.
- Follow-up actions: Add sanitized live quota XML shape after provider update.

## Kimi
- Version: 0.0.1
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: All manifest lines covered.
- Risk level: High.
- Known evidence gaps: No sanitized live `/usages` response in current audit set.
- Metric classifications: Session=optional; Weekly=optional.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Tests prove either line can be omitted depending on quota shape.
- Follow-up actions: Add sanitized live usage capture before marking any Kimi line required.

## Kiro
- Version: 0.0.1
- Live verified: Yes, 2026-06-07 Pro+ audit.
- Fixture coverage: All manifest lines covered.
- Risk level: Medium.
- Known evidence gaps: No active bonus pool in live audit.
- Metric classifications: Credits=required; Bonus Credits=optional; Overages=planDependent.
- Last audit date: 2026-06-07.
- Last validation date: 2026-06-07.
- Notes: Parser matched audited Pro+ shape.
- Follow-up actions: Capture sanitized active bonus/free-trial shape.

## MiniMax
- Version: 0.0.1
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: All manifest lines covered.
- Risk level: Medium.
- Known evidence gaps: No sanitized live global/CN remains response in current audit set.
- Metric classifications: Session=required.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Tests cover global/CN region and reset fallbacks.
- Follow-up actions: Add sanitized live remains response for both regions when available.

## OpenCode Go
- Version: 0.0.1
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: All manifest lines covered.
- Risk level: Medium.
- Known evidence gaps: Local observed spend only; no account-truth API.
- Metric classifications: Session=required; Weekly=required; Monthly=required.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Missing remote/other-device usage is not estimated.
- Follow-up actions: Revisit if OpenCode exposes account usage API.

## Perplexity
- Version: 0.0.2
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: Partial; `Agentic Research` lacks provider docs/test evidence.
- Risk level: High.
- Known evidence gaps: Local cache format, Cloudflare blocking, and unclassified Agentic Research line.
- Metric classifications: API credits=optional; Queries=planDependent; Deep Research=planDependent; Labs=planDependent; Agentic Research=unclassified.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Agentic Research remains in the manifest for compatibility but is intentionally unclassified.
- Follow-up actions: Add docs/tests or live evidence before classifying Agentic Research.

## Synthetic
- Version: 0.0.1
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: All manifest lines covered.
- Risk level: Medium.
- Known evidence gaps: No sanitized live quota response in current audit set.
- Metric classifications: 5h Rate Limit=optional; Mana Bar=optional; Rate Limited=optional; Subscription=deprecated; Free Tool Calls=deprecated; Search=optional.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Subscription and Free Tool Calls are legacy fallback lines.
- Follow-up actions: Add sanitized live v3 and legacy quota captures.

## Warp
- Version: 0.0.1
- Live verified: Yes, 2026-06-07 local state audit.
- Fixture coverage: All manifest lines covered.
- Risk level: Medium.
- Known evidence gaps: Personal/add-on credit fields were absent in live billing metadata.
- Metric classifications: Base Credits=required; Personal Credits=optional; Monthly Spend Limit=optional; Auto-reload=optional; Purchased This Month=optional.
- Last audit date: 2026-06-07.
- Last validation date: 2026-06-07.
- Notes: Parser matched audited local shape.
- Follow-up actions: Capture sanitized billing metadata with add-on credits enabled.

## Z.ai
- Version: 0.0.1
- Live verified: No current live audit; docs/tests only.
- Fixture coverage: All manifest lines covered.
- Risk level: High.
- Known evidence gaps: Undocumented endpoints lack current live audit evidence.
- Metric classifications: Session=required; Weekly=optional; Web Searches=optional.
- Last audit date: None recorded.
- Last validation date: 2026-06-07 fixture/docs validation.
- Notes: Weekly and Web Searches can be absent in fixture-backed shapes.
- Follow-up actions: Add sanitized live subscription/quota capture.
