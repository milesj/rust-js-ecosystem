use crate::compiler_options::CompilerOptions;
use crate::path_types::PathOrGlob;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TsConfigExtends {
    String(PathBuf),
    Array(Vec<PathBuf>),
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TsConfigJson {
    pub compile_on_save: Option<bool>,

    pub compiler_options: Option<CompilerOptions>,

    pub exclude: Option<Vec<PathOrGlob>>,

    pub extends: Option<TsConfigExtends>,

    pub files: Option<Vec<PathBuf>>,

    pub include: Option<Vec<PathOrGlob>>,

    pub references: Option<Vec<ProjectReference>>,

    pub type_acquisition: Option<TypeAcquisition>,

    #[serde(flatten)]
    pub other_fields: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct ProjectReference {
    pub path: PathBuf,
    pub prepend: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeAcquisition {
    pub enable: bool,

    pub include: Option<Vec<String>>,

    pub exclude: Option<Vec<String>>,

    pub disable_filename_based_type_acquisition: Option<bool>,
}
