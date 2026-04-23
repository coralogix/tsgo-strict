#[derive(Debug, Clone)]
pub struct StrictPluginConfig {
    pub name: String,
    pub paths: Option<Vec<String>>,
    pub exclude_pattern: Option<Vec<String>>,
}
