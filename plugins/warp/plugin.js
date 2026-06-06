(function () {
  const PLIST_PATHS = [
    "~/Library/Preferences/dev.warp.Warp-Stable.plist",
    "~/Library/Preferences/dev.warp.Warp-Preview.plist",
    "~/Library/Preferences/dev.warp.Warp-Canary.plist",
  ]
  const SQLITE_PATHS = [
    "~/Library/Group Containers/2BBY89MBSN.dev.warp/Library/Application Support/dev.warp.Warp-Stable/warp.sqlite",
    "~/Library/Group Containers/2BBY89MBSN.dev.warp/Library/Application Support/dev.warp.Warp-Preview/warp.sqlite",
    "~/Library/Group Containers/2BBY89MBSN.dev.warp/Library/Application Support/dev.warp.Warp-Canary/warp.sqlite",
  ]
  const PLAN_SQL =
    "SELECT billing_metadata_json FROM teams WHERE billing_metadata_json IS NOT NULL AND billing_metadata_json != '' ORDER BY ROWID DESC LIMIT 1;"
  const REQUEST_INFO_KEYS = ["AIRequestLimitInfo", "AIAssistantRequestLimitInfo"]
  const REFRESH_DURATION_MS = {
    hourly: 60 * 60 * 1000,
    daily: 24 * 60 * 60 * 1000,
    weekly: 7 * 24 * 60 * 60 * 1000,
    monthly: 30 * 24 * 60 * 60 * 1000,
    yearly: 365 * 24 * 60 * 60 * 1000,
  }

  function isObject(value) {
    return !!value && typeof value === "object" && !Array.isArray(value)
  }

  function parseJsonLike(ctx, value) {
    if (isObject(value) || Array.isArray(value)) return value
    return ctx.util.tryParseJson(value)
  }

  function toNumber(value) {
    if (value === null || value === undefined || value === "") return null
    const num = Number(value)
    return Number.isFinite(num) ? num : null
  }

  function pickNumber(obj, keys) {
    for (let i = 0; i < keys.length; i += 1) {
      const num = toNumber(obj[keys[i]])
      if (num !== null) return num
    }
    return null
  }

  function readPrefs(ctx) {
    let sawFile = false

    for (let i = 0; i < PLIST_PATHS.length; i += 1) {
      const path = PLIST_PATHS[i]
      if (!ctx.host.fs.exists(path)) continue
      sawFile = true

      let prefs = null

      try {
        prefs = parseJsonLike(ctx, ctx.host.fs.readText(path))
      } catch (e) {
        ctx.host.log.warn("plist text read failed for " + path + ": " + String(e))
      }

      if (
        !isObject(prefs) &&
        ctx.host.plist &&
        typeof ctx.host.plist.read === "function"
      ) {
        try {
          prefs = parseJsonLike(ctx, ctx.host.plist.read(path))
        } catch (e) {
          ctx.host.log.warn("plist parser read failed for " + path + ": " + String(e))
        }
      }

      if (isObject(prefs)) {
        return { path, prefs, sawFile: true }
      }

      ctx.host.log.warn("plist returned invalid json: " + path)
    }

    return { path: null, prefs: null, sawFile }
  }

  function normalizeRequestLimitInfo(ctx, raw) {
    const value = parseJsonLike(ctx, raw)
    if (!isObject(value)) return null

    const limit = pickNumber(value, ["limit", "request_limit"])
    const used = pickNumber(value, [
      "num_requests_used_since_refresh",
      "requests_used_since_last_refresh",
      "requestsUsedSinceLastRefresh",
    ])

    if (limit === null || limit <= 0 || used === null) return null

    return {
      limit,
      used: used < 0 ? 0 : used,
      resetsAt: value.next_refresh_time || value.nextRefreshTime || null,
      duration:
        value.request_limit_refresh_duration ||
        value.requestLimitRefreshDuration ||
        null,
    }
  }

  function pickRequestLimitInfo(ctx, prefs) {
    for (let i = 0; i < REQUEST_INFO_KEYS.length; i += 1) {
      const info = normalizeRequestLimitInfo(ctx, prefs[REQUEST_INFO_KEYS[i]])
      if (info) return info
    }
    return null
  }

  function collectCycleEndTimes(ctx, prefs) {
    const quotaInfo = parseJsonLike(ctx, prefs.AIRequestQuotaInfoSetting)
    if (!isObject(quotaInfo) || !Array.isArray(quotaInfo.cycle_history)) {
      return []
    }

    const seen = Object.create(null)
    const endTimes = []

    for (let i = 0; i < quotaInfo.cycle_history.length; i += 1) {
      const entry = quotaInfo.cycle_history[i]
      if (!isObject(entry)) continue

      const endMs = ctx.util.parseDateMs(entry.end_date || entry.endDate)
      if (!Number.isFinite(endMs)) continue

      const key = String(endMs)
      if (seen[key]) continue
      seen[key] = true
      endTimes.push(endMs)
    }

    endTimes.sort((a, b) => a - b)
    return endTimes
  }

  function derivePeriodDurationMs(ctx, prefs, info) {
    const nextRefreshMs = ctx.util.parseDateMs(info.resetsAt)
    const cycleEnds = collectCycleEndTimes(ctx, prefs)

    if (Number.isFinite(nextRefreshMs)) {
      let previousEndMs = null

      for (let i = 0; i < cycleEnds.length; i += 1) {
        const endMs = cycleEnds[i]
        if (endMs < nextRefreshMs) {
          previousEndMs = endMs
        }
      }

      if (
        previousEndMs !== null &&
        Number.isFinite(previousEndMs) &&
        nextRefreshMs > previousEndMs
      ) {
        return nextRefreshMs - previousEndMs
      }
    }

    if (cycleEnds.length >= 2) {
      const latest = cycleEnds[cycleEnds.length - 1]
      const previous = cycleEnds[cycleEnds.length - 2]
      if (latest > previous) return latest - previous
    }

    if (typeof info.duration !== "string") return null
    return REFRESH_DURATION_MS[String(info.duration).trim().toLowerCase()] || null
  }

  function formatCycleLabel(value) {
    if (typeof value !== "string") return null
    const text = value.trim()
    if (!text) return null
    return text
      .replace(/[_-]+/g, " ")
      .replace(/\b([a-z])/g, function (_, letter) {
        return letter.toUpperCase()
      })
  }

  function readPathValue(obj, path) {
    let current = obj
    for (let i = 0; i < path.length; i += 1) {
      if (!isObject(current)) return null
      current = current[path[i]]
    }
    return current
  }

  function pickBillingNumber(metadata, paths) {
    for (let i = 0; i < paths.length; i += 1) {
      const spec = Array.isArray(paths[i]) ? { path: paths[i] } : paths[i]
      const value = readPathValue(metadata, spec.path)
      const num = toNumber(value)
      if (num !== null) return spec.cents ? num / 100 : num
    }
    return null
  }

  function pickBillingValue(metadata, paths) {
    for (let i = 0; i < paths.length; i += 1) {
      const value = readPathValue(metadata, paths[i])
      if (value !== undefined && value !== null && value !== "") return value
    }
    return null
  }

  function pickBillingBoolean(metadata, paths) {
    const value = pickBillingValue(metadata, paths)
    if (typeof value === "boolean") return value
    if (typeof value === "string") {
      const text = value.trim().toLowerCase()
      if (text === "true" || text === "enabled" || text === "on") return true
      if (text === "false" || text === "disabled" || text === "off") return false
    }
    return null
  }

  function formatDollars(value) {
    const num = toNumber(value)
    if (num === null) return null
    return "$" + num.toFixed(2)
  }

  function formatCount(value) {
    const num = toNumber(value)
    if (num === null) return null
    return String(Math.round(num)).replace(/\B(?=(\d{3})+(?!\d))/g, ",")
  }

  function formatDateLabel(ctx, prefix, value) {
    const iso = ctx.util.toIso(value)
    if (!iso) return null
    const d = new Date(iso)
    if (!Number.isFinite(d.getTime())) return null
    const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
    return prefix + " " + months[d.getUTCMonth()] + " " + String(d.getUTCDate()) + ", " + String(d.getUTCFullYear())
  }

  function emptyBilling() {
    return {
      plan: null,
      baseRemaining: null,
      baseLimit: null,
      baseUsed: null,
      baseResetsAt: null,
      personalRemaining: null,
      personalExpiresAt: null,
      monthlySpendLimit: null,
      autoReloadEnabled: null,
      purchasedThisMonth: null,
    }
  }

  function extractBilling(metadata) {
    if (!isObject(metadata)) return emptyBilling()

    const tier = isObject(metadata.tier) ? metadata.tier : null
    const rawName = tier && typeof tier.name === "string" ? tier.name : null
    const plan = rawName && rawName.trim() ? rawName.trim() : null

    const baseRemaining = pickBillingNumber(metadata, [
      ["base_credits", "remaining"],
      ["baseCredits", "remaining"],
      ["credits", "base", "remaining"],
      ["monthly_credits", "remaining"],
      ["monthlyCredits", "remaining"],
      ["plan_credits", "remaining"],
      ["planCredits", "remaining"],
      ["balance", "base_credits", "remaining"],
      ["base_credits_remaining"],
      ["baseCreditsRemaining"],
    ])

    const baseLimit = pickBillingNumber(metadata, [
      ["base_credits", "limit"],
      ["base_credits", "total"],
      ["base_credits", "allowance"],
      ["baseCredits", "limit"],
      ["baseCredits", "total"],
      ["baseCredits", "allowance"],
      ["credits", "base", "limit"],
      ["monthly_credits", "limit"],
      ["monthlyCredits", "limit"],
      ["plan_credits", "limit"],
      ["planCredits", "limit"],
      ["base_credits_limit"],
      ["baseCreditsLimit"],
    ])

    const baseUsed = pickBillingNumber(metadata, [
      ["base_credits", "used"],
      ["baseCredits", "used"],
      ["credits", "base", "used"],
      ["monthly_credits", "used"],
      ["monthlyCredits", "used"],
      ["plan_credits", "used"],
      ["planCredits", "used"],
      ["base_credits_used"],
      ["baseCreditsUsed"],
    ])

    const baseResetsAt = pickBillingValue(metadata, [
      ["base_credits", "resets_at"],
      ["base_credits", "reset_at"],
      ["base_credits", "period_end"],
      ["baseCredits", "resetsAt"],
      ["baseCredits", "resetAt"],
      ["baseCredits", "periodEnd"],
      ["credits", "base", "resets_at"],
      ["monthly_credits", "resets_at"],
      ["monthlyCredits", "resetsAt"],
      ["plan_credits", "resets_at"],
      ["planCredits", "resetsAt"],
      ["billing_period", "end"],
      ["billingPeriod", "end"],
      ["current_period_end"],
      ["currentPeriodEnd"],
    ])

    const addOnCredit = pickBillingNumber(metadata, [
      ["add_on_credits", "balance"],
      ["add_on_credits", "remaining"],
      ["addons", "credits", "balance"],
      ["credits", "add_on", "balance"],
      ["addOnCredits", "balance"],
      ["addOnCredits", "remaining"],
      ["add_on_credits_balance"],
      ["add_on_credit_balance"],
      ["addon_credits_balance"],
    ])

    const personalRemaining = pickBillingNumber(metadata, [
      ["personal_credits", "remaining"],
      ["personalCredits", "remaining"],
      ["credits", "personal", "remaining"],
      ["personal_credits_remaining"],
      ["personalCreditsRemaining"],
    ])

    const personalExpiresAt = pickBillingValue(metadata, [
      ["personal_credits", "expires_at"],
      ["personal_credits", "expiresAt"],
      ["personalCredits", "expiresAt"],
      ["add_on_credits", "expires_at"],
      ["add_on_credits", "expiresAt"],
      ["addOnCredits", "expiresAt"],
      ["credits", "personal", "expires_at"],
      ["credits", "add_on", "expires_at"],
      ["personal_credits_expires_at"],
      ["personalCreditsExpiresAt"],
      ["add_on_credits_expires_at"],
    ])

    const monthlySpendLimit = pickBillingNumber(metadata, [
      ["add_on_credits", "monthly_spend_limit"],
      ["add_on_credits", "monthlySpendLimit"],
      { path: ["add_on_credits", "monthly_spend_limit_cents"], cents: true },
      ["addOnCredits", "monthlySpendLimit"],
      { path: ["addOnCredits", "monthlySpendLimitCents"], cents: true },
      ["auto_reload", "monthly_spend_limit"],
      { path: ["auto_reload", "monthly_spend_limit_cents"], cents: true },
      ["autoReload", "monthlySpendLimit"],
      { path: ["autoReload", "monthlySpendLimitCents"], cents: true },
      ["monthly_spend_limit"],
      { path: ["monthly_spend_limit_cents"], cents: true },
      ["monthlySpendLimit"],
      { path: ["monthlySpendLimitCents"], cents: true },
    ])

    const autoReloadEnabled = pickBillingBoolean(metadata, [
      ["add_on_credits", "auto_reload_enabled"],
      ["add_on_credits", "autoReloadEnabled"],
      ["addOnCredits", "autoReloadEnabled"],
      ["auto_reload", "enabled"],
      ["autoReload", "enabled"],
      ["auto_reload_enabled"],
      ["autoReloadEnabled"],
    ])

    const purchasedThisMonth = pickBillingNumber(metadata, [
      ["add_on_credits", "purchased_this_month"],
      ["add_on_credits", "purchasedThisMonth"],
      { path: ["add_on_credits", "purchased_this_month_cents"], cents: true },
      ["addons", "credits", "purchased_this_month"],
      ["credits", "add_on", "purchased_this_month"],
      ["addOnCredits", "purchasedThisMonth"],
      { path: ["addOnCredits", "purchasedThisMonthCents"], cents: true },
      ["add_on_credits_purchased_this_month"],
      ["add_on_credit_purchased_this_month"],
      { path: ["add_on_credits_purchased_this_month_cents"], cents: true },
    ])

    return {
      plan,
      baseRemaining,
      baseLimit,
      baseUsed,
      baseResetsAt,
      personalRemaining: personalRemaining !== null ? personalRemaining : addOnCredit,
      personalExpiresAt,
      monthlySpendLimit,
      autoReloadEnabled,
      purchasedThisMonth,
    }
  }

  function readBilling(ctx) {
    for (let i = 0; i < SQLITE_PATHS.length; i += 1) {
      const path = SQLITE_PATHS[i]
      if (!ctx.host.fs.exists(path)) continue

      try {
        const rows = parseJsonLike(ctx, ctx.host.sqlite.query(path, PLAN_SQL))
        if (!Array.isArray(rows)) continue

        for (let r = 0; r < rows.length; r += 1) {
          const row = rows[r]
          if (!isObject(row)) continue

          const rawMetadata = row.billing_metadata_json || row.billingMetadataJson || null
          const metadata = parseJsonLike(ctx, rawMetadata)
          const billing = extractBilling(metadata)
          if (
            billing.plan ||
            billing.baseRemaining !== null ||
            billing.baseLimit !== null ||
            billing.baseUsed !== null ||
            billing.personalRemaining !== null ||
            billing.monthlySpendLimit !== null ||
            billing.autoReloadEnabled !== null ||
            billing.purchasedThisMonth !== null
          ) {
            return {
              plan: billing.plan ? ctx.fmt.planLabel(billing.plan) : null,
              baseRemaining: billing.baseRemaining,
              baseLimit: billing.baseLimit,
              baseUsed: billing.baseUsed,
              baseResetsAt: billing.baseResetsAt,
              personalRemaining: billing.personalRemaining,
              personalExpiresAt: billing.personalExpiresAt,
              monthlySpendLimit: billing.monthlySpendLimit,
              autoReloadEnabled: billing.autoReloadEnabled,
              purchasedThisMonth: billing.purchasedThisMonth,
            }
          }
        }
      } catch (e) {
        ctx.host.log.warn("warp sqlite read failed for " + path + ": " + String(e))
      }
    }

    return emptyBilling()
  }

  function hasBillingUsage(billing) {
    return Boolean(
      billing.plan ||
      billing.baseRemaining !== null ||
      billing.baseLimit !== null ||
      billing.baseUsed !== null ||
      billing.personalRemaining !== null ||
      billing.monthlySpendLimit !== null ||
      billing.autoReloadEnabled !== null ||
      billing.purchasedThisMonth !== null
    )
  }

  function probe(ctx) {
    const prefsState = readPrefs(ctx)
    const billing = readBilling(ctx)
    if (!prefsState.prefs && !hasBillingUsage(billing)) {
      if (prefsState.sawFile) {
        throw "Warp usage data unavailable. Open Warp and try again."
      }
      throw "Warp not detected. Open Warp and try again."
    }

    const info = prefsState.prefs ? pickRequestLimitInfo(ctx, prefsState.prefs) : null
    if (!info && billing.baseRemaining === null && billing.baseLimit === null && billing.baseUsed === null) {
      throw "Warp usage data unavailable. Open Warp and try again."
    }

    const baseLimit = billing.baseLimit !== null ? billing.baseLimit : info ? info.limit : null
    const baseUsed = billing.baseUsed !== null
      ? billing.baseUsed
      : billing.baseRemaining !== null && baseLimit !== null
        ? Math.max(0, baseLimit - billing.baseRemaining)
        : info ? info.used : null
    if (baseLimit === null || baseUsed === null) {
      throw "Warp usage data unavailable. Open Warp and try again."
    }

    const line = {
      label: "Base Credits",
      used: baseUsed,
      limit: baseLimit,
      format: { kind: "count", suffix: "credits" },
    }

    const resetsAt = ctx.util.toIso(billing.baseResetsAt || (info && info.resetsAt))
    if (resetsAt) line.resetsAt = resetsAt

    const periodDurationMs = info ? derivePeriodDurationMs(ctx, prefsState.prefs, info) : null
    if (periodDurationMs) line.periodDurationMs = periodDurationMs

    const lines = [ctx.line.progress(line)]

    const personalCredits = formatCount(billing.personalRemaining)
    if (personalCredits) {
      const personalLine = {
        label: "Personal Credits",
        value: personalCredits + " remaining",
      }
      const expiry = formatDateLabel(ctx, "Expires", billing.personalExpiresAt)
      if (expiry) personalLine.subtitle = expiry
      lines.push(ctx.line.text(personalLine))
    }

    const monthlySpendLimit = formatDollars(billing.monthlySpendLimit)
    if (monthlySpendLimit) {
      lines.push(ctx.line.text({ label: "Monthly Spend Limit", value: monthlySpendLimit }))
    }

    if (billing.autoReloadEnabled !== null) {
      lines.push(ctx.line.badge({
        label: "Auto-reload",
        text: billing.autoReloadEnabled ? "Enabled" : "Disabled",
        color: billing.autoReloadEnabled ? "#22c55e" : "#a3a3a3",
      }))
    }

    const purchasedThisMonth = formatDollars(billing.purchasedThisMonth)
    if (purchasedThisMonth) {
      lines.push(ctx.line.text({
        label: "Purchased This Month",
        value: purchasedThisMonth,
      }))
    }

    ctx.host.log.info("warp usage loaded from " + (prefsState.path || "sqlite"))

    return {
      plan: billing.plan,
      lines,
    }
  }

  globalThis.__pulseusage_plugin = { id: "warp", probe }
})()
