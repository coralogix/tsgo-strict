use crate::config::ProjectContext;
use crate::errors::Error;
use camino::Utf8PathBuf;
use globset::{Glob, GlobSetBuilder};
use ignore::WalkBuilder;

const TS_EXTENSIONS: &[&str] = &["ts", "tsx", "cts", "mts"];
const DEFAULT_IGNORE: &[&str] = &["**/node_modules/**", "**/.git/**"];

#[derive(Debug, Clone)]
pub struct ProjectScope {
    pub files: Vec<Utf8PathBuf>,
}

/// Enumerate the TS files considered part of the project. The TS impl uses
/// `ts.getParsedCommandLineOfConfigFile(...).fileNames`, which expands the
/// tsconfig `files` + `include` + `exclude`. We reimplement the subset of that
/// behavior that the plugin relies on: if `files` is present use it verbatim;
/// otherwise walk `include` (or the config dir if absent), subtract `exclude`,
/// filter to TS extensions, skip node_modules and .git.
pub fn enumerate_project_files(ctx: &ProjectContext) -> Result<ProjectScope, Error> {
    // Prefer resolved fields from the extends chain; fall back to raw_config.
    if let Some(ref files_list) = ctx.resolved_files {
        return explicit_files_from_resolved(files_list, &ctx.config_dir);
    }
    if let Some(explicit) = explicit_files(ctx)? {
        return Ok(ProjectScope { files: explicit });
    }

    let include_patterns = ctx
        .resolved_include
        .clone()
        .unwrap_or_else(|| include_patterns(&ctx.raw_config));
    let exclude_patterns = ctx
        .resolved_exclude
        .clone()
        .unwrap_or_else(|| exclude_patterns(&ctx.raw_config));

    let include_set = build_glob_set(&include_patterns, &ctx.config_dir)?;
    let exclude_set = build_glob_set(
        &exclude_patterns
            .into_iter()
            .chain(DEFAULT_IGNORE.iter().map(|s| s.to_string()))
            .collect::<Vec<_>>(),
        &ctx.config_dir,
    )?;

    let mut builder = WalkBuilder::new(ctx.config_dir.as_std_path());
    builder
        .standard_filters(false)
        .hidden(false)
        .git_ignore(false)
        .git_global(false)
        .git_exclude(false)
        .follow_links(false);

    let mut files: Vec<Utf8PathBuf> = Vec::new();
    for result in builder.build() {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };
        let Some(ft) = entry.file_type() else {
            continue;
        };
        if !ft.is_file() {
            continue;
        }
        let Ok(path) = Utf8PathBuf::try_from(entry.into_path()) else {
            continue;
        };
        if !is_ts_file(&path) {
            continue;
        }

        if let Some(excl) = &exclude_set {
            if excl.is_match(path.as_std_path()) {
                continue;
            }
        }
        if let Some(incl) = &include_set {
            if !incl.is_match(path.as_std_path()) {
                continue;
            }
        }

        files.push(path);
    }

    Ok(ProjectScope { files })
}

fn explicit_files_from_resolved(
    files_list: &[String],
    config_dir: &Utf8PathBuf,
) -> Result<ProjectScope, Error> {
    let mut out = Vec::with_capacity(files_list.len());
    for rel in files_list {
        let joined = config_dir.as_std_path().join(rel);
        match Utf8PathBuf::try_from(joined) {
            Ok(p) => out.push(p),
            Err(e) => {
                return Err(Error::msg(format!(
                    "tsconfig files entry not valid UTF-8: {}",
                    e.into_path_buf().to_string_lossy()
                )))
            }
        }
    }
    Ok(ProjectScope { files: out })
}

fn explicit_files(ctx: &ProjectContext) -> Result<Option<Vec<Utf8PathBuf>>, Error> {
    let Some(files_array) = ctx.raw_config.get("files").and_then(|v| v.as_array()) else {
        return Ok(None);
    };
    let mut out = Vec::with_capacity(files_array.len());
    for f in files_array {
        let Some(rel) = f.as_str() else { continue };
        let joined = ctx.config_dir.as_std_path().join(rel);
        match Utf8PathBuf::try_from(joined) {
            Ok(p) => out.push(p),
            Err(e) => {
                return Err(Error::msg(format!(
                    "tsconfig files entry not valid UTF-8: {}",
                    e.into_path_buf().to_string_lossy()
                )))
            }
        }
    }
    Ok(Some(out))
}

fn include_patterns(raw: &serde_json::Value) -> Vec<String> {
    raw.get("include")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|e| e.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

fn exclude_patterns(raw: &serde_json::Value) -> Vec<String> {
    raw.get("exclude")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|e| e.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

fn build_glob_set(
    patterns: &[String],
    base: &Utf8PathBuf,
) -> Result<Option<globset::GlobSet>, Error> {
    if patterns.is_empty() {
        return Ok(None);
    }
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let anchored =
            if pattern.starts_with('/') || pattern.starts_with("**/") || pattern.contains(':') {
                pattern.clone()
            } else {
                format!("{}/{}", base.as_str().trim_end_matches('/'), pattern)
            };

        // TypeScript treats bare directory names (no glob chars) as `dir/**/*`.
        // Append `/**` so that a pattern like `src` matches `src/foo/bar.ts`.
        let expanded = if !anchored.contains('*') && !anchored.contains('?') {
            format!("{}/**", anchored.trim_end_matches('/'))
        } else {
            anchored
        };

        let glob = Glob::new(&expanded)
            .map_err(|e| Error::msg(format!("invalid glob pattern '{}': {}", pattern, e)))?;
        builder.add(glob);
    }
    let set = builder
        .build()
        .map_err(|e| Error::msg(format!("glob build error: {}", e)))?;
    Ok(Some(set))
}

fn is_ts_file(path: &Utf8PathBuf) -> bool {
    match path.extension() {
        Some(ext) => TS_EXTENSIONS.iter().any(|e| e.eq_ignore_ascii_case(ext)),
        None => false,
    }
}
