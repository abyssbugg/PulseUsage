use crate::plugin_engine::diagnostics::ProbeDiagnosticsRecorder;
use rquickjs::{Ctx, Function, Object};
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

pub(crate) const CCUSAGE_VERSION: &str = "20.0.2";
pub(crate) const CCUSAGE_PACKAGE_NAME: &str = "ccusage";
pub(crate) const CCUSAGE_BIN_NAME: &str = "ccusage";
pub(crate) const CCUSAGE_LEGACY_VERSION: &str = "18.0.11";
pub(crate) const CCUSAGE_LEGACY_CLAUDE_PACKAGE_NAME: &str = "ccusage";
pub(crate) const CCUSAGE_LEGACY_CODEX_PACKAGE_NAME: &str = "@ccusage/codex";
pub(crate) const CCUSAGE_LEGACY_CODEX_BIN_NAME: &str = "ccusage-codex";
pub(crate) const CCUSAGE_TIMEOUT_SECS: u64 = 15;
pub(crate) const CCUSAGE_POLL_INTERVAL_MS: u64 = 100;

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CcusageQueryOpts {
    pub(crate) provider: Option<String>,
    pub(crate) since: Option<String>,
    pub(crate) until: Option<String>,
    pub(crate) home_path: Option<String>,
    pub(crate) claude_path: Option<String>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum CcusageProvider {
    Claude,
    Codex,
}

pub(crate) static CCUSAGE_ACTIVE_PROVIDERS: OnceLock<Mutex<HashSet<CcusageProvider>>> = OnceLock::new();

pub(crate) struct CcusageQueryGuard {
    pub(crate) provider: CcusageProvider,
}

impl CcusageQueryGuard {
    pub(crate) fn acquire(provider: CcusageProvider) -> Option<Self> {
        let active = CCUSAGE_ACTIVE_PROVIDERS.get_or_init(|| Mutex::new(HashSet::new()));
        let mut active = active.lock().unwrap_or_else(|err| err.into_inner());
        if !active.insert(provider) {
            return None;
        }
        Some(Self { provider })
    }
}

impl Drop for CcusageQueryGuard {
    fn drop(&mut self) {
        let active = CCUSAGE_ACTIVE_PROVIDERS.get_or_init(|| Mutex::new(HashSet::new()));
        let mut active = active.lock().unwrap_or_else(|err| err.into_inner());
        active.remove(&self.provider);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum CcusageRunnerKind {
    Bunx,
    PnpmDlx,
    YarnDlx,
    NpmExec,
    Npx,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum CcusageCommandFlavor {
    Current,
    Legacy,
}

pub(crate) fn ccusage_runner_order() -> [CcusageRunnerKind; 5] {
    [
        CcusageRunnerKind::Bunx,
        CcusageRunnerKind::PnpmDlx,
        CcusageRunnerKind::YarnDlx,
        CcusageRunnerKind::NpmExec,
        CcusageRunnerKind::Npx,
    ]
}

pub(crate) fn ccusage_runner_label(kind: CcusageRunnerKind) -> &'static str {
    match kind {
        CcusageRunnerKind::Bunx => "bunx",
        CcusageRunnerKind::PnpmDlx => "pnpm dlx",
        CcusageRunnerKind::YarnDlx => "yarn dlx",
        CcusageRunnerKind::NpmExec => "npm exec",
        CcusageRunnerKind::Npx => "npx",
    }
}

#[derive(Copy, Clone)]
pub(crate) struct CcusageProviderConfig {
    command_namespace: &'static str,
    home_env_var: &'static str,
}

pub(crate) fn parse_ccusage_provider(value: &str) -> Option<CcusageProvider> {
    match value.trim().to_ascii_lowercase().as_str() {
        "claude" => Some(CcusageProvider::Claude),
        "codex" => Some(CcusageProvider::Codex),
        _ => None,
    }
}

pub(crate) fn infer_ccusage_provider(plugin_id: &str) -> Option<CcusageProvider> {
    parse_ccusage_provider(plugin_id)
}

pub(crate) fn resolve_ccusage_provider(opts: &CcusageQueryOpts, plugin_id: &str) -> CcusageProvider {
    opts.provider
        .as_deref()
        .and_then(parse_ccusage_provider)
        .or_else(|| infer_ccusage_provider(plugin_id))
        .unwrap_or(CcusageProvider::Claude)
}

pub(crate) fn ccusage_provider_config(provider: CcusageProvider) -> CcusageProviderConfig {
    match provider {
        CcusageProvider::Claude => CcusageProviderConfig {
            command_namespace: "claude",
            home_env_var: "CLAUDE_CONFIG_DIR",
        },
        CcusageProvider::Codex => CcusageProviderConfig {
            command_namespace: "codex",
            home_env_var: "CODEX_HOME",
        },
    }
}

pub(crate) fn ccusage_package_spec() -> String {
    format!("{}@{}", CCUSAGE_PACKAGE_NAME, CCUSAGE_VERSION)
}

pub(crate) fn ccusage_legacy_package_spec(provider: CcusageProvider) -> String {
    let package_name = match provider {
        CcusageProvider::Claude => CCUSAGE_LEGACY_CLAUDE_PACKAGE_NAME,
        CcusageProvider::Codex => CCUSAGE_LEGACY_CODEX_PACKAGE_NAME,
    };
    format!("{}@{}", package_name, CCUSAGE_LEGACY_VERSION)
}

pub(crate) fn ccusage_home_override<'a>(
    opts: &'a CcusageQueryOpts,
    provider: CcusageProvider,
) -> Option<&'a str> {
    if let Some(home_path) = opts
        .home_path
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return Some(home_path);
    }

    match provider {
        CcusageProvider::Claude => opts
            .claude_path
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty()),
        CcusageProvider::Codex => None,
    }
}

