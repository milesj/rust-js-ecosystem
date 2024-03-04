mod utils;

use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::*;

mod json {
    use super::*;

    #[test]
    fn supports_json5() {
        let sandbox = create_sandbox("json");
        let module = generate_module_for_file(sandbox.path(), "object.json5");

        assert!(!module.exports.is_empty());
    }

    #[test]
    fn supports_jsonc() {
        let sandbox = create_sandbox("json");
        let module = generate_module_for_file(sandbox.path(), "object.jsonc");

        assert!(!module.exports.is_empty());
    }

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
