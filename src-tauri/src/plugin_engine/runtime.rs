use crate::plugin_engine::diagnostics::{
    AuthDetected, DataSourceReachability, DiagnosticsHealth, DiagnosticsLikelyCause,
    ManifestMetricDiagnostic, MetricClassification, ParserExecutionStatus,
    ProbeDiagnosticsRecorder, ProviderDiagnostics, ReturnedMetricDiagnostic, SafeHostFacts,
    normalize_metric_classification, redact_diagnostic_text,
};
use crate::plugin_engine::host_api;
use crate::plugin_engine::manifest::LoadedPlugin;
use rquickjs::{Array, Context, Ctx, Error, Object, Promise, Runtime, Value};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, Instant};

const PROBE_TIMEOUT_SECS: u64 = 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ProgressFormat {
    Percent,
    Dollars,
    Count { suffix: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarChartPoint {
    label: String,
    value: f64,
    #[serde(rename = "valueLabel")]
    value_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MetricLine {
    Text {
        label: String,
        value: String,
        color: Option<String>,
        subtitle: Option<String>,
    },
    Progress {
        label: String,
        used: f64,
        limit: f64,
        format: ProgressFormat,
        #[serde(rename = "resetsAt")]
        resets_at: Option<String>,
        #[serde(rename = "periodDurationMs")]
        period_duration_ms: Option<u64>,
        color: Option<String>,
    },
    Badge {
        label: String,
        text: String,
        color: Option<String>,
        subtitle: Option<String>,
    },
    #[serde(rename = "barChart")]
    BarChart {
        label: String,
        points: Vec<BarChartPoint>,
        note: Option<String>,
        color: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginOutput {
    pub provider_id: String,
    pub display_name: String,
    pub plan: Option<String>,
    pub lines: Vec<MetricLine>,
    pub icon_url: String,
    pub diagnostics: ProviderDiagnostics,
}

pub fn run_probe(plugin: &LoadedPlugin, app_data_dir: &PathBuf, app_version: &str) -> PluginOutput {
    run_probe_with_timeout(
        plugin,
        app_data_dir,
        app_version,
        Duration::from_secs(PROBE_TIMEOUT_SECS),
    )
}

fn run_probe_with_timeout(
    plugin: &LoadedPlugin,
    app_data_dir: &PathBuf,
    app_version: &str,
    timeout: Duration,
) -> PluginOutput {
    let diagnostics_recorder = ProbeDiagnosticsRecorder::default();
    let fallback = error_output_with_facts(
        plugin,
        "runtime error".to_string(),
        diagnostics_recorder.snapshot(),
    );
    let timeout_message = probe_timeout_message(timeout);
    let deadline_at = Instant::now()
        .checked_add(timeout)
        .unwrap_or_else(Instant::now);
    let deadline = host_api::ProbeDeadline::at(deadline_at);

    let rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return fallback,
    };
    rt.set_interrupt_handler(Some(Box::new(move || Instant::now() >= deadline_at)));

    let ctx = match Context::full(&rt) {
        Ok(ctx) => ctx,
        Err(_) => return fallback,
    };

    let plugin_id = plugin.manifest.id.clone();
    let display_name = plugin.manifest.name.clone();
    let entry_script = plugin.entry_script.clone();
    let icon_url = plugin.icon_data_url.clone();
    let app_data = app_data_dir.clone();

    ctx.with(|ctx| {
        if host_api::inject_host_api_with_deadline(
            &ctx,
            &plugin_id,
            &app_data,
            app_version,
            deadline,
            diagnostics_recorder.clone(),
        )
        .is_err()
        {
            if deadline.has_elapsed() {
                return error_output_with_facts(
                    plugin,
                    timeout_message.clone(),
                    diagnostics_recorder.snapshot(),
                );
            }
            return error_output_with_facts(
                plugin,
                "host api injection failed".to_string(),
                diagnostics_recorder.snapshot(),
            );
        }
        if host_api::patch_http_wrapper(&ctx).is_err() {
            if deadline.has_elapsed() {
                return error_output_with_facts(
                    plugin,
                    timeout_message.clone(),
                    diagnostics_recorder.snapshot(),
                );
            }
            return error_output_with_facts(
                plugin,
                "http wrapper patch failed".to_string(),
                diagnostics_recorder.snapshot(),
            );
        }
        if host_api::patch_ls_wrapper(&ctx).is_err() {
            if deadline.has_elapsed() {
                return error_output_with_facts(
                    plugin,
                    timeout_message.clone(),
                    diagnostics_recorder.snapshot(),
                );
            }
            return error_output_with_facts(
                plugin,
                "ls wrapper patch failed".to_string(),
                diagnostics_recorder.snapshot(),
            );
        }
        if host_api::patch_ccusage_wrapper(&ctx).is_err() {
            if deadline.has_elapsed() {
                return error_output_with_facts(
                    plugin,
                    timeout_message.clone(),
                    diagnostics_recorder.snapshot(),
                );
            }
            return error_output_with_facts(
                plugin,
                "ccusage wrapper patch failed".to_string(),
                diagnostics_recorder.snapshot(),
            );
        }
        if host_api::inject_utils(&ctx).is_err() {
            if deadline.has_elapsed() {
                return error_output_with_facts(
                    plugin,
                    timeout_message.clone(),
                    diagnostics_recorder.snapshot(),
                );
            }
            return error_output_with_facts(
                plugin,
                "utils injection failed".to_string(),
                diagnostics_recorder.snapshot(),
            );
        }

        if ctx.eval::<(), _>(entry_script.as_bytes()).is_err() {
            if deadline.has_elapsed() {
                return error_output_with_facts(
                    plugin,
                    timeout_message.clone(),
                    diagnostics_recorder.snapshot(),
                );
            }
            return error_output_with_facts(
                plugin,
                "script eval failed".to_string(),
                diagnostics_recorder.snapshot(),
            );
        }

        let globals = ctx.globals();
        let plugin_obj: Object = match globals.get("__pulseusage_plugin") {
            Ok(obj) => obj,
            Err(_) => {
                return error_output_with_facts(
                    plugin,
                    "missing __pulseusage_plugin".to_string(),
                    diagnostics_recorder.snapshot(),
                );
            }
        };

        let probe_fn: rquickjs::Function = match plugin_obj.get("probe") {
            Ok(f) => f,
            Err(_) => {
                return error_output_with_facts(
                    plugin,
                    "missing probe()".to_string(),
                    diagnostics_recorder.snapshot(),
                );
            }
        };

        let probe_ctx: Value = globals
            .get("__pulseusage_ctx")
            .unwrap_or_else(|_| Value::new_undefined(ctx.clone()));

        let result_value: Value = match probe_fn.call((probe_ctx,)) {
            Ok(r) => r,
            Err(_) => {
                if deadline.has_elapsed() {
                    return error_output_with_facts(
                        plugin,
                        timeout_message.clone(),
                        diagnostics_recorder.snapshot(),
                    );
                }
                return error_output_with_facts(
                    plugin,
                    extract_error_string(&ctx),
                    diagnostics_recorder.snapshot(),
                );
            }
        };
        if deadline.has_elapsed() {
            return error_output_with_facts(
                plugin,
                timeout_message.clone(),
                diagnostics_recorder.snapshot(),
            );
        }
        let result: Object = if result_value.is_promise() {
            let promise: Promise = match result_value.into_promise() {
                Some(promise) => promise,
                None => {
                    return error_output_with_facts(
                        plugin,
                        "probe() returned invalid promise".to_string(),
                        diagnostics_recorder.snapshot(),
                    );
                }
            };
            match promise.finish::<Object>() {
                Ok(obj) => obj,
                Err(Error::WouldBlock) => {
                    return error_output_with_facts(
                        plugin,
                        "probe() returned unresolved promise".to_string(),
                        diagnostics_recorder.snapshot(),
                    );
                }
                Err(_) => {
                    if deadline.has_elapsed() {
                        return error_output_with_facts(
                            plugin,
                            timeout_message.clone(),
                            diagnostics_recorder.snapshot(),
                        );
                    }
                    return error_output_with_facts(
                        plugin,
                        extract_error_string(&ctx),
                        diagnostics_recorder.snapshot(),
                    );
                }
            }
        } else {
            match result_value.into_object() {
                Some(obj) => obj,
                None => {
                    return error_output_with_facts(
                        plugin,
                        "probe() returned non-object".to_string(),
                        diagnostics_recorder.snapshot(),
                    );
                }
            }
        };
        if deadline.has_elapsed() {
            return error_output_with_facts(
                plugin,
                timeout_message.clone(),
                diagnostics_recorder.snapshot(),
            );
        }

        let plan: Option<String> = result
            .get::<_, String>("plan")
            .ok()
            .filter(|s| !s.is_empty());

        let lines = match parse_lines(&result) {
            Ok(lines) if !lines.is_empty() => lines,
            Ok(_) => vec![error_line("no lines returned".to_string())],
            Err(msg) => vec![error_line(msg)],
        };

        PluginOutput {
            provider_id: plugin_id,
            display_name,
            plan,
            diagnostics: build_diagnostics(plugin, &lines, diagnostics_recorder.snapshot()),
            lines,
            icon_url,
        }
    })
}

fn parse_lines(result: &Object) -> Result<Vec<MetricLine>, String> {
    let lines: Array = result
        .get("lines")
        .map_err(|_| "missing lines".to_string())?;

    let mut out = Vec::new();
    let len = lines.len();
    for idx in 0..len {
        let line: Object = lines
            .get(idx)
            .map_err(|_| format!("invalid line at index {}", idx))?;

        let line_type: String = line.get("type").unwrap_or_default();
        let label = line.get::<_, String>("label").unwrap_or_default();
        let color = line.get::<_, String>("color").ok();
        let subtitle = line.get::<_, String>("subtitle").ok();

        match line_type.as_str() {
            "text" => {
                let value = line.get::<_, String>("value").unwrap_or_default();
                out.push(MetricLine::Text {
                    label,
                    value,
                    color,
                    subtitle,
                });
            }
            "progress" => {
                let used_value: Value = match line.get("used") {
                    Ok(v) => v,
                    Err(_) => {
                        out.push(error_line(format!(
                            "progress line at index {} missing used",
                            idx
                        )));
                        continue;
                    }
                };
                let used = match used_value.as_number() {
                    Some(n) => n,
                    None => {
                        out.push(error_line(format!(
                            "progress line at index {} invalid used (expected number)",
                            idx
                        )));
                        continue;
                    }
                };

                let limit_value: Value = match line.get("limit") {
                    Ok(v) => v,
                    Err(_) => {
                        out.push(error_line(format!(
                            "progress line at index {} missing limit",
                            idx
                        )));
                        continue;
                    }
                };
                let limit = match limit_value.as_number() {
                    Some(n) => n,
                    None => {
                        out.push(error_line(format!(
                            "progress line at index {} invalid limit (expected number)",
                            idx
                        )));
                        continue;
                    }
                };

                if !used.is_finite() || used < 0.0 {
                    out.push(error_line(format!(
                        "progress line at index {} invalid used: {}",
                        idx, used
                    )));
                    continue;
                }
                if !limit.is_finite() || limit <= 0.0 {
                    out.push(error_line(format!(
                        "progress line at index {} invalid limit: {}",
                        idx, limit
                    )));
                    continue;
                }

                let format_obj: Object = match line.get("format") {
                    Ok(obj) => obj,
                    Err(_) => {
                        out.push(error_line(format!(
                            "progress line at index {} missing format",
                            idx
                        )));
                        continue;
                    }
                };
                let kind_value: Value = match format_obj.get("kind") {
                    Ok(v) => v,
                    Err(_) => {
                        out.push(error_line(format!(
                            "progress line at index {} missing format.kind",
                            idx
                        )));
                        continue;
                    }
                };
                let kind = match kind_value.as_string() {
                    Some(s) => s.to_string().unwrap_or_default(),
                    None => {
                        out.push(error_line(format!(
                            "progress line at index {} invalid format.kind (expected string)",
                            idx
                        )));
                        continue;
                    }
                };
                let format = match kind.as_str() {
                    "percent" => {
                        if limit != 100.0 {
                            out.push(error_line(format!(
                                "progress line at index {}: percent format requires limit=100 (got {})",
                                idx, limit
                            )));
                            continue;
                        }
                        ProgressFormat::Percent
                    }
                    "dollars" => ProgressFormat::Dollars,
                    "count" => {
                        let suffix_value: Value = match format_obj.get("suffix") {
                            Ok(v) => v,
                            Err(_) => {
                                out.push(error_line(format!(
                                    "progress line at index {}: count format missing suffix",
                                    idx
                                )));
                                continue;
                            }
                        };
                        let suffix = match suffix_value.as_string() {
                            Some(s) => s.to_string().unwrap_or_default(),
                            None => {
                                out.push(error_line(format!(
                                    "progress line at index {}: count format suffix must be a string",
                                    idx
                                )));
                                continue;
                            }
                        };
                        let suffix = suffix.trim().to_string();
                        if suffix.is_empty() {
                            out.push(error_line(format!(
                                "progress line at index {}: count format suffix must be non-empty",
                                idx
                            )));
                            continue;
                        }
                        ProgressFormat::Count { suffix }
                    }
                    _ => {
                        out.push(error_line(format!(
                            "progress line at index {} invalid format.kind: {}",
                            idx, kind
                        )));
                        continue;
                    }
                };

                let resets_at = match line.get::<_, Value>("resetsAt") {
                    Ok(v) => {
                        if v.is_null() || v.is_undefined() {
                            None
                        } else if let Some(s) = v.as_string() {
                            let raw = s.to_string().unwrap_or_default();
                            let value = raw.trim().to_string();
                            if value.is_empty() {
                                None
                            } else {
                                let parsed = time::OffsetDateTime::parse(
                                    &value,
                                    &time::format_description::well_known::Rfc3339,
                                );
                                if parsed.is_ok() {
                                    Some(value)
                                } else {
                                    // ISO-like but missing timezone: assume UTC.
                                    let is_missing_tz =
                                        value.contains('T') && !value.ends_with('Z') && {
                                            let tail = value.splitn(2, 'T').nth(1).unwrap_or("");
                                            !tail.contains('+') && !tail.contains('-')
                                        };
                                    if is_missing_tz {
                                        let with_z = format!("{}Z", value);
                                        let parsed_with_z = time::OffsetDateTime::parse(
                                            &with_z,
                                            &time::format_description::well_known::Rfc3339,
                                        );
                                        if parsed_with_z.is_ok() {
                                            Some(with_z)
                                        } else {
                                            log::warn!(
                                                "invalid resetsAt at index {} (value='{}'), omitting",
                                                idx,
                                                raw
                                            );
                                            None
                                        }
                                    } else {
                                        log::warn!(
                                            "invalid resetsAt at index {} (value='{}'), omitting",
                                            idx,
                                            raw
                                        );
                                        None
                                    }
                                }
                            }
                        } else {
                            log::warn!("invalid resetsAt at index {} (non-string), omitting", idx);
                            None
                        }
                    }
                    Err(_) => None,
                };

                // Parse optional periodDurationMs
                let period_duration_ms: Option<u64> = match line.get::<_, Value>("periodDurationMs")
                {
                    Ok(val) => {
                        if val.is_null() || val.is_undefined() {
                            None
                        } else if let Some(n) = val.as_number() {
                            let ms = n as u64;
                            if ms > 0 {
                                Some(ms)
                            } else {
                                log::warn!(
                                    "periodDurationMs at index {} must be positive, omitting",
                                    idx
                                );
                                None
                            }
                        } else {
                            log::warn!(
                                "invalid periodDurationMs at index {} (non-number), omitting",
                                idx
                            );
                            None
                        }
                    }
                    Err(_) => None,
                };

                out.push(MetricLine::Progress {
                    label,
                    used,
                    limit,
                    format,
                    resets_at,
                    period_duration_ms,
                    color,
                });
            }
            "badge" => {
                let text = line.get::<_, String>("text").unwrap_or_default();
                out.push(MetricLine::Badge {
                    label,
                    text,
                    color,
                    subtitle,
                });
            }
            "barChart" => {
                let (chart, errors) = parse_bar_chart_line(&line, idx, label, color);
                for message in errors {
                    out.push(error_line(message));
                }
                if let Some(chart) = chart {
                    out.push(chart);
                }
            }
            _ => {
                out.push(error_line(format!(
                    "unknown line type at index {}: {}",
                    idx, line_type
                )));
            }
        }
    }

    Ok(out)
}

// Upper bound on barChart points parsed from a plugin. The chart is daily
// history (plugins emit ~31), so a year of points is generous headroom while
// keeping the loop and allocations bounded — parse_lines runs natively after
// the JS returns, so the probe's interrupt-based timeout can't cap it here.
const MAX_BAR_CHART_POINTS: usize = 366;

// Parses a barChart line, keeping its point/value/note validation out of
// parse_lines. Returns the built line (when at least one point is valid) plus
// any per-point error messages the caller should surface as error lines.
fn parse_bar_chart_line<'js>(
    line: &Object<'js>,
    idx: usize,
    label: String,
    color: Option<String>,
) -> (Option<MetricLine>, Vec<String>) {
    let mut errors: Vec<String> = Vec::new();

    let points_array: Array = match line.get("points") {
        Ok(points) => points,
        Err(_) => {
            errors.push(format!("barChart line at index {} missing points", idx));
            return (None, errors);
        }
    };

    // Bound the loop to a plugin-independent maximum so a huge points array
    // can't exhaust CPU/memory in this native (non-interruptible) path.
    let total_points = points_array.len();
    let scan_count = total_points.min(MAX_BAR_CHART_POINTS);
    if total_points > MAX_BAR_CHART_POINTS {
        log::warn!(
            "barChart line at index {} has {} points; capping at {}",
            idx,
            total_points,
            MAX_BAR_CHART_POINTS
        );
    }

    let mut points = Vec::new();
    for point_idx in 0..scan_count {
        let point: Object = match points_array.get(point_idx) {
            Ok(point) => point,
            Err(_) => {
                errors.push(format!(
                    "barChart line at index {} has invalid point at index {}",
                    idx, point_idx
                ));
                continue;
            }
        };
        let point_label = point.get::<_, String>("label").unwrap_or_default();
        let point_label = point_label.trim().to_string();
        if point_label.is_empty() {
            errors.push(format!(
                "barChart line at index {} has empty point label at index {}",
                idx, point_idx
            ));
            continue;
        }

        let value: Value = match point.get("value") {
            Ok(v) => v,
            Err(_) => {
                errors.push(format!(
                    "barChart line at index {} point {} missing value",
                    idx, point_idx
                ));
                continue;
            }
        };
        let value = match value.as_number() {
            Some(n) if n.is_finite() && n >= 0.0 => n,
            _ => {
                errors.push(format!(
                    "barChart line at index {} point {} invalid value",
                    idx, point_idx
                ));
                continue;
            }
        };

        let value_label = match point.get::<_, Value>("valueLabel") {
            Ok(v) => {
                if v.is_null() || v.is_undefined() {
                    None
                } else if let Some(s) = v.as_string() {
                    let value = s.to_string().unwrap_or_default();
                    let trimmed = value.trim().to_string();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed)
                    }
                } else {
                    log::warn!(
                        "invalid barChart valueLabel at line {} point {}, omitting",
                        idx,
                        point_idx
                    );
                    None
                }
            }
            Err(_) => None,
        };

        points.push(BarChartPoint {
            label: point_label,
            value,
            value_label,
        });
    }

    if points.is_empty() {
        errors.push(format!(
            "barChart line at index {} has no valid points",
            idx
        ));
        return (None, errors);
    }

    let note = match line.get::<_, Value>("note") {
        Ok(v) => {
            if v.is_null() || v.is_undefined() {
                None
            } else if let Some(s) = v.as_string() {
                let value = s.to_string().unwrap_or_default();
                let trimmed = value.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            } else {
                log::warn!("invalid note at index {} (non-string), omitting", idx);
                None
            }
        }
        Err(_) => None,
    };

    (
        Some(MetricLine::BarChart {
            label,
            points,
            note,
            color,
        }),
        errors,
    )
}

fn error_output_with_facts(
    plugin: &LoadedPlugin,
    message: String,
    host_facts: SafeHostFacts,
) -> PluginOutput {
    let lines = vec![error_line(message)];
    PluginOutput {
        provider_id: plugin.manifest.id.clone(),
        display_name: plugin.manifest.name.clone(),
        plan: None,
        diagnostics: build_diagnostics(plugin, &lines, host_facts),
        lines,
        icon_url: plugin.icon_data_url.clone(),
    }
}

fn build_diagnostics(
    plugin: &LoadedPlugin,
    lines: &[MetricLine],
    host_facts: SafeHostFacts,
) -> ProviderDiagnostics {
    let manifest_metrics: Vec<ManifestMetricDiagnostic> = plugin
        .manifest
        .lines
        .iter()
        .map(|line| ManifestMetricDiagnostic {
            label: line.label.clone(),
            line_type: line.line_type.clone(),
            scope: line.scope.clone(),
            classification: normalize_metric_classification(line.classification.as_deref()),
        })
        .collect();
    let returned_metrics: Vec<ReturnedMetricDiagnostic> = lines
        .iter()
        .filter(|line| !is_error_line(line))
        .map(|line| ReturnedMetricDiagnostic {
            label: metric_line_label(line).to_string(),
            line_type: metric_line_type(line).to_string(),
        })
        .collect();
    let returned_labels: std::collections::HashSet<&str> = returned_metrics
        .iter()
        .map(|line| line.label.as_str())
        .collect();
    let missing_metrics: Vec<ManifestMetricDiagnostic> = manifest_metrics
        .iter()
        .filter(|line| !returned_labels.contains(line.label.as_str()))
        .cloned()
        .collect();
    let last_error = last_error_from_lines(lines).map(|message| redact_diagnostic_text(&message));
    let parser_execution_status = if last_error.is_some() {
        ParserExecutionStatus::Failed
    } else {
        ParserExecutionStatus::Success
    };
    let auth_detected = derive_auth_detected(&host_facts);
    let data_source_reachable = derive_data_source_reachable(&host_facts);
    let likely_causes = derive_likely_causes(
        parser_execution_status,
        last_error.as_deref(),
        &host_facts,
        data_source_reachable,
        &missing_metrics,
        &returned_metrics,
    );
    let health_summary =
        derive_health_summary(parser_execution_status, &host_facts, &missing_metrics);

    ProviderDiagnostics {
        provider_loaded: true,
        provider_version: Some(plugin.manifest.version.clone()),
        auth_detected,
        data_source_reachable,
        last_successful_refresh_at: None,
        manifest_metrics,
        returned_metrics,
        missing_metrics,
        last_error,
        parser_execution_status,
        health_summary,
        likely_causes,
        host_facts,
    }
}

fn metric_line_label(line: &MetricLine) -> &str {
    match line {
        MetricLine::Text { label, .. }
        | MetricLine::Progress { label, .. }
        | MetricLine::Badge { label, .. }
        | MetricLine::BarChart { label, .. } => label,
    }
}

fn metric_line_type(line: &MetricLine) -> &'static str {
    match line {
        MetricLine::Text { .. } => "text",
        MetricLine::Progress { .. } => "progress",
        MetricLine::Badge { .. } => "badge",
        MetricLine::BarChart { .. } => "barChart",
    }
}

