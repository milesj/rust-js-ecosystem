#![allow(deprecated)]

use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHasher};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::hash::BuildHasherDefault;
use std::path::PathBuf;

// Note: We only support fields that are extremely common.
// Everything else can be accessed with `other_fields`.

pub type CompilerOptionsPathsMap = IndexMap<String, Vec<PathBuf>, BuildHasherDefault<FxHasher>>;

// https://www.typescriptlang.org/tsconfig#compilerOptions
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    pub allow_js: Option<bool>,
    pub base_url: Option<PathBuf>,
    pub composite: Option<bool>,
    pub custom_conditions: Option<Vec<String>>,
    pub declaration_dir: Option<PathBuf>,
    pub declaration_map: Option<bool>,
    pub declaration: Option<bool>,
    pub emit_declaration_only: Option<bool>,
    pub emit_decorator_metadata: Option<bool>,
    pub es_module_interop: Option<bool>,
    pub experimental_decorators: Option<bool>,
    pub incremental: Option<bool>,
    pub isolated_modules: Option<bool>,
    pub jsx_factory: Option<String>,
    pub jsx_fragment_factory: Option<String>,
    pub jsx_import_source: Option<String>,
    pub jsx: Option<JsxField>,
    pub lib: Option<Vec<String>>,
    pub module: Option<ModuleField>,
    pub module_detection: Option<ModuleDetectionField>,
    pub module_resolution: Option<ModuleResolutionField>,
    pub module_suffixes: Option<Vec<String>>,
    pub no_emit: Option<bool>,
    pub out_dir: Option<PathBuf>,
    pub out_file: Option<PathBuf>,
    pub paths: Option<CompilerOptionsPathsMap>,
    pub plugins: Option<Vec<FxHashMap<String, serde_json::Value>>>,
    pub pretty: Option<bool>,
    pub resolve_json_module: Option<bool>,
    pub resolve_package_json_exports: Option<bool>,
    pub resolve_package_json_imports: Option<bool>,
    pub root_dir: Option<PathBuf>,
    pub root_dirs: Option<Vec<PathBuf>>,
    pub skip_lib_check: Option<bool>,
    pub source_map: Option<bool>,
    pub strict: Option<bool>,
    pub target: Option<TargetField>,
    pub type_roots: Option<Vec<PathBuf>>,
    pub types: Option<Vec<String>>,
    pub verbatim_module_syntax: Option<bool>,

    // For all other fields we don't want to explicitly support,
    // but consumers may want to access for some reason
    #[serde(flatten)]
    pub other_fields: BTreeMap<String, serde_json::Value>,
}

// https://www.typescriptlang.org/tsconfig#jsx
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum JsxField {
    #[serde(alias = "react")]
    React,
    #[serde(alias = "react-jsx")]
    ReactJsx,
    #[serde(alias = "react-jsxdev")]
    ReactJsxdev,
    #[serde(alias = "react-native")]
    ReactNative,
    #[serde(alias = "preserve")]
    Preserve,
}

// https://www.typescriptlang.org/tsconfig#module
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub enum ModuleField {
    #[serde(alias = "amd")]
    Amd,
    #[serde(alias = "commonjs")]
    CommonJs,
    #[serde(alias = "es6")]
    Es6,
    #[serde(alias = "es2015")]
    Es2015,
    #[serde(alias = "es2020")]
    Es2020,
    #[serde(alias = "es2022")]
    Es2022,
    #[serde(alias = "esnext")]
    EsNext,
    #[deprecated]
    #[serde(alias = "node12")]
    Node12,
    #[serde(alias = "node16")]
    Node16,
    #[serde(alias = "nodenext")]
    NodeNext,
    #[default]
    #[serde(alias = "none")]
    None,
    #[serde(alias = "system")]
    System,
    #[serde(alias = "umd")]
    Umd,
}

// https://www.typescriptlang.org/tsconfig#moduleDetection
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum ModuleDetectionField {
    #[serde(alias = "auto")]
    Auto,
    #[serde(alias = "legacy")]
    Legacy,
    #[serde(alias = "force")]
    Force,
}

// https://www.typescriptlang.org/tsconfig#moduleResolution
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum ModuleResolutionField {
    #[serde(alias = "bundler")]
    Bundler,
    #[serde(alias = "classic")]
    Classic,
    #[serde(alias = "node")]
    Node,
    #[serde(alias = "node10")]
    Node10,
    #[deprecated]
    #[serde(alias = "node12")]
    Node12,
    #[serde(alias = "node16")]
    Node16,
    #[serde(alias = "nodenext")]
    NodeNext,
}

// https://www.typescriptlang.org/tsconfig#target
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum TargetField {
    #[serde(alias = "es3")]
    Es3,
    #[serde(alias = "es5")]
    Es5,
    #[serde(alias = "es6")]
    Es6,
    #[deprecated]
    #[serde(alias = "es7")]
    Es7,
    #[serde(alias = "es2015")]
    Es2015,
    #[serde(alias = "es2016")]
    Es2016,
    #[serde(alias = "es2017")]
    Es2017,
    #[serde(alias = "es2018")]
    Es2018,
    #[serde(alias = "es2019")]
    Es2019,
    #[serde(alias = "es2020")]
    Es2020,
    #[serde(alias = "es2021")]
    Es2021,
    #[serde(alias = "es2022")]
    Es2022,
    #[serde(alias = "esnext")]
    EsNext,
}
