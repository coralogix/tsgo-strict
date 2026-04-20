use crate::errors::Error;
use camino::Utf8PathBuf;
use serde_json::Value;
use std::path::{Path, PathBuf};

use super::base_url::{resolve_effective_base_url, resolve_effective_compiler_options};
use super::extends::load_extends_chain;
use super::plugin::StrictPluginConfig;

/// A resolved top-level array field (`include`, `exclude`, or `files`) paired
/// with the directory of the tsconfig that defined it. TypeScript resolves
/// inherited globs relative to the config that specified them, not the leaf.
#[derive(Debug, Clone)]
pub struct ResolvedField {
    pub patterns: Vec<String>,
    pub config_dir: Utf8PathBuf,
}

/// The project context we build once per invocation. Mirrors the TS
/// `ProjectContext` shape so the rest of the pipeline reads the same fields.
#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub cwd: Utf8PathBuf,
    pub project_path: Utf8PathBuf,
    pub config_dir: Utf8PathBuf,
    /// Raw tsconfig JSON (the project's own, not a merge of the chain). Used as
    /// the source of `compilerOptions` when writing the temp tsconfig that tsgo
    /// actually consumes.
    pub raw_config: serde_json::Value,
    pub strict_plugin_config: Option<StrictPluginConfig>,
    /// Resolved `include` from the extends chain (last-one-to-specify-wins).
    pub resolved_include: Option<ResolvedField>,
    /// Resolved `exclude` from the extends chain (last-one-to-specify-wins).
    pub resolved_exclude: Option<ResolvedField>,
    /// Resolved `files` from the extends chain (last-one-to-specify-wins).
    pub resolved_files: Option<ResolvedField>,
    /// Absolute directory that `baseUrl` resolves to, if set anywhere in the
    /// extends chain. When present, the temp config must inline all
    /// compilerOptions instead of using `extends` to avoid TS5102 in tsgo.
    pub effective_base_url: Option<Utf8PathBuf>,
    /// Shallow-per-key merge of `compilerOptions` from the full extends chain.
    /// Only populated when `effective_base_url` is `Some` (to avoid unnecessary
    /// work). Used by `write_temp_config` when inlining without `extends`.
    pub effective_compiler_options: Option<serde_json::Map<String, Value>>,
}

pub fn load_project_context(
    cwd: &Utf8PathBuf,
    project_arg: &str,
    plugin_name: &str,
) -> Result<ProjectContext, Error> {
    let project_path_std: PathBuf = cwd.as_std_path().join(project_arg);
    let project_path = Utf8PathBuf::try_from(project_path_std).map_err(|e| {
        Error::msg(format!(
            "project path is not valid UTF-8: {}",
            e.into_path_buf().to_string_lossy()
        ))
    })?;

    let config_dir = project_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| Utf8PathBuf::from("."));

    let raw_config = read_raw_tsconfig(project_path.as_std_path())?;

    let chain = load_extends_chain(project_path.as_std_path())?;
    let strict_plugin_config = resolve_plugin_config(&chain, plugin_name);
    let resolved_include = resolve_inherited_field(&chain, "include")?;
    let resolved_exclude = resolve_inherited_field(&chain, "exclude")?;

    // TS treats files/include as a group for inheritance. If the leaf specifies
    // `include`, a parent's `files: []` must not short-circuit file resolution.
    let leaf_has_include = chain
        .last()
        .map(|(_, cfg)| cfg.get("include").and_then(|v| v.as_array()).is_some())
        .unwrap_or(false);
    let resolved_files = if leaf_has_include {
        resolve_leaf_only_field(&chain, "files")?
    } else {
        resolve_inherited_field(&chain, "files")?
    };

    let effective_base_url = resolve_effective_base_url(&chain);
    let effective_compiler_options = if effective_base_url.is_some() {
        Some(resolve_effective_compiler_options(&chain))
    } else {
        None
    };

    Ok(ProjectContext {
        cwd: cwd.clone(),
        project_path,
        config_dir,
        raw_config,
        strict_plugin_config,
        resolved_include,
        resolved_exclude,
        resolved_files,
        effective_base_url,
        effective_compiler_options,
    })
}

