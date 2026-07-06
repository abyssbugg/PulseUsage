use crate::plugin_engine::diagnostics::ProbeDiagnosticsRecorder;
use rquickjs::{function::Opt, Ctx, Exception, Function, Object};
use std::ffi::OsString;

fn current_macos_keychain_account_from_user_env(user_env: Option<String>) -> String {
    user_env
        .and_then(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .or_else(|| crate::plugin_engine::host_api::env::read_env_value_via_command("id", &["-un"]))
        .unwrap_or_else(|| "pulseusage-user".to_string())
}

fn current_macos_keychain_account() -> String {
    current_macos_keychain_account_from_user_env(crate::plugin_engine::host_api::env::read_env_from_process("USER"))
}

fn keychain_find_generic_password_args(service: &str) -> Vec<OsString> {
    vec![
        OsString::from("find-generic-password"),
        OsString::from("-s"),
        OsString::from(service),
        OsString::from("-w"),
    ]
}

fn keychain_find_generic_password_args_for_account(service: &str, account: &str) -> Vec<OsString> {
    vec![
        OsString::from("find-generic-password"),
        OsString::from("-a"),
        OsString::from(account),
        OsString::from("-s"),
        OsString::from(service),
        OsString::from("-w"),
    ]
}

fn keychain_add_generic_password_args(service: &str, value: &str) -> Vec<OsString> {
    // macOS 27 (Build 26A5368g+) requires -a account for security add-generic-password.
    // Use the current macOS user as the default account, matching the read path's
    // fallback when no account is specified. This keeps service-only writes working.
    let account = current_macos_keychain_account();
    vec![
        OsString::from("add-generic-password"),
        OsString::from("-U"),
        OsString::from("-a"),
        OsString::from(account),
        OsString::from("-s"),
        OsString::from(service),
        OsString::from("-w"),
        OsString::from(value),
    ]
}

fn keychain_add_generic_password_args_for_account(
    service: &str,
    account: &str,
    value: &str,
) -> Vec<OsString> {
    vec![
        OsString::from("add-generic-password"),
        OsString::from("-U"),
        OsString::from("-a"),
        OsString::from(account),
        OsString::from("-s"),
        OsString::from(service),
        OsString::from("-w"),
        OsString::from(value),
    ]
}

/// Inject the `ctx.host.keychain` API.
///
/// Public JS API:
/// - `ctx.host.keychain.readGenericPassword(service, account?)` (account optional)
/// - `ctx.host.keychain.readGenericPasswordForCurrentUser(service)`
/// - `ctx.host.keychain.writeGenericPassword(service, value)`
/// - `ctx.host.keychain.writeGenericPasswordForCurrentUser(service, value)`
pub(crate) fn inject_keychain<'js>(
    ctx: &Ctx<'js>,
    host: &Object<'js>,
    plugin_id: &str,
    diagnostics_recorder: ProbeDiagnosticsRecorder,
) -> rquickjs::Result<()> {
    let keychain_obj = Object::new(ctx.clone())?;
    let pid_read = plugin_id.to_string();
    let read_recorder = diagnostics_recorder.clone();

    keychain_obj.set(
        "readGenericPassword",
        Function::new(
            ctx.clone(),
            move |ctx_inner: Ctx<'_>,
                  service: String,
                  account: Opt<String>|
                  -> rquickjs::Result<String> {
                if !cfg!(target_os = "macos") {
                    read_recorder.record_auth_read(false);
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        "keychain API is only supported on macOS",
                    ));
                }
                let account = account.0.and_then(|value| {
                    let trimmed = value.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.to_string())
                    }
                });
                let redacted_account = account
                    .as_ref()
                    .map(|value| crate::plugin_engine::redaction::redact_value(value));
                if let Some(ref redacted) = redacted_account {
                    log::info!(
                        "[plugin:{}] keychain read: service={}, account={}",
                        pid_read,
                        service,
                        redacted
                    );
                } else {
                    log::info!("[plugin:{}] keychain read: service={}", pid_read, service);
                }
                let args = if let Some(ref account) = account {
                    keychain_find_generic_password_args_for_account(&service, account)
                } else {
                    keychain_find_generic_password_args(&service)
                };
                let output = std::process::Command::new("security")
                    .args(args)
                    .output()
                    .map_err(|e| {
                        read_recorder.record_auth_read(false);
                        Exception::throw_message(
                            &ctx_inner,
                            &format!("keychain read failed: {}", e),
                        )
                    })?;

                if !output.status.success() {
                    read_recorder.record_auth_read(false);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let first_line = stderr.lines().next().unwrap_or("").trim();
                    if let Some(ref redacted) = redacted_account {
                        log::warn!(
                            "[plugin:{}] keychain read miss: service={}, account={}, error={}",
                            pid_read,
                            service,
                            redacted,
                            first_line
                        );
                    } else {
                        log::warn!(
                            "[plugin:{}] keychain read miss: service={}, error={}",
                            pid_read,
                            service,
                            first_line
                        );
                    }
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        &format!("keychain item not found: {}", first_line),
                    ));
                }

                if let Some(ref redacted) = redacted_account {
                    log::info!(
                        "[plugin:{}] keychain read hit: service={}, account={}",
                        pid_read,
                        service,
                        redacted
                    );
                } else {
                    log::info!(
                        "[plugin:{}] keychain read hit: service={}",
                        pid_read,
                        service
                    );
                }
                read_recorder.record_auth_read(true);
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            },
        )?,
    )?;

    let pid_read_current_user = plugin_id.to_string();
    let read_current_user_recorder = diagnostics_recorder;
    keychain_obj.set(
        "readGenericPasswordForCurrentUser",
        Function::new(
            ctx.clone(),
            move |ctx_inner: Ctx<'_>, service: String| -> rquickjs::Result<String> {
                if !cfg!(target_os = "macos") {
                    read_current_user_recorder.record_auth_read(false);
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        "keychain API is only supported on macOS",
                    ));
                }
                let account = current_macos_keychain_account();
                let args = keychain_find_generic_password_args_for_account(&service, &account);
                let redacted_account = crate::plugin_engine::redaction::redact_value(&account);
                log::info!(
                    "[plugin:{}] keychain read: service={}, account={}",
                    pid_read_current_user,
                    service,
                    redacted_account
                );
                let output = std::process::Command::new("security")
                    .args(&args)
                    .output()
                    .map_err(|e| {
                        read_current_user_recorder.record_auth_read(false);
                        Exception::throw_message(
                            &ctx_inner,
                            &format!("keychain read failed: {}", e),
                        )
                    })?;

                if !output.status.success() {
                    read_current_user_recorder.record_auth_read(false);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let first_line = stderr.lines().next().unwrap_or("").trim();
                    log::warn!(
                        "[plugin:{}] keychain read miss: service={}, account={}, error={}",
                        pid_read_current_user,
                        service,
                        redacted_account,
                        first_line
                    );
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        &format!("keychain item not found: {}", first_line),
                    ));
                }

                log::info!(
                    "[plugin:{}] keychain read hit: service={}, account={}",
                    pid_read_current_user,
                    service,
                    redacted_account
                );
                read_current_user_recorder.record_auth_read(true);
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            },
        )?,
    )?;

    let pid_write = plugin_id.to_string();
    keychain_obj.set(
        "writeGenericPassword",
        Function::new(
            ctx.clone(),
            move |ctx_inner: Ctx<'_>, service: String, value: String| -> rquickjs::Result<()> {
                if !cfg!(target_os = "macos") {
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        "keychain API is only supported on macOS",
                    ));
                }
                log::info!("[plugin:{}] keychain write: service={}", pid_write, service);

                let mut account_arg: Option<String> = None;
                let find_output = std::process::Command::new("security")
                    .args(["find-generic-password", "-s", &service])
                    .output();

                if let Ok(output) = find_output {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        for line in stdout.lines() {
                            if let Some(start) = line.find("\"acct\"<blob>=\"") {
                                let rest = &line[start + 14..];
                                if let Some(end) = rest.find('"') {
                                    account_arg = Some(rest[..end].to_string());
                                    break;
                                }
                            }
                        }
                    }
                }

                let output = if let Some(ref acct) = account_arg {
                    std::process::Command::new("security")
                        .args(keychain_add_generic_password_args_for_account(
                            &service, acct, &value,
                        ))
                        .output()
                } else {
                    std::process::Command::new("security")
                        .args(keychain_add_generic_password_args(&service, &value))
                        .output()
                }
                .map_err(|e| {
                    Exception::throw_message(&ctx_inner, &format!("keychain write failed: {}", e))
                })?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let first_line = stderr.lines().next().unwrap_or("").trim();
                    log::warn!(
                        "[plugin:{}] keychain write failed: service={}, error={}",
                        pid_write,
                        service,
                        first_line
                    );
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        &format!("keychain write failed: {}", first_line),
                    ));
                }

                log::info!(
                    "[plugin:{}] keychain write succeeded: service={}",
                    pid_write,
                    service
                );
                Ok(())
            },
        )?,
    )?;

    let pid_write_current_user = plugin_id.to_string();
    keychain_obj.set(
        "writeGenericPasswordForCurrentUser",
        Function::new(
            ctx.clone(),
            move |ctx_inner: Ctx<'_>, service: String, value: String| -> rquickjs::Result<()> {
                if !cfg!(target_os = "macos") {
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        "keychain API is only supported on macOS",
                    ));
                }
                let account = current_macos_keychain_account();
                let args =
                    keychain_add_generic_password_args_for_account(&service, &account, &value);
                let redacted_account = crate::plugin_engine::redaction::redact_value(&account);
                log::info!(
                    "[plugin:{}] keychain write: service={}, account={}",
                    pid_write_current_user,
                    service,
                    redacted_account
                );
                let output = std::process::Command::new("security")
                    .args(&args)
                    .output()
                    .map_err(|e| {
                        Exception::throw_message(
                            &ctx_inner,
                            &format!("keychain write failed: {}", e),
                        )
                    })?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let first_line = stderr.lines().next().unwrap_or("").trim();
                    log::warn!(
                        "[plugin:{}] keychain write failed: service={}, account={}, error={}",
                        pid_write_current_user,
                        service,
                        redacted_account,
                        first_line
                    );
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        &format!("keychain write failed: {}", first_line),
                    ));
                }

                log::info!(
                    "[plugin:{}] keychain write succeeded: service={}, account={}",
                    pid_write_current_user,
                    service,
                    redacted_account
                );
                Ok(())
            },
        )?,
    )?;

    host.set("keychain", keychain_obj)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_macos_keychain_account_prefers_explicit_user_value() {
        assert_eq!(
            current_macos_keychain_account_from_user_env(Some("pulseusage-test-user".to_string())),
            "pulseusage-test-user"
        );
    }

    #[test]
    fn keychain_find_generic_password_args_include_service_only_lookup() {
        let args = keychain_find_generic_password_args("Claude Code-credentials");
        let rendered: Vec<String> = args
            .into_iter()
            .map(|value| value.to_string_lossy().into_owned())
            .collect();

        assert_eq!(
            rendered,
            vec![
                "find-generic-password",
                "-s",
                "Claude Code-credentials",
                "-w",
            ]
        );
    }

    #[test]
    fn keychain_find_generic_password_args_for_account_include_account_and_service() {
        let args = keychain_find_generic_password_args_for_account(
            "Claude Code-credentials",
            "pulseusage-test-user",
        );
        let rendered: Vec<String> = args
            .into_iter()
            .map(|value| value.to_string_lossy().into_owned())
            .collect();

        assert_eq!(
            rendered,
            vec![
                "find-generic-password",
                "-a",
                "pulseusage-test-user",
                "-s",
                "Claude Code-credentials",
                "-w",
            ]
        );
    }

    #[test]
    fn keychain_add_generic_password_args_include_service_only_write() {
        let args = keychain_add_generic_password_args("Claude Code-credentials", "secret-value");
        let rendered: Vec<String> = args
            .into_iter()
            .map(|value| value.to_string_lossy().into_owned())
            .collect();

        // macOS 27 requires -a account for add-generic-password. The service-only
        // path now includes the current user's account via current_macos_keychain_account().
        // Verify the -a flag and account value are present alongside service and value.
        assert!(rendered.contains(&"add-generic-password".to_string()));
        assert!(rendered.contains(&"-U".to_string()));
        assert!(rendered.contains(&"-a".to_string()));
        assert!(rendered.contains(&"-s".to_string()));
        assert!(rendered.contains(&"Claude Code-credentials".to_string()));
        assert!(rendered.contains(&"-w".to_string()));
        assert!(rendered.contains(&"secret-value".to_string()));
        // The account is the current macOS user — verify it's a non-empty string.
        let account = rendered
            .iter()
            .position(|v| v == "-a")
            .map(|i| rendered.get(i + 1).cloned())
            .flatten()
            .unwrap_or_default();
        assert!(!account.is_empty(), "account must not be empty");
    }

    #[test]
    fn keychain_add_generic_password_args_for_account_include_update_account_service_and_value() {
        let args = keychain_add_generic_password_args_for_account(
            "Claude Code-credentials",
            "pulseusage-test-user",
            "secret-value",
        );
        let rendered: Vec<String> = args
            .into_iter()
            .map(|value| value.to_string_lossy().into_owned())
            .collect();

        assert_eq!(
            rendered,
            vec![
                "add-generic-password",
                "-U",
                "-a",
                "pulseusage-test-user",
                "-s",
                "Claude Code-credentials",
                "-w",
                "secret-value",
            ]
        );
    }
}
