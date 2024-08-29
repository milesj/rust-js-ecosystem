use starbase_sandbox::locate_fixture;
use std::fs;
use std::path::PathBuf;
use typescript_tsbuildinfo::*;

mod v5_x {
    use super::*;

    #[test]
    fn deserializes() {
        let path = locate_fixture("files").join("5.x.json");
        let info: TsBuildInfo = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();

        assert_eq!(info.version, "5.4.5");
        assert_eq!(
            info.program.latest_changed_dts_file,
            Some(PathBuf::from("./cjs/index.d.ts"))
        );
    }
}
