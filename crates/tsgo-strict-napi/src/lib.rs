//! N-API binding for the tsgo-strict core pipeline.
//!
//! Exposes a single `run` function that mirrors the CLI surface but returns
//! structured diagnostics and per-phase timings instead of text/JSON. The
//! actual work is synchronous and I/O-heavy (2 tsgo subprocesses), so we run
//! it on a blocking tokio thread so the Node event loop is not stalled.

#![deny(clippy::all)]

use camino::Utf8PathBuf;
use napi::bindgen_prelude::*;
use napi::tokio::task;
use napi_derive::napi;
use tsgo_strict_core::diagnostics::Category;
use tsgo_strict_core::options::{CliOptions, Mode};
use tsgo_strict_core::run_structured;

#[napi(object)]
pub struct RunOptions {
    /// Path to the project tsconfig, absolute or relative to `cwd`.
    pub project: Option<String>,
    /// Working directory for binary + tsconfig resolution. Defaults to the
    /// Node process cwd.
    pub cwd: Option<String>,
    /// Plugin name to look up in `compilerOptions.plugins`. Defaults to
    /// `typescript-strict-plugin`.
    pub strict_plugin: Option<String>,
    /// `"exact"` (default) or `"fast"`.
    pub mode: Option<String>,
    /// Explicit file or directory inputs to restrict the check to. Empty means
    /// full project.
    pub subset: Option<Vec<String>>,
    /// Max number of diagnostics to return. `None` / `0` means no cap.
    pub max_diagnostics: Option<u32>,
    /// Forwarded to the tsgo child as `--pretty`. Defaults to `false` so the
    /// diagnostic parser gets clean, predictable output.
    pub pretty: Option<bool>,
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
    pub mode: String,
    pub error_count: u32,
    pub exit_code: i32,
    pub truncated: bool,
    pub diagnostics: Vec<RunDiagnostic>,
    pub timings: Vec<RunTiming>,
}

#[napi]
pub async fn run(options: RunOptions) -> Result<RunResult> {
    let cli = build_cli_options(options).map_err(napi_err)?;
    let outcome = task::spawn_blocking(move || run_structured(&cli))
        .await
        .map_err(|e| napi_err(format!("join error: {e}")))?
        .map_err(|e| napi_err(e.to_string()))?;

    let total = outcome.diagnostics.len();
    let (displayed, truncated) = match outcome.max_diagnostics {
        Some(n) if n > 0 && total > n => (&outcome.diagnostics[..n], true),
        _ => (outcome.diagnostics.as_slice(), false),
    };

    let diagnostics = displayed
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
        mode: outcome.mode.as_str().to_string(),
        error_count: u32::try_from(total).unwrap_or(u32::MAX),
        exit_code: outcome.exit_code,
        truncated,
        diagnostics,
        timings,
    })
}

fn build_cli_options(opts: RunOptions) -> std::result::Result<CliOptions, String> {
    let cwd = match opts.cwd {
        Some(p) => Utf8PathBuf::from(p),
        None => Utf8PathBuf::from(
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_default(),
        ),
    };

    let mode = match opts.mode.as_deref().unwrap_or("exact") {
        "exact" => Mode::Exact,
        "fast" => Mode::Fast,
        other => {
            return Err(format!(
                "invalid mode '{other}' (expected 'exact' or 'fast')"
            ))
        }
    };

    Ok(CliOptions {
        project: opts.project.unwrap_or_else(|| "tsconfig.json".to_string()),
        json: false,
        pretty: opts.pretty,
        trace_performance: true,
        strict_plugin: opts
            .strict_plugin
            .unwrap_or_else(|| "typescript-strict-plugin".to_string()),
        mode,
        max_diagnostics: opts.max_diagnostics.filter(|n| *n > 0).map(|n| n as usize),
        cwd,
        subset_inputs: opts.subset.unwrap_or_default(),
    })
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
            strict_plugin: None,
            mode: None,
            subset: None,
            max_diagnostics: None,
            pretty: None,
        }
    }

    #[test]
    fn defaults_applied_when_options_are_empty() {
        let cli = build_cli_options(base_options()).unwrap();
        assert_eq!(cli.project, "tsconfig.json");
        assert!(!cli.json);
        assert_eq!(cli.pretty, None);
        assert!(cli.trace_performance);
        assert_eq!(cli.strict_plugin, "typescript-strict-plugin");
        assert!(matches!(cli.mode, Mode::Exact));
        assert_eq!(cli.max_diagnostics, None);
        assert_eq!(cli.cwd.as_str(), "/tmp/proj");
        assert!(cli.subset_inputs.is_empty());
    }

    #[test]
    fn mode_fast_is_accepted() {
        let mut opts = base_options();
        opts.mode = Some("fast".into());
        let cli = build_cli_options(opts).unwrap();
        assert!(matches!(cli.mode, Mode::Fast));
    }

    #[test]
    fn invalid_mode_is_rejected() {
        let mut opts = base_options();
        opts.mode = Some("turbo".into());
        let err = build_cli_options(opts).unwrap_err();
        assert!(err.contains("invalid mode 'turbo'"));
    }

    #[test]
    fn zero_max_diagnostics_is_treated_as_uncapped() {
        let mut opts = base_options();
        opts.max_diagnostics = Some(0);
        assert_eq!(build_cli_options(opts).unwrap().max_diagnostics, None);
    }

    #[test]
    fn positive_max_diagnostics_is_preserved() {
        let mut opts = base_options();
        opts.max_diagnostics = Some(42);
        assert_eq!(build_cli_options(opts).unwrap().max_diagnostics, Some(42));
    }

    #[test]
    fn custom_plugin_name_is_forwarded() {
        let mut opts = base_options();
        opts.strict_plugin = Some("my-strict".into());
        assert_eq!(build_cli_options(opts).unwrap().strict_plugin, "my-strict");
    }

    #[test]
    fn subset_inputs_are_forwarded_verbatim() {
        let mut opts = base_options();
        opts.subset = Some(vec!["src/a".into(), "src/b.ts".into()]);
        let cli = build_cli_options(opts).unwrap();
        assert_eq!(cli.subset_inputs, vec!["src/a", "src/b.ts"]);
    }
}
