use std::path::PathBuf;

use crate::compiler_options::CompilerOptions;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TsConfigExtends {
    String(String),
    Array(Vec<String>),
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TsConfigJson {
    pub compile_on_save: Option<bool>,

    pub compiler_options: Option<CompilerOptions>,

    pub exclude: Option<Vec<String>>,

    pub extends: Option<TsConfigExtends>,

    pub files: Option<Vec<String>>,

    pub include: Option<Vec<String>>,

    pub references: Option<Vec<ProjectReference>>,

    pub type_acquisition: Option<TypeAcquisition>,
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
