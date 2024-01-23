use super::package_json::PackageJson;
use std::path::PathBuf;

pub struct Package {
    pub manifest: PackageJson,
    pub root: PathBuf,
}
