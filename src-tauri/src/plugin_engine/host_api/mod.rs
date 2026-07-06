mod ccusage;
mod crypto;
mod env;
mod fs;
mod http;
mod keychain;
mod logging;
mod ls;
mod plist;
mod sqlite;
mod utils;

pub use ccusage::patch_ccusage_wrapper;
pub use http::patch_http_wrapper;
pub use ls::patch_ls_wrapper;
pub use utils::inject_utils;

use crate::plugin_engine::capability::{HostCapability, HostCapabilitySet};
use crate::plugin_engine::diagnostics::ProbeDiagnosticsRecorder;
#[cfg(test)]
use aes_gcm::{
    AesGcm, Nonce,
    aead::{Aead, KeyInit, OsRng, generic_array::typenum::U16, rand_core::RngCore},
    aes::Aes256,
};
#[cfg(test)]
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use rquickjs::{Ctx, Object};
use std::path::PathBuf;
#[cfg(test)]
use std::time::Instant;

/// Redact sensitive value to first4...last4 format (UTF-8 safe)
#[cfg(test)]
pub(crate) fn inject_host_api<'js>(
    ctx: &Ctx<'js>,
    plugin_id: &str,
    app_data_dir: &PathBuf,
    app_version: &str,
) -> rquickjs::Result<()> {
    let capabilities = if cfg!(test) {
        crate::plugin_engine::capability::HostCapabilitySet::all()
    } else {
        crate::plugin_engine::capability::infer_v1_capabilities(plugin_id)
    };
    inject_host_api_with_deadline(
        ctx,
        plugin_id,
        app_data_dir,
        app_version,
        crate::plugin_engine::shared::ProbeDeadline::none(),
        ProbeDiagnosticsRecorder::default(),
        capabilities,
    )
}

