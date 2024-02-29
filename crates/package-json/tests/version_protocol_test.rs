use nodejs_package_json::VersionProtocol;
use semver::VersionReq;
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
fn range() {
    let exp = VersionProtocol::Requirement(VersionReq::parse(">=1.2.3, <=4.5.6").unwrap());

    assert_eq!(VersionProtocol::from_str("1.2.3 - 4.5.6").unwrap(), exp);
    assert_eq!(exp.to_string(), ">=1.2.3, <=4.5.6")
}
