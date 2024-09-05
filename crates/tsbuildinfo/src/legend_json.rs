use serde::Deserialize;
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct LegendItem {
    pub config_file_path: PathBuf,
    pub trace_path: PathBuf,
    pub types_path: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct LegendJson(pub Vec<LegendItem>);

impl Deref for LegendJson {
    type Target = Vec<LegendItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