/// Read a JSONC tsconfig from disk and parse it as loose JSON. Supports the
/// same relaxations TypeScript's `readConfigFile` accepts (line+block comments,
/// trailing commas).
pub fn read_raw_tsconfig(path: &Path) -> Result<serde_json::Value, Error> {
    let contents = std::fs::read_to_string(path).map_err(|source| Error::TsconfigRead {
        path: path.to_path_buf(),
        source,
    })?;
    parse_jsonc(&contents).map_err(|message| Error::TsconfigParse {
        path: path.to_path_buf(),
        message,
    })
}

fn parse_jsonc(source: &str) -> Result<serde_json::Value, String> {
    use jsonc_parser::{parse_to_serde_value, ParseOptions};

    let parsed =
        parse_to_serde_value(source, &ParseOptions::default()).map_err(|e| e.to_string())?;
    Ok(parsed.unwrap_or(serde_json::Value::Null))
}

/// Walk the extends chain (root-first) and return the last-specified value for
/// a top-level array field (`include`, `exclude`, or `files`) together with the
/// directory of the config that defined it. TypeScript resolves inherited globs
/// relative to whichever config specified them, not the leaf.
fn resolve_inherited_field(
    chain: &[(PathBuf, serde_json::Value)],
    field: &str,
) -> Result<Option<ResolvedField>, Error> {
    let mut result: Option<ResolvedField> = None;
    for (dir, cfg) in chain {
        if let Some(arr) = cfg.get(field).and_then(|v| v.as_array()) {
            let config_dir = Utf8PathBuf::try_from(dir.clone()).map_err(|e| {
                Error::msg(format!(
                    "config directory is not valid UTF-8: {}",
                    e.into_path_buf().to_string_lossy()
                ))
            })?;
            result = Some(ResolvedField {
                patterns: arr
                    .iter()
                    .filter_map(|e| e.as_str().map(String::from))
                    .collect(),
                config_dir,
            });
        }
    }
    Ok(result)
}

/// Like `resolve_inherited_field`, but only checks the leaf (last) entry in
/// the chain. Used when the leaf specifies `include` and we must not let a
/// parent's `files: []` short-circuit file enumeration.
fn resolve_leaf_only_field(
    chain: &[(PathBuf, serde_json::Value)],
    field: &str,
) -> Result<Option<ResolvedField>, Error> {
    if let Some((dir, cfg)) = chain.last() {
        if let Some(arr) = cfg.get(field).and_then(|v| v.as_array()) {
            let config_dir = Utf8PathBuf::try_from(dir.clone()).map_err(|e| {
                Error::msg(format!(
                    "config directory is not valid UTF-8: {}",
                    e.into_path_buf().to_string_lossy()
                ))
            })?;
            return Ok(Some(ResolvedField {
                patterns: arr
                    .iter()
                    .filter_map(|e| e.as_str().map(String::from))
                    .collect(),
                config_dir,
            }));
        }
    }
    Ok(None)
}

