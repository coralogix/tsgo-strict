pub mod parse;
pub mod spawn;
pub mod temp_config;

mod pipeline;

pub use pipeline::{run, RunOutcome};
