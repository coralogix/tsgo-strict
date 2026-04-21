use camino::{Utf8Path, Utf8PathBuf};
use serde_json::Value;
use std::path::PathBuf;

/// Resolve the effective `baseUrl` from the extends chain. Returns the absolute
/// directory that `baseUrl` resolves to (i.e. `config_dir / baseUrl_value`),
/// using last-one-wins semantics matching TypeScript's own merge behavior.
pub fn resolve_effective_base_url(chain: &[(PathBuf, Value)]) -> Option<Utf8PathBuf> {
    let mut result: Option<Utf8PathBuf> = None;
    for (dir, cfg) in chain {
        if let Some(base_url) = cfg
            .get("compilerOptions")
            .and_then(|co| co.get("baseUrl"))
            .and_then(|v| v.as_str())
        {
            let config_dir = match Utf8PathBuf::try_from(dir.clone()) {
                Ok(d) => d,
                Err(_) => continue,
            };
            let resolved = config_dir.join(base_url);
            result = Some(resolved);
        }
    }
    result
}

/// Return the directory of the last config in the extends chain to declare
/// `compilerOptions.typeRoots`. tsc resolves `typeRoots` entries relative to
/// the tsconfig that defined them (not against `baseUrl`), so the transient
/// tsconfig writer needs this anchor to rewrite relative entries correctly
/// when the real tsconfig is nested below its base.
pub fn resolve_effective_type_roots_dir(chain: &[(PathBuf, Value)]) -> Option<Utf8PathBuf> {
    let mut result: Option<Utf8PathBuf> = None;
    for (dir, cfg) in chain {
        if cfg
            .get("compilerOptions")
            .and_then(|co| co.get("typeRoots"))
            .and_then(|v| v.as_array())
            .is_some()
        {
            if let Ok(d) = Utf8PathBuf::try_from(dir.clone()) {
                result = Some(d);
            }
        }
    }
    result
}

/// Shallow-per-key merge of `compilerOptions` across the extends chain.
/// Last-one-wins per key, matching TypeScript's config inheritance.
pub fn resolve_effective_compiler_options(
    chain: &[(PathBuf, Value)],
) -> serde_json::Map<String, Value> {
    let mut merged = serde_json::Map::new();
    for (_dir, cfg) in chain {
        if let Some(co) = cfg.get("compilerOptions").and_then(|v| v.as_object()) {
            for (key, value) in co {
                merged.insert(key.clone(), value.clone());
            }
        }
    }
    merged
}

/// Remove `baseUrl` from compiler options and rewrite `paths` entries to
/// absolute filesystem paths anchored at `base_url_dir`, so they resolve
/// correctly regardless of where the transient tsconfig is written.
///
/// `typeRoots` is deliberately not touched here — tsc resolves `typeRoots`
/// against the tsconfig that *defined* them, not against `baseUrl`. The temp
/// config writer rewrites `typeRoots` separately via
/// [`rewrite_relative_type_roots`] with the correct anchor directory.
///
/// - `compiler_options`: the merged compilerOptions map (will be mutated)
/// - `base_url_dir`: the absolute directory that `baseUrl` resolved to
pub fn normalize_base_url(
    compiler_options: &mut serde_json::Map<String, Value>,
    base_url_dir: &Utf8Path,
) {
    compiler_options.remove("baseUrl");

    // Rewrite `paths`
    if let Some(paths_val) = compiler_options.get_mut("paths") {
        if let Some(paths_obj) = paths_val.as_object_mut() {
            for (_key, patterns) in paths_obj.iter_mut() {
                if let Some(arr) = patterns.as_array_mut() {
                    for entry in arr.iter_mut() {
                        if let Some(s) = entry.as_str() {
                            *entry = Value::String(resolve_path_entry(s, base_url_dir));
                        }
                    }
                }
            }
        }
    } else {
        // baseUrl without paths: synthesize wildcard mapping to preserve
        // bare-module resolution behavior
        let mut wildcard = serde_json::Map::new();
        let pattern = format!("{}/*", base_url_dir);
        wildcard.insert("*".to_string(), Value::Array(vec![Value::String(pattern)]));
        compiler_options.insert("paths".to_string(), Value::Object(wildcard));
    }
}

/// Resolve a single path/typeRoots entry against `base_url_dir` to produce an
/// absolute path. Already-absolute entries are returned as-is.
fn resolve_path_entry(entry: &str, base_url_dir: &Utf8Path) -> String {
    if Utf8Path::new(entry).is_absolute() {
        return entry.to_string();
    }
    base_url_dir.join(entry).to_string()
}

