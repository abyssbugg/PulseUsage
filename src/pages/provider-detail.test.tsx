import { render, screen } from "@testing-library/react"
import { describe, expect, it } from "vitest"

import { ProviderDetailPage } from "@/pages/provider-detail"

describe("ProviderDetailPage", () => {
  it("shows not found when plugin missing", () => {
    render(<ProviderDetailPage plugin={null} displayMode="used" resetTimerDisplayMode="relative" />)
    expect(screen.getByText("Provider not found")).toBeInTheDocument()
  })

  it("renders ProviderCard with all scope when plugin present", async () => {
    render(
      <ProviderDetailPage
        displayMode="used"
        resetTimerDisplayMode="relative"
        plugin={{
          meta: { id: "a", name: "Alpha", iconUrl: "", lines: [] },
          data: { providerId: "a", displayName: "Alpha", iconUrl: "", lines: [] },
          loading: false,
          error: null,
          lastManualRefreshAt: null,
          lastUpdatedAt: null,
        }}
      />
    )
    expect(screen.getAllByText("Alpha").length).toBeGreaterThan(0)
  })

  it("renders when plugin data is null (still shows provider name)", () => {
    render(
      <ProviderDetailPage
        displayMode="used"
        resetTimerDisplayMode="relative"
        plugin={{
          meta: { id: "a", name: "Alpha", iconUrl: "", lines: [] },
          data: null,
          loading: false,
          error: null,
          lastManualRefreshAt: null,
          lastUpdatedAt: null,
        }}
      />
    )
    expect(screen.getAllByText("Alpha").length).toBeGreaterThan(0)
  })

  it("renders quick links when provided by plugin meta", () => {
    render(
      <ProviderDetailPage
        displayMode="used"
        resetTimerDisplayMode="relative"
        plugin={{
          meta: {
            id: "a",
            name: "Alpha",
            iconUrl: "",
            lines: [],
            links: [{ label: "Status", url: "https://status.example.com" }],
          },
          data: null,
          loading: false,
          error: null,
          lastManualRefreshAt: null,
          lastUpdatedAt: null,
        }}
      />
    )
    expect(screen.getByRole("button", { name: /status/i })).toBeInTheDocument()
  })

  it("renders safe provider diagnostics with missing metrics", () => {
    const rawEmail = ["user", String.fromCharCode(64), "example.invalid"].join("")
    const rawSecret = ["sk", "-", "test", "-", "secret", "-1234567890"].join("")
    const rawPath = ["/", "Users", "/", "sample", "/app"].join("")
    render(
      <ProviderDetailPage
        displayMode="used"
        resetTimerDisplayMode="relative"
        plugin={{
          meta: {
            id: "a",
            name: "Alpha",
            version: "0.1.0",
            iconUrl: "",
            lines: [
              { type: "progress", label: "Base Credits", scope: "overview", classification: "required" },
              { type: "text", label: "Bonus Credits", scope: "detail", classification: "optional" },
            ],
          },
          data: {
            providerId: "a",
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
              authDetected: "unknown",
              dataSourceReachable: "reachable",
              lastSuccessfulRefreshAt: null,
              manifestMetrics: [],
              returnedMetrics: [],
              missingMetrics: [],
              lastError: `HTTP 401 for ${rawEmail} with ${rawSecret} at ${rawPath}`,
              parserExecutionStatus: "success",
              healthSummary: "warning",
              likelyCauses: ["manifestMismatch"],
              hostFacts: {
                httpRequestsAttempted: 1,
                http2xxResponsesSeen: 1,
                authStatusResponsesSeen: 1,
                localReadsAttempted: 0,
                localReadFailures: 0,
                authReadAttempts: 0,
                authReadSuccesses: 0,
              },
            },
          },
          loading: false,
          error: null,
          lastManualRefreshAt: null,
          lastUpdatedAt: 1_800_000_000_000,
        }}
      />
    )

    expect(screen.getByText("Diagnostics")).toBeInTheDocument()
    expect(screen.getByText("Needs attention")).toBeInTheDocument()
    expect(screen.getAllByText("Bonus Credits").length).toBeGreaterThan(0)
    expect(screen.getByText("optional")).toBeInTheDocument()
    expect(screen.getByText("HTTP attempted: 1")).toBeInTheDocument()
    expect(screen.queryByText(rawEmail)).toBeNull()
    expect(screen.queryByText(rawSecret)).toBeNull()
    expect(screen.queryByText(rawPath)).toBeNull()
  })
})
