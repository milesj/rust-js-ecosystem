#![allow(deprecated)]

use crate::path_types::PathOrUrl;
use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHasher};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::hash::BuildHasherDefault;
use std::path::PathBuf;

pub type CompilerOptionsPathsMap = IndexMap<String, Vec<PathBuf>, BuildHasherDefault<FxHasher>>;

// https://www.typescriptlang.org/tsconfig#compilerOptions
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    pub allow_js: Option<bool>,

    pub allow_arbitrary_extensions: Option<bool>,

    pub allow_importing_ts_extensions: Option<bool>,

    pub allow_synthetic_default_imports: Option<bool>,

    pub allow_umd_global_access: Option<bool>,

    pub allow_unreachable_code: Option<bool>,

    pub allow_unused_labels: Option<bool>,

    pub always_strict: Option<bool>,

    pub assume_changes_only_affect_direct_dependencies: Option<bool>,

    pub base_url: Option<PathBuf>,

    pub check_js: Option<bool>,

    pub composite: Option<bool>,

    pub custom_conditions: Option<Vec<String>>,

    pub declaration_dir: Option<PathBuf>,

    pub declaration_map: Option<bool>,

    pub declaration: Option<bool>,

    pub diagnostics: Option<bool>,

    pub disable_referenced_project_load: Option<bool>,

    pub disable_size_limit: Option<bool>,

    pub disable_solution_searching: Option<bool>,

    pub disable_source_of_project_reference_redirect: Option<bool>,

    pub downlevel_iteration: Option<bool>,

    #[serde(rename = "emitBOM")]
    pub emit_bom: Option<bool>,

    pub emit_declaration_only: Option<bool>,

    pub emit_decorator_metadata: Option<bool>,

    pub es_module_interop: Option<bool>,

    pub exact_optional_property_types: Option<bool>,

    pub experimental_decorators: Option<bool>,

    pub explain_files: Option<bool>,

    pub extended_diagnostics: Option<bool>,

    pub force_consistent_casing_in_file_names: Option<bool>,

    pub generate_cpu_profile: Option<bool>,

    pub import_helpers: Option<bool>,

    pub incremental: Option<bool>,

    pub inline_source_map: Option<bool>,

    pub inline_sources: Option<bool>,

    pub isolated_modules: Option<bool>,

    pub jsx_factory: Option<String>,

    pub jsx_fragment_factory: Option<String>,

    pub jsx_import_source: Option<String>,

    pub jsx: Option<JsxField>,

    pub lib: Option<Vec<String>>,

    pub list_emitted_files: Option<bool>,

    pub list_files: Option<bool>,

    pub map_root: Option<PathOrUrl>,

    pub max_node_module_js_depth: Option<u32>,

    pub module: Option<ModuleField>,

    pub module_detection: Option<ModuleDetectionField>,

    pub module_resolution: Option<ModuleResolutionField>,

    pub module_suffixes: Option<Vec<String>>,

    pub new_line: Option<String>,

    pub no_emit_helpers: Option<bool>,

    pub no_emit_on_error: Option<bool>,

    pub no_emit: Option<bool>,

    pub no_error_truncation: Option<bool>,

    pub no_fallthrough_cases_in_switch: Option<bool>,

    pub no_implicit_any: Option<bool>,

    pub no_implicit_override: Option<bool>,

    pub no_implicit_returns: Option<bool>,

    pub no_implicit_this: Option<bool>,

    pub no_lib: Option<bool>,

    pub no_property_access_from_index_signature: Option<bool>,

    pub no_resolve: Option<bool>,

    pub no_unchecked_indexed_access: Option<bool>,

    pub no_unused_locals: Option<bool>,

    pub no_unused_parameters: Option<bool>,

    pub out_dir: Option<PathBuf>,

    pub out_file: Option<PathBuf>,

    pub paths: Option<CompilerOptionsPathsMap>,

    pub plugins: Option<Vec<FxHashMap<String, serde_json::Value>>>,

    pub preserve_const_enums: Option<bool>,

    pub preserve_symlinks: Option<bool>,

    pub preserve_watch_output: Option<bool>,

    pub pretty: Option<bool>,

    #[deprecated]
    pub react_namespace: Option<String>,

    pub remove_comments: Option<bool>,

    pub resolve_json_module: Option<bool>,

    pub resolve_package_json_exports: Option<bool>,

    pub resolve_package_json_imports: Option<bool>,

    pub root_dir: Option<PathBuf>,

    pub root_dirs: Option<Vec<PathBuf>>,

    pub skip_default_lib_check: Option<bool>,

    pub skip_lib_check: Option<bool>,

    pub source_map: Option<bool>,

    pub source_root: Option<PathOrUrl>,

    pub strict_bind_call_apply: Option<bool>,

    pub strict_function_types: Option<bool>,

    pub strict_null_checks: Option<bool>,

    pub strict_property_initialization: Option<bool>,

    pub strict: Option<bool>,

    pub strip_internal: Option<bool>,

    pub target: Option<TargetField>,

    pub trace_resolution: Option<bool>,

    pub ts_build_info_file: Option<String>,

    pub type_roots: Option<Vec<PathBuf>>,

    pub types: Option<Vec<String>>,

    pub use_define_for_class_fields: Option<bool>,

    pub use_unknown_in_catch_variables: Option<bool>,

    pub verbatim_module_syntax: Option<bool>,

    pub watch_options: Option<WatchOptions>,

    #[deprecated]
    pub charset: Option<String>,

    #[deprecated]
    pub imports_not_used_as_values: Option<String>,

    #[deprecated]
    pub keyof_strings_only: Option<bool>,

    #[deprecated]
    pub no_implicit_use_strict: Option<bool>,

    #[deprecated]
    pub no_strict_generic_checks: Option<bool>,

    #[deprecated]
    pub out: Option<PathBuf>,

    #[deprecated]
    pub preserve_value_imports: Option<bool>,

    #[deprecated]
    pub suppress_excess_property_errors: Option<bool>,

    #[deprecated]
    pub suppress_implicit_any_index_errors: Option<bool>,

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

// https://www.typescriptlang.org/tsconfig#watch-options
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WatchOptions {
    pub exclude_directories: Option<Vec<String>>,

    pub exclude_files: Option<Vec<String>>,

    pub fallback_polling: Option<String>,

    pub synchronous_watch_directory: Option<bool>,

    pub watch_directory: Option<String>,

    pub watch_file: Option<String>,
}
