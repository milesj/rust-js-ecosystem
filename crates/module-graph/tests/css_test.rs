mod utils;

use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::generate_graph_for_file;

mod css {
    use super::*;

    #[test]
    fn normal() {
        let sandbox = create_sandbox("css");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "styles.css"));
    }

    #[test]
    fn modules() {
        let sandbox = create_sandbox("css");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "styles.module.css"));
    }
}
