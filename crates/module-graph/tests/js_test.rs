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
}
