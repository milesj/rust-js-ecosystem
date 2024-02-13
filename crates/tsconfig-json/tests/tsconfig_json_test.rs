use starbase_sandbox::create_empty_sandbox;
use std::path::PathBuf;
use typescript_tsconfig_json::{ExtendsField, PathOrGlob, ResolvedTsConfigChain, TsConfigJson};

#[test]
fn handles_path_types() {
    let tsconfig: TsConfigJson = serde_json::from_str(
        r#"{
  "include": ["file.ts", "dir/**/*"],
  "exclude": ["file.tsx?", "dir"],
  "files": ["file.tsx"]
}"#,
    )
    .unwrap();

    assert_eq!(
        tsconfig,
        TsConfigJson {
            include: Some(vec![
                PathOrGlob::Path("file.ts".into()),
                PathOrGlob::Glob("dir/**/*".into()),
            ]),
            exclude: Some(vec![
                PathOrGlob::Glob("file.tsx?".into()),
                PathOrGlob::Path("dir".into()),
            ]),
            files: Some(vec![PathBuf::from("file.tsx")]),
            ..Default::default()
        }
    );
}

mod node_modules_config {
    use super::*;

    #[test]
    fn returns_none_if_not_exists() {
        let sandbox = create_empty_sandbox();

        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules(
                "@scope/package/tsconfig.json",
                sandbox.path()
            ),
            None
        );
    }

    #[test]
    fn resolves_with_package_name() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("node_modules/@scope/package/tsconfig.json", "{}");

        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules("@scope/package", sandbox.path()),
            Some(
                sandbox
                    .path()
                    .join("node_modules/@scope/package/tsconfig.json")
            )
        );
    }

    #[test]
    fn resolves_with_package_name_and_custom_file() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("node_modules/@scope/package/tsconfig.alt.json", "{}");

        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules(
                "@scope/package/tsconfig.json",
                sandbox.path()
            ),
            None
        );
        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules(
                "@scope/package/tsconfig.alt.json",
                sandbox.path()
            ),
            Some(
                sandbox
                    .path()
                    .join("node_modules/@scope/package/tsconfig.alt.json")
            )
        );
    }

    #[test]
    fn resolves_nested_file_in_package() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("node_modules/@scope/package/configs/tsconfig.json", "{}");

        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules(
                "@scope/package/configs/tsconfig.json",
                sandbox.path()
            ),
            Some(
                sandbox
                    .path()
                    .join("node_modules/@scope/package/configs/tsconfig.json")
            )
        );
        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules("@scope/package/configs", sandbox.path()),
            Some(
                sandbox
                    .path()
                    .join("node_modules/@scope/package/configs/tsconfig.json")
            )
        );
    }

    #[test]
    fn resolves_traversing_upwards() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("node_modules/@scope/package/tsconfig.json", "{}");

        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules(
                "@scope/package",
                sandbox.path().join("nested/project/with/node_modules")
            ),
            Some(
                sandbox
                    .path()
                    .join("node_modules/@scope/package/tsconfig.json")
            )
        );
    }
}

mod extends_chain {
    use super::*;

    #[test]
    fn resolves_self() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("tsconfig.json", "{}");

        let chain =
            TsConfigJson::resolve_extends_chain(sandbox.path().join("tsconfig.json")).unwrap();

        assert_eq!(
            chain,
            vec![ResolvedTsConfigChain {
                path: sandbox.path().join("tsconfig.json"),
                config: TsConfigJson::default()
            }]
        );
    }

    #[test]
    fn resolves_single() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "tsconfig.json",
            r#"{ "extends": "./tsconfig.1.json", "include": ["file.tsx"] }"#,
        );
        sandbox.create_file("tsconfig.1.json", r#"{ "include": ["dir/**/*"] }"#);

        let chain =
            TsConfigJson::resolve_extends_chain(sandbox.path().join("tsconfig.json")).unwrap();

        assert_eq!(
            chain,
            vec![
                ResolvedTsConfigChain {
                    path: sandbox.path().join("tsconfig.1.json"),
                    config: TsConfigJson {
                        include: Some(vec![PathOrGlob::Glob("dir/**/*".into())]),
                        ..TsConfigJson::default()
                    }
                },
                ResolvedTsConfigChain {
                    path: sandbox.path().join("tsconfig.json"),
                    config: TsConfigJson {
                        extends: Some(ExtendsField::Single("./tsconfig.1.json".into())),
                        include: Some(vec![PathOrGlob::Path("file.tsx".into())]),
                        ..TsConfigJson::default()
                    }
                }
            ]
        );
    }

    #[test]
    fn resolves_multiple() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "tsconfig.json",
            r#"{ "extends": ["./tsconfig.1.json", "./tsconfig.2.json"], "include": ["file.tsx"] }"#,
        );
        sandbox.create_file("tsconfig.1.json", r#"{ "include": ["dir/**/*"] }"#);
        sandbox.create_file("tsconfig.2.json", r#"{ "exclude": ["build/**/*"] }"#);

        let chain =
            TsConfigJson::resolve_extends_chain(sandbox.path().join("tsconfig.json")).unwrap();

        assert_eq!(
            chain,
            vec![
                ResolvedTsConfigChain {
                    path: sandbox.path().join("tsconfig.1.json"),
                    config: TsConfigJson {
                        include: Some(vec![PathOrGlob::Glob("dir/**/*".into())]),
                        ..TsConfigJson::default()
                    }
                },
                ResolvedTsConfigChain {
                    path: sandbox.path().join("tsconfig.2.json"),
                    config: TsConfigJson {
                        exclude: Some(vec![PathOrGlob::Glob("build/**/*".into())]),
                        ..TsConfigJson::default()
                    }
                },
                ResolvedTsConfigChain {
                    path: sandbox.path().join("tsconfig.json"),
                    config: TsConfigJson {
                        extends: Some(ExtendsField::Multiple(vec![
                            "./tsconfig.1.json".into(),
                            "./tsconfig.2.json".into()
                        ])),
                        include: Some(vec![PathOrGlob::Path("file.tsx".into())]),
                        ..TsConfigJson::default()
                    }
                },
            ]
        );
    }
}
