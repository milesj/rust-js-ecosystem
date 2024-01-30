use node_package_json::WorkspaceProtocol;
use semver::Version;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn star() {
    let exp = WorkspaceProtocol::Any { alias: None };

    assert_eq!(WorkspaceProtocol::from_str("*").unwrap(), exp);
    assert_eq!(exp.to_string(), "*")
}

#[test]
fn star_with_alias() {
    let exp = WorkspaceProtocol::Any {
        alias: Some("foo".into()),
    };

    assert_eq!(WorkspaceProtocol::from_str("foo@*").unwrap(), exp);
    assert_eq!(exp.to_string(), "foo@*")
}

#[test]
#[should_panic(expected = "StarNoVersion")]
fn errors_star_more_info() {
    WorkspaceProtocol::from_str("*1.2.3").unwrap();
}

#[test]
fn tilde() {
    let exp = WorkspaceProtocol::Tilde {
        alias: None,
        version: None,
    };

    assert_eq!(WorkspaceProtocol::from_str("~").unwrap(), exp);
    assert_eq!(exp.to_string(), "~")
}

#[test]
fn tilde_with_alias() {
    let exp = WorkspaceProtocol::Tilde {
        alias: Some("foo".into()),
        version: None,
    };

    assert_eq!(WorkspaceProtocol::from_str("foo@~").unwrap(), exp);
    assert_eq!(exp.to_string(), "foo@~")
}

#[test]
fn tilde_with_version() {
    let exp = WorkspaceProtocol::Tilde {
        alias: None,
        version: Some(Version::new(1, 2, 3)),
    };

    assert_eq!(WorkspaceProtocol::from_str("~1.2.3").unwrap(), exp);
    assert_eq!(exp.to_string(), "~1.2.3")
}

#[test]
fn tilde_with_alias_and_version() {
    let exp = WorkspaceProtocol::Tilde {
        alias: Some("foo".into()),
        version: Some(Version::new(1, 2, 3)),
    };

    assert_eq!(WorkspaceProtocol::from_str("foo@~1.2.3").unwrap(), exp);
    assert_eq!(exp.to_string(), "foo@~1.2.3")
}

#[test]
#[should_panic(expected = "unexpected end of input while parsing minor version number")]
fn errors_tilde_invalid_version() {
    WorkspaceProtocol::from_str("~1.2").unwrap();
}

#[test]
fn caret() {
    let exp = WorkspaceProtocol::Caret {
        alias: None,
        version: None,
    };

    assert_eq!(WorkspaceProtocol::from_str("^").unwrap(), exp);
    assert_eq!(exp.to_string(), "^")
}

#[test]
fn caret_with_alias() {
    let exp = WorkspaceProtocol::Caret {
        alias: Some("foo".into()),
        version: None,
    };

    assert_eq!(WorkspaceProtocol::from_str("foo@^").unwrap(), exp);
    assert_eq!(exp.to_string(), "foo@^")
}

#[test]
fn caret_with_version() {
    let exp = WorkspaceProtocol::Caret {
        alias: None,
        version: Some(Version::new(1, 2, 3)),
    };

    assert_eq!(WorkspaceProtocol::from_str("^1.2.3").unwrap(), exp);
    assert_eq!(exp.to_string(), "^1.2.3")
}

#[test]
fn caret_with_alias_and_version() {
    assert_eq!(
        WorkspaceProtocol::from_str("foo@^1.2.3").unwrap(),
        WorkspaceProtocol::Caret {
            alias: Some("foo".into()),
            version: Some(Version::new(1, 2, 3)),
        }
    );
    assert_eq!(
        WorkspaceProtocol::Caret {
            alias: Some("foo".into()),
            version: Some(Version::new(1, 2, 3)),
        }
        .to_string(),
        "foo@^1.2.3"
    )
}

#[test]
#[should_panic(expected = "unexpected end of input while parsing minor version number")]
fn errors_caret_invalid_version() {
    WorkspaceProtocol::from_str("^1.2").unwrap();
}

#[test]
fn file_relative() {
    assert_eq!(
        WorkspaceProtocol::from_str("../dir").unwrap(),
        WorkspaceProtocol::File(PathBuf::from("../dir"))
    );
    assert_eq!(
        WorkspaceProtocol::File(PathBuf::from("../dir")).to_string(),
        "../dir"
    )
}

#[test]
fn file_absolute() {
    assert_eq!(
        WorkspaceProtocol::from_str("/dir").unwrap(),
        WorkspaceProtocol::File(PathBuf::from("/dir"))
    );
    assert_eq!(
        WorkspaceProtocol::File(PathBuf::from("/dir")).to_string(),
        "/dir"
    )
}

#[test]
fn version() {
    assert_eq!(
        WorkspaceProtocol::from_str("1.2.3").unwrap(),
        WorkspaceProtocol::Version(Version::new(1, 2, 3))
    );
    assert_eq!(
        WorkspaceProtocol::Version(Version::new(1, 2, 3)).to_string(),
        "1.2.3"
    )
}

#[test]
#[should_panic(expected = "unexpected end of input while parsing minor version number")]
fn errors_invalid_version() {
    WorkspaceProtocol::from_str("1.2").unwrap();
}
