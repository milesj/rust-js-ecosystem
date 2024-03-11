use lightningcss::error::{Error, ParserError, PrinterErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CssModuleError {
    #[error("Failed to parse CSS: {0}")]
    ParseFailed(Error<ParserError<'static>>),

    #[error("Failed to parse CSS module: {0}")]
    ParseModuleFailed(Error<PrinterErrorKind>),
}
