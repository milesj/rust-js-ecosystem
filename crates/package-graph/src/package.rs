use crate::package_graph_error::PackageGraphError;
use nodejs_package_json::PackageJson;
use petgraph::graph::NodeIndex;
use starbase_utils::json;
use std::path::{Path, PathBuf};

#[derive(Debug)]
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

    pub fn load<T: AsRef<Path>>(root: T) -> Result<Self, PackageGraphError> {
        let root = root.as_ref();
        let manifest: PackageJson = json::read_file(root.join("package.json"))?;

        Ok(Self::new(root.to_owned(), manifest))
    }

    pub fn get_name(&self) -> Result<&str, PackageGraphError> {
        self.manifest
            .name
            .as_deref()
            .ok_or_else(|| PackageGraphError::MissingPackageName(self.root.clone()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DependencyType {
    Development,
    Production,
    Peer,
    Optional,
}
