import fs from "node:fs"
import path from "node:path"

const CLASSIFICATIONS = new Map([
  ["required", "Required"],
  ["optional", "Optional"],
  ["plan-dependent", "Plan-dependent"],
  ["deprecated", "Deprecated"],
])

const DOCS_ONLY_CLASSIFICATIONS = new Map([
  ["unclassified", "Unclassified"],
])

export function validateLinks(manifest, result, providerId, manifestPath) {
  if (manifest.links === undefined) return
  if (!Array.isArray(manifest.links)) {
    addIssue(result.errors, "invalid-links", providerId, "links must be an array when present.", manifestPath)
    return
  }

  for (const link of manifest.links) {
    if (!isPlainObject(link)) {
      addIssue(result.errors, "invalid-link", providerId, "each link must be an object.", manifestPath)
      continue
    }
    const label = typeof link.label === "string" ? link.label.trim() : ""
    const url = typeof link.url === "string" ? link.url.trim() : ""
    if (!label) {
      addIssue(result.errors, "empty-link-label", providerId, "link label must be non-empty.", manifestPath)
    }
    if (!/^https?:\/\//.test(url)) {
      addIssue(result.errors, "invalid-link-url", providerId, `link '${label || "(missing label)"}' must use http(s).`, manifestPath)
    }
  }
}

export function validateDocs({ docsText, docsRelPath, lineState, providerId, result }) {
  if (docsText === null) return

  const docsClassifications = parseDocsClassificationTable(docsText, result, providerId, docsRelPath)
  if (docsClassifications === null) {
    const hasManifestClassifications = lineState.classifiedLines.length > 0
    if (result.strict || hasManifestClassifications) {
      const destination = result.strict ? result.errors : result.warnings
      addIssue(
        destination,
        "missing-docs-classification-table",
        providerId,
        "provider docs are missing a Metric classification table.",
        docsRelPath
      )
    }
    return
  }

  for (const [label] of docsClassifications) {
    if (!lineState.labels.has(label)) {
      const destination = result.strict ? result.errors : result.warnings
      addIssue(destination, "docs-unknown-metric", providerId, `docs classify '${label}', but manifest lines do not.`, docsRelPath)
    }
  }

  for (const line of lineState.classifiedLines) {
    const docsClassification = docsClassifications.get(line.label)
    if (!docsClassification) {
      addIssue(
        result.errors,
        "docs-missing-classification-row",
        providerId,
        `docs are missing classification row for '${line.label}'.`,
        docsRelPath
      )
      continue
    }
    if (docsClassification !== line.classification) {
      addIssue(
        result.errors,
        "docs-classification-drift",
        providerId,
        `docs classify '${line.label}' as '${docsClassification}', but manifest says '${line.classification}'.`,
        docsRelPath
      )
    }
  }

  if (result.strict) {
    for (const label of lineState.labels) {
      if (!docsClassifications.has(label)) {
        addIssue(
          result.errors,
          "docs-missing-classification-row",
          providerId,
          `docs are missing classification row for '${label}'.`,
          docsRelPath
        )
      }
    }
  }
}

export function validateRequiredMetricCoverage({ rootDir, providerId, providerDir, lineState, result }) {
  if (lineState.requiredLabels.length === 0) return

  const testPath = path.join(providerDir, "plugin.test.js")
  const testRelPath = relativePath(rootDir, testPath)
  if (!fs.existsSync(testPath)) {
    addIssue(
      result.errors,
      "missing-provider-test-file",
      providerId,
      "required classified metrics need plugin.test.js fixture coverage.",
      testRelPath
    )
    return
  }

  const testText = fs.readFileSync(testPath, "utf8")
  for (const label of lineState.requiredLabels) {
    if (!testText.includes(label)) {
      addIssue(
        result.errors,
        "missing-required-metric-test-coverage",
        providerId,
        `required metric '${label}' is not referenced by plugin.test.js.`,
        testRelPath
      )
    }
  }
}

export function validateRelativeFile(result, options) {
  const {
    rootDir,
    providerId,
    providerDir,
    manifestPath,
    fieldName,
    fileName,
    unsafeCode,
    missingCode,
    extension,
    extensionCode,
  } = options

  if (typeof fileName !== "string" || fileName.trim() === "") {
    addIssue(result.errors, missingCode, providerId, `${fieldName} must be a non-empty relative path.`, manifestPath)
    return
  }

  const trimmed = fileName.trim()
  const unsafe = path.isAbsolute(trimmed) || trimmed.split(/[\\/]/).includes("..")
  if (unsafe) {
    addIssue(result.errors, unsafeCode, providerId, `${fieldName} must stay inside the provider directory.`, manifestPath)
    return
  }

  if (extension && path.extname(trimmed) !== extension) {
    addIssue(result.errors, extensionCode, providerId, `${fieldName} must point to a ${extension} file.`, manifestPath)
  }

  const targetPath = path.resolve(providerDir, trimmed)
  if (!fs.existsSync(targetPath)) {
    addIssue(result.errors, missingCode, providerId, `${fieldName} file '${trimmed}' is missing.`, relativePath(rootDir, targetPath))
  }
}

