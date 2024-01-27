use super::version_protocol::VersionProtocol;
use semver::Version;
use serde::Deserialize;
use starbase_utils::json;
use std::collections::BTreeMap;
use std::path::Path;

pub type DependenciesMap = BTreeMap<String, VersionProtocol>;

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PackageJson {
    pub name: String,
    pub version: Option<Version>,

    // Dependencies
    pub dependencies: Option<DependenciesMap>,
    pub dependencies_meta: Option<BTreeMap<String, DependencyMeta>>,
    pub dev_dependencies: Option<DependenciesMap>,
    pub peer_dependencies: Option<DependenciesMap>,
    pub peer_dependencies_meta: Option<BTreeMap<String, PeerDependencyMeta>>,
    pub bundle_dependencies: Option<Vec<String>>,
    pub optional_dependencies: Option<DependenciesMap>,

    // Workspaces
    pub workspaces: Option<Workspaces>,

    // For all other fields we don't want to explicitly support
    #[serde(flatten)]
    pub unknown: BTreeMap<String, serde_json::Value>,
}

impl PackageJson {
    pub fn load<T: AsRef<Path>>(file: T) -> miette::Result<Self> {
        Ok(json::read_file(file.as_ref())?)
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DependencyMeta {
    // pnpm - https://pnpm.io/package_json#dependenciesmeta
    pub injected: bool,
    // yarn - https://yarnpkg.com/configuration/manifest#dependenciesMeta
    pub built: bool,
    pub optional: bool,
    pub unplugged: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PeerDependencyMeta {
    pub optional: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Workspaces {
    Globs(Vec<String>),
    Config {
        packages: Vec<String>,
        nohoist: Option<Vec<String>>,
    },
}