fn resolve_plugin_config(
    chain: &[(PathBuf, serde_json::Value)],
    plugin_name: &str,
) -> Option<StrictPluginConfig> {
    let mut matched: Option<StrictPluginConfig> = None;
    for (_dir, cfg) in chain {
        let Some(plugins) = cfg
            .get("compilerOptions")
            .and_then(|co| co.get("plugins"))
            .and_then(|p| p.as_array())
        else {
            continue;
        };

        for plugin in plugins {
            let Some(obj) = plugin.as_object() else {
                continue;
            };
            if obj.get("name").and_then(|v| v.as_str()) != Some(plugin_name) {
                continue;
            }

            let paths = obj.get("paths").and_then(|v| v.as_array()).map(|arr| {
                arr.iter()
                    .filter_map(|entry| entry.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            });

            // Match the original plugin: `excludePattern` is `string[]` of
            // minimatch globs. Accept a bare string too for convenience —
            // `typescript-strict-plugin` doesn't, but the TS typings let you
            // get away with it and users in the wild write both shapes.
            let exclude_pattern = match obj.get("excludePattern") {
                Some(serde_json::Value::String(s)) => Some(vec![s.clone()]),
                Some(serde_json::Value::Array(arr)) => Some(
                    arr.iter()
                        .filter_map(|entry| entry.as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            };

            matched = Some(StrictPluginConfig {
                name: plugin_name.to_string(),
                paths,
                exclude_pattern,
            });
        }
    }
    matched
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn chain_entry(dir: &str, value: serde_json::Value) -> (PathBuf, serde_json::Value) {
        (PathBuf::from(dir), value)
    }

    #[test]
    fn inherit_include_from_base_when_leaf_omits() {
        let chain = vec![
            chain_entry("/base", json!({ "include": ["src/**/*"] })),
            chain_entry("/leaf", json!({})),
        ];
        let field = resolve_inherited_field(&chain, "include").unwrap().unwrap();
        assert_eq!(field.patterns, vec!["src/**/*".to_string()]);
        assert_eq!(field.config_dir, Utf8PathBuf::from("/base"));
    }

    #[test]
    fn leaf_include_overrides_base() {
        let chain = vec![
            chain_entry("/base", json!({ "include": ["lib/**/*"] })),
            chain_entry("/leaf", json!({ "include": ["src/**/*"] })),
        ];
        let field = resolve_inherited_field(&chain, "include").unwrap().unwrap();
        assert_eq!(field.patterns, vec!["src/**/*".to_string()]);
        assert_eq!(field.config_dir, Utf8PathBuf::from("/leaf"));
    }

    #[test]
    fn inherit_exclude_from_base_when_leaf_omits() {
        let chain = vec![
            chain_entry("/base", json!({ "exclude": ["dist"] })),
            chain_entry("/leaf", json!({})),
        ];
        let field = resolve_inherited_field(&chain, "exclude").unwrap().unwrap();
        assert_eq!(field.patterns, vec!["dist".to_string()]);
        assert_eq!(field.config_dir, Utf8PathBuf::from("/base"));
    }

    #[test]
    fn no_field_in_chain_returns_none() {
        let chain = vec![
            chain_entry("/base", json!({})),
            chain_entry("/leaf", json!({})),
        ];
        assert!(resolve_inherited_field(&chain, "include")
            .unwrap()
            .is_none());
    }

    #[test]
    fn three_level_chain_middle_overrides_root() {
        let chain = vec![
            chain_entry("/root", json!({ "include": ["a"] })),
            chain_entry("/mid", json!({ "include": ["b"] })),
            chain_entry("/leaf", json!({})),
        ];
        let field = resolve_inherited_field(&chain, "include").unwrap().unwrap();
        assert_eq!(field.patterns, vec!["b".to_string()]);
        assert_eq!(field.config_dir, Utf8PathBuf::from("/mid"));
    }

    #[test]
    fn parent_files_empty_does_not_block_leaf_include() {
        // Parent has files: [], leaf has include — resolved_files should be None
        // so that file enumeration falls through to include-based resolution.
        let chain = vec![
            chain_entry("/parent", json!({ "files": [] })),
            chain_entry("/leaf", json!({ "include": ["src/**/*.ts"] })),
        ];
        // leaf has include → resolve_leaf_only_field for files → leaf has no files → None
        let resolved = resolve_leaf_only_field(&chain, "files").unwrap();
        assert!(resolved.is_none());
        // include still resolves normally
        let include = resolve_inherited_field(&chain, "include").unwrap().unwrap();
        assert_eq!(include.patterns, vec!["src/**/*.ts".to_string()]);
    }

    #[test]
    fn leaf_files_still_used_when_leaf_has_include() {
        // Leaf has both files and include — leaf's files should be used.
        let chain = vec![
            chain_entry("/parent", json!({ "files": ["parent.ts"] })),
            chain_entry(
                "/leaf",
                json!({ "files": ["leaf.ts"], "include": ["src/**/*.ts"] }),
            ),
        ];
        let resolved = resolve_leaf_only_field(&chain, "files").unwrap().unwrap();
        assert_eq!(resolved.patterns, vec!["leaf.ts".to_string()]);
        assert_eq!(resolved.config_dir, Utf8PathBuf::from("/leaf"));
    }

    #[test]
    fn parent_files_inherited_when_leaf_has_no_include() {
        // Leaf has no include — parent's files should be inherited (existing behavior).
        let chain = vec![
            chain_entry("/parent", json!({ "files": ["a.ts"] })),
            chain_entry("/leaf", json!({})),
        ];
        let resolved = resolve_inherited_field(&chain, "files").unwrap().unwrap();
        assert_eq!(resolved.patterns, vec!["a.ts".to_string()]);
        assert_eq!(resolved.config_dir, Utf8PathBuf::from("/parent"));
    }
}
