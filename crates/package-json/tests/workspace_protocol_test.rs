use nodejs_package_json::{Requirement, Version, WorkspaceProtocol};
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn none() {
    let exp = WorkspaceProtocol::Any { alias: None };

    assert_eq!(WorkspaceProtocol::from_str("").unwrap(), exp);
    assert_eq!(exp.to_string(), "*")
}

#[test]
fn none_with_alias() {
    let exp = WorkspaceProtocol::Any {
        alias: Some("@scope/foo".into()),
    };

    assert_eq!(WorkspaceProtocol::from_str("@scope/foo@").unwrap(), exp);
    assert_eq!(exp.to_string(), "@scope/foo@*")
}

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
fn tilde_partial_version() {
    assert_eq!(
        WorkspaceProtocol::from_str("~1.2").unwrap(),
        WorkspaceProtocol::Requirement {
            alias: None,
            req: Requirement::parse("~1.2").unwrap(),
        }
    );
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
fn caret_partial_version() {
    assert_eq!(
        WorkspaceProtocol::from_str("^1.2").unwrap(),
        WorkspaceProtocol::Requirement {
            alias: None,
            req: Requirement::parse("^1.2").unwrap(),
        }
    );
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
fn partial_version() {
    assert_eq!(
        WorkspaceProtocol::from_str("1.2").unwrap(),
        WorkspaceProtocol::Requirement {
            alias: None,
            req: Requirement::parse("1.2").unwrap(),
        }
    );
}

#[test]
fn requirements() {
    for (value, exp_req) in [
        ("^1", "^1"),
        ("~1.2", "~1.2"),
        (">=1.0.0", ">=1.0.0"),
        ("1", "~1"),
    ] {
        assert_eq!(
            WorkspaceProtocol::from_str(value).unwrap(),
            WorkspaceProtocol::Requirement {
                alias: None,
                req: Requirement::parse(exp_req).unwrap(),
            },
            "{value}"
        );
    }
}

#[test]
fn requirement_with_alias() {
    let exp = WorkspaceProtocol::Requirement {
        alias: Some("foo".into()),
        req: Requirement::parse("^1").unwrap(),
    };

    assert_eq!(WorkspaceProtocol::from_str("foo@^1").unwrap(), exp);
    assert_eq!(exp.to_string(), "foo@^1")
}

#[test]
#[should_panic(expected = "Spec")]
fn errors_invalid_requirement() {
    WorkspaceProtocol::from_str("^not-a-range").unwrap();
}
