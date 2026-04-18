use camino::Utf8PathBuf;

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub project: String,
    pub json: bool,
    pub pretty: Option<bool>,
    pub trace_performance: bool,
    pub strict_plugin: String,
    pub max_diagnostics: Option<usize>,
    pub cwd: Utf8PathBuf,
    pub subset_inputs: Vec<String>,
}
