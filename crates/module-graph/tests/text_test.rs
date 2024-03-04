mod utils;

use starbase_sandbox::{assert_snapshot, create_sandbox};
use utils::generate_graph_for_file;

mod text {
    use super::*;

    #[test]
    fn graphql() {
        let sandbox = create_sandbox("text");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "graphql.gql"));
    }

    #[test]
    fn graphql_alt_extension() {
        let sandbox = create_sandbox("text");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "graphql.graphql"));
    }

    #[test]
    fn html() {
        let sandbox = create_sandbox("text");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "index.html"));
    }

    #[test]
    fn less() {
        let sandbox = create_sandbox("text");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "styles.less"));
    }

    #[test]
    fn map() {
        let sandbox = create_sandbox("text");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "source.js.map"));
    }

    #[test]
    fn sass() {
        let sandbox = create_sandbox("text");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "styles.sass"));
    }

    #[test]
    fn scss() {
        let sandbox = create_sandbox("text");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "styles.scss"));
    }

    #[test]
    fn stylus() {
        let sandbox = create_sandbox("text");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "styles.styl"));
    }

    #[test]
    fn svg() {
        let sandbox = create_sandbox("text");

        assert_snapshot!(generate_graph_for_file(sandbox.path(), "icon.svg"));
    }
}
