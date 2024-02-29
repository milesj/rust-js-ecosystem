use nodejs_package_graph::*;
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

mod monorepo {
    use super::*;

    mod local_relationships {
        use super::*;

        #[test]
        fn links_version_star() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ver-star").unwrap(),
                vec![
                    ("b".into(), DependencyType::Development),
                    ("a".into(), DependencyType::Production),
                ]
            );
        }

        #[test]
        fn links_version_if_match() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ver").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_version_req_if_match() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ver-req").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_file() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("file").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_link() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("link").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_portal() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("portal").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_workspace_star() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ws-star").unwrap(),
                vec![
                    ("b".into(), DependencyType::Development),
                    ("a".into(), DependencyType::Production),
                ]
            );
        }

        #[test]
        fn links_workspace_star_with_alias() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ws-star-alias").unwrap(),
                vec![
                    ("b".into(), DependencyType::Development),
                    ("a".into(), DependencyType::Production),
                ]
            );
        }

        #[test]
        fn links_workspace_caret() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ws-caret").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_workspace_caret_with_alias() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ws-caret-alias").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_workspace_tilde() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ws-tilde").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_workspace_tilde_with_alias() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ws-tilde-alias").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_workspace_file() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ws-file").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_workspace_version() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ws-version").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn supports_peer() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("peer-deps").unwrap(),
                vec![("a".into(), DependencyType::Peer)]
            );
        }

        #[test]
        fn supports_optional() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("optional-deps").unwrap(),
                vec![("a".into(), DependencyType::Optional)]
            );
        }
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
