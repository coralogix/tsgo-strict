use crate::errors::Error;
use camino::Utf8PathBuf;
use std::path::{Path, PathBuf};

use super::extends::load_extends_chain;
use super::plugin::StrictPluginConfig;

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
    pub resolved_include: Option<Vec<String>>,
    /// Resolved `exclude` from the extends chain (last-one-to-specify-wins).
    pub resolved_exclude: Option<Vec<String>>,
    /// Resolved `files` from the extends chain (last-one-to-specify-wins).
    pub resolved_files: Option<Vec<String>>,
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
    let resolved_include = resolve_inherited_field(&chain, "include");
    let resolved_exclude = resolve_inherited_field(&chain, "exclude");
    let resolved_files = resolve_inherited_field(&chain, "files");

    Ok(ProjectContext {
        cwd: cwd.clone(),
        project_path,
        config_dir,
        raw_config,
        strict_plugin_config,
        resolved_include,
        resolved_exclude,
        resolved_files,
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
/// a top-level array field (`include`, `exclude`, or `files`). This matches
/// TypeScript's own inheritance: the nearest config that specifies the field wins.
fn resolve_inherited_field(chain: &[serde_json::Value], field: &str) -> Option<Vec<String>> {
    let mut result: Option<Vec<String>> = None;
    for cfg in chain {
        if let Some(arr) = cfg.get(field).and_then(|v| v.as_array()) {
            result = Some(
                arr.iter()
                    .filter_map(|e| e.as_str().map(String::from))
                    .collect(),
            );
        }
    }
    result
}

fn resolve_plugin_config(
    chain: &[serde_json::Value],
    plugin_name: &str,
) -> Option<StrictPluginConfig> {
    let mut matched: Option<StrictPluginConfig> = None;
    for cfg in chain {
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

    #[test]
    fn inherit_include_from_base_when_leaf_omits() {
        let chain = vec![
            json!({ "include": ["src/**/*"] }),
            json!({}),
        ];
        assert_eq!(
            resolve_inherited_field(&chain, "include"),
            Some(vec!["src/**/*".to_string()])
        );
    }

    #[test]
    fn leaf_include_overrides_base() {
        let chain = vec![
            json!({ "include": ["lib/**/*"] }),
            json!({ "include": ["src/**/*"] }),
        ];
        assert_eq!(
            resolve_inherited_field(&chain, "include"),
            Some(vec!["src/**/*".to_string()])
        );
    }

    #[test]
    fn inherit_exclude_from_base_when_leaf_omits() {
        let chain = vec![
            json!({ "exclude": ["dist"] }),
            json!({}),
        ];
        assert_eq!(
            resolve_inherited_field(&chain, "exclude"),
            Some(vec!["dist".to_string()])
        );
    }

    #[test]
    fn no_field_in_chain_returns_none() {
        let chain = vec![json!({}), json!({})];
        assert_eq!(resolve_inherited_field(&chain, "include"), None);
    }

    #[test]
    fn three_level_chain_middle_overrides_root() {
        let chain = vec![
            json!({ "include": ["a"] }),
            json!({ "include": ["b"] }),
            json!({}),
        ];
        assert_eq!(
            resolve_inherited_field(&chain, "include"),
            Some(vec!["b".to_string()])
        );
    }
}
