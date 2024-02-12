use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged, from = "String")]
pub enum PathType {
    Path(PathBuf),
    Glob(String),
}

// https://www.typescriptlang.org/tsconfig#include
impl From<&str> for PathType {
    fn from(value: &str) -> Self {
        if value.contains('*') || value.contains('?') {
            Self::Glob(value.to_owned())
        } else {
            Self::Path(PathBuf::from(value))
        }
    }
}

impl From<String> for PathType {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged, from = "String")]
pub enum PathOrUrl {
    Path(PathBuf),
    Url(String),
}

impl From<&str> for PathOrUrl {
    fn from(value: &str) -> Self {
        if value.starts_with("http") {
            Self::Url(value.to_owned())
        } else {
            Self::Path(PathBuf::from(value))
        }
    }
}

impl From<String> for PathOrUrl {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}
