use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum JsModuleError {
    #[error("Failed to parse JS module {path}: {error}")]
    #[cfg_attr(feature = "miette", diagnostic(code(module_graph::js::parse_failed)))]
    ParseFailed { path: PathBuf, error: String },

    #[error("Failed to parse JS module {path}. Parsing panicked and did not return a result!")]
    #[cfg_attr(feature = "miette", diagnostic(code(module_graph::js::parse_panicked)))]
    ParsePanicked { path: PathBuf },
}
