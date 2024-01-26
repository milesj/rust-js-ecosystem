use super::package_json::PackageJson;
use petgraph::graph::NodeIndex;
use std::path::{Path, PathBuf};

pub struct Package {
    pub manifest: PackageJson,
    pub root: PathBuf,

    // Index based on load + insertion order.
    // The root package is always 0.
    pub index: u8,

    // Index in the package graph.
    pub node_index: NodeIndex,
}

impl Package {
    pub fn new(root: PathBuf, manifest: PackageJson) -> Self {
        Self {
            manifest,
            root,
            index: 0,
            node_index: NodeIndex::new(0),
        }
    }

    pub fn load<T: AsRef<Path>>(root: T) -> miette::Result<Self> {
        let root = root.as_ref();
        let manifest: PackageJson = PackageJson::load(root.join("package.json"))?;

        Ok(Self::new(root.to_owned(), manifest))
    }
}

#[derive(Clone, Copy)]
pub enum DependencyType {
    Development,
    Production,
    Peer,
    Optional,
}
