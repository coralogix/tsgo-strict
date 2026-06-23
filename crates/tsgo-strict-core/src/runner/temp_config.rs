// Copyright 2026 Coralogix Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::config::base_url::{
    normalize_base_url, rewrite_relative_paths, rewrite_relative_type_roots,
};
use crate::config::v6_compat::apply_v6_compat_shims;
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
    effective_type_roots_dir: Option<&Utf8PathBuf>,
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

        // Normalize baseUrl: remove it and rewrite paths to absolute.
        normalize_base_url(&mut compiler_options, base_url_dir);

        // `typeRoots` is anchored at the config that defined it, not baseUrl.
        // Fall back to the leaf's directory if we don't have a specific
        // anchor — in practice this means the leaf defined the typeRoots.
        let type_roots_anchor = effective_type_roots_dir
            .map(|p| p.as_path())
            .unwrap_or_else(|| project_path.parent().unwrap_or(Utf8Path::new(".")));
        rewrite_relative_type_roots(&mut compiler_options, type_roots_anchor);

        // Inject auto-discovered type directives when types is not explicit.
        if let Some(types) = auto_type_directives {
            compiler_options.insert(
                "types".to_string(),
                Value::Array(types.iter().map(|t| Value::String(t.clone())).collect()),
            );
        }

        // v5→v6 compatibility shims. Since the leaf here IS the merged view
        // of the full chain (no `extends`), use it as `effective` too.
        let effective_snapshot = compiler_options.clone();
        apply_v6_compat_shims(&mut compiler_options, &effective_snapshot);

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

        // The temp config lives at <project>/.tsgo-strict-tmp/run-XXX/, two
        // directories below the original tsconfig. Relative `paths` and
        // `typeRoots` copied from the leaf's compilerOptions would resolve
        // from that temp dir instead of the original — rewrite them to
        // absolute paths anchored at the original tsconfig's directory so
        // tsgo resolves them the same way it would from the real config.
        let leaf_dir = project_path.parent().unwrap_or(Utf8Path::new("."));
        rewrite_relative_paths(&mut compiler_options, leaf_dir);
        let type_roots_anchor = effective_type_roots_dir
            .map(|p| p.as_path())
            .unwrap_or(leaf_dir);
        rewrite_relative_type_roots(&mut compiler_options, type_roots_anchor);

        // Inject auto-discovered type directives when types is not explicit.
        if let Some(types) = auto_type_directives {
            compiler_options.insert(
                "types".to_string(),
                Value::Array(types.iter().map(|t| Value::String(t.clone())).collect()),
            );
        }

        // v5→v6 compatibility shims. Use the merged extends-chain view for
        // deciding whether the user set a key anywhere; fall back to the
        // leaf itself if no merged view was supplied.
        let effective = effective_compiler_options
            .cloned()
            .unwrap_or_else(|| compiler_options.clone());
        apply_v6_compat_shims(&mut compiler_options, &effective);

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
        assert_eq!(content["compilerOptions"]["ignoreDeprecations"], "6.0");
        assert_eq!(
            content["compilerOptions"]["noUncheckedSideEffectImports"],
            false
        );
        assert_eq!(content["compilerOptions"]["libReplacement"], true);
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
        // v6 compat shims applied
        assert_eq!(content["compilerOptions"]["ignoreDeprecations"], "6.0");
        assert_eq!(
            content["compilerOptions"]["noUncheckedSideEffectImports"],
            false
        );
        assert_eq!(content["compilerOptions"]["libReplacement"], true);
    }

    #[test]
    fn temp_config_rewrites_esmoduleinterop_false_from_effective_options() {
        let project_root = tempfile::tempdir().unwrap();
        let project_dir = Utf8Path::from_path(project_root.path()).unwrap();
        let tsconfig_path = project_dir.join("tsconfig.json");
        std::fs::write(&tsconfig_path, "{}").unwrap();

        let raw_config = serde_json::json!({ "compilerOptions": {} });

        let mut effective_co = serde_json::Map::new();
        effective_co.insert("esModuleInterop".to_string(), serde_json::json!(false));
        effective_co.insert(
            "allowSyntheticDefaultImports".to_string(),
            serde_json::json!(false),
        );
        effective_co.insert("alwaysStrict".to_string(), serde_json::json!(false));

        let files = vec![project_dir.join("src/a.ts")];

        let temp = write_temp_config(
            tsconfig_path.as_ref(),
            &raw_config,
            &files,
            None,
            Some(&effective_co),
            None,
            None,
        )
        .unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&temp.path).unwrap()).unwrap();

        // Hard-removed-false keys rewritten to true on the leaf so they win
        // over the inherited `false` via `extends`.
        assert_eq!(content["compilerOptions"]["esModuleInterop"], true);
        assert_eq!(
            content["compilerOptions"]["allowSyntheticDefaultImports"],
            true
        );
        assert_eq!(content["compilerOptions"]["alwaysStrict"], true);
    }

    #[test]
    fn type_roots_anchored_at_defining_config_when_base_url_present() {
        // Real-world shape from Nx workspaces: base config declares
        // `baseUrl: "."` at the workspace root, a nested leaf tsconfig
        // overrides `typeRoots` with paths like `../../node_modules/@types`
        // relative to *the leaf's own directory*. tsc resolves typeRoots
        // against the config that defined them (not baseUrl's dir), so the
        // temp config must anchor the rewrite at the leaf's directory — not
        // at base_url_dir — or the paths land in the wrong place.
        let project_root = tempfile::tempdir().unwrap();
        let project_dir = Utf8Path::from_path(project_root.path()).unwrap();
        // Simulate workspace/libs/testing/ structure.
        let leaf_dir = project_dir.join("libs/testing");
        std::fs::create_dir_all(&leaf_dir).unwrap();
        let tsconfig_path = leaf_dir.join("tsconfig.lib.json");
        std::fs::write(&tsconfig_path, "{}").unwrap();

        let raw_config = serde_json::json!({
            "compilerOptions": {
                "typeRoots": ["../../node_modules/@types", "../../node_modules"],
                "types": ["vitest/globals"]
            }
        });

        let mut effective_co = serde_json::Map::new();
        effective_co.insert("baseUrl".to_string(), serde_json::json!("."));
        effective_co.insert(
            "typeRoots".to_string(),
            serde_json::json!(["../../node_modules/@types", "../../node_modules"]),
        );

        // baseUrl resolves to the workspace root.
        let base_url_dir = project_dir.to_owned();
        let leaf_dir_owned = leaf_dir.clone();
        let files = vec![leaf_dir.join("src/a.ts")];

        let temp = write_temp_config(
            tsconfig_path.as_ref(),
            &raw_config,
            &files,
            Some(&base_url_dir),
            Some(&effective_co),
            Some(&leaf_dir_owned),
            None,
        )
        .unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&temp.path).unwrap()).unwrap();

        let type_roots = content["compilerOptions"]["typeRoots"].as_array().unwrap();
        let first = type_roots[0].as_str().unwrap();

        // Anchor must be the LEAF's dir, not base_url_dir. If anchored at
        // base_url_dir (= project_dir), `../../node_modules/@types` would
        // point *outside* the project tree entirely. Anchoring at leaf_dir
        // (= project_dir/libs/testing) lands inside the project at
        // `<project>/node_modules/@types`, which is what tsc would resolve.
        let wrong_anchor_prefix = project_dir.join("../../node_modules");
        assert!(
            !first.starts_with(wrong_anchor_prefix.as_str()),
            "typeRoots[0] appears anchored at base_url_dir — \
             `../../` from the project root escapes the tree: {first}"
        );
        // Canonicalize both sides to collapse `..` segments and compare as
        // filesystem paths — the string form includes the unresolved segments.
        let expected_resolved = std::path::PathBuf::from(leaf_dir.as_str())
            .join("../../node_modules/@types")
            .canonicalize()
            .ok();
        let got_resolved = std::path::PathBuf::from(first).canonicalize().ok();
        // Both paths refer to <project_dir>/node_modules/@types, which
        // doesn't exist in this test, so canonicalize() returns None for
        // both. Compare via lexical equality of the unresolved form instead.
        if expected_resolved.is_some() && got_resolved.is_some() {
            assert_eq!(got_resolved, expected_resolved);
        } else {
            // Fall back: first must *start* with leaf_dir (proving anchor).
            assert!(
                first.starts_with(leaf_dir.as_str()),
                "typeRoots[0] should start with leaf dir {leaf_dir}, got: {first}"
            );
            assert!(
                first.ends_with("/node_modules/@types"),
                "typeRoots[0] should end with /node_modules/@types, got: {first}"
            );
        }
    }

    #[test]
    fn relative_type_roots_rewritten_to_absolute_in_no_base_url_branch() {
        // The temp config lives at <project>/.tsgo-strict-tmp/run-XXX/strict.json —
        // two directories deeper than the project's own tsconfig. Relative
        // typeRoots entries copied verbatim from the leaf's compilerOptions
        // resolve against the TEMP dir rather than the project dir, so
        // `../../node_modules` ends up pointing at `<project>/node_modules`
        // (missing the workspace-root two levels up). Rewrite to absolute so
        // tsgo resolves them the same way it would from the original tsconfig.
        let project_root = tempfile::tempdir().unwrap();
        let project_dir = Utf8Path::from_path(project_root.path()).unwrap();
        let tsconfig_path = project_dir.join("tsconfig.json");
        std::fs::write(&tsconfig_path, "{}").unwrap();

        let raw_config = serde_json::json!({
            "compilerOptions": {
                "typeRoots": ["../../node_modules/@types", "../../node_modules"],
                "types": ["vitest/globals"]
            }
        });
        let files = vec![project_dir.join("src/a.ts")];

        let temp = write_temp_config(
            tsconfig_path.as_ref(),
            &raw_config,
            &files,
            None,
            None,
            None,
            None,
        )
        .unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&temp.path).unwrap()).unwrap();

        let type_roots = content["compilerOptions"]["typeRoots"].as_array().unwrap();
        let first = type_roots[0].as_str().unwrap();
        let second = type_roots[1].as_str().unwrap();

        assert!(
            Utf8Path::new(first).is_absolute(),
            "typeRoots[0] should be absolute, got: {first}"
        );
        assert!(
            Utf8Path::new(second).is_absolute(),
            "typeRoots[1] should be absolute, got: {second}"
        );

        // The absolute paths should resolve against the original tsconfig's
        // directory, not the temp config's directory — so `../../node_modules`
        // from the project dir lands two levels above the project.
        let expected_suffix = "/node_modules/@types";
        assert!(
            first.ends_with(expected_suffix),
            "typeRoots[0] should end with {expected_suffix}, got: {first}"
        );
    }

    #[test]
    fn dot_relative_type_roots_rewritten_to_absolute() {
        // `./node_modules/@types` relative to the project dir should end up
        // absolute at `<project_dir>/node_modules/@types`.
        let project_root = tempfile::tempdir().unwrap();
        let project_dir = Utf8Path::from_path(project_root.path()).unwrap();
        let tsconfig_path = project_dir.join("tsconfig.json");
        std::fs::write(&tsconfig_path, "{}").unwrap();

        let raw_config = serde_json::json!({
            "compilerOptions": {
                "typeRoots": ["./node_modules/@types"]
            }
        });
        let files = vec![project_dir.join("src/a.ts")];

        let temp = write_temp_config(
            tsconfig_path.as_ref(),
            &raw_config,
            &files,
            None,
            None,
            None,
            None,
        )
        .unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&temp.path).unwrap()).unwrap();

        let type_roots = content["compilerOptions"]["typeRoots"].as_array().unwrap();
        let entry = type_roots[0].as_str().unwrap();
        assert!(
            Utf8Path::new(entry).is_absolute(),
            "typeRoots[0] should be absolute, got: {entry}"
        );
        assert!(
            entry.contains("/node_modules/@types"),
            "typeRoots[0] should contain /node_modules/@types, got: {entry}"
        );
    }

    #[test]
    fn absolute_type_roots_left_alone() {
        let project_root = tempfile::tempdir().unwrap();
        let project_dir = Utf8Path::from_path(project_root.path()).unwrap();
        let tsconfig_path = project_dir.join("tsconfig.json");
        std::fs::write(&tsconfig_path, "{}").unwrap();

        let abs = if cfg!(windows) {
            "C:/abs/types"
        } else {
            "/abs/types"
        };

        let raw_config = serde_json::json!({
            "compilerOptions": {
                "typeRoots": [abs]
            }
        });
        let files = vec![project_dir.join("src/a.ts")];

        let temp = write_temp_config(
            tsconfig_path.as_ref(),
            &raw_config,
            &files,
            None,
            None,
            None,
            None,
        )
        .unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&temp.path).unwrap()).unwrap();

        let type_roots = content["compilerOptions"]["typeRoots"].as_array().unwrap();
        assert_eq!(type_roots[0].as_str().unwrap(), abs);
    }

    #[test]
    fn relative_paths_rewritten_to_absolute_in_no_base_url_branch() {
        // Same reasoning as typeRoots — without baseUrl, tsc resolves `paths`
        // entries relative to the tsconfig that defines them. Copied verbatim
        // into the temp config they would resolve two levels deeper.
        let project_root = tempfile::tempdir().unwrap();
        let project_dir = Utf8Path::from_path(project_root.path()).unwrap();
        let tsconfig_path = project_dir.join("tsconfig.json");
        std::fs::write(&tsconfig_path, "{}").unwrap();

        let raw_config = serde_json::json!({
            "compilerOptions": {
                "paths": { "@lib/*": ["./src/lib/*"] }
            }
        });
        let files = vec![project_dir.join("src/a.ts")];

        let temp = write_temp_config(
            tsconfig_path.as_ref(),
            &raw_config,
            &files,
            None,
            None,
            None,
            None,
        )
        .unwrap();

        let content: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&temp.path).unwrap()).unwrap();

        let first = content["compilerOptions"]["paths"]["@lib/*"][0]
            .as_str()
            .unwrap();
        assert!(
            Utf8Path::new(first).is_absolute(),
            "paths entry should be absolute, got: {first}"
        );
        assert!(
            first.ends_with("/src/lib/*"),
            "paths entry should end with /src/lib/*, got: {first}"
        );
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
