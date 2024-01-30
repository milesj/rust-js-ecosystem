use serde::Deserialize;

// https://pnpm.io/pnpm-workspace_yaml
#[derive(Default, Deserialize)]
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
