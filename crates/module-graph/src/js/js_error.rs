use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsModuleError {
    #[error("Failed to parse JS module {path}: {error}")]
    ParseFailed { path: PathBuf, error: String },

    #[error("Failed to parse JS module {path}. Parsing panicked and did not return a result!")]
    ParsePanicked { path: PathBuf },
}
