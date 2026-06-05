import { renderHook, act } from "@testing-library/react"
import { describe, expect, it } from "vitest"

import { useAppUpdate } from "@/hooks/use-app-update"

describe("useAppUpdate", () => {
  it("stays idle because independent update channel is disabled", async () => {
    const { result } = renderHook(() => useAppUpdate())

    expect(result.current.updateStatus).toEqual({ status: "idle" })

    await act(() => result.current.checkForUpdates())
    expect(result.current.updateStatus).toEqual({ status: "idle" })

    await act(() => result.current.triggerInstall())
    expect(result.current.updateStatus).toEqual({ status: "idle" })
  })
})