pub(crate) fn ccusage_runner_candidates(kind: CcusageRunnerKind) -> Vec<String> {
    let mut candidates: Vec<String> = Vec::new();
    match kind {
        CcusageRunnerKind::Bunx => {
            if let Some(home) = dirs::home_dir() {
                candidates.push(home.join(".bun/bin/bunx").to_string_lossy().to_string());
            }
            candidates.extend(
                ["/opt/homebrew/bin/bunx", "/usr/local/bin/bunx", "bunx"]
                    .into_iter()
                    .map(str::to_string),
            );
        }
        CcusageRunnerKind::PnpmDlx => {
            candidates.extend(
                ["/opt/homebrew/bin/pnpm", "/usr/local/bin/pnpm", "pnpm"]
                    .into_iter()
                    .map(str::to_string),
            );
        }
        CcusageRunnerKind::YarnDlx => {
            candidates.extend(
                ["/opt/homebrew/bin/yarn", "/usr/local/bin/yarn", "yarn"]
                    .into_iter()
                    .map(str::to_string),
            );
        }
        CcusageRunnerKind::NpmExec => {
            candidates.extend(
                ["/opt/homebrew/bin/npm", "/usr/local/bin/npm", "npm"]
                    .into_iter()
                    .map(str::to_string),
            );
        }
        CcusageRunnerKind::Npx => {
            candidates.extend(
                ["/opt/homebrew/bin/npx", "/usr/local/bin/npx", "npx"]
                    .into_iter()
                    .map(str::to_string),
            );
        }
    }

    let mut unique = Vec::new();
    for candidate in candidates {
        if candidate.is_empty() || unique.iter().any(|c| c == &candidate) {
            continue;
        }
        unique.push(candidate);
    }
    unique
}

pub(crate) fn nvm_default_bin_path(home: &Path) -> Option<PathBuf> {
    let alias_path = home.join(".nvm/alias/default");
    let version = std::fs::read_to_string(&alias_path).ok()?;
    let version = version.trim();
    if version.is_empty() {
        return None;
    }
    let version = if version.starts_with('v') {
        version.to_string()
    } else {
        format!("v{version}")
    };
    Some(home.join(".nvm/versions/node").join(version).join("bin"))
}

pub(crate) fn ccusage_path_entries_with(home: Option<&Path>, existing_path: Option<&OsStr>) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = Vec::new();

    if let Some(home) = home {
        entries.push(home.join(".bun/bin"));
        entries.push(home.join(".nvm/current/bin"));
        if let Some(nvm_bin) = nvm_default_bin_path(home) {
            entries.push(nvm_bin);
        }
        entries.push(home.join(".local/bin"));
    }

    entries.extend(
        ["/opt/homebrew/bin", "/usr/local/bin"]
            .into_iter()
            .map(PathBuf::from),
    );

    if let Some(existing_path) = existing_path {
        for path in std::env::split_paths(existing_path) {
            entries.push(path);
        }
    }

    let mut unique_entries = Vec::new();
    for entry in entries {
        if entry.as_os_str().is_empty() || unique_entries.iter().any(|path| path == &entry) {
            continue;
        }
        unique_entries.push(entry);
    }
    unique_entries
}

