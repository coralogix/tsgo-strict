use crate::binary::resolve_tsgo_binary;
use crate::config::load_project_context;
use crate::diagnostics::{Category, Diagnostic};
use crate::errors::Error;
use crate::files::{
    build_glob_set, enumerate_project_files, find_strict_candidates, resolve_subset_inputs,
    walk_plugin_paths,
};
use crate::format::format_text_output;
use crate::options::CliOptions;
use crate::perf::{Timer, TimerEntry};
use crate::runner::spawn::{query_reachable_files, run_tsgo, RunInput, TsgoRunResult};
use camino::Utf8PathBuf;
use std::collections::HashSet;

const STRICT_PLUGIN_NAME: &str = "typescript-strict-plugin";

pub struct RunOutcome {
    pub stdout: String,
    pub exit_code: i32,
}

/// Structured result suitable for programmatic consumers (the N-API addon,
/// integration tests). Same pipeline as [`run`], minus text formatting —
/// the caller renders as needed.
pub struct StructuredOutcome {
    pub diagnostics: Vec<Diagnostic>,
    pub timings: Vec<TimerEntry>,
    pub exit_code: i32,
}

pub fn run_structured(options: &CliOptions) -> Result<StructuredOutcome, Error> {
    let mut timer = Timer::new();

    timer.start("config-load");
    let context = load_project_context(&options.cwd, &options.project, STRICT_PLUGIN_NAME)?;
    timer.end("config-load");

    let binary = resolve_tsgo_binary(&options.cwd)?;

    timer.start("file-resolution");
    let subset_files = resolve_subset_inputs(&options.subset_inputs, &options.cwd)?;

    let effective_targets =
        resolve_effective_targets(&context, &subset_files, &binary, &options.cwd, &mut timer)?;
    timer.end("file-resolution");

    if effective_targets.is_empty() {
        return Ok(StructuredOutcome {
            diagnostics: Vec::new(),
            timings: timer.entries().to_vec(),
            exit_code: 0,
        });
    }

    timer.start("strict-run");
    let result = run_tsgo(RunInput {
        cwd: &options.cwd,
        project_path: &context.project_path,
        raw_config: &context.raw_config,
        files: &effective_targets,
        binary: &binary,
        effective_base_url: context.effective_base_url.as_ref(),
        effective_compiler_options: context.effective_compiler_options.as_ref(),
        effective_type_roots_dir: context.effective_type_roots_dir.as_ref(),
        auto_type_directives: context.auto_type_directives.as_deref(),
    })?;
    timer.end("strict-run");

    check_tsgo_result(&result)?;

    let diagnostics = filter_to_targets(result.diagnostics, &effective_targets);

    let errors: Vec<Diagnostic> = diagnostics
        .into_iter()
        .filter(|d| d.category == Category::Error)
        .collect();

    let exit_code = if errors.is_empty() { 0 } else { 1 };

    Ok(StructuredOutcome {
        diagnostics: errors,
        timings: timer.entries().to_vec(),
        exit_code,
    })
}

/// Resolve the list of files that would be strict-checked without actually
/// running the type checker. Useful for debugging config/file-resolution.
pub fn list_files(options: &CliOptions) -> Result<Vec<Utf8PathBuf>, Error> {
    let mut timer = Timer::new();

    timer.start("config-load");
    let context = load_project_context(&options.cwd, &options.project, STRICT_PLUGIN_NAME)?;
    timer.end("config-load");

    let binary = resolve_tsgo_binary(&options.cwd)?;

    timer.start("file-resolution");
    let subset_files = resolve_subset_inputs(&options.subset_inputs, &options.cwd)?;

    let mut effective =
        resolve_effective_targets(&context, &subset_files, &binary, &options.cwd, &mut timer)?;
    timer.end("file-resolution");

    effective.sort();
    Ok(effective)
}

