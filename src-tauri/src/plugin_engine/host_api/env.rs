use crate::plugin_engine::diagnostics::ProbeDiagnosticsRecorder;
use rquickjs::{Ctx, Function, Object};
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Mutex, OnceLock};

pub(crate) const WHITELISTED_ENV_VARS: [&str; 16] = [
    "CODEX_HOME",
    "CLAUDE_CONFIG_DIR",
    "CLAUDE_CODE_OAUTH_TOKEN",
    "USER_TYPE",
    "USE_STAGING_OAUTH",
    "USE_LOCAL_OAUTH",
    "CLAUDE_CODE_CUSTOM_OAUTH_URL",
    "CLAUDE_CODE_OAUTH_CLIENT_ID",
    "CLAUDE_LOCAL_OAUTH_API_BASE",
    "ZAI_API_KEY",
    "GLM_API_KEY",
    "MINIMAX_API_KEY",
    "MINIMAX_API_TOKEN",
    "MINIMAX_CN_API_KEY",
    "SYNTHETIC_API_KEY",
    "PI_CODING_AGENT_DIR",
];

pub(crate) fn is_credential_env_var(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    upper.contains("TOKEN")
        || upper.contains("API_KEY")
        || upper.ends_with("_KEY")
        || upper.contains("SECRET")
        || upper.contains("PASSWORD")
}

pub(crate) fn last_non_empty_trimmed_line(text: &str) -> Option<String> {
    text.lines()
        .map(|line| line.trim())
        .rev()
        .find(|line| !line.is_empty())
        .map(|line| line.to_string())
}

pub(crate) fn sanitize_env_value(text: &str) -> Option<String> {
    let mut cleaned = if let Ok(ansi_re) = regex_lite::Regex::new(r"\x1B\[[0-?]*[ -/]*[@-~]") {
        ansi_re.replace_all(text, "").to_string()
    } else {
        text.to_string()
    };
    cleaned.retain(|ch| ch == '\n' || ch == '\r' || ch == '\t' || !ch.is_control());
    last_non_empty_trimmed_line(&cleaned)
}

pub(crate) fn extract_marked_value(
    text: &str,
    start_marker: &str,
    end_marker: &str,
) -> Option<String> {
    let start = text.find(start_marker)?;
    let after_start = &text[start + start_marker.len()..];
    let end = after_start.find(end_marker)?;
    sanitize_env_value(&after_start[..end])
}

pub(crate) fn parse_interactive_shell_env_output(
    text: &str,
    start_marker: &str,
    end_marker: &str,
) -> Option<String> {
    if let Some(marked) = extract_marked_value(text, start_marker, end_marker) {
        return Some(marked);
    }

    let has_complete_markers = text.contains(start_marker) && text.contains(end_marker);
    if has_complete_markers {
        return None;
    }

    sanitize_env_value(text)
}

pub(crate) fn read_env_from_process(name: &str) -> Option<String> {
    let value = std::env::var(name).ok()?;
    sanitize_env_value(&value)
}

fn read_command_stdout(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub(crate) fn read_env_value_via_command(program: &str, args: &[&str]) -> Option<String> {
    let stdout = read_command_stdout(program, args)?;
    sanitize_env_value(&stdout)
}

fn terminal_env_cache() -> &'static Mutex<HashMap<String, Option<String>>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Option<String>>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn shell_from_env() -> Option<String> {
    let shell = std::env::var("SHELL").ok()?;
    let trimmed = shell.trim();
    if trimmed.is_empty() {
        return None;
    }
    let file = std::path::Path::new(trimmed).file_name()?.to_string_lossy();
    let allowed = file == "zsh" || file == "bash" || file == "fish";
    if allowed {
        Some(trimmed.to_string())
    } else {
        None
    }
}

fn read_env_from_interactive_shell(program: &str, name: &str) -> Option<String> {
    const START_MARKER: &str = "__OPENUSAGE_ENV_START__";
    const END_MARKER: &str = "__OPENUSAGE_ENV_END__";

    let script = format!(
        "printf '{}\\n'; printenv {}; printf '{}\\n'",
        START_MARKER, name, END_MARKER
    );
    let output = read_command_stdout(program, &["-ilc", script.as_str()])?;
    parse_interactive_shell_env_output(&output, START_MARKER, END_MARKER)
}

fn read_env_from_interactive_shells(name: &str) -> Option<String> {
    let mut programs: Vec<String> = Vec::new();

    if let Some(shell) = shell_from_env() {
        programs.push(shell);
    }

    for program in [
        "/bin/zsh",
        "/bin/bash",
        "/opt/homebrew/bin/fish",
        "/usr/local/bin/fish",
        "/opt/local/bin/fish",
    ] {
        if !programs.iter().any(|p| p == program) {
            programs.push(program.to_string());
        }
    }

    for program in programs {
        if let Some(value) = read_env_from_interactive_shell(program.as_str(), name) {
            return Some(value);
        }
    }

    None
}

pub(crate) fn resolve_env_value(name: &str) -> Option<String> {
    // Prefer the current process env (fast + supports launchctl/terminal-launch).
    if let Some(value) = read_env_from_process(name) {
        return Some(value);
    }

    if let Ok(cache) = terminal_env_cache().lock() {
        if let Some(cached) = cache.get(name) {
            return cached.clone();
        }
    }

    let resolved = read_env_from_interactive_shells(name);
    if let Ok(mut cache) = terminal_env_cache().lock() {
        cache.insert(name.to_string(), resolved.clone());
    }
    resolved
}

pub(crate) fn inject_env<'js>(
    ctx: &Ctx<'js>,
    host: &Object<'js>,
    _plugin_id: &str,
    diagnostics_recorder: ProbeDiagnosticsRecorder,
) -> rquickjs::Result<()> {
    let env_obj = Object::new(ctx.clone())?;
    env_obj.set(
        "get",
        Function::new(ctx.clone(), move |name: String| -> Option<String> {
            if !WHITELISTED_ENV_VARS.contains(&name.as_str()) {
                return None;
            }
            let value = resolve_env_value(&name);
            if is_credential_env_var(&name) {
                diagnostics_recorder.record_auth_read(value.is_some());
            }
            value
        })?,
    )?;
    host.set("env", env_obj)?;
    Ok(())
}
