use camino::Utf8PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Exact,
    Fast,
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Exact => "exact",
            Mode::Fast => "fast",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub project: String,
    pub json: bool,
    pub pretty: Option<bool>,
    pub trace_performance: bool,
    pub strict_plugin: String,
    pub mode: Mode,
    pub max_diagnostics: Option<usize>,
    pub cwd: Utf8PathBuf,
    pub subset_inputs: Vec<String>,
}
