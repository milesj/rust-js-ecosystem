mod utils;

use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::generate_graph_for_file;

mod yaml {
    use super::*;

    #[test]
    fn object() {
        let sandbox = create_sandbox("yaml");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "object.yaml"));
    }

    #[test]
    fn array() {
        let sandbox = create_sandbox("yaml");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "array.yaml"));
    }

    #[test]
    fn primitive() {
        let sandbox = create_sandbox("yaml");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "primitive.yaml"));
    }
}
