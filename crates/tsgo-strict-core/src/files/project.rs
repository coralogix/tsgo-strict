use crate::config::ProjectContext;
use crate::errors::Error;
use crate::files::selection::{is_absolute_posix, path_to_posix, posix_resolve};
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
    if let Some(ref resolved) = ctx.resolved_files {
        return explicit_files_from_resolved(&resolved.patterns, &resolved.config_dir);
    }
    if let Some(explicit) = explicit_files(ctx)? {
        return Ok(ProjectScope { files: explicit });
    }

    let (include_patterns, include_base) = match ctx.resolved_include {
        Some(ref f) => (f.patterns.clone(), f.config_dir.clone()),
        None => (include_patterns(&ctx.raw_config), ctx.config_dir.clone()),
    };
    let (exclude_patterns, exclude_base) = match ctx.resolved_exclude {
        Some(ref f) => (f.patterns.clone(), f.config_dir.clone()),
        None => (exclude_patterns(&ctx.raw_config), ctx.config_dir.clone()),
    };

    let include_set = build_glob_set(&include_patterns, &include_base)?;
    let exclude_set = build_glob_set(
        &exclude_patterns
            .into_iter()
            .chain(DEFAULT_IGNORE.iter().map(|s| s.to_string()))
            .collect::<Vec<_>>(),
        &exclude_base,
    )?;

    let mut builder = WalkBuilder::new(include_base.as_std_path());
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

