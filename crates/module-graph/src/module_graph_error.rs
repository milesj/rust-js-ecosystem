use crate::css::CssModuleError;
use crate::js::JsModuleError;
use oxc_resolver::ResolveError;
use starbase_utils::fs::FsError;
use starbase_utils::json::JsonError;
use starbase_utils::yaml::YamlError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum ModuleGraphError {
    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Css(#[from] Box<CssModuleError>),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Fs(#[from] FsError),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Js(#[from] Box<JsModuleError>),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Json(#[from] JsonError),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Yaml(#[from] YamlError),

    #[error("Failed to resolve `{specifier}` from {dir}: {error}")]
    #[cfg_attr(feature = "miette", diagnostic(code(module_graph::resolve_failed)))]
    ResolveFailed {
        dir: PathBuf,
        specifier: String,
        error: Box<ResolveError>,
    },

    #[error("Unsupported file {0}. Not a valid file type or module format.")]
    #[cfg_attr(feature = "miette", diagnostic(code(module_graph::unsupported_file)))]
    UnsupportedFileType(PathBuf),
}