/// Compute the set of files tsgo should type-check.
///
/// Always scope to the files named by the tsconfig's `include` / `files`
/// (possibly augmented by plugin `paths` walks), filtered through
/// `find_strict_candidates` (pragma + plugin `paths`/`excludePattern`), then
/// intersected with the reachable set to drop orphans that match the include
/// glob but are never imported. The reachable set is a **filter**, never the
/// source set — feeding it back as the effective targets would pull in
/// cross-lib transitive imports and break per-lib strict-check scoping
/// (every lib would suddenly see errors from every dependency).
fn resolve_effective_targets(
    context: &crate::config::ProjectContext,
    subset_files: &[Utf8PathBuf],
    binary: &Utf8PathBuf,
    cwd: &Utf8PathBuf,
    timer: &mut Timer,
) -> Result<Vec<Utf8PathBuf>, Error> {
    let project_files = resolve_project_files(context, subset_files)?;

    // Reachable is only needed to drop orphans from the enumerate set. Skip
    // the query when a user subset was supplied — they explicitly chose.
    let reachable: Option<Vec<String>> = if subset_files.is_empty() {
        timer.start("reachable-query");
        let r = query_reachable_files(binary, &context.project_path, cwd)?;
        timer.end("reachable-query");
        Some(r)
    } else {
        None
    };

    select_effective_targets(
        project_files,
        reachable.as_deref(),
        context.strict_plugin_config.as_ref(),
        &context.config_dir,
        subset_files,
    )
}

/// Pure selection logic — no IO — so it can be unit-tested by supplying
/// mock `project_files` and `reachable` sets.
///
/// Contract:
/// * `project_files` is the enumerate-based candidate set (already augmented
///   with plugin `paths` walks upstream). This is the *only* source of
///   truth for which files belong to the current strict-check scope.
/// * `reachable`, when `Some`, is an *orphan filter* — candidates not in the
///   reachable set are dropped. It must never be used as the source set.
/// * `subset` shortcut: a caller-supplied file list takes precedence over
///   the reachable orphan filter (the user explicitly chose).
fn select_effective_targets(
    project_files: Vec<Utf8PathBuf>,
    reachable: Option<&[String]>,
    strict_plugin_config: Option<&crate::config::StrictPluginConfig>,
    config_dir: &Utf8PathBuf,
    subset: &[Utf8PathBuf],
) -> Result<Vec<Utf8PathBuf>, Error> {
    let strict_candidates =
        find_strict_candidates(project_files, strict_plugin_config, config_dir)?;
    let candidates = effective_targets(&strict_candidates, subset);

    if subset.is_empty() && !candidates.is_empty() {
        if let Some(reachable_paths) = reachable {
            let reachable_set: HashSet<String> = reachable_paths
                .iter()
                .map(|s| s.to_ascii_lowercase())
                .collect();
            return Ok(candidates
                .into_iter()
                .filter(|f| reachable_set.contains(&normalize(f)))
                .collect());
        }
    }
    Ok(candidates)
}

/// Gather the tsconfig's candidate file set before strict filtering.
/// When the user passes a subset it IS the scope (still honouring tsconfig
/// exclude for things like `test-setup.ts`). Otherwise enumerate from the
/// tsconfig, and augment with plugin `paths` walks when plugin is active.
fn resolve_project_files(
    context: &crate::config::ProjectContext,
    subset_files: &[Utf8PathBuf],
) -> Result<Vec<Utf8PathBuf>, Error> {
    if subset_files.is_empty() {
        let mut files = enumerate_project_files(context)?.files;

        if let Some(ref plugin_cfg) = context.strict_plugin_config {
            if let Some(ref paths) = plugin_cfg.paths {
                if !paths.is_empty() {
                    let (exclude_patterns, exclude_base) = match &context.resolved_exclude {
                        Some(f) => (f.patterns.clone(), f.config_dir.clone()),
                        None => (Vec::new(), context.config_dir.clone()),
                    };
                    let walked = walk_plugin_paths(
                        paths,
                        &context.config_dir,
                        &exclude_patterns,
                        &exclude_base,
                    )?;
                    let existing: HashSet<String> = files.iter().map(normalize).collect();
                    for f in walked {
                        if !existing.contains(&normalize(&f)) {
                            files.push(f);
                        }
                    }
                }
            }
        }

        Ok(files)
    } else {
        let (exclude_patterns, exclude_base) = match context.resolved_exclude {
            Some(ref f) => (f.patterns.clone(), f.config_dir.clone()),
            None => (Vec::new(), context.config_dir.clone()),
        };
        if exclude_patterns.is_empty() {
            Ok(subset_files.to_vec())
        } else {
            let exclude_set = build_glob_set(&exclude_patterns, &exclude_base)?;
            match exclude_set {
                Some(set) => Ok(subset_files
                    .iter()
                    .filter(|f| !set.is_match(f.as_std_path()))
                    .cloned()
                    .collect()),
                None => Ok(subset_files.to_vec()),
            }
        }
    }
}

