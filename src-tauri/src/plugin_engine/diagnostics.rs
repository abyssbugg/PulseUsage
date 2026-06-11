use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MetricClassification {
    Required,
    Optional,
    PlanDependent,
    Deprecated,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AuthDetected {
    Detected,
    NotDetected,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DataSourceReachability {
    Reachable,
    Unreachable,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ParserExecutionStatus {
    NotRun,
    Success,
    Failed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DiagnosticsHealth {
    Ok,
    Warning,
    Error,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum DiagnosticsLikelyCause {
    AuthMissing,
    AuthRejected,
    DataSourceUnreachable,
    ParserError,
    ManifestMismatch,
    NoMetricsReturned,
    Unknown,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SafeHostFacts {
    pub http_requests_attempted: u32,
    pub http_2xx_responses_seen: u32,
    pub auth_status_responses_seen: u32,
    pub local_reads_attempted: u32,
    pub local_read_failures: u32,
    pub auth_read_attempts: u32,
    pub auth_read_successes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ManifestMetricDiagnostic {
    pub label: String,
    #[serde(rename = "type")]
    pub line_type: String,
    pub scope: String,
    pub classification: MetricClassification,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReturnedMetricDiagnostic {
    pub label: String,
    #[serde(rename = "type")]
    pub line_type: String,
}

pub type MissingMetricDiagnostic = ManifestMetricDiagnostic;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDiagnostics {
    pub provider_loaded: bool,
    pub provider_version: Option<String>,
    pub auth_detected: AuthDetected,
    pub data_source_reachable: DataSourceReachability,
    pub last_successful_refresh_at: Option<u64>,
    pub manifest_metrics: Vec<ManifestMetricDiagnostic>,
    pub returned_metrics: Vec<ReturnedMetricDiagnostic>,
    pub missing_metrics: Vec<MissingMetricDiagnostic>,
    pub last_error: Option<String>,
    pub parser_execution_status: ParserExecutionStatus,
    pub health_summary: DiagnosticsHealth,
    pub likely_causes: Vec<DiagnosticsLikelyCause>,
    pub host_facts: SafeHostFacts,
}

impl Default for ProviderDiagnostics {
    fn default() -> Self {
        Self {
            provider_loaded: false,
            provider_version: None,
            auth_detected: AuthDetected::Unknown,
            data_source_reachable: DataSourceReachability::Unknown,
            last_successful_refresh_at: None,
            manifest_metrics: Vec::new(),
            returned_metrics: Vec::new(),
            missing_metrics: Vec::new(),
            last_error: None,
            parser_execution_status: ParserExecutionStatus::NotRun,
            health_summary: DiagnosticsHealth::Unknown,
            likely_causes: Vec::new(),
            host_facts: SafeHostFacts::default(),
        }
    }
}

#[derive(Clone, Default)]
pub(crate) struct ProbeDiagnosticsRecorder {
    facts: Arc<Mutex<SafeHostFacts>>,
}

impl ProbeDiagnosticsRecorder {
    pub(crate) fn snapshot(&self) -> SafeHostFacts {
        self.facts
            .lock()
            .map(|facts| facts.clone())
            .unwrap_or_default()
    }

    pub(crate) fn record_http_attempt(&self) {
        self.update(|facts| {
            facts.http_requests_attempted = facts.http_requests_attempted.saturating_add(1);
        });
    }

    pub(crate) fn record_http_status(&self, status: u16) {
        self.update(|facts| {
            if (200..300).contains(&status) {
                facts.http_2xx_responses_seen = facts.http_2xx_responses_seen.saturating_add(1);
            }
            if status == 401 || status == 403 {
                facts.auth_status_responses_seen =
                    facts.auth_status_responses_seen.saturating_add(1);
            }
        });
    }

    pub(crate) fn record_local_read(&self, succeeded: bool) {
        self.update(|facts| {
            facts.local_reads_attempted = facts.local_reads_attempted.saturating_add(1);
            if !succeeded {
                facts.local_read_failures = facts.local_read_failures.saturating_add(1);
            }
        });
    }

    pub(crate) fn record_auth_read(&self, succeeded: bool) {
        self.update(|facts| {
            facts.auth_read_attempts = facts.auth_read_attempts.saturating_add(1);
            if succeeded {
                facts.auth_read_successes = facts.auth_read_successes.saturating_add(1);
            }
        });
    }

    fn update(&self, update: impl FnOnce(&mut SafeHostFacts)) {
        if let Ok(mut facts) = self.facts.lock() {
            update(&mut facts);
        }
    }
}

pub(crate) fn normalize_metric_classification(value: Option<&str>) -> MetricClassification {
    match value.map(str::trim).map(str::to_ascii_lowercase).as_deref() {
        Some("required") => MetricClassification::Required,
        Some("optional") => MetricClassification::Optional,
        Some("plandependent" | "plan-dependent" | "plan dependent" | "plan_dependent") => {
            MetricClassification::PlanDependent
        }
        Some("deprecated") => MetricClassification::Deprecated,
        _ => MetricClassification::Unknown,
    }
}

pub(crate) fn normalized_classification_string(value: Option<&str>) -> Option<String> {
    match normalize_metric_classification(value) {
        MetricClassification::Required => Some("required".to_string()),
        MetricClassification::Optional => Some("optional".to_string()),
        MetricClassification::PlanDependent => Some("planDependent".to_string()),
        MetricClassification::Deprecated => Some("deprecated".to_string()),
        MetricClassification::Unknown => None,
    }
}

pub(crate) fn redact_diagnostic_text(text: &str) -> String {
    let mut result = text.to_string();

    if let Ok(email_re) = regex_lite::Regex::new(r"(?i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b") {
        result = email_re.replace_all(&result, "[REDACTED]").to_string();
    }
    if let Ok(api_re) =
        regex_lite::Regex::new(r"\b(?:sk-|pk-|api_|key_|secret_)[A-Za-z0-9_-]{8,}\b")
    {
        result = api_re.replace_all(&result, "[REDACTED]").to_string();
    }
    if let Ok(jwt_re) =
        regex_lite::Regex::new(r"\beyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\b")
    {
        result = jwt_re.replace_all(&result, "[REDACTED]").to_string();
    }
    if let Ok(bearer_re) = regex_lite::Regex::new(r"(?i)\bBearer\s+[A-Za-z0-9._-]+\b") {
        result = bearer_re
            .replace_all(&result, "Bearer [REDACTED]")
            .to_string();
    }
    if let Ok(id_re) = regex_lite::Regex::new(
        r"(?i)\b(?:user|account|org|organization|session)[_-]?id\s*[:=]\s*[A-Za-z0-9._-]+\b",
    ) {
        result = id_re.replace_all(&result, "[REDACTED_ID]").to_string();
    }
    if let Ok(url_re) = regex_lite::Regex::new(r#"https?://[^\s"',)]+"#) {
        result = url_re.replace_all(&result, "[URL]").to_string();
    }
    if let Ok(path_re) =
        regex_lite::Regex::new(r#"/(?:Users|home|private|var|tmp|Applications)/[^\s"',)]+"#)
    {
        result = path_re.replace_all(&result, "[PATH]").to_string();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_supported_metric_classifications() {
        assert_eq!(
            normalize_metric_classification(Some("required")),
            MetricClassification::Required
        );
        assert_eq!(
            normalize_metric_classification(Some("Optional")),
            MetricClassification::Optional
        );
        assert_eq!(
            normalize_metric_classification(Some("planDependent")),
            MetricClassification::PlanDependent
        );
        assert_eq!(
            normalize_metric_classification(Some("plan-dependent")),
            MetricClassification::PlanDependent
        );
        assert_eq!(
            normalize_metric_classification(Some("deprecated")),
            MetricClassification::Deprecated
        );
        assert_eq!(
            normalize_metric_classification(Some("unclassified")),
            MetricClassification::Unknown
        );
    }
}
