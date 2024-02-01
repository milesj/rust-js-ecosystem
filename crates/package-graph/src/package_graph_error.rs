use starbase_utils::fs::FsError;
use starbase_utils::glob::GlobError;
use starbase_utils::json::JsonError;
use starbase_utils::yaml::YamlError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum PackageGraphError {
    #[error("Package requires a `name` field. Missing in {0}.")]
    MissingPackageName(PathBuf),

    #[error("Unknown package `{0}`. Not found in package graph.")]
    UnknownPackage(String),

    #[error(transparent)]
    Fs(#[from] FsError),

    #[error(transparent)]
    Glob(#[from] GlobError),

    #[error(transparent)]
    Json(#[from] JsonError),

    #[error(transparent)]
    Yaml(#[from] YamlError),
}
