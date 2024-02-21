use node_module_graph::ModuleGraph;
use std::env;

fn main() {
    let mut graph = ModuleGraph::new();
    let cwd = env::current_dir().unwrap();

    // let _ = graph.load_module_at_path(
    //     cwd.join("crates/module-graph/tests/__fixtures__/files/javascript.mjs"),
    //     None,
    //     None,
    // );

    let _ = graph.load_module_at_path(
        cwd.join("crates/module-graph/tests/__fixtures__/files/imports.mjs"),
        None,
        None,
    );

    println!("{:#?}", graph);
}
