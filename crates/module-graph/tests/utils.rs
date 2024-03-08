#![allow(dead_code)]

use nodejs_module_graph::{Module, ModuleGraph};
use std::path::Path;

pub fn generate_module_for_file(
    root: impl AsRef<Path>,
    file: impl AsRef<str>,
) -> std::sync::Arc<Module> {
    let root = root.as_ref();

    let mut graph = ModuleGraph::new();
    let id = graph
        .load_module_at_path(root.join(file.as_ref()), None, None, None)
        .unwrap();

    graph.modules.swap_remove(&id).unwrap()
}

pub fn generate_graph_for_file(root: impl AsRef<Path>, file: impl AsRef<str>) -> String {
    let root = root.as_ref();

    let mut graph = ModuleGraph::new();
    let _ = graph
        .load_module_at_path(root.join(file.as_ref()), None, None, None)
        .unwrap();

    // Remove file paths so snapshots are deterministic
    format!("{:#?}", graph.modules)
        .replace(root.to_string_lossy().to_string().as_str(), "/root")
        .replace("/private", "")
}