/// Rewrite relative entries in `paths` to absolute paths anchored at
/// `paths_anchor`. Used when writing the transient tsconfig in the no-baseUrl
/// branch — because the temp file lives in `<project>/.tsgo-strict-tmp/run-XXX/`,
/// any relative paths copied verbatim from the leaf's compilerOptions would
/// resolve two directories too deep.
pub fn rewrite_relative_paths(
    compiler_options: &mut serde_json::Map<String, Value>,
    paths_anchor: &Utf8Path,
) {
    if let Some(paths_val) = compiler_options.get_mut("paths") {
        if let Some(paths_obj) = paths_val.as_object_mut() {
            for (_key, patterns) in paths_obj.iter_mut() {
                if let Some(arr) = patterns.as_array_mut() {
                    for entry in arr.iter_mut() {
                        if let Some(s) = entry.as_str() {
                            *entry = Value::String(resolve_path_entry(s, paths_anchor));
                        }
                    }
                }
            }
        }
    }
}

/// Rewrite relative entries in `typeRoots` to absolute paths anchored at
/// `type_roots_anchor` — the directory of the config that last defined
/// `typeRoots` in the extends chain. tsc resolves `typeRoots` relative to the
/// tsconfig that defined them (not against `baseUrl`), and the temp config
/// sitting in `<project>/.tsgo-strict-tmp/run-XXX/` otherwise resolves them
/// two directories deeper than intended.
pub fn rewrite_relative_type_roots(
    compiler_options: &mut serde_json::Map<String, Value>,
    type_roots_anchor: &Utf8Path,
) {
    if let Some(type_roots_val) = compiler_options.get_mut("typeRoots") {
        if let Some(arr) = type_roots_val.as_array_mut() {
            for entry in arr.iter_mut() {
                if let Some(s) = entry.as_str() {
                    *entry = Value::String(resolve_path_entry(s, type_roots_anchor));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn base_url_dot_at_root_prefixes_paths() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("."));
        co.insert(
            "paths".to_string(),
            json!({
                "@app/*": ["src/app/*"],
                "@lib/*": ["src/lib/*"]
            }),
        );

        normalize_base_url(&mut co, Utf8Path::new("/project"));

        assert!(!co.contains_key("baseUrl"));
        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["@app/*"], json!(["/project/src/app/*"]));
        assert_eq!(paths["@lib/*"], json!(["/project/src/lib/*"]));
    }

    #[test]
    fn base_url_src_prefixes_paths() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("./src"));
        co.insert(
            "paths".to_string(),
            json!({
                "@app/*": ["app/*"],
                "@lib/*": ["lib/*"]
            }),
        );

        normalize_base_url(&mut co, Utf8Path::new("/project/src"));

        assert!(!co.contains_key("baseUrl"));
        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["@app/*"], json!(["/project/src/app/*"]));
        assert_eq!(paths["@lib/*"], json!(["/project/src/lib/*"]));
    }

    #[test]
    fn relative_paths_resolved_to_absolute() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("."));
        co.insert(
            "paths".to_string(),
            json!({
                "@foo/*": ["./src/foo/*"],
                "@bar/*": ["../shared/bar/*"]
            }),
        );

        normalize_base_url(&mut co, Utf8Path::new("/project"));

        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["@foo/*"], json!(["/project/./src/foo/*"]));
        assert_eq!(paths["@bar/*"], json!(["/project/../shared/bar/*"]));
    }

    #[test]
    fn wildcard_positions_preserved() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("."));
        co.insert(
            "paths".to_string(),
            json!({
                "*": ["types/*", "fallback/*"]
            }),
        );

        normalize_base_url(&mut co, Utf8Path::new("/project"));

        let paths = co["paths"].as_object().unwrap();
        assert_eq!(
            paths["*"],
            json!(["/project/types/*", "/project/fallback/*"])
        );
    }

    #[test]
    fn base_url_without_paths_synthesizes_wildcard() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("."));

        normalize_base_url(&mut co, Utf8Path::new("/project"));

        assert!(!co.contains_key("baseUrl"));
        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["*"], json!(["/project/*"]));
    }

    #[test]
    fn base_url_src_without_paths_synthesizes_wildcard_with_prefix() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("./src"));

        normalize_base_url(&mut co, Utf8Path::new("/project/src"));

        assert!(!co.contains_key("baseUrl"));
        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["*"], json!(["/project/src/*"]));
    }

    #[test]
    fn normalize_base_url_leaves_type_roots_untouched() {
        // tsc resolves typeRoots against the config that defined them, not
        // against baseUrl. normalize_base_url used to rewrite typeRoots too,
        // but that was wrong for nested configs where typeRoots lives in a
        // leaf whose dir differs from baseUrl's dir.
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("."));
        co.insert(
            "typeRoots".to_string(),
            json!(["node_modules/@types", "custom-types"]),
        );

        normalize_base_url(&mut co, Utf8Path::new("/project"));

        let roots = co["typeRoots"].as_array().unwrap();
        assert_eq!(roots[0], json!("node_modules/@types"));
        assert_eq!(roots[1], json!("custom-types"));
    }

    #[test]
    fn rewrite_relative_type_roots_against_defining_dir() {
        let mut co = serde_json::Map::new();
        co.insert(
            "typeRoots".to_string(),
            json!(["../../node_modules/@types", "custom-types"]),
        );

        // typeRoots was defined in /workspace/libs/testing/tsconfig.lib.json
        rewrite_relative_type_roots(&mut co, Utf8Path::new("/workspace/libs/testing"));

        let roots = co["typeRoots"].as_array().unwrap();
        assert_eq!(
            roots[0],
            json!("/workspace/libs/testing/../../node_modules/@types")
        );
        assert_eq!(roots[1], json!("/workspace/libs/testing/custom-types"));
    }

    #[test]
    fn rewrite_relative_type_roots_leaves_absolute_alone() {
        let mut co = serde_json::Map::new();
        co.insert("typeRoots".to_string(), json!(["/abs/types", "./relative"]));

        rewrite_relative_type_roots(&mut co, Utf8Path::new("/anchor"));

        let roots = co["typeRoots"].as_array().unwrap();
        assert_eq!(roots[0], json!("/abs/types"));
        assert_eq!(roots[1], json!("/anchor/./relative"));
    }

    #[test]
    fn rewrite_relative_type_roots_is_noop_when_unset() {
        let mut co = serde_json::Map::new();
        co.insert("target".to_string(), json!("ES2022"));
        rewrite_relative_type_roots(&mut co, Utf8Path::new("/anchor"));
        assert!(co.get("typeRoots").is_none());
    }

    #[test]
    fn resolve_effective_type_roots_dir_uses_last_defining_config() {
        let chain = vec![
            (
                PathBuf::from("/workspace"),
                json!({ "compilerOptions": { "typeRoots": ["node_modules/@types"] } }),
            ),
            (
                PathBuf::from("/workspace/libs/testing"),
                json!({ "compilerOptions": { "typeRoots": ["../../node_modules"] } }),
            ),
        ];
        let result = resolve_effective_type_roots_dir(&chain);
        assert_eq!(result, Some(Utf8PathBuf::from("/workspace/libs/testing")));
    }

    #[test]
    fn resolve_effective_type_roots_dir_inherits_when_leaf_omits() {
        let chain = vec![
            (
                PathBuf::from("/workspace"),
                json!({ "compilerOptions": { "typeRoots": ["node_modules/@types"] } }),
            ),
            (
                PathBuf::from("/workspace/libs/testing"),
                json!({ "compilerOptions": {} }),
            ),
        ];
        let result = resolve_effective_type_roots_dir(&chain);
        assert_eq!(result, Some(Utf8PathBuf::from("/workspace")));
    }

    #[test]
    fn resolve_effective_type_roots_dir_none_when_absent() {
        let chain = vec![(PathBuf::from("/workspace"), json!({}))];
        assert!(resolve_effective_type_roots_dir(&chain).is_none());
    }

    #[test]
    fn already_absolute_paths_unchanged() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("."));
        co.insert(
            "paths".to_string(),
            json!({
                "@abs/*": ["/absolute/path/*"]
            }),
        );

        normalize_base_url(&mut co, Utf8Path::new("/project"));

        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["@abs/*"], json!(["/absolute/path/*"]));
    }

    #[test]
    fn resolve_effective_base_url_from_ancestor() {
        let chain = vec![
            (
                PathBuf::from("/base"),
                json!({ "compilerOptions": { "baseUrl": "." } }),
            ),
            (PathBuf::from("/leaf"), json!({})),
        ];
        let result = resolve_effective_base_url(&chain);
        assert_eq!(result, Some(Utf8PathBuf::from("/base/.")));
    }

    #[test]
    fn resolve_effective_base_url_leaf_overrides() {
        let chain = vec![
            (
                PathBuf::from("/base"),
                json!({ "compilerOptions": { "baseUrl": "." } }),
            ),
            (
                PathBuf::from("/leaf"),
                json!({ "compilerOptions": { "baseUrl": "./src" } }),
            ),
        ];
        let result = resolve_effective_base_url(&chain);
        assert_eq!(result, Some(Utf8PathBuf::from("/leaf/./src")));
    }

    #[test]
    fn resolve_effective_base_url_none_when_absent() {
        let chain = vec![
            (PathBuf::from("/base"), json!({})),
            (PathBuf::from("/leaf"), json!({})),
        ];
        assert!(resolve_effective_base_url(&chain).is_none());
    }

    #[test]
    fn resolve_effective_compiler_options_merges_per_key() {
        let chain = vec![
            (
                PathBuf::from("/base"),
                json!({ "compilerOptions": { "target": "ES2020", "strict": false } }),
            ),
            (
                PathBuf::from("/leaf"),
                json!({ "compilerOptions": { "strict": true, "outDir": "./dist" } }),
            ),
        ];
        let merged = resolve_effective_compiler_options(&chain);
        assert_eq!(merged["target"], json!("ES2020"));
        assert_eq!(merged["strict"], json!(true));
        assert_eq!(merged["outDir"], json!("./dist"));
    }
}
