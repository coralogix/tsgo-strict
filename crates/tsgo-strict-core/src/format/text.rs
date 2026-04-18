use crate::diagnostics::Diagnostic;
use camino::Utf8PathBuf;
use std::cmp::Ordering;

pub struct TextOutput {
    pub text: String,
    pub total_count: usize,
}

pub fn format_text_output(diagnostics: &[Diagnostic], cwd: &Utf8PathBuf) -> TextOutput {
    let mut sorted: Vec<&Diagnostic> = diagnostics.iter().collect();
    sorted.sort_by(|a, b| diagnostic_cmp(a, b));

    let total_count = sorted.len();
    let mut lines: Vec<String> = sorted.iter().map(|d| format_line(d, cwd)).collect();

    lines.push(format!(
        "Found {} strict error{}.",
        total_count,
        if total_count == 1 { "" } else { "s" }
    ));

    TextOutput {
        text: lines.join("\n"),
        total_count,
    }
}

fn format_line(d: &Diagnostic, cwd: &Utf8PathBuf) -> String {
    let code = format!("TS{}", d.code);
    let category = d.category.as_str();
    match (&d.file, d.line, d.column) {
        (Some(file), Some(line), Some(col)) => {
            let display = make_relative(file, cwd);
            format!("{display}({line},{col}): {category} {code}: {}", d.message)
        }
        _ => format!("{category} {code}: {}", d.message),
    }
}

fn make_relative(file: &Utf8PathBuf, cwd: &Utf8PathBuf) -> String {
    match pathdiff::diff_paths(file.as_std_path(), cwd.as_std_path()) {
        Some(rel) => {
            let rel = rel.to_string_lossy().replace('\\', "/");
            if rel.is_empty() || rel.starts_with("..") {
                file.to_string()
            } else {
                rel
            }
        }
        None => file.to_string(),
    }
}

fn diagnostic_cmp(a: &Diagnostic, b: &Diagnostic) -> Ordering {
    let af = a.file.as_ref().map(|p| p.as_str()).unwrap_or("");
    let bf = b.file.as_ref().map(|p| p.as_str()).unwrap_or("");
    match af.cmp(bf) {
        Ordering::Equal => {}
        other => return other,
    }
    match a.line.unwrap_or(0).cmp(&b.line.unwrap_or(0)) {
        Ordering::Equal => {}
        other => return other,
    }
    match a.column.unwrap_or(0).cmp(&b.column.unwrap_or(0)) {
        Ordering::Equal => {}
        other => return other,
    }
    match a.code.cmp(&b.code) {
        Ordering::Equal => {}
        other => return other,
    }
    a.message.cmp(&b.message)
}
