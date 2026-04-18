use crate::binary::resolve_tsgo_binary;
use crate::config::{load_project_context, ProjectContext};
use crate::diagnostics::{Category, Diagnostic};
use crate::diff::diff_diagnostics;
use crate::errors::Error;
#[allow(unused_imports)]
use crate::files::ProjectScope;
use crate::files::{enumerate_project_files, find_strict_candidates, resolve_subset_inputs};
use crate::format::{format_json_output, format_text_output};
use crate::options::{CliOptions, Mode};
use crate::perf::{Timer, TimerEntry};
use crate::runner::spawn::{run_tsgo, RunInput};
use camino::Utf8PathBuf;
use std::collections::HashSet;

pub struct RunOutcome {
    pub stdout: String,
    pub stderr_timings: Option<String>,
    pub exit_code: i32,
}

/// Structured result suitable for programmatic consumers (the N-API addon,
/// integration tests). Same pipeline as [`run`], minus text/JSON formatting —
/// the caller renders as needed.
pub struct StructuredOutcome {
    pub mode: Mode,
    pub diagnostics: Vec<Diagnostic>,
    pub timings: Vec<TimerEntry>,
    pub exit_code: i32,
    pub max_diagnostics: Option<usize>,
}

pub fn run_structured(options: &CliOptions) -> Result<StructuredOutcome, Error> {
    let mut timer = Timer::new();

    timer.start("config-load");
    let context = load_project_context(&options.cwd, &options.project, &options.strict_plugin)?;
    timer.end("config-load");

    timer.start("file-resolution");
    let subset_files = resolve_subset_inputs(&options.subset_inputs, &options.cwd)?;

    // When the user passes a subset, the subset IS the scope. Walking the whole
    // project tsconfig include set would be wasted work. When no subset is
    // given, we enumerate from the tsconfig.
    let project_files: Vec<Utf8PathBuf> = if subset_files.is_empty() {
        enumerate_project_files(&context)?.files
    } else {
        subset_files.clone()
    };

    let strict_candidates = find_strict_candidates(
        &project_files,
        context.strict_plugin_config.as_ref(),
        &context.config_dir,
    )?;

    let effective_targets = effective_targets(&strict_candidates, &subset_files);
    timer.end("file-resolution");

    if effective_targets.is_empty() {
        return Ok(StructuredOutcome {
            mode: options.mode,
            diagnostics: Vec::new(),
            timings: timer.entries().to_vec(),
            exit_code: 0,
            max_diagnostics: options.max_diagnostics,
        });
    }

    let binary = resolve_tsgo_binary(&options.cwd)?;

    let diagnostics = match options.mode {
        Mode::Fast => run_fast(&context, &binary, &effective_targets, options, &mut timer)?,
        Mode::Exact => run_exact(&context, &binary, &effective_targets, options, &mut timer)?,
    };

    let errors: Vec<Diagnostic> = diagnostics
        .into_iter()
        .filter(|d| d.category == Category::Error)
        .collect();

    let exit_code = if errors.is_empty() { 0 } else { 1 };

    Ok(StructuredOutcome {
        mode: options.mode,
        diagnostics: errors,
        timings: timer.entries().to_vec(),
        exit_code,
        max_diagnostics: options.max_diagnostics,
    })
}

pub fn run(options: &CliOptions) -> Result<RunOutcome, Error> {
    let structured = run_structured(options)?;

    let mut timer = Timer::from_entries(structured.timings.clone());

    timer.start("formatting");
    let body = if options.json {
        format_json_output(
            &structured.diagnostics,
            structured.mode,
            structured.max_diagnostics,
        )
        .text
    } else {
        format_text_output(
            &structured.diagnostics,
            &options.cwd,
            structured.max_diagnostics,
        )
        .text
    };
    timer.end("formatting");

    let stderr_timings = options.trace_performance.then(|| render_timings(&timer));

    Ok(RunOutcome {
        stdout: format!("{body}\n"),
        stderr_timings,
        exit_code: structured.exit_code,
    })
}

fn run_fast(
    context: &ProjectContext,
    binary: &Utf8PathBuf,
    targets: &[Utf8PathBuf],
    options: &CliOptions,
    timer: &mut Timer,
) -> Result<Vec<Diagnostic>, Error> {
    timer.start("strict-run");
    let result = run_tsgo(RunInput {
        cwd: &options.cwd,
        project_path: &context.project_path,
        raw_config: &context.raw_config,
        files: targets,
        strict_enabled: true,
        pretty: options.pretty,
        binary,
    })?;
    timer.end("strict-run");

    Ok(filter_to_targets(result.diagnostics, targets))
}

