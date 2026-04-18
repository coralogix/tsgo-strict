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

    Ok(ProjectContext {
        cwd: cwd.clone(),
        project_path,
        config_dir,
        raw_config,
        strict_plugin_config,
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

            let exclude_pattern = obj
                .get("excludePattern")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            matched = Some(StrictPluginConfig {
                name: plugin_name.to_string(),
                paths,
                exclude_pattern,
            });
        }
    }
    matched
}
