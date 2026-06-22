// Copyright 2026 Coralogix Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod pragma;
pub mod project;
pub mod resolve;
pub mod selection;

pub(crate) use project::build_glob_set;
pub use project::{enumerate_project_files, walk_plugin_paths, ProjectScope};
pub use resolve::resolve_subset_inputs;
pub use selection::find_strict_candidates;
