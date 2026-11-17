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
use std::str::FromStr;

/// Environment variable that selects the backend when no explicit option is
/// given, so a CI job can switch backends without changing scripts.
pub const BACKEND_ENV: &str = "TSGO_STRICT_BACKEND";

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub project: String,
    pub cwd: Utf8PathBuf,
    pub subset_inputs: Vec<String>,
    pub list_files: bool,
    pub timing: bool,
    /// Compiler backend. `None` defers to [`BACKEND_ENV`], then [`Backend::Auto`].
    pub backend: Option<Backend>,
    /// Pass `--fast` to tsrs. Rejected for any other backend.
    pub fast: bool,
}

/// The type checker tsgo-strict drives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Backend {
    /// tsrs when the project installs it, otherwise the native TypeScript compiler.
    #[default]
    Auto,
    /// The native TypeScript compiler: `typescript@7+` or `@typescript/native-preview`.
    Tsc,
    /// [tsrs](https://github.com/oxc-project/tsrs), the Rust port of the TypeScript 7 compiler.
    Tsrs,
}

impl Backend {
    pub fn as_str(&self) -> &'static str {
        match self {
            Backend::Auto => "auto",
            Backend::Tsc => "tsc",
            Backend::Tsrs => "tsrs",
        }
    }

    /// An explicit choice wins, then a non-empty [`BACKEND_ENV`], then `auto`.
    pub fn resolve(explicit: Option<Backend>) -> Result<Backend, Error> {
        if let Some(backend) = explicit {
            return Ok(backend);
        }
        match std::env::var(BACKEND_ENV) {
            Ok(v) if !v.trim().is_empty() => v
                .parse()
                .map_err(|e: String| Error::msg(format!("{BACKEND_ENV}: {e}"))),
            _ => Ok(Backend::default()),
        }
    }
}

impl FromStr for Backend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "auto" => Ok(Backend::Auto),
            "tsc" => Ok(Backend::Tsc),
            "tsrs" => Ok(Backend::Tsrs),
            other => Err(format!(
                "unknown backend '{other}', expected 'auto', 'tsc' or 'tsrs'"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_backends() {
        assert_eq!("auto".parse::<Backend>(), Ok(Backend::Auto));
        assert_eq!("tsc".parse::<Backend>(), Ok(Backend::Tsc));
        assert_eq!(" tsrs ".parse::<Backend>(), Ok(Backend::Tsrs));
        assert!("tsgo".parse::<Backend>().is_err());
    }

    #[test]
    fn explicit_backend_wins_without_reading_env() {
        assert_eq!(
            Backend::resolve(Some(Backend::Tsrs)).unwrap(),
            Backend::Tsrs
        );
        assert_eq!(Backend::resolve(Some(Backend::Tsc)).unwrap(), Backend::Tsc);
    }
}
