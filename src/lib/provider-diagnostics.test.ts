import { describe, expect, it } from "vitest"

import {
  buildProviderDiagnostics,
  normalizeMetricClassification,
  redactDiagnosticText,
} from "@/lib/provider-diagnostics"
import type { PluginDisplayState } from "@/lib/plugin-types"

function pluginState(overrides: Partial<PluginDisplayState> = {}): PluginDisplayState {
  return {
    meta: {
      id: "alpha",
      name: "Alpha",
      version: "0.1.0",
      iconUrl: "",
      lines: [
        { type: "progress", label: "Base Credits", scope: "overview", classification: "required" },
        { type: "text", label: "Bonus Credits", scope: "detail", classification: "optional" },
        { type: "badge", label: "Plan", scope: "overview" },
      ],
      primaryCandidates: [],
    },
    data: {
      providerId: "alpha",
      displayName: "Alpha",
      iconUrl: "",
      lines: [
        {
          type: "progress",
          label: "Base Credits",
          used: 10,
          limit: 100,
          format: { kind: "count", suffix: "credits" },
        },
      ],
      diagnostics: {
        providerLoaded: true,
        providerVersion: "0.1.0",
        authDetected: "detected",
        dataSourceReachable: "reachable",
        lastSuccessfulRefreshAt: null,
        manifestMetrics: [],
        returnedMetrics: [],
        missingMetrics: [],
        lastError: null,
        parserExecutionStatus: "success",
        healthSummary: "ok",
        likelyCauses: [],
        hostFacts: {
          httpRequestsAttempted: 1,
          http2xxResponsesSeen: 1,
          authStatusResponsesSeen: 0,
          localReadsAttempted: 0,
          localReadFailures: 0,
          authReadAttempts: 1,
          authReadSuccesses: 1,
        },
      },
    },
    loading: false,
    error: null,
    lastManualRefreshAt: null,
    lastUpdatedAt: 1_800_000_000_000,
    ...overrides,
  }
}

describe("buildProviderDiagnostics", () => {
  it("derives safe diagnostics from manifest metadata, runtime output, and probe state", () => {
    const diagnostics = buildProviderDiagnostics(pluginState())

    expect(diagnostics.providerLoaded).toBe(true)
    expect(diagnostics.providerVersion).toBe("0.1.0")
    expect(diagnostics.authDetected).toBe("detected")
    expect(diagnostics.dataSourceReachable).toBe("reachable")
    expect(diagnostics.lastSuccessfulRefreshAt).toBe(1_800_000_000_000)
    expect(diagnostics.returnedMetrics).toEqual([
      { label: "Base Credits", type: "progress" },
    ])
    expect(diagnostics.missingMetrics).toEqual([
      { label: "Bonus Credits", type: "text", scope: "detail", classification: "optional" },
      { label: "Plan", type: "badge", scope: "overview", classification: "unknown" },
    ])
  })

  it("reports required missing metrics as a degraded health summary", () => {
    const diagnostics = buildProviderDiagnostics(pluginState({
      meta: {
        id: "alpha",
        name: "Alpha",
        version: "0.1.0",
        iconUrl: "",
        lines: [
          { type: "progress", label: "Base Credits", scope: "overview", classification: "required" },
          { type: "text", label: "Required Detail", scope: "detail", classification: "required" },
        ],
        primaryCandidates: [],
      },
    }))

    expect(diagnostics.healthSummary).toBe("warning")
    expect(diagnostics.likelyCauses).toContain("manifestMismatch")
    expect(diagnostics.missingMetrics).toContainEqual({
      label: "Required Detail",
      type: "text",
      scope: "detail",
      classification: "required",
    })
  })

  it("normalizes all supported classification spellings", () => {
    expect(normalizeMetricClassification("required")).toBe("required")
    expect(normalizeMetricClassification("Optional")).toBe("optional")
    expect(normalizeMetricClassification("planDependent")).toBe("planDependent")
    expect(normalizeMetricClassification("plan-dependent")).toBe("planDependent")
    expect(normalizeMetricClassification("deprecated")).toBe("deprecated")
    expect(normalizeMetricClassification("unclassified")).toBe("unknown")
  })

  it("redacts sensitive diagnostic error text", () => {
    const rawEmail = ["user", String.fromCharCode(64), "example.invalid"].join("")
    const rawSecret = ["sk", "-", "test", "-", "secret", "-1234567890"].join("")
    const rawPath = ["/", "Users", "/", "sample", "/.config/app"].join("")
    const raw = `Failed for ${rawEmail} with ${rawSecret} at ${rawPath}`
    const redacted = redactDiagnosticText(raw)
    expect(redacted).not.toContain(rawEmail)
    expect(redacted).not.toContain(rawSecret)
    expect(redacted).not.toContain(rawPath)
    expect(redacted).toContain("[REDACTED]")
  })

  it("uses safe fallback diagnostics when runtime diagnostics are absent", () => {
    const diagnostics = buildProviderDiagnostics(pluginState({
      data: {
        providerId: "alpha",
        displayName: "Alpha",
        iconUrl: "",
        lines: [
          { type: "badge", label: "Error", text: "Login required", color: "#ef4444" },
        ],
      },
      error: "Login required",
      lastUpdatedAt: null,
    }))

    expect(diagnostics.parserExecutionStatus).toBe("failed")
    expect(diagnostics.healthSummary).toBe("error")
    expect(diagnostics.lastError).toBe("Login required")
    expect(diagnostics.likelyCauses).toContain("authMissing")
  })
})
