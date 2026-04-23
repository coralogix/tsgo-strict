use crate::errors::Error;
use camino::Utf8PathBuf;
use std::path::{Path, PathBuf};

/// Locate the native `tsgo` binary:
///
/// 1. `TSGO_BINARY` env var (trimmed, non-empty).
/// 2. Walk up from `cwd` looking for the platform-specific native binary inside
///    `@typescript/native-preview-{platform}-{arch}/lib/tsgo`.
/// 3. Walk up from `cwd` checking `node_modules/@typescript/native-preview/`
///    and `node_modules/tsgo/` for their package.json bin entry.
/// 4. Fall back to `which tsgo` on PATH.
///
/// We intentionally skip `node_modules/.bin/tsgo` because it is a Node.js ESM
/// wrapper script. Spawning it adds Node startup overhead, and on some Node
/// versions the `#getExePath` package import fails entirely.
pub fn resolve_tsgo_binary(cwd: &Utf8PathBuf) -> Result<Utf8PathBuf, Error> {
    if let Ok(v) = std::env::var("TSGO_BINARY") {
        let trimmed = v.trim();
        if !trimmed.is_empty() {
            return Ok(Utf8PathBuf::from(trimmed));
        }
    }

    if let Some(bin) = find_native_preview_binary(cwd.as_std_path()) {
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

/// Resolve the platform-specific package name for `@typescript/native-preview`.
/// Mirrors the logic in `@typescript/native-preview/lib/getExePath.js`.
fn native_preview_platform_package() -> Option<String> {
    let platform = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "windows") {
        "win32"
    } else {
        return None;
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else {
        return None;
    };

    Some(format!("@typescript/native-preview-{}-{}", platform, arch))
}

fn exe_name() -> &'static str {
    if cfg!(windows) {
        "tsgo.exe"
    } else {
        "tsgo"
    }
}

/// Walk up from `start` looking for the native tsgo binary inside the
/// platform-specific `@typescript/native-preview-{platform}-{arch}` package.
fn find_native_preview_binary(start: &Path) -> Option<PathBuf> {
    let package = native_preview_platform_package()?;
    let mut dir: Option<&Path> = Some(start);
    while let Some(d) = dir {
        let candidate = d
            .join("node_modules")
            .join(&package)
            .join("lib")
            .join(exe_name());
        if candidate.is_file() {
            return Some(candidate);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn touch_exe(path: &Path) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, b"#!/bin/sh\nexit 0\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }

    fn write_pkg(dir: &Path, bin: &serde_json::Value) {
        fs::create_dir_all(dir).unwrap();
        let pkg = serde_json::json!({ "name": "fake", "bin": bin });
        fs::write(dir.join("package.json"), pkg.to_string()).unwrap();
    }

    #[test]
    fn find_native_preview_binary_discovers_platform_package() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        if let Some(pkg) = native_preview_platform_package() {
            let bin = root
                .join("node_modules")
                .join(&pkg)
                .join("lib")
                .join(exe_name());
            touch_exe(&bin);

            let nested = root.join("packages").join("app").join("src");
            fs::create_dir_all(&nested).unwrap();

            let found =
                find_native_preview_binary(&nested).expect("native binary should be discovered");
            assert_eq!(found, bin);
        }
    }

    #[test]
    fn find_native_preview_binary_returns_none_when_absent() {
        let tmp = TempDir::new().unwrap();
        assert!(find_native_preview_binary(tmp.path()).is_none());
    }

    #[test]
    fn find_package_bin_resolves_native_preview_like_a_peer_dep() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let pkg_dir = root
            .join("node_modules")
            .join("@typescript")
            .join("native-preview");
        write_pkg(&pkg_dir, &serde_json::json!({ "tsgo": "bin/tsgo.js" }));
        touch_exe(&pkg_dir.join("bin").join("tsgo.js"));

        let nested = root.join("app");
        fs::create_dir_all(&nested).unwrap();

        let found =
            find_package_bin(&nested, "@typescript/native-preview").expect("peer dep resolution");
        assert_eq!(found, pkg_dir.join("bin").join("tsgo.js"));
    }

    #[test]
    fn find_package_bin_returns_none_when_bin_file_missing() {
        let tmp = TempDir::new().unwrap();
        let pkg_dir = tmp.path().join("node_modules").join("tsgo");
        write_pkg(&pkg_dir, &serde_json::json!({ "tsgo": "bin/missing.js" }));

        assert!(find_package_bin(tmp.path(), "tsgo").is_none());
    }

    #[test]
    fn extract_bin_handles_string_object_and_fallback() {
        let s = serde_json::json!({ "bin": "bin/tsgo.js" });
        assert_eq!(extract_bin(&s), Some("bin/tsgo.js".to_string()));

        let obj = serde_json::json!({ "bin": { "tsgo": "bin/tsgo.js", "other": "x" } });
        assert_eq!(extract_bin(&obj), Some("bin/tsgo.js".to_string()));

        let other = serde_json::json!({ "bin": { "only": "bin/only.js" } });
        assert_eq!(extract_bin(&other), Some("bin/only.js".to_string()));

        let none = serde_json::json!({ "name": "fake" });
        assert_eq!(extract_bin(&none), None);
    }

    #[test]
    fn resolve_tsgo_binary_honors_env_override() {
        let tmp = TempDir::new().unwrap();
        let fake = tmp.path().join("my-tsgo");
        touch_exe(&fake);
        let _guard = EnvGuard::set("TSGO_BINARY", fake.to_str().unwrap());

        let cwd = Utf8PathBuf::from_path_buf(tmp.path().to_path_buf()).unwrap();
        let resolved = resolve_tsgo_binary(&cwd).unwrap();
        assert_eq!(resolved.as_std_path(), fake);
    }

    struct EnvGuard {
        key: &'static str,
        previous: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self { key, previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match self.previous.take() {
                Some(v) => std::env::set_var(self.key, v),
                None => std::env::remove_var(self.key),
            }
        }
    }
}
