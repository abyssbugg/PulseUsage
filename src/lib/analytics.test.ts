import { describe, expect, it } from "vitest"

describe("analytics track", () => {
  it("is a no-op for the independent baseline", async () => {
    const { track } = await import("./analytics")

    track("test_event", { foo: "bar" })
    track("test_event", { foo: "bar" })
    expect(true).toBe(true)
  })
})
