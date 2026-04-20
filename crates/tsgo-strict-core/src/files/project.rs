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

pub(crate) fn build_glob_set(
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

        if !anchored.contains('*') && !anchored.contains('?') {
            // Literal pattern: add both a literal match (for file paths like
            // "src/test-setup.ts") and a directory match (for directories like
            // "src/legacy"). GlobSet matches if ANY glob in the set matches.
            let literal_glob = Glob::new(&anchored)
                .map_err(|e| Error::msg(format!("invalid glob pattern '{}': {}", pattern, e)))?;
            builder.add(literal_glob);
            let dir_glob = Glob::new(&format!("{}/**", anchored.trim_end_matches('/')))
                .map_err(|e| Error::msg(format!("invalid glob pattern '{}': {}", pattern, e)))?;
            builder.add(dir_glob);
        } else {
            let glob = Glob::new(&anchored)
                .map_err(|e| Error::msg(format!("invalid glob pattern '{}': {}", pattern, e)))?;
            builder.add(glob);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn build_glob_set_matches_file_specific_exclude() {
        let base = Utf8PathBuf::from("/project");
        let patterns = vec!["src/test-setup.ts".to_string()];
        let set = build_glob_set(&patterns, &base).unwrap().unwrap();
        assert!(
            set.is_match(Path::new("/project/src/test-setup.ts")),
            "should match exact file path"
        );
        assert!(
            !set.is_match(Path::new("/project/src/app.ts")),
            "should not match other files"
        );
    }

    #[test]
    fn build_glob_set_matches_directory_exclude() {
        let base = Utf8PathBuf::from("/project");
        let patterns = vec!["src/legacy".to_string()];
        let set = build_glob_set(&patterns, &base).unwrap().unwrap();
        assert!(
            set.is_match(Path::new("/project/src/legacy/foo.ts")),
            "should match files inside directory"
        );
        assert!(
            set.is_match(Path::new("/project/src/legacy/deep/bar.ts")),
            "should match deeply nested files"
        );
        assert!(
            !set.is_match(Path::new("/project/src/other/foo.ts")),
            "should not match files outside directory"
        );
    }

    #[test]
    fn build_glob_set_wildcard_patterns_work() {
        let base = Utf8PathBuf::from("/project");
        let patterns = vec!["src/**/*.spec.ts".to_string()];
        let set = build_glob_set(&patterns, &base).unwrap().unwrap();
        assert!(set.is_match(Path::new("/project/src/app.spec.ts")));
        assert!(set.is_match(Path::new("/project/src/deep/foo.spec.ts")));
        assert!(!set.is_match(Path::new("/project/src/app.ts")));
    }

    #[test]
    fn build_glob_set_mixed_patterns() {
        let base = Utf8PathBuf::from("/project");
        let patterns = vec![
            "src/**/*.spec.ts".to_string(),
            "src/test-setup.ts".to_string(),
        ];
        let set = build_glob_set(&patterns, &base).unwrap().unwrap();
        assert!(
            set.is_match(Path::new("/project/src/app.spec.ts")),
            "should match glob pattern"
        );
        assert!(
            set.is_match(Path::new("/project/src/test-setup.ts")),
            "should match literal file"
        );
        assert!(
            !set.is_match(Path::new("/project/src/app.ts")),
            "should not match unrelated files"
        );
    }

    #[test]
    fn build_glob_set_empty_returns_none() {
        let base = Utf8PathBuf::from("/project");
        let patterns: Vec<String> = vec![];
        assert!(build_glob_set(&patterns, &base).unwrap().is_none());
    }
}
