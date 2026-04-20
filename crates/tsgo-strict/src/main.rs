use camino::Utf8PathBuf;
use clap::Parser;
use std::io::Write;
use std::process::ExitCode;
use tsgo_strict_core::{list_files, run_structured, CliOptions};

#[derive(Parser, Debug)]
#[command(name = "tsgo-strict", version)]
#[command(about = "High-performance strict-only TypeScript checking with tsgo")]
#[command(override_usage = "tsgo-strict [fileOrGlob ...]")]
struct Cli {
    /// Path to tsconfig
    #[arg(short = 'p', long, default_value = "tsconfig.json")]
    project: String,

    /// Files or globs to restrict the strict check to
    subset: Vec<String>,

    /// Print the resolved file list and exit without type-checking
    #[arg(long)]
    list_files: bool,

    /// Print phase timing breakdown to stderr
    #[arg(long)]
    timing: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let cwd = match std::env::current_dir()
        .ok()
        .and_then(|p| Utf8PathBuf::from_path_buf(p).ok())
        .and_then(|p| p.canonicalize_utf8().ok())
    {
        Some(p) => p,
        None => {
            let _ = writeln!(std::io::stderr(), "tsgo-strict error: cannot resolve cwd");
            return ExitCode::from(2);
        }
    };

    let options = CliOptions {
        project: cli.project,
        cwd,
        subset_inputs: cli.subset,
        list_files: cli.list_files,
        timing: cli.timing,
    };

    if options.list_files {
        match list_files(&options) {
            Ok(files) => {
                let stdout = std::io::stdout();
                let mut out = stdout.lock();
                for file in &files {
                    let _ = writeln!(out, "{file}");
                }
                return ExitCode::from(0);
            }
            Err(e) => {
                let _ = writeln!(std::io::stderr(), "tsgo-strict error: {e}");
                return ExitCode::from(2);
            }
        }
    }

    match run_structured(&options) {
        Ok(outcome) => {
            let body =
                tsgo_strict_core::format::format_text_output(&outcome.diagnostics, &options.cwd)
                    .text;
            let _ = std::io::stdout().write_all(format!("{body}\n").as_bytes());

            if options.timing {
                print_timings(&outcome.timings);
            }

            ExitCode::from(outcome.exit_code.clamp(0, 255) as u8)
        }
        Err(e) => {
            let _ = writeln!(std::io::stderr(), "tsgo-strict error: {e}");
            ExitCode::from(2)
        }
    }
}

fn print_timings(timings: &[tsgo_strict_core::perf::TimerEntry]) {
    let stderr = std::io::stderr();
    let mut err = stderr.lock();

    let total_ms: u128 = timings.iter().map(|t| t.duration_ms).sum();

    let _ = writeln!(err, "\nTimings:");
    for entry in timings {
        let _ = writeln!(
            err,
            "  {:<17}{}ms",
            format!("{}:", entry.label),
            entry.duration_ms
        );
    }
    let _ = writeln!(err, "  {:<17}{total_ms}ms", "total:");
}
