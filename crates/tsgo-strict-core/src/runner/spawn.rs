use crate::diagnostics::Diagnostic;
use crate::errors::Error;
use crate::runner::parse::parse_diagnostics;
use crate::runner::temp_config::{write_temp_config, TempConfig};
use camino::Utf8PathBuf;
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct TsgoRunResult {
    pub diagnostics: Vec<Diagnostic>,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u128,
}

pub struct RunInput<'a> {
    pub cwd: &'a Utf8PathBuf,
    pub project_path: &'a Utf8PathBuf,
    pub raw_config: &'a serde_json::Value,
    pub files: &'a [Utf8PathBuf],
    pub binary: &'a Utf8PathBuf,
}

pub fn run_tsgo(input: RunInput<'_>) -> Result<TsgoRunResult, Error> {
    let temp: TempConfig = write_temp_config(
        input.cwd,
        input.project_path,
        input.raw_config,
        input.files,
    )?;

    let started = Instant::now();

    // Force `--pretty false` so tsgo's output is one diagnostic per line —
    // no code-frame snippets, no "Found N errors in M files" summary block.
    // Pretty output would be mis-parsed as continuation text of the preceding
    // diagnostic, corrupting the final report.
    let mut cmd = Command::new(input.binary.as_std_path());
    cmd.args(["--noEmit", "--pretty", "false", "-p", temp.path.as_str()])
        .current_dir(input.cwd.as_std_path())
        .env("NO_COLOR", "1")
        .env("FORCE_COLOR", "0");

    let output = cmd
        .output()
        .map_err(|e| Error::msg(format!("failed to spawn tsgo ({}): {}", input.binary, e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let exit_code = output.status.code().unwrap_or(1);
    let duration_ms = started.elapsed().as_millis();

    let diagnostics = parse_diagnostics(&stdout, &stderr, input.cwd);

    Ok(TsgoRunResult {
        diagnostics,
        stdout,
        stderr,
        exit_code,
        duration_ms,
    })
}
