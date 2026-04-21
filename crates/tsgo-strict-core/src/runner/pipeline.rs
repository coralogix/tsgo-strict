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

    timer.start("file-resolution");
    let subset_files = resolve_subset_inputs(&options.subset_inputs, &options.cwd)?;

    // When the user passes a subset, the subset IS the scope. Walking the whole
    // project tsconfig include set would be wasted work. When no subset is
    // given, we enumerate from the tsconfig.
    let project_files: Vec<Utf8PathBuf> = if subset_files.is_empty() {
        let mut files = enumerate_project_files(&context)?.files;

        // When plugin `paths` are configured, walk those directories to discover
        // transitively-imported files that may not appear in `files: [...]`.
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

        files
    } else {
        // Even for subset files, honour the tsconfig exclude so that
        // explicitly excluded files (e.g. test-setup.ts) are not checked.
        let (exclude_patterns, exclude_base) = match context.resolved_exclude {
            Some(ref f) => (f.patterns.clone(), f.config_dir.clone()),
            None => (Vec::new(), context.config_dir.clone()),
        };
        if exclude_patterns.is_empty() {
            subset_files.clone()
        } else {
            let exclude_set = build_glob_set(&exclude_patterns, &exclude_base)?;
            match exclude_set {
                Some(set) => subset_files
                    .iter()
                    .filter(|f| !set.is_match(f.as_std_path()))
                    .cloned()
                    .collect(),
                None => subset_files.clone(),
            }
        }
    };

    let strict_candidates = find_strict_candidates(
        project_files,
        context.strict_plugin_config.as_ref(),
        &context.config_dir,
    )?;

    let effective_targets = effective_targets(&strict_candidates, &subset_files);
    timer.end("file-resolution");

    if effective_targets.is_empty() {
        return Ok(StructuredOutcome {
            diagnostics: Vec::new(),
            timings: timer.entries().to_vec(),
            exit_code: 0,
        });
    }

    let binary = resolve_tsgo_binary(&options.cwd)?;

    // Filter to only files reachable from the tsconfig's entry points. This
    // excludes orphan files that live under plugin `paths` but are never
    // imported. Skip when the user passed a subset — they explicitly chose.
    let effective_targets = if subset_files.is_empty() {
        timer.start("reachable-query");
        let reachable = query_reachable_files(&binary, &context.project_path, &options.cwd)?;
        timer.end("reachable-query");
        effective_targets
            .into_iter()
            .filter(|f| reachable.contains(&normalize(f)))
            .collect()
    } else {
        effective_targets
    };

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

    timer.start("file-resolution");
    let subset_files = resolve_subset_inputs(&options.subset_inputs, &options.cwd)?;

    let project_files: Vec<Utf8PathBuf> = if subset_files.is_empty() {
        let mut files = enumerate_project_files(&context)?.files;

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

        files
    } else {
        let (exclude_patterns, exclude_base) = match context.resolved_exclude {
            Some(ref f) => (f.patterns.clone(), f.config_dir.clone()),
            None => (Vec::new(), context.config_dir.clone()),
        };
        if exclude_patterns.is_empty() {
            subset_files.clone()
        } else {
            let exclude_set = build_glob_set(&exclude_patterns, &exclude_base)?;
            match exclude_set {
                Some(set) => subset_files
                    .iter()
                    .filter(|f| !set.is_match(f.as_std_path()))
                    .cloned()
                    .collect(),
                None => subset_files.clone(),
            }
        }
    };

    let strict_candidates = find_strict_candidates(
        project_files,
        context.strict_plugin_config.as_ref(),
        &context.config_dir,
    )?;

    let mut effective = effective_targets(&strict_candidates, &subset_files);
    timer.end("file-resolution");

    // Filter to only files reachable from the tsconfig's entry points.
    if subset_files.is_empty() && !effective.is_empty() {
        let binary = resolve_tsgo_binary(&options.cwd)?;
        timer.start("reachable-query");
        let reachable = query_reachable_files(&binary, &context.project_path, &options.cwd)?;
        timer.end("reachable-query");
        effective.retain(|f| reachable.contains(&normalize(f)));
    }

    effective.sort();
    Ok(effective)
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
