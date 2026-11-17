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
use crate::options::Backend;
use camino::Utf8PathBuf;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A resolved type checker plus the settings every invocation of it needs.
#[derive(Debug, Clone)]
pub struct Compiler {
    pub path: Utf8PathBuf,
    pub backend: Backend,
    pub fast: bool,
}

impl Compiler {
    /// A command for this compiler with plain, uncolored output and a
    /// deterministic checker configuration.
    ///
    /// `TSRS_FAST` is always removed: tsrs fast mode can change diagnostics,
    /// and an ambient value (tsrs-lint and oxlint turn it on) must not
    /// silently alter a strict check. Fast mode is enabled only through
    /// [`Compiler::fast`], which adds `--fast`.
    pub fn command(&self) -> Command {
        let mut cmd = Command::new(self.path.as_std_path());
        cmd.env("NO_COLOR", "1")
            .env("FORCE_COLOR", "0")
            .env_remove("TSRS_FAST");
        if self.fast {
            cmd.arg("--fast");
        }
        cmd
    }
}

/// Resolve the compiler for `backend`. `fast` is a tsrs-only mode.
///
/// `Auto` honors an explicit `TSGO_BINARY` first (treated as the native
/// compiler), then prefers a tsrs install found from `cwd` (`TSRS_BINARY` or
/// the `tsrs` package), and otherwise falls back to the native compiler. It
/// never picks up a `tsrs` that is only on PATH, so a global install cannot
/// switch a project's checker.
pub fn resolve_compiler(
    cwd: &Utf8PathBuf,
    backend: Backend,
    fast: bool,
) -> Result<Compiler, Error> {
    let (path, resolved) = match backend {
        Backend::Tsc => (resolve_tsgo_binary(cwd)?, Backend::Tsc),
        Backend::Tsrs => (resolve_tsrs_binary(cwd)?, Backend::Tsrs),
        Backend::Auto => match env_path("TSGO_BINARY") {
            Some(p) => (p, Backend::Tsc),
            None => match find_installed_tsrs(cwd) {
                Some(p) => (p, Backend::Tsrs),
                None => (resolve_tsgo_binary(cwd)?, Backend::Tsc),
            },
        },
    };
    if fast && resolved != Backend::Tsrs {
        return Err(Error::msg(format!(
            "fast mode is only available with the tsrs backend, but {} resolved to the native TypeScript compiler ({path})",
            backend.as_str()
        )));
    }
    Ok(Compiler {
        path,
        backend: resolved,
        fast,
    })
}

fn env_path(var: &str) -> Option<Utf8PathBuf> {
    let v = std::env::var(var).ok()?;
    let trimmed = v.trim();
    (!trimmed.is_empty()).then(|| Utf8PathBuf::from(trimmed))
}

/// A tsrs the project chose: `TSRS_BINARY`, or the `tsrs` package's
/// platform binary found from `cwd`.
fn find_installed_tsrs(cwd: &Utf8PathBuf) -> Option<Utf8PathBuf> {
    env_path("TSRS_BINARY").or_else(|| {
        find_tsrs_platform_binary(cwd.as_std_path()).and_then(|p| Utf8PathBuf::try_from(p).ok())
    })
}

/// Locate the tsrs native binary:
///
/// 1. `TSGO_BINARY`, then tsrs's own `TSRS_BINARY` override (trimmed, non-empty).
/// 2. Walk up from `cwd` to `node_modules/tsrs` and take the platform binary
///    from the `@tsrs/<platform>` package installed beside it. Resolving
///    through the real path of `tsrs` covers both a flat `node_modules` and
///    pnpm's isolated layout, where the platform package is not hoisted.
/// 3. Fall back to `tsrs` on PATH.
///
/// The `tsrs` package's own `bin` is skipped: it is a Node launcher, and the
/// platform package also carries the `lib.*.d.ts` files the binary needs.
pub fn resolve_tsrs_binary(cwd: &Utf8PathBuf) -> Result<Utf8PathBuf, Error> {
    if let Some(p) = env_path("TSGO_BINARY") {
        return Ok(p);
    }
    if let Some(p) = find_installed_tsrs(cwd) {
        return Ok(p);
    }

    if let Ok(p) = which::which("tsrs") {
        return Utf8PathBuf::try_from(p).map_err(|e| {
            Error::msg(format!(
                "tsrs path on PATH is not valid UTF-8: {}",
                e.into_path_buf().to_string_lossy()
            ))
        });
    }

    Err(Error::TsrsNotFound)
}

/// tsrs platform package names, preferred first. Linux packages carry a libc
/// suffix; the one matching this build's libc is tried before the other.
fn tsrs_platform_packages() -> Vec<String> {
    let Some(suffix) = platform_suffix() else {
        return Vec::new();
    };
    if cfg!(target_os = "linux") {
        let libcs = if cfg!(target_env = "musl") {
            ["musl", "gnu"]
        } else {
            ["gnu", "musl"]
        };
        libcs
            .iter()
            .map(|libc| format!("@tsrs/{suffix}-{libc}"))
            .collect()
    } else {
        vec![format!("@tsrs/{suffix}")]
    }
}

