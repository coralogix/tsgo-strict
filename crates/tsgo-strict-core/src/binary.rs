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
use camino::Utf8PathBuf;
use std::path::{Path, PathBuf};

/// Known native TypeScript distributions, in priority order. Each entry is a
/// `(platform-package base, executable base name)` pair:
///
/// * `@typescript/typescript` ships the native compiler as `tsc` (TypeScript 7
///   and up — the official release channel).
/// * `@typescript/native-preview` shipped it as `tsgo` (the pre-7 preview
///   channel), kept here so existing preview installs keep working.
///
/// The platform binary lives at `<base>-{platform}-{arch}/lib/<exe>`, matching
/// the resolution in each package's `lib/getExePath.js`.
const NATIVE_DISTS: &[(&str, &str)] = &[
    ("@typescript/typescript", "tsc"),
    ("@typescript/native-preview", "tsgo"),
];

/// Locate the native TypeScript compiler binary:
///
/// 1. `TSGO_BINARY` env var (trimmed, non-empty).
/// 2. Walk up from `cwd` looking for a platform-specific native binary shipped
///    by a known distribution (see [`NATIVE_DISTS`]).
/// 3. Walk up from `cwd` checking `node_modules/<pkg>` for its package.json bin
///    entry, for `typescript`, `@typescript/native-preview`, and `tsgo`.
/// 4. Fall back to `tsgo` then `tsc` on PATH.
///
/// We intentionally skip `node_modules/.bin/*` because those are Node.js ESM
/// wrapper scripts. Spawning them adds Node startup overhead, and on some Node
/// versions the `#getExePath` package import fails entirely.
pub fn resolve_tsgo_binary(cwd: &Utf8PathBuf) -> Result<Utf8PathBuf, Error> {
    if let Ok(v) = std::env::var("TSGO_BINARY") {
        let trimmed = v.trim();
        if !trimmed.is_empty() {
            return Ok(Utf8PathBuf::from(trimmed));
        }
    }

    for (base, exe) in NATIVE_DISTS {
        if let Some(bin) = find_platform_binary(cwd.as_std_path(), base, exe) {
            if let Ok(utf8) = Utf8PathBuf::try_from(bin) {
                return Ok(utf8);
            }
        }
    }

    for package in ["typescript", "@typescript/native-preview", "tsgo"] {
        if let Some(bin) = find_package_bin(cwd.as_std_path(), package) {
            if let Ok(utf8) = Utf8PathBuf::try_from(bin) {
                return Ok(utf8);
            }
        }
    }

    for cmd in ["tsgo", "tsc"] {
        if let Ok(p) = which::which(cmd) {
            return Utf8PathBuf::try_from(p).map_err(|e| {
                Error::msg(format!(
                    "{} path on PATH is not valid UTF-8: {}",
                    cmd,
                    e.into_path_buf().to_string_lossy()
                ))
            });
        }
    }

    Err(Error::TsgoNotFound)
}

/// The `{platform}-{arch}` suffix used by the native platform packages, e.g.
/// `darwin-arm64`. Returns `None` on targets none of the distributions ship.
fn platform_suffix() -> Option<String> {
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

    Some(format!("{}-{}", platform, arch))
}

fn exe_file_name(base: &str) -> String {
    if cfg!(windows) {
        format!("{}.exe", base)
    } else {
        base.to_string()
    }
}

/// Walk up from `start` looking for the native binary inside the
/// platform-specific `<package_base>-{platform}-{arch}` package.
fn find_platform_binary(start: &Path, package_base: &str, exe_base: &str) -> Option<PathBuf> {
    let package = format!("{}-{}", package_base, platform_suffix()?);
    let exe = exe_file_name(exe_base);
    let mut dir: Option<&Path> = Some(start);
    while let Some(d) = dir {
        let candidate = d.join("node_modules").join(&package).join("lib").join(&exe);
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
        for key in ["tsgo", "tsc"] {
            if let Some(s) = obj.get(key).and_then(|v| v.as_str()) {
                return Some(s.to_string());
            }
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
    use std::sync::Mutex;
    use tempfile::TempDir;

    /// Serializes tests that mutate the shared process environment
    /// (`TSGO_BINARY`), which would otherwise race under the parallel test
    /// runner.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

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

    /// Place a fake native binary for the given distribution under `root` and
    /// return its path, or `None` if the current target isn't shipped.
    fn install_platform_binary(root: &Path, package_base: &str, exe_base: &str) -> Option<PathBuf> {
        let suffix = platform_suffix()?;
        let bin = root
            .join("node_modules")
            .join(format!("{package_base}-{suffix}"))
            .join("lib")
            .join(exe_file_name(exe_base));
        touch_exe(&bin);
        Some(bin)
    }

    #[test]
    fn find_platform_binary_discovers_typescript_7_package() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        if let Some(bin) = install_platform_binary(root, "@typescript/typescript", "tsc") {
            let nested = root.join("packages").join("app").join("src");
            fs::create_dir_all(&nested).unwrap();

            let found = find_platform_binary(&nested, "@typescript/typescript", "tsc")
                .expect("native binary should be discovered");
            assert_eq!(found, bin);
        }
    }

    #[test]
    fn find_platform_binary_discovers_native_preview_package() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        if let Some(bin) = install_platform_binary(root, "@typescript/native-preview", "tsgo") {
            let found = find_platform_binary(root, "@typescript/native-preview", "tsgo")
                .expect("native binary should be discovered");
            assert_eq!(found, bin);
        }
    }

    #[test]
    fn find_platform_binary_returns_none_when_absent() {
        let tmp = TempDir::new().unwrap();
        assert!(find_platform_binary(tmp.path(), "@typescript/typescript", "tsc").is_none());
    }

    #[test]
    fn resolve_prefers_typescript_7_over_native_preview() {
        // When both distributions are installed, the official TypeScript 7
        // package must win.
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let _env = ENV_LOCK.lock().unwrap();
        let (Some(ts), Some(_preview)) = (
            install_platform_binary(root, "@typescript/typescript", "tsc"),
            install_platform_binary(root, "@typescript/native-preview", "tsgo"),
        ) else {
            return; // target not shipped by the platform packages
        };
        let _guard = EnvGuard::remove("TSGO_BINARY");

        let cwd = Utf8PathBuf::from_path_buf(root.to_path_buf()).unwrap();
        let resolved = resolve_tsgo_binary(&cwd).unwrap();
        assert_eq!(resolved.as_std_path(), ts);
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

        let tsc = serde_json::json!({ "bin": { "tsc": "bin/tsc", "other": "x" } });
        assert_eq!(extract_bin(&tsc), Some("bin/tsc".to_string()));

        let other = serde_json::json!({ "bin": { "only": "bin/only.js" } });
        assert_eq!(extract_bin(&other), Some("bin/only.js".to_string()));

        let none = serde_json::json!({ "name": "fake" });
        assert_eq!(extract_bin(&none), None);
    }

    #[test]
    fn resolve_tsgo_binary_honors_env_override() {
        let _env = ENV_LOCK.lock().unwrap();
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

        fn remove(key: &'static str) -> Self {
            let previous = std::env::var(key).ok();
            std::env::remove_var(key);
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
