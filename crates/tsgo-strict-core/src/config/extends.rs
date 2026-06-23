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

use crate::errors::Error;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::tsconfig::read_raw_tsconfig;

/// Walk the `extends` chain starting at `project_path`, returning
/// `(config_dir, raw_json)` pairs ordered from root-most ancestor (first) to
/// the project itself (last). `config_dir` is the parent directory of each
/// config file — needed so that inherited `include`/`exclude`/`files` globs
/// can be resolved relative to the config that defined them.
pub fn load_extends_chain(project_path: &Path) -> Result<Vec<(PathBuf, serde_json::Value)>, Error> {
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut chain: Vec<(PathBuf, serde_json::Value)> = Vec::new();

    let mut current: Option<PathBuf> = Some(project_path.to_path_buf());
    while let Some(path) = current {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
        if !visited.insert(canonical.clone()) {
            break;
        }

        let value = read_raw_tsconfig(&path)?;

        let ext = value
            .get("extends")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let config_dir = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        chain.push((config_dir, value));

        current = match ext {
            Some(spec) => Some(resolve_extends_path(
                path.parent().unwrap_or_else(|| Path::new(".")),
                &spec,
            )?),
            None => None,
        };
    }

    chain.reverse();
    Ok(chain)
}

fn resolve_extends_path(base_dir: &Path, spec: &str) -> Result<PathBuf, Error> {
    if spec.starts_with('.') || spec.starts_with('/') {
        return Ok(ensure_json_extension(base_dir.join(spec)));
    }

    if let Some(resolved) = resolve_node_module(base_dir, spec) {
        return Ok(resolved);
    }

    Err(Error::ExtendsNotFound {
        target: spec.to_string(),
        from: base_dir.to_path_buf(),
    })
}

fn ensure_json_extension(path: PathBuf) -> PathBuf {
    if path.extension().is_some() {
        return path;
    }
    let mut s = path.into_os_string();
    s.push(".json");
    PathBuf::from(s)
}

fn resolve_node_module(base_dir: &Path, spec: &str) -> Option<PathBuf> {
    let mut dir: Option<&Path> = Some(base_dir);
    while let Some(d) = dir {
        let candidate = d.join("node_modules").join(spec);
        if let Some(resolved) = resolve_module_target(&candidate) {
            return Some(resolved);
        }

        let with_json = ensure_json_extension(candidate);
        if with_json.is_file() {
            return Some(with_json);
        }

        dir = d.parent();
    }
    None
}

fn resolve_module_target(candidate: &Path) -> Option<PathBuf> {
    if candidate.is_file() {
        return Some(candidate.to_path_buf());
    }
    if candidate.is_dir() {
        let pkg = candidate.join("package.json");
        if pkg.is_file() {
            if let Ok(raw) = std::fs::read_to_string(&pkg) {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) {
                    if let Some(rel) = value.get("tsconfig").and_then(|v| v.as_str()) {
                        let target = candidate.join(rel);
                        if target.is_file() {
                            return Some(target);
                        }
                    }
                }
            }
        }
        let default = candidate.join("tsconfig.json");
        if default.is_file() {
            return Some(default);
        }
    }
    None
}
