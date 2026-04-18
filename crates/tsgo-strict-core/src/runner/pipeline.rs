use crate::binary::resolve_tsgo_binary;
use crate::config::{load_project_context, ProjectContext};
use crate::diagnostics::{Category, Diagnostic};
use crate::diff::diff_diagnostics;
use crate::errors::Error;
use crate::files::{enumerate_project_files, find_strict_candidates, resolve_subset_inputs};
#[allow(unused_imports)]
use crate::files::ProjectScope;
use crate::format::{format_json_output, format_text_output};
use crate::options::{CliOptions, Mode};
use crate::perf::Timer;
use crate::runner::spawn::{run_tsgo, RunInput};
use camino::Utf8PathBuf;
use std::collections::HashSet;

pub struct RunOutcome {
    pub stdout: String,
    pub stderr_timings: Option<String>,
    pub exit_code: i32,
}

pub fn run(options: &CliOptions) -> Result<RunOutcome, Error> {
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

    let strict_candidates =
        find_strict_candidates(&project_files, context.strict_plugin_config.as_ref(), &context.config_dir)?;

    let effective_targets = effective_targets(&strict_candidates, &subset_files);
    timer.end("file-resolution");

    if effective_targets.is_empty() {
        return Ok(emit_zero(options, &timer));
    }

    let binary = resolve_tsgo_binary(&options.cwd)?;

    let diagnostics = match options.mode {
        Mode::Fast => run_fast(&context, &binary, &effective_targets, options, &mut timer)?,
        Mode::Exact => run_exact(&context, &binary, &effective_targets, options, &mut timer)?,
    };

    timer.start("formatting");
    let errors: Vec<Diagnostic> = diagnostics
        .into_iter()
        .filter(|d| d.category == Category::Error)
        .collect();

    let body = if options.json {
        format_json_output(&errors, options.mode, options.max_diagnostics).text
    } else {
        format_text_output(&errors, &options.cwd, options.max_diagnostics).text
    };
    timer.end("formatting");

    let stderr_timings = options
        .trace_performance
        .then(|| render_timings(&timer));

    Ok(RunOutcome {
        stdout: format!("{body}\n"),
        stderr_timings,
        exit_code: if errors.is_empty() { 0 } else { 1 },
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
) -> Result<(crate::runner::spawn::TsgoRunResult, crate::runner::spawn::TsgoRunResult), Error> {
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
    let set: HashSet<String> = subset.iter().map(|p| normalize(p)).collect();
    strict_candidates
        .iter()
        .filter(|p| set.contains(&normalize(p)))
        .cloned()
        .collect()
}

fn filter_to_targets(diagnostics: Vec<Diagnostic>, targets: &[Utf8PathBuf]) -> Vec<Diagnostic> {
    let set: HashSet<String> = targets.iter().map(|p| normalize(p)).collect();
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

fn emit_zero(options: &CliOptions, timer: &Timer) -> RunOutcome {
    let body = if options.json {
        format!(
            "{{\n  \"mode\": \"{}\",\n  \"errorCount\": 0,\n  \"diagnostics\": [],\n  \"truncated\": false\n}}\n",
            options.mode.as_str()
        )
    } else {
        "Found 0 strict errors.\n".to_string()
    };

    let stderr_timings = options.trace_performance.then(|| render_timings(timer));

    RunOutcome {
        stdout: body,
        stderr_timings,
        exit_code: 0,
    }
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
