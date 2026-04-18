use crate::diagnostics::{Category, Diagnostic};
use camino::Utf8PathBuf;
use once_cell::sync::Lazy;
use regex::Regex;

static FORMAT_PAREN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(.*)\((\d+),(\d+)\):\s(error|warning)\sTS(\d+):\s(.*)$").unwrap());
static FORMAT_COLON: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(.*):(\d+):(\d+)\s-\s(error|warning)\sTS(\d+):\s(.*)$").unwrap());
static FORMAT_NO_FILE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(error|warning)\sTS(\d+):\s(.*)$").unwrap());

/// Parse the combined tsgo stdout+stderr into structured diagnostics.
/// Recognizes three output formats (paren, colon, no-file) and aggregates
/// continuation lines into the previous diagnostic's message.
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
    if p.is_absolute() {
        return Utf8PathBuf::try_from(normalize_path(p))
            .unwrap_or_else(|_| Utf8PathBuf::from(file));
    }

    // tsgo emits file paths relative to the config file. When the config
    // lives in a temp directory and the source files have absolute paths in
    // the `files` array, tsgo may emit paths like `../../path/to/src/app.ts`
    // where stripping the `../` prefixes reveals the original absolute path.
    // Try that first before falling back to joining against cwd.
    let stripped = file.trim_start_matches("../");
    let with_slash = format!("/{}", stripped);
    let candidate = std::path::Path::new(&with_slash);
    if candidate.is_absolute() {
        let normalized = normalize_path(candidate);
        if normalized.exists() {
            return Utf8PathBuf::try_from(normalized)
                .unwrap_or_else(|_| Utf8PathBuf::from(file));
        }
    }

    let joined = cwd.as_std_path().join(p);
    let cleaned = normalize_path(&joined);
    Utf8PathBuf::try_from(cleaned).unwrap_or_else(|_| Utf8PathBuf::from(file))
}

/// Collapse `.` and `..` components without touching the filesystem. tsgo
/// emits diagnostic paths relative to its spawn cwd, so when the project lives
/// outside cwd the output is `../../foo/bar.ts`. Joining that to cwd produces
/// `/repo/../../foo/bar.ts`; without normalization it fails to match the
/// absolute target paths we track.
///
/// Rules: `..` pops the previous Normal component, is a no-op against a
/// filesystem root (`/` or a Windows drive prefix), and accumulates as an
/// explicit `..` segment when the accumulated path has no normal component
/// to pop (i.e. the input was already relative and starts with `..`).
fn normalize_path(path: &std::path::Path) -> std::path::PathBuf {
    use std::path::{Component, PathBuf};
    let mut stack: Vec<Component<'_>> = Vec::new();
    for c in path.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => match stack.last() {
                Some(Component::Normal(_)) => {
                    stack.pop();
                }
                Some(Component::RootDir) | Some(Component::Prefix(_)) => {
                    // can't go above the filesystem root; drop this ..
                }
                _ => stack.push(Component::ParentDir),
            },
            other => stack.push(other),
        }
    }
    let mut out = PathBuf::new();
    for c in &stack {
        out.push(c.as_os_str());
    }
    if out.as_os_str().is_empty() {
        out.push(".");
    }
    out
}

fn strip_ansi(value: &str) -> String {
    let bytes = value.as_bytes();
    let stripped = strip_ansi_escapes::strip(bytes);
    String::from_utf8_lossy(&stripped).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_collapses_parent_and_current_segments() {
        let out = normalize_path(std::path::Path::new("/repo/../../tmp/foo/bar.ts"));
        assert_eq!(out, std::path::PathBuf::from("/tmp/foo/bar.ts"));
    }

    #[test]
    fn normalize_keeps_leading_parent_when_no_base_to_pop() {
        let out = normalize_path(std::path::Path::new("../../foo.ts"));
        assert_eq!(out, std::path::PathBuf::from("../../foo.ts"));
    }

    #[test]
    fn normalize_drops_current_dir_segments() {
        let out = normalize_path(std::path::Path::new("/a/./b/./c.ts"));
        assert_eq!(out, std::path::PathBuf::from("/a/b/c.ts"));
    }

    #[test]
    fn normalize_is_noop_on_already_clean_absolute() {
        let out = normalize_path(std::path::Path::new("/a/b/c.ts"));
        assert_eq!(out, std::path::PathBuf::from("/a/b/c.ts"));
    }

    /// Regression for the cross-cwd diagnostic path bug: when tsgo's cwd is the
    /// wrapper's cwd but the project lives outside it, tsgo prints paths like
    /// `../../tmp/foo/bar.ts`. `resolve_relative` must collapse those so the
    /// result matches the absolute target set maintained by the pipeline.
    #[test]
    fn resolve_relative_collapses_parent_dir_from_tsgo_output() {
        let cwd = Utf8PathBuf::from("/home/user/tsgo-strict-plugin");
        let got = resolve_relative(&cwd, "../../../tmp/tsgo-verify/src/a/bad.ts");
        assert_eq!(got, Utf8PathBuf::from("/tmp/tsgo-verify/src/a/bad.ts"));
    }

    #[test]
    fn resolve_relative_passes_through_absolute_paths() {
        let cwd = Utf8PathBuf::from("/home/user/tsgo-strict-plugin");
        let got = resolve_relative(&cwd, "/tmp/verify/src/bad.ts");
        assert_eq!(got, Utf8PathBuf::from("/tmp/verify/src/bad.ts"));
    }

    #[test]
    fn resolve_relative_joins_clean_relative_path() {
        let cwd = Utf8PathBuf::from("/home/user/project");
        let got = resolve_relative(&cwd, "src/a.ts");
        assert_eq!(got, Utf8PathBuf::from("/home/user/project/src/a.ts"));
    }
}
