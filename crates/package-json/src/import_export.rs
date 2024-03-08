use crate::FxIndexMap;
use serde::Deserialize;
use std::fmt;

pub type ImportExportMap = FxIndexMap<ImportExportKey, ImportExportField>;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged)]
pub enum ImportExportField {
    #[default]
    None, // For `undefined` or `null` value.
    String(String),
    Array(Vec<ImportExportField>),
    Map(ImportExportMap),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged, from = "String", into = "String")]
pub enum ImportExportKey {
    Main,
    Pattern(String),
    CustomCondition(String),
}

impl From<&str> for ImportExportKey {
    fn from(key: &str) -> Self {
        if key == "." {
            Self::Main
        } else if key.starts_with("./") {
            Self::Pattern(key.trim_start_matches('.').to_string())
        } else if key.starts_with('#') {
            Self::Pattern(key.to_string())
        } else {
            Self::CustomCondition(key.to_string())
        }
    }
}

impl From<String> for ImportExportKey {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<ImportExportKey> for String {
    fn from(value: ImportExportKey) -> String {
        value.to_string()
    }
}

impl fmt::Display for ImportExportKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Main => ".".to_owned(),
                Self::Pattern(pat) =>
                    if pat.starts_with('#') {
                        pat.to_owned()
                    } else {
                        format!(".{pat}")
                    },
                Self::CustomCondition(con) => con.to_owned(),
            }
        )
    }
}