/// Walk directories specified in the plugin's `paths` config to discover all
/// TS files. This ensures transitively-imported files are included in the
/// project scope even when the tsconfig uses `files: [...]` instead of `include`.
pub fn walk_plugin_paths(
    paths: &[String],
    config_dir: &Utf8PathBuf,
    exclude_patterns: &[String],
    exclude_base: &Utf8PathBuf,
) -> Result<Vec<Utf8PathBuf>, Error> {
    let base_posix = path_to_posix(config_dir.as_str());

    let resolved_dirs: Vec<Utf8PathBuf> = paths
        .iter()
        .filter_map(|p| {
            let normalized = path_to_posix(p);
            let joined = if is_absolute_posix(&normalized) {
                normalized
            } else {
                format!("{}/{}", base_posix.trim_end_matches('/'), normalized)
            };
            let resolved = posix_resolve(&joined);
            let dir = Utf8PathBuf::from(&resolved);
            if dir.exists() {
                Some(dir)
            } else {
                None
            }
        })
        .collect();

    if resolved_dirs.is_empty() {
        return Ok(Vec::new());
    }

    let all_exclude: Vec<String> = exclude_patterns
        .iter()
        .cloned()
        .chain(DEFAULT_IGNORE.iter().map(|s| s.to_string()))
        .collect();
    let exclude_set = build_glob_set(&all_exclude, exclude_base)?;

    let mut files: Vec<Utf8PathBuf> = Vec::new();
    for dir in &resolved_dirs {
        let mut builder = WalkBuilder::new(dir.as_std_path());
        builder
            .standard_filters(false)
            .hidden(false)
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false)
            .follow_links(false);

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
            if let Some(ref excl) = exclude_set {
                if excl.is_match(path.as_std_path()) {
                    continue;
                }
            }
            files.push(path);
        }
    }

    Ok(files)
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

        // Collapse `.` and `..` segments so patterns like `./src/index.ts`
        // anchor to `/proj/src/index.ts` instead of `/proj/./src/index.ts`,
        // which globset would never match against the walker's real paths.
        let anchored = posix_resolve(&anchored);

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
    use crate::config::tsconfig::ResolvedField;
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
    fn build_glob_set_normalizes_dot_segments() {
        // `./src/index.ts` must match `/project/src/index.ts`. Without
        // collapsing `.` segments, the anchored pattern becomes
        // `/project/./src/index.ts` and globset never matches the walker's
        // real path `/project/src/index.ts`.
        let base = Utf8PathBuf::from("/project");
        let patterns = vec!["./src/index.ts".to_string()];
        let set = build_glob_set(&patterns, &base).unwrap().unwrap();
        assert!(
            set.is_match(Path::new("/project/src/index.ts")),
            "should match real path after collapsing ./ in pattern"
        );
    }

    #[test]
    fn build_glob_set_normalizes_dot_segments_in_wildcards() {
        // Same collapsing rule applies to glob patterns with wildcards —
        // `./src/**/*.ts` must match `/project/src/foo.ts`.
        let base = Utf8PathBuf::from("/project");
        let patterns = vec!["./src/**/*.ts".to_string()];
        let set = build_glob_set(&patterns, &base).unwrap().unwrap();
        assert!(set.is_match(Path::new("/project/src/foo.ts")));
        assert!(set.is_match(Path::new("/project/src/deep/bar.ts")));
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

    /// Build a ProjectContext for cross-directory extends tests.
    fn make_cross_dir_context(
        project_dir: &std::path::Path,
        shared_dir: &std::path::Path,
    ) -> ProjectContext {
        let project_dir_utf8 =
            Utf8PathBuf::try_from(project_dir.to_path_buf()).expect("utf8 project dir");
        let shared_dir_utf8 =
            Utf8PathBuf::try_from(shared_dir.to_path_buf()).expect("utf8 shared dir");
        ProjectContext {
            cwd: project_dir_utf8.clone(),
            project_path: project_dir_utf8.join("tsconfig.json"),
            config_dir: project_dir_utf8,
            raw_config: serde_json::json!({}),
            strict_plugin_config: None,
            resolved_include: Some(ResolvedField {
                patterns: vec!["src/**/*".to_string()],
                config_dir: shared_dir_utf8.clone(),
            }),
            resolved_exclude: Some(ResolvedField {
                patterns: vec!["dist".to_string()],
                config_dir: shared_dir_utf8,
            }),
            resolved_files: None,
            effective_base_url: None,
            effective_compiler_options: None,
            effective_type_roots_dir: None,
            auto_type_directives: None,
        }
    }

    #[test]
    fn cross_directory_extends_resolves_globs_relative_to_base() {
        let tmp = tempfile::tempdir().expect("create tmpdir");
        let shared = tmp.path().join("shared");
        let project = tmp.path().join("project");
        std::fs::create_dir_all(shared.join("src")).unwrap();
        std::fs::create_dir_all(shared.join("dist")).unwrap();
        std::fs::create_dir_all(&project).unwrap();

        std::fs::write(shared.join("src/lib.ts"), "export const x = 1;").unwrap();
        std::fs::write(shared.join("dist/out.ts"), "export const y = 2;").unwrap();

        let ctx = make_cross_dir_context(&project, &shared);
        let scope = enumerate_project_files(&ctx).expect("enumerate");

        let names: Vec<String> = scope
            .files
            .iter()
            .map(|p| p.file_name().unwrap_or("").to_string())
            .collect();
        assert!(
            names.contains(&"lib.ts".to_string()),
            "should include shared/src/lib.ts, got: {names:?}"
        );
        assert!(
            !names.contains(&"out.ts".to_string()),
            "should exclude shared/dist/out.ts, got: {names:?}"
        );
    }

    #[test]
    fn walk_plugin_paths_discovers_ts_files_recursively() {
        let tmp = tempfile::tempdir().expect("create tmpdir");
        let root = Utf8PathBuf::try_from(tmp.path().to_path_buf()).unwrap();
        std::fs::create_dir_all(tmp.path().join("src/lib")).unwrap();
        std::fs::write(tmp.path().join("src/main.ts"), "export {};").unwrap();
        std::fs::write(tmp.path().join("src/lib/util.ts"), "export {};").unwrap();
        std::fs::write(tmp.path().join("src/lib/readme.md"), "# hi").unwrap();

        let paths = vec!["./src".to_string()];
        let result = walk_plugin_paths(&paths, &root, &[], &root).unwrap();
        let names: Vec<&str> = result.iter().filter_map(|p| p.file_name()).collect();
        assert!(names.contains(&"main.ts"), "got: {names:?}");
        assert!(names.contains(&"util.ts"), "got: {names:?}");
        assert!(!names.contains(&"readme.md"), "got: {names:?}");
    }

    #[test]
    fn walk_plugin_paths_skips_node_modules() {
        let tmp = tempfile::tempdir().expect("create tmpdir");
        let root = Utf8PathBuf::try_from(tmp.path().to_path_buf()).unwrap();
        std::fs::create_dir_all(tmp.path().join("src/node_modules/pkg")).unwrap();
        std::fs::write(tmp.path().join("src/app.ts"), "export {};").unwrap();
        std::fs::write(
            tmp.path().join("src/node_modules/pkg/index.ts"),
            "export {};",
        )
        .unwrap();

        let paths = vec!["./src".to_string()];
        let result = walk_plugin_paths(&paths, &root, &[], &root).unwrap();
        let names: Vec<&str> = result.iter().filter_map(|p| p.file_name()).collect();
        assert!(names.contains(&"app.ts"), "got: {names:?}");
        assert!(
            !result.iter().any(|p| p.as_str().contains("node_modules")),
            "got: {result:?}"
        );
    }

    #[test]
    fn walk_plugin_paths_respects_exclude_patterns() {
        let tmp = tempfile::tempdir().expect("create tmpdir");
        let root = Utf8PathBuf::try_from(tmp.path().to_path_buf()).unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::create_dir_all(tmp.path().join("dist")).unwrap();
        std::fs::write(tmp.path().join("src/app.ts"), "export {};").unwrap();
        std::fs::write(tmp.path().join("dist/out.ts"), "export {};").unwrap();

        let paths = vec!["./src".to_string(), "./dist".to_string()];
        let exclude = vec!["dist".to_string()];
        let result = walk_plugin_paths(&paths, &root, &exclude, &root).unwrap();
        let names: Vec<&str> = result.iter().filter_map(|p| p.file_name()).collect();
        assert!(names.contains(&"app.ts"), "got: {names:?}");
        assert!(!names.contains(&"out.ts"), "got: {names:?}");
    }

    #[test]
    fn walk_plugin_paths_nonexistent_path_is_silent() {
        let tmp = tempfile::tempdir().expect("create tmpdir");
        let root = Utf8PathBuf::try_from(tmp.path().to_path_buf()).unwrap();

        let paths = vec!["./does-not-exist".to_string()];
        let result = walk_plugin_paths(&paths, &root, &[], &root).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn leaf_override_resolves_globs_relative_to_leaf() {
        let tmp = tempfile::tempdir().expect("create tmpdir");
        let shared = tmp.path().join("shared");
        let project = tmp.path().join("project");
        std::fs::create_dir_all(shared.join("src")).unwrap();
        std::fs::create_dir_all(project.join("src")).unwrap();

        std::fs::write(shared.join("src/base.ts"), "export const a = 1;").unwrap();
        std::fs::write(project.join("src/leaf.ts"), "export const b = 2;").unwrap();

        let project_utf8 = Utf8PathBuf::try_from(project.clone()).expect("utf8 project dir");
        let ctx = ProjectContext {
            cwd: project_utf8.clone(),
            project_path: project_utf8.join("tsconfig.json"),
            config_dir: project_utf8.clone(),
            raw_config: serde_json::json!({}),
            strict_plugin_config: None,
            resolved_include: Some(ResolvedField {
                patterns: vec!["src/**/*".to_string()],
                config_dir: project_utf8,
            }),
            resolved_exclude: None,
            resolved_files: None,
            effective_base_url: None,
            effective_compiler_options: None,
            effective_type_roots_dir: None,
            auto_type_directives: None,
        };

        let scope = enumerate_project_files(&ctx).expect("enumerate");
        let names: Vec<String> = scope
            .files
            .iter()
            .map(|p| p.file_name().unwrap_or("").to_string())
            .collect();
        assert!(
            names.contains(&"leaf.ts".to_string()),
            "should include project/src/leaf.ts, got: {names:?}"
        );
        assert!(
            !names.contains(&"base.ts".to_string()),
            "should NOT include shared/src/base.ts, got: {names:?}"
        );
    }
}
