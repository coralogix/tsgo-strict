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

use serde_json::Value;
use std::path::PathBuf;

/// When `compilerOptions.types` is absent from the entire extends chain,
/// TypeScript auto-discovers every package under every `typeRoots` directory
/// (default: `node_modules/@types` walking up from the project). Our transient
/// temp config breaks this auto-discovery because tsgo resolves typeRoots
/// relative to the temp directory. This function replicates tsc's
/// `getAutomaticTypeDirectiveNames`: enumerate every subdirectory of every
/// typeRoot and return them as an explicit `types` array that can be injected
/// into the temp config.
///
/// Returns `None` when `types` IS explicitly set (respect the user's choice).
pub fn resolve_auto_type_directives(
    chain: &[(PathBuf, Value)],
    project_dir: &std::path::Path,
) -> Option<Vec<String>> {
    // If `types` is set anywhere in the chain, respect it — no auto-discovery.
    for (_dir, cfg) in chain {
        if let Some(co) = cfg.get("compilerOptions").and_then(|v| v.as_object()) {
            if co.contains_key("types") {
                return None;
            }
        }
    }

    // Resolve effective typeRoots from the chain (last-one-wins).
    let type_roots = resolve_type_roots(chain, project_dir);

    let mut directives: Vec<String> = Vec::new();
    for root in &type_roots {
        if !root.is_dir() {
            continue;
        }
        enumerate_type_root(root, &mut directives);
    }

    directives.sort();
    directives.dedup();

    Some(directives)
}

/// Resolve effective `typeRoots` from the extends chain. If none are specified,
/// use the default: walk up from `project_dir` collecting every
/// `node_modules/@types` directory that exists.
fn resolve_type_roots(chain: &[(PathBuf, Value)], project_dir: &std::path::Path) -> Vec<PathBuf> {
    // Last-one-wins from the chain.
    let mut found: Option<(PathBuf, Vec<String>)> = None;
    for (dir, cfg) in chain {
        if let Some(roots) = cfg
            .get("compilerOptions")
            .and_then(|co| co.get("typeRoots"))
            .and_then(|v| v.as_array())
        {
            let entries: Vec<String> = roots
                .iter()
                .filter_map(|e| e.as_str().map(String::from))
                .collect();
            found = Some((dir.clone(), entries));
        }
    }

    if let Some((config_dir, entries)) = found {
        // Resolve relative paths against the config dir that defined them.
        entries
            .iter()
            .map(|e| {
                let p = std::path::Path::new(e);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    config_dir.join(e)
                }
            })
            .collect()
    } else {
        // Default: walk up from project_dir collecting node_modules/@types.
        default_type_roots(project_dir)
    }
}

/// Walk up from `start` collecting every `node_modules/@types` directory.
fn default_type_roots(start: &std::path::Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let mut dir = Some(start);
    while let Some(d) = dir {
        let candidate = d.join("node_modules").join("@types");
        if candidate.is_dir() {
            roots.push(candidate);
        }
        dir = d.parent();
    }
    roots
}