pub fn run(options: &CliOptions) -> Result<RunOutcome, Error> {
    let structured = run_structured(options)?;
    let body = format_text_output(&structured.diagnostics, &options.cwd).text;
    Ok(RunOutcome {
        stdout: format!("{body}\n"),
        exit_code: structured.exit_code,
    })
}

fn effective_targets(
    strict_candidates: &[Utf8PathBuf],
    subset: &[Utf8PathBuf],
) -> Vec<Utf8PathBuf> {
    if subset.is_empty() {
        return strict_candidates.to_vec();
    }
    let set: HashSet<String> = subset.iter().map(normalize).collect();
    strict_candidates
        .iter()
        .filter(|p| set.contains(&normalize(p)))
        .cloned()
        .collect()
}

fn filter_to_targets(diagnostics: Vec<Diagnostic>, targets: &[Utf8PathBuf]) -> Vec<Diagnostic> {
    let set: HashSet<String> = targets.iter().map(normalize).collect();
    diagnostics
        .into_iter()
        .filter(|d| match &d.file {
            None => true,
            Some(f) => set.contains(&normalize(f)),
        })
        .collect()
}

fn check_tsgo_result(result: &TsgoRunResult) -> Result<(), Error> {
    match result.exit_code {
        0 => Ok(()),
        2 if !result.diagnostics.is_empty() => Ok(()),
        code => {
            let stderr = result.stderr.trim();
            let detail = if stderr.is_empty() {
                result.stdout.trim().to_string()
            } else {
                stderr.to_string()
            };
            Err(Error::TsgoFailed {
                exit_code: code,
                stderr: detail,
            })
        }
    }
}

