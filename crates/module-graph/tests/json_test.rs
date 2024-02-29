mod utils;

use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::generate_graph_for_file;

mod json {
    use super::*;

    #[test]
    fn object() {
        let sandbox = create_sandbox("json");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "object.json"));
    }

    #[test]
    fn array() {
        let sandbox = create_sandbox("json");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "array.json"));
    }

    #[test]
    fn primitive() {
        let sandbox = create_sandbox("json");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "primitive.json"));
    }
}
