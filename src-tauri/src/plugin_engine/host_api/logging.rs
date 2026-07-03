use rquickjs::{Ctx, Function, Object};

pub(crate) fn inject_log<'js>(
    ctx: &Ctx<'js>,
    host: &Object<'js>,
    plugin_id: &str,
) -> rquickjs::Result<()> {
    let log_obj = Object::new(ctx.clone())?;

    let pid = plugin_id.to_string();
    log_obj.set(
        "info",
        Function::new(ctx.clone(), move |msg: String| {
            log::info!(
                "[plugin:{}] {}",
                pid,
                crate::plugin_engine::redaction::redact_log_message(&msg)
            );
        })?,
    )?;

    let pid = plugin_id.to_string();
    log_obj.set(
        "warn",
        Function::new(ctx.clone(), move |msg: String| {
            log::warn!(
                "[plugin:{}] {}",
                pid,
                crate::plugin_engine::redaction::redact_log_message(&msg)
            );
        })?,
    )?;

    let pid = plugin_id.to_string();
    log_obj.set(
        "error",
        Function::new(ctx.clone(), move |msg: String| {
            log::error!(
                "[plugin:{}] {}",
                pid,
                crate::plugin_engine::redaction::redact_log_message(&msg)
            );
        })?,
    )?;

    host.set("log", log_obj)?;
    Ok(())
}
