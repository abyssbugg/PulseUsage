import { describe, expect, test } from "bun:test"
import { readFileSync } from "node:fs"
import { join } from "node:path"

const root = new URL("..", import.meta.url).pathname
const readJson = (path) => JSON.parse(readFileSync(join(root, path), "utf8"))

describe("application metadata", () => {
  test("keeps runtime identity stable for PR-A1", () => {
    const pkg = readJson("package.json")
    const tauri = readJson("src-tauri/tauri.conf.json")

    expect(pkg.name).toBe("pulseusage")
    expect(tauri.productName).toBe("PulseUsage")
    expect(tauri.identifier).toBe("com.abyssbugg.pulseusage")
    expect(tauri.app.windows[0].title).toBe("PulseUsage")
  })
})
