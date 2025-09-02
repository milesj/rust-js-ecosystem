use nodejs_package_json::VersionProtocol;
use semver::{Version, VersionReq};
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn empty() {
    let exp = VersionProtocol::Requirement(VersionReq::parse("*").unwrap());

    assert_eq!(VersionProtocol::from_str("").unwrap(), exp);
    assert_eq!(exp.to_string(), "*")
}

#[test]
fn star() {
    let exp = VersionProtocol::Requirement(VersionReq::parse("*").unwrap());

    assert_eq!(VersionProtocol::from_str("*").unwrap(), exp);
    assert_eq!(exp.to_string(), "*")
}

#[test]
fn file() {
    let exp = VersionProtocol::File(PathBuf::from("../dir"));

    assert_eq!(VersionProtocol::from_str("file:../dir").unwrap(), exp);
    assert_eq!(exp.to_string(), "file:../dir")
}

#[test]
fn git() {
    let exp = VersionProtocol::Git {
        reference: None,
        url: "git://github.com/npm/cli.git".into(),
    };

    assert_eq!(
        VersionProtocol::from_str("git://github.com/npm/cli.git").unwrap(),
        exp
    );
    assert_eq!(exp.to_string(), "git://github.com/npm/cli.git");

    let exp = VersionProtocol::Git {
        reference: None,
        url: "git+https://isaacs@github.com/npm/cli.git".into(),
    };

    assert_eq!(
        VersionProtocol::from_str("git+https://isaacs@github.com/npm/cli.git").unwrap(),
        exp
    );
    assert_eq!(exp.to_string(), "git+https://isaacs@github.com/npm/cli.git");
}

#[test]
fn git_with_ref() {
    let exp = VersionProtocol::Git {
        reference: Some("semver:^5.0".into()),
        url: "git+ssh://git@github.com:npm/cli".into(),
    };

    assert_eq!(
        VersionProtocol::from_str("git+ssh://git@github.com:npm/cli#semver:^5.0").unwrap(),
        exp
    );
    assert_eq!(
        exp.to_string(),
        "git+ssh://git@github.com:npm/cli#semver:^5.0"
    );

    let exp = VersionProtocol::Git {
        reference: Some("v1.0.27".into()),
        url: "git+http://git@github.com:npm/cli.git".into(),
    };

    assert_eq!(
        VersionProtocol::from_str("git+http://git@github.com:npm/cli.git#v1.0.27").unwrap(),
        exp
    );
    assert_eq!(
        exp.to_string(),
        "git+http://git@github.com:npm/cli.git#v1.0.27"
    );
}

#[test]
fn github() {
    let exp = VersionProtocol::GitHub {
        reference: None,
        owner: "user".into(),
        repo: "repo-name".into(),
    };

    assert_eq!(VersionProtocol::from_str("user/repo-name").unwrap(), exp);
    assert_eq!(exp.to_string(), "user/repo-name")
}

#[test]
fn github_with_ref() {
    let exp = VersionProtocol::GitHub {
        reference: Some("feature/branch".into()),
        owner: "org-name".into(),
        repo: "repo".into(),
    };

    assert_eq!(
        VersionProtocol::from_str("org-name/repo#feature/branch").unwrap(),
        exp
    );
    assert_eq!(exp.to_string(), "org-name/repo#feature/branch")
}

#[test]
fn http_url() {
    let exp = VersionProtocol::Url("http://domain.com/dep.tgz".into());

    assert_eq!(
        VersionProtocol::from_str("http://domain.com/dep.tgz").unwrap(),
        exp
    );
    assert_eq!(exp.to_string(), "http://domain.com/dep.tgz")
}

#[test]
fn https_url() {
    let exp = VersionProtocol::Url("https://domain.com/dep.tgz".into());

    assert_eq!(
        VersionProtocol::from_str("https://domain.com/dep.tgz").unwrap(),
        exp
    );
    assert_eq!(exp.to_string(), "https://domain.com/dep.tgz")
}

#[test]
fn link() {
    let exp = VersionProtocol::Link(PathBuf::from("../dir"));

    assert_eq!(VersionProtocol::from_str("link:../dir").unwrap(), exp);
    assert_eq!(exp.to_string(), "link:../dir")
}

#[test]
fn portal() {
    let exp = VersionProtocol::Portal(PathBuf::from("../dir"));

    assert_eq!(VersionProtocol::from_str("portal:../dir").unwrap(), exp);
    assert_eq!(exp.to_string(), "portal:../dir")
}

#[test]
fn version() {
    let exp = VersionProtocol::Version(Version::parse("1.2.3").unwrap());

    assert_eq!(VersionProtocol::from_str("1.2.3").unwrap(), exp);
    assert_eq!(exp.to_string(), "1.2.3")
}

#[test]
fn version_meta() {
    let exp = VersionProtocol::Version(Version::parse("7.0.0-dev.20250828.1").unwrap());

    assert_eq!(
        VersionProtocol::from_str("7.0.0-dev.20250828.1").unwrap(),
        exp
    );
    assert_eq!(exp.to_string(), "7.0.0-dev.20250828.1")
}

#[test]
fn version_build() {
    let exp = VersionProtocol::Version(Version::parse("2.0.4+20210327").unwrap());

    assert_eq!(VersionProtocol::from_str("2.0.4+20210327").unwrap(), exp);
    assert_eq!(exp.to_string(), "2.0.4+20210327")
}

#[test]
fn range() {
    let exp = VersionProtocol::Requirement(VersionReq::parse(">=1.2.3, <=4.5.6").unwrap());

    assert_eq!(VersionProtocol::from_str("1.2.3 - 4.5.6").unwrap(), exp);
    assert_eq!(exp.to_string(), ">=1.2.3, <=4.5.6")
}

#[test]
fn catalog() {
    let exp = VersionProtocol::Catalog(Some("name".into()));

    assert_eq!(VersionProtocol::from_str("catalog:name").unwrap(), exp);
    assert_eq!(exp.to_string(), "catalog:name")
}

#[test]
fn catalog_no_name() {
    let exp = VersionProtocol::Catalog(None);

    assert_eq!(VersionProtocol::from_str("catalog:").unwrap(), exp);
    assert_eq!(exp.to_string(), "catalog:");
}

#[test]
fn tag() {
    let exp = VersionProtocol::Tag("latest".into());

    assert_eq!(VersionProtocol::from_str("latest").unwrap(), exp);
    assert_eq!(exp.to_string(), "latest");

    let exp = VersionProtocol::Tag("next".into());

    assert_eq!(VersionProtocol::from_str("next").unwrap(), exp);
    assert_eq!(exp.to_string(), "next");
}

#[test]
fn tag_version() {
    let exp = VersionProtocol::Tag("v1.2.3".into());

    assert_eq!(VersionProtocol::from_str("v1.2.3").unwrap(), exp);
    assert_eq!(exp.to_string(), "v1.2.3");
}

#[test]
fn alias_jsr() {
    let exp = VersionProtocol::Alias("jsr:@std/assert@^1.0.13".into());

    assert_eq!(
        VersionProtocol::from_str("jsr:@std/assert@^1.0.13").unwrap(),
        exp
    );
    assert_eq!(exp.to_string(), "jsr:@std/assert@^1.0.13");
}

#[test]
fn alias_npm() {
    let exp = VersionProtocol::Alias("npm:pkg@1.0.0".into());

    assert_eq!(VersionProtocol::from_str("npm:pkg@1.0.0").unwrap(), exp);
    assert_eq!(exp.to_string(), "npm:pkg@1.0.0");
}