pub(crate) fn ccusage_enriched_path_with(
    home: Option<&Path>,
    existing_path: Option<&OsStr>,
) -> Option<OsString> {
    let entries = ccusage_path_entries_with(home, existing_path);
    if entries.is_empty() {
        return None;
    }
    std::env::join_paths(entries).ok()
}

pub(crate) fn ccusage_enriched_path() -> Option<OsString> {
    let home = dirs::home_dir();
    let existing_path = std::env::var_os("PATH");
    ccusage_enriched_path_with(home.as_deref(), existing_path.as_deref())
}

pub(crate) fn ccusage_runner_available(candidate: &str, enriched_path: Option<&OsStr>) -> bool {
    let mut command = std::process::Command::new(candidate);
    command.arg("--version");
    if let Some(path) = enriched_path {
        command.env("PATH", path);
    }
    command
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());

    command.status().map(|s| s.success()).unwrap_or(false)
}

pub(crate) fn configure_ccusage_command(
    command: &mut std::process::Command,
    args: &[String],
    enriched_path: Option<&OsStr>,
) {
    command.args(args);
    if let Some(path) = enriched_path {
        command.env("PATH", path);
    }
    command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
}

pub(crate) fn resolve_ccusage_runner_binary(kind: CcusageRunnerKind) -> Option<String> {
    let path = ccusage_enriched_path();
    for candidate in ccusage_runner_candidates(kind) {
        if ccusage_runner_available(&candidate, path.as_deref()) {
            return Some(candidate);
        }
    }
    None
}

pub(crate) fn collect_ccusage_runners_with<F>(mut resolver: F) -> Vec<(CcusageRunnerKind, String)>
where
    F: FnMut(CcusageRunnerKind) -> Option<String>,
{
    let mut runners = Vec::new();
    for kind in ccusage_runner_order() {
        if let Some(program) = resolver(kind) {
            runners.push((kind, program));
        }
    }
    runners
}

pub(crate) fn collect_ccusage_runners() -> Vec<(CcusageRunnerKind, String)> {
    collect_ccusage_runners_with(resolve_ccusage_runner_binary)
}

pub(crate) fn append_ccusage_common_args(
    args: &mut Vec<String>,
    opts: &CcusageQueryOpts,
    provider: CcusageProvider,
    flavor: CcusageCommandFlavor,
) {
    let config = ccusage_provider_config(provider);
    if flavor == CcusageCommandFlavor::Current {
        args.push(config.command_namespace.to_string());
    }
    args.extend([
        "daily".to_string(),
        "--json".to_string(),
        "--order".to_string(),
        "desc".to_string(),
    ]);

    if let Some(since) = opts
        .since
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        args.push("--since".to_string());
        args.push(since.to_string());
    }

    if let Some(until) = opts
        .until
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        args.push("--until".to_string());
        args.push(until.to_string());
    }
}

pub(crate) fn ccusage_runner_args(
    kind: CcusageRunnerKind,
    opts: &CcusageQueryOpts,
    provider: CcusageProvider,
    flavor: CcusageCommandFlavor,
) -> Vec<String> {
    let package_spec = match flavor {
        CcusageCommandFlavor::Current => ccusage_package_spec(),
        CcusageCommandFlavor::Legacy => ccusage_legacy_package_spec(provider),
    };
    let npm_exec_bin = match (flavor, provider) {
        (CcusageCommandFlavor::Current, _) => CCUSAGE_BIN_NAME,
        (CcusageCommandFlavor::Legacy, CcusageProvider::Claude) => CCUSAGE_BIN_NAME,
        (CcusageCommandFlavor::Legacy, CcusageProvider::Codex) => CCUSAGE_LEGACY_CODEX_BIN_NAME,
    };
    let mut args: Vec<String> = match kind {
        CcusageRunnerKind::Bunx => vec!["--silent".to_string(), package_spec.clone()],
        CcusageRunnerKind::PnpmDlx => {
            vec!["-s".to_string(), "dlx".to_string(), package_spec.clone()]
        }
        CcusageRunnerKind::YarnDlx => {
            vec!["dlx".to_string(), "-q".to_string(), package_spec.clone()]
        }
        CcusageRunnerKind::NpmExec => vec![
            "exec".to_string(),
            "--yes".to_string(),
            format!("--package={package_spec}"),
            "--".to_string(),
            npm_exec_bin.to_string(),
        ],
        CcusageRunnerKind::Npx => vec!["--yes".to_string(), package_spec],
    };

    append_ccusage_common_args(&mut args, opts, provider, flavor);
    args
}

