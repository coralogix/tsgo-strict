pub mod base_url;
pub mod extends;
pub mod plugin;
pub mod tsconfig;
pub mod type_roots;
pub mod v6_compat;

pub use plugin::StrictPluginConfig;
pub use tsconfig::{load_project_context, ProjectContext, ResolvedField};
