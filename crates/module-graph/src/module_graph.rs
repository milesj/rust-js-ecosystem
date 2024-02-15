use crate::module::Module;
use crate::module_graph_error::ModuleGraphError;
use nodejs_package_json::PackageJson;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use rustc_hash::FxHashMap;
use std::path::Path;

pub type ModuleId = u32;

pub type ModuleGraphType = GraphMap<ModuleId, (), Directed>;

pub struct ModuleGraph {
    pub graph: ModuleGraphType,
    pub modules: FxHashMap<ModuleId, Module>,
}

impl ModuleGraph {
    pub fn generate_from_file<T: AsRef<Path>>(path: T) {
        let path = path.as_ref();

        assert!(path.is_absolute(), "Path must be absolute!");
    }

    pub fn generate_from_package(_package_json: &PackageJson) {}

    // fn parse_and_create_module() -> Result<Module, ModuleGraphError> {}
}
