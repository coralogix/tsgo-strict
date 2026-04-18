use crate::diagnostics::Diagnostic;
use crate::options::Mode;
use serde::Serialize;

#[derive(Serialize)]
struct JsonOutput<'a> {
    mode: &'static str,
    #[serde(rename = "errorCount")]
    error_count: usize,
    diagnostics: Vec<DiagOut<'a>>,
    truncated: bool,
}

#[derive(Serialize)]
struct DiagOut<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<u32>,
    category: &'static str,
    code: u32,
    message: &'a str,
    #[serde(rename = "rawLine", skip_serializing_if = "Option::is_none")]
    raw_line: Option<&'a str>,
}

pub struct JsonFormatted {
    pub text: String,
    pub displayed_count: usize,
    pub total_count: usize,
    pub truncated: bool,
}

pub fn format_json_output(
    diagnostics: &[Diagnostic],
    mode: Mode,
    max_diagnostics: Option<usize>,
) -> JsonFormatted {
    let total_count = diagnostics.len();
    let truncated = matches!(max_diagnostics, Some(n) if n > 0 && total_count > n);
    let displayed: Vec<&Diagnostic> = match max_diagnostics {
        Some(n) if truncated => diagnostics.iter().take(n).collect(),
        _ => diagnostics.iter().collect(),
    };

    let diag_out: Vec<DiagOut> = displayed
        .iter()
        .map(|d| DiagOut {
            file: d.file.as_ref().map(|p| p.as_str()),
            line: d.line,
            column: d.column,
            code: d.code,
            category: d.category.as_str(),
            message: &d.message,
            raw_line: d.raw_line.as_deref(),
        })
        .collect();

    let output = JsonOutput {
        mode: mode.as_str(),
        error_count: total_count,
        diagnostics: diag_out,
        truncated,
    };

    let text = serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string());

    JsonFormatted {
        text,
        displayed_count: displayed.len(),
        total_count,
        truncated,
    }
}
