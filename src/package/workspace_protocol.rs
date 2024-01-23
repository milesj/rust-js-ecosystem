#![allow(clippy::from_over_into)]

use semver::{Error, Version};
use serde::Deserialize;
use std::fmt::{self, Display};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
#[serde(untagged, into = "String", try_from = "String")]
pub enum WorkspaceProtocol {
    // *
    Any {
        alias: Option<String>,
    },
    // ~
    Tilde {
        alias: Option<String>,
        version: Option<Version>,
    },
    // ^
    Caret {
        alias: Option<String>,
        version: Option<Version>,
    },
    // ../file
    File(PathBuf),
    // 1.2.3
    Version(Version),
}

impl FromStr for WorkspaceProtocol {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut alias = None;
        let mut value = value;

        // https://pnpm.io/workspaces#referencing-workspace-packages-through-aliases
        if let Some(index) = value.find('@') {
            alias = Some(value[0..index].to_owned());
            value = &value[index + 1..];
        }

        match &value[0..1] {
            "*" => {
                if value.len() != 1 {
                    panic!("Wildcard workspace does not support versions.");
                }

                return Ok(WorkspaceProtocol::Any { alias });
            }
            "^" | "~" => {
                let mut version = None;

                if value.len() > 1 {
                    version = Some(Version::parse(&value[1..])?);
                }

                if value.starts_with('^') {
                    return Ok(WorkspaceProtocol::Caret { alias, version });
                } else {
                    return Ok(WorkspaceProtocol::Tilde { alias, version });
                }
            }
            "." | "/" => {
                return Ok(WorkspaceProtocol::File(PathBuf::from(value)));
            }
            _ => {}
        };

        Ok(WorkspaceProtocol::Version(Version::parse(value)?))
    }
}

impl TryFrom<String> for WorkspaceProtocol {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl Into<String> for WorkspaceProtocol {
    fn into(self) -> String {
        self.to_string()
    }
}

fn format_variant(prefix: char, alias: Option<&String>, version: Option<&Version>) -> String {
    let mut result = format!("{prefix}");

    if let Some(alias) = alias {
        result = format!("{alias}@{result}");
    }

    if let Some(version) = version {
        result.push_str(&version.to_string());
    }

    result
}

impl Display for WorkspaceProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "workspace:{}",
            match self {
                WorkspaceProtocol::Any { alias } => format_variant('*', alias.as_ref(), None),
                WorkspaceProtocol::Tilde { alias, version } =>
                    format_variant('~', alias.as_ref(), version.as_ref()),
                WorkspaceProtocol::Caret { alias, version } =>
                    format_variant('^', alias.as_ref(), version.as_ref()),
                WorkspaceProtocol::File(path) => path.display().to_string(),
                WorkspaceProtocol::Version(ver) => ver.to_string(),
            }
        )
    }
}
