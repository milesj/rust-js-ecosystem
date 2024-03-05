use serde::Deserialize;
use std::borrow::Cow;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged, from = "String", into = "String")]
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

impl Into<String> for PathOrGlob {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for PathOrGlob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Glob(glob) => Cow::Borrowed(glob.as_str()),
                Self::Path(path) => path.to_string_lossy(),
            }
            .replace('\\', "/")
        )
    }
}
