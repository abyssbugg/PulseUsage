#!/usr/bin/env node

import fs from "node:fs"
import path from "node:path"
import { fileURLToPath } from "node:url"
import {
  addIssue,
  canonicalClassification,
  finish,
  formatIssue,
  isPlainObject,
  readJson,
  relativePath,
  validateDocs,
  validateLinks,
  validateRelativeFile,
  validateRequiredMetricCoverage,
} from "./provider-metadata-validation-rules.mjs"

const VALID_LINE_TYPES = new Set(["text", "progress", "badge", "barChart"])
const VALID_SCOPES = new Set(["overview", "detail"])
const MOCK_PROVIDER_ID = "mock"

export async function validateProviderMetadata(options = {}) {
  const rootDir = path.resolve(options.rootDir || process.cwd())
  const strict = Boolean(options.strict)
  const result = {
    ok: true,
    strict,
    checkedProviderIds: [],
    errors: [],
    warnings: [],
  }

  const pluginsDir = path.join(rootDir, "plugins")
  if (!fs.existsSync(pluginsDir)) {
    addIssue(result.errors, "missing-plugins-dir", null, "plugins directory is missing.", "plugins")
    finish(result)
    return result
  }

  const providerIds = fs
    .readdirSync(pluginsDir, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .filter((providerId) => providerId !== MOCK_PROVIDER_ID)
    .filter((providerId) => fs.existsSync(path.join(pluginsDir, providerId, "plugin.json")))
    .sort()

  result.checkedProviderIds = providerIds

  const seenManifestIds = new Map()
  for (const providerId of providerIds) {
    validateProvider(rootDir, providerId, seenManifestIds, result)
  }

  finish(result)
  return result
}

export function formatValidationResult(result) {
  const providerCount = result.checkedProviderIds.length
  const mode = result.strict ? "strict" : "default"
  const lines = [
    result.ok
      ? `Provider metadata validation passed (${providerCount} providers, ${mode} mode).`
      : `Provider metadata validation failed (${providerCount} providers, ${mode} mode).`,
  ]

  if (result.warnings.length > 0) {
    lines.push(`Warnings: ${result.warnings.length}`)
    for (const warning of result.warnings) {
      lines.push(formatIssue("WARN", warning))
    }
  }

  if (result.errors.length > 0) {
    lines.push(`Errors: ${result.errors.length}`)
    for (const error of result.errors) {
      lines.push(formatIssue("ERROR", error))
    }
  }

  return lines.join("\n")
}

function validateProvider(rootDir, providerId, seenManifestIds, result) {
  const providerDir = path.join(rootDir, "plugins", providerId)
  const manifestFile = path.join(providerDir, "plugin.json")
  const manifestPath = relativePath(rootDir, manifestFile)
  const manifest = readJson(manifestFile, result, providerId, manifestPath)
  if (!manifest) return

  if (!isPlainObject(manifest)) {
    addIssue(result.errors, "invalid-manifest-json", providerId, "plugin.json must contain a JSON object.", manifestPath)
    return
  }

  const manifestId = typeof manifest.id === "string" ? manifest.id.trim() : ""
  if (manifest.schemaVersion !== 1) {
    addIssue(result.errors, "invalid-schema-version", providerId, "schemaVersion must be 1.", manifestPath)
  }
  if (!manifestId) {
    addIssue(result.errors, "missing-provider-id", providerId, "id must be a non-empty string.", manifestPath)
  } else {
    if (manifestId !== providerId) {
      addIssue(
        result.errors,
        "provider-id-mismatch",
        providerId,
        `manifest id '${manifestId}' must match provider directory '${providerId}'.`,
        manifestPath
      )
    }
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(manifestId)) {
      addIssue(result.errors, "malformed-provider-id", providerId, "id must be kebab-case lowercase.", manifestPath)
    }
    const previousProvider = seenManifestIds.get(manifestId)
    if (previousProvider) {
      addIssue(
        result.errors,
        "duplicate-provider-id",
        providerId,
        `manifest id duplicates provider '${previousProvider}'.`,
        manifestPath
      )
    } else {
      seenManifestIds.set(manifestId, providerId)
    }
  }

  if (typeof manifest.name !== "string" || manifest.name.trim() === "") {
    addIssue(result.errors, "empty-provider-name", providerId, "name must be a non-empty string.", manifestPath)
  }
  if (typeof manifest.version !== "string" || !/^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$/.test(manifest.version)) {
    addIssue(result.errors, "malformed-version", providerId, "version must be semver-like, for example 0.0.1.", manifestPath)
  }
  if (manifest.brandColor !== undefined && manifest.brandColor !== null) {
    if (typeof manifest.brandColor !== "string" || !/^#[0-9A-Fa-f]{6}$/.test(manifest.brandColor)) {
      addIssue(result.errors, "invalid-brand-color", providerId, "brandColor must be a 6-digit hex color.", manifestPath)
    }
  }

  validateRelativeFile(result, {
    rootDir,
    providerId,
    providerDir,
    manifestPath,
    fieldName: "entry",
    fileName: manifest.entry,
    unsafeCode: "unsafe-entry-path",
    missingCode: "missing-entry-file",
  })

  validateRelativeFile(result, {
    rootDir,
    providerId,
    providerDir,
    manifestPath,
    fieldName: "icon",
    fileName: manifest.icon,
    unsafeCode: "unsafe-icon-path",
    missingCode: "missing-icon-file",
    extension: ".svg",
    extensionCode: "invalid-icon-extension",
  })

  const docsPath = path.join(rootDir, "docs", "providers", `${providerId}.md`)
  const docsRelPath = relativePath(rootDir, docsPath)
  const docsText = fs.existsSync(docsPath) ? fs.readFileSync(docsPath, "utf8") : null
  if (docsText === null) {
    addIssue(result.errors, "missing-provider-docs", providerId, `missing provider docs at ${docsRelPath}.`, docsRelPath)
  }

  const lineState = validateLines(manifest, result, providerId, manifestPath)
  validateLinks(manifest, result, providerId, manifestPath)
  validateDocs({
    docsText,
    docsRelPath,
    lineState,
    providerId,
    result,
  })
  validateRequiredMetricCoverage({
    rootDir,
    providerId,
    providerDir,
    lineState,
    result,
  })
}

