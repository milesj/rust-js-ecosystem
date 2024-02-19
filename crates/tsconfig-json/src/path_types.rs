use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged, from = "String")]
pub enum PathOrGlob {
    Path(PathBuf),
    Glob(String),
}

// https://www.typescriptlang.org/tsconfig#include
impl From<&str> for PathOrGlob {
    fn from(value: &str) -> Self {
        if value.contains('*') || value.contains('?') {
            Self::Glob(value.to_owned())
        } else {
            Self::Path(PathBuf::from(value))
        }
    }
}

impl From<String> for PathOrGlob {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}
