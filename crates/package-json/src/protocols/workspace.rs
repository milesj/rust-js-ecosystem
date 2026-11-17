use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;
use version_spec::{Requirement, SpecError, Version};

#[derive(Debug, Error)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum WorkspaceProtocolError {
    #[error("Star workspace (workspace:*) does not support versions.")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(code(package_json::workspace::no_version_with_star))
    )]
    StarNoVersion,

    #[error(transparent)]
    #[cfg_attr(
        feature = "miette",
        diagnostic(code(package_json::workspace::invalid_version))
    )]
    Spec(#[from] SpecError),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged, try_from = "String", into = "String")]
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
    // ^1, ~1.2, >=1.0.0, etc
    Requirement {
        alias: Option<String>,
        req: Requirement,
    },
    // 1.2.3
    Version {
        alias: Option<String>,
        version: Version,
    },
}

impl WorkspaceProtocol {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, WorkspaceProtocolError> {
        Self::from_str(value.as_ref())
    }
}

impl FromStr for WorkspaceProtocol {
    type Err = WorkspaceProtocolError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut alias = None;
        let mut value = value.trim();

        // https://pnpm.io/workspaces#referencing-workspace-packages-through-aliases
        // An alias may start with a scope's `@`, but not a path character,
        // so that a path containing an `@`, like `../@scope/pkg`, isn't split
        if !value.starts_with(['.', '_', '/'])
            && let Some((index, _)) = value.char_indices().skip(1).find(|(_, ch)| *ch == '@')
        {
            alias = Some(value[0..index].to_owned());
            value = &value[index + 1..];
        }

        let Some(first) = value.chars().next() else {
            return Ok(WorkspaceProtocol::Any { alias });
        };

        match first {
            '*' => {
                if value.len() != 1 {
                    return Err(WorkspaceProtocolError::StarNoVersion);
                }

                return Ok(WorkspaceProtocol::Any { alias });
            }
            '^' | '~' => {
                let mut version = None;

                if value.len() > 1 {
                    match Version::parse(&value[1..]) {
                        Ok(inner) => version = Some(inner),
                        // Partial versions, like ^1 or ~1.2
                        Err(_) => {
                            return Ok(WorkspaceProtocol::Requirement {
                                alias,
                                req: Requirement::parse(value)?,
                            });
                        }
                    };
                }

                if value.starts_with('^') {
                    return Ok(WorkspaceProtocol::Caret { alias, version });
                } else {
                    return Ok(WorkspaceProtocol::Tilde { alias, version });
                }
            }
            // A path can't have an alias, so fall through to fail parsing
            '.' | '/' if alias.is_none() => {
                return Ok(WorkspaceProtocol::File(PathBuf::from(value)));
            }
            _ => {}
        };

        match Version::parse(value) {
            Ok(version) => Ok(WorkspaceProtocol::Version { alias, version }),
            // Any other range, like >=1.0.0 or 1
            Err(_) => Ok(WorkspaceProtocol::Requirement {
                alias,
                req: Requirement::parse(value)?,
            }),
        }
    }
}

impl TryFrom<String> for WorkspaceProtocol {
    type Error = WorkspaceProtocolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<WorkspaceProtocol> for String {
    fn from(value: WorkspaceProtocol) -> String {
        value.to_string()
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

impl fmt::Display for WorkspaceProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorkspaceProtocol::Any { alias } => format_variant('*', alias.as_ref(), None),
                WorkspaceProtocol::Tilde { alias, version } =>
                    format_variant('~', alias.as_ref(), version.as_ref()),
                WorkspaceProtocol::Caret { alias, version } =>
                    format_variant('^', alias.as_ref(), version.as_ref()),
                WorkspaceProtocol::File(path) => path.display().to_string(),
                WorkspaceProtocol::Requirement { alias, req } => match alias {
                    Some(alias) => format!("{alias}@{req}"),
                    None => req.to_string(),
                },
                WorkspaceProtocol::Version { alias, version } => match alias {
                    Some(alias) => format!("{alias}@{version}"),
                    None => version.to_string(),
                },
            }
        )
    }
}
