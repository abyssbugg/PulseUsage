import type {
  AuthDetectedStatus,
  DataSourceReachability,
  DiagnosticsHealth,
  DiagnosticsLikelyCause,
  ManifestMetricDiagnostic,
  MetricClassification,
  MetricLine,
  MissingMetricDiagnostic,
  ParserExecutionStatus,
  PluginDisplayState,
  ProviderDiagnostics,
  ReturnedMetricDiagnostic,
  SafeHostFacts,
} from "@/lib/plugin-types"

const EMPTY_HOST_FACTS: SafeHostFacts = {
  httpRequestsAttempted: 0,
  http2xxResponsesSeen: 0,
  authStatusResponsesSeen: 0,
  localReadsAttempted: 0,
  localReadFailures: 0,
  authReadAttempts: 0,
  authReadSuccesses: 0,
}

export function normalizeMetricClassification(
  value: string | null | undefined
): MetricClassification {
  if (!value) return "unknown"
  const key = value
    .trim()
    .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
    .toLowerCase()
    .replace(/[\s_]+/g, "-")
  if (key === "required" || key === "optional" || key === "deprecated") return key
  if (key === "plan-dependent") return "planDependent"
  return "unknown"
}

export function redactDiagnosticText(text: string): string {
  let result = text

  result = result.replace(
    /\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b/gi,
    "[REDACTED]"
  )
  result = result.replace(
    /\b(?:sk-|pk-|api_|key_|secret_)[A-Za-z0-9_-]{8,}\b/g,
    "[REDACTED]"
  )
  result = result.replace(
    /\beyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\b/g,
    "[REDACTED]"
  )
  result = result.replace(/\bBearer\s+[A-Za-z0-9._-]+\b/gi, "Bearer [REDACTED]")
  result = result.replace(
    /\b(?:user|account|org|organization|session)[_-]?id\s*[:=]\s*[A-Za-z0-9._-]+\b/gi,
    (match) => match.replace(/[:=]\s*[A-Za-z0-9._-]+$/, "=[REDACTED]")
  )
  result = result.replace(/https?:\/\/[^\s"',)]+/gi, "[URL]")
  result = result.replace(
    /\/(?:Users|home|private|var|tmp|Applications)\/[^\s"',)]+/g,
    "[PATH]"
  )

  return result
}

export function buildProviderDiagnostics(plugin: PluginDisplayState): ProviderDiagnostics {
  const runtime = plugin.data?.diagnostics
  const hostFacts = { ...EMPTY_HOST_FACTS, ...runtime?.hostFacts }
  const manifestMetrics = buildManifestMetrics(plugin)
  const returnedMetrics = buildReturnedMetrics(plugin.data?.lines ?? [])
  const missingMetrics = buildMissingMetrics(manifestMetrics, returnedMetrics)
  const runtimeError = errorFromLines(plugin.data?.lines ?? [])
  const rawError = plugin.error ?? runtime?.lastError ?? runtimeError
  const lastError = rawError ? redactDiagnosticText(rawError) : null
  const parserExecutionStatus =
    runtime?.parserExecutionStatus ?? deriveParserExecutionStatus(plugin, lastError)
  const authDetected = runtime?.authDetected ?? deriveAuthDetected(hostFacts)
  const dataSourceReachable =
    runtime?.dataSourceReachable ?? deriveDataSourceReachable(hostFacts)
  const likelyCauses = deriveLikelyCauses({
    runtimeCauses: runtime?.likelyCauses ?? [],
    parserExecutionStatus,
    lastError,
    hostFacts,
    dataSourceReachable,
    missingMetrics,
    returnedMetrics,
    hasRuntimeData: Boolean(plugin.data),
  })
  const healthSummary = deriveHealthSummary({
    runtimeHealth: runtime?.healthSummary,
    parserExecutionStatus,
    hostFacts,
    missingMetrics,
  })

  return {
    providerLoaded: true,
    providerVersion: runtime?.providerVersion ?? plugin.meta.version ?? null,
    authDetected,
    dataSourceReachable,
    lastSuccessfulRefreshAt:
      plugin.lastUpdatedAt ?? runtime?.lastSuccessfulRefreshAt ?? null,
    manifestMetrics,
    returnedMetrics,
    missingMetrics,
    lastError,
    parserExecutionStatus,
    healthSummary,
    likelyCauses,
    hostFacts,
  }
}

function buildManifestMetrics(plugin: PluginDisplayState): ManifestMetricDiagnostic[] {
  return plugin.meta.lines.map((line) => ({
    label: line.label,
    type: line.type,
    scope: line.scope,
    classification: normalizeMetricClassification(line.classification),
  }))
}

function buildReturnedMetrics(lines: MetricLine[]): ReturnedMetricDiagnostic[] {
  return lines
    .filter((line) => !(line.type === "badge" && line.label === "Error"))
    .map((line) => ({ label: line.label, type: line.type }))
}

function buildMissingMetrics(
  manifestMetrics: ManifestMetricDiagnostic[],
  returnedMetrics: ReturnedMetricDiagnostic[]
): MissingMetricDiagnostic[] {
  const returnedLabels = new Set(returnedMetrics.map((line) => line.label))
  return manifestMetrics.filter((line) => !returnedLabels.has(line.label))
}

function errorFromLines(lines: MetricLine[]): string | null {
  if (lines.length !== 1) return null
  const line = lines[0]
  if (line.type === "badge" && line.label === "Error") {
    return line.text || "Couldn't update data. Try again?"
  }
  return null
}

function deriveParserExecutionStatus(
  plugin: PluginDisplayState,
  lastError: string | null
): ParserExecutionStatus {
  if (lastError) return "failed"
  if (plugin.data) return "success"
  return "notRun"
}

function deriveAuthDetected(hostFacts: SafeHostFacts): AuthDetectedStatus {
  if (hostFacts.authReadSuccesses > 0) return "detected"
  if (hostFacts.authReadAttempts > 0) return "notDetected"
  return "unknown"
}

function deriveDataSourceReachable(hostFacts: SafeHostFacts): DataSourceReachability {
  if (hostFacts.http2xxResponsesSeen > 0) return "reachable"
  if (
    hostFacts.localReadsAttempted > 0 &&
    hostFacts.localReadFailures < hostFacts.localReadsAttempted
  ) {
    return "reachable"
  }
  if (
    hostFacts.httpRequestsAttempted > 0 ||
    hostFacts.localReadsAttempted > 0
  ) {
    return "unreachable"
  }
  return "unknown"
}

function deriveHealthSummary({
  runtimeHealth,
  parserExecutionStatus,
  hostFacts,
  missingMetrics,
}: {
  runtimeHealth?: DiagnosticsHealth
  parserExecutionStatus: ParserExecutionStatus
  hostFacts: SafeHostFacts
  missingMetrics: MissingMetricDiagnostic[]
}): DiagnosticsHealth {
  if (parserExecutionStatus === "failed") return "error"
  if (runtimeHealth === "error") return "error"
  if (
    runtimeHealth === "warning" ||
    hostFacts.authStatusResponsesSeen > 0 ||
    missingMetrics.some((metric) => metric.classification === "required")
  ) {
    return "warning"
  }
  if (parserExecutionStatus === "notRun") return "unknown"
  return "ok"
}

function deriveLikelyCauses({
  runtimeCauses,
  parserExecutionStatus,
  lastError,
  hostFacts,
  dataSourceReachable,
  missingMetrics,
  returnedMetrics,
  hasRuntimeData,
}: {
  runtimeCauses: DiagnosticsLikelyCause[]
  parserExecutionStatus: ParserExecutionStatus
  lastError: string | null
  hostFacts: SafeHostFacts
  dataSourceReachable: DataSourceReachability
  missingMetrics: MissingMetricDiagnostic[]
  returnedMetrics: ReturnedMetricDiagnostic[]
  hasRuntimeData: boolean
}): DiagnosticsLikelyCause[] {
  const causes = new Set<DiagnosticsLikelyCause>(runtimeCauses)
  const errorText = lastError?.toLowerCase() ?? ""

  if (parserExecutionStatus === "failed") causes.add("parserError")
  if (hostFacts.authStatusResponsesSeen > 0) causes.add("authRejected")
  if (
    !causes.has("authRejected") &&
    /\b(auth|login|token|credential|keychain)\b/.test(errorText)
  ) {
    causes.add("authMissing")
  }
  if (dataSourceReachable === "unreachable") causes.add("dataSourceUnreachable")
  if (missingMetrics.length > 0) causes.add("manifestMismatch")
  if (hasRuntimeData && returnedMetrics.length === 0 && !lastError) {
    causes.add("noMetricsReturned")
  }
  if (causes.size === 0 && parserExecutionStatus === "failed") causes.add("unknown")

  return [...causes]
}