pub(crate) fn extract_last_json_value(stdout: &str) -> Option<String> {
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return None;
    }

    if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
        return Some(trimmed.to_string());
    }

    let mut starts: Vec<usize> = trimmed
        .char_indices()
        .filter(|(_, c)| *c == '{' || *c == '[')
        .map(|(idx, _)| idx)
        .collect();
    starts.reverse();

    for start in starts {
        let candidate = trimmed[start..].trim();
        if serde_json::from_str::<serde_json::Value>(candidate).is_ok() {
            return Some(candidate.to_string());
        }
    }

    None
}

pub(crate) fn normalize_ccusage_output(stdout: &str) -> Option<String> {
    let json_value = extract_last_json_value(stdout)?;
    let parsed: serde_json::Value = serde_json::from_str(&json_value).ok()?;

    let normalized = match parsed {
        serde_json::Value::Array(daily) => serde_json::json!({ "daily": daily }),
        serde_json::Value::Object(map) => {
            let daily = map.get("daily")?;
            if !daily.is_array() {
                return None;
            }
            serde_json::Value::Object(map)
        }
        _ => return None,
    };

    serde_json::to_string(&normalized).ok()
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CcusageRunnerResult {
    Success(String),
    Failed,
    TimedOut,
}

#[cfg(unix)]
pub(crate) fn kill_ccusage_process_group(child_id: u32) -> std::io::Result<()> {
    let pgid = i32::try_from(child_id)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid child pid"))?;
    let rc = unsafe { libc::kill(-pgid, libc::SIGKILL) };
    if rc == 0 {
        return Ok(());
    }

    let err = std::io::Error::last_os_error();
    if err.raw_os_error() == Some(libc::ESRCH) {
        return Ok(());
    }
    Err(err)
}

pub(crate) fn kill_ccusage_on_timeout(child: &mut std::process::Child) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        kill_ccusage_process_group(child.id())
    }

    #[cfg(not(unix))]
    {
        child.kill()
    }
}

pub(crate) fn format_ccusage_timeout(timeout: std::time::Duration) -> String {
    if timeout.subsec_millis() == 0 {
        return format!("{}s", timeout.as_secs());
    }
    if timeout.as_secs() == 0 {
        return format!("{}ms", timeout.as_millis());
    }
    format!("{:.3}s", timeout.as_secs_f64())
}

#[cfg(test)]
pub(crate) fn run_ccusage_with_runner(
    kind: CcusageRunnerKind,
    program: &str,
    opts: &CcusageQueryOpts,
    provider: CcusageProvider,
    plugin_id: &str,
) -> CcusageRunnerResult {
    run_ccusage_with_runner_deadline(
        kind,
        program,
        opts,
        provider,
        plugin_id,
        crate::plugin_engine::shared::ProbeDeadline::none(),
    )
}

pub(crate) fn run_ccusage_with_runner_deadline(
    kind: CcusageRunnerKind,
    program: &str,
    opts: &CcusageQueryOpts,
    provider: CcusageProvider,
    plugin_id: &str,
    deadline: crate::plugin_engine::shared::ProbeDeadline,
) -> CcusageRunnerResult {
    if deadline.has_elapsed() {
        log::warn!("[plugin:{}] ccusage skipped: probe timed out", plugin_id);
        return CcusageRunnerResult::TimedOut;
    }

    let Some(current_timeout) = deadline.clamp_duration(Duration::from_secs(CCUSAGE_TIMEOUT_SECS))
    else {
        crate::plugin_engine::shared::log_probe_deadline_skip(plugin_id, "ccusage");
        return CcusageRunnerResult::TimedOut;
    };

    let current = run_ccusage_with_runner_timeout(
        kind,
        program,
        opts,
        provider,
        plugin_id,
        CcusageCommandFlavor::Current,
        current_timeout,
    );
    match current {
        CcusageRunnerResult::Failed if deadline.has_elapsed() => CcusageRunnerResult::TimedOut,
        CcusageRunnerResult::Failed => {
            let Some(legacy_timeout) =
                deadline.clamp_duration(Duration::from_secs(CCUSAGE_TIMEOUT_SECS))
            else {
                crate::plugin_engine::shared::log_probe_deadline_skip(
                    plugin_id,
                    "ccusage legacy fallback",
                );
                return CcusageRunnerResult::TimedOut;
            };
            run_ccusage_with_runner_timeout(
                kind,
                program,
                opts,
                provider,
                plugin_id,
                CcusageCommandFlavor::Legacy,
                legacy_timeout,
            )
        }
        other => other,
    }
}

