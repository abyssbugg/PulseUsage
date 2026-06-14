import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs"
import os from "node:os"
import path from "node:path"
import { describe, expect, test } from "vitest"
import { validateProviderMetadata } from "./validate-provider-metadata.mjs"

function makeRoot() {
  return mkdtempSync(path.join(os.tmpdir(), "pulseusage-provider-validator-"))
}

function writeProvider(rootDir, id, options = {}) {
  const pluginDir = path.join(rootDir, "plugins", id)
  mkdirSync(pluginDir, { recursive: true })

  const manifest = options.manifest || {
    schemaVersion: 1,
    id,
    name: "Sample Provider",
    version: "0.0.1",
    entry: "plugin.js",
    icon: "icon.svg",
    brandColor: "#123456",
    lines: [
      {
        type: "progress",
        label: "Usage",
        scope: "overview",
        primaryOrder: 1,
        classification: "required",
      },
      {
        type: "text",
        label: "Entitlement",
        scope: "detail",
        classification: "planDependent",
      },
      {
        type: "text",
        label: "Legacy",
        scope: "detail",
        classification: "deprecated",
      },
    ],
    links: [{ label: "Status", url: "https://status.example.com" }],
  }

  writeFileSync(path.join(pluginDir, "plugin.json"), JSON.stringify(manifest, null, 2) + "\n")

  if (options.entryText !== null) {
    writeFileSync(
      path.join(pluginDir, "plugin.js"),
      options.entryText ||
        `globalThis.__pulseusage_plugin = { id: "${manifest.id}", probe: function () { return { lines: [] } } }\n`
    )
  }

  if (options.iconText !== null) {
    writeFileSync(
      path.join(pluginDir, "icon.svg"),
      options.iconText || `<svg viewBox="0 0 16 16"><path fill="currentColor" d="M0 0h16v16H0z"/></svg>\n`
    )
  }

  if (options.testText !== null) {
    writeFileSync(path.join(pluginDir, "plugin.test.js"), options.testText || `expectLabel("Usage")\n`)
  }

  if (options.docsText !== null) {
    const docsDir = path.join(rootDir, "docs", "providers")
    mkdirSync(docsDir, { recursive: true })
    writeFileSync(
      path.join(docsDir, `${id}.md`),
      options.docsText ||
        `# Sample Provider\n\n## Metric classification\n\n| Metric | Classification | Evidence |\n|---|---|---|\n| Usage | Required | Sample fixture returns this metric. |\n| Entitlement | Plan-dependent | Sample fixture covers entitlement-dependent output. |\n| Legacy | Deprecated | Sample fixture covers legacy fallback output. |\n`
    )
  }
}

function codes(result) {
  return result.errors.map((error) => error.code)
}

