use crate::plugin_engine::diagnostics::ProbeDiagnosticsRecorder;
use crate::plugin_engine::shared::{probe_timeout_error, ProbeDeadline};
use rquickjs::{Ctx, Exception, Function, Object};
use std::time::Duration;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HttpReqParams {
    url: String,
    method: Option<String>,
    headers: Option<std::collections::HashMap<String, String>>,
    body_text: Option<String>,
    timeout_ms: Option<u64>,
    dangerously_ignore_tls: Option<bool>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct HttpRespParams {
    status: u16,
    headers: std::collections::HashMap<String, String>,
    body_text: String,
}

/// Inject the `ctx.host.http` API.
///
/// Public JS API: `ctx.host.http.request(req)` (wrapped from `_requestRaw`
/// by `patch_http_wrapper`).
pub(crate) fn inject_http<'js>(
    ctx: &Ctx<'js>,
    host: &Object<'js>,
    plugin_id: &str,
    deadline: ProbeDeadline,
    diagnostics_recorder: ProbeDiagnosticsRecorder,
) -> rquickjs::Result<()> {
    let http_obj = Object::new(ctx.clone())?;
    let pid = plugin_id.to_string();

    http_obj.set(
        "_requestRaw",
        Function::new(
            ctx.clone(),
            move |ctx_inner: Ctx<'_>, req_json: String| -> rquickjs::Result<String> {
                let req: HttpReqParams = serde_json::from_str(&req_json).map_err(|e| {
                    Exception::throw_message(&ctx_inner, &format!("invalid request: {}", e))
                })?;

                if deadline.has_elapsed() {
                    return Err(Exception::throw_message(&ctx_inner, "probe timed out"));
                }

                let method_str = req.method.as_deref().unwrap_or("GET");
                let redacted_url = crate::plugin_engine::redaction::redact_url(&req.url);
                log::info!("[plugin:{}] HTTP {} {}", pid, method_str, redacted_url);
                diagnostics_recorder.record_http_attempt();

                let mut header_map = reqwest::header::HeaderMap::new();
                if let Some(headers) = &req.headers {
                    for (key, val) in headers {
                        let name = reqwest::header::HeaderName::from_bytes(key.as_bytes())
                            .map_err(|e| {
                                Exception::throw_message(
                                    &ctx_inner,
                                    &format!("invalid header name '{}': {}", key, e),
                                )
                            })?;
                        let value = reqwest::header::HeaderValue::from_str(val).map_err(|e| {
                            Exception::throw_message(
                                &ctx_inner,
                                &format!("invalid header value for '{}': {}", key, e),
                            )
                        })?;
                        header_map.insert(name, value);
                    }
                }

                let timeout_ms = req.timeout_ms.unwrap_or(10_000);
                let Some(timeout) = deadline.clamp_duration(Duration::from_millis(timeout_ms))
                else {
                    return Err(probe_timeout_error(&ctx_inner));
                };
                let mut builder = reqwest::blocking::Client::builder()
                    .timeout(timeout)
                    .connect_timeout(timeout)
                    .redirect(reqwest::redirect::Policy::none());

                // Apply pre-resolved proxy (localhost bypass already configured)
                if let Some(resolved) = crate::config::get_resolved_proxy() {
                    builder = builder.proxy(resolved.proxy.clone());
                    log::debug!("[http] proxy active");
                } else {
                    log::debug!("[http] proxy not used");
                }

                if req.dangerously_ignore_tls.unwrap_or(false) {
                    let url = &req.url;
                    let is_localhost = url
                        .strip_prefix("https://")
                        .or_else(|| url.strip_prefix("http://"))
                        .and_then(|rest| rest.split(|c| c == '/' || c == ':').next())
                        .map(|host| host == "127.0.0.1" || host == "localhost" || host == "::1")
                        .unwrap_or(false);
                    if !is_localhost {
                        return Err(Exception::throw_message(
                            &ctx_inner,
                            "dangerouslyIgnoreTls is only permitted for localhost (127.0.0.1, localhost, ::1)",
                        ));
                    }
                    builder = builder.danger_accept_invalid_certs(true);
                }
                let client = builder
                    .build()
                    .map_err(|e| Exception::throw_message(&ctx_inner, &e.to_string()))?;

                let method = req.method.as_deref().unwrap_or("GET");
                let method = reqwest::Method::from_bytes(method.as_bytes()).map_err(|e| {
                    Exception::throw_message(
                        &ctx_inner,
                        &format!("invalid http method '{}': {}", method, e),
                    )
                })?;
                let mut builder = client.request(method, &req.url);
                builder = builder.headers(header_map);
                if let Some(body) = req.body_text {
                    builder = builder.body(body);
                }

                let response = builder
                    .send()
                    .map_err(|e| Exception::throw_message(&ctx_inner, &e.to_string()))?;

                let status = response.status().as_u16();
                diagnostics_recorder.record_http_status(status);
                let mut resp_headers = std::collections::HashMap::new();
                for (key, value) in response.headers().iter() {
                    let header_value = value.to_str().map_err(|e| {
                        Exception::throw_message(
                            &ctx_inner,
                            &format!("invalid response header '{}': {}", key, e),
                        )
                    })?;
                    resp_headers.insert(key.to_string(), header_value.to_string());
                }
                let body = response
                    .text()
                    .map_err(|e| Exception::throw_message(&ctx_inner, &e.to_string()))?;

                // Redact BEFORE truncation to ensure sensitive values are caught while intact
                let redacted_body = crate::plugin_engine::redaction::redact_body(&body);
                let body_preview = if redacted_body.len() > 500 {
                    // UTF-8 safe truncation: find valid char boundary at or before 500
                    let truncated: String = redacted_body
                        .char_indices()
                        .take_while(|(i, _)| *i < 500)
                        .map(|(_, c)| c)
                        .collect();
                    format!("{}... ({} bytes total)", truncated, body.len())
                } else {
                    redacted_body
                };
                log::info!(
                    "[plugin:{}] HTTP {} {} -> {} | {}",
                    pid,
                    method_str,
                    redacted_url,
                    status,
                    body_preview
                );

                let resp = HttpRespParams {
                    status,
                    headers: resp_headers,
                    body_text: body,
                };

                serde_json::to_string(&resp)
                    .map_err(|e| Exception::throw_message(&ctx_inner, &e.to_string()))
            },
        )?,
    )?;

    ctx.eval::<(), _>(
        r#"
        (function() {
            // Will be patched after __pulseusage_ctx is set.
            if (typeof __pulseusage_ctx !== "undefined") {
                void 0;
            }
        })();
        "#
        .as_bytes(),
    )
    .map_err(|e| Exception::throw_message(ctx, &format!("http wrapper init failed: {}", e)))?;

    host.set("http", http_obj)?;
    Ok(())
}

/// Patch the JS `ctx.host.http.request` wrapper that JSON-encodes req
/// for the Rust `_requestRaw` function.
pub fn patch_http_wrapper(ctx: &rquickjs::Ctx<'_>) -> rquickjs::Result<()> {
    ctx.eval::<(), _>(
        r#"
        (function() {
            var rawFn = __pulseusage_ctx.host.http._requestRaw;
            __pulseusage_ctx.host.http.request = function(req) {
                var json = JSON.stringify({
                    url: req.url,
                    method: req.method || "GET",
                    headers: req.headers || null,
                    bodyText: req.bodyText || null,
                    timeoutMs: req.timeoutMs || 10000,
                    dangerouslyIgnoreTls: req.dangerouslyIgnoreTls || false
                });
                var respJson = rawFn(json);
                return JSON.parse(respJson);
            };
        })();
        "#
        .as_bytes(),
    )
}
