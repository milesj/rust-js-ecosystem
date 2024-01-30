use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum PackageGraphError {
    #[error("Package requires a `name` field. Missing in {0}.")]
    MissingPackageName(PathBuf),

    #[error("Unknown package `{0}`. Not found in package graph.")]
    UnknownPackage(String),
}
