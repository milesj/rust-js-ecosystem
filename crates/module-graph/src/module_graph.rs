use crate::module::*;
use crate::module_graph_error::ModuleGraphError;
use clean_path::Clean;
use oxc_resolver::{ResolveOptions, Resolver};
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use rustc_hash::FxHashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug)]
pub enum ModuleGraphEdge {
    Import,
    Export,
}

pub type ModuleGraphType = GraphMap<ModuleId, ModuleGraphEdge, Directed>;

#[derive(Debug)]
pub struct ModuleGraph {
    pub graph: ModuleGraphType,
    pub modules: FxHashMap<ModuleId, Arc<Module>>,
    pub resolver: Resolver,

    id_count: u32,
    paths_to_ids: FxHashMap<PathBuf, ModuleId>,
}

impl ModuleGraph {
    pub fn new() -> Self {
        Self {
            graph: GraphMap::default(),
            modules: FxHashMap::default(),
            resolver: Resolver::new(ResolveOptions {
                condition_names: vec![
                    "import".into(),
                    "module".into(),
                    "require".into(),
                    "node".into(),
                    "default".into(),
                ],
                extensions: vec![
                    ".ts".into(),
                    ".tsx".into(),
                    ".mts".into(),
                    ".cts".into(),
                    ".mjs".into(),
                    ".cjs".into(),
                    ".js".into(),
                    ".jsx".into(),
                ],
                main_fields: vec!["module".into(), "main".into()],
                ..ResolveOptions::default()
            }),
            id_count: 1, // Default/empty modules are 0
            paths_to_ids: FxHashMap::default(),
        }
    }

    pub fn load_module(
        &mut self,
        parent_dir: &Path,
        specifier: &str,
    ) -> Result<ModuleId, ModuleGraphError> {
        let resolved_path = self.resolver.resolve(parent_dir, specifier).unwrap();

        // Module already exists in the graph, avoid duplicates
        if let Some(module_id) = self.paths_to_ids.get(resolved_path.path()) {
            return Ok(*module_id);
        }

        let parent_dir = resolved_path.path().parent().unwrap();

        // Generate the ID and add to the graph
        let module_id = self.graph.add_node(self.id_count);
        let absolute_path = resolved_path.path().to_path_buf().clean();

        self.id_count += 1;
        self.paths_to_ids.insert(absolute_path.clone(), module_id);

        // Load and parse the module, then add to the graph
        let mut module = Module {
            id: module_id,
            fragment: resolved_path.fragment().map(|frag| frag.to_owned()),
            path: absolute_path,
            query: resolved_path.query().map(|query| query.to_owned()),
            ..Module::default()
        };

        module.load_and_parse_source()?;

        // Load each imported and exported module, then connect edges
        for import in module.imports.iter_mut() {
            import.module_id = self.load_module(parent_dir, &import.source)?;

            self.graph
                .add_edge(module_id, import.module_id, ModuleGraphEdge::Import);
        }

        for export in module.exports.iter_mut() {
            let Some(source) = &export.source else {
                continue;
            };

            export.module_id = self.load_module(parent_dir, source)?;

            self.graph
                .add_edge(module_id, export.module_id, ModuleGraphEdge::Export);
        }

        // Store the module in the graph
        self.modules.insert(module_id, Arc::new(module));

        Ok(module_id)
    }
}
