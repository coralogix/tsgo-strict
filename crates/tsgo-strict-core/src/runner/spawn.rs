use crate::diagnostics::Diagnostic;
use crate::errors::Error;
use crate::runner::parse::parse_diagnostics;
use crate::runner::temp_config::{write_temp_config, TempConfig};
use camino::Utf8PathBuf;
use std::collections::HashSet;
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
    /// When set, the temp config will inline all compilerOptions (without
    /// `extends`) and rewrite paths to compensate for the removed `baseUrl`.
    pub effective_base_url: Option<&'a Utf8PathBuf>,
    /// Merged compilerOptions from the full extends chain. Required when
    /// `effective_base_url` is `Some`.
    pub effective_compiler_options: Option<&'a serde_json::Map<String, serde_json::Value>>,
}

pub fn run_tsgo(input: RunInput<'_>) -> Result<TsgoRunResult, Error> {
    let temp: TempConfig = write_temp_config(
        input.project_path,
        input.raw_config,
        input.files,
        input.effective_base_url,
        input.effective_compiler_options,
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

/// Run `tsgo --listFilesOnly` against the original tsconfig and return the
/// compiler's reachable file set as normalized absolute paths.
pub fn query_reachable_files(
    binary: &Utf8PathBuf,
    project_path: &Utf8PathBuf,
    cwd: &Utf8PathBuf,
) -> Result<HashSet<String>, Error> {
    let output = Command::new(binary.as_std_path())
        .args([
            "--listFilesOnly",
            "--pretty",
            "false",
            "-p",
            project_path.as_str(),
        ])
        .current_dir(cwd.as_std_path())
        .env("NO_COLOR", "1")
        .env("FORCE_COLOR", "0")
        .output()
        .map_err(|e| {
            Error::msg(format!(
                "failed to spawn tsgo --listFilesOnly ({}): {}",
                binary, e
            ))
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let exit_code = output.status.code().unwrap_or(1);

    if exit_code != 0 && stdout.trim().is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(Error::TsgoFailed {
            exit_code,
            stderr: stderr.trim().to_string(),
        });
    }

    Ok(parse_list_files_output(&stdout))
}

/// Parse the line-per-file output of `tsgo --listFilesOnly` into a set of
/// normalized paths (forward slashes, lowercase). Filters out node_modules.
pub fn parse_list_files_output(stdout: &str) -> HashSet<String> {
    stdout
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter(|l| !l.contains("/node_modules/") && !l.contains("\\node_modules\\"))
        .map(|l| l.replace('\\', "/").to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list_files_output_empty_input() {
        assert!(parse_list_files_output("").is_empty());
        assert!(parse_list_files_output("   \n  \n").is_empty());
    }

    #[test]
    fn parse_list_files_output_filters_node_modules() {
        let input = "/proj/src/main.ts\n/proj/node_modules/lib/index.d.ts\n/proj/src/util.ts\n";
        let set = parse_list_files_output(input);
        assert_eq!(set.len(), 2);
        assert!(set.contains("/proj/src/main.ts"));
        assert!(set.contains("/proj/src/util.ts"));
    }

    #[test]
    fn parse_list_files_output_normalizes_backslashes_and_case() {
        let input = "C:\\Proj\\Src\\Main.ts\n";
        let set = parse_list_files_output(input);
        assert_eq!(set.len(), 1);
        assert!(set.contains("c:/proj/src/main.ts"));
    }

    #[test]
    fn parse_list_files_output_handles_blank_lines_and_whitespace() {
        let input = "\n  /a/b.ts  \n\n  /c/d.ts\n  \n";
        let set = parse_list_files_output(input);
        assert_eq!(set.len(), 2);
        assert!(set.contains("/a/b.ts"));
        assert!(set.contains("/c/d.ts"));
    }
}
