use crate::plugin_engine::diagnostics::ProbeDiagnosticsRecorder;
use rquickjs::{Ctx, Exception, Function, Object};

/// Inject the `ctx.host.sqlite` API.
///
/// `query` (read-only) is available to all plugins.
/// `exec` (write) is gated to `cursor` only (see SQLITE_WRITE_ALLOWED).
///
/// Public JS API: `ctx.host.sqlite.query(dbPath, sql)` and
/// `ctx.host.sqlite.exec(dbPath, sql)` (cursor only).
pub(crate) fn inject_sqlite<'js>(
    ctx: &Ctx<'js>,
    host: &Object<'js>,
    plugin_id: &str,
    diagnostics_recorder: ProbeDiagnosticsRecorder,
) -> rquickjs::Result<()> {
    let sqlite_obj = Object::new(ctx.clone())?;

    sqlite_obj.set(
        "query",
        Function::new(
            ctx.clone(),
            move |ctx_inner: Ctx<'_>, db_path: String, sql: String| -> rquickjs::Result<String> {
                if sql.lines().any(|line| line.trim_start().starts_with('.')) {
                    diagnostics_recorder.record_local_read(false);
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        "sqlite3 dot-commands are not allowed",
                    ));
                }
                let expanded = crate::plugin_engine::shared::expand_path(&db_path);

                // Prefer a normal read-only open so WAL contents are visible (common for app state DBs).
                // Fall back to immutable=1 to bypass WAL/SHM lock issues after macOS sleep.
                let primary = std::process::Command::new("sqlite3")
                    .args(["-readonly", "-json", &expanded, &sql])
                    .output()
                    .map_err(|e| {
                        diagnostics_recorder.record_local_read(false);
                        Exception::throw_message(&ctx_inner, &format!("sqlite3 exec failed: {}", e))
                    })?;

                if primary.status.success() {
                    diagnostics_recorder.record_local_read(true);
                    return Ok(String::from_utf8_lossy(&primary.stdout).to_string());
                }

                // Percent-encode special chars for valid URI (% must be first!)
                let encoded = expanded
                    .replace('%', "%25")
                    .replace(' ', "%20")
                    .replace('#', "%23")
                    .replace('?', "%3F");
                let uri_path = format!("file:{}?immutable=1", encoded);
                let fallback = std::process::Command::new("sqlite3")
                    .args(["-readonly", "-json", &uri_path, &sql])
                    .output()
                    .map_err(|e| {
                        diagnostics_recorder.record_local_read(false);
                        Exception::throw_message(&ctx_inner, &format!("sqlite3 exec failed: {}", e))
                    })?;

                if !fallback.status.success() {
                    diagnostics_recorder.record_local_read(false);
                    let stderr_primary = String::from_utf8_lossy(&primary.stderr);
                    let stderr_fallback = String::from_utf8_lossy(&fallback.stderr);
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        &format!(
                            "sqlite3 error: {} (fallback: {})",
                            stderr_primary.trim(),
                            stderr_fallback.trim()
                        ),
                    ));
                }

                diagnostics_recorder.record_local_read(true);
                Ok(String::from_utf8_lossy(&fallback.stdout).to_string())
            },
        )?,
    )?;

    // sqlite.exec (write capability) is gated to plugins that need it.
    // Only cursor writes to its state DB; all others get read-only query.
    const SQLITE_WRITE_ALLOWED: &[&str] = &["cursor"];
    if SQLITE_WRITE_ALLOWED.contains(&plugin_id) {
        sqlite_obj.set(
            "exec",
            Function::new(
                ctx.clone(),
                move |ctx_inner: Ctx<'_>, db_path: String, sql: String| -> rquickjs::Result<()> {
                    if sql.lines().any(|line| line.trim_start().starts_with('.')) {
                        return Err(Exception::throw_message(
                            &ctx_inner,
                            "sqlite3 dot-commands are not allowed",
                        ));
                    }
                    let expanded = crate::plugin_engine::shared::expand_path(&db_path);
                    let output = std::process::Command::new("sqlite3")
                        .args([&expanded, &sql])
                        .output()
                        .map_err(|e| {
                            Exception::throw_message(
                                &ctx_inner,
                                &format!("sqlite3 exec failed: {}", e),
                            )
                        })?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(Exception::throw_message(
                            &ctx_inner,
                            &format!("sqlite3 error: {}", stderr.trim()),
                        ));
                    }

                    Ok(())
                },
            )?,
        )?;
    }

    host.set("sqlite", sqlite_obj)?;
    Ok(())
}
