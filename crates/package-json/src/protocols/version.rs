use super::workspace::*;
use once_cell::sync::Lazy;
use regex::Regex;
use semver::{Version, VersionReq};
use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;

static PROTOCOL: Lazy<Regex> = Lazy::new(|| Regex::new("^(?<protocol>[a-z+]+):").unwrap());

static GITHUB: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        "(?<owner>[A-Za-z0-9_.-]+)/(?<repo>[A-Za-z0-9_.-]+)(?:#(?<commit>[A-Za-z0-9_.-/]+))?",
    )
    .unwrap()
});

#[derive(Debug, Error)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum VersionProtocolError {
    #[error("Missing start version for range.")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(code(package_json::version::missing_range_start))
    )]
    RangeMissingStartVersion,

    #[error("Missing stop version for range.")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(code(package_json::version::missing_range_stop))
    )]
    RangeMissingStopVersion,

    #[error("Failed to parse version or requirement: {0}")]
    #[cfg_attr(feature = "miette", diagnostic(code(package_json::version::invalid)))]
    Semver(#[from] semver::Error),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Workspace(#[from] WorkspaceProtocolError),
}

// https://docs.npmjs.com/cli/v10/configuring-npm/package-json#dependencies
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged, try_from = "String", into = "String")]
pub enum VersionProtocol {
    File(PathBuf),
    Git {
        reference: Option<String>,
        url: String,
    },
    GitHub {
        reference: Option<String>,
        owner: String,
        repo: String,
    },
    Link(PathBuf),
    Portal(PathBuf),
    Range(Vec<VersionReq>),
    Requirement(VersionReq),
    Url(String),
    Version(Version),
    Workspace(WorkspaceProtocol),
}

impl FromStr for VersionProtocol {
    type Err = VersionProtocolError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() || value == "*" {
            return Ok(VersionProtocol::Requirement(VersionReq::parse("*")?));
        }

        if let Some(caps) = PROTOCOL.captures(value) {
            let protocol = caps.name("protocol").unwrap().as_str();
            let index = protocol.len();

            match protocol {
                "http" | "https" => {
                    return Ok(VersionProtocol::Url(value.to_owned()));
                }
                "git" | "git+ssh" | "git+http" | "git+https" | "git+file" => {
                    let mut parts = value.split('#');

                    return Ok(VersionProtocol::Git {
                        url: parts.next().unwrap().to_owned(),
                        reference: parts.next().map(|p| p.to_owned()),
                    });
                }
                "file" => {
                    return Ok(VersionProtocol::File(PathBuf::from(&value[index + 1..])));
                }
                "link" => {
                    return Ok(VersionProtocol::Link(PathBuf::from(&value[index + 1..])));
                }
                "portal" => {
                    return Ok(VersionProtocol::Portal(PathBuf::from(&value[index + 1..])));
                }
                "workspace" => {
                    return Ok(VersionProtocol::Workspace(WorkspaceProtocol::from_str(
                        &value[index + 1..],
                    )?));
                }
                _ => {}
            }
        }

        if let Some(caps) = GITHUB.captures(value) {
            return Ok(VersionProtocol::GitHub {
                owner: caps.name("owner").unwrap().as_str().to_owned(),
                repo: caps.name("repo").unwrap().as_str().to_owned(),
                reference: caps.name("commit").map(|c| c.as_str().to_owned()),
            });
        }

        if value.contains('-') {
            let mut parts = value.split('-');
            let l = parts
                .next()
                .ok_or(VersionProtocolError::RangeMissingStartVersion)?
                .trim();
            let r = parts
                .next()
                .ok_or(VersionProtocolError::RangeMissingStopVersion)?
                .trim();

            return Ok(VersionProtocol::Requirement(VersionReq::parse(&format!(
                ">={l}, <={r}"
            ))?));
        }

        if value.contains("||") {
            let mut ranges = vec![];

            for range in value.split("||") {
                ranges.push(VersionReq::parse(range.trim())?);
            }

            return Ok(VersionProtocol::Range(ranges));
        }

        if value.contains('^')
            || value.contains('~')
            || value.contains('>')
            || value.contains('<')
            || value.contains('=')
        {
            return Ok(VersionProtocol::Requirement(VersionReq::parse(value)?));
        }

        Ok(VersionProtocol::Version(Version::parse(value)?))
    }
}

impl TryFrom<String> for VersionProtocol {
    type Error = VersionProtocolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<VersionProtocol> for String {
    fn from(value: VersionProtocol) -> String {
        value.to_string()
    }
}

impl fmt::Display for VersionProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VersionProtocol::File(path) => format!("file:{}", path.display()),
                VersionProtocol::Git { reference, url } => reference
                    .as_ref()
                    .map(|c| format!("{url}#{c}"))
                    .unwrap_or_else(|| url.to_owned()),
                VersionProtocol::GitHub {
                    reference,
                    owner,
                    repo,
                } => {
                    let github = format!("{owner}/{repo}");

                    reference
                        .as_ref()
                        .map(|c| format!("{github}#{c}"))
                        .unwrap_or_else(|| github)
                }
                VersionProtocol::Link(path) => format!("link:{}", path.display()),
                VersionProtocol::Portal(path) => format!("portal:{}", path.display()),
                VersionProtocol::Range(range) => range
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>()
                    .join(" || "),
                VersionProtocol::Requirement(req) => req.to_string(),
                VersionProtocol::Url(url) => url.to_owned(),
                VersionProtocol::Version(ver) => ver.to_string(),
                VersionProtocol::Workspace(ws) => format!("workspace:{ws}"),
            }
        )
    }
}
