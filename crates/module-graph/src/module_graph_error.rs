use starbase_utils::fs::FsError;
use starbase_utils::json::JsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModuleGraphError {
    #[error(transparent)]
    Fs(#[from] FsError),

    #[error(transparent)]
    Json(#[from] JsonError),
}
