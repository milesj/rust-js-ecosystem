use serde::Deserialize;
use std::borrow::Cow;
use std::fmt;
use std::path::{Path, PathBuf};

pub(crate) fn replace_path_config_dir(orig_path: &Path, config_dir: &Path) -> PathBuf {
    let mut next_path = PathBuf::new();

    orig_path.into_iter().for_each(|comp| {
        if comp == "${configDir}" {
            next_path.push(config_dir);
        } else {
            next_path.push(comp);
        }
    });

    next_path
}

pub(crate) fn replace_string_config_dir(orig_path: &str, config_dir: &Path) -> String {
    orig_path.replace("${configDir}", &config_dir.to_string_lossy())
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged, from = "String", into = "String")]
pub enum PathOrGlob {
    Path(PathBuf),
    Glob(String),
}

impl PathOrGlob {
    pub fn apply_config_dir(&mut self, config_dir: &Path) {
        match self {
            Self::Path(ref mut path) => {
                *path = replace_path_config_dir(&path, config_dir);
            }
            Self::Glob(ref mut glob) => {
                *glob = replace_string_config_dir(&glob, config_dir);
            }
        }
    }
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

impl From<PathOrGlob> for String {
    fn from(value: PathOrGlob) -> String {
        value.to_string()
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
