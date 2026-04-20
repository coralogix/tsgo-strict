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
    project_path: &Utf8Path,
    raw_config: &serde_json::Value,
    files: &[Utf8PathBuf],
) -> Result<TempConfig, Error> {
    let parent = std::env::temp_dir().join("tsgo-strict");
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

    // Use absolute paths in the files array. Relative paths cause tsgo to
    // emit diagnostic paths that are difficult to map back to the original
    // file paths, especially when the temp config lives in a nested temp
    // directory.
    let absolute_files: Vec<serde_json::Value> = files
        .iter()
        .map(|f| serde_json::Value::String(f.to_string()))
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
        serde_json::Value::Array(absolute_files),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_config_lives_in_system_temp_dir() {
        let project_path = Utf8Path::new("/fake/project/tsconfig.json");
        let raw_config = serde_json::json!({
            "compilerOptions": { "target": "ES2020" }
        });
        let files = vec![
            Utf8PathBuf::from("/fake/project/src/a.ts"),
            Utf8PathBuf::from("/fake/project/src/b.ts"),
        ];

        let temp = write_temp_config(project_path, &raw_config, &files).unwrap();

        // Config is written under the system temp dir, not the project dir
        assert!(
            !temp.path.starts_with("/fake/project"),
            "temp config should not be under the project dir: {}",
            temp.path
        );
        assert!(
            temp.path.as_str().contains("tsgo-strict/run-"),
            "expected path to contain tsgo-strict/run-: {}",
            temp.path
        );

        // Read back and verify structure
        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&temp.path).unwrap()).unwrap();

        assert_eq!(content["extends"], "/fake/project/tsconfig.json");
        assert_eq!(content["compilerOptions"]["strict"], true);
        assert_eq!(content["compilerOptions"]["noEmit"], true);
        assert_eq!(content["compilerOptions"]["target"], "ES2020");
        assert_eq!(
            content["files"],
            serde_json::json!(["/fake/project/src/a.ts", "/fake/project/src/b.ts"])
        );
    }
}
