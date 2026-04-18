use crate::config::StrictPluginConfig;
use crate::errors::Error;
use crate::files::pragma::{detect_pragma, PragmaHint};
use camino::{Utf8Path, Utf8PathBuf};
use globset::{Glob, GlobMatcher};
use rayon::prelude::*;
use regex::Regex;

/// Decide which files qualify as "strict" under the plugin config. Mirrors
/// `findStrictCandidates` + `isStrictFile` in src/config/strictFileSelection.ts:
///
/// 1. Read first 4096 bytes, look for pragma:
///    * `@ts-strict-ignore` → excluded, regardless of config.
///    * `@ts-strict` → included, bypasses paths and excludePattern.
/// 2. Otherwise, if plugin config is absent → include.
/// 3. Otherwise, file is included iff its path (relative to `config_dir`,
///    forward-slash normalized) matches any `paths` pattern (or `paths` is
///    empty/absent) AND does not match `excludePattern` (treated as a regex).
pub fn find_strict_candidates(
    project_files: &[Utf8PathBuf],
    plugin_config: Option<&StrictPluginConfig>,
    config_dir: &Utf8PathBuf,
) -> Result<Vec<Utf8PathBuf>, Error> {
    let path_matchers = compile_path_matchers(plugin_config)?;
    let exclude_regex = compile_exclude_regex(plugin_config)?;

    let mut candidates: Vec<Utf8PathBuf> = project_files
        .par_iter()
        .filter_map(|file| {
            if !is_strict_file(
                file,
                plugin_config,
                config_dir,
                path_matchers.as_deref(),
                exclude_regex.as_ref(),
            ) {
                return None;
            }
            Some(file.clone())
        })
        .collect();

    candidates.sort();
    candidates.dedup();
    Ok(candidates)
}

fn is_strict_file(
    file: &Utf8Path,
    plugin_config: Option<&StrictPluginConfig>,
    config_dir: &Utf8PathBuf,
    path_matchers: Option<&[GlobMatcher]>,
    exclude_regex: Option<&Regex>,
) -> bool {
    let pragma = detect_pragma(file.as_std_path());
    if pragma == PragmaHint::Ignore {
        return false;
    }

    let Some(plugin) = plugin_config else {
        return true;
    };

    let relative = normalize_relative(file, config_dir);
    let relative_path = std::path::Path::new(&relative);

    let in_paths = match path_matchers {
        None => true,
        Some(matchers) if matchers.is_empty() => true,
        Some(matchers) => matchers.iter().any(|m| m.is_match(relative_path)),
    };

    let excluded = match exclude_regex {
        Some(re) => re.is_match(&relative),
        None => false,
    };

    if pragma == PragmaHint::Strict {
        return true;
    }

    let _ = plugin;
    in_paths && !excluded
}

fn compile_path_matchers(
    plugin_config: Option<&StrictPluginConfig>,
) -> Result<Option<Vec<GlobMatcher>>, Error> {
    let Some(cfg) = plugin_config else {
        return Ok(None);
    };
    let Some(paths) = &cfg.paths else {
        return Ok(None);
    };
    let mut out = Vec::with_capacity(paths.len());
    for pattern in paths {
        let normalized = pattern.replace('\\', "/");
        let glob = Glob::new(&normalized).map_err(|e| {
            Error::msg(format!(
                "invalid plugin 'paths' pattern '{}': {}",
                pattern, e
            ))
        })?;
        out.push(glob.compile_matcher());
    }
    Ok(Some(out))
}

fn compile_exclude_regex(plugin_config: Option<&StrictPluginConfig>) -> Result<Option<Regex>, Error> {
    let Some(cfg) = plugin_config else {
        return Ok(None);
    };
    let Some(pattern) = &cfg.exclude_pattern else {
        return Ok(None);
    };
    let regex = Regex::new(pattern).map_err(|e| {
        Error::msg(format!(
            "invalid plugin 'excludePattern' regex '{}': {}",
            pattern, e
        ))
    })?;
    Ok(Some(regex))
}

fn normalize_relative(file: &Utf8Path, base: &Utf8PathBuf) -> String {
    let relative = pathdiff(file, base).unwrap_or_else(|| file.to_string());
    relative.replace('\\', "/")
}

fn pathdiff(path: &Utf8Path, base: &Utf8PathBuf) -> Option<String> {
    use std::path::Component;
    let path = path.as_std_path();
    let base = base.as_std_path();

    let mut path_components = path.components();
    let mut base_components = base.components();
    let mut out: Vec<String> = Vec::new();

    loop {
        match (path_components.next(), base_components.next()) {
            (Some(a), Some(b)) if a == b => continue,
            (Some(a), Some(_)) => {
                out.push("..".to_string());
                push_component(&mut out, a);
                for _ in base_components.by_ref() {
                    out.insert(0, "..".to_string());
                }
                for c in path_components.by_ref() {
                    push_component(&mut out, c);
                }
                break;
            }
            (Some(a), None) => {
                push_component(&mut out, a);
                for c in path_components.by_ref() {
                    push_component(&mut out, c);
                }
                break;
            }
            (None, Some(_)) => {
                out.insert(0, "..".to_string());
                for _ in base_components.by_ref() {
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

    Some(out.join("/"))
}
