use crate::diagnostics::Diagnostic;
use std::collections::HashSet;

/// Diff strict-mode diagnostics against a baseline. The key is
/// `path|line|col|code|category|message` with path lowercased and
/// forward-slashed, and message whitespace flattened.
pub fn diff_diagnostics(strict: Vec<Diagnostic>, baseline: &[Diagnostic]) -> Vec<Diagnostic> {
    let baseline_keys: HashSet<String> = baseline.iter().map(normalize_key).collect();
    strict
        .into_iter()
        .filter(|d| !baseline_keys.contains(&normalize_key(d)))
        .collect()
}

pub fn normalize_key(d: &Diagnostic) -> String {
    let file = d
        .file
        .as_ref()
        .map(|p| normalize_path(p.as_str()))
        .unwrap_or_default();
    format!(
        "{}|{}|{}|{}|{}|{}",
        file,
        d.line.unwrap_or(0),
        d.column.unwrap_or(0),
        d.code,
        d.category.as_str(),
        flatten_message(&d.message),
    )
}

fn normalize_path(s: &str) -> String {
    s.replace('\\', "/").to_ascii_lowercase()
}

pub fn flatten_message(message: &str) -> String {
    let mut out = String::with_capacity(message.len());
    let mut last_was_space = false;
    for c in message.chars() {
        if c.is_whitespace() {
            if !last_was_space {
                out.push(' ');
                last_was_space = true;
            }
        } else {
            out.push(c);
            last_was_space = false;
        }
    }
    out.trim().to_string()
}
