use super::version_protocol::VersionProtocol;
use serde::Deserialize;
use starbase_utils::json;
use std::collections::BTreeMap;
use std::path::Path;

pub type DependenciesMap = BTreeMap<String, VersionProtocol>;

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PackageJson {
    pub name: String,
    pub dependencies: Option<DependenciesMap>,
    pub dependencies_meta: Option<BTreeMap<String, DependencyMeta>>,
    pub dev_dependencies: Option<DependenciesMap>,
    pub peer_dependencies: Option<DependenciesMap>,
    pub peer_dependencies_meta: Option<BTreeMap<String, PeerDependencyMeta>>,
    pub bundle_dependencies: Option<Vec<String>>,
    pub workspaces: Option<Workspaces>,
}

impl PackageJson {
    pub fn load<T: AsRef<Path>>(file: T) -> miette::Result<Self> {
        Ok(json::read_file(file.as_ref())?)
    }
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DependencyMeta {
    // pnpm - https://pnpm.io/package_json#dependenciesmeta
    injected: bool,
    // yarn - https://yarnpkg.com/configuration/manifest#dependenciesMeta
    built: bool,
    optional: bool,
    unplugged: bool,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PeerDependencyMeta {
    optional: bool,
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
