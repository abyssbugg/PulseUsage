import { describe, expect, it } from "vitest"

import {
  APP_DISPLAY_NAME,
  DOCUMENTATION_URL,
  GITHUB_OWNER,
  GITHUB_REPO,
  GITHUB_REPOSITORY,
  GITHUB_REPOSITORY_URL,
  ISSUES_URL,
  MARKETING_NAME,
  RELEASES_URL,
  SUPPORT_URL,
  buildCommitUrl,
  buildPullRequestUrl,
  buildReleaseApiUrl,
} from "./app-identity"

describe("app identity", () => {
  it("centralizes current PulseUsage identity without changing visible behavior", () => {
    expect(APP_DISPLAY_NAME).toBe("PulseUsage")
    expect(MARKETING_NAME).toBe("PulseUsage")
    expect(GITHUB_OWNER).toBe("abyssbugg")
    expect(GITHUB_REPO).toBe("PulseUsage")
    expect(GITHUB_REPOSITORY).toBe("abyssbugg/PulseUsage")
    expect(GITHUB_REPOSITORY_URL).toBe("https://github.com/abyssbugg/PulseUsage")
    expect(ISSUES_URL).toBe("https://github.com/abyssbugg/PulseUsage/issues")
    expect(RELEASES_URL).toBe("https://github.com/abyssbugg/PulseUsage/releases")
    expect(DOCUMENTATION_URL).toBe("https://github.com/abyssbugg/PulseUsage#readme")
    expect(SUPPORT_URL).toBe("https://github.com/abyssbugg/PulseUsage/issues")
  })

  it("builds GitHub URLs from the centralized repository identity", () => {
    expect(buildReleaseApiUrl("v0.7.0-rc.1")).toBe(
      "https://api.github.com/repos/abyssbugg/PulseUsage/releases/tags/v0.7.0-rc.1",
    )
    expect(buildPullRequestUrl(52)).toBe("https://github.com/abyssbugg/PulseUsage/pull/52")
    expect(buildCommitUrl("abc123")).toBe("https://github.com/abyssbugg/PulseUsage/commit/abc123")
  })
})
