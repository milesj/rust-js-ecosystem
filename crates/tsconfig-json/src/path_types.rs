use clean_path::Clean;
use serde::Deserialize;
use std::borrow::Cow;
use std::fmt;
use std::path::{Path, PathBuf};

pub(crate) fn replace_path_config_dir(
    orig_path: &Path,
    source_dir: &Path, // Dir where the config exists
    target_dir: &Path, // Dir of the project to resolve against
) -> PathBuf {
    let has_config_token = orig_path.iter().any(|comp| comp == "${configDir}");

    if !has_config_token {
        return source_dir.join(orig_path).clean();
    }

    orig_path
        .iter()
        .map(|comp| {
            if comp == "${configDir}" {
                target_dir.as_os_str()
            } else {
                comp
            }
        })
        .collect()
}

pub(crate) fn replace_string_config_dir(
    orig_path: &str,
    source_dir: &Path,
    target_dir: &Path,
) -> String {
    if orig_path.contains("${configDir}") {
        orig_path.replace("${configDir}", &target_dir.to_string_lossy())
    } else {
        source_dir
            .join(orig_path)
            .clean()
            .to_string_lossy()
            .to_string()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged, from = "String", into = "String")]
pub enum PathOrGlob {
    Path(PathBuf),
    Glob(String),
}

impl PathOrGlob {
    pub fn expand(&mut self, source_dir: &Path, target_dir: &Path) {
        match self {
            Self::Path(ref mut path) => {
                *path = replace_path_config_dir(path, source_dir, target_dir);
            }
            Self::Glob(ref mut glob) => {
                *glob = replace_string_config_dir(glob, source_dir, target_dir);
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