pub(crate) fn run_ccusage_with_runner_timeout(
    kind: CcusageRunnerKind,
    program: &str,
    opts: &CcusageQueryOpts,
    provider: CcusageProvider,
    plugin_id: &str,
    flavor: CcusageCommandFlavor,
    timeout: std::time::Duration,
) -> CcusageRunnerResult {
    let args = ccusage_runner_args(kind, opts, provider, flavor);
    let enriched_path = ccusage_enriched_path();
    let mut command = std::process::Command::new(program);
    configure_ccusage_command(&mut command, &args, enriched_path.as_deref());

    if let Some(home_path) = ccusage_home_override(opts, provider) {
        let config = ccusage_provider_config(provider);
        command.env(
            config.home_env_var,
            crate::plugin_engine::shared::expand_path(&home_path),
        );
    }

    let redacted_program = crate::plugin_engine::redaction::redact_log_message(program);

    log::info!(
        "[plugin:{}] ccusage query via {} {:?} ({})",
        plugin_id,
        ccusage_runner_label(kind),
        flavor,
        redacted_program
    );

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => {
            log::warn!(
                "[plugin:{}] ccusage spawn failed for {}: {}",
                plugin_id,
                ccusage_runner_label(kind),
                e
            );
            return CcusageRunnerResult::Failed;
        }
    };

    // Drain pipes concurrently while the process is running so the child cannot block on full
    // stdout/stderr buffers before exit.
    let mut stdout_reader = child.stdout.take().map(|mut stdout| {
        std::thread::spawn(move || {
            let mut v = Vec::new();
            let _ = std::io::Read::read_to_end(&mut stdout, &mut v);
            v
        })
    });
    let mut stderr_reader = child.stderr.take().map(|mut stderr| {
        std::thread::spawn(move || {
            let mut v = Vec::new();
            let _ = std::io::Read::read_to_end(&mut stderr, &mut v);
            v
        })
    });

    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = stdout_reader
                    .take()
                    .and_then(|reader| reader.join().ok())
                    .unwrap_or_default();
                let stderr = stderr_reader
                    .take()
                    .and_then(|reader| reader.join().ok())
                    .unwrap_or_default();

                if status.success() {
                    let out = String::from_utf8_lossy(&stdout);
                    if let Some(normalized_json) = normalize_ccusage_output(&out) {
                        return CcusageRunnerResult::Success(normalized_json);
                    }
                    log::warn!(
                        "[plugin:{}] ccusage output parse failed for {}",
                        plugin_id,
                        ccusage_runner_label(kind)
                    );
                    return CcusageRunnerResult::Failed;
                }

                let err = String::from_utf8_lossy(&stderr);
                log::warn!(
                    "[plugin:{}] ccusage failed for {}: {}",
                    plugin_id,
                    ccusage_runner_label(kind),
                    err.trim()
                );
                return CcusageRunnerResult::Failed;
            }
            Ok(None) => {
                if start.elapsed() > timeout {
                    if let Err(e) = kill_ccusage_on_timeout(&mut child) {
                        log::warn!(
                            "[plugin:{}] ccusage process group kill failed for {}: {}",
                            plugin_id,
                            ccusage_runner_label(kind),
                            e
                        );
                        let _ = child.kill();
                    }
                    let _ = child.wait();
                    let _ = stdout_reader.take().and_then(|reader| reader.join().ok());
                    let _ = stderr_reader.take().and_then(|reader| reader.join().ok());
                    log::warn!(
                        "[plugin:{}] ccusage timed out after {} for {}",
                        plugin_id,
                        format_ccusage_timeout(timeout),
                        ccusage_runner_label(kind)
                    );
                    return CcusageRunnerResult::TimedOut;
                }
                std::thread::sleep(std::time::Duration::from_millis(CCUSAGE_POLL_INTERVAL_MS));
            }
            Err(e) => {
                log::warn!(
                    "[plugin:{}] ccusage wait failed for {}: {}",
                    plugin_id,
                    ccusage_runner_label(kind),
                    e
                );
                return CcusageRunnerResult::Failed;
            }
        }
    }
}

