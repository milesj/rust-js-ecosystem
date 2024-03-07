mod utils;

use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::*;

mod mjs {
    use super::*;

    #[test]
    fn export_star() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "mjs/export-star.mjs"
        ));
    }

    #[test]
    fn export_named() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "mjs/export-named.mjs"
        ));
    }

    #[test]
    fn export_default_function() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "mjs/export-def-func.mjs"
        ));
        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "mjs/export-def-anon-func.mjs"
        ));
    }

    #[test]
    fn export_default_class() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "mjs/export-def-class.mjs"
        ));
        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "mjs/export-def-anon-class.mjs"
        ));
    }

    #[test]
    fn export_default_reference() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "mjs/export-def-ref.mjs"
        ));
    }

    #[test]
    fn dynamic_import_top_level_await() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "mjs/dyn-import-tla.mjs"
        ));
    }

    #[test]
    fn dynamic_import_scope_depths() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "mjs/dyn-import-scopes.mjs"
        ));
    }

    #[test]
    fn dynamic_import_destructure_patterns() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "mjs/dyn-import-patterns.mjs"
        ));
    }
}
