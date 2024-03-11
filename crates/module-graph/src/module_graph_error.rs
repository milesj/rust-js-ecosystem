use crate::css::CssModuleError;
use crate::js::JsModuleError;
use oxc_resolver::ResolveError;
use starbase_utils::fs::FsError;
use starbase_utils::json::JsonError;
use starbase_utils::yaml::YamlError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModuleGraphError {
    #[error(transparent)]
    Css(#[from] Box<CssModuleError>),

    #[error(transparent)]
    Fs(#[from] FsError),

    #[error(transparent)]
    Js(#[from] Box<JsModuleError>),

    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    Yaml(#[from] YamlError),

    #[error("Failed to resolve {specifier} from {dir}: {error}")]
    ResolveFailed {
        dir: PathBuf,
        specifier: String,
        error: Box<ResolveError>,
    },

    #[error("Unsupported module {0}. Not a valid file type or format.")]
    UnsupportedFileType(PathBuf),
}
