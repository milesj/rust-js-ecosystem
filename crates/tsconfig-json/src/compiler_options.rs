#![allow(deprecated)]

use crate::path_types::replace_path_config_dir;
use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHasher};
use serde::{Deserialize, Deserializer};
use std::hash::BuildHasherDefault;
use std::path::{Path, PathBuf};

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
    pub isolated_declarations: Option<bool>,

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

impl CompilerOptions {
    pub fn apply_config_dir(&mut self, config_dir: &Path) {
        if let Some(path) = &mut self.base_url {
            *path = replace_path_config_dir(path, config_dir);
        }

        if let Some(path) = &mut self.declaration_dir {
            *path = replace_path_config_dir(path, config_dir);
        }

        if let Some(path) = &mut self.out_dir {
            *path = replace_path_config_dir(path, config_dir);
        }

        if let Some(path) = &mut self.out_file {
            *path = replace_path_config_dir(path, config_dir);
        }

        if let Some(path) = &mut self.root_dir {
            *path = replace_path_config_dir(path, config_dir);
        }

        if let Some(paths) = &mut self.root_dirs {
            for path in paths.iter_mut() {
                *path = replace_path_config_dir(path, config_dir);
            }
        }

        if let Some(paths) = &mut self.type_roots {
            for path in paths.iter_mut() {
                *path = replace_path_config_dir(path, config_dir);
            }
        }
    }
}

// https://www.typescriptlang.org/tsconfig#jsx
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "kebab-case"))]
pub enum JsxField {
    React,
    ReactJsx,
    ReactJsxdev,
    ReactNative,
    Preserve,
}

impl<'de> Deserialize<'de> for JsxField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Ok(match value.to_lowercase().as_str() {
            "react" => Self::React,
            "reactjsx" | "react-jsx" => Self::ReactJsx,
            "reactjsxdev" | "react-jsxdev" => Self::ReactJsxdev,
            "reactnative" | "react-native" => Self::ReactNative,
            _ => Self::Preserve,
        })
    }
}

// https://www.typescriptlang.org/tsconfig#module
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum ModuleField {
    Amd,
    CommonJs,
    Es6,
    Es2015,
    Es2020,
    Es2022,
    EsNext,
    #[deprecated]
    Node12,
    Node16,
    NodeNext,
    #[default]
    None,
    Preserve, // TS 5.4
    System,
    Umd,
}

impl<'de> Deserialize<'de> for ModuleField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Ok(match value.to_lowercase().as_str() {
            "amd" => Self::Amd,
            "cjs" | "commonjs" => Self::CommonJs,
            "esm" | "es6" => Self::Es6,
            "es2015" => Self::Es2015,
            "es2020" => Self::Es2020,
            "es2022" => Self::Es2022,
            "esnext" => Self::EsNext,
            "node12" => Self::Node12,
            "node16" => Self::Node16,
            "nodenext" => Self::NodeNext,
            "preserve" => Self::Preserve,
            "system" => Self::System,
            "umd" => Self::Umd,
            _ => Self::None,
        })
    }
}

// https://www.typescriptlang.org/tsconfig#moduleDetection
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum ModuleDetectionField {
    Auto,
    Legacy,
    Force,
}

impl<'de> Deserialize<'de> for ModuleDetectionField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Ok(match value.to_lowercase().as_str() {
            "legacy" => Self::Legacy,
            "force" => Self::Force,
            _ => Self::Auto,
        })
    }
}

// https://www.typescriptlang.org/tsconfig#moduleResolution
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum ModuleResolutionField {
    Bundler,
    Classic,
    Node,
    Node10,
    #[deprecated]
    Node12,
    Node16,
    NodeNext,
}

impl<'de> Deserialize<'de> for ModuleResolutionField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Ok(match value.to_lowercase().as_str() {
            "bundler" => Self::Bundler,
            "classic" => Self::Classic,
            "node10" => Self::Node10,
            "node12" => Self::Node12,
            "node16" => Self::Node16,
            "nodenext" => Self::NodeNext,
            _ => Self::Node,
        })
    }
}

// https://www.typescriptlang.org/tsconfig#target
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "lowercase"))]
pub enum TargetField {
    Es3,
    Es5,
    Es6,
    #[deprecated]
    Es7,
    Es2015,
    Es2016,
    Es2017,
    Es2018,
    Es2019,
    Es2020,
    Es2021,
    Es2022,
    EsNext,
}

impl<'de> Deserialize<'de> for TargetField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Ok(match value.to_lowercase().as_str() {
            "es3" => Self::Es3,
            "es5" => Self::Es5,
            "es6" => Self::Es6,
            "es7" => Self::Es7,
            "es2015" => Self::Es2015,
            "es2016" => Self::Es2016,
            "es2017" => Self::Es2017,
            "es2018" => Self::Es2018,
            "es2019" => Self::Es2019,
            "es2020" => Self::Es2020,
            "es2021" => Self::Es2021,
            "es2022" => Self::Es2022,
            _ => Self::EsNext,
        })
    }
}