fn is_error_line(line: &MetricLine) -> bool {
    matches!(line, MetricLine::Badge { label, .. } if label == "Error")
}

fn last_error_from_lines(lines: &[MetricLine]) -> Option<String> {
    lines.iter().rev().find_map(|line| match line {
        MetricLine::Badge { label, text, .. } if label == "Error" => {
            Some(error_message_or_default(text))
        }
        _ => None,
    })
}

fn error_message_or_default(text: &str) -> String {
    if text.trim().is_empty() {
        "Couldn't update data. Try again?".to_string()
    } else {
        text.to_string()
    }
}

fn has_auth_keyword(text: &str) -> bool {
    text.split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(|token| {
            matches!(
                token,
                "auth" | "login" | "token" | "credential" | "keychain"
            )
        })
}

fn has_required_missing_metric(missing_metrics: &[ManifestMetricDiagnostic]) -> bool {
    missing_metrics
        .iter()
        .any(|metric| metric.classification == MetricClassification::Required)
}

fn derive_auth_detected(host_facts: &SafeHostFacts) -> AuthDetected {
    if host_facts.auth_read_successes > 0 {
        AuthDetected::Detected
    } else if host_facts.auth_read_attempts > 0 {
        AuthDetected::NotDetected
    } else {
        AuthDetected::Unknown
    }
}

