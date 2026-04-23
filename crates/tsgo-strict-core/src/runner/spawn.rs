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
    /// When set, the temp config will inline all compilerOptions (without
    /// `extends`) and rewrite paths to compensate for the removed `baseUrl`.
    pub effective_base_url: Option<&'a Utf8PathBuf>,
    /// Merged compilerOptions from the full extends chain. Required when
    /// `effective_base_url` is `Some`.
    pub effective_compiler_options: Option<&'a serde_json::Map<String, serde_json::Value>>,
    /// Directory of the last config in the extends chain to declare
    /// `typeRoots`. Used to rewrite relative `typeRoots` entries to absolute
    /// paths when writing the transient tsconfig.
    pub effective_type_roots_dir: Option<&'a Utf8PathBuf>,
    /// Auto-discovered type directives to inject as `types` in the temp config.
    pub auto_type_directives: Option<&'a [String]>,
}

pub fn run_tsgo(input: RunInput<'_>) -> Result<TsgoRunResult, Error> {
    let temp: TempConfig = write_temp_config(
        input.project_path,
        input.raw_config,
        input.files,
        input.effective_base_url,
        input.effective_compiler_options,
        input.effective_type_roots_dir,
        input.auto_type_directives,
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
/// compiler's reachable file set. Paths are returned in their original case
/// (only `\` → `/` separator normalization is applied) so that downstream
/// consumers can pass them back to tsgo as `files: [...]` without creating
/// case-duplicate entries (TS1149) on case-insensitive filesystems. Callers
/// that need a containment set should build a lowercased set themselves.
pub fn query_reachable_files(
    binary: &Utf8PathBuf,
    project_path: &Utf8PathBuf,
    cwd: &Utf8PathBuf,
) -> Result<Vec<String>, Error> {
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

/// Parse the line-per-file output of `tsgo --listFilesOnly` into a list of
/// absolute file paths (forward-slash separators, original case preserved).
/// Filters out node_modules and non-file lines — tsgo occasionally
/// interleaves TS5090 / TS5102 config diagnostics on stdout when the
/// tsconfig has issues (e.g. a legacy `baseUrl`), and we must not mistake
/// those for file entries.
pub fn parse_list_files_output(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter(|l| !l.contains("/node_modules/") && !l.contains("\\node_modules\\"))
        .filter(|l| is_listfiles_file_entry(l))
        .map(|l| l.replace('\\', "/"))
        .collect()
}

/// A real file entry is an absolute path (posix `/...` or Windows `X:\`)
/// ending in a TypeScript extension. Diagnostic lines — even when they
/// start with a file-ish path — carry `(line,col): error TSxxxx` decoration
/// so they never end cleanly in a TS extension.
fn is_listfiles_file_entry(line: &str) -> bool {
    let bytes = line.as_bytes();
    let absolute = line.starts_with('/')
        || (bytes.len() >= 3 && bytes[1] == b':' && (bytes[2] == b'\\' || bytes[2] == b'/'));
    if !absolute {
        return false;
    }
    let lower = line.to_ascii_lowercase();
    lower.ends_with(".ts")
        || lower.ends_with(".tsx")
        || lower.ends_with(".cts")
        || lower.ends_with(".mts")
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
        let files = parse_list_files_output(input);
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f == "/proj/src/main.ts"));
        assert!(files.iter().any(|f| f == "/proj/src/util.ts"));
    }

    #[test]
    fn parse_list_files_output_preserves_case_and_normalizes_separators() {
        // Case must be preserved verbatim — passing lowercased paths back to
        // tsgo via `files: [...]` would create case-duplicate program entries
        // on case-insensitive filesystems (TS1149).
        let input = "C:\\Proj\\Src\\Main.ts\n";
        let files = parse_list_files_output(input);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], "C:/Proj/Src/Main.ts");
    }

    #[test]
    fn parse_list_files_output_drops_interleaved_tsgo_diagnostics() {
        // tsgo emits TS5090/TS5102 diagnostics to stdout when the tsconfig has
        // legacy options like `baseUrl`. Those lines look like
        // `libs/foo/tsconfig.json(3,3): error TS5102: ...` — relative-ish,
        // not ending in a TS extension — and must not be mistaken for files.
        let input = "/proj/src/main.ts\n\
                     libs/_data/olly/api-client/tsconfig.lib.json(3,3): error TS5090: non-relative paths are not allowed. did you forget a leading './'?\n\
                     libs/_data/olly/api-client/tsconfig.lib.json(3,3): error TS5102: Option 'baseURL' has been removed. Please remove it from your configuration.\n\
                     use '\"paths\": {\"*\": [\"../../../../*\"]}' instead.\n\
                     /proj/src/util.ts\n";
        let files = parse_list_files_output(input);
        assert_eq!(
            files.len(),
            2,
            "should keep only the two real files, got {files:?}"
        );
        assert!(files.iter().any(|f| f == "/proj/src/main.ts"));
        assert!(files.iter().any(|f| f == "/proj/src/util.ts"));
    }

    #[test]
    fn parse_list_files_output_handles_blank_lines_and_whitespace() {
        let input = "\n  /a/b.ts  \n\n  /c/d.ts\n  \n";
        let files = parse_list_files_output(input);
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f == "/a/b.ts"));
        assert!(files.iter().any(|f| f == "/c/d.ts"));
    }
}
