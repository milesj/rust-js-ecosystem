use super::workspace::*;
use regex::Regex;
use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::LazyLock;
use thiserror::Error;
use version_spec::{Clause, Op, Range, Requirement, SpecError, Version};

static PROTOCOL: LazyLock<Regex> = LazyLock::new(|| Regex::new("^(?<protocol>[a-z+]+):").unwrap());

static GITHUB: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^(?<owner>[A-Za-z0-9_.-]+)/(?<repo>[A-Za-z0-9_.-]+)(?:#(?<commit>.+))?$").unwrap()
});

// SCP-like git URLs without a protocol, like `git@github.com:npm/cli.git`
static GIT_SCP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[A-Za-z0-9_.-]+@[A-Za-z0-9_.-]+:").unwrap());

// https://docs.npmjs.com/cli/v10/configuring-npm/package-json#local-paths
fn is_local_path(value: &str) -> bool {
    let bytes = value.as_bytes();

    value.starts_with(['.', '/', '\\'])
        || value.starts_with("~/")
        // Windows drives, like `C:\`
        || bytes.len() > 2
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && matches!(bytes[2], b'/' | b'\\')
}

fn parse_git(value: &str) -> VersionProtocol {
    let (url, reference) = match value.split_once('#') {
        Some((url, reference)) => (url, Some(reference.to_owned())),
        None => (value, None),
    };

    VersionProtocol::Git {
        reference,
        url: url.to_owned(),
    }
}

// npm matches a full version without an operator exactly, like the `1.2.3` in
// `1.2.3 || 2`, while version_spec defaults to `~`, so add an explicit `=`
fn parse_range(value: &str) -> Result<Range, SpecError> {
    let value = value
        .split("||")
        .map(|clause| {
            // A bounded range requires versions without an operator
            if clause.contains(" - ") {
                return clause.trim().to_owned();
            }

            let mut reqs = vec![];
            let mut op = String::new();

            for part in clause.split([' ', ',']) {
                if part.is_empty() || part == "&&" {
                    continue;
                }

                // An operator may be separated from its version, like `>= 1.2.3`
                if part
                    .chars()
                    .all(|ch| matches!(ch, '<' | '>' | '=' | '~' | '^' | '!'))
                {
                    op.push_str(part);
                    continue;
                }

                if op.is_empty() && Version::parse(part).is_ok() {
                    reqs.push(format!("={part}"));
                } else {
                    reqs.push(format!("{op}{part}"));
                }

                op.clear();
            }

            // A dangling operator fails to parse
            if !op.is_empty() {
                reqs.push(op);
            }

            reqs.join(" ")
        })
        .collect::<Vec<_>>()
        .join(" || ");

    Range::parse(value)
}

// A range with a single requirement is unwrapped, like `^1.2` or `>=1`,
// while anything else remains a range, like `>=1 <2` or `^1 || ^2`
fn from_range(mut range: Range) -> VersionProtocol {
    if range.clauses.is_empty() {
        return VersionProtocol::Requirement(Requirement {
            op: Op::Wildcard,
            ..Default::default()
        });
    }

    if range.clauses.len() == 1
        && let Clause::Only(req) = &mut range.clauses[0]
    {
        return VersionProtocol::Requirement(std::mem::take(req));
    }

    VersionProtocol::Range(range)
}

// An exact match on a full version is written without an operator,
// as it was most likely parsed without one, like `1.2.3 || 2`
fn format_requirement(req: &Requirement) -> String {
    let value = req.to_string();

    match value.strip_prefix('=') {
        Some(version) if req.op == Op::Exact && req.patch.is_some() => version.to_owned(),
        _ => value,
    }
}

// Formats a range with npm's syntax, in which requirements that must all
// match are separated by a space, instead of `&&`
// https://docs.npmjs.com/cli/v10/configuring-npm/package-json#dependencies
fn format_range(range: &Range) -> String {
    if range.clauses.is_empty() {
        return "*".into();
    }

    range
        .clauses
        .iter()
        .map(|clause| match clause {
            Clause::All(reqs) => reqs
                .iter()
                .map(format_requirement)
                .collect::<Vec<_>>()
                .join(" "),
            Clause::Only(req) => format_requirement(req),
            clause => clause.to_string(),
        })
        .collect::<Vec<_>>()
        .join(" || ")
}

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

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(code(package_json::version::invalid)))]
    Spec(#[from] SpecError),

    #[error(transparent)]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Workspace(#[from] WorkspaceProtocolError),
}

// https://docs.npmjs.com/cli/v10/configuring-npm/package-json#dependencies
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged, try_from = "String", into = "String")]
pub enum VersionProtocol {
    Alias(String),
    Catalog(Option<String>),
    CustomProtocol(String, String),
    Exec(PathBuf),
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
    Patch(String),
    Path(PathBuf),
    Portal(PathBuf),
    Range(Range),
    Requirement(Requirement),
    Tag(String),
    Url(String),
    Version(Version),
    Workspace(WorkspaceProtocol),

    // Fallback case instead of hard failing!
    Unknown(String),
}

impl VersionProtocol {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, VersionProtocolError> {
        Self::from_str(value.as_ref())
    }
}

