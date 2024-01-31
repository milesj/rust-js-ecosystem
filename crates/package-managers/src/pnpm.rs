use crate::LockfileDependency;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use serde_yaml::{Error, Value};

// https://pnpm.io/pnpm-workspace_yaml
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PnpmWorkspaceYaml {
    pub packages: Vec<String>,
}

#[cfg(feature = "loader")]
impl PnpmWorkspaceYaml {
    pub fn load<T: AsRef<std::path::Path>>(file: T) -> miette::Result<Self> {
        Ok(starbase_utils::yaml::read_file(file.as_ref())?)
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PnpmLockPackage {
    pub dependencies: Option<FxHashMap<String, Value>>, // string or number
    pub resolution: PnpmLockPackageResolution,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PnpmLockPackageResolution {
    pub commit: Option<String>, // git
    pub integrity: Option<String>,
    pub tarball: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PnpmLockYaml {
    pub packages: Option<FxHashMap<String, PnpmLockPackage>>,
}

impl PnpmLockYaml {
    pub fn parse<T: AsRef<str>>(content: T) -> Result<Vec<LockfileDependency>, Error> {
        let data: PnpmLockYaml = serde_yaml::from_str(content.as_ref())?;
        let mut deps = vec![];

        if let Some(packages) = data.packages {
            for (name, package) in packages {
                let mut dependencies = FxHashMap::default();

                if let Some(deps) = package.dependencies {
                    for (dep, value) in deps {
                        let value = match value {
                            Value::Number(num) => num.to_string(),
                            Value::String(val) => val,
                            _ => continue,
                        };

                        dependencies.insert(dep, value);
                    }
                }

                deps.push(LockfileDependency {
                    name,
                    version: package.version,
                    integrity: package.resolution.integrity.or(package.resolution.commit),
                    dependencies,
                });
            }
        }

        Ok(deps)
    }
}