fn find_tsrs_platform_binary(start: &Path) -> Option<PathBuf> {
    let packages = tsrs_platform_packages();
    let exe = exe_file_name("tsrs");
    let mut dir: Option<&Path> = Some(start);
    while let Some(d) = dir {
        let tsrs_pkg = d.join("node_modules").join("tsrs");
        if tsrs_pkg.join("package.json").is_file() {
            let real = std::fs::canonicalize(&tsrs_pkg).unwrap_or(tsrs_pkg);
            if let Some(modules_dir) = real.parent() {
                for package in &packages {
                    let candidate = modules_dir.join(package).join(&exe);
                    if candidate.is_file() {
                        return Some(candidate);
                    }
                }
            }
        }
        dir = d.parent();
    }
    None
}

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
///    entry, for `@typescript/native-preview`, `tsgo`, then `typescript`. The
///    dedicated native packages are checked first: they are *always* the native
///    compiler, whereas `typescript` is only native from v7 on. This lets a team
///    that stays on TypeScript 5/6 for their app install `@typescript/native-preview`
///    for strict checking and have it win over their ambient `typescript`.
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

    for package in ["@typescript/native-preview", "tsgo", "typescript"] {
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
    fn resolve_prefers_native_preview_package_over_ambient_typescript() {
        // A team stays on TypeScript 5/6 for their app but installs
        // `@typescript/native-preview` for strict checking. With no platform
        // binaries present (e.g. pnpm, where they aren't hoisted), the
        // package-bin step must pick the native preview over the ambient
        // `typescript` package (which would be the slower JS compiler).
        let _env = ENV_LOCK.lock().unwrap();
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        let ts_dir = root.join("node_modules").join("typescript");
        write_pkg(
            &ts_dir,
            &serde_json::json!({ "tsc": "bin/tsc", "tsserver": "bin/tsserver" }),
        );
        touch_exe(&ts_dir.join("bin").join("tsc"));

        let np_dir = root
            .join("node_modules")
            .join("@typescript")
            .join("native-preview");
        write_pkg(&np_dir, &serde_json::json!({ "tsgo": "bin/tsgo.js" }));
        let np_bin = np_dir.join("bin").join("tsgo.js");
        touch_exe(&np_bin);

        let _guard = EnvGuard::remove("TSGO_BINARY");
        let cwd = Utf8PathBuf::from_path_buf(root.to_path_buf()).unwrap();
        let resolved = resolve_tsgo_binary(&cwd).unwrap();
        assert_eq!(resolved.as_std_path(), np_bin);
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

    /// Lay out `tsrs` the way pnpm does: the package lives in the virtual
    /// store, with its platform package as a sibling, and only `tsrs` itself
    /// is linked into the top-level `node_modules`.
    fn install_tsrs_pnpm_layout(root: &Path) -> Option<PathBuf> {
        let package = tsrs_platform_packages().into_iter().next()?;
        let store = root.join("node_modules/.pnpm/tsrs@1.0.0/node_modules");
        write_pkg(&store.join("tsrs"), &serde_json::json!("bin/tsrs"));
        let bin = store.join(&package).join(exe_file_name("tsrs"));
        touch_exe(&bin);
        #[cfg(unix)]
        std::os::unix::fs::symlink(store.join("tsrs"), root.join("node_modules/tsrs")).unwrap();
        #[cfg(not(unix))]
        return None;
        Some(fs::canonicalize(bin).unwrap())
    }

    #[test]
    fn find_tsrs_platform_binary_follows_pnpm_symlink() {
        let tmp = TempDir::new().unwrap();
        if let Some(bin) = install_tsrs_pnpm_layout(tmp.path()) {
            let nested = tmp.path().join("packages/app");
            fs::create_dir_all(&nested).unwrap();
            let found = find_tsrs_platform_binary(&nested).expect("tsrs binary");
            assert_eq!(fs::canonicalize(found).unwrap(), bin);
        }
    }

    #[test]
    fn find_tsrs_platform_binary_handles_flat_node_modules() {
        let tmp = TempDir::new().unwrap();
        let Some(package) = tsrs_platform_packages().into_iter().next() else {
            return;
        };
        let modules = tmp.path().join("node_modules");
        write_pkg(&modules.join("tsrs"), &serde_json::json!("bin/tsrs"));
        let bin = modules.join(&package).join(exe_file_name("tsrs"));
        touch_exe(&bin);
        let found = find_tsrs_platform_binary(tmp.path()).expect("tsrs binary");
        assert_eq!(
            fs::canonicalize(found).unwrap(),
            fs::canonicalize(bin).unwrap()
        );
    }

    #[test]
    fn find_tsrs_platform_binary_none_without_platform_package() {
        let tmp = TempDir::new().unwrap();
        write_pkg(
            &tmp.path().join("node_modules/tsrs"),
            &serde_json::json!("bin/tsrs"),
        );
        assert_eq!(find_tsrs_platform_binary(tmp.path()), None);
    }

    #[test]
    fn resolve_tsrs_binary_honors_overrides_in_order() {
        let _env = ENV_LOCK.lock().unwrap();
        let tmp = TempDir::new().unwrap();
        let cwd = Utf8PathBuf::from_path_buf(tmp.path().to_path_buf()).unwrap();

        let _tsgo = EnvGuard::set("TSGO_BINARY", "/opt/a/tsrs");
        let _tsrs = EnvGuard::set("TSRS_BINARY", "/opt/b/tsrs");
        assert_eq!(resolve_tsrs_binary(&cwd).unwrap(), "/opt/a/tsrs");

        let _tsgo = EnvGuard::remove("TSGO_BINARY");
        assert_eq!(resolve_tsrs_binary(&cwd).unwrap(), "/opt/b/tsrs");
    }

    #[test]
    fn resolve_compiler_rejects_fast_when_native_compiler_resolves() {
        let _env = ENV_LOCK.lock().unwrap();
        let _tsgo = EnvGuard::set("TSGO_BINARY", "/opt/tsc");
        let cwd = Utf8PathBuf::from("/tmp");
        for backend in [Backend::Tsc, Backend::Auto] {
            let err = resolve_compiler(&cwd, backend, true).unwrap_err();
            assert!(err.to_string().contains("tsrs backend"), "{err}");
        }
    }

    #[test]
    fn auto_prefers_installed_tsrs_over_native_compiler() {
        let _env = ENV_LOCK.lock().unwrap();
        let _tsgo = EnvGuard::remove("TSGO_BINARY");
        let _tsrs = EnvGuard::remove("TSRS_BINARY");
        let tmp = TempDir::new().unwrap();
        install_platform_binary(tmp.path(), "@typescript/typescript", "tsc");
        let Some(tsrs) = install_tsrs_pnpm_layout(tmp.path()) else {
            return;
        };
        let cwd = Utf8PathBuf::from_path_buf(tmp.path().to_path_buf()).unwrap();

        let auto = resolve_compiler(&cwd, Backend::Auto, true).unwrap();
        assert_eq!(auto.backend, Backend::Tsrs);
        assert_eq!(fs::canonicalize(auto.path).unwrap(), tsrs);

        let tsc = resolve_compiler(&cwd, Backend::Tsc, false).unwrap();
        assert_eq!(tsc.backend, Backend::Tsc);
    }

    #[test]
    fn auto_uses_native_compiler_when_tsrs_absent_and_honors_tsgo_binary() {
        let _env = ENV_LOCK.lock().unwrap();
        let _tsrs = EnvGuard::remove("TSRS_BINARY");
        let tmp = TempDir::new().unwrap();
        let cwd = Utf8PathBuf::from_path_buf(tmp.path().to_path_buf()).unwrap();

        let _tsgo = EnvGuard::remove("TSGO_BINARY");
        if let Some(tsc) = install_platform_binary(tmp.path(), "@typescript/typescript", "tsc") {
            let auto = resolve_compiler(&cwd, Backend::Auto, false).unwrap();
            assert_eq!(
                (auto.backend, auto.path.as_std_path()),
                (Backend::Tsc, tsc.as_path())
            );
        }

        install_tsrs_pnpm_layout(tmp.path());
        let _tsgo = EnvGuard::set("TSGO_BINARY", "/opt/explicit");
        let auto = resolve_compiler(&cwd, Backend::Auto, false).unwrap();
        assert_eq!(
            (auto.backend, auto.path.as_str()),
            (Backend::Tsc, "/opt/explicit")
        );
    }

    #[test]
    fn compiler_command_scrubs_tsrs_fast_and_adds_flag_only_when_opted_in() {
        let env_of = |cmd: &Command, key: &str| {
            cmd.get_envs()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v.map(|v| v.to_owned()))
        };
        let compiler = Compiler {
            path: Utf8PathBuf::from("/bin/tsrs"),
            backend: Backend::Tsrs,
            fast: false,
        };
        let cmd = compiler.command();
        assert_eq!(
            env_of(&cmd, "TSRS_FAST"),
            Some(None),
            "TSRS_FAST must be removed"
        );
        assert_eq!(cmd.get_args().count(), 0);

        let fast = Compiler {
            fast: true,
            ..compiler
        };
        let args: Vec<_> = fast.command().get_args().map(|a| a.to_owned()).collect();
        assert_eq!(args, vec!["--fast"]);
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