impl FromStr for VersionProtocol {
    type Err = VersionProtocolError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();

        if value.is_empty() || value == "*" {
            return Ok(VersionProtocol::Requirement(Requirement::parse("*")?));
        }

        // Before protocols, as a Windows drive looks like one, like `c:/`
        if is_local_path(value) {
            return Ok(VersionProtocol::Path(PathBuf::from(value)));
        }

        if let Some(caps) = PROTOCOL.captures(value) {
            let protocol = caps.name("protocol").unwrap().as_str();
            let index = protocol.len();

            match protocol {
                "catalog" => {
                    return Ok(VersionProtocol::Catalog(if index == value.len() - 1 {
                        None
                    } else {
                        Some(String::from(&value[index + 1..]))
                    }));
                }
                "http" | "https" => {
                    return Ok(VersionProtocol::Url(value.to_owned()));
                }
                "git" | "git+ssh" | "git+http" | "git+https" | "git+file" => {
                    return Ok(parse_git(value));
                }
                "exec" => {
                    return Ok(VersionProtocol::Exec(PathBuf::from(&value[index + 1..])));
                }
                "file" => {
                    return Ok(VersionProtocol::File(PathBuf::from(&value[index + 1..])));
                }
                "link" => {
                    return Ok(VersionProtocol::Link(PathBuf::from(&value[index + 1..])));
                }
                "patch" => {
                    return Ok(VersionProtocol::Patch(String::from(&value[index + 1..])));
                }
                "portal" => {
                    return Ok(VersionProtocol::Portal(PathBuf::from(&value[index + 1..])));
                }
                "workspace" => {
                    return Ok(VersionProtocol::Workspace(WorkspaceProtocol::parse(
                        &value[index + 1..],
                    )?));
                }
                "jsr" | "npm" => {
                    return Ok(VersionProtocol::Alias(value.to_owned()));
                }
                custom => {
                    if !value.contains("//") {
                        return Ok(VersionProtocol::CustomProtocol(
                            custom.to_owned(),
                            value[index + 1..].to_owned(),
                        ));
                    }
                }
            }
        }

        if GIT_SCP.is_match(value) {
            return Ok(parse_git(value));
        }

        if let Some(caps) = GITHUB.captures(value) {
            return Ok(VersionProtocol::GitHub {
                owner: caps.name("owner").unwrap().as_str().to_owned(),
                repo: caps.name("repo").unwrap().as_str().to_owned(),
                reference: caps.name("commit").map(|c| c.as_str().to_owned()),
            });
        }

        if value.contains(" - ") {
            if let Ok(range) = parse_range(value) {
                return Ok(from_range(range));
            }

            // A bounded range requires full versions, while npm
            // also supports partial versions, like `1.2 - 2`
            let mut parts = value.split(" - ");
            let l = parts
                .next()
                .ok_or(VersionProtocolError::RangeMissingStartVersion)?
                .trim();
            let r = parts
                .next()
                .ok_or(VersionProtocolError::RangeMissingStopVersion)?
                .trim();

            return Ok(from_range(Range::parse(format!(">={l} <={r}"))?));
        }

        if value.contains("||") {
            return Ok(from_range(parse_range(value)?));
        }

        // Better way to capture tags?
        if value.chars().next().is_some_and(|ch| ch.is_alphabetic()) {
            return Ok(VersionProtocol::Tag(value.to_owned()));
        }

        if let Ok(version) = Version::parse(value) {
            return Ok(VersionProtocol::Version(version));
        }

        match parse_range(value) {
            Ok(range) => Ok(from_range(range)),
            Err(error) => {
                if value.contains(['^', '~', '>', '<', '=']) {
                    Err(error.into())
                } else {
                    Ok(VersionProtocol::Unknown(value.to_owned()))
                }
            }
        }
    }
}

// Deserializing never fails, so that one dependency we can't parse doesn't
// fail the whole `package.json`. Use `parse` or `from_str` for the error.
impl TryFrom<String> for VersionProtocol {
    type Error = VersionProtocolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::from_str(&value).unwrap_or(VersionProtocol::Unknown(value)))
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
                VersionProtocol::Alias(value) => value.to_owned(),
                VersionProtocol::Catalog(value) =>
                    format!("catalog:{}", value.as_deref().unwrap_or_default()),
                VersionProtocol::CustomProtocol(protocol, value) => format!("{protocol}:{value}"),
                VersionProtocol::Exec(path) => format!("exec:{}", path.display()),
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
                VersionProtocol::Patch(patch) => format!("patch:{patch}"),
                VersionProtocol::Path(path) => path.display().to_string(),
                VersionProtocol::Portal(path) => format!("portal:{}", path.display()),
                VersionProtocol::Range(range) => format_range(range),
                VersionProtocol::Requirement(req) => req.to_string(),
                VersionProtocol::Tag(tag) => tag.to_owned(),
                VersionProtocol::Url(url) => url.to_owned(),
                VersionProtocol::Version(ver) => ver.to_string(),
                VersionProtocol::Workspace(ws) => format!("workspace:{ws}"),
                VersionProtocol::Unknown(value) => value.to_owned(),
            }
        )
    }
}