fn derive_data_source_reachable(host_facts: &SafeHostFacts) -> DataSourceReachability {
    if host_facts.http_2xx_responses_seen > 0
        || (host_facts.local_reads_attempted > 0
            && host_facts.local_read_failures < host_facts.local_reads_attempted)
    {
        DataSourceReachability::Reachable
    } else if host_facts.http_requests_attempted > 0 || host_facts.local_reads_attempted > 0 {
        DataSourceReachability::Unreachable
    } else {
        DataSourceReachability::Unknown
    }
}

fn derive_health_summary(
    parser_execution_status: ParserExecutionStatus,
    host_facts: &SafeHostFacts,
    missing_metrics: &[ManifestMetricDiagnostic],
) -> DiagnosticsHealth {
    if parser_execution_status == ParserExecutionStatus::Failed {
        return DiagnosticsHealth::Error;
    }
    if parser_execution_status == ParserExecutionStatus::NotRun {
        return DiagnosticsHealth::Unknown;
    }
    if host_facts.auth_status_responses_seen > 0 || has_required_missing_metric(missing_metrics) {
        return DiagnosticsHealth::Warning;
    }
    DiagnosticsHealth::Ok
}

fn derive_likely_causes(
    parser_execution_status: ParserExecutionStatus,
    last_error: Option<&str>,
    host_facts: &SafeHostFacts,
    data_source_reachable: DataSourceReachability,
    missing_metrics: &[ManifestMetricDiagnostic],
    returned_metrics: &[ReturnedMetricDiagnostic],
) -> Vec<DiagnosticsLikelyCause> {
    let mut causes = Vec::new();
    push_cause_if(
        &mut causes,
        parser_execution_status == ParserExecutionStatus::Failed,
        DiagnosticsLikelyCause::ParserError,
    );
    push_cause_if(
        &mut causes,
        host_facts.auth_status_responses_seen > 0,
        DiagnosticsLikelyCause::AuthRejected,
    );
    let lower_error = last_error.unwrap_or_default().to_ascii_lowercase();
    push_cause_if(
        &mut causes,
        host_facts.auth_status_responses_seen == 0 && has_auth_keyword(&lower_error),
        DiagnosticsLikelyCause::AuthMissing,
    );
    push_cause_if(
        &mut causes,
        data_source_reachable == DataSourceReachability::Unreachable,
        DiagnosticsLikelyCause::DataSourceUnreachable,
    );
    push_cause_if(
        &mut causes,
        has_required_missing_metric(missing_metrics),
        DiagnosticsLikelyCause::ManifestMismatch,
    );
    push_cause_if(
        &mut causes,
        returned_metrics.is_empty() && last_error.is_none(),
        DiagnosticsLikelyCause::NoMetricsReturned,
    );
    causes
}