describe("validateProviderMetadata", () => {
  test("accepts valid classified provider metadata in strict mode", async () => {
    const rootDir = makeRoot()
    writeProvider(rootDir, "sample-provider")

    const result = await validateProviderMetadata({ rootDir, strict: true })

    expect(result.ok).toBe(true)
    expect(result.checkedProviderIds).toEqual(["sample-provider"])
    expect(result.errors).toEqual([])
  })

  test("keeps classification metadata backward-compatible outside strict mode", async () => {
    const rootDir = makeRoot()
    writeProvider(rootDir, "sample-provider", {
      manifest: {
        schemaVersion: 1,
        id: "sample-provider",
        name: "Sample Provider",
        version: "0.0.1",
        entry: "plugin.js",
        icon: "icon.svg",
        lines: [{ type: "progress", label: "Usage", scope: "overview", primaryOrder: 1 }],
      },
    })

    const defaultResult = await validateProviderMetadata({ rootDir })
    const strictResult = await validateProviderMetadata({ rootDir, strict: true })

    expect(defaultResult.ok).toBe(true)
    expect(defaultResult.warnings.map((warning) => warning.code)).toContain("missing-line-classification")
    expect(strictResult.ok).toBe(false)
    expect(codes(strictResult)).toContain("missing-line-classification")
  })

  test("reports malformed manifests and missing provider files", async () => {
    const rootDir = makeRoot()
    writeProvider(rootDir, "bad-provider", {
      manifest: {
        schemaVersion: 1,
        id: "Bad Provider",
        name: " ",
        version: "latest",
        entry: "../outside.js",
        icon: "icon.png",
        lines: [
          { type: "progress", label: "Usage", scope: "overview", primaryOrder: 1 },
          { type: "text", label: "Usage", scope: "detail" },
          { type: "badge", label: "Status", scope: "side-panel", primaryOrder: 2 },
        ],
        links: [{ label: "Docs", url: "ftp://example.com/docs" }],
      },
      iconText: null,
      docsText: null,
    })

    const result = await validateProviderMetadata({ rootDir })

    expect(result.ok).toBe(false)
    expect(codes(result)).toEqual(
      expect.arrayContaining([
        "provider-id-mismatch",
        "malformed-provider-id",
        "empty-provider-name",
        "malformed-version",
        "unsafe-entry-path",
        "invalid-icon-extension",
        "missing-icon-file",
        "missing-provider-docs",
        "duplicate-line-label",
        "invalid-line-scope",
        "primary-order-on-non-progress",
        "invalid-link-url",
      ])
    )
  })

  test("does not report missing files for unsafe relative paths", async () => {
    const rootDir = makeRoot()
    writeProvider(rootDir, "bad-provider", {
      manifest: {
        schemaVersion: 1,
        id: "bad-provider",
        name: "Bad Provider",
        version: "0.0.1",
        entry: "../outside.js",
        icon: "icon.svg",
        lines: [{ type: "progress", label: "Usage", scope: "overview", primaryOrder: 1, classification: "required" }],
      },
    })

    const result = await validateProviderMetadata({ rootDir })

    expect(codes(result)).toContain("unsafe-entry-path")
    expect(codes(result)).not.toContain("missing-entry-file")
  })

  test("detects docs drift and missing required metric fixture coverage when classification exists", async () => {
    const rootDir = makeRoot()
    writeProvider(rootDir, "sample-provider", {
      testText: "expectLabel('Plan')\n",
      docsText: `# Sample Provider

Metric classification:

| Metric | Classification | Evidence |
|---|---|---|
| Usage | Optional | Old docs text. |
`,
    })

    const result = await validateProviderMetadata({ rootDir })

    expect(result.ok).toBe(false)
    expect(codes(result)).toEqual(
      expect.arrayContaining(["docs-classification-drift", "missing-required-metric-test-coverage"])
    )
  })

  test("allows unclassified documentation rows while keeping manifests strict", async () => {
    const rootDir = makeRoot()
    writeProvider(rootDir, "sample-provider", {
      manifest: {
        schemaVersion: 1,
        id: "sample-provider",
        name: "Sample Provider",
        version: "0.0.1",
        entry: "plugin.js",
        icon: "icon.svg",
        lines: [
          { type: "progress", label: "Usage", scope: "overview", primaryOrder: 1, classification: "required" },
          { type: "text", label: "Future", scope: "detail" },
        ],
      },
      docsText: `# Sample Provider

## Metric classification

| Metric | Classification | Evidence |
|---|---|---|
| Usage | Required | Sample fixture returns this metric. |
| Future | Unclassified | Blocked until evidence exists. |
`,
    })

    const defaultResult = await validateProviderMetadata({ rootDir })
    const strictResult = await validateProviderMetadata({ rootDir, strict: true })

    expect(defaultResult.ok).toBe(true)
    expect(defaultResult.warnings.map((warning) => warning.code)).toContain("missing-line-classification")
    expect(strictResult.ok).toBe(false)
    expect(codes(strictResult)).toContain("missing-line-classification")
  })

  test("skips mock provider manifests", async () => {
    const rootDir = makeRoot()
    writeProvider(rootDir, "mock", {
      manifest: {
        schemaVersion: 1,
        id: "Bad Mock",
        name: "",
        version: "not-semver",
        entry: "/absolute.js",
        icon: "icon.png",
        lines: [],
      },
      entryText: null,
      iconText: null,
      docsText: null,
      testText: null,
    })

    const result = await validateProviderMetadata({ rootDir, strict: true })

    expect(result.ok).toBe(true)
    expect(result.checkedProviderIds).toEqual([])
    expect(result.errors).toEqual([])
  })
})
