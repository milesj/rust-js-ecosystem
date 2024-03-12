use std::path::PathBuf;

use lightningcss::error::{Error, ParserError, PrinterErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum CssModuleError {
    #[error("Failed to parse CSS file {path}: {error}")]
    #[cfg_attr(feature = "miette", diagnostic(code(module_graph::css::parse_failed)))]
    ParseFailed {
        path: PathBuf,
        error: Box<Error<ParserError<'static>>>,
    },

    #[error("Failed to parse CSS module {path}: {error}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(code(module_graph::css::parse_module_failed))
    )]
    ParseModuleFailed {
        path: PathBuf,
        error: Box<Error<PrinterErrorKind>>,
    },
}