function parseDocsClassificationTable(docsText, result, providerId, docsRelPath) {
  const lines = docsText.split(/\r?\n/)
  const headingIndex = lines.findIndex((line) =>
    /^metric classifications?:?$/i.test(normalizeMarkdownHeading(line))
  )
  if (headingIndex === -1) return null

  const tableLines = []
  let collecting = false
  for (let i = headingIndex + 1; i < lines.length; i++) {
    const line = lines[i].trim()
    if (line.startsWith("|")) {
      tableLines.push(line)
      collecting = true
      continue
    }
    if (collecting && line !== "") break
  }

  if (tableLines.length === 0) return new Map()

  const rows = tableLines.map(splitMarkdownTableRow)
  const header = rows.find((row) => !isMarkdownSeparatorRow(row))
  if (!header) return new Map()

  const metricIndex = header.findIndex((cell) => /^metric$/i.test(cell))
  const classificationIndex = header.findIndex((cell) => /^classification$/i.test(cell))
  if (metricIndex === -1 || classificationIndex === -1) {
    addIssue(
      result.errors,
      "invalid-docs-classification-table",
      providerId,
      "Metric classification table must include Metric and Classification columns.",
      docsRelPath
    )
    return new Map()
  }

  const out = new Map()
  for (const row of rows) {
    if (row === header || isMarkdownSeparatorRow(row)) continue
    const label = row[metricIndex] ? row[metricIndex].trim() : ""
    const classification = canonicalClassification(row[classificationIndex], {
      allowUnclassified: true,
    })
    if (!label) continue
    if (!classification) {
      addIssue(
        result.errors,
        "invalid-docs-classification",
        providerId,
        `docs classification for '${label}' must be Required, Optional, Plan-dependent, Deprecated, or Unclassified.`,
        docsRelPath
      )
      continue
    }
    out.set(label, classification)
  }
  return out
}

function normalizeMarkdownHeading(line) {
  return line
    .trim()
    .replace(/^#{1,6}\s*/, "")
    .replace(/\s+#{1,6}$/, "")
    .trim()
}

function splitMarkdownTableRow(line) {
  return line
    .replace(/^\s*\|/, "")
    .replace(/\|\s*$/, "")
    .split("|")
    .map((cell) => cell.trim())
}

function isMarkdownSeparatorRow(row) {
  return row.every((cell) => /^:?-{3,}:?$/.test(cell.replace(/\s/g, "")))
}

export function canonicalClassification(value, options = {}) {
  if (typeof value !== "string") return null
  const key = value
    .trim()
    .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
    .toLowerCase()
    .replace(/[\s_]+/g, "-")
  return (
    CLASSIFICATIONS.get(key) ||
    (options.allowUnclassified ? DOCS_ONLY_CLASSIFICATIONS.get(key) : null) ||
    null
  )
}

export function readJson(filePath, result, providerId, relPath) {
  try {
    return JSON.parse(fs.readFileSync(filePath, "utf8"))
  } catch (error) {
    addIssue(result.errors, "invalid-manifest-json", providerId, `failed to parse plugin.json: ${error.message}`, relPath)
    return null
  }
}

export function addIssue(list, code, providerId, message, file) {
  list.push({
    code,
    providerId,
    file,
    message,
  })
}

export function finish(result) {
  result.errors.sort(compareIssue)
  result.warnings.sort(compareIssue)
  result.ok = result.errors.length === 0
}

function compareIssue(a, b) {
  return (
    String(a.providerId || "").localeCompare(String(b.providerId || "")) ||
    String(a.file || "").localeCompare(String(b.file || "")) ||
    a.code.localeCompare(b.code) ||
    a.message.localeCompare(b.message)
  )
}

export function formatIssue(level, issue) {
  const provider = issue.providerId ? `[${issue.providerId}] ` : ""
  const file = issue.file ? `${issue.file}: ` : ""
  return `${level} ${provider}${file}${issue.code} - ${issue.message}`
}

export function relativePath(rootDir, filePath) {
  if (!filePath) return undefined
  return path.relative(rootDir, filePath).replaceAll(path.sep, "/")
}

export function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value)
}
