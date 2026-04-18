use camino::Utf8PathBuf;
use clap::Parser;
use std::io::Write;
use std::process::ExitCode;
use tsgo_strict_core::{run, CliOptions};

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
    };

    match run(&options) {
        Ok(outcome) => {
            let _ = std::io::stdout().write_all(outcome.stdout.as_bytes());
            ExitCode::from(outcome.exit_code.clamp(0, 255) as u8)
        }
        Err(e) => {
            let _ = writeln!(std::io::stderr(), "tsgo-strict error: {e}");
            ExitCode::from(2)
        }
    }
}
