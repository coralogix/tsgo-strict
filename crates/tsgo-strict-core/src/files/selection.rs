use crate::config::StrictPluginConfig;
use crate::errors::Error;
use crate::files::pragma::{detect_pragma, PragmaHint};
use camino::{Utf8Path, Utf8PathBuf};
use globset::{Glob, GlobMatcher};
use rayon::prelude::*;
use rayon::ThreadPool;
use std::sync::OnceLock;

/// Thread pool sized for I/O-bound pragma scans. The default rayon pool is
/// sized for CPU work (~num_cpus); disk reads block, so using more threads
/// lets more `open`/`read` syscalls overlap. Benchmarks on 4k-file projects
/// showed ~10-17% wall-clock wins moving from 16 to 64 threads; beyond 64 the
/// scheduler overhead wins out. Capped at 4x cpus to stay modest on small
/// machines.
fn io_pool() -> &'static ThreadPool {
    static POOL: OnceLock<ThreadPool> = OnceLock::new();
    POOL.get_or_init(|| {
        let cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let threads = (cpus * 4).clamp(cpus, 64);
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .thread_name(|i| format!("tsgo-strict-io-{i}"))
            .build()
            .expect("build strict-plugin IO thread pool")
    })
}

/// Decide which files qualify as "strict" under the plugin config, mirroring
/// `typescript-strict-plugin`'s `isFileStrict`:
///
/// 1. Read first 4096 bytes, look for pragma:
///    * `@ts-strict-ignore` → excluded, regardless of config.
///    * `@ts-strict` → included, bypasses paths and excludePattern.
/// 2. Otherwise, if plugin config is absent → include.
/// 3. Otherwise, the file is included iff:
///    * `paths` is absent, OR its absolute posix path starts with any
///      `paths` entry resolved against `config_dir` (directory-prefix
///      match, matching the upstream plugin's `startsWith` behavior);
///    * AND no `excludePattern` minimatch glob matches the file's
///      absolute posix path.
pub fn find_strict_candidates(
    project_files: Vec<Utf8PathBuf>,
    plugin_config: Option<&StrictPluginConfig>,
    config_dir: &Utf8PathBuf,
) -> Result<Vec<Utf8PathBuf>, Error> {
    let path_prefixes = compile_path_prefixes(plugin_config, config_dir);
    let exclude_matchers = compile_exclude_matchers(plugin_config)?;

    let mut candidates: Vec<Utf8PathBuf> = io_pool().install(|| {
        project_files
            .into_par_iter()
            .filter(|file| {
                is_strict_file(
                    file,
                    plugin_config,
                    path_prefixes.as_deref(),
                    exclude_matchers.as_deref(),
                )
            })
            .collect()
    });

    candidates.sort();
    Ok(candidates)
}

fn is_strict_file(
    file: &Utf8Path,
    plugin_config: Option<&StrictPluginConfig>,
    path_prefixes: Option<&[String]>,
    exclude_matchers: Option<&[GlobMatcher]>,
) -> bool {
    let pragma = detect_pragma(file.as_std_path());
    match pragma {
        PragmaHint::Ignore => return false,
        PragmaHint::Strict => return true,
        PragmaHint::None => {}
    }

    if plugin_config.is_none() {
        return true;
    }

    let posix = to_posix(file);

    let in_paths = match path_prefixes {
        None => true,
        Some(prefixes) => prefixes.is_empty() || prefixes.iter().any(|p| is_on_path(&posix, p)),
    };

    let excluded = match exclude_matchers {
        Some(matchers) => matchers.iter().any(|m| m.is_match(&posix)),
        None => false,
    };

    in_paths && !excluded
}

/// Resolve each `paths` entry against `config_dir` into an absolute posix
/// directory prefix, matching `getAbsolutePath(projectPath, strictPath)` in
/// the upstream plugin (Node's `path.resolve` + posix normalization).
fn compile_path_prefixes(
    plugin_config: Option<&StrictPluginConfig>,
    config_dir: &Utf8PathBuf,
) -> Option<Vec<String>> {
    let cfg = plugin_config?;
    let paths = cfg.paths.as_ref()?;
    let base_posix = path_to_posix(config_dir.as_str());
    let out = paths
        .iter()
        .map(|pattern| {
            let normalized = path_to_posix(pattern);
            let joined = if is_absolute_posix(&normalized) {
                normalized
            } else {
                format!("{}/{}", base_posix.trim_end_matches('/'), normalized)
            };
            posix_resolve(&joined)
        })
        .collect();
    Some(out)
}

/// Collapse `.` and `..` segments and duplicate slashes from a posix path,
/// mirroring `path.resolve`'s cleanup. Doesn't touch the filesystem.
fn posix_resolve(path: &str) -> String {
    let absolute = path.starts_with('/');
    let drive = path
        .as_bytes()
        .get(1)
        .and_then(|b| (*b == b':').then(|| &path[..2]));

    let rest = if absolute {
        &path[1..]
    } else if drive.is_some() {
        // Treat Windows-style `C:/foo` as rooted at `C:/`.
        if path.as_bytes().get(2) == Some(&b'/') {
            &path[3..]
        } else {
            &path[2..]
        }
    } else {
        path
    };

    let mut stack: Vec<&str> = Vec::new();
    for seg in rest.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            other => stack.push(other),
        }
    }

    let body = stack.join("/");
    match (absolute, drive) {
        (true, _) => format!("/{body}"),
        (false, Some(d)) => format!("{d}/{body}"),
        (false, None) => body,
    }
    .trim_end_matches('/')
    .to_string()
}

