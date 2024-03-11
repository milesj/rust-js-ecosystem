use crate::css::CssModuleError;
use starbase_utils::fs::FsError;
use starbase_utils::json::JsonError;
use starbase_utils::yaml::YamlError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModuleGraphError {
    #[error(transparent)]
    Css(#[from] CssModuleError),

    #[error(transparent)]
    Fs(#[from] FsError),

    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    Yaml(#[from] YamlError),
}