pub(crate) fn inject_host_api_with_deadline<'js>(
    ctx: &Ctx<'js>,
    plugin_id: &str,
    app_data_dir: &PathBuf,
    app_version: &str,
    deadline: crate::plugin_engine::shared::ProbeDeadline,
    diagnostics_recorder: ProbeDiagnosticsRecorder,
    capabilities: HostCapabilitySet,
) -> rquickjs::Result<()> {
    let globals = ctx.globals();
    let probe_ctx = Object::new(ctx.clone())?;

    probe_ctx.set("nowIso", crate::plugin_engine::shared::iso_now())?;

    let app_obj = Object::new(ctx.clone())?;
    app_obj.set("version", app_version)?;
    app_obj.set("platform", std::env::consts::OS)?;
    app_obj.set("appDataDir", app_data_dir.to_string_lossy().to_string())?;
    let plugin_data_dir = app_data_dir.join("plugins_data").join(plugin_id);
    if let Err(err) = std::fs::create_dir_all(&plugin_data_dir) {
        log::warn!(
            "[plugin:{}] failed to create plugin data dir: {}",
            plugin_id,
            err
        );
    }
    app_obj.set(
        "pluginDataDir",
        plugin_data_dir.to_string_lossy().to_string(),
    )?;
    probe_ctx.set("app", app_obj)?;

    let host = Object::new(ctx.clone())?;

    // Log is always available — not a capability.
    logging::inject_log(ctx, &host, plugin_id)?;

    // Each host.* module is injected only if the plugin declared the
    // corresponding capability. Undeclared capabilities result in the
    // JS function not existing on ctx.host (TypeError at call site —
    // fail-safe).
    if capabilities.contains(HostCapability::FsRead)
        || capabilities.contains(HostCapability::FsWrite)
        || capabilities.contains(HostCapability::FsListDir)
    {
        fs::inject_fs(ctx, &host, diagnostics_recorder.clone())?;
    }

    if capabilities.contains(HostCapability::PlistRead) {
        plist::inject_plist(ctx, &host, diagnostics_recorder.clone())?;
    }

    if capabilities.contains(HostCapability::CryptoAes)
        || capabilities.contains(HostCapability::CryptoSha)
    {
        crypto::inject_crypto(ctx, &host)?;
    }

    if capabilities.contains(HostCapability::EnvRead) {
        env::inject_env(ctx, &host, plugin_id, diagnostics_recorder.clone())?;
    }

    if capabilities.contains(HostCapability::HttpRequest) {
        http::inject_http(
            ctx,
            &host,
            plugin_id,
            deadline,
            diagnostics_recorder.clone(),
        )?;
    }

    if capabilities.contains(HostCapability::KeychainRead)
        || capabilities.contains(HostCapability::KeychainWrite)
        || capabilities.contains(HostCapability::KeychainDelete)
    {
        keychain::inject_keychain(ctx, &host, plugin_id, diagnostics_recorder.clone())?;
    }

    if capabilities.contains(HostCapability::SqliteQuery)
        || capabilities.contains(HostCapability::SqliteExec)
    {
        sqlite::inject_sqlite(ctx, &host, plugin_id, diagnostics_recorder.clone())?;
    }

    if capabilities.contains(HostCapability::LsDiscover) {
        ls::inject_ls(ctx, &host, plugin_id, diagnostics_recorder.clone())?;
    }

    if capabilities.contains(HostCapability::CcusageQuery) {
        ccusage::inject_ccusage(ctx, &host, plugin_id, deadline, diagnostics_recorder)?;
    }

    probe_ctx.set("host", host)?;
    globals.set("__pulseusage_ctx", probe_ctx)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin_engine::host_api::ccusage::*;
    use rquickjs::{Context, Function, Object, Runtime};
    use std::time::Duration;

    fn encrypt_aes_256_gcm_envelope_for_test(key: &[u8], plaintext: &str) -> String {
        let iv = [7_u8; 16];
        type Aes256Gcm16 = AesGcm<Aes256, U16>;
        let cipher = Aes256Gcm16::new_from_slice(key).expect("encrypt init");
        let nonce = Nonce::<U16>::from_slice(&iv);
        let ciphertext_and_tag = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .expect("encrypt finalize");
        let split_at = ciphertext_and_tag.len() - 16;
        let (ciphertext, tag) = ciphertext_and_tag.split_at(split_at);

        format!(
            "{}:{}:{}",
            BASE64_STANDARD.encode(iv),
            BASE64_STANDARD.encode(tag),
            BASE64_STANDARD.encode(ciphertext)
        )
    }

    fn node_generated_aes_256_gcm_vector_for_test() -> (&'static str, &'static str, &'static str) {
        (
            "CwsLCwsLCwsLCwsLCwsLCwsLCwsLCwsLCwsLCwsLCws=",
            "BwcHBwcHBwcHBwcHBwcHBw==:yFbCs4LOJ0aj9NPNf5pfVA==:7PKjtOdATLClvaWrMw0b0M8Nov4KPhxwQX4hdczqQlcZi9Zhi6DjAoK+WolvMwuhPIk=",
            r#"{"access_token":"token","refresh_token":"refresh"}"#,
        )
    }

    #[test]
    fn last_non_empty_trimmed_line_uses_final_value_when_stdout_is_noisy() {
        let stdout = "banner line\nanother message\n  sk-test-key-12345  \n";
        let value = env::last_non_empty_trimmed_line(stdout);
        assert_eq!(value.as_deref(), Some("sk-test-key-12345"));
    }

    #[test]
    fn credential_env_var_detection_ignores_non_secret_config() {
        assert!(!env::is_credential_env_var("CODEX_HOME"));
        assert!(!env::is_credential_env_var("USE_LOCAL_OAUTH"));
        assert!(!env::is_credential_env_var("CLAUDE_CODE_OAUTH_CLIENT_ID"));
        assert!(env::is_credential_env_var("CLAUDE_CODE_OAUTH_TOKEN"));
        assert!(env::is_credential_env_var("ZAI_API_KEY"));
    }

    #[test]
    fn last_non_empty_trimmed_line_returns_none_for_empty_stdout() {
        let stdout = "  \n\n\t\n";
        let value = env::last_non_empty_trimmed_line(stdout);
        assert!(value.is_none());
    }

    #[test]
    fn decrypt_aes_256_gcm_envelope_round_trips_plaintext() {
        let key = [11_u8; 32];
        let key_b64 = BASE64_STANDARD.encode(key);
        let plaintext = r#"{"access_token":"token","refresh_token":"refresh"}"#;
        let envelope = encrypt_aes_256_gcm_envelope_for_test(&key, plaintext);

        let decrypted =
            crypto::decrypt_aes_256_gcm_envelope(&envelope, &key_b64).expect("decrypt envelope");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn encrypt_aes_256_gcm_envelope_round_trips_plaintext() {
        let key = [21_u8; 32];
        let key_b64 = BASE64_STANDARD.encode(key);
        let plaintext = r#"{"access_token":"token-2","refresh_token":"refresh-2"}"#;

        let envelope =
            crypto::encrypt_aes_256_gcm_envelope(plaintext, &key_b64).expect("encrypt envelope");
        let decrypted =
            crypto::decrypt_aes_256_gcm_envelope(&envelope, &key_b64).expect("decrypt envelope");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn decrypt_aes_256_gcm_envelope_rejects_invalid_component_lengths() {
        let key_b64 = BASE64_STANDARD.encode([9_u8; 32]);
        let short_key_b64 = BASE64_STANDARD.encode([7_u8; 31]);
        let iv_b64 = BASE64_STANDARD.encode([1_u8; 15]);
        let tag_b64 = BASE64_STANDARD.encode([2_u8; 16]);
        let ciphertext_b64 = BASE64_STANDARD.encode([3_u8; 8]);

        let key_err = crypto::decrypt_aes_256_gcm_envelope("AQ==:AQ==:AQ==", &short_key_b64)
            .expect_err("key length");
        assert!(key_err.contains("expected 32 bytes"));

        let iv_err = crypto::decrypt_aes_256_gcm_envelope(
            &format!("{}:{}:{}", iv_b64, tag_b64, ciphertext_b64),
            &key_b64,
        )
        .expect_err("iv length");
        assert!(iv_err.contains("iv length"));

        let short_tag_b64 = BASE64_STANDARD.encode([2_u8; 15]);
        let tag_err = crypto::decrypt_aes_256_gcm_envelope(
            &format!(
                "{}:{}:{}",
                BASE64_STANDARD.encode([1_u8; 16]),
                short_tag_b64,
                ciphertext_b64
            ),
            &key_b64,
        )
        .expect_err("tag length");
        assert!(tag_err.contains("auth tag length"));
    }

    #[test]
    fn sanitize_env_value_strips_ansi_and_control_sequences() {
        let raw = "\u{1b}[?1000l\n  sk-test-key-12345\u{1b}[?2004h\r\n";
        let value = env::sanitize_env_value(raw);
        assert_eq!(value.as_deref(), Some("sk-test-key-12345"));
    }

    #[test]
    fn extract_marked_value_ignores_noisy_shell_output() {
        let stdout = concat!(
            "startup banner\n",
            "\u{1b}[31mplugin failed\u{1b}[0m\n",
            "__OPENUSAGE_ENV_START__\n",
            "  sk-test-key-12345  \n",
            "__OPENUSAGE_ENV_END__\n",
            "\u{1b}[32muser@host\u{1b}[0m\n"
        );
        let value =
            env::extract_marked_value(stdout, "__OPENUSAGE_ENV_START__", "__OPENUSAGE_ENV_END__");
        assert_eq!(value.as_deref(), Some("sk-test-key-12345"));
    }

    #[test]
    fn extract_marked_value_strips_inline_terminal_sequences_from_marked_value() {
        let stdout = concat!(
            "__OPENUSAGE_ENV_START__\n",
            "\u{1b}[?1000l\n",
            "  sk-test-key-12345\u{1b}[?2004h\r\n",
            "__OPENUSAGE_ENV_END__\n"
        );
        let value =
            env::extract_marked_value(stdout, "__OPENUSAGE_ENV_START__", "__OPENUSAGE_ENV_END__");
        assert_eq!(value.as_deref(), Some("sk-test-key-12345"));
    }

    #[test]
    fn extract_marked_value_returns_none_when_marked_value_is_empty() {
        let stdout = "__OPENUSAGE_ENV_START__\n  \n__OPENUSAGE_ENV_END__\n";
        let value =
            env::extract_marked_value(stdout, "__OPENUSAGE_ENV_START__", "__OPENUSAGE_ENV_END__");
        assert!(value.is_none());
    }

    #[test]
    fn parse_interactive_shell_env_output_does_not_fallback_to_end_marker_for_empty_value() {
        let stdout = "__OPENUSAGE_ENV_START__\n  \n__OPENUSAGE_ENV_END__\n";
        let value = env::parse_interactive_shell_env_output(
            stdout,
            "__OPENUSAGE_ENV_START__",
            "__OPENUSAGE_ENV_END__",
        );
        assert!(value.is_none());
    }

    #[test]
    fn parse_interactive_shell_env_output_falls_back_without_markers() {
        let stdout = "\u{1b}[?1000l\n  sk-test-key-12345\u{1b}[?2004h\r\n";
        let value = env::parse_interactive_shell_env_output(
            stdout,
            "__OPENUSAGE_ENV_START__",
            "__OPENUSAGE_ENV_END__",
        );
        assert_eq!(value.as_deref(), Some("sk-test-key-12345"));
    }

    #[test]
    fn crypto_api_exposes_decrypt() {
        let rt = Runtime::new().expect("runtime");
        let ctx = Context::full(&rt).expect("context");
        ctx.with(|ctx| {
            let app_data = std::env::temp_dir();
            inject_host_api(&ctx, "test", &app_data, "0.0.0").expect("inject host api");
            let globals = ctx.globals();
            let probe_ctx: Object = globals.get("__pulseusage_ctx").expect("probe ctx");
            let host: Object = probe_ctx.get("host").expect("host");
            let crypto: Object = host.get("crypto").expect("crypto");
            let _decrypt: Function = crypto.get("decryptAes256Gcm").expect("decryptAes256Gcm");
            let _encrypt: Function = crypto.get("encryptAes256Gcm").expect("encryptAes256Gcm");
        });
    }

    #[test]
    fn crypto_api_decrypts_node_generated_envelope_from_js() {
        let (key_b64, envelope, expected_plaintext) = node_generated_aes_256_gcm_vector_for_test();
        let rt = Runtime::new().expect("runtime");
        let ctx = Context::full(&rt).expect("context");
        ctx.with(|ctx| {
            let app_data = std::env::temp_dir();
            inject_host_api(&ctx, "test", &app_data, "0.0.0").expect("inject host api");
            let js_expr = format!(
                r#"__pulseusage_ctx.host.crypto.decryptAes256Gcm("{}", "{}")"#,
                envelope, key_b64
            );
            let decrypted: String = ctx.eval(js_expr).expect("js decrypt");
            assert_eq!(decrypted, expected_plaintext);
        });
    }

    #[test]
    fn crypto_api_exposes_sha256_hex() {
        let rt = Runtime::new().expect("runtime");
        let ctx = Context::full(&rt).expect("context");
        ctx.with(|ctx| {
            let app_data = std::env::temp_dir();
            inject_host_api(&ctx, "test", &app_data, "0.0.0").expect("inject host api");
            // Vector: `printf '%s' 'hello' | shasum -a 256`
            let result: String = ctx
                .eval(r#"__pulseusage_ctx.host.crypto.sha256Hex("hello")"#)
                .expect("js sha256");
            assert_eq!(
                result,
                "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
            );

            let empty: String = ctx
                .eval(r#"__pulseusage_ctx.host.crypto.sha256Hex("")"#)
                .expect("js sha256 empty");
            assert_eq!(
                empty,
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            );
        });
    }

    #[test]
    fn keychain_api_exposes_write_variants() {
        let rt = Runtime::new().expect("runtime");
        let ctx = Context::full(&rt).expect("context");
        ctx.with(|ctx| {
            let app_data = std::env::temp_dir();
            inject_host_api(&ctx, "test", &app_data, "0.0.0").expect("inject host api");
            let globals = ctx.globals();
            let probe_ctx: Object = globals.get("__pulseusage_ctx").expect("probe ctx");
            let host: Object = probe_ctx.get("host").expect("host");
            let keychain: Object = host.get("keychain").expect("keychain");
            let _read: Function = keychain
                .get("readGenericPassword")
                .expect("readGenericPassword");
            let _read_current_user: Function = keychain
                .get("readGenericPasswordForCurrentUser")
                .expect("readGenericPasswordForCurrentUser");
            let _write: Function = keychain
                .get("writeGenericPassword")
                .expect("writeGenericPassword");
            let _write_current_user: Function = keychain
                .get("writeGenericPasswordForCurrentUser")
                .expect("writeGenericPasswordForCurrentUser");
        });
    }

    /// Contract test: readGenericPassword must accept a single service argument
    /// (omitting account). Plugins call `readGenericPassword(service)` without an
    /// account, so the host binding must treat the account parameter as optional.
    /// This test prevents regressions where rquickjs's strict arg count rejects
    /// the 1-arg call pattern that production plugins depend on.
    #[test]
    fn keychain_read_generic_password_accepts_single_arg() {
        let rt = Runtime::new().expect("runtime");
        let ctx = Context::full(&rt).expect("context");
        let err_msg = ctx.with(|ctx| -> String {
            let app_data = std::env::temp_dir();
            inject_host_api(&ctx, "test", &app_data, "0.0.0").expect("inject host api");
            // Call readGenericPassword with ONE argument from JS, exactly as
            // copilot/cursor/codex/factory/claude plugins do. On non-macOS this
            // throws "keychain API is only supported on macOS" — that specific
            // message proves the 1-arg call reached the host function body.
            // An arg-count rejection would throw a different TypeError
            // ("1 argument(s) while 2 where expected") and fail this test.
            let result: rquickjs::Value = ctx
                .eval(
                    r#"
                    (function() {
                        try {
                            __pulseusage_ctx.host.keychain.readGenericPassword("test-service");
                            return "no-throw";
                        } catch (e) { return String(e); }
                    })()
                    "#,
                )
                .expect("eval");
            match result.into_string() {
                Some(s) => s.to_string().unwrap_or_default(),
                None => String::new(),
            }
        });
        assert!(
            err_msg.contains("keychain API is only supported on macOS")
                || err_msg.contains("keychain item not found")
                || err_msg.contains("no-throw"),
            "1-arg readGenericPassword should reach the host body, got: {}",
            err_msg
        );
        // The error "1 argument(s) while 2 where expected" would mean the binding
        // regressed to requiring 2 args — that is exactly what this test guards.
        assert!(
            !err_msg.contains("argument(s) while"),
            "readGenericPassword regressed to strict arg count: {}",
            err_msg
        );
    }

    #[test]
    fn env_api_respects_allowlist_in_host_and_js() {
        let claude_env_vars = [
            "CLAUDE_CONFIG_DIR",
            "CLAUDE_CODE_OAUTH_TOKEN",
            "USER_TYPE",
            "USE_STAGING_OAUTH",
            "USE_LOCAL_OAUTH",
            "CLAUDE_CODE_CUSTOM_OAUTH_URL",
            "CLAUDE_CODE_OAUTH_CLIENT_ID",
            "CLAUDE_LOCAL_OAUTH_API_BASE",
        ];

        for name in claude_env_vars {
            assert!(
                env::WHITELISTED_ENV_VARS.contains(&name),
                "{name} must be whitelisted for Claude auth compatibility"
            );
        }

        let rt = Runtime::new().expect("runtime");
        let ctx = Context::full(&rt).expect("context");
        ctx.with(|ctx| {
            let app_data = std::env::temp_dir();
            inject_host_api(&ctx, "test", &app_data, "0.0.0").expect("inject host api");
            let globals = ctx.globals();
            let probe_ctx: Object = globals.get("__pulseusage_ctx").expect("probe ctx");
            let host: Object = probe_ctx.get("host").expect("host");
            let env: Object = host.get("env").expect("env");
            let get: Function = env.get("get").expect("get");

            for name in env::WHITELISTED_ENV_VARS {
                let expected = env::resolve_env_value(name);
                let value: Option<String> =
                    get.call((name.to_string(),)).expect("get whitelisted var");
                assert_eq!(value, expected, "{name} should match host env resolver");

                let js_expr = format!(r#"__pulseusage_ctx.host.env.get("{}")"#, name);
                let js_value: Option<String> = ctx.eval(js_expr).expect("js get whitelisted var");
                assert_eq!(
                    js_value, expected,
                    "{name} should match host env resolver from JS"
                );
            }

            let blocked: Option<String> = get
                .call(("__OPENUSAGE_TEST_NOT_WHITELISTED__".to_string(),))
                .expect("get blocked var");
            assert!(
                blocked.is_none(),
                "non-whitelisted vars must not be exposed"
            );

            let js_blocked: Option<String> = ctx
                .eval(r#"__pulseusage_ctx.host.env.get("__OPENUSAGE_TEST_NOT_WHITELISTED__")"#)
                .expect("js get blocked var");
            assert!(
                js_blocked.is_none(),
                "non-whitelisted vars must not be exposed from JS"
            );
        });
    }

    #[test]
    fn env_api_prefers_process_env() {
        struct RestoreEnvVar {
            name: &'static str,
            old: Option<String>,
        }

        impl Drop for RestoreEnvVar {
            fn drop(&mut self) {
                if let Some(value) = self.old.take() {
                    // SAFETY: tests serialize env changes via this guard; value is restored on drop.
                    unsafe { std::env::set_var(self.name, value) };
                } else {
                    // SAFETY: tests serialize env changes via this guard; var is restored/removed on drop.
                    unsafe { std::env::remove_var(self.name) };
                }
            }
        }

        let name = "ZAI_API_KEY";
        let old = std::env::var(name).ok();
        let _restore = RestoreEnvVar { name, old };
        // SAFETY: this test restores the previous value in `Drop`.
        unsafe { std::env::set_var(name, "sk-process-env-test-1234567890") };

        let rt = Runtime::new().expect("runtime");
        let ctx = Context::full(&rt).expect("context");
        ctx.with(|ctx| {
            let app_data = std::env::temp_dir();
            inject_host_api(&ctx, "test", &app_data, "0.0.0").expect("inject host api");
            let globals = ctx.globals();
            let probe_ctx: Object = globals.get("__pulseusage_ctx").expect("probe ctx");
            let host: Object = probe_ctx.get("host").expect("host");
            let env: Object = host.get("env").expect("env");
            let get: Function = env.get("get").expect("get");

            let value: Option<String> = get.call((name.to_string(),)).expect("get");
            assert_eq!(
                value.as_deref(),
                Some("sk-process-env-test-1234567890"),
                "process env should be preferred over shell lookup"
            );

            let js_value: Option<String> = ctx
                .eval(r#"__pulseusage_ctx.host.env.get("ZAI_API_KEY")"#)
                .expect("js get");
            assert_eq!(
                js_value.as_deref(),
                Some("sk-process-env-test-1234567890"),
                "process env should be preferred from JS"
            );
        });
    }

    #[test]
    fn expand_path_expands_tilde_prefix() {
        let home = dirs::home_dir().expect("home dir");
        let expected = home.join(".claude-custom").to_string_lossy().to_string();

        assert_eq!(
            crate::plugin_engine::shared::expand_path("~/.claude-custom"),
            expected
        );
    }

    #[test]
    fn redact_value_shows_first_and_last_four() {
        assert_eq!(
            crate::plugin_engine::redaction::redact_value("sk-1234567890abcdef"),
            "sk-1...cdef"
        );
        assert_eq!(
            crate::plugin_engine::redaction::redact_value("short"),
            "[REDACTED]"
        );
    }

    #[test]
    fn redact_url_redacts_api_key_param() {
        let url = "https://api.example.com/v1?api_key=sk-1234567890abcdef&other=value";
        let redacted = crate::plugin_engine::redaction::redact_url(url);
        assert!(redacted.contains("api_key=sk-1...cdef"));
        assert!(redacted.contains("other=value"));
    }

    #[test]
    fn redact_url_redacts_user_query_param() {
        let url = "https://cursor.com/api/usage?user=user_abcdefghijklmnopqrstuvwxyz&limit=10";
        let redacted = crate::plugin_engine::redaction::redact_url(url);
        assert!(
            redacted.contains("user=user...wxyz"),
            "user query param should be redacted, got: {}",
            redacted
        );
        assert!(
            redacted.contains("limit=10"),
            "non-sensitive params should be preserved, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_url_preserves_non_sensitive_params() {
        let url = "https://api.example.com/v1?limit=10&offset=20";
        assert_eq!(crate::plugin_engine::redaction::redact_url(url), url);
    }

    #[test]
    fn redact_url_redacts_profile_arn_query_param() {
        let url = "https://q.us-east-1.amazonaws.com/getUsageLimits?profileArn=arn:aws:codewhisperer:us-east-1:699475941385:profile/EHGA3GRVQMUK&origin=AI_EDITOR";
        let redacted = crate::plugin_engine::redaction::redact_url(url);
        assert!(
            !redacted.contains("699475941385"),
            "profileArn should be redacted, got: {}",
            redacted
        );
        assert!(
            redacted.contains("origin=AI_EDITOR"),
            "non-sensitive params should remain visible, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_body_redacts_jwt() {
        let body = r#"{"token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U"}"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        // JWT gets redacted to first4...last4 format
        assert!(
            !redacted.contains("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"),
            "full JWT should be redacted, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_body_redacts_api_keys() {
        let body = r#"{"key": "sk-1234567890abcdefghij"}"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        assert!(redacted.contains("sk-1...ghij"));
    }

    #[test]
    fn redact_body_redacts_devin_session_token() {
        let body = r#"metadata apiKey=devin-session-token$abcdefghijklmnopqrstuvwxyz123456"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        assert!(
            !redacted.contains("devin-session-token$abcdefghijklmnopqrstuvwxyz123456"),
            "Devin session token should be redacted, got: {}",
            redacted
        );
        assert!(
            redacted.contains("devi...3456"),
            "Devin session token should use first4...last4 redaction, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_body_redacts_json_password_field() {
        let body = r#"{"password": "supersecretpassword123"}"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        assert!(
            !redacted.contains("supersecretpassword123"),
            "password should be redacted, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_body_redacts_user_id_and_email() {
        let body = r#"{"user_id": "user-iupzZ7KFykMLrnzpkHSq7wjo", "email": "sample@example.com"}"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        assert!(
            !redacted.contains("user-iupzZ7KFykMLrnzpkHSq7wjo"),
            "user_id should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("sample@example.com"),
            "email should be redacted, got: {}",
            redacted
        );
        // Should show first4...last4
        assert!(
            redacted.contains("user...7wjo"),
            "user_id should show first4...last4, got: {}",
            redacted
        );
        assert!(
            redacted.contains("samp....com"),
            "email should show first4...last4, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_body_redacts_camel_case_user_and_account_ids() {
        let body = r#"{"userId": "user_abcdefghijklmnopqrstuvwxyz", "accountId": "acct_1234567890abcdef"}"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        assert!(
            !redacted.contains("user_abcdefghijklmnopqrstuvwxyz"),
            "userId should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("acct_1234567890abcdef"),
            "accountId should be redacted, got: {}",
            redacted
        );
        assert!(
            redacted.contains("user...wxyz"),
            "userId should show first4...last4, got: {}",
            redacted
        );
        assert!(
            redacted.contains("acct...cdef"),
            "accountId should show first4...last4, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_body_redacts_devin_org_and_account_display_name() {
        let body = r#"{"orgId":"org-6b6e9de248db472bb25b296599ea3dc0","accountDisplayName":"user@example.com","devinInfo":{"org_id":"org-abcdef1234567890","account_display_name":"team@example.com"}}"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        assert!(
            !redacted.contains("org-6b6e9de248db472bb25b296599ea3dc0"),
            "orgId should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("user@example.com"),
            "accountDisplayName should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("org-abcdef1234567890"),
            "org_id should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("team@example.com"),
            "account_display_name should be redacted, got: {}",
            redacted
        );
        assert!(
            redacted.contains("org-...3dc0"),
            "orgId should show first4...last4, got: {}",
            redacted
        );
        assert!(
            redacted.contains("user....com"),
            "accountDisplayName should show first4...last4, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_body_redacts_team_id_payment_id_and_paths() {
        let body = r#"{"teamId":"cc1ac023-9ff5-4c1f-a5a4-ae2a82df4243","paymentId":"cus_S5m1PGxjLWoc1c","binaryPath":"/opt/homebrew/bin/bunx","homePath":"/Users/sample/.claude"}"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        assert!(
            !redacted.contains("cc1ac023-9ff5-4c1f-a5a4-ae2a82df4243"),
            "teamId should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("cus_S5m1PGxjLWoc1c"),
            "paymentId should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("/opt/homebrew/bin/bunx"),
            "path should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("/Users/sample/.claude"),
            "path should be redacted, got: {}",
            redacted
        );
        assert!(
            redacted.contains("[PATH]"),
            "expected path marker, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_body_redacts_profile_arn_fields() {
        let body = r#"{"profileArn":"arn:aws:codewhisperer:us-east-1:699475941385:profile/EHGA3GRVQMUK","profile_arn":"arn:aws:codewhisperer:us-east-1:699475941385:profile/EHGA3GRVQMUK"}"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        assert!(
            !redacted.contains("699475941385"),
            "profile arn should be redacted, got: {}",
            redacted
        );
        assert!(
            redacted.contains("arn:...QMUK"),
            "profile arn should use first4...last4 redaction, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_log_message_redacts_jwt_and_api_key() {
        let msg = "token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U key=sk-1234567890abcdef";
        let redacted = crate::plugin_engine::redaction::redact_log_message(msg);
        assert!(
            !redacted.contains("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"),
            "JWT should be redacted"
        );
        assert!(
            !redacted.contains("sk-1234567890abcdef"),
            "API key should be redacted"
        );
    }

    #[test]
    fn redact_log_message_redacts_devin_session_token() {
        let msg = "auth=devin-session-token$abcdefghijklmnopqrstuvwxyz123456";
        let redacted = crate::plugin_engine::redaction::redact_log_message(msg);
        assert!(
            !redacted.contains("devin-session-token$abcdefghijklmnopqrstuvwxyz123456"),
            "Devin session token should be redacted, got: {}",
            redacted
        );
        assert!(
            redacted.contains("devi...3456"),
            "Devin session token should use first4...last4 redaction, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_log_message_redacts_account_and_paths() {
        let msg = "keychain read: service=Claude Code-credentials, account=sample path=/opt/homebrew/bin/bunx home=/Users/sample/.claude";
        let redacted = crate::plugin_engine::redaction::redact_log_message(msg);
        assert!(
            !redacted.contains("account=sample"),
            "account should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("/opt/homebrew/bin/bunx"),
            "path should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("/Users/sample/.claude"),
            "path should be redacted, got: {}",
            redacted
        );
        assert!(
            redacted.contains("account=[REDACTED]"),
            "expected redacted account, got: {}",
            redacted
        );
        assert!(
            redacted.contains("[PATH]"),
            "expected redacted path, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_body_redacts_login_and_analytics_tracking_id() {
        let body =
            r#"{"login":"testuser","analytics_tracking_id":"c9df3f012bb8c2eb7aae6868ee8da6cf"}"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        assert!(
            !redacted.contains("testuser"),
            "login should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("c9df3f012bb8c2eb7aae6868ee8da6cf"),
            "analytics_tracking_id should be redacted, got: {}",
            redacted
        );
        // login is short (<=12 chars) so becomes [REDACTED]; analytics_tracking_id is long so first4...last4
        assert!(
            redacted.contains("[REDACTED]"),
            "login should be redacted, got: {}",
            redacted
        );
        assert!(
            redacted.contains("c9df...a6cf"),
            "analytics_tracking_id should show first4...last4, got: {}",
            redacted
        );
    }

    #[test]
    fn redact_body_redacts_name_field() {
        let body =
            r#"{"userStatus":{"name":"Sample User","email":"sample@example.com","planStatus":{}}}"#;
        let redacted = crate::plugin_engine::redaction::redact_body(body);
        assert!(
            !redacted.contains("Sample User"),
            "name should be redacted, got: {}",
            redacted
        );
        assert!(
            !redacted.contains("sample@example.com"),
            "email should be redacted, got: {}",
            redacted
        );
        // "Sample User" is 11 chars (<=12) so becomes [REDACTED]
        assert!(
            redacted.contains("\"name\": \"[REDACTED]\""),
            "name should show [REDACTED], got: {}",
            redacted
        );
    }

    #[test]
    fn ccusage_runner_order_matches_expected_priority() {
        assert_eq!(
            ccusage_runner_order(),
            [
                CcusageRunnerKind::Bunx,
                CcusageRunnerKind::PnpmDlx,
                CcusageRunnerKind::YarnDlx,
                CcusageRunnerKind::NpmExec,
                CcusageRunnerKind::Npx
            ]
        );
    }

    #[test]
    fn ccusage_runner_args_include_expected_non_interactive_flags() {
        let opts = CcusageQueryOpts {
            provider: None,
            since: Some("20260101".to_string()),
            until: Some("20260131".to_string()),
            home_path: None,
            claude_path: None,
        };
        let expected_ccusage_package = ccusage_package_spec();
        assert_eq!(expected_ccusage_package, "ccusage@20.0.2");
        let expected_npm_exec_package = format!("--package={expected_ccusage_package}");

        let bunx = ccusage_runner_args(
            CcusageRunnerKind::Bunx,
            &opts,
            CcusageProvider::Claude,
            CcusageCommandFlavor::Current,
        );
        assert_eq!(
            bunx,
            vec![
                "--silent",
                expected_ccusage_package.as_str(),
                "claude",
                "daily",
                "--json",
                "--order",
                "desc",
                "--since",
                "20260101",
                "--until",
                "20260131"
            ]
        );

        let pnpm = ccusage_runner_args(
            CcusageRunnerKind::PnpmDlx,
            &opts,
            CcusageProvider::Claude,
            CcusageCommandFlavor::Current,
        );
        assert_eq!(
            pnpm,
            vec![
                "-s",
                "dlx",
                expected_ccusage_package.as_str(),
                "claude",
                "daily",
                "--json",
                "--order",
                "desc",
                "--since",
                "20260101",
                "--until",
                "20260131"
            ]
        );

        let yarn = ccusage_runner_args(
            CcusageRunnerKind::YarnDlx,
            &opts,
            CcusageProvider::Claude,
            CcusageCommandFlavor::Current,
        );
        assert_eq!(
            yarn,
            vec![
                "dlx",
                "-q",
                expected_ccusage_package.as_str(),
                "claude",
                "daily",
                "--json",
                "--order",
                "desc",
                "--since",
                "20260101",
                "--until",
                "20260131"
            ]
        );

        let npm_exec = ccusage_runner_args(
            CcusageRunnerKind::NpmExec,
            &opts,
            CcusageProvider::Claude,
            CcusageCommandFlavor::Current,
        );
        assert_eq!(
            npm_exec,
            vec![
                "exec",
                "--yes",
                expected_npm_exec_package.as_str(),
                "--",
                "ccusage",
                "claude",
                "daily",
                "--json",
                "--order",
                "desc",
                "--since",
                "20260101",
                "--until",
                "20260131"
            ]
        );

        let npx = ccusage_runner_args(
            CcusageRunnerKind::Npx,
            &opts,
            CcusageProvider::Claude,
            CcusageCommandFlavor::Current,
        );
        assert_eq!(
            npx,
            vec![
                "--yes",
                expected_ccusage_package.as_str(),
                "claude",
                "daily",
                "--json",
                "--order",
                "desc",
                "--since",
                "20260101",
                "--until",
                "20260131"
            ]
        );
    }

    #[test]
    fn ccusage_runner_args_codex_use_unified_package_and_bin() {
        let opts = CcusageQueryOpts {
            provider: Some("codex".to_string()),
            since: Some("20260101".to_string()),
            until: Some("20260131".to_string()),
            home_path: None,
            claude_path: None,
        };
        let expected_ccusage_package = ccusage_package_spec();
        let expected_npm_exec_package = format!("--package={expected_ccusage_package}");

        let bunx = ccusage_runner_args(
            CcusageRunnerKind::Bunx,
            &opts,
            CcusageProvider::Codex,
            CcusageCommandFlavor::Current,
        );
        assert_eq!(
            bunx,
            vec![
                "--silent",
                expected_ccusage_package.as_str(),
                "codex",
                "daily",
                "--json",
                "--order",
                "desc",
                "--since",
                "20260101",
                "--until",
                "20260131"
            ]
        );

        let npm_exec = ccusage_runner_args(
            CcusageRunnerKind::NpmExec,
            &opts,
            CcusageProvider::Codex,
            CcusageCommandFlavor::Current,
        );
        assert_eq!(
            npm_exec,
            vec![
                "exec",
                "--yes",
                expected_npm_exec_package.as_str(),
                "--",
                "ccusage",
                "codex",
                "daily",
                "--json",
                "--order",
                "desc",
                "--since",
                "20260101",
                "--until",
                "20260131"
            ]
        );

        let npx = ccusage_runner_args(
            CcusageRunnerKind::Npx,
            &opts,
            CcusageProvider::Codex,
            CcusageCommandFlavor::Current,
        );
        assert_eq!(
            npx,
            vec![
                "--yes",
                expected_ccusage_package.as_str(),
                "codex",
                "daily",
                "--json",
                "--order",
                "desc",
                "--since",
                "20260101",
                "--until",
                "20260131"
            ]
        );
    }

    #[test]
    fn ccusage_runner_args_legacy_fallback_uses_release_age_safe_packages() {
        let opts = CcusageQueryOpts {
            provider: None,
            since: Some("20260101".to_string()),
            until: Some("20260131".to_string()),
            home_path: None,
            claude_path: None,
        };

        let claude = ccusage_runner_args(
            CcusageRunnerKind::Bunx,
            &opts,
            CcusageProvider::Claude,
            CcusageCommandFlavor::Legacy,
        );
        assert_eq!(
            claude,
            vec![
                "--silent",
                "ccusage@18.0.11",
                "daily",
                "--json",
                "--order",
                "desc",
                "--since",
                "20260101",
                "--until",
                "20260131"
            ]
        );

        let codex_npm = ccusage_runner_args(
            CcusageRunnerKind::NpmExec,
            &opts,
            CcusageProvider::Codex,
            CcusageCommandFlavor::Legacy,
        );
        assert_eq!(
            codex_npm,
            vec![
                "exec",
                "--yes",
                "--package=@ccusage/codex@18.0.11",
                "--",
                "ccusage-codex",
                "daily",
                "--json",
                "--order",
                "desc",
                "--since",
                "20260101",
                "--until",
                "20260131"
            ]
        );
    }

    #[test]
    fn ccusage_path_entries_with_home_and_existing_path_preserves_order() {
        let home = std::path::PathBuf::from("/tmp/pulseusage-home");
        let existing = std::env::join_paths([
            std::path::PathBuf::from("/usr/bin"),
            std::path::PathBuf::from("/bin"),
        ])
        .expect("join existing path");

        let entries = ccusage_path_entries_with(Some(home.as_path()), Some(existing.as_os_str()));
        assert_eq!(
            entries,
            vec![
                home.join(".bun/bin"),
                home.join(".nvm/current/bin"),
                home.join(".local/bin"),
                std::path::PathBuf::from("/opt/homebrew/bin"),
                std::path::PathBuf::from("/usr/local/bin"),
                std::path::PathBuf::from("/usr/bin"),
                std::path::PathBuf::from("/bin"),
            ]
        );
    }

    #[test]
    fn ccusage_path_entries_with_deduplicates_prefix_and_existing_entries() {
        let existing = std::env::join_paths([
            std::path::PathBuf::from("/usr/local/bin"),
            std::path::PathBuf::from("/custom/bin"),
            std::path::PathBuf::from("/custom/bin"),
            std::path::PathBuf::from("/opt/homebrew/bin"),
        ])
        .expect("join existing path");

        let entries = ccusage_path_entries_with(None, Some(existing.as_os_str()));
        assert_eq!(
            entries,
            vec![
                std::path::PathBuf::from("/opt/homebrew/bin"),
                std::path::PathBuf::from("/usr/local/bin"),
                std::path::PathBuf::from("/custom/bin"),
            ]
        );
    }

    #[test]
    fn ccusage_enriched_path_with_uses_defaults_without_home_or_existing_path() {
        let enriched = ccusage_enriched_path_with(None, None).expect("enriched path");
        let entries: Vec<std::path::PathBuf> =
            std::env::split_paths(enriched.as_os_str()).collect();
        assert_eq!(
            entries,
            vec![
                std::path::PathBuf::from("/opt/homebrew/bin"),
                std::path::PathBuf::from("/usr/local/bin"),
            ]
        );
    }

    #[test]
    fn ccusage_enriched_path_with_preserves_entries_after_join_and_split() {
        let home = std::path::PathBuf::from("/tmp/pulseusage-home");
        let existing = std::env::join_paths([
            std::path::PathBuf::from("/usr/bin"),
            std::path::PathBuf::from("/bin"),
        ])
        .expect("join existing path");

        let enriched = ccusage_enriched_path_with(Some(home.as_path()), Some(existing.as_os_str()))
            .expect("path");
        let entries: Vec<std::path::PathBuf> =
            std::env::split_paths(enriched.as_os_str()).collect();

        assert_eq!(
            entries,
            vec![
                home.join(".bun/bin"),
                home.join(".nvm/current/bin"),
                home.join(".local/bin"),
                std::path::PathBuf::from("/opt/homebrew/bin"),
                std::path::PathBuf::from("/usr/local/bin"),
                std::path::PathBuf::from("/usr/bin"),
                std::path::PathBuf::from("/bin"),
            ]
        );
    }

    #[test]
    fn nvm_default_bin_path_resolves_version_with_v_prefix() {
        let home = std::env::temp_dir().join("pulseusage-test-nvm-v-prefix");
        let alias_dir = home.join(".nvm/alias");
        std::fs::create_dir_all(&alias_dir).expect("create alias dir");
        std::fs::write(alias_dir.join("default"), "v22.16.0").expect("write alias");
        let result = nvm_default_bin_path(&home);
        let _ = std::fs::remove_dir_all(&home);
        assert_eq!(result, Some(home.join(".nvm/versions/node/v22.16.0/bin")));
    }

    #[test]
    fn nvm_default_bin_path_resolves_version_without_v_prefix() {
        let home = std::env::temp_dir().join("pulseusage-test-nvm-no-v-prefix");
        let alias_dir = home.join(".nvm/alias");
        std::fs::create_dir_all(&alias_dir).expect("create alias dir");
        std::fs::write(alias_dir.join("default"), "22.16.0").expect("write alias");
        let result = nvm_default_bin_path(&home);
        let _ = std::fs::remove_dir_all(&home);
        assert_eq!(result, Some(home.join(".nvm/versions/node/v22.16.0/bin")));
    }

    #[test]
    fn nvm_default_bin_path_returns_none_when_alias_missing() {
        let home = std::env::temp_dir().join("pulseusage-test-nvm-no-alias");
        let _ = std::fs::remove_dir_all(&home);
        let result = nvm_default_bin_path(&home);
        assert_eq!(result, None);
    }

    #[test]
    fn ccusage_path_entries_with_includes_nvm_default_version() {
        let home = std::env::temp_dir().join("pulseusage-test-nvm-entries");
        let alias_dir = home.join(".nvm/alias");
        std::fs::create_dir_all(&alias_dir).expect("create alias dir");
        std::fs::write(alias_dir.join("default"), "22.16.0").expect("write alias");
        let entries = ccusage_path_entries_with(Some(&home), None);
        let _ = std::fs::remove_dir_all(&home);
        assert!(
            entries.contains(&home.join(".nvm/versions/node/v22.16.0/bin")),
            "expected nvm default version bin in entries"
        );
    }

    #[test]
    fn configure_ccusage_command_sets_path_override() {
        let mut command = std::process::Command::new("echo");
        let args = vec!["daily".to_string(), "--json".to_string()];
        let path = std::env::join_paths([
            std::path::PathBuf::from("/tmp/bin"),
            std::path::PathBuf::from("/usr/bin"),
        ])
        .expect("join path override");

        configure_ccusage_command(&mut command, &args, Some(path.as_os_str()));

        let configured_args: Vec<String> = command
            .get_args()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect();
        assert_eq!(configured_args, args);

        let configured_path = command
            .get_envs()
            .find(|(key, _)| *key == std::ffi::OsStr::new("PATH"))
            .and_then(|(_, value)| value.map(std::borrow::ToOwned::to_owned));
        assert_eq!(configured_path.as_deref(), Some(path.as_os_str()));
    }

    #[test]
    fn configure_ccusage_command_skips_path_override_when_absent() {
        let mut command = std::process::Command::new("echo");
        let args = vec!["daily".to_string()];

        configure_ccusage_command(&mut command, &args, None);

        let has_path_override = command
            .get_envs()
            .any(|(key, _)| key == std::ffi::OsStr::new("PATH"));
        assert!(
            !has_path_override,
            "PATH should only be set when an override exists"
        );
    }

    #[test]
    fn resolve_ccusage_provider_prefers_explicit_opt_then_plugin_id() {
        let opts_explicit = CcusageQueryOpts {
            provider: Some("codex".to_string()),
            since: None,
            until: None,
            home_path: None,
            claude_path: None,
        };
        assert_eq!(
            resolve_ccusage_provider(&opts_explicit, "claude"),
            CcusageProvider::Codex
        );

        let opts_empty = CcusageQueryOpts::default();
        assert_eq!(
            resolve_ccusage_provider(&opts_empty, "codex"),
            CcusageProvider::Codex
        );
        assert_eq!(
            resolve_ccusage_provider(&opts_empty, "claude"),
            CcusageProvider::Claude
        );
        assert_eq!(
            resolve_ccusage_provider(&opts_empty, "unknown-provider"),
            CcusageProvider::Claude
        );
    }

    #[test]
    fn ccusage_home_override_supports_home_path_and_claude_compat() {
        let with_home = CcusageQueryOpts {
            provider: None,
            since: None,
            until: None,
            home_path: Some("/tmp/shared-home".to_string()),
            claude_path: Some("/tmp/claude-home".to_string()),
        };
        assert_eq!(
            ccusage_home_override(&with_home, CcusageProvider::Claude),
            Some("/tmp/shared-home")
        );
        assert_eq!(
            ccusage_home_override(&with_home, CcusageProvider::Codex),
            Some("/tmp/shared-home")
        );

        let claude_compat = CcusageQueryOpts {
            provider: None,
            since: None,
            until: None,
            home_path: None,
            claude_path: Some("/tmp/legacy-claude-path".to_string()),
        };
        assert_eq!(
            ccusage_home_override(&claude_compat, CcusageProvider::Claude),
            Some("/tmp/legacy-claude-path")
        );
        assert_eq!(
            ccusage_home_override(&claude_compat, CcusageProvider::Codex),
            None
        );
    }

    #[test]
    fn normalize_ccusage_output_converts_empty_array_to_daily_object() {
        let normalized = normalize_ccusage_output("noise\n[]\n").expect("normalized output");
        let value: serde_json::Value = serde_json::from_str(&normalized).expect("valid json");
        assert_eq!(value, serde_json::json!({ "daily": [] }));
    }

    #[test]
    fn normalize_ccusage_output_keeps_daily_object_shape() {
        let output = r#"
Saved lockfile
{
  "daily": [
    { "date": "2026-02-21", "totalTokens": 123, "totalCost": 0.5 }
  ],
  "totals": { "totalTokens": 123 }
}
"#;
        let normalized = normalize_ccusage_output(output).expect("normalized output");
        let value: serde_json::Value = serde_json::from_str(&normalized).expect("valid json");
        assert!(value.get("daily").and_then(|v| v.as_array()).is_some());
        assert!(value.get("totals").is_some());
    }

    #[test]
    fn normalize_ccusage_output_rejects_invalid_payloads() {
        assert!(normalize_ccusage_output("not-json").is_none());
        assert!(normalize_ccusage_output(r#"{"totals":{"totalTokens":1}}"#).is_none());
    }

    #[test]
    fn collect_ccusage_runners_uses_fallback_order() {
        let runners = collect_ccusage_runners_with(|kind| match kind {
            CcusageRunnerKind::Bunx => None,
            CcusageRunnerKind::PnpmDlx => Some("pnpm".to_string()),
            CcusageRunnerKind::YarnDlx => Some("yarn".to_string()),
            CcusageRunnerKind::NpmExec => Some("npm".to_string()),
            CcusageRunnerKind::Npx => Some("npx".to_string()),
        });
        assert_eq!(
            runners,
            vec![
                (CcusageRunnerKind::PnpmDlx, "pnpm".to_string()),
                (CcusageRunnerKind::YarnDlx, "yarn".to_string()),
                (CcusageRunnerKind::NpmExec, "npm".to_string()),
                (CcusageRunnerKind::Npx, "npx".to_string()),
            ]
        );
    }

    #[test]
    fn collect_ccusage_runners_returns_empty_when_none_available() {
        let runners = collect_ccusage_runners_with(|_| None);
        assert!(runners.is_empty());
    }

    #[test]
    fn ccusage_query_guard_blocks_overlapping_provider_query() {
        let first = CcusageQueryGuard::acquire(CcusageProvider::Codex)
            .expect("first query should acquire guard");
        assert!(
            CcusageQueryGuard::acquire(CcusageProvider::Codex).is_none(),
            "second query for same provider should be blocked"
        );
        assert!(
            CcusageQueryGuard::acquire(CcusageProvider::Claude).is_some(),
            "different provider should have its own guard"
        );
        drop(first);
        assert!(
            CcusageQueryGuard::acquire(CcusageProvider::Codex).is_some(),
            "guard should release on drop"
        );
    }

    #[test]
    fn ccusage_timeout_stops_runner_fallback() {
        let opts = CcusageQueryOpts::default();
        let runners = vec![
            (CcusageRunnerKind::Bunx, "bunx".to_string()),
            (CcusageRunnerKind::Npx, "npx".to_string()),
        ];
        let mut calls = Vec::new();

        let result = run_ccusage_query_with_runners(
            runners,
            &opts,
            CcusageProvider::Codex,
            "codex",
            |kind, _, _, _, _| {
                calls.push(kind);
                CcusageRunnerResult::TimedOut
            },
        );

        let value: serde_json::Value = serde_json::from_str(&result).expect("valid status json");
        assert_eq!(value["status"], "runner_failed");
        assert_eq!(calls, vec![CcusageRunnerKind::Bunx]);
    }

    #[cfg(unix)]
    #[test]
    fn ccusage_runner_retries_legacy_package_when_current_package_fails() {
        use std::io::Write;
        use std::os::unix::fs::PermissionsExt;

        let test_id = format!(
            "pulseusage-ccusage-legacy-fallback-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time")
                .as_nanos()
        );
        let dir = std::env::temp_dir().join(test_id);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        let script_path = dir.join("fake-bunx.sh");
        let args_path = dir.join("args.log");

        let mut script = std::fs::File::create(&script_path).expect("create script");
        let script_body = format!(
            r#"#!/bin/sh
echo "$*" >> "{}"
case "$*" in
  *"@ccusage/codex@18.0.11"*)
    printf '{{"daily":[]}}\n'
    exit 0
    ;;
  *)
    echo "blocked current package" >&2
    exit 1
    ;;
esac
"#,
            args_path.display()
        );
        script
            .write_all(script_body.as_bytes())
            .expect("write script");
        let mut permissions = script.metadata().expect("script metadata").permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&script_path, permissions).expect("make script executable");

        let opts = CcusageQueryOpts {
            provider: Some("codex".to_string()),
            since: Some("20260101".to_string()),
            until: None,
            home_path: None,
            claude_path: None,
        };
        let result = run_ccusage_with_runner(
            CcusageRunnerKind::Bunx,
            script_path.to_string_lossy().as_ref(),
            &opts,
            CcusageProvider::Codex,
            "codex",
        );
        assert_eq!(
            result,
            CcusageRunnerResult::Success(r#"{"daily":[]}"#.to_string())
        );

        let calls = std::fs::read_to_string(&args_path).expect("read args log");
        assert!(calls.contains("ccusage@20.0.2 codex daily"));
        assert!(calls.contains("@ccusage/codex@18.0.11 daily"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ccusage_timeout_log_uses_actual_timeout() {
        assert_eq!(
            format_ccusage_timeout(std::time::Duration::from_millis(100)),
            "100ms"
        );
        assert_eq!(
            format_ccusage_timeout(std::time::Duration::from_secs(CCUSAGE_TIMEOUT_SECS)),
            "15s"
        );
    }

    #[test]
    fn probe_deadline_clamps_host_timeout_to_remaining_budget() {
        let deadline = crate::plugin_engine::shared::ProbeDeadline::at(
            Instant::now() + Duration::from_millis(25),
        );
        let clamped = deadline
            .clamp_duration(Duration::from_secs(10))
            .expect("remaining budget should produce a host timeout");

        assert!(
            clamped <= Duration::from_millis(25),
            "host timeout should not exceed remaining probe budget"
        );
        assert!(
            clamped >= Duration::from_millis(1),
            "host timeout should stay non-zero for blocking clients"
        );
    }

    #[test]
    fn probe_deadline_does_not_extend_elapsed_budget() {
        let deadline = crate::plugin_engine::shared::ProbeDeadline::at(Instant::now());

        assert_eq!(deadline.clamp_duration(Duration::from_secs(10)), None);
    }

    #[cfg(unix)]
    #[test]
    fn ccusage_timeout_kills_descendant_and_closes_pipes() {
        use std::io::Write;
        use std::os::unix::fs::PermissionsExt;
        use std::path::Path;
        use std::time::Duration;
        #[cfg(test)]
        use std::time::Instant;

        fn pid_exists(pid: i32) -> bool {
            unsafe { libc::kill(pid, 0) == 0 }
        }

        fn read_pid_file(path: &Path, deadline: Instant) -> i32 {
            loop {
                if let Ok(pid_text) = std::fs::read_to_string(path) {
                    let pid_text = pid_text.trim();
                    if !pid_text.is_empty() {
                        return pid_text.parse().expect("parse descendant pid");
                    }
                }
                if Instant::now() >= deadline {
                    panic!("descendant pid file was not created at {}", path.display());
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        }

        let test_id = format!(
            "pulseusage-ccusage-timeout-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time")
                .as_nanos()
        );
        let dir = std::env::temp_dir().join(test_id);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        let script_path = dir.join("fake-ccusage-runner.sh");
        let pid_path = dir.join("descendant.pid");

        let mut script = std::fs::File::create(&script_path).expect("create script");
        let script_body = format!(
            r#"#!/bin/sh
sh -c 'sleep 30' &
echo $! > "{}"
echo "started"
wait
"#,
            pid_path.display()
        );
        script
            .write_all(script_body.as_bytes())
            .expect("write script");
        let mut permissions = script.metadata().expect("script metadata").permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&script_path, permissions).expect("make script executable");

        let start = Instant::now();
        let runner_path = script_path.to_string_lossy().to_string();
        let runner = std::thread::spawn(move || {
            let opts = CcusageQueryOpts::default();
            run_ccusage_with_runner_timeout(
                CcusageRunnerKind::Bunx,
                &runner_path,
                &opts,
                CcusageProvider::Codex,
                "codex",
                CcusageCommandFlavor::Current,
                Duration::from_secs(3),
            )
        });

        let descendant_pid = read_pid_file(&pid_path, Instant::now() + Duration::from_secs(2));
        let result = runner.join().expect("ccusage runner thread should finish");

        assert_eq!(result, CcusageRunnerResult::TimedOut);
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "timeout cleanup should not hang on inherited stdout/stderr pipes"
        );

        let deadline = Instant::now() + Duration::from_secs(2);
        while pid_exists(descendant_pid) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(
            !pid_exists(descendant_pid),
            "descendant process should be killed with ccusage process group"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
