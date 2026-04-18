//! N-API binding for the tsgo-strict core pipeline.
//!
//! Exposes a single `run` function that mirrors the CLI surface but returns
//! structured diagnostics and per-phase timings instead of text. The actual
//! work is synchronous and I/O-heavy (one tsgo subprocess), so we run it on a
//! blocking tokio thread so the Node event loop is not stalled.

#![deny(clippy::all)]

use camino::Utf8PathBuf;
use napi::bindgen_prelude::*;
use napi::tokio::task;
use napi_derive::napi;
use tsgo_strict_core::diagnostics::Category;
use tsgo_strict_core::options::CliOptions;
use tsgo_strict_core::run_structured;

#[napi(object)]
pub struct RunOptions {
    /// Path to the project tsconfig, absolute or relative to `cwd`.
    pub project: Option<String>,
    /// Working directory for binary + tsconfig resolution. Defaults to the
    /// Node process cwd.
    pub cwd: Option<String>,
    /// Explicit file or directory inputs to restrict the check to. Empty means
    /// full project.
    pub subset: Option<Vec<String>>,
}

#[napi(object)]
pub struct RunDiagnostic {
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub code: u32,
    pub category: String,
    pub message: String,
}

#[napi(object)]
pub struct RunTiming {
    pub label: String,
    pub duration_ms: u32,
}

#[napi(object)]
pub struct RunResult {
    pub error_count: u32,
    pub exit_code: i32,
    pub diagnostics: Vec<RunDiagnostic>,
    pub timings: Vec<RunTiming>,
}

#[napi]
pub async fn run(options: RunOptions) -> Result<RunResult> {
    let cli = build_cli_options(options);
    let outcome = task::spawn_blocking(move || run_structured(&cli))
        .await
        .map_err(|e| napi_err(format!("join error: {e}")))?
        .map_err(|e| napi_err(e.to_string()))?;

    let total = outcome.diagnostics.len();
    let diagnostics = outcome
        .diagnostics
        .iter()
        .map(|d| RunDiagnostic {
            file: d.file.as_ref().map(|p| p.to_string()),
            line: d.line,
            column: d.column,
            code: d.code,
            category: match d.category {
                Category::Error => "error",
                Category::Warning => "warning",
                Category::Message => "message",
            }
            .to_string(),
            message: d.message.clone(),
        })
        .collect();

    let timings = outcome
        .timings
        .into_iter()
        .map(|t| RunTiming {
            label: t.label,
            // u128 ms clamps to u32 only above 49 days of elapsed time — fine
            // for our "report per-phase" usage.
            duration_ms: u32::try_from(t.duration_ms).unwrap_or(u32::MAX),
        })
        .collect();

    Ok(RunResult {
        error_count: u32::try_from(total).unwrap_or(u32::MAX),
        exit_code: outcome.exit_code,
        diagnostics,
        timings,
    })
}

fn build_cli_options(opts: RunOptions) -> CliOptions {
    let cwd = match opts.cwd {
        Some(p) => Utf8PathBuf::from(p),
        None => Utf8PathBuf::from(
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_default(),
        ),
    };

    CliOptions {
        project: opts.project.unwrap_or_else(|| "tsconfig.json".to_string()),
        cwd,
        subset_inputs: opts.subset.unwrap_or_default(),
    }
}

fn napi_err<E: std::fmt::Display>(e: E) -> napi::Error {
    napi::Error::new(Status::GenericFailure, e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_options() -> RunOptions {
        RunOptions {
            project: None,
            cwd: Some("/tmp/proj".to_string()),
            subset: None,
        }
    }

    #[test]
    fn defaults_applied_when_options_are_empty() {
        let cli = build_cli_options(base_options());
        assert_eq!(cli.project, "tsconfig.json");
        assert_eq!(cli.cwd.as_str(), "/tmp/proj");
        assert!(cli.subset_inputs.is_empty());
    }

    #[test]
    fn subset_inputs_are_forwarded_verbatim() {
        let mut opts = base_options();
        opts.subset = Some(vec!["src/a".into(), "src/b.ts".into()]);
        let cli = build_cli_options(opts);
        assert_eq!(cli.subset_inputs, vec!["src/a", "src/b.ts"]);
    }

    #[test]
    fn custom_project_is_forwarded() {
        let mut opts = base_options();
        opts.project = Some("packages/app/tsconfig.json".into());
        let cli = build_cli_options(opts);
        assert_eq!(cli.project, "packages/app/tsconfig.json");
    }
}
