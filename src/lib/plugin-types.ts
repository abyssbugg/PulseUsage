export type ProgressFormat =
  | { kind: "percent" }
  | { kind: "dollars" }
  | { kind: "count"; suffix: string }

export type BarChartPoint = {
  label: string
  value: number
  valueLabel?: string
}

export type MetricClassification =
  | "required"
  | "optional"
  | "planDependent"
  | "deprecated"
  | "unknown"

export type AuthDetectedStatus = "detected" | "notDetected" | "unknown"

export type DataSourceReachability = "reachable" | "unreachable" | "unknown"

export type ParserExecutionStatus = "notRun" | "success" | "failed"

export type DiagnosticsHealth = "ok" | "warning" | "error" | "unknown"

export type DiagnosticsLikelyCause =
  | "authMissing"
  | "authRejected"
  | "dataSourceUnreachable"
  | "parserError"
  | "manifestMismatch"
  | "noMetricsReturned"
  | "unknown"

export type SafeHostFacts = {
  httpRequestsAttempted: number
  http2xxResponsesSeen: number
  authStatusResponsesSeen: number
  localReadsAttempted: number
  localReadFailures: number
  authReadAttempts: number
  authReadSuccesses: number
}

export type ManifestMetricDiagnostic = {
  label: string
  type: string
  scope: "overview" | "detail" | string
  classification: MetricClassification
}

export type ReturnedMetricDiagnostic = {
  label: string
  type: MetricLine["type"]
}

export type MissingMetricDiagnostic = ManifestMetricDiagnostic

export type ProviderDiagnostics = {
  providerLoaded: boolean
  providerVersion: string | null
  authDetected: AuthDetectedStatus
  dataSourceReachable: DataSourceReachability
  lastSuccessfulRefreshAt: number | null
  manifestMetrics: ManifestMetricDiagnostic[]
  returnedMetrics: ReturnedMetricDiagnostic[]
  missingMetrics: MissingMetricDiagnostic[]
  lastError: string | null
  parserExecutionStatus: ParserExecutionStatus
  healthSummary: DiagnosticsHealth
  likelyCauses: DiagnosticsLikelyCause[]
  hostFacts: SafeHostFacts
}

export type MetricLine =
  | { type: "text"; label: string; value: string; color?: string; subtitle?: string }
  | {
      type: "progress"
      label: string
      used: number
      limit: number
      format: ProgressFormat
      resetsAt?: string
      periodDurationMs?: number
      color?: string
    }
  | { type: "badge"; label: string; text: string; color?: string; subtitle?: string }
  | { type: "barChart"; label: string; points: BarChartPoint[]; note?: string; color?: string }

export type ManifestLine = {
  type: "text" | "progress" | "badge" | "barChart"
  label: string
  scope: "overview" | "detail"
  classification?: MetricClassification
}

export type PluginLink = {
  label: string
  url: string
}

export type PluginOutput = {
  providerId: string
  displayName: string
  plan?: string
  lines: MetricLine[]
  iconUrl: string
  diagnostics?: ProviderDiagnostics
}

export type PluginMeta = {
  id: string
  name: string
  version?: string
  iconUrl: string
  brandColor?: string
  lines: ManifestLine[]
  links?: PluginLink[]
  /** Ordered list of primary metric candidates. Frontend picks first available. */
  primaryCandidates: string[]
}

export type PluginDisplayState = {
  meta: PluginMeta
  data: PluginOutput | null
  loading: boolean
  error: string | null
  lastManualRefreshAt: number | null
  lastUpdatedAt: number | null
}
