use crate::protocols::VersionProtocol;
use crate::{import_export::*, FxIndexMap};
use rustc_hash::FxHashMap;
use semver::Version;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

// Note: We only support fields that we actually need and
// are useful. Everything else can be accessed with `other_fields`.
//
// Why not support all fields? Because it's unsafe. There are
// far too many packages out there, many with invalid fields,
// values, or types, that would fail the serde process.

pub type DependenciesMap<T> = BTreeMap<String, T>;
pub type EnginesMap = FxIndexMap<String, VersionProtocol>;
pub type ScriptsMap = FxIndexMap<String, String>;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[cfg(feature = "protocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,

    #[cfg(not(feature = "protocols"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_of: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<ScriptsMap>,

    // Entry points
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<BrowserField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub imports: Option<ImportExportMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports: Option<ImportExportField>,

    // Dependencies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<DependenciesMap<VersionProtocol>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_meta: Option<DependenciesMap<DependencyMetaField>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<DependenciesMap<VersionProtocol>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<DependenciesMap<VersionProtocol>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_dependencies_meta: Option<DependenciesMap<PeerDependencyMetaField>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_dependencies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_dependencies: Option<DependenciesMap<VersionProtocol>>,

    // Constraints
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<EnginesMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_manager: Option<String>,

    // Workspaces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<WorkspacesField>,

    // For all other fields we don't want to explicitly support,
    // but consumers may want to access for some reason
    #[serde(flatten)]
    pub other_fields: FxHashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged)]
pub enum BrowserField {
    String(String),
    Map(FxHashMap<PathBuf, serde_json::Value>),
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct DependencyMetaField {
    // pnpm - https://pnpm.io/package_json#dependenciesmeta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub injected: Option<bool>,

    // yarn - https://yarnpkg.com/configuration/manifest#dependenciesMeta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unplugged: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct PeerDependencyMetaField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkspacesField {
    Globs(Vec<String>),
    Config {
        packages: Vec<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        nohoist: Option<Vec<String>>,
    },
}