fn run_exact(
    context: &ProjectContext,
    binary: &Utf8PathBuf,
    targets: &[Utf8PathBuf],
    options: &CliOptions,
    timer: &mut Timer,
) -> Result<Vec<Diagnostic>, Error> {
    let parallel = std::env::var("TSGO_STRICT_PARALLEL").ok().as_deref() != Some("0");

    let (baseline, strict) = if parallel {
        timer.start("baseline-run");
        timer.start("strict-run");
        let results = run_parallel(context, binary, targets, options)?;
        timer.end("baseline-run");
        timer.end("strict-run");
        results
    } else {
        timer.start("baseline-run");
        let baseline = run_tsgo(RunInput {
            cwd: &options.cwd,
            project_path: &context.project_path,
            raw_config: &context.raw_config,
            files: targets,
            strict_enabled: false,
            pretty: options.pretty,
            binary,
        })?;
        timer.end("baseline-run");
        timer.start("strict-run");
        let strict = run_tsgo(RunInput {
            cwd: &options.cwd,
            project_path: &context.project_path,
            raw_config: &context.raw_config,
            files: targets,
            strict_enabled: true,
            pretty: options.pretty,
            binary,
        })?;
        timer.end("strict-run");
        (baseline, strict)
    };

    let strict_filtered = filter_to_targets(strict.diagnostics, targets);
    let baseline_filtered = filter_to_targets(baseline.diagnostics, targets);

    timer.start("diff");
    let diffed = diff_diagnostics(strict_filtered, &baseline_filtered);
    timer.end("diff");

    Ok(diffed)
}

fn run_parallel(
    context: &ProjectContext,
    binary: &Utf8PathBuf,
    targets: &[Utf8PathBuf],
    options: &CliOptions,
) -> Result<
    (
        crate::runner::spawn::TsgoRunResult,
        crate::runner::spawn::TsgoRunResult,
    ),
    Error,
> {
    let (btx, brx) = std::sync::mpsc::channel();
    let (stx, srx) = std::sync::mpsc::channel();

    std::thread::scope(|scope| -> Result<(), Error> {
        let baseline_input = BuiltRunInput::from(context, binary, targets, options, false);
        let strict_input = BuiltRunInput::from(context, binary, targets, options, true);

        let btx2 = btx.clone();
        scope.spawn(move || {
            let input = baseline_input.as_input();
            let _ = btx2.send(run_tsgo(input));
        });
        let stx2 = stx.clone();
        scope.spawn(move || {
            let input = strict_input.as_input();
            let _ = stx2.send(run_tsgo(input));
        });
        Ok(())
    })?;

    let baseline = brx.recv().map_err(|e| Error::msg(e.to_string()))??;
    let strict = srx.recv().map_err(|e| Error::msg(e.to_string()))??;
    Ok((baseline, strict))
}

/// Owned copy of RunInput fields so we can move it into a scoped thread. The
/// RunInput struct itself holds borrows, which don't survive across thread
/// boundaries the way we want for clarity here.
struct BuiltRunInput {
    cwd: Utf8PathBuf,
    project_path: Utf8PathBuf,
    raw_config: serde_json::Value,
    files: Vec<Utf8PathBuf>,
    strict_enabled: bool,
    pretty: Option<bool>,
    binary: Utf8PathBuf,
}

impl BuiltRunInput {
    fn from(
        context: &ProjectContext,
        binary: &Utf8PathBuf,
        targets: &[Utf8PathBuf],
        options: &CliOptions,
        strict_enabled: bool,
    ) -> Self {
        Self {
            cwd: options.cwd.clone(),
            project_path: context.project_path.clone(),
            raw_config: context.raw_config.clone(),
            files: targets.to_vec(),
            strict_enabled,
            pretty: options.pretty,
            binary: binary.clone(),
        }
    }

    fn as_input(&self) -> RunInput<'_> {
        RunInput {
            cwd: &self.cwd,
            project_path: &self.project_path,
            raw_config: &self.raw_config,
            files: &self.files,
            strict_enabled: self.strict_enabled,
            pretty: self.pretty,
            binary: &self.binary,
        }
    }
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

fn normalize(path: &Utf8PathBuf) -> String {
    path.as_str().replace('\\', "/").to_ascii_lowercase()
}

fn render_timings(timer: &Timer) -> String {
    let entries = timer.entries();
    if entries.is_empty() {
        return String::new();
    }
    let mut out = String::from("Performance timings (ms):\n");
    for e in entries {
        out.push_str(&format!("  {}: {}\n", e.label, e.duration_ms));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::Category;

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
}
