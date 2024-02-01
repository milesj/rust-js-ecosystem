use super::protocols::VersionProtocol;
use semver::Version;
use serde::Deserialize;
use std::collections::BTreeMap;

pub type DependenciesMap = BTreeMap<String, VersionProtocol>;

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<Version>,

    // Dependencies
    pub dependencies: Option<DependenciesMap>,
    pub dependencies_meta: Option<BTreeMap<String, DependencyMetaField>>,
    pub dev_dependencies: Option<DependenciesMap>,
    pub peer_dependencies: Option<DependenciesMap>,
    pub peer_dependencies_meta: Option<BTreeMap<String, PeerDependencyMetaField>>,
    pub bundle_dependencies: Option<Vec<String>>,
    pub optional_dependencies: Option<DependenciesMap>,

    // Workspaces
    pub workspaces: Option<WorkspacesField>,

    // For all other fields we don't want to explicitly support,
    // but consumers may want to access for some reason
    #[serde(flatten)]
    pub unknown: BTreeMap<String, serde_json::Value>,
}

#[cfg(feature = "loader")]
impl PackageJson {
    pub fn load<T: AsRef<std::path::Path>>(file: T) -> miette::Result<Self> {
        Ok(starbase_utils::json::read_file(file.as_ref())?)
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DependencyMetaField {
    // pnpm - https://pnpm.io/package_json#dependenciesmeta
    pub injected: bool,
    // yarn - https://yarnpkg.com/configuration/manifest#dependenciesMeta
    pub built: bool,
    pub optional: bool,
    pub unplugged: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PeerDependencyMetaField {
    pub optional: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkspacesField {
    Globs(Vec<String>),
    Config {
        packages: Vec<String>,
        nohoist: Option<Vec<String>>,
    },
}
