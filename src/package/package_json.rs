use std::collections::BTreeMap;

use serde::Deserialize;

pub type DependenciesMap = BTreeMap<String, String>;

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PackageJson {
    name: String,
    dependencies: Option<DependenciesMap>,
    dev_dependencies: Option<DependenciesMap>,
    peer_dependencies: Option<DependenciesMap>,
}
