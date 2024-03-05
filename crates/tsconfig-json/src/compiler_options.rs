#![allow(deprecated)]

use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHasher};
use serde::Deserialize;
use std::hash::BuildHasherDefault;
use std::path::PathBuf;

// Note: We only support fields that are extremely common.
// Everything else can be accessed with `other_fields`.

pub type CompilerOptionsPathsMap = IndexMap<String, Vec<String>, BuildHasherDefault<FxHasher>>;

// https://www.typescriptlang.org/tsconfig#compilerOptions
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_js: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_conditions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration_dir: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration_map: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_declaration_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_decorator_metadata: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_module_interop: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_decorators: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolated_modules: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_factory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_fragment_factory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_import_source: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx: Option<JsxField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lib: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<ModuleField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_detection: Option<ModuleDetectionField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_resolution: Option<ModuleResolutionField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_suffixes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_emit: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_dir: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_file: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<CompilerOptionsPathsMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<FxHashMap<String, serde_json::Value>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_json_module: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_package_json_exports: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_package_json_imports: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_dir: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_dirs: Option<Vec<PathBuf>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_lib_check: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_map: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TargetField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_roots: Option<Vec<PathBuf>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbatim_module_syntax: Option<bool>,

    // For all other fields we don't want to explicitly support,
    // but consumers may want to access for some reason
    #[serde(flatten)]
    pub other_fields: FxHashMap<String, serde_json::Value>,
}

// https://www.typescriptlang.org/tsconfig#jsx
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
