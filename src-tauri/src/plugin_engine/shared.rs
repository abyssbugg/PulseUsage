use rquickjs::{Ctx, Exception};
use std::time::{Duration, Instant};

const MIN_BLOCKING_TIMEOUT: Duration = Duration::from_millis(1);

pub(crate) fn probe_timeout_error<'js>(ctx: &Ctx<'js>) -> rquickjs::Error {
    Exception::throw_message(ctx, "probe timed out")
}

pub(crate) fn iso_now() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|err| {
            log::error!("nowIso format failed: {}", err);
            "1970-01-01T00:00:00Z".to_string()
        })
}

pub(crate) fn expand_path(path: &str) -> String {
    if path == "~" {
        if let Some(home) = dirs::home_dir() {
            return home.to_string_lossy().to_string();
        }
    }
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]).to_string_lossy().to_string();
        }
    }
    path.to_string()
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ProbeDeadline {
    expires_at: Option<Instant>,
}

impl ProbeDeadline {
    #[cfg(test)]
    pub(crate) fn none() -> Self {
        Self { expires_at: None }
    }

    pub(crate) fn at(expires_at: Instant) -> Self {
        Self {
            expires_at: Some(expires_at),
        }
    }

    pub(crate) fn has_elapsed(self) -> bool {
        self.expires_at
            .map(|expires_at| Instant::now() >= expires_at)
            .unwrap_or(false)
    }

    pub(crate) fn clamp_duration(self, requested: Duration) -> Option<Duration> {
        let Some(expires_at) = self.expires_at else {
            return Some(requested);
        };
        let remaining = expires_at
            .checked_duration_since(Instant::now())
            .filter(|remaining| *remaining >= MIN_BLOCKING_TIMEOUT)?;
        Some(requested.min(remaining))
    }
}

pub(crate) fn log_probe_deadline_skip(plugin_id: &str, operation: &str) {
    log::warn!(
        "[plugin:{}] {} skipped: probe timed out",
        plugin_id,
        operation
    );
}
