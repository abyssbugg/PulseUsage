//! Diagnostics export.
//!
//! Copies the application log files and writes a redacted metadata snapshot
//! into a timestamped folder under the user's Downloads directory. The caller
//! then reveals that folder in Finder via `tauri-plugin-opener`.
//!
//! Redaction boundary (deliberately narrow):
//! - Included: app name, app version, platform, export timestamp, log file
//!   names, and plugin manifest fields (id / name / version) plus a redacted
//!   plugin directory path.
//! - Excluded: the settings store, credentials, auth tokens, cookies,
//!   request/response bodies, and any plugin runtime output. We never read
//!   `settings.json`; plugin manifests are static descriptor files shipped
//!   with the plugin and contain no user secrets.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Manager};
use time::format_description::well_known::Rfc3339;

use crate::log_path;
use crate::plugin_engine::redaction::redact_log_message;

const DIAGNOSTICS_SUBDIR: &str = "diagnostics";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticsMetadata {
    app_name: String,
    app_version: String,
    platform: String,
    exported_at: String,
    export_dir: String,
    log_files: Vec<String>,
    plugins: Vec<PluginDiagnosticEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PluginDiagnosticEntry {
    id: String,
    name: String,
    version: String,
    /// Redacted via `redact_log_message` to avoid leaking the user's home path.
    plugin_dir: String,
}

/// Export diagnostics to `<Downloads>/<app>-diagnostics-<timestamp>/`.
///
/// Returns the created directory path on success. The directory contains a
/// `logs/` copy of the current log files and a `metadata.json` snapshot.
pub fn export(app: &AppHandle) -> Result<PathBuf, String> {
    let app_name = app.package_info().name.clone();
    let app_version = app.package_info().version.to_string();
    let log_file = log_path::for_app(app)?;
    let log_dir = log_file
        .parent()
        .ok_or_else(|| "log path has no parent directory".to_string())?;

    let timestamp = time::OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|e| format!("failed to format timestamp: {}", e))?
        .replace(':', "-");

    let dest = export_dir(&app_name, &timestamp);
    let logs_dest = dest.join("logs");
    fs::create_dir_all(&logs_dest)
        .map_err(|e| format!("failed to create diagnostics logs dir: {}", e))?;

    let log_files = copy_log_artifacts(log_dir, &logs_dest)?;
    let plugins = collect_plugins(app);

    let metadata = DiagnosticsMetadata {
        app_name,
        app_version,
        platform: platform_string(),
        exported_at: timestamp,
        export_dir: redact_log_message(&dest.display().to_string()),
        log_files,
        plugins,
    };
    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("failed to serialize metadata: {}", e))?;
    fs::write(dest.join("metadata.json"), json)
        .map_err(|e| format!("failed to write metadata.json: {}", e))?;

    log::info!("diagnostics exported to {}", dest.display());
    Ok(dest)
}

fn export_dir(app_name: &str, timestamp: &str) -> PathBuf {
    let base = dirs::download_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."));
    base.join(format!("{}-{}-{}", app_name, DIAGNOSTICS_SUBDIR, timestamp))
}

/// Copy every regular file from `log_dir` into `dest`, returning the copied
/// file names (sorted). Subdirectories and unreadable entries are skipped
/// with a warning rather than failing the whole export.
fn copy_log_artifacts(log_dir: &Path, dest: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    let entries = match fs::read_dir(log_dir) {
        Ok(e) => e,
        Err(e) => {
            log::warn!("cannot read log dir {}: {}", log_dir.display(), e);
            return Ok(names);
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                log::warn!("skipping unreadable log entry: {}", e);
                continue;
            }
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name() else {
            continue;
        };
        let dest_path = dest.join(name);
        if let Err(e) = fs::copy(&path, &dest_path) {
            log::warn!("skipped log file {}: {}", path.display(), e);
            continue;
        }
        names.push(name.to_string_lossy().to_string());
    }
    names.sort();
    Ok(names)
}

fn collect_plugins(app: &AppHandle) -> Vec<PluginDiagnosticEntry> {
    let state = app.state::<Mutex<crate::AppState>>();
    let guard = match state.lock() {
        Ok(g) => g,
        Err(e) => {
            log::error!("plugin state lock poisoned: {}", e);
            return Vec::new();
        }
    };
    guard
        .plugins
        .iter()
        .map(|p| PluginDiagnosticEntry {
            id: p.manifest.id.clone(),
            name: p.manifest.name.clone(),
            version: p.manifest.version.clone(),
            plugin_dir: redact_log_message(&p.plugin_dir.display().to_string()),
        })
        .collect()
}

fn platform_string() -> String {
    format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn export_dir_names_folder_with_app_and_timestamp() {
        let dir = export_dir("PulseUsage", "2026-07-02T12-00-00Z");
        let name = dir.file_name().unwrap().to_string_lossy().to_string();
        assert_eq!(name, "PulseUsage-diagnostics-2026-07-02T12-00-00Z");
    }

    #[test]
    fn platform_string_contains_os_and_arch() {
        let p = platform_string();
        assert!(!p.is_empty(), "platform string must not be empty");
        assert!(
            p.contains(std::env::consts::OS),
            "platform string should contain OS constant"
        );
        assert!(
            p.contains(std::env::consts::ARCH),
            "platform string should contain ARCH constant"
        );
    }

    #[test]
    fn copy_log_artifacts_copies_files_and_skips_subdirs() {
        let root =
            std::env::temp_dir().join(format!("pulseusage-diag-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let src = root.join("src");
        let dest = root.join("dest");
        fs::create_dir_all(&src).unwrap();
        fs::create_dir_all(&dest).unwrap();
        fs::write(src.join("a.log"), "aaa").unwrap();
        fs::write(src.join("b.log"), "bbb").unwrap();
        fs::create_dir_all(src.join("nested")).unwrap();

        let mut names = copy_log_artifacts(&src, &dest).unwrap();
        names.sort();

        assert_eq!(names, vec!["a.log".to_string(), "b.log".to_string()]);
        assert!(dest.join("a.log").is_file());
        assert!(dest.join("b.log").is_file());
        assert!(!dest.join("nested").exists());

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn copy_log_artifacts_returns_empty_when_dir_missing() {
        let missing = PathBuf::from("/this/path/should/not/exist/pulseusage-test");
        let names = copy_log_artifacts(&missing, &missing).unwrap();
        assert!(names.is_empty());
    }
}
