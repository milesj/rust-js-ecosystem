use starbase_utils::fs::FsError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModuleGraphError {
    #[error(transparent)]
    Fs(#[from] FsError),
}
