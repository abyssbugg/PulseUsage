import { ProviderCard } from "@/components/provider-card"
import { ProviderDiagnosticsSection } from "@/components/provider-diagnostics"
import type { PluginDisplayState } from "@/lib/plugin-types"
import type { DisplayMode, ResetTimerDisplayMode, TimeFormatMode } from "@/lib/settings"
import { Home } from "lucide-react"

interface ProviderDetailPageProps {
  plugin: PluginDisplayState | null
  onRetry?: () => void
  displayMode: DisplayMode
  resetTimerDisplayMode: ResetTimerDisplayMode
  timeFormatMode?: TimeFormatMode
  onResetTimerDisplayModeToggle?: () => void
  onNavigateHome?: () => void
}

export function ProviderDetailPage({
  plugin,
  onRetry,
  displayMode,
  resetTimerDisplayMode,
  timeFormatMode = "auto",
  onResetTimerDisplayModeToggle,
  onNavigateHome,
}: ProviderDetailPageProps) {
  if (!plugin) {
    return (
      <div className="flex flex-col items-center justify-center text-center py-[var(--space-12)] px-[var(--space-4)]">
        <Home className="size-[var(--icon-xl)] text-muted-foreground mb-[var(--space-3)]" />
        <p className="text-sm font-medium text-foreground mb-[var(--space-1)]">
          Provider not found
        </p>
        <p className="text-[var(--font-size-2xs)] text-muted-foreground mb-[var(--space-4)] max-w-[200px]">
          This provider may have been disabled or removed.
        </p>
        {onNavigateHome && (
          <button
            type="button"
            onClick={onNavigateHome}
            className="text-[var(--font-size-2xs)] text-[var(--link)] hover:underline cursor-pointer"
          >
            Back to Overview
          </button>
        )}
      </div>
    )
  }

  return (
    <div className="space-y-3">
      <ProviderCard
        name={plugin.meta.name}
        plan={plugin.data?.plan}
        links={plugin.meta.links}
        showSeparator={false}
        loading={plugin.loading}
        error={plugin.error}
        lines={plugin.data?.lines ?? []}
        skeletonLines={plugin.meta.lines}
        lastManualRefreshAt={plugin.lastManualRefreshAt}
        lastUpdatedAt={plugin.lastUpdatedAt}
        onRetry={onRetry}
        scopeFilter="all"
        showUnavailableManifestLines
        displayMode={displayMode}
        resetTimerDisplayMode={resetTimerDisplayMode}
        timeFormatMode={timeFormatMode}
        onResetTimerDisplayModeToggle={onResetTimerDisplayModeToggle}
      />
      <ProviderDiagnosticsSection plugin={plugin} />
    </div>
  )
}