fn normalize(path: &Utf8PathBuf) -> String {
    path.as_str().replace('\\', "/").to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::Category;
    use crate::runner::spawn::TsgoRunResult;

    fn mk(file: Option<&str>, code: u32) -> Diagnostic {
        Diagnostic {
            file: file.map(Utf8PathBuf::from),
            line: Some(1),
            column: Some(1),
            code,
            category: Category::Error,
            message: format!("e{code}"),
            raw_line: None,
        }
    }

    /// Regression for subset scoping: tsgo compiles every file the parent
    /// tsconfig's `include` reaches (because `files` in a child doesn't
    /// override a parent's `include`), so its diagnostic list contains paths
    /// outside the effective target set. `filter_to_targets` must drop them.
    #[test]
    fn filter_to_targets_drops_paths_outside_the_subset() {
        let targets = vec![Utf8PathBuf::from("/p/src/b/bad.ts")];
        let diagnostics = vec![
            mk(Some("/p/src/a/bad.ts"), 7006),
            mk(Some("/p/src/b/bad.ts"), 7006),
            mk(Some("/p/src/a/other.ts"), 2322),
        ];
        let kept = filter_to_targets(diagnostics, &targets);
        assert_eq!(kept.len(), 1);
        assert_eq!(
            kept[0].file.as_ref().map(|p| p.as_str()),
            Some("/p/src/b/bad.ts")
        );
    }

    #[test]
    fn filter_to_targets_keeps_fileless_diagnostics() {
        let targets = vec![Utf8PathBuf::from("/p/src/b/bad.ts")];
        let diagnostics = vec![mk(None, 5083)];
        let kept = filter_to_targets(diagnostics, &targets);
        assert_eq!(kept.len(), 1);
    }

    #[test]
    fn filter_to_targets_matches_case_insensitively_and_via_forward_slash() {
        let targets = vec![Utf8PathBuf::from("/P/Src/B/Bad.ts")];
        let diagnostics = vec![mk(Some(r"\p\src\b\bad.ts"), 7006)];
        let kept = filter_to_targets(diagnostics, &targets);
        assert_eq!(kept.len(), 1);
    }

    #[test]
    fn effective_targets_empty_subset_returns_all_strict_candidates() {
        let candidates = vec![Utf8PathBuf::from("/p/a.ts"), Utf8PathBuf::from("/p/b.ts")];
        let subset: Vec<Utf8PathBuf> = Vec::new();
        let got = effective_targets(&candidates, &subset);
        assert_eq!(got, candidates);
    }

    #[test]
    fn effective_targets_intersects_subset_with_candidates() {
        let candidates = vec![Utf8PathBuf::from("/p/a.ts"), Utf8PathBuf::from("/p/b.ts")];
        let subset = vec![Utf8PathBuf::from("/p/b.ts"), Utf8PathBuf::from("/p/c.ts")];
        let got = effective_targets(&candidates, &subset);
        assert_eq!(got, vec![Utf8PathBuf::from("/p/b.ts")]);
    }

    // ------------------------------------------------------------------
    // select_effective_targets — regression tests for per-lib scoping.
    //
    // The "reachable" set from `tsgo --listFilesOnly` is the *full
    // compilation graph* and in a monorepo it transitively pulls in files
    // from every dependency lib. If we ever use it as the source set
    // instead of as an orphan filter, a per-lib strict-check would
    // suddenly report errors from unrelated libs. These tests lock the
    // scoping contract down.
    // ------------------------------------------------------------------

    fn cfg_dir() -> Utf8PathBuf {
        Utf8PathBuf::from("/proj/libs/core")
    }

    #[test]
    fn select_effective_targets_no_plugin_no_subset_does_not_leak_reachable_only_files() {
        // Enumerate returned 2 files in THIS lib. Reachable additionally
        // contains a cross-lib file (common in a monorepo where a test
        // imports from a sibling lib). The effective set must be the
        // two in-lib files only — never the cross-lib one.
        let project_files = vec![
            Utf8PathBuf::from("/proj/libs/core/src/a.ts"),
            Utf8PathBuf::from("/proj/libs/core/src/b.ts"),
        ];
        let reachable = vec![
            "/proj/libs/core/src/a.ts".to_string(),
            "/proj/libs/core/src/b.ts".to_string(),
            "/proj/libs/utils/src/other.ts".to_string(), // cross-lib
        ];
        let result = select_effective_targets(
            project_files.clone(),
            Some(&reachable),
            None, // no plugin
            &cfg_dir(),
            &[], // no subset
        )
        .unwrap();
        let mut result_paths: Vec<String> = result.iter().map(|p| p.to_string()).collect();
        result_paths.sort();
        assert_eq!(
            result_paths,
            vec![
                "/proj/libs/core/src/a.ts".to_string(),
                "/proj/libs/core/src/b.ts".to_string(),
            ],
            "must scope to project_files, never use reachable as the source set"
        );
    }

    #[test]
    fn select_effective_targets_drops_orphans_not_in_reachable() {
        // An orphan file (matches include glob but is never imported).
        // Reachable excludes it, so effective targets must drop it.
        let project_files = vec![
            Utf8PathBuf::from("/proj/libs/core/src/used.ts"),
            Utf8PathBuf::from("/proj/libs/core/src/orphan.ts"),
        ];
        let reachable = vec!["/proj/libs/core/src/used.ts".to_string()];
        let result =
            select_effective_targets(project_files, Some(&reachable), None, &cfg_dir(), &[])
                .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].to_string(), "/proj/libs/core/src/used.ts");
    }

    #[test]
    fn select_effective_targets_intersects_case_insensitively() {
        // Filesystem walk gave us original-case paths; tsgo emits
        // lowercased paths on case-insensitive filesystems. The filter
        // must match regardless.
        let project_files = vec![Utf8PathBuf::from("/Proj/Libs/Core/Src/A.ts")];
        let reachable = vec!["/proj/libs/core/src/a.ts".to_string()];
        let result =
            select_effective_targets(project_files, Some(&reachable), None, &cfg_dir(), &[])
                .unwrap();
        assert_eq!(result.len(), 1, "case-mismatched paths must still match");
        assert_eq!(result[0].to_string(), "/Proj/Libs/Core/Src/A.ts");
    }

    #[test]
    fn select_effective_targets_with_subset_skips_reachable_filter() {
        // The user explicitly named files — we must not filter them
        // against reachable (the subset IS the authoritative scope).
        let project_files = vec![Utf8PathBuf::from("/proj/libs/core/src/a.ts")];
        let subset = vec![Utf8PathBuf::from("/proj/libs/core/src/a.ts")];
        // Even with empty reachable, the subset survives.
        let empty_reachable: Vec<String> = Vec::new();
        let result = select_effective_targets(
            project_files,
            Some(&empty_reachable),
            None,
            &cfg_dir(),
            &subset,
        )
        .unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn select_effective_targets_no_reachable_returns_candidates_verbatim() {
        // When the orchestrator didn't query reachable (e.g. user passed
        // subset), we must not silently drop candidates.
        let project_files = vec![Utf8PathBuf::from("/proj/libs/core/src/a.ts")];
        let result = select_effective_targets(project_files, None, None, &cfg_dir(), &[]).unwrap();
        assert_eq!(result.len(), 1);
    }

    fn make_run_result(
        exit_code: i32,
        diagnostics: Vec<Diagnostic>,
        stderr: &str,
    ) -> TsgoRunResult {
        TsgoRunResult {
            exit_code,
            diagnostics,
            stdout: String::new(),
            stderr: stderr.to_string(),
            duration_ms: 0,
        }
    }

    #[test]
    fn check_tsgo_result_exit_0_is_ok() {
        let r = make_run_result(0, vec![], "");
        assert!(check_tsgo_result(&r).is_ok());
    }

    #[test]
    fn check_tsgo_result_exit_2_with_diagnostics_is_ok() {
        let r = make_run_result(2, vec![mk(Some("/p/a.ts"), 7006)], "");
        assert!(check_tsgo_result(&r).is_ok());
    }

    #[test]
    fn check_tsgo_result_exit_1_is_err() {
        let r = make_run_result(1, vec![], "error TS5090: invalid option\n");
        let err = check_tsgo_result(&r).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("exit code 1"), "got: {msg}");
        assert!(msg.contains("TS5090"), "got: {msg}");
    }

    #[test]
    fn check_tsgo_result_exit_99_is_err() {
        let r = make_run_result(99, vec![], "segfault");
        let err = check_tsgo_result(&r).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("exit code 99"), "got: {msg}");
        assert!(msg.contains("segfault"), "got: {msg}");
    }

    #[test]
    fn check_tsgo_result_exit_2_no_diagnostics_is_err() {
        let r = make_run_result(2, vec![], "config error\n");
        let err = check_tsgo_result(&r).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("exit code 2"), "got: {msg}");
    }

    #[test]
    fn normalize_deduplicates_case_and_separators() {
        let a = Utf8PathBuf::from("/proj/src/Foo.ts");
        let b = Utf8PathBuf::from("/proj/src/foo.ts");
        let c = Utf8PathBuf::from(r"\proj\src\foo.ts");
        assert_eq!(normalize(&a), normalize(&b));
        assert_eq!(normalize(&b), normalize(&c));
    }
}
