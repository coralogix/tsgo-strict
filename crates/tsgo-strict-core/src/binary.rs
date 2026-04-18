use crate::errors::Error;
use camino::Utf8PathBuf;
use std::path::{Path, PathBuf};

/// Locate the `tsgo` binary, mirroring resolveTsgoBinary in
/// src/runner/tsgoRunner.ts:
///
/// 1. `TSGO_BINARY` env var (trimmed, non-empty).
/// 2. Walk up from `cwd` checking `node_modules/.bin/tsgo` (or `.cmd`/`.exe`).
/// 3. Walk up from `cwd` checking `node_modules/@typescript/native-preview/`
///    and `node_modules/tsgo/` for their package.json bin entry.
/// 4. Fall back to `which tsgo` on PATH.
pub fn resolve_tsgo_binary(cwd: &Utf8PathBuf) -> Result<Utf8PathBuf, Error> {
    if let Ok(v) = std::env::var("TSGO_BINARY") {
        let trimmed = v.trim();
        if !trimmed.is_empty() {
            return Ok(Utf8PathBuf::from(trimmed));
        }
    }

    if let Some(bin) = find_node_modules_bin(cwd.as_std_path()) {
        return Ok(Utf8PathBuf::try_from(bin).unwrap());
    }

    for package in ["@typescript/native-preview", "tsgo"] {
        if let Some(bin) = find_package_bin(cwd.as_std_path(), package) {
            if let Ok(utf8) = Utf8PathBuf::try_from(bin) {
                return Ok(utf8);
            }
        }
    }

    match which::which("tsgo") {
        Ok(p) => Utf8PathBuf::try_from(p).map_err(|e| {
            Error::msg(format!(
                "tsgo path on PATH is not valid UTF-8: {}",
                e.into_path_buf().to_string_lossy()
            ))
        }),
        Err(_) => Err(Error::TsgoNotFound),
    }
}

fn bin_name() -> &'static str {
    if cfg!(windows) {
        "tsgo.cmd"
    } else {
        "tsgo"
    }
}

fn find_node_modules_bin(start: &Path) -> Option<PathBuf> {
    let mut dir: Option<&Path> = Some(start);
    while let Some(d) = dir {
        let candidate = d.join("node_modules").join(".bin").join(bin_name());
        if candidate.is_file() {
            return Some(candidate);
        }
        if cfg!(windows) {
            let exe = d.join("node_modules").join(".bin").join("tsgo.exe");
            if exe.is_file() {
                return Some(exe);
            }
        }
        dir = d.parent();
    }
    None
}

fn find_package_bin(start: &Path, package: &str) -> Option<PathBuf> {
    let mut dir: Option<&Path> = Some(start);
    while let Some(d) = dir {
        let pkg_dir = d.join("node_modules").join(package);
        let pkg_json = pkg_dir.join("package.json");
        if pkg_json.is_file() {
            if let Ok(raw) = std::fs::read_to_string(&pkg_json) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) {
                    if let Some(rel) = extract_bin(&v) {
                        let resolved = pkg_dir.join(rel);
                        if resolved.is_file() {
                            return Some(resolved);
                        }
                    }
                }
            }
        }
        dir = d.parent();
    }
    None
}

fn extract_bin(pkg: &serde_json::Value) -> Option<String> {
    let bin = pkg.get("bin")?;
    if let Some(s) = bin.as_str() {
        return Some(s.to_string());
    }
    if let Some(obj) = bin.as_object() {
        if let Some(s) = obj.get("tsgo").and_then(|v| v.as_str()) {
            return Some(s.to_string());
        }
        if let Some((_, v)) = obj.iter().next() {
            return v.as_str().map(|s| s.to_string());
        }
    }
    None
}
