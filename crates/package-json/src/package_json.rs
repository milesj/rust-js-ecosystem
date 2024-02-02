use crate::protocols::VersionProtocol;
use crate::{import_export::*, FxIndexMap};
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

pub type DependenciesMap = BTreeMap<String, VersionProtocol>;
pub type EnginesMap = FxIndexMap<String, VersionProtocol>;
pub type ScriptsMap = FxIndexMap<String, String>;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<Version>,
    pub scripts: Option<ScriptsMap>,

    // Entry points
    pub main: Option<PathBuf>,
    pub module: Option<PathBuf>,
    pub browser: Option<BrowserField>,
    pub imports: Option<ImportExportMap>,
    pub exports: Option<ImportExportField>,

    // Dependencies
    pub dependencies: Option<DependenciesMap>,
    pub dependencies_meta: Option<BTreeMap<String, DependencyMetaField>>,
    pub dev_dependencies: Option<DependenciesMap>,
    pub peer_dependencies: Option<DependenciesMap>,
    pub peer_dependencies_meta: Option<BTreeMap<String, PeerDependencyMetaField>>,
    pub bundle_dependencies: Option<Vec<String>>,
    pub optional_dependencies: Option<DependenciesMap>,

    // Constraints
    pub engines: Option<EnginesMap>,
    pub package_manager: Option<String>,

    // Workspaces
    pub workspaces: Option<WorkspacesField>,

    // For all other fields we don't want to explicitly support,
    // but consumers may want to access for some reason
    #[serde(flatten)]
    pub other_fields: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BrowserField {
    String(String),
    Map(BTreeMap<PathBuf, serde_json::Value>),
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct DependencyMetaField {
    // pnpm - https://pnpm.io/package_json#dependenciesmeta
    pub injected: bool,
    // yarn - https://yarnpkg.com/configuration/manifest#dependenciesMeta
    pub built: bool,
    pub optional: bool,
    pub unplugged: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct PeerDependencyMetaField {
    pub optional: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkspacesField {
    Globs(Vec<String>),
    Config {
        packages: Vec<String>,
        nohoist: Option<Vec<String>>,
    },
}
