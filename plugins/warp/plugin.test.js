import { readFileSync } from "node:fs"
import { beforeEach, describe, expect, it, vi } from "vitest"
import { makeCtx } from "../test-helpers.js"

const PREF_PATH = "~/Library/Preferences/dev.warp.Warp-Stable.plist"
const DB_PATH =
  "~/Library/Group Containers/2BBY89MBSN.dev.warp/Library/Application Support/dev.warp.Warp-Stable/warp.sqlite"

const loadPlugin = async () => {
  await import("./plugin.js")
  return globalThis.__pulseusage_plugin
}

function setPrefs(ctx, prefs, path = PREF_PATH) {
  ctx.host.fs.writeText(path, JSON.stringify(prefs))
}

function setPlan(ctx, planName, path = DB_PATH) {
  setBilling(ctx, { tier: { name: planName } }, path)
}

function setBilling(ctx, billingMetadata, path = DB_PATH) {
  ctx.host.fs.writeText(path, "sqlite")
  ctx.host.sqlite.query.mockImplementation((dbPath, sql) => {
    expect(dbPath).toBe(path)
    expect(String(sql)).toContain("SELECT billing_metadata_json FROM teams")
    return JSON.stringify([
      {
        billing_metadata_json: JSON.stringify(billingMetadata),
      },
    ])
  })
}