pub(crate) fn run_ccusage_query_with_runners<F>(
    runners: Vec<(CcusageRunnerKind, String)>,
    opts: &CcusageQueryOpts,
    provider: CcusageProvider,
    plugin_id: &str,
    mut run: F,
) -> String
where
    F: FnMut(
        CcusageRunnerKind,
        &str,
        &CcusageQueryOpts,
        CcusageProvider,
        &str,
    ) -> CcusageRunnerResult,
{
    if runners.is_empty() {
        log::warn!(
            "[plugin:{}] no package runner found for ccusage query",
            plugin_id
        );
        return serde_json::json!({ "status": "no_runner" }).to_string();
    }

    for (kind, program) in runners {
        match run(kind, &program, opts, provider, plugin_id) {
            CcusageRunnerResult::Success(result) => {
                let data: serde_json::Value = match serde_json::from_str(&result) {
                    Ok(v) => v,
                    Err(e) => {
                        log::warn!(
                            "[plugin:{}] ccusage normalized payload parse failed: {}",
                            plugin_id,
                            e
                        );
                        continue;
                    }
                };
                return serde_json::json!({ "status": "ok", "data": data }).to_string();
            }
            CcusageRunnerResult::Failed => {}
            CcusageRunnerResult::TimedOut => {
                log::warn!(
                    "[plugin:{}] ccusage query timed out; skipping fallback runners",
                    plugin_id
                );
                return serde_json::json!({ "status": "runner_failed" }).to_string();
            }
        }
    }

    log::warn!(
        "[plugin:{}] ccusage query failed with all available runners",
        plugin_id
    );
    serde_json::json!({ "status": "runner_failed" }).to_string()
}

pub(crate) fn inject_ccusage<'js>(
    ctx: &Ctx<'js>,
    host: &Object<'js>,
    plugin_id: &str,
    deadline: crate::plugin_engine::shared::ProbeDeadline,
    diagnostics_recorder: ProbeDiagnosticsRecorder,
) -> rquickjs::Result<()> {
    let ccusage_obj = Object::new(ctx.clone())?;
    let pid = plugin_id.to_string();

    ccusage_obj.set(
        "_queryRaw",
        Function::new(
            ctx.clone(),
            move |_ctx_inner: Ctx<'_>, opts_json: String| -> rquickjs::Result<String> {
                let opts: CcusageQueryOpts = match serde_json::from_str(&opts_json) {
                    Ok(v) => v,
                    Err(e) => {
                        log::warn!("[plugin:{}] invalid ccusage opts JSON: {}", pid, e);
                        CcusageQueryOpts::default()
                    }
                };
                let provider = resolve_ccusage_provider(&opts, &pid);
                let Some(_active_query) = CcusageQueryGuard::acquire(provider) else {
                    diagnostics_recorder.record_local_read(false);
                    log::warn!("[plugin:{}] ccusage query already running", pid);
                    return Ok(serde_json::json!({ "status": "runner_failed" }).to_string());
                };
                let runners = collect_ccusage_runners();
                let result = run_ccusage_query_with_runners(
                    runners,
                    &opts,
                    provider,
                    &pid,
                    |kind, program, opts, provider, plugin_id| {
                        run_ccusage_with_runner_deadline(
                            kind, program, opts, provider, plugin_id, deadline,
                        )
                    },
                );
                let succeeded = serde_json::from_str::<serde_json::Value>(&result)
                    .ok()
                    .and_then(|value| {
                        value
                            .get("status")
                            .and_then(|status| status.as_str())
                            .map(str::to_string)
                    })
                    .as_deref()
                    == Some("ok");
                diagnostics_recorder.record_local_read(succeeded);
                Ok(result)
            },
        )?,
    )?;

    host.set("ccusage", ccusage_obj)?;
    Ok(())
}

pub fn patch_ccusage_wrapper(ctx: &rquickjs::Ctx<'_>) -> rquickjs::Result<()> {
    ctx.eval::<(), _>(
        r#"
        (function() {
            var rawFn = __pulseusage_ctx.host.ccusage._queryRaw;
            __pulseusage_ctx.host.ccusage.query = function(opts) {
                var result = rawFn(JSON.stringify(opts || {}));
                try {
                    var parsed = JSON.parse(result);
                    if (parsed && typeof parsed === "object" && typeof parsed.status === "string") {
                        return parsed;
                    }
                } catch (e) {}
                return { status: "runner_failed" };
            };
        })();
        "#
        .as_bytes(),
    )
}
