use camino::Utf8PathBuf;

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub project: String,
    pub cwd: Utf8PathBuf,
    pub subset_inputs: Vec<String>,
    pub list_files: bool,
    pub timing: bool,
}
