pub mod pragma;
pub mod project;
pub mod resolve;
pub mod selection;

pub(crate) use project::build_glob_set;
pub use project::{enumerate_project_files, ProjectScope};
pub use resolve::resolve_subset_inputs;
pub use selection::find_strict_candidates;
