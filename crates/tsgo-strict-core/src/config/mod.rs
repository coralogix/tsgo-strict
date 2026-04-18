pub mod extends;
pub mod plugin;
pub mod tsconfig;

pub use plugin::StrictPluginConfig;
pub use tsconfig::{load_project_context, ProjectContext};
