use crate::config::base_url::normalize_base_url;
use crate::errors::Error;
use camino::{Utf8Path, Utf8PathBuf};
use serde_json::Value;
use tempfile::TempDir;

/// We flip the single `strict` flag, matching the original
/// `typescript-strict-plugin`. tsgo unfurls it into the standard strict
/// bundle (`strictNullChecks`, `noImplicitAny`, `strictFunctionTypes`, …);
/// no additional opt-ins like `noUncheckedIndexedAccess` or
/// `exactOptionalPropertyTypes` are forced on.
pub const STRICT_FAMILY_FLAGS: &[&str] = &["strict"];

pub struct TempConfig {
    pub path: Utf8PathBuf,
    /// The `run-XXX` temp directory. Wrapped in `Option` so `Drop` can take it.
    _dir: Option<TempDir>,
    /// The `.tsgo-strict-tmp` parent — removed if empty after `_dir` is dropped.
    parent_dir: std::path::PathBuf,
}

impl Drop for TempConfig {
    fn drop(&mut self) {
        // Drop the run-XXX directory first.
        drop(self._dir.take());
        // Remove the .tsgo-strict-tmp parent if it's now empty (no concurrent runs).
        let _ = std::fs::remove_dir(&self.parent_dir);
    }
}

pub fn write_temp_config(
    project_path: &Utf8Path,
    raw_config: &serde_json::Value,
    files: &[Utf8PathBuf],
    effective_base_url: Option<&Utf8PathBuf>,
    effective_compiler_options: Option<&serde_json::Map<String, Value>>,
    auto_type_directives: Option<&[String]>,
) -> Result<TempConfig, Error> {
    let project_dir = project_path.parent().unwrap_or(Utf8Path::new("."));
    let parent = project_dir.as_std_path().join(".tsgo-strict-tmp");
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

    // Use absolute paths in the files array. Relative paths cause tsgo to
    // emit diagnostic paths that are difficult to map back to the original
    // file paths, especially when the temp config lives in a nested temp
    // directory.
    let absolute_files: Vec<Value> = files.iter().map(|f| Value::String(f.to_string())).collect();

    let root = if let Some(base_url_dir) = effective_base_url {
        // baseUrl detected in the extends chain — inline all compilerOptions
        // without `extends` to prevent TS5102 from tsgo.
        let mut compiler_options = effective_compiler_options.cloned().unwrap_or_default();

        // Apply strict overrides
        compiler_options.insert("noEmit".to_string(), Value::Bool(true));
        for flag in STRICT_FAMILY_FLAGS {
            compiler_options.insert(flag.to_string(), Value::Bool(true));
        }

        // Normalize baseUrl: remove it and rewrite paths/typeRoots
        normalize_base_url(&mut compiler_options, base_url_dir);

        // Inject auto-discovered type directives when types is not explicit.
        if let Some(types) = auto_type_directives {
            compiler_options.insert(
                "types".to_string(),
                Value::Array(types.iter().map(|t| Value::String(t.clone())).collect()),
            );
        }

        let mut root = serde_json::Map::new();
        root.insert(
            "compilerOptions".to_string(),
            Value::Object(compiler_options),
        );
        root.insert("files".to_string(), Value::Array(absolute_files));
        root
    } else {
        // No baseUrl — use `extends` as before
        let mut compiler_options = raw_config
            .get("compilerOptions")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        compiler_options.insert("noEmit".to_string(), Value::Bool(true));
        for flag in STRICT_FAMILY_FLAGS {
            compiler_options.insert(flag.to_string(), Value::Bool(true));
        }

        // Inject auto-discovered type directives when types is not explicit.
        if let Some(types) = auto_type_directives {
            compiler_options.insert(
                "types".to_string(),
                Value::Array(types.iter().map(|t| Value::String(t.clone())).collect()),
            );
        }

        let mut root = serde_json::Map::new();
        root.insert(
            "extends".to_string(),
            Value::String(project_path.to_string()),
        );
        root.insert(
            "compilerOptions".to_string(),
            Value::Object(compiler_options),
        );
        root.insert("files".to_string(), Value::Array(absolute_files));
        root
    };

    let body = serde_json::to_string_pretty(&Value::Object(root))
        .map_err(|e| Error::msg(format!("failed to serialize temp tsconfig: {}", e)))?;
    std::fs::write(&config_path, format!("{body}\n"))
        .map_err(|e| Error::msg(format!("cannot write {}: {}", config_path.display(), e)))?;

    Ok(TempConfig {
        path: Utf8PathBuf::try_from(config_path).unwrap(),
        _dir: Some(dir),
        parent_dir: parent,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_config_lives_under_project_dir() {
        let project_root = tempfile::tempdir().unwrap();
        let project_dir = Utf8Path::from_path(project_root.path()).unwrap();
        let tsconfig_path = project_dir.join("tsconfig.json");
        std::fs::write(&tsconfig_path, "{}").unwrap();

        let raw_config = serde_json::json!({
            "compilerOptions": { "target": "ES2020" }
        });
        let files = vec![project_dir.join("src/a.ts"), project_dir.join("src/b.ts")];

        let temp = write_temp_config(
            tsconfig_path.as_ref(),
            &raw_config,
            &files,
            None,
            None,
            None,
        )
        .unwrap();

        // Config should be under <project_dir>/.tsgo-strict-tmp/run-
        assert!(
            temp.path.starts_with(project_dir),
            "temp config should be under the project dir: {}",
            temp.path
        );
        assert!(
            temp.path.as_str().contains(".tsgo-strict-tmp/run-"),
            "expected path to contain .tsgo-strict-tmp/run-: {}",
            temp.path
        );

        // Read back and verify structure
        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&temp.path).unwrap()).unwrap();

        assert_eq!(content["extends"], tsconfig_path.as_str());
        assert_eq!(content["compilerOptions"]["strict"], true);
        assert_eq!(content["compilerOptions"]["noEmit"], true);
        assert_eq!(content["compilerOptions"]["target"], "ES2020");
        assert_eq!(content["files"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn temp_config_inlines_when_base_url_present() {
        let project_root = tempfile::tempdir().unwrap();
        let project_dir = Utf8Path::from_path(project_root.path()).unwrap();
        let tsconfig_path = project_dir.join("tsconfig.json");
        std::fs::write(&tsconfig_path, "{}").unwrap();

        let raw_config = serde_json::json!({
            "compilerOptions": { "target": "ES2020", "baseUrl": ".", "paths": { "@app/*": ["src/app/*"] } }
        });

        let mut effective_co = serde_json::Map::new();
        effective_co.insert("target".to_string(), serde_json::json!("ES2020"));
        effective_co.insert("baseUrl".to_string(), serde_json::json!("."));
        effective_co.insert(
            "paths".to_string(),
            serde_json::json!({ "@app/*": ["src/app/*"] }),
        );

        let base_url_dir = project_dir.to_owned();
        let files = vec![project_dir.join("src/a.ts")];

        let temp = write_temp_config(
            tsconfig_path.as_ref(),
            &raw_config,
            &files,
            Some(&base_url_dir),
            Some(&effective_co),
            None,
        )
        .unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&temp.path).unwrap()).unwrap();

        // Should NOT have extends
        assert!(
            content.get("extends").is_none(),
            "temp config should not use extends when baseUrl is present"
        );
        // baseUrl should be removed
        assert!(
            content["compilerOptions"].get("baseUrl").is_none(),
            "baseUrl should be stripped"
        );
        // paths should be rewritten to absolute
        let paths_app = &content["compilerOptions"]["paths"]["@app/*"];
        let first_path = paths_app[0].as_str().unwrap();
        assert!(
            first_path.ends_with("/src/app/*"),
            "paths should end with /src/app/*: {first_path}"
        );
        // strict flags present
        assert_eq!(content["compilerOptions"]["strict"], true);
        assert_eq!(content["compilerOptions"]["noEmit"], true);
    }

    #[test]
    fn drop_cleans_up_parent_dir() {
        let project_root = tempfile::tempdir().unwrap();
        let project_dir = Utf8Path::from_path(project_root.path()).unwrap();
        let tsconfig_path = project_dir.join("tsconfig.json");
        std::fs::write(&tsconfig_path, "{}").unwrap();

        let raw_config = serde_json::json!({
            "compilerOptions": { "target": "ES2020" }
        });
        let files = vec![project_dir.join("src/a.ts")];

        let temp = write_temp_config(
            tsconfig_path.as_ref(),
            &raw_config,
            &files,
            None,
            None,
            None,
        )
        .unwrap();

        let parent = project_dir.join(".tsgo-strict-tmp");
        assert!(
            parent.exists(),
            ".tsgo-strict-tmp should exist while temp config is alive"
        );

        // Drop the temp config — should clean up both run-XXX and .tsgo-strict-tmp
        drop(temp);

        assert!(
            !parent.exists(),
            ".tsgo-strict-tmp should be removed after drop"
        );
    }
}
