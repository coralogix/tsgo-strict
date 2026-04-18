pub mod binary;
pub mod config;
pub mod diagnostics;
pub mod errors;
pub mod files;
pub mod format;
pub mod options;
pub mod perf;
pub mod runner;

pub use errors::Error;
pub use options::CliOptions;
pub use runner::{run, run_structured, RunOutcome, StructuredOutcome};
