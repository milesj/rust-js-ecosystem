use graphity::PackageGraph;
use starbase_sandbox::{assert_snapshot, create_sandbox};

mod polyrepo {
    use super::*;

    #[test]
    fn loads_the_root_package() {
        let sandbox = create_sandbox("graph-poly");
        let graph = PackageGraph::generate(sandbox.path()).unwrap();

        assert_snapshot!(graph.to_dot())
    }
}

mod monorepo_npm {
    use super::*;

    #[test]
    fn loads_all_packages() {
        let sandbox = create_sandbox("graph-mono-npm");
        let graph = PackageGraph::generate(sandbox.path()).unwrap();

        assert_snapshot!(graph.to_dot())
    }
}

mod monorepo_pnpm {
    use super::*;

    #[test]
    fn loads_all_packages() {
        let sandbox = create_sandbox("graph-mono-pnpm");
        let graph = PackageGraph::generate(sandbox.path()).unwrap();

        assert_snapshot!(graph.to_dot())
    }
}

mod monorepo_yarn {
    use super::*;

    #[test]
    fn loads_all_packages() {
        let sandbox = create_sandbox("graph-mono-yarn");
        let graph = PackageGraph::generate(sandbox.path()).unwrap();

        assert_snapshot!(graph.to_dot())
    }
}
