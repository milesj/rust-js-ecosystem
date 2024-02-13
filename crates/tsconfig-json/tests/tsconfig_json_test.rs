use std::path::PathBuf;
use typescript_tsconfig_json::{PathOrGlob, TsConfigJson};

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
