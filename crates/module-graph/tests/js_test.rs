mod utils;

use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::*;

mod mjs {
    use super::*;

    #[test]
    fn normal_exports() {
        let sandbox = create_sandbox("js");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "mjs/exports.mjs"));
    }
}
