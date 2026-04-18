use crate::errors::Error;
use camino::{Utf8Path, Utf8PathBuf};
use tempfile::TempDir;

/// We flip the single `strict` flag, matching the original
/// `typescript-strict-plugin`. tsgo unfurls it into the standard strict
/// bundle (`strictNullChecks`, `noImplicitAny`, `strictFunctionTypes`, …);
/// no additional opt-ins like `noUncheckedIndexedAccess` or
/// `exactOptionalPropertyTypes` are forced on.
pub const STRICT_FAMILY_FLAGS: &[&str] = &["strict"];

pub struct TempConfig {
    pub path: Utf8PathBuf,
    pub _dir: TempDir,
}

pub fn write_temp_config(
    cwd: &Utf8PathBuf,
    project_path: &Utf8Path,
    raw_config: &serde_json::Value,
    files: &[Utf8PathBuf],
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

    let config_path = dir.path().join("strict.json");

    let mut compiler_options = raw_config
        .get("compilerOptions")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();

    compiler_options.insert("noEmit".to_string(), serde_json::Value::Bool(true));
    for flag in STRICT_FAMILY_FLAGS {
        compiler_options.insert(flag.to_string(), serde_json::Value::Bool(true));
    }

    let relative_files: Vec<serde_json::Value> = files
        .iter()
        .map(|f| {
            // `diff_paths` returns `Some("")` when the file path equals the
            // temp-dir path; tsgo would reject an empty `files[]` entry, so
            // fall back to the absolute path in that edge case.
            let rel = pathdiff::diff_paths(f.as_std_path(), dir.path())
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| f.to_string());
            serde_json::Value::String(rel)
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
