use std::path::PathBuf;

use lightningcss::error::{Error, ParserError, PrinterErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CssModuleError {
    #[error("Failed to parse CSS file {path}: {error}")]
    ParseFailed {
        path: PathBuf,
        error: Error<ParserError<'static>>,
    },

    #[error("Failed to parse CSS module {path}: {error}")]
    ParseModuleFailed {
        path: PathBuf,
        error: Error<PrinterErrorKind>,
    },
}
