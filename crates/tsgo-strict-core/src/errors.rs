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

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unable to read tsconfig at {path}: {source}")]
    TsconfigRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("unable to parse tsconfig at {path}: {message}")]
    TsconfigParse { path: PathBuf, message: String },

    #[error("cannot find extends target '{target}' referenced from {from}")]
    ExtendsNotFound { target: String, from: PathBuf },

    #[error("unable to locate the TypeScript compiler (install typescript@7 or later, set TSGO_BINARY to a tsc/tsgo binary, or add one to PATH)")]
    TsgoNotFound,

    #[error("tsgo exited with exit code {exit_code}: {stderr}")]
    TsgoFailed { exit_code: i32, stderr: String },

    #[error("tsgo invocation failed: {0}")]
    TsgoSpawn(#[from] std::io::Error),

    #[error("{0}")]
    Msg(String),
}

impl Error {
    pub fn msg(message: impl Into<String>) -> Self {
        Error::Msg(message.into())
    }
}
