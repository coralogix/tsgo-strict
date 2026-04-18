use crate::config::StrictPluginConfig;
use crate::errors::Error;
use crate::files::pragma::{detect_pragma, PragmaHint};
use camino::{Utf8Path, Utf8PathBuf};
use globset::{Glob, GlobMatcher};
use rayon::prelude::*;
use rayon::ThreadPool;
use regex::Regex;
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

/// Decide which files qualify as "strict" under the plugin config:
///
/// 1. Read first 4096 bytes, look for pragma:
///    * `@ts-strict-ignore` → excluded, regardless of config.
///    * `@ts-strict` → included, bypasses paths and excludePattern.
/// 2. Otherwise, if plugin config is absent → include.
/// 3. Otherwise, file is included iff its path (relative to `config_dir`,
///    forward-slash normalized) matches any `paths` pattern (or `paths` is
///    empty/absent) AND does not match `excludePattern` (treated as a regex).
pub fn find_strict_candidates(
    project_files: Vec<Utf8PathBuf>,
    plugin_config: Option<&StrictPluginConfig>,
    config_dir: &Utf8PathBuf,
) -> Result<Vec<Utf8PathBuf>, Error> {
    let path_matchers = compile_path_matchers(plugin_config)?;
    let exclude_regex = compile_exclude_regex(plugin_config)?;

    let mut candidates: Vec<Utf8PathBuf> = io_pool().install(|| {
        project_files
            .into_par_iter()
            .filter(|file| {
                is_strict_file(
                    file,
                    plugin_config,
                    config_dir,
                    path_matchers.as_deref(),
                    exclude_regex.as_ref(),
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
        Some(matchers) => matchers.is_empty() || matchers.iter().any(|m| m.is_match(relative_path)),
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

fn compile_exclude_regex(
    plugin_config: Option<&StrictPluginConfig>,
) -> Result<Option<Regex>, Error> {
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
    pathdiff::diff_paths(file.as_std_path(), base.as_std_path())
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|| file.to_string())
}