/// Enumerate one typeRoot directory, adding package names to `out`.
/// Handles scoped packages (e.g. `@testing-library/jest-dom`).
fn enumerate_type_root(root: &std::path::Path, out: &mut Vec<String>) {
    let entries = match std::fs::read_dir(root) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Skip hidden directories.
        if name_str.starts_with('.') {
            continue;
        }

        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        if name_str.starts_with('@') {
            // Scoped package — enumerate one level deeper.
            if let Ok(inner) = std::fs::read_dir(&path) {
                for inner_entry in inner.flatten() {
                    let inner_name = inner_entry.file_name();
                    let inner_name_str = inner_name.to_string_lossy();
                    if inner_name_str.starts_with('.') {
                        continue;
                    }
                    if inner_entry.path().is_dir() {
                        out.push(format!("{}/{}", name_str, inner_name_str));
                    }
                }
            }
        } else {
            out.push(name_str.into_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    fn chain_entry(dir: &str, value: Value) -> (PathBuf, Value) {
        (PathBuf::from(dir), value)
    }

    #[test]
    fn returns_none_when_types_explicitly_set() {
        let chain = vec![chain_entry(
            "/project",
            json!({ "compilerOptions": { "types": ["node"] } }),
        )];
        assert!(resolve_auto_type_directives(&chain, std::path::Path::new("/project")).is_none());
    }

    #[test]
    fn returns_none_when_types_set_in_ancestor() {
        let chain = vec![
            chain_entry("/base", json!({ "compilerOptions": { "types": ["jest"] } })),
            chain_entry("/leaf", json!({})),
        ];
        assert!(resolve_auto_type_directives(&chain, std::path::Path::new("/leaf")).is_none());
    }

    #[test]
    fn returns_none_when_types_empty_array() {
        // An explicit empty `types: []` means "no auto type directives".
        let chain = vec![chain_entry(
            "/project",
            json!({ "compilerOptions": { "types": [] } }),
        )];
        assert!(resolve_auto_type_directives(&chain, std::path::Path::new("/project")).is_none());
    }

    #[test]
    fn discovers_from_custom_type_roots() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        // Create custom typeRoot with packages.
        let custom = root.join("@types");
        fs::create_dir_all(custom.join("build-globals")).unwrap();
        fs::create_dir_all(custom.join("node")).unwrap();

        let chain = vec![chain_entry(
            root.to_str().unwrap(),
            json!({ "compilerOptions": { "typeRoots": ["@types"] } }),
        )];

        let result = resolve_auto_type_directives(&chain, root).unwrap();
        assert_eq!(result, vec!["build-globals", "node"]);
    }

    #[test]
    fn discovers_scoped_packages() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        let types_dir = root.join("node_modules").join("@types");
        fs::create_dir_all(types_dir.join("node")).unwrap();
        fs::create_dir_all(types_dir.join("@testing-library").join("jest-dom")).unwrap();

        let chain = vec![chain_entry(root.to_str().unwrap(), json!({}))];
        let result = resolve_auto_type_directives(&chain, root).unwrap();

        assert!(result.contains(&"@testing-library/jest-dom".to_string()));
        assert!(result.contains(&"node".to_string()));
    }

    #[test]
    fn skips_hidden_dirs() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        let types_dir = root.join("node_modules").join("@types");
        fs::create_dir_all(types_dir.join("node")).unwrap();
        fs::create_dir_all(types_dir.join(".hidden")).unwrap();

        let chain = vec![chain_entry(root.to_str().unwrap(), json!({}))];
        let result = resolve_auto_type_directives(&chain, root).unwrap();
        assert_eq!(result, vec!["node"]);
    }

    #[test]
    fn default_type_roots_walks_up() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        // Create @types at root level only.
        let types_dir = root.join("node_modules").join("@types");
        fs::create_dir_all(types_dir.join("node")).unwrap();

        // Project is nested.
        let project_dir = root.join("packages").join("app");
        fs::create_dir_all(&project_dir).unwrap();

        let chain = vec![chain_entry(project_dir.to_str().unwrap(), json!({}))];
        let result = resolve_auto_type_directives(&chain, &project_dir).unwrap();
        assert!(result.contains(&"node".to_string()));
    }

    #[test]
    fn empty_when_no_type_roots_exist() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        let chain = vec![chain_entry(root.to_str().unwrap(), json!({}))];
        let result = resolve_auto_type_directives(&chain, root).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn deduplicates_and_sorts() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        // Two typeRoots with overlapping packages.
        let root1 = root.join("types1");
        let root2 = root.join("types2");
        fs::create_dir_all(root1.join("node")).unwrap();
        fs::create_dir_all(root1.join("jest")).unwrap();
        fs::create_dir_all(root2.join("node")).unwrap();
        fs::create_dir_all(root2.join("react")).unwrap();

        let chain = vec![chain_entry(
            root.to_str().unwrap(),
            json!({ "compilerOptions": { "typeRoots": ["types1", "types2"] } }),
        )];

        let result = resolve_auto_type_directives(&chain, root).unwrap();
        assert_eq!(result, vec!["jest", "node", "react"]);
    }

    #[test]
    fn type_roots_from_last_config_wins() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        let base_root = root.join("base-types");
        let leaf_root = root.join("leaf-types");
        fs::create_dir_all(base_root.join("base-pkg")).unwrap();
        fs::create_dir_all(leaf_root.join("leaf-pkg")).unwrap();

        let chain = vec![
            chain_entry(
                root.to_str().unwrap(),
                json!({ "compilerOptions": { "typeRoots": ["base-types"] } }),
            ),
            chain_entry(
                root.to_str().unwrap(),
                json!({ "compilerOptions": { "typeRoots": ["leaf-types"] } }),
            ),
        ];

        let result = resolve_auto_type_directives(&chain, root).unwrap();
        // Only leaf-types should be used.
        assert_eq!(result, vec!["leaf-pkg"]);
    }
}
