use crate::plugin_engine::diagnostics::ProbeDiagnosticsRecorder;
use rquickjs::{Ctx, Exception, Function, Object};

pub(crate) fn inject_fs<'js>(
    ctx: &Ctx<'js>,
    host: &Object<'js>,
    diagnostics_recorder: ProbeDiagnosticsRecorder,
) -> rquickjs::Result<()> {
    let fs_obj = Object::new(ctx.clone())?;

    let read_text_recorder = diagnostics_recorder.clone();
    fs_obj.set(
        "exists",
        Function::new(ctx.clone(), move |path: String| -> bool {
            let expanded = crate::plugin_engine::shared::expand_path(&path);
            std::path::Path::new(&expanded).exists()
        })?,
    )?;

    fs_obj.set(
        "readText",
        Function::new(
            ctx.clone(),
            move |ctx_inner: Ctx<'_>, path: String| -> rquickjs::Result<String> {
                let expanded = crate::plugin_engine::shared::expand_path(&path);
                match std::fs::read_to_string(&expanded) {
                    Ok(value) => {
                        read_text_recorder.record_local_read(true);
                        Ok(value)
                    }
                    Err(e) => {
                        read_text_recorder.record_local_read(false);
                        Err(Exception::throw_message(&ctx_inner, &e.to_string()))
                    }
                }
            },
        )?,
    )?;

    fs_obj.set(
        "writeText",
        Function::new(
            ctx.clone(),
            move |ctx_inner: Ctx<'_>, path: String, content: String| -> rquickjs::Result<()> {
                let expanded = crate::plugin_engine::shared::expand_path(&path);
                std::fs::write(&expanded, &content)
                    .map_err(|e| Exception::throw_message(&ctx_inner, &e.to_string()))
            },
        )?,
    )?;

    let list_dir_recorder = diagnostics_recorder;
    fs_obj.set(
        "listDir",
        Function::new(
            ctx.clone(),
            move |ctx_inner: Ctx<'_>, path: String| -> rquickjs::Result<Vec<String>> {
                let expanded = crate::plugin_engine::shared::expand_path(&path);
                let entries = match std::fs::read_dir(&expanded) {
                    Ok(entries) => {
                        list_dir_recorder.record_local_read(true);
                        entries
                    }
                    Err(e) => {
                        list_dir_recorder.record_local_read(false);
                        return Err(Exception::throw_message(&ctx_inner, &e.to_string()));
                    }
                };

                let mut names = Vec::new();
                for entry in entries {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(_) => continue,
                    };
                    let name_os = entry.file_name();
                    let name = name_os.to_string_lossy().to_string();
                    if !name.is_empty() {
                        names.push(name);
                    }
                }
                names.sort();
                Ok(names)
            },
        )?,
    )?;

    host.set("fs", fs_obj)?;
    Ok(())
}
