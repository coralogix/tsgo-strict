use camino::Utf8PathBuf;
use clap::{Parser, ValueEnum};
use std::io::Write;
use std::process::ExitCode;
use tsgo_strict_core::{run, CliOptions, Mode};

#[derive(Parser, Debug)]
#[command(name = "tsgo-strict", version)]
#[command(about = "High-performance strict-only TypeScript checking with tsgo")]
#[command(override_usage = "tsgo-strict [fileOrGlob ...]")]
struct Cli {
    /// Path to tsconfig
    #[arg(short = 'p', long, default_value = "tsconfig.json")]
    project: String,

    /// Emit JSON diagnostics
    #[arg(long, default_value_t = false)]
    json: bool,

    /// Pretty diagnostic output from backend checker
    #[arg(long)]
    pretty: Option<bool>,

    /// Emit timing breakdown
    #[arg(long, default_value_t = false)]
    trace_performance: bool,

    /// Strict plugin name to inspect in compilerOptions.plugins
    #[arg(long, default_value = "typescript-strict-plugin")]
    strict_plugin: String,

    /// Diagnostic mode
    #[arg(long, value_enum, default_value_t = ModeArg::Exact)]
    mode: ModeArg,

    /// Maximum number of diagnostics to print
    #[arg(long)]
    max_diagnostics: Option<usize>,

    /// Working directory
    #[arg(long)]
    cwd: Option<String>,

    /// Files or globs to restrict the strict check to
    subset: Vec<String>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ModeArg {
    Exact,
    Fast,
}

impl From<ModeArg> for Mode {
    fn from(value: ModeArg) -> Self {
        match value {
            ModeArg::Exact => Mode::Exact,
            ModeArg::Fast => Mode::Fast,
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let cwd_raw = cli.cwd.unwrap_or_else(|| {
        std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    });
    let cwd = match Utf8PathBuf::from(cwd_raw).canonicalize_utf8() {
        Ok(p) => p,
        Err(e) => {
            let _ = writeln!(std::io::stderr(), "tsgo-strict error: invalid --cwd: {e}");
            return ExitCode::from(2);
        }
    };

    let options = CliOptions {
        project: cli.project,
        json: cli.json,
        pretty: cli.pretty,
        trace_performance: cli.trace_performance,
        strict_plugin: cli.strict_plugin,
        mode: cli.mode.into(),
        max_diagnostics: cli.max_diagnostics,
        cwd,
        subset_inputs: cli.subset,
    };

    match run(&options) {
        Ok(outcome) => {
            let _ = std::io::stdout().write_all(outcome.stdout.as_bytes());
            if let Some(t) = outcome.stderr_timings {
                let _ = std::io::stderr().write_all(t.as_bytes());
            }
            ExitCode::from(outcome.exit_code.clamp(0, 255) as u8)
        }
        Err(e) => {
            let _ = writeln!(std::io::stderr(), "tsgo-strict error: {e}");
            ExitCode::from(2)
        }
    }
}