fn push_cause_if(
    causes: &mut Vec<DiagnosticsLikelyCause>,
    condition: bool,
    cause: DiagnosticsLikelyCause,
) {
    if condition && !causes.contains(&cause) {
        causes.push(cause);
    }
}

fn extract_error_string(ctx: &Ctx<'_>) -> String {
    let exc = ctx.catch();
    if exc.is_null() || exc.is_undefined() {
        return "The plugin failed, try again or contact plugin author.".to_string();
    }
    if let Some(str_val) = exc.as_string() {
        let message: String = str_val.to_string().unwrap_or_default();
        let trimmed = message.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    "The plugin failed, try again or contact plugin author.".to_string()
}

fn probe_timeout_message(timeout: Duration) -> String {
    if timeout.subsec_millis() == 0 {
        return format!("probe timed out after {}s", timeout.as_secs());
    }
    if timeout.as_secs() == 0 {
        return format!("probe timed out after {}ms", timeout.as_millis());
    }
    format!("probe timed out after {:.3}s", timeout.as_secs_f64())
}

fn error_line(message: String) -> MetricLine {
    MetricLine::Badge {
        label: "Error".to_string(),
        text: message,
        color: Some("#ef4444".to_string()),
        subtitle: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin_engine::manifest::{LoadedPlugin, ManifestLine, PluginManifest};
    use serde_json::Value as JsonValue;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_plugin(entry_script: &str) -> LoadedPlugin {
        LoadedPlugin {
            manifest: PluginManifest {
                schema_version: 1,
                id: "test".to_string(),
                name: "Test".to_string(),
                version: "0.0.0".to_string(),
                entry: "plugin.js".to_string(),
                icon: "icon.svg".to_string(),
                brand_color: None,
                lines: vec![],
                links: vec![],
            },
            plugin_dir: PathBuf::from("."),
            entry_script: entry_script.to_string(),
            icon_data_url: "data:image/svg+xml;base64,".to_string(),
        }
    }

    fn test_plugin_with_manifest_lines(
        entry_script: &str,
        lines: Vec<ManifestLine>,
    ) -> LoadedPlugin {
        let mut plugin = test_plugin(entry_script);
        plugin.manifest.version = "0.1.0".to_string();
        plugin.manifest.lines = lines;
        plugin
    }

    fn temp_app_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        std::env::temp_dir().join(format!("pulseusage-test-{}-{}", label, nanos))
    }

    fn error_text(output: PluginOutput) -> String {
        match output.lines.first() {
            Some(MetricLine::Badge { text, .. }) => text.clone(),
            other => panic!("expected error badge, got {:?}", other),
        }
    }

    #[test]
    fn run_probe_returns_thrown_string_from_sync_error() {
        let plugin = test_plugin(
            r#"
            globalThis.__pulseusage_plugin = {
                probe() {
                    throw "boom";
                }
            };
            "#,
        );
        let output = run_probe(&plugin, &temp_app_dir("sync"), "0.0.0");
        assert_eq!(error_text(output), "boom");
    }

    #[test]
    fn run_probe_returns_thrown_string_from_async_error() {
        let plugin = test_plugin(
            r#"
            globalThis.__pulseusage_plugin = {
                probe: async function () {
                    throw "boom";
                }
            };
            "#,
        );
        let output = run_probe(&plugin, &temp_app_dir("async"), "0.0.0");
        assert_eq!(error_text(output), "boom");
    }

    #[test]
    fn run_probe_times_out_cpu_bound_script() {
        let plugin = test_plugin(
            r#"
            globalThis.__pulseusage_plugin = {
                probe() {
                    while (true) {}
                }
            };
            "#,
        );

        let output = run_probe_with_timeout(
            &plugin,
            &temp_app_dir("timeout"),
            "0.0.0",
            Duration::from_millis(5),
        );

        assert_eq!(error_text(output), "probe timed out after 5ms");
    }

    #[test]
    fn run_probe_includes_safe_diagnostics_and_missing_manifest_metrics() {
        let plugin = test_plugin_with_manifest_lines(
            r#"
            globalThis.__pulseusage_plugin = {
                probe(ctx) {
                    try {
                        ctx.host.fs.readText("missing-diagnostics-fixture");
                    } catch (e) {}
                    return {
                        lines: [
                            ctx.line.progress({
                                label: "Base Credits",
                                used: 10,
                                limit: 100,
                                format: { kind: "count", suffix: "credits" }
                            })
                        ]
                    };
                }
            };
            "#,
            vec![
                ManifestLine {
                    line_type: "progress".to_string(),
                    label: "Base Credits".to_string(),
                    scope: "overview".to_string(),
                    primary_order: None,
                    classification: Some("required".to_string()),
                },
                ManifestLine {
                    line_type: "text".to_string(),
                    label: "Bonus Credits".to_string(),
                    scope: "detail".to_string(),
                    primary_order: None,
                    classification: Some("optional".to_string()),
                },
                ManifestLine {
                    line_type: "badge".to_string(),
                    label: "Plan".to_string(),
                    scope: "overview".to_string(),
                    primary_order: None,
                    classification: None,
                },
            ],
        );

        let output = run_probe(&plugin, &temp_app_dir("diagnostics"), "0.0.0");

        assert!(output.diagnostics.provider_loaded);
        assert_eq!(
            output.diagnostics.provider_version.as_deref(),
            Some("0.1.0")
        );
        assert_eq!(
            output.diagnostics.parser_execution_status,
            ParserExecutionStatus::Success
        );
        assert_eq!(output.diagnostics.auth_detected, AuthDetected::Unknown);
        assert_eq!(
            output.diagnostics.data_source_reachable,
            DataSourceReachability::Unreachable
        );
        assert_eq!(output.diagnostics.host_facts.local_reads_attempted, 1);
        assert_eq!(output.diagnostics.host_facts.local_read_failures, 1);
        assert_eq!(output.diagnostics.returned_metrics.len(), 1);
        assert_eq!(output.diagnostics.missing_metrics.len(), 2);
        assert_eq!(
            output.diagnostics.missing_metrics[0].classification,
            MetricClassification::Optional
        );
        assert_eq!(
            output.diagnostics.missing_metrics[1].classification,
            MetricClassification::Unknown
        );
    }

    #[test]
    fn run_probe_diagnostics_redacts_sensitive_error_text() {
        let raw_email = ["user", "@", "example.invalid"].concat();
        let raw_secret = ["sk", "-", "test", "-", "secret", "-1234567890"].concat();
        let raw_path = ["/", "Users", "/", "sample", "/.config/app"].concat();
        let raw_url = "https://example.invalid/path?token=abc";
        let raw_error = format!("Failed for {raw_email} with {raw_secret} at {raw_path} {raw_url}");
        let js_error = serde_json::to_string(&raw_error).expect("serialize JS error string");
        let plugin_source = format!(
            r#"
            globalThis.__pulseusage_plugin = {{
                probe() {{
                    throw {js_error};
                }}
            }};
            "#,
        );
        let plugin = test_plugin(&plugin_source);

        let output = run_probe(&plugin, &temp_app_dir("diagnostics-redaction"), "0.0.0");
        let last_error = output
            .diagnostics
            .last_error
            .as_deref()
            .expect("diagnostic error");
        assert!(!last_error.contains(&raw_email));
        assert!(!last_error.contains(&raw_secret));
        assert!(!last_error.contains(&raw_path));
        assert!(!last_error.contains(raw_url));
        assert!(last_error.contains("[REDACTED]"));
        assert!(last_error.contains("[URL]"));
    }

    #[test]
    fn run_probe_marks_mixed_error_output_as_failed() {
        let plugin = test_plugin(
            r#"
            globalThis.__pulseusage_plugin = {
                probe(ctx) {
                    return {
                        lines: [
                            ctx.line.progress({
                                label: "Base Credits",
                                used: 10,
                                limit: 100,
                                format: { kind: "count", suffix: "credits" }
                            }),
                            ctx.line.badge({ label: "Error", text: "Token expired" })
                        ]
                    };
                }
            };
            "#,
        );

        let output = run_probe(&plugin, &temp_app_dir("diagnostics-mixed-error"), "0.0.0");

        assert_eq!(
            output.diagnostics.parser_execution_status,
            ParserExecutionStatus::Failed
        );
        assert_eq!(
            output.diagnostics.last_error.as_deref(),
            Some("Token expired")
        );
        assert_eq!(output.diagnostics.returned_metrics.len(), 1);
        assert!(
            output
                .diagnostics
                .likely_causes
                .contains(&DiagnosticsLikelyCause::ParserError)
        );
    }

    #[test]
    fn derive_likely_causes_uses_auth_token_boundaries_and_required_missing_metrics() {
        let optional_missing = vec![ManifestMetricDiagnostic {
            label: "Bonus".to_string(),
            line_type: "text".to_string(),
            scope: "detail".to_string(),
            classification: MetricClassification::Optional,
        }];
        let required_missing = vec![ManifestMetricDiagnostic {
            label: "Base Credits".to_string(),
            line_type: "progress".to_string(),
            scope: "overview".to_string(),
            classification: MetricClassification::Required,
        }];
        let returned_metrics = vec![ReturnedMetricDiagnostic {
            label: "Base Credits".to_string(),
            line_type: "progress".to_string(),
        }];

        let author_causes = derive_likely_causes(
            ParserExecutionStatus::Success,
            Some("author lookup failed"),
            &SafeHostFacts::default(),
            DataSourceReachability::Unknown,
            &optional_missing,
            &returned_metrics,
        );
        assert!(!author_causes.contains(&DiagnosticsLikelyCause::AuthMissing));
        assert!(!author_causes.contains(&DiagnosticsLikelyCause::ManifestMismatch));

        let token_causes = derive_likely_causes(
            ParserExecutionStatus::Success,
            Some("token missing"),
            &SafeHostFacts::default(),
            DataSourceReachability::Unknown,
            &required_missing,
            &returned_metrics,
        );
        assert!(token_causes.contains(&DiagnosticsLikelyCause::AuthMissing));
        assert!(token_causes.contains(&DiagnosticsLikelyCause::ManifestMismatch));
    }

    #[test]
    fn progress_resets_at_serializes_as_resets_at_camelcase() {
        let line = MetricLine::Progress {
            label: "Session".to_string(),
            used: 1.0,
            limit: 100.0,
            format: ProgressFormat::Percent,
            resets_at: Some("2099-01-01T00:00:00.000Z".to_string()),
            period_duration_ms: None,
            color: None,
        };

        let json: JsonValue = serde_json::to_value(&line).expect("serialize");
        let obj = json.as_object().expect("object");
        assert!(obj.get("resetsAt").is_some(), "expected resetsAt key");
        assert!(
            obj.get("resets_at").is_none(),
            "did not expect resets_at key"
        );
    }

    #[test]
    fn bar_chart_line_round_trips_from_builder() {
        let plugin = test_plugin(
            r#"
            globalThis.__pulseusage_plugin = {
                probe(ctx) {
                    return {
                        lines: [
                            ctx.line.barChart({
                                label: "Usage Trend",
                                points: [{ label: "Today", value: 42, valueLabel: "42 tokens" }],
                                note: "Estimated from local logs"
                            })
                        ]
                    };
                }
            };
            "#,
        );

        let output = run_probe(&plugin, &temp_app_dir("bar-chart"), "0.0.0");
        let json: JsonValue = serde_json::to_value(&output.lines[0]).expect("serialize");
        assert_eq!(json["type"], "barChart");
        assert_eq!(json["label"], "Usage Trend");
        assert_eq!(json["points"][0]["valueLabel"], "42 tokens");
        assert_eq!(json["note"], "Estimated from local logs");
    }

    #[test]
    fn bar_chart_caps_excessive_points() {
        // A plugin-controlled points array must not parse unbounded: this path
        // is native and runs after the JS deadline interrupt can fire.
        let plugin = test_plugin(
            r#"
            globalThis.__pulseusage_plugin = {
                probe(ctx) {
                    var points = [];
                    for (var i = 0; i < 5000; i++) {
                        points.push({ label: "d" + i, value: i });
                    }
                    return { lines: [ctx.line.barChart({ label: "Big", points: points })] };
                }
            };
            "#,
        );

        let output = run_probe(&plugin, &temp_app_dir("bar-chart-cap"), "0.0.0");
        let json: JsonValue = serde_json::to_value(&output.lines[0]).expect("serialize");
        assert_eq!(json["type"], "barChart");
        assert_eq!(
            json["points"].as_array().expect("points array").len(),
            MAX_BAR_CHART_POINTS
        );
    }
}
