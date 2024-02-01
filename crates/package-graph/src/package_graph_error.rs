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
    #[cfg_attr(
        feature = "miette",
        diagnostic(code(package_graph::missing_package_name))
    )]
    MissingPackageName(PathBuf),

    #[error("Unknown package `{0}`. Not found in package graph.")]
    #[cfg_attr(feature = "miette", diagnostic(code(package_graph::unknown_package)))]
    UnknownPackage(String),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Fs(#[from] FsError),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Glob(#[from] GlobError),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Json(#[from] JsonError),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Yaml(#[from] YamlError),
}
