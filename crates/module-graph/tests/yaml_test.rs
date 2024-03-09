mod utils;

use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::*;

mod yaml {
    use super::*;

    #[test]
    fn supports_yml_extension() {
        let sandbox = create_sandbox("yaml");
        let module = generate_module_for_file(sandbox.path(), "object.yml");

        assert!(!module.exports.is_empty());
    }

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
