use crate::FxIndexMap;
use serde::Deserialize;

pub type ImportExportMap = FxIndexMap<ImportExportKey, ImportExportField>;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ImportExportField {
    #[default]
    None, // For `undefined` or `null` value.
    String(String),
    Array(Vec<ImportExportField>),
    Map(ImportExportMap),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(untagged, from = "String")]
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
