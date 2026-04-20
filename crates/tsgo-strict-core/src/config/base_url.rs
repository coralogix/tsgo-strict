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

/// Remove `baseUrl` from compiler options and rewrite `paths` and `typeRoots`
/// entries so they resolve correctly without it.
///
/// - `compiler_options`: the merged compilerOptions map (will be mutated)
/// - `base_url_dir`: the absolute directory that `baseUrl` resolved to
/// - `tsconfig_dir`: the directory of the config whose `paths`/`typeRoots` are
///   being resolved relative to (typically the leaf project's config dir)
pub fn normalize_base_url(
    compiler_options: &mut serde_json::Map<String, Value>,
    base_url_dir: &Utf8Path,
    tsconfig_dir: &Utf8Path,
) {
    compiler_options.remove("baseUrl");

    let rel_prefix = relative_prefix(tsconfig_dir, base_url_dir);

    // Rewrite `paths`
    if let Some(paths_val) = compiler_options.get_mut("paths") {
        if let Some(paths_obj) = paths_val.as_object_mut() {
            for (_key, patterns) in paths_obj.iter_mut() {
                if let Some(arr) = patterns.as_array_mut() {
                    for entry in arr.iter_mut() {
                        if let Some(s) = entry.as_str() {
                            if let Some(rewritten) = rewrite_path_entry(s, &rel_prefix) {
                                *entry = Value::String(rewritten);
                            }
                        }
                    }
                }
            }
        }
    } else {
        // baseUrl without paths: synthesize wildcard mapping to preserve
        // bare-module resolution behavior
        let mut wildcard = serde_json::Map::new();
        let pattern = if rel_prefix == "." {
            "./*".to_string()
        } else {
            format!("{}/*", rel_prefix)
        };
        wildcard.insert("*".to_string(), Value::Array(vec![Value::String(pattern)]));
        compiler_options.insert("paths".to_string(), Value::Object(wildcard));
    }

    // Rewrite `typeRoots`
    if let Some(type_roots_val) = compiler_options.get_mut("typeRoots") {
        if let Some(arr) = type_roots_val.as_array_mut() {
            for entry in arr.iter_mut() {
                if let Some(s) = entry.as_str() {
                    if let Some(rewritten) = rewrite_path_entry(s, &rel_prefix) {
                        *entry = Value::String(rewritten);
                    }
                }
            }
        }
    }
}

/// Compute a relative path prefix from `from` to `to`, using forward slashes.
fn relative_prefix(from: &Utf8Path, to: &Utf8Path) -> String {
    // Use pathdiff for reliable relative path computation
    match pathdiff::diff_utf8_paths(to, from) {
        Some(rel) => {
            let s = rel.as_str().replace('\\', "/");
            if s.is_empty() {
                ".".to_string()
            } else {
                s
            }
        }
        None => to.as_str().to_string(),
    }
}

/// Rewrite a single path/typeRoots entry by prefixing it with the relative
/// path from the tsconfig dir to the baseUrl dir. Entries that already start
/// with `./` or `../` are assumed to be intentionally relative and are left
/// alone.
fn rewrite_path_entry(entry: &str, rel_prefix: &str) -> Option<String> {
    if entry.starts_with("./") || entry.starts_with("../") {
        return None;
    }
    if rel_prefix == "." {
        Some(format!("./{}", entry))
    } else {
        Some(format!("{}/{}", rel_prefix, entry))
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

        normalize_base_url(
            &mut co,
            Utf8Path::new("/project"),
            Utf8Path::new("/project"),
        );

        assert!(!co.contains_key("baseUrl"));
        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["@app/*"], json!(["./src/app/*"]));
        assert_eq!(paths["@lib/*"], json!(["./src/lib/*"]));
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

        normalize_base_url(
            &mut co,
            Utf8Path::new("/project/src"),
            Utf8Path::new("/project"),
        );

        assert!(!co.contains_key("baseUrl"));
        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["@app/*"], json!(["src/app/*"]));
        assert_eq!(paths["@lib/*"], json!(["src/lib/*"]));
    }

    #[test]
    fn already_relative_paths_not_double_prefixed() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("."));
        co.insert(
            "paths".to_string(),
            json!({
                "@foo/*": ["./src/foo/*"],
                "@bar/*": ["../shared/bar/*"]
            }),
        );

        normalize_base_url(
            &mut co,
            Utf8Path::new("/project"),
            Utf8Path::new("/project"),
        );

        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["@foo/*"], json!(["./src/foo/*"]));
        assert_eq!(paths["@bar/*"], json!(["../shared/bar/*"]));
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

        normalize_base_url(
            &mut co,
            Utf8Path::new("/project"),
            Utf8Path::new("/project"),
        );

        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["*"], json!(["./types/*", "./fallback/*"]));
    }

    #[test]
    fn base_url_without_paths_synthesizes_wildcard() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("."));

        normalize_base_url(
            &mut co,
            Utf8Path::new("/project"),
            Utf8Path::new("/project"),
        );

        assert!(!co.contains_key("baseUrl"));
        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["*"], json!(["./*"]));
    }

    #[test]
    fn base_url_src_without_paths_synthesizes_wildcard_with_prefix() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("./src"));

        normalize_base_url(
            &mut co,
            Utf8Path::new("/project/src"),
            Utf8Path::new("/project"),
        );

        assert!(!co.contains_key("baseUrl"));
        let paths = co["paths"].as_object().unwrap();
        assert_eq!(paths["*"], json!(["src/*"]));
    }

    #[test]
    fn type_roots_rewriting() {
        let mut co = serde_json::Map::new();
        co.insert("baseUrl".to_string(), json!("."));
        co.insert(
            "typeRoots".to_string(),
            json!(["node_modules/@types", "custom-types"]),
        );

        normalize_base_url(
            &mut co,
            Utf8Path::new("/project"),
            Utf8Path::new("/project"),
        );

        let roots = co["typeRoots"].as_array().unwrap();
        assert_eq!(roots[0], json!("./node_modules/@types"));
        assert_eq!(roots[1], json!("./custom-types"));
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
