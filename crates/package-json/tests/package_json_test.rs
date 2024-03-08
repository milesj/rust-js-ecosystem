use indexmap::IndexMap;
use nodejs_package_json::{
    DependencyMetaField, ImportExportField, ImportExportMap, PackageJson, VersionProtocol,
    WorkspaceProtocol,
};
use semver::{Version, VersionReq};
use std::collections::BTreeMap;

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
                VersionProtocol::Requirement(VersionReq::parse("^1.20").unwrap()),
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
