use indexmap::IndexMap;
use nodejs_package_json::{
    DependencyMetaField, ImportExportField, ImportExportMap, PackageJson, Requirement, Version,
    VersionProtocol, WorkspaceProtocol,
};
use starbase_sandbox::locate_fixture;
use std::collections::BTreeMap;
use std::fs;

#[test]
fn serializes_packages_json() {
    let pkg = PackageJson {
        name: Some("vite".into()),
        version: Some(Version::new(1, 2, 3)),
        scripts: Some(IndexMap::from_iter([("build".into(), "vite build".into())])),
        exports: Some(ImportExportField::Map(ImportExportMap::from_iter([
            (
                ".".into(),
                ImportExportField::String("./mjs/index.mjs".into()),
            ),
            (
                "./bin".into(),
                ImportExportField::Map(ImportExportMap::from_iter([
                    (
                        "import".into(),
                        ImportExportField::String("./mjs/bin.mjs".into()),
                    ),
                    (
                        "require".into(),
                        ImportExportField::String("./cjs/bin.cjs".into()),
                    ),
                ])),
            ),
        ]))),
        dependencies: Some(BTreeMap::from_iter([
            (
                "@moonrepo/cli".into(),
                VersionProtocol::Requirement(Requirement::parse("^1.20").unwrap()),
            ),
            (
                "vite".into(),
                VersionProtocol::Workspace(WorkspaceProtocol::Any { alias: None }),
            ),
        ])),
        dependencies_meta: Some(BTreeMap::from_iter([(
            "vite".into(),
            DependencyMetaField {
                injected: Some(true),
                ..Default::default()
            },
        )])),
        package_manager: Some("npm@1.0.0".into()),
        ..Default::default()
    };

    assert_eq!(
        serde_json::to_string_pretty(&pkg).unwrap(),
        r#"{
  "name": "vite",
  "version": "1.2.3",
  "scripts": {
    "build": "vite build"
  },
  "exports": {
    ".": "./mjs/index.mjs",
    "./bin": {
      "import": "./mjs/bin.mjs",
      "require": "./cjs/bin.cjs"
    }
  },
  "dependencies": {
    "@moonrepo/cli": "^1.20",
    "vite": "workspace:*"
  },
  "dependenciesMeta": {
    "vite": {
      "injected": true
    }
  },
  "packageManager": "npm@1.0.0"
}"#
    );
}

#[test]
fn parses_babel() {
    let pkg: PackageJson = serde_json::from_str(
        &fs::read_to_string(locate_fixture("packages").join("babel.json")).unwrap(),
    )
    .unwrap();

    dbg!(pkg);
}

#[test]
fn parses_eslint() {
    let pkg: PackageJson = serde_json::from_str(
        &fs::read_to_string(locate_fixture("packages").join("eslint.json")).unwrap(),
    )
    .unwrap();

    dbg!(pkg);
}

#[test]
fn parses_typescript() {
    let pkg: PackageJson = serde_json::from_str(
        &fs::read_to_string(locate_fixture("packages").join("typescript.json")).unwrap(),
    )
    .unwrap();

    dbg!(pkg);
}

#[test]
fn parses_typescript_eslint() {
    let pkg: PackageJson = serde_json::from_str(
        &fs::read_to_string(locate_fixture("packages").join("typescript-eslint.json")).unwrap(),
    )
    .unwrap();

    dbg!(pkg);
}

#[test]
fn parses_webpack() {
    let pkg: PackageJson = serde_json::from_str(
        &fs::read_to_string(locate_fixture("packages").join("webpack.json")).unwrap(),
    )
    .unwrap();

    dbg!(pkg);
}

#[test]
fn parses_yarn() {
    let pkg: PackageJson = serde_json::from_str(
        &fs::read_to_string(locate_fixture("packages").join("yarn.json")).unwrap(),
    )
    .unwrap();

    dbg!(pkg);
}

#[test]
fn skips_fields_that_cant_be_parsed() {
    let pkg: PackageJson = serde_json::from_str(
        r#"{
    "name": "app",
    "version": "1.0",
    "bundleDependencies": true,
    "engines": ["node >= 0.10"],
    "dependencies": {
        "a": "^1.0.0",
        "b": "workspace:^not-a-range"
    }
}"#,
    )
    .unwrap();

    assert_eq!(pkg.name.as_deref(), Some("app"));
    assert_eq!(pkg.version, None);
    assert_eq!(pkg.bundle_dependencies, None);
    assert_eq!(pkg.engines, None);

    let deps = pkg.dependencies.unwrap();

    assert_eq!(
        deps.get("a").unwrap(),
        &VersionProtocol::Requirement(Requirement::parse("^1.0.0").unwrap())
    );
    assert_eq!(
        deps.get("b").unwrap(),
        &VersionProtocol::Unknown("workspace:^not-a-range".into())
    );
}

#[test]
fn keeps_dependencies_that_arent_strings() {
    let pkg: PackageJson = serde_json::from_str(
        r#"{
    "dependencies": {
        "a": "^1.0.0",
        "b": 1,
        "c": { "version": "1.0.0" }
    }
}"#,
    )
    .unwrap();

    let deps = pkg.dependencies.unwrap();

    assert_eq!(
        deps.get("a").unwrap(),
        &VersionProtocol::Requirement(Requirement::parse("^1.0.0").unwrap())
    );
    assert_eq!(
        deps.get("b").unwrap(),
        &VersionProtocol::Unknown("1".into())
    );
    assert_eq!(
        deps.get("c").unwrap(),
        &VersionProtocol::Unknown(r#"{"version":"1.0.0"}"#.into())
    );
}

#[test]
fn keeps_the_order_of_maps() {
    let pkg: PackageJson = serde_json::from_str(
        r#"{
    "scripts": { "z-build": "a", "a-test": "b" },
    "exports": {
        ".": { "node": "./n.js", "import": "./i.mjs", "default": "./d.js" }
    }
}"#,
    )
    .unwrap();

    assert_eq!(
        pkg.scripts.unwrap().keys().collect::<Vec<_>>(),
        ["z-build", "a-test"]
    );

    let Some(ImportExportField::Map(exports)) = pkg.exports else {
        unreachable!();
    };
    let Some(ImportExportField::Map(conditions)) = exports.values().next() else {
        unreachable!();
    };

    assert_eq!(
        conditions
            .keys()
            .map(|key| key.to_string())
            .collect::<Vec<_>>(),
        ["node", "import", "default"]
    );
}

#[test]
fn parses_workspaces_without_packages() {
    let pkg: PackageJson = serde_json::from_str(
        r#"{
    "workspaces": {
        "catalog": { "react": "^19.0.0" }
    }
}"#,
    )
    .unwrap();

    assert!(matches!(
        pkg.workspaces,
        Some(nodejs_package_json::WorkspacesField::Config { ref packages, ref catalog, .. })
            if packages.is_empty() && catalog.is_some()
    ));
}
