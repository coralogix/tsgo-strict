use serde_json::{Map, Value};

/// Apply v5→v6 compatibility shims to the transient tsconfig's compilerOptions.
///
/// Every rule respects user-set values except where TypeScript 6.0 forces an
/// override. `compiler_options` is the map about to be serialized to
/// `strict.json`; `effective_options` is the shallow-merged view of the full
/// extends chain, used only to decide whether the user "set" a given key
/// anywhere. Pass the same map for both when a merged view is unavailable.
///
/// Idempotent; no I/O.
pub fn apply_v6_compat_shims(
    compiler_options: &mut Map<String, Value>,
    effective_options: &Map<String, Value>,
) {
    // 1. ignoreDeprecations: inject "6.0" only when the user hasn't picked a
    //    value. Respecting a user's explicit setting preserves their ability
    //    to see specific deprecation warnings.
    if !effective_options.contains_key("ignoreDeprecations") {
        compiler_options.insert(
            "ignoreDeprecations".to_string(),
            Value::String("6.0".to_string()),
        );
    }

    // 2. Hard-removed-false keys: v6 does not allow `false` for these. When
    //    the effective value is `false`, write `true` into the leaf so it
    //    wins over any inherited `false` via `extends`.
    for key in [
        "esModuleInterop",
        "allowSyntheticDefaultImports",
        "alwaysStrict",
    ] {
        if matches!(effective_options.get(key), Some(Value::Bool(false))) {
            compiler_options.insert(key.to_string(), Value::Bool(true));
        }
    }

    // 3. v6 default flips that affect type-checking. Preserve v5 defaults
    //    only when the user hasn't picked a value.
    if !effective_options.contains_key("noUncheckedSideEffectImports") {
        compiler_options.insert(
            "noUncheckedSideEffectImports".to_string(),
            Value::Bool(false),
        );
    }
    if !effective_options.contains_key("libReplacement") {
        compiler_options.insert("libReplacement".to_string(), Value::Bool(true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn empty() -> Map<String, Value> {
        Map::new()
    }

    fn co(pairs: &[(&str, Value)]) -> Map<String, Value> {
        let mut m = Map::new();
        for (k, v) in pairs {
            m.insert((*k).to_string(), v.clone());
        }
        m
    }

    #[test]
    fn ignore_deprecations_injected_when_absent() {
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &empty());
        assert_eq!(leaf["ignoreDeprecations"], json!("6.0"));
    }

    #[test]
    fn ignore_deprecations_respects_user_value() {
        let effective = co(&[("ignoreDeprecations", json!("5.5"))]);
        let mut leaf = co(&[("ignoreDeprecations", json!("5.5"))]);
        apply_v6_compat_shims(&mut leaf, &effective);
        assert_eq!(leaf["ignoreDeprecations"], json!("5.5"));
    }

    #[test]
    fn es_module_interop_false_rewritten_to_true() {
        let effective = co(&[("esModuleInterop", json!(false))]);
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert_eq!(leaf["esModuleInterop"], json!(true));
    }

    #[test]
    fn es_module_interop_true_left_alone() {
        let effective = co(&[("esModuleInterop", json!(true))]);
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert!(leaf.get("esModuleInterop").is_none());
    }

    #[test]
    fn es_module_interop_absent_not_injected() {
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &empty());
        assert!(leaf.get("esModuleInterop").is_none());
    }

    #[test]
    fn allow_synthetic_default_imports_false_rewritten_to_true() {
        let effective = co(&[("allowSyntheticDefaultImports", json!(false))]);
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert_eq!(leaf["allowSyntheticDefaultImports"], json!(true));
    }

    #[test]
    fn allow_synthetic_default_imports_true_left_alone() {
        let effective = co(&[("allowSyntheticDefaultImports", json!(true))]);
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert!(leaf.get("allowSyntheticDefaultImports").is_none());
    }

    #[test]
    fn allow_synthetic_default_imports_absent_not_injected() {
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &empty());
        assert!(leaf.get("allowSyntheticDefaultImports").is_none());
    }

    #[test]
    fn always_strict_false_rewritten_to_true() {
        let effective = co(&[("alwaysStrict", json!(false))]);
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert_eq!(leaf["alwaysStrict"], json!(true));
    }

    #[test]
    fn always_strict_true_left_alone() {
        let effective = co(&[("alwaysStrict", json!(true))]);
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert!(leaf.get("alwaysStrict").is_none());
    }

    #[test]
    fn always_strict_absent_not_injected() {
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &empty());
        assert!(leaf.get("alwaysStrict").is_none());
    }

    #[test]
    fn no_unchecked_side_effect_imports_absent_injected_as_false() {
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &empty());
        assert_eq!(leaf["noUncheckedSideEffectImports"], json!(false));
    }

    #[test]
    fn no_unchecked_side_effect_imports_true_preserved() {
        let effective = co(&[("noUncheckedSideEffectImports", json!(true))]);
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert!(leaf.get("noUncheckedSideEffectImports").is_none());
    }

    #[test]
    fn no_unchecked_side_effect_imports_false_preserved() {
        let effective = co(&[("noUncheckedSideEffectImports", json!(false))]);
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert!(leaf.get("noUncheckedSideEffectImports").is_none());
    }

    #[test]
    fn lib_replacement_absent_injected_as_true() {
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &empty());
        assert_eq!(leaf["libReplacement"], json!(true));
    }

    #[test]
    fn lib_replacement_true_preserved() {
        let effective = co(&[("libReplacement", json!(true))]);
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert!(leaf.get("libReplacement").is_none());
    }

    #[test]
    fn lib_replacement_false_preserved() {
        let effective = co(&[("libReplacement", json!(false))]);
        let mut leaf = empty();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert!(leaf.get("libReplacement").is_none());
    }

    #[test]
    fn idempotent_when_called_twice() {
        let effective = co(&[("esModuleInterop", json!(false))]);
        let mut once = empty();
        apply_v6_compat_shims(&mut once, &effective);
        let mut twice = once.clone();
        apply_v6_compat_shims(&mut twice, &effective);
        assert_eq!(once, twice);
    }

    #[test]
    fn does_not_touch_unrelated_keys() {
        let effective = co(&[
            ("target", json!("ES2020")),
            ("module", json!("esnext")),
            ("strict", json!(true)),
            ("paths", json!({ "@app/*": ["src/app/*"] })),
        ]);
        let mut leaf = effective.clone();
        apply_v6_compat_shims(&mut leaf, &effective);
        assert_eq!(leaf["target"], json!("ES2020"));
        assert_eq!(leaf["module"], json!("esnext"));
        assert_eq!(leaf["strict"], json!(true));
        assert_eq!(leaf["paths"], json!({ "@app/*": ["src/app/*"] }));
    }
}