fn compile_exclude_matchers(
    plugin_config: Option<&StrictPluginConfig>,
) -> Result<Option<Vec<GlobMatcher>>, Error> {
    let Some(cfg) = plugin_config else {
        return Ok(None);
    };
    let Some(patterns) = &cfg.exclude_pattern else {
        return Ok(None);
    };
    let mut out = Vec::with_capacity(patterns.len());
    for pattern in patterns {
        let glob = Glob::new(pattern).map_err(|e| {
            Error::msg(format!(
                "invalid plugin 'excludePattern' glob '{}': {}",
                pattern, e
            ))
        })?;
        out.push(glob.compile_matcher());
    }
    Ok(Some(out))
}

/// Directory-prefix check: `file` is "on" `prefix` iff it equals it or lives
/// under it. Matches `filePath.startsWith(prefix + path.sep)` in the upstream.
fn is_on_path(file_posix: &str, prefix: &str) -> bool {
    let prefix = prefix.trim_end_matches('/');
    if prefix.is_empty() {
        return true;
    }
    if !file_posix.starts_with(prefix) {
        return false;
    }
    matches!(file_posix.as_bytes().get(prefix.len()), Some(&b'/'))
}

fn to_posix(file: &Utf8Path) -> String {
    path_to_posix(file.as_str())
}

fn path_to_posix(s: &str) -> String {
    s.replace('\\', "/")
}

fn is_absolute_posix(s: &str) -> bool {
    s.starts_with('/') || s.as_bytes().get(1).map(|&b| b == b':').unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(paths: Option<Vec<&str>>, exclude: Option<Vec<&str>>) -> StrictPluginConfig {
        StrictPluginConfig {
            name: "typescript-strict-plugin".to_string(),
            paths: paths.map(|v| v.into_iter().map(String::from).collect()),
            exclude_pattern: exclude.map(|v| v.into_iter().map(String::from).collect()),
        }
    }

    #[test]
    fn path_prefix_matches_directory_not_glob() {
        let base = Utf8PathBuf::from("/proj");
        let prefixes = compile_path_prefixes(Some(&cfg(Some(vec!["src/a"]), None)), &base).unwrap();
        assert_eq!(prefixes, vec!["/proj/src/a"]);

        assert!(is_on_path("/proj/src/a/foo.ts", &prefixes[0]));
        assert!(is_on_path("/proj/src/a/sub/foo.ts", &prefixes[0]));
        // `/proj/src/ab.ts` must not match `/proj/src/a` — prefix needs a `/`.
        assert!(!is_on_path("/proj/src/ab.ts", &prefixes[0]));
        assert!(!is_on_path("/proj/src/b/foo.ts", &prefixes[0]));
    }

    #[test]
    fn absolute_path_entries_bypass_config_dir() {
        let base = Utf8PathBuf::from("/proj");
        let prefixes =
            compile_path_prefixes(Some(&cfg(Some(vec!["/other/root"]), None)), &base).unwrap();
        assert_eq!(prefixes, vec!["/other/root"]);
    }

    #[test]
    fn empty_paths_list_means_include_everything() {
        let base = Utf8PathBuf::from("/proj");
        let cfg = cfg(Some(vec![]), None);
        let file = Utf8PathBuf::from("/proj/src/x.ts");
        assert!(is_strict_file(
            &file,
            Some(&cfg),
            compile_path_prefixes(Some(&cfg), &base).as_deref(),
            None,
        ));
    }

    #[test]
    fn exclude_pattern_is_minimatch_on_absolute_posix_path() {
        let matchers =
            compile_exclude_matchers(Some(&cfg(None, Some(vec!["**/*.test.ts"])))).unwrap();
        let matchers = matchers.unwrap();
        assert!(matchers[0].is_match("/proj/src/foo.test.ts"));
        assert!(!matchers[0].is_match("/proj/src/foo.ts"));
    }

    #[test]
    fn path_entries_get_normalized_like_path_resolve() {
        let base = Utf8PathBuf::from("/proj");
        let prefixes = compile_path_prefixes(
            Some(&cfg(
                Some(vec!["./src/a", "src/b/", "./src/./c/../c"]),
                None,
            )),
            &base,
        )
        .unwrap();
        assert_eq!(prefixes, vec!["/proj/src/a", "/proj/src/b", "/proj/src/c"]);
    }

    #[test]
    fn exclude_pattern_accepts_multiple_entries() {
        let matchers = compile_exclude_matchers(Some(&cfg(
            None,
            Some(vec!["**/*.test.ts", "**/__mocks__/**"]),
        )))
        .unwrap()
        .unwrap();
        assert_eq!(matchers.len(), 2);
        assert!(matchers[1].is_match("/proj/src/__mocks__/foo.ts"));
    }
}
