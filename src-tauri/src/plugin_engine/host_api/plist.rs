use crate::plugin_engine::diagnostics::ProbeDiagnosticsRecorder;
use rquickjs::{Ctx, Exception, Function, Object};

pub(crate) fn inject_plist<'js>(
    ctx: &Ctx<'js>,
    host: &Object<'js>,
    diagnostics_recorder: ProbeDiagnosticsRecorder,
) -> rquickjs::Result<()> {
    let plist_obj = Object::new(ctx.clone())?;

    plist_obj.set(
        "read",
        Function::new(
            ctx.clone(),
            move |ctx_inner: Ctx<'_>, path: String| -> rquickjs::Result<String> {
                if !cfg!(target_os = "macos") {
                    diagnostics_recorder.record_local_read(false);
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        "plist API is only supported on macOS",
                    ));
                }

                let expanded = crate::plugin_engine::shared::expand_path(&path);
                let output = std::process::Command::new("plutil")
                    .args(["-convert", "json", "-o", "-", &expanded])
                    .output()
                    .map_err(|e| {
                        diagnostics_recorder.record_local_read(false);
                        Exception::throw_message(&ctx_inner, &format!("plist read failed: {}", e))
                    })?;

                if !output.status.success() {
                    diagnostics_recorder.record_local_read(false);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(Exception::throw_message(
                        &ctx_inner,
                        &format!("plist read failed: {}", stderr.trim()),
                    ));
                }

                diagnostics_recorder.record_local_read(true);
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            },
        )?,
    )?;

    host.set("plist", plist_obj)?;
    Ok(())
}
