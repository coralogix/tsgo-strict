use crate::errors::Error;
use camino::{Utf8Path, Utf8PathBuf};
use globset::{Glob, GlobSetBuilder};
use ignore::WalkBuilder;
use std::collections::HashSet;

const TS_EXTENSIONS: &[&str] = &["ts", "tsx", "cts", "mts"];
const DEFAULT_IGNORE: &[&str] = &["**/node_modules/**", "**/.git/**"];

/// Port of `resolveSubsetInputs` in src/files/resolveSubset.ts. Positional CLI
/// arguments can be: explicit files (kept as-is if TS), directories (walked
/// recursively for TS files), or glob patterns (matched from cwd).
pub fn resolve_subset_inputs(
    inputs: &[String],
    cwd: &Utf8PathBuf,
) -> Result<Vec<Utf8PathBuf>, Error> {
    if inputs.is_empty() {
        return Ok(Vec::new());
    }

    let mut explicit_files: Vec<Utf8PathBuf> = Vec::new();
    let mut directories: Vec<Utf8PathBuf> = Vec::new();
    let mut patterns: Vec<String> = Vec::new();

    for input in inputs {
        if is_glob(input) {
            patterns.push(input.clone());
            continue;
        }

        let abs = cwd.as_std_path().join(input);
        let meta = std::fs::metadata(&abs).ok();
        match meta {
            None => patterns.push(input.clone()),
            Some(m) if m.is_dir() => {
                if let Ok(p) = Utf8PathBuf::try_from(abs) {
                    directories.push(p);
                }
            }
            Some(_) => match Utf8PathBuf::try_from(abs) {
                Ok(p) if is_ts_file(&p) => explicit_files.push(p),
                _ => {}
            },
        }
    }

    let dir_walked = walk_directories(&directories)?;
    let globbed = if patterns.is_empty() {
        Vec::new()
    } else {
        glob_walk(&patterns, cwd)?
    };

    let mut merged: HashSet<Utf8PathBuf> = HashSet::new();
    let mut out: Vec<Utf8PathBuf> = Vec::new();
    for f in explicit_files.into_iter().chain(dir_walked).chain(globbed) {
        if is_ts_file(&f) && merged.insert(f.clone()) {
            out.push(f);
        }
    }

    Ok(out)
}

fn walk_directories(dirs: &[Utf8PathBuf]) -> Result<Vec<Utf8PathBuf>, Error> {
    if dirs.is_empty() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    for dir in dirs {
        let mut builder = WalkBuilder::new(dir.as_std_path());
        builder
            .standard_filters(false)
            .hidden(false)
            .git_ignore(false)
            .follow_links(false);
        for result in builder.build() {
            let Ok(entry) = result else { continue };
            let Some(ft) = entry.file_type() else {
                continue;
            };
            if !ft.is_file() {
                continue;
            }
            let path = entry.into_path();
            if path
                .components()
                .any(|c| matches!(c.as_os_str().to_str(), Some("node_modules" | ".git")))
            {
                continue;
            }
            let Ok(utf8) = Utf8PathBuf::try_from(path) else {
                continue;
            };
            if is_ts_file(&utf8) {
                files.push(utf8);
            }
        }
    }
    Ok(files)
}

fn glob_walk(patterns: &[String], cwd: &Utf8PathBuf) -> Result<Vec<Utf8PathBuf>, Error> {
    let mut include_builder = GlobSetBuilder::new();
    for pattern in patterns {
        let anchored = anchor_pattern(pattern, cwd);
        let glob = Glob::new(&anchored)
            .map_err(|e| Error::msg(format!("invalid glob '{}': {}", pattern, e)))?;
        include_builder.add(glob);
    }
    let include_set = include_builder
        .build()
        .map_err(|e| Error::msg(format!("glob build error: {}", e)))?;

    let mut ignore_builder = GlobSetBuilder::new();
    for p in DEFAULT_IGNORE {
        let glob = Glob::new(p)
            .map_err(|e| Error::msg(format!("invalid default ignore '{}': {}", p, e)))?;
        ignore_builder.add(glob);
    }
    let ignore_set = ignore_builder
        .build()
        .map_err(|e| Error::msg(format!("glob build error: {}", e)))?;

    let mut files: Vec<Utf8PathBuf> = Vec::new();
    let mut builder = WalkBuilder::new(cwd.as_std_path());
    builder
        .standard_filters(false)
        .hidden(false)
        .git_ignore(false)
        .follow_links(false);

    for result in builder.build() {
        let Ok(entry) = result else { continue };
        let Some(ft) = entry.file_type() else {
            continue;
        };
        if !ft.is_file() {
            continue;
        }
        let Ok(path) = Utf8PathBuf::try_from(entry.into_path()) else {
            continue;
        };
        if ignore_set.is_match(path.as_std_path()) {
            continue;
        }
        if !include_set.is_match(path.as_std_path()) {
            continue;
        }
        files.push(path);
    }

    Ok(files)
}

fn anchor_pattern(pattern: &str, cwd: &Utf8PathBuf) -> String {
    if pattern.starts_with('/') || pattern.contains(':') {
        pattern.to_string()
    } else {
        format!("{}/{}", cwd.as_str().trim_end_matches('/'), pattern)
    }
}

fn is_glob(input: &str) -> bool {
    input
        .chars()
        .any(|c| matches!(c, '*' | '?' | '[' | ']' | '{' | '}' | '(' | ')' | '!'))
}

fn is_ts_file(path: &Utf8Path) -> bool {
    match path.extension() {
        Some(ext) => TS_EXTENSIONS.iter().any(|e| e.eq_ignore_ascii_case(ext)),
        None => false,
    }
}
