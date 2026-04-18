use crate::errors::Error;
use camino::{Utf8Path, Utf8PathBuf};
use std::path::Path;
use tempfile::TempDir;

/// 14 flags in the "strict family" — the full set tsc/tsgo treats as strict
/// when `strict: true` is enabled, plus the four flags it does not bundle.
pub const STRICT_FAMILY_FLAGS: &[&str] = &[
    "strict",
    "strictBindCallApply",
    "strictBuiltinIteratorReturn",
    "strictFunctionTypes",
    "strictNullChecks",
    "strictPropertyInitialization",
    "useUnknownInCatchVariables",
    "noImplicitAny",
    "noImplicitThis",
    "noImplicitOverride",
    "noPropertyAccessFromIndexSignature",
    "noUncheckedIndexedAccess",
    "noUncheckedSideEffectImports",
    "exactOptionalPropertyTypes",
];

pub struct TempConfig {
    pub path: Utf8PathBuf,
    pub _dir: TempDir,
}

pub fn write_temp_config(
    cwd: &Utf8PathBuf,
    project_path: &Utf8Path,
    raw_config: &serde_json::Value,
    files: &[Utf8PathBuf],
    strict_enabled: bool,
) -> Result<TempConfig, Error> {
    let parent = cwd.as_std_path().join(".tsgo-strict-tmp");
    std::fs::create_dir_all(&parent)
        .map_err(|e| Error::msg(format!("cannot create {}: {}", parent.display(), e)))?;

    let dir = tempfile::Builder::new()
        .prefix("run-")
        .tempdir_in(&parent)
        .map_err(|e| {
            Error::msg(format!(
                "cannot create temp dir in {}: {}",
                parent.display(),
                e
            ))
        })?;

    let filename = if strict_enabled {
        "strict.json"
    } else {
        "baseline.json"
    };
    let config_path = dir.path().join(filename);

    let mut compiler_options = raw_config
        .get("compilerOptions")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();

    compiler_options.insert("noEmit".to_string(), serde_json::Value::Bool(true));
    for flag in STRICT_FAMILY_FLAGS {
        compiler_options.insert(flag.to_string(), serde_json::Value::Bool(strict_enabled));
    }

    let relative_files: Vec<serde_json::Value> = files
        .iter()
        .map(|f| {
            let rel = relative_to(f.as_std_path(), dir.path());
            serde_json::Value::String(rel.replace('\\', "/"))
        })
        .collect();

    let mut root = serde_json::Map::new();
    root.insert(
        "extends".to_string(),
        serde_json::Value::String(project_path.to_string()),
    );
    root.insert(
        "compilerOptions".to_string(),
        serde_json::Value::Object(compiler_options),
    );
    root.insert(
        "files".to_string(),
        serde_json::Value::Array(relative_files),
    );

    let body = serde_json::to_string_pretty(&serde_json::Value::Object(root))
        .map_err(|e| Error::msg(format!("failed to serialize temp tsconfig: {}", e)))?;
    std::fs::write(&config_path, format!("{body}\n"))
        .map_err(|e| Error::msg(format!("cannot write {}: {}", config_path.display(), e)))?;

    Ok(TempConfig {
        path: Utf8PathBuf::try_from(config_path).unwrap(),
        _dir: dir,
    })
}

fn relative_to(path: &Path, base: &Path) -> String {
    use std::path::Component;

    let abs = path.to_path_buf();
    let base = base.to_path_buf();
    let mut path_iter = abs.components();
    let mut base_iter = base.components();
    let mut out: Vec<String> = Vec::new();

    loop {
        match (path_iter.next(), base_iter.next()) {
            (Some(a), Some(b)) if a == b => continue,
            (Some(a), Some(_)) => {
                out.push("..".to_string());
                push_component(&mut out, a);
                for _ in base_iter.by_ref() {
                    out.insert(0, "..".to_string());
                }
                for c in path_iter.by_ref() {
                    push_component(&mut out, c);
                }
                break;
            }
            (Some(a), None) => {
                push_component(&mut out, a);
                for c in path_iter.by_ref() {
                    push_component(&mut out, c);
                }
                break;
            }
            (None, Some(_)) => {
                out.insert(0, "..".to_string());
                for _ in base_iter.by_ref() {
                    out.insert(0, "..".to_string());
                }
                break;
            }
            (None, None) => break,
        }
    }

    fn push_component(out: &mut Vec<String>, component: Component<'_>) {
        match component {
            Component::Normal(os) => out.push(os.to_string_lossy().into_owned()),
            Component::ParentDir => out.push("..".to_string()),
            Component::CurDir => {}
            Component::RootDir => out.push("/".to_string()),
            Component::Prefix(p) => out.push(p.as_os_str().to_string_lossy().into_owned()),
        }
    }

    out.join(std::path::MAIN_SEPARATOR_STR)
}
