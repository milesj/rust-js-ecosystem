#![allow(clippy::from_over_into)]

use super::workspace_protocol::WorkspaceProtocol;
use once_cell::sync::Lazy;
use regex::Regex;
use semver::{Error, Version, VersionReq};
use serde::Deserialize;
use std::fmt::{self, Display};
use std::path::PathBuf;
use std::str::FromStr;

static GITHUB: Lazy<Regex> = Lazy::new(|| {
    Regex::new("(?<owner>[A-Za-z0-9_.-]+)/(?<repo>[A-Za-z0-9_.-]+)(?:#(?<commit>[a-z0-9]+))")
        .unwrap()
});

// https://docs.npmjs.com/cli/v10/configuring-npm/package-json#dependencies
#[derive(Debug, Deserialize)]
#[serde(untagged, into = "String", try_from = "String")]
pub enum VersionProtocol {
    File(PathBuf),
    Git {
        commit: Option<String>,
        url: String,
    },
    GitHub {
        commit: Option<String>,
        owner: String,
        repo: String,
    },
    Range(Vec<VersionReq>),
    Requirement(VersionReq),
    Url(String),
    Version(Version),
    Workspace(WorkspaceProtocol),
}

impl FromStr for VersionProtocol {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() || value == "*" {
            return Ok(VersionProtocol::Requirement(VersionReq::parse("*")?));
        }

        if let Some(index) = value.find(':') {
            match &value[0..index] {
                "http" | "https" => {
                    return Ok(VersionProtocol::Url(value.to_owned()));
                }
                "git" | "git+ssh" | "git+http" | "git+https" | "git+file" => {
                    let mut parts = value.split('#');

                    return Ok(VersionProtocol::Git {
                        url: parts.next().expect("Missing Git URL!").to_owned(),
                        commit: parts.next().map(|p| p.to_owned()),
                    });
                }
                "file" => {
                    return Ok(VersionProtocol::File(PathBuf::from(&value[index + 1..])));
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
                owner: caps
                    .name("owner")
                    .expect("Missing GitHub user or organization!")
                    .as_str()
                    .to_owned(),
                repo: caps
                    .name("repo")
                    .expect("Missing GitHub repository name!")
                    .as_str()
                    .to_owned(),
                commit: caps.name("commit").map(|c| c.as_str().to_owned()),
            });
        }

        if value.contains('-') {
            let mut parts = value.split('-');
            let l = parts.next().unwrap();
            let r = parts.next().unwrap();

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
    type Error = Error;

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
                VersionProtocol::Git { commit, url } => commit
                    .as_ref()
                    .map(|c| format!("{url}#{c}"))
                    .unwrap_or_else(|| url.to_owned()),
                VersionProtocol::GitHub {
                    commit,
                    owner,
                    repo,
                } => {
                    let github = format!("{owner}/{repo}");

                    commit
                        .as_ref()
                        .map(|c| format!("{github}#{c}"))
                        .unwrap_or_else(|| github)
                }
                VersionProtocol::Range(range) => range
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>()
                    .join(" || "),
                VersionProtocol::Requirement(req) => req.to_string(),
                VersionProtocol::Url(url) => url.to_owned(),
                VersionProtocol::Version(ver) => ver.to_string(),
                VersionProtocol::Workspace(ws) => ws.to_string(),
            }
        )
    }
}
