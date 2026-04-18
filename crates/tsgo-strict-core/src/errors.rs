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

    #[error("unable to locate tsgo binary (set TSGO_BINARY, install @typescript/native-preview, or add tsgo to PATH)")]
    TsgoNotFound,

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
