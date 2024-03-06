mod utils;

use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::*;

mod ts {
    use super::*;

    #[test]
    fn export_star() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "ts/export-star.ts"));
    }

    #[test]
    fn export_named() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "ts/export-named.ts"
        ));
    }

    #[test]
    fn export_default_function() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "ts/export-def-func.ts"
        ));
        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "ts/export-def-anon-func.ts"
        ));
    }

    #[test]
    fn export_default_class() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "ts/export-def-class.ts"
        ));
        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "ts/export-def-anon-class.ts"
        ));
    }

    #[test]
    fn export_default_reference() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "ts/export-def-ref.ts"
        ));
    }

    #[test]
    fn export_default_interface() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "ts/export-def-interface.ts"
        ));
    }

    // This doesn't seem to generate exports,
    // even though oxc is a valid enum type!
    #[test]
    fn export_default_enum() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(
            sandbox.path(),
            "ts/export-def-enum.ts"
        ));
    }
}
