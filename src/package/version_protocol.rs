#![allow(clippy::from_over_into)]

use super::workspace_protocol::WorkspaceProtocol;
use miette::IntoDiagnostic;
use once_cell::sync::Lazy;
use regex::Regex;
use semver::{Version, VersionReq};
use serde::Deserialize;
use std::fmt::{self, Display};
use std::path::PathBuf;
use std::str::FromStr;

static PROTOCOL: Lazy<Regex> = Lazy::new(|| Regex::new("^(?<protocol>[a-z+]+):").unwrap());

static GITHUB: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        "(?<owner>[A-Za-z0-9_.-]+)/(?<repo>[A-Za-z0-9_.-]+)(?:#(?<commit>[A-Za-z0-9_.-/]+))?",
    )
    .unwrap()
});

// https://docs.npmjs.com/cli/v10/configuring-npm/package-json#dependencies
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged, into = "String", try_from = "String")]
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
    type Err = miette::Report;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() || value == "*" {
            return Ok(VersionProtocol::Requirement(
                VersionReq::parse("*").into_diagnostic()?,
            ));
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
                        url: parts
                            .next()
                            .ok_or_else(|| miette::miette!("Missing Git URL."))?
                            .to_owned(),
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
                .ok_or_else(|| miette::miette!("Missing start range."))?;
            let r = parts
                .next()
                .ok_or_else(|| miette::miette!("Missing stop range."))?;

            return Ok(VersionProtocol::Requirement(
                VersionReq::parse(&format!(">={l}, <={r}")).into_diagnostic()?,
            ));
        }

        if value.contains("||") {
            let mut ranges = vec![];

            for range in value.split("||") {
                ranges.push(VersionReq::parse(range.trim()).into_diagnostic()?);
            }

            return Ok(VersionProtocol::Range(ranges));
        }

        if value.contains('^')
            || value.contains('~')
            || value.contains('>')
            || value.contains('<')
            || value.contains('=')
        {
            return Ok(VersionProtocol::Requirement(
                VersionReq::parse(value).into_diagnostic()?,
            ));
        }

        Ok(VersionProtocol::Version(
            Version::parse(value).into_diagnostic()?,
        ))
    }
}

impl TryFrom<String> for VersionProtocol {
    type Error = miette::Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl Into<String> for VersionProtocol {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Display for VersionProtocol {
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