describe("warp plugin", () => {
  beforeEach(() => {
    delete globalThis.__pulseusage_plugin
    vi.resetModules()
  })

  it("ships plugin metadata with the expected line layout", () => {
    const manifest = JSON.parse(readFileSync("plugins/warp/plugin.json", "utf8"))

    expect(manifest.id).toBe("warp")
    expect(manifest.name).toBe("Warp")
    expect(manifest.brandColor).toBe("#353534")
    expect(manifest.lines).toEqual([
      {
        type: "progress",
        label: "Base Credits",
        scope: "overview",
        classification: "required",
        primaryOrder: 1,
      },
      {
        type: "text",
        label: "Personal Credits",
        scope: "overview",
        classification: "optional",
      },
      {
        type: "text",
        label: "Monthly Spend Limit",
        scope: "detail",
        classification: "optional",
      },
      {
        type: "badge",
        label: "Auto-reload",
        scope: "detail",
        classification: "optional",
      },
      {
        type: "text",
        label: "Purchased This Month",
        scope: "detail",
        classification: "optional",
      },
    ])
  })

  it("throws when Warp is not detected", async () => {
    const ctx = makeCtx()
    const plugin = await loadPlugin()

    expect(() => plugin.probe(ctx)).toThrow(
      "Warp not detected. Open Warp and try again."
    )
  })

  it("reads request limits from plist and plan from sqlite", async () => {
    const ctx = makeCtx()
    setPrefs(ctx, {
      AIRequestLimitInfo: {
        limit: 18000,
        num_requests_used_since_refresh: 1004,
        next_refresh_time: "2026-05-04T20:27:42Z",
        request_limit_refresh_duration: "Monthly",
      },
      AIRequestQuotaInfoSetting: {
        cycle_history: [
          { end_date: "2026-03-04T20:27:42Z" },
          { end_date: "2026-04-04T20:27:42Z" },
        ],
      },
    })
    setPlan(ctx, "Max")

    const plugin = await loadPlugin()
    const result = plugin.probe(ctx)

    expect(result.plan).toBe("Max")

    const requestsLine = result.lines.find((line) => line.label === "Base Credits")
    expect(requestsLine).toBeTruthy()
    expect(requestsLine.used).toBe(1004)
    expect(requestsLine.limit).toBe(18000)
    expect(requestsLine.format).toEqual({ kind: "count", suffix: "credits" })
    expect(requestsLine.resetsAt).toBe("2026-05-04T20:27:42.000Z")
    expect(requestsLine.periodDurationMs).toBe(30 * 24 * 60 * 60 * 1000)

  })

  it("handles current live local shape with tier metadata but no add-on balances", async () => {
    const ctx = makeCtx()
    setPrefs(ctx, {
      AIRequestLimitInfo: {
        embedding_generation_batch_size: 25,
        is_unlimited: false,
        is_unlimited_codebase_indices: false,
        is_unlimited_voice: false,
        limit: 18000,
        max_codebase_indices: 5,
        max_files_per_repo: 1000,
        next_refresh_time: "2026-07-04T20:27:42Z",
        num_requests_used_since_refresh: 1004,
        request_limit_refresh_duration: "Monthly",
        voice_request_limit: 50,
        voice_requests_used_since_last_refresh: 0,
      },
      AIRequestQuotaInfoSetting: {
        cycle_history: [
          { end_date: "2026-05-04T20:27:42Z", was_quota_exceeded: false },
          { end_date: "2026-06-04T20:27:42Z", was_quota_exceeded: false },
        ],
      },
    })
    setBilling(ctx, {
      customer_type: "Individual",
      delinquency_status: "NoDelinquency",
      tier: {
        name: "Max",
        description: "Max tier - Build plan with 18,000 monthly credits.",
        warp_ai_policy: {
          limit: 18000,
          is_voice_enabled: true,
        },
        purchase_add_on_credits_policy: {
          enabled: true,
        },
        usage_based_pricing_policy: {
          toggleable: false,
        },
      },
    })

    const plugin = await loadPlugin()
    const result = plugin.probe(ctx)

    expect(result.plan).toBe("Max")
    expect(result.lines.map((line) => line.label)).toEqual(["Base Credits"])
    expect(result.lines.find((line) => line.label === "Base Credits")).toMatchObject({
      type: "progress",
      used: 1004,
      limit: 18000,
      format: { kind: "count", suffix: "credits" },
      resetsAt: "2026-07-04T20:27:42.000Z",
    })
  })

  it("parses live plist values when request info is stored as JSON strings", async () => {
    const ctx = makeCtx()
    setPrefs(ctx, {
      AIRequestLimitInfo: JSON.stringify({
        limit: 18000,
        num_requests_used_since_refresh: 1505,
        next_refresh_time: "2026-05-04T20:27:42Z",
        request_limit_refresh_duration: "Monthly",
      }),
      AIRequestQuotaInfoSetting: JSON.stringify({
        cycle_history: [
          { end_date: "2026-03-04T20:27:42Z" },
          { end_date: "2026-04-04T20:27:42Z" },
        ],
      }),
    })

    const plugin = await loadPlugin()
    const result = plugin.probe(ctx)

    const requestsLine = result.lines.find((line) => line.label === "Base Credits")
    expect(requestsLine.used).toBe(1505)
    expect(requestsLine.limit).toBe(18000)
    expect(requestsLine.resetsAt).toBe("2026-05-04T20:27:42.000Z")
    expect(requestsLine.periodDurationMs).toBe(30 * 24 * 60 * 60 * 1000)

  })

  it("falls back to assistant request info when the main key is missing", async () => {
    const ctx = makeCtx()
    setPrefs(ctx, {
      AIAssistantRequestLimitInfo: {
        limit: 250,
        num_requests_used_since_refresh: 12,
        next_refresh_time: "2026-05-01T00:00:00Z",
        request_limit_refresh_duration: "Weekly",
      },
    })

    const plugin = await loadPlugin()
    const result = plugin.probe(ctx)

    const requestsLine = result.lines.find((line) => line.label === "Base Credits")
    expect(requestsLine.used).toBe(12)
    expect(requestsLine.limit).toBe(250)
    expect(requestsLine.periodDurationMs).toBe(7 * 24 * 60 * 60 * 1000)

  })

  it("keeps usage working when plan lookup fails", async () => {
    const ctx = makeCtx()
    setPrefs(ctx, {
      AIRequestLimitInfo: {
        limit: 100,
        num_requests_used_since_refresh: 40,
        next_refresh_time: "2026-05-01T00:00:00Z",
      },
    })
    ctx.host.fs.writeText(DB_PATH, "sqlite")
    ctx.host.sqlite.query.mockImplementation(() => {
      throw new Error("boom")
    })

    const plugin = await loadPlugin()
    const result = plugin.probe(ctx)

    expect(result.plan).toBeNull()
    expect(result.lines.find((line) => line.label === "Base Credits")).toBeTruthy()
    expect(ctx.host.log.warn).toHaveBeenCalled()
  })

  it("reads current Billing and Usage credit metrics from billing metadata", async () => {
    const ctx = makeCtx()
    setPrefs(ctx, {
      AIRequestLimitInfo: {
        limit: 18000,
        num_requests_used_since_refresh: 1004,
        next_refresh_time: "2026-05-04T20:27:42Z",
        request_limit_refresh_duration: "Monthly",
      },
    })
    setBilling(ctx, {
      tier: { name: "Max" },
      base_credits: {
        remaining: 13422,
        limit: 18000,
        resets_at: "2026-07-04T13:27:00Z",
      },
      personal_credits: {
        remaining: 668,
        expires_at: "2027-05-22T00:00:00Z",
      },
      add_on_credits: {
        monthly_spend_limit: 1000,
        auto_reload_enabled: false,
        purchased_this_month: 0,
      },
    })

    const plugin = await loadPlugin()
    const result = plugin.probe(ctx)

    expect(result.plan).toBe("Max")
    expect(result.lines.find((line) => line.label === "Base Credits")).toMatchObject({
      type: "progress",
      label: "Base Credits",
      used: 4578,
      limit: 18000,
      format: { kind: "count", suffix: "credits" },
      resetsAt: "2026-07-04T13:27:00.000Z",
    })
    expect(result.lines.find((line) => line.label === "Personal Credits")).toEqual({
      type: "text",
      label: "Personal Credits",
      value: "668 remaining",
      subtitle: "Expires May 22, 2027",
    })
    expect(result.lines.find((line) => line.label === "Monthly Spend Limit")).toEqual({
      type: "text",
      label: "Monthly Spend Limit",
      value: "$1000.00",
    })
    expect(result.lines.find((line) => line.label === "Auto-reload")).toEqual({
      type: "badge",
      label: "Auto-reload",
      text: "Disabled",
      color: "#a3a3a3",
    })
    expect(result.lines.find((line) => line.label === "Purchased This Month")).toEqual({
      type: "text",
      label: "Purchased This Month",
      value: "$0.00",
    })
  })

  it("throws when the plist exists but request data is missing", async () => {
    const ctx = makeCtx()
    setPrefs(ctx, {
      AIRequestQuotaInfoSetting: {
        cycle_history: [{ end_date: "2026-04-04T20:27:42Z" }],
      },
    })

    const plugin = await loadPlugin()
    expect(() => plugin.probe(ctx)).toThrow(
      "Warp usage data unavailable. Open Warp and try again."
    )
  })
})
