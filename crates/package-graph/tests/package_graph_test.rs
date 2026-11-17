use nodejs_package_graph::*;
use nodejs_package_managers::PackageManager;
use starbase_sandbox::{assert_snapshot, create_empty_sandbox, create_sandbox};

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
        fn links_version_range_if_match() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ver-range").unwrap(),
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
        fn links_path() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("path").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_workspace_version_with_alias() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependencies_of("ws-version-alias").unwrap(),
                vec![("a".into(), DependencyType::Production)]
            );
        }

        #[test]
        fn links_workspace_caret_and_tilde_as_ranges() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            // c is 1.2.0, so ~1.0.0 doesn't match
            assert_eq!(
                graph.dependencies_of("ws-range").unwrap(),
                vec![
                    ("c".into(), DependencyType::Peer),
                    ("c".into(), DependencyType::Production),
                ]
            );
        }

        #[test]
        fn returns_dependents() {
            let sandbox = create_sandbox("graph-protocols");
            let graph = PackageGraph::generate(sandbox.path()).unwrap();

            assert_eq!(
                graph.dependents_of("c").unwrap(),
                vec![
                    ("ws-range".into(), DependencyType::Peer),
                    ("ws-range".into(), DependencyType::Production),
                ]
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

mod package_root {
    use super::*;

    #[test]
    fn detects_bun_text_lockfile() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("bun.lock", "{}");

        let (root, manager) = PackageGraph::find_package_root(sandbox.path()).unwrap();

        assert_eq!(root, sandbox.path());
        assert!(manager == PackageManager::Bun);
    }

    #[test]
    fn detects_bun_binary_lockfile() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("bun.lockb", "");

        let (_, manager) = PackageGraph::find_package_root(sandbox.path()).unwrap();

        assert!(manager == PackageManager::Bun);
    }
}

mod large_workspace {
    use super::*;

    #[test]
    fn supports_more_than_255_packages() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "package.json",
            r#"{"name": "root", "workspaces": ["packages/*"]}"#,
        );

        for i in 0..300 {
            sandbox.create_file(
                format!("packages/p{i}/package.json"),
                format!(r#"{{"name": "p{i}"}}"#),
            );
        }

        let graph = PackageGraph::generate(sandbox.path()).unwrap();

        assert_eq!(graph.packages.len(), 300);
        assert_eq!(
            graph.packages.values().map(|pkg| pkg.index).max(),
            Some(300)
        );
    }
}
