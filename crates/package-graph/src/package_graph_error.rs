use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum PackageGraphError {
    #[error("Unknown package {0}. Not found in package graph.")]
    UnknownPackage(String),
}