function validateLines(manifest, result, providerId, manifestPath) {
  const lineState = {
    labels: new Set(),
    classifiedLines: [],
    requiredLabels: [],
  }

  if (!Array.isArray(manifest.lines) || manifest.lines.length === 0) {
    addIssue(result.errors, "missing-lines", providerId, "lines must be a non-empty array.", manifestPath)
    return lineState
  }

  const labelCounts = new Map()
  const primaryOrderOwners = new Map()
  const missingClassificationLabels = []

  for (const line of manifest.lines) {
    if (!isPlainObject(line)) {
      addIssue(result.errors, "invalid-line", providerId, "each line must be an object.", manifestPath)
      continue
    }

    const label = typeof line.label === "string" ? line.label.trim() : ""
    if (!label) {
      addIssue(result.errors, "empty-line-label", providerId, "line label must be a non-empty string.", manifestPath)
    } else {
      lineState.labels.add(label)
      labelCounts.set(label, (labelCounts.get(label) || 0) + 1)
    }

    if (!VALID_LINE_TYPES.has(line.type)) {
      addIssue(result.errors, "invalid-line-type", providerId, `line '${label || "(missing label)"}' has invalid type.`, manifestPath)
    }
    if (!VALID_SCOPES.has(line.scope)) {
      addIssue(result.errors, "invalid-line-scope", providerId, `line '${label || "(missing label)"}' has invalid scope.`, manifestPath)
    }

    if (line.primaryOrder !== undefined) {
      if (!Number.isInteger(line.primaryOrder) || line.primaryOrder < 1) {
        addIssue(result.errors, "invalid-primary-order", providerId, `line '${label}' primaryOrder must be a positive integer.`, manifestPath)
      }
      if (line.type !== "progress") {
        addIssue(result.errors, "primary-order-on-non-progress", providerId, `line '${label}' has primaryOrder but is not progress.`, manifestPath)
      } else if (Number.isInteger(line.primaryOrder)) {
        const previousLabel = primaryOrderOwners.get(line.primaryOrder)
        if (previousLabel) {
          addIssue(
            result.errors,
            "duplicate-primary-order",
            providerId,
            `primaryOrder ${line.primaryOrder} is used by both '${previousLabel}' and '${label}'.`,
            manifestPath
          )
        } else {
          primaryOrderOwners.set(line.primaryOrder, label)
        }
      }
    }

    if (line.classification === undefined) {
      if (label) missingClassificationLabels.push(label)
      continue
    }

    const classification = canonicalClassification(line.classification)
    if (!classification) {
      addIssue(result.errors, "invalid-line-classification", providerId, `line '${label}' has invalid classification.`, manifestPath)
      continue
    }
    lineState.classifiedLines.push({ label, classification })
    if (classification === "Required" && label) {
      lineState.requiredLabels.push(label)
    }
  }

  for (const [label, count] of labelCounts) {
    if (count > 1) {
      addIssue(result.errors, "duplicate-line-label", providerId, `line label '${label}' appears ${count} times.`, manifestPath)
    }
  }

  if (missingClassificationLabels.length > 0) {
    const destination = result.strict ? result.errors : result.warnings
    addIssue(
      destination,
      "missing-line-classification",
      providerId,
      `classification metadata is missing for ${missingClassificationLabels.length} line(s): ${missingClassificationLabels.join(", ")}.`,
      manifestPath
    )
  }

  lineState.requiredLabels.sort()
  lineState.classifiedLines.sort((a, b) => a.label.localeCompare(b.label))
  return lineState
}

function parseCliArgs(argv) {
  const out = {
    rootDir: process.cwd(),
    strict: process.env.PULSEUSAGE_PROVIDER_METADATA_STRICT === "1",
  }

  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i]
    if (arg === "--strict") {
      out.strict = true
    } else if (arg === "--root") {
      const value = argv[i + 1]
      if (!value) throw new Error("--root requires a path")
      out.rootDir = value
      i += 1
    } else if (arg === "--help" || arg === "-h") {
      out.help = true
    } else {
      throw new Error(`unknown argument: ${arg}`)
    }
  }

  return out
}

function printHelp() {
  console.log(`Usage: bun scripts/validate-provider-metadata.mjs [--strict] [--root <path>]

Default mode is migration-safe: missing line classification is a warning.
Strict mode is for the metadata migration branch and requires classification/docs coverage.`)
}

async function main() {
  let args
  try {
    args = parseCliArgs(process.argv.slice(2))
  } catch (error) {
    console.error(error.message)
    printHelp()
    process.exitCode = 2
    return
  }

  if (args.help) {
    printHelp()
    return
  }

  const result = await validateProviderMetadata(args)
  console.log(formatValidationResult(result))
  if (!result.ok) {
    process.exitCode = 1
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  await main()
}
