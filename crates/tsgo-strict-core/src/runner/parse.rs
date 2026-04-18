use crate::diagnostics::{Category, Diagnostic};
use camino::Utf8PathBuf;
use once_cell::sync::Lazy;
use regex::Regex;

static FORMAT_PAREN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(.*)\((\d+),(\d+)\):\s(error|warning)\sTS(\d+):\s(.*)$").unwrap()
});
static FORMAT_COLON: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(.*):(\d+):(\d+)\s-\s(error|warning)\sTS(\d+):\s(.*)$").unwrap()
});
static FORMAT_NO_FILE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(error|warning)\sTS(\d+):\s(.*)$").unwrap());

/// Parse the combined tsgo stdout+stderr into structured diagnostics, matching
/// parseDiagnostics + parseDiagnosticLine in src/runner/tsgoRunner.ts.
/// Recognizes three formats and aggregates continuation lines into the
/// previous diagnostic's message.
pub fn parse_diagnostics(stdout: &str, stderr: &str, cwd: &Utf8PathBuf) -> Vec<Diagnostic> {
    let combined = format!("{stdout}\n{stderr}");
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let mut current: Option<Diagnostic> = None;

    for raw_line in combined.lines() {
        let line = strip_ansi(raw_line);
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(diag) = parse_line(trimmed, cwd) {
            if let Some(prev) = current.take() {
                diagnostics.push(prev);
            }
            current = Some(diag);
            continue;
        }

        if let Some(cur) = current.as_mut() {
            cur.message.push('\n');
            cur.message.push_str(trimmed.trim());
        }
    }

    if let Some(last) = current {
        diagnostics.push(last);
    }

    diagnostics
}

fn parse_line(line: &str, cwd: &Utf8PathBuf) -> Option<Diagnostic> {
    if let Some(caps) = FORMAT_PAREN.captures(line) {
        return Some(Diagnostic {
            file: Some(resolve_relative(cwd, &caps[1])),
            line: caps[2].parse().ok(),
            column: caps[3].parse().ok(),
            category: parse_category(&caps[4]),
            code: caps[5].parse().unwrap_or_default(),
            message: caps[6].to_string(),
            raw_line: Some(line.to_string()),
        });
    }

    if let Some(caps) = FORMAT_COLON.captures(line) {
        return Some(Diagnostic {
            file: Some(resolve_relative(cwd, &caps[1])),
            line: caps[2].parse().ok(),
            column: caps[3].parse().ok(),
            category: parse_category(&caps[4]),
            code: caps[5].parse().unwrap_or_default(),
            message: caps[6].to_string(),
            raw_line: Some(line.to_string()),
        });
    }

    if let Some(caps) = FORMAT_NO_FILE.captures(line) {
        return Some(Diagnostic {
            file: None,
            line: None,
            column: None,
            category: parse_category(&caps[1]),
            code: caps[2].parse().unwrap_or_default(),
            message: caps[3].to_string(),
            raw_line: Some(line.to_string()),
        });
    }

    None
}

fn parse_category(raw: &str) -> Category {
    match raw {
        "error" => Category::Error,
        "warning" => Category::Warning,
        _ => Category::Message,
    }
}

fn resolve_relative(cwd: &Utf8PathBuf, file: &str) -> Utf8PathBuf {
    let p = std::path::Path::new(file);
    let joined = if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.as_std_path().join(p)
    };
    let cleaned = normalize_path(&joined);
    Utf8PathBuf::try_from(cleaned).unwrap_or_else(|_| Utf8PathBuf::from(file))
}

/// Collapse `.` and `..` components without touching the filesystem. tsgo
/// emits diagnostic paths relative to its spawn cwd, so when the project lives
/// outside cwd the output is `../../foo/bar.ts`. Joining that to cwd produces
/// `/repo/../../foo/bar.ts`; without normalization it fails to match the
/// absolute target paths we track.
fn normalize_path(path: &std::path::Path) -> std::path::PathBuf {
    use std::path::{Component, PathBuf};
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::ParentDir => {
                if !out.pop() {
                    out.push("..");
                }
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn strip_ansi(value: &str) -> String {
    let bytes = value.as_bytes();
    let stripped = strip_ansi_escapes::strip(bytes);
    String::from_utf8_lossy(&stripped).into_owned()
}
