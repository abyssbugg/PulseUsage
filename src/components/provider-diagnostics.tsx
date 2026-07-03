import { Badge } from "@/components/ui/badge"
import { buildProviderDiagnostics } from "@/lib/provider-diagnostics"
import type {
  AuthDetectedStatus,
  DataSourceReachability,
  DiagnosticsHealth,
  DiagnosticsLikelyCause,
  ParserExecutionStatus,
  PluginDisplayState,
  ProviderDiagnostics,
} from "@/lib/plugin-types"

type ProviderDiagnosticsSectionProps = {
  plugin: PluginDisplayState
}

const HEALTH_LABELS: Record<DiagnosticsHealth, string> = {
  ok: "OK",
  warning: "Needs attention",
  error: "Error",
  unknown: "Unknown",
}

const AUTH_LABELS: Record<AuthDetectedStatus, string> = {
  detected: "Detected",
  notDetected: "Not detected",
  unknown: "Unknown",
}

const DATA_SOURCE_LABELS: Record<DataSourceReachability, string> = {
  reachable: "Reachable",
  unreachable: "Not reachable",
  unknown: "Unknown",
}

const PARSER_LABELS: Record<ParserExecutionStatus, string> = {
  notRun: "Not run",
  success: "Success",
  failed: "Failed",
}

const CAUSE_LABELS: Record<DiagnosticsLikelyCause, string> = {
  authMissing: "Auth missing",
  authRejected: "Auth rejected",
  dataSourceUnreachable: "Data source unreachable",
  parserError: "Parser error",
  manifestMismatch: "Manifest mismatch",
  noMetricsReturned: "No metrics returned",
  unknown: "Unknown",
}

export function ProviderDiagnosticsSection({ plugin }: ProviderDiagnosticsSectionProps) {
  const diagnostics = buildProviderDiagnostics(plugin)

  return (
    <section className="rounded-lg border border-border/80 bg-muted/20 p-3">
      <div className="mb-2 flex items-center justify-between gap-2">
        <h3 className="text-sm font-medium">Diagnostics</h3>
        <Badge variant="outline">{HEALTH_LABELS[diagnostics.healthSummary]}</Badge>
      </div>

      <div className="grid gap-1.5 text-xs text-muted-foreground">
        <DiagnosticRow label="Provider" value={providerLoadedLabel(diagnostics)} />
        <DiagnosticRow label="Auth" value={AUTH_LABELS[diagnostics.authDetected]} />
        <DiagnosticRow
          label="Data source"
          value={DATA_SOURCE_LABELS[diagnostics.dataSourceReachable]}
        />
        <DiagnosticRow
          label="Parser"
          value={PARSER_LABELS[diagnostics.parserExecutionStatus]}
        />
        <DiagnosticRow
          label="Last success"
          value={formatLastSuccess(diagnostics.lastSuccessfulRefreshAt)}
        />
      </div>

      <div className="mt-3 flex flex-wrap gap-1.5 text-[11px] text-muted-foreground">
        <span>HTTP attempted: {diagnostics.hostFacts.httpRequestsAttempted}</span>
        <span>HTTP 2xx: {diagnostics.hostFacts.http2xxResponsesSeen}</span>
        <span>Auth status: {diagnostics.hostFacts.authStatusResponsesSeen}</span>
        <span>Local reads: {diagnostics.hostFacts.localReadsAttempted}</span>
        <span>Local read failures: {diagnostics.hostFacts.localReadFailures}</span>
      </div>

      {diagnostics.missingMetrics.length > 0 && (
        <div className="mt-3">
          <div className="mb-1 text-xs font-medium">Missing metrics</div>
          <ul className="space-y-1">
            {diagnostics.missingMetrics.map((metric) => (
              <li
                key={`${metric.scope}-${metric.label}`}
                className="flex items-center justify-between gap-2 text-xs text-muted-foreground"
              >
                <span className="min-w-0 truncate">{metric.label}</span>
                <Badge variant="outline" className="h-5 text-[10px]">
                  {metric.classification}
                </Badge>
              </li>
            ))}
          </ul>
        </div>
      )}

      {diagnostics.lastError && (
        <div className="mt-3">
          <div className="mb-1 text-xs font-medium">Last error</div>
          <p className="break-words text-xs text-muted-foreground">
            {diagnostics.lastError}
          </p>
        </div>
      )}

      {diagnostics.likelyCauses.length > 0 && (
        <div className="mt-3 flex flex-wrap gap-1">
          {diagnostics.likelyCauses.map((cause) => (
            <Badge key={cause} variant="outline" className="h-5 text-[10px]">
              {CAUSE_LABELS[cause]}
            </Badge>
          ))}
        </div>
      )}
    </section>
  )
}

function DiagnosticRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between gap-2">
      <span>{label}</span>
      <span className="text-right text-foreground">{value}</span>
    </div>
  )
}

function providerLoadedLabel(diagnostics: ProviderDiagnostics): string {
  if (!diagnostics.providerLoaded) return "Not loaded"
  if (diagnostics.providerVersion) return `Loaded (${diagnostics.providerVersion})`
  return "Loaded"
}

function formatLastSuccess(value: number | null): string {
  if (value == null) return "Unavailable"
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return "Unavailable"
  return date.toLocaleString()
}
