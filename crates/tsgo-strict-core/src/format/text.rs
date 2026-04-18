use crate::diagnostics::Diagnostic;
use camino::Utf8PathBuf;
use std::cmp::Ordering;

pub struct TextOutput {
    pub text: String,
    pub displayed_count: usize,
    pub total_count: usize,
    pub truncated: bool,
}

pub fn format_text_output(
    diagnostics: &[Diagnostic],
    cwd: &Utf8PathBuf,
    max_diagnostics: Option<usize>,
) -> TextOutput {
    let mut sorted: Vec<&Diagnostic> = diagnostics.iter().collect();
    sorted.sort_by(|a, b| diagnostic_cmp(a, b));

    let total_count = sorted.len();
    let truncated = matches!(max_diagnostics, Some(n) if n > 0 && total_count > n);
    let displayed = match max_diagnostics {
        Some(n) if truncated => &sorted[..n],
        _ => &sorted[..],
    };

    let mut lines: Vec<String> = displayed.iter().map(|d| format_line(d, cwd)).collect();

    if truncated {
        let n = max_diagnostics.unwrap();
        lines.push(format!(
            "... {} additional diagnostics omitted by --max-diagnostics={}",
            total_count - n,
            n
        ));
    }

    lines.push(format!(
        "Found {} strict error{}.",
        total_count,
        if total_count == 1 { "" } else { "s" }
    ));

    TextOutput {
        text: lines.join("\n"),
        displayed_count: displayed.len(),
        total_count,
        truncated,
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
    match pathdiff(file, cwd) {
        Some(rel) if !rel.is_empty() && !rel.starts_with("..") => rel,
        _ => file.to_string(),
    }
}

fn pathdiff(path: &Utf8PathBuf, base: &Utf8PathBuf) -> Option<String> {
    let path_components: Vec<_> = path.components().collect();
    let base_components: Vec<_> = base.components().collect();

    let mut i = 0;
    while i < path_components.len()
        && i < base_components.len()
        && path_components[i] == base_components[i]
    {
        i += 1;
    }

    let mut out: Vec<String> = Vec::new();
    for _ in i..base_components.len() {
        out.push("..".to_string());
    }
    for c in &path_components[i..] {
        out.push(c.as_str().to_string());
    }
    Some(out.join("/"))
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
