# Performance Audit Charter (PERF)

> Trigger: on demand; mandatory before an RC when startup-, memory-, or polling-affecting work has merged since the last run. Process: [AuditProcess.md](AuditProcess.md).

## Context

PulseBar is a macOS menu-bar app — the performance bar is "invisible resident": negligible idle CPU (App Nap friendly, see `src-tauri/src/app_nap.rs`), small memory footprint, instant panel open. It is not a throughput system; audit against residency costs, not latency SLOs.

## Scope

| Concern | Where to look | What to examine |
|---|---|---|
| Startup | `src-tauri/src/main.rs`, `lib.rs` setup, `webkit_config.rs` | Work done before first tray render; anything deferrable (provider probing, plugin injection) done eagerly |
| Idle cost | `app_nap.rs`, polling/refresh timers in hooks (`src/hooks/`) | Timer frequencies; work performed while panel closed; App Nap opt-outs and their justification |
| Subprocess spawning | process-exec call sites (TECHNICAL_DEBT #16) | Spawn frequency per refresh cycle; this is the known energy hotspot — measure before recommending the native-crate migration, its trigger is "energy complaints OR >25 plugins" |
| Rendering | `src/components/`, panel open path (`src-tauri/src/panel.rs`) | Re-render breadth on metric updates; memoization of provider rows; panel open-to-paint |
| IPC | Tauri command surface | Payload sizes; chatty command patterns per refresh |
| Memory | plugin runtime (`plugin_engine/runtime.rs`) | Per-plugin runtime cost; growth across long residency |

## Method

1. Evidence is measured, not inferred: use Instruments / `hyperfine` on launch, Activity Monitor energy impact over a 30-minute idle window, React DevTools profiler for render breadth. A performance finding without a number does not survive refutation.
2. Compare against the previous run's numbers in `history/` — the audit's real product is the trend line.
3. Recommendations must state expected improvement and regression risk; "could be faster" is discarded at synthesis.

## Out of scope

Micro-optimizations in code paths that run only on user interaction. Bundle-size work unless it measurably affects startup.

## Run history

Completed runs are recorded as individual files in [history/](history/) named `<CHARTER>-<YYYY-MM-DD>.md`.
