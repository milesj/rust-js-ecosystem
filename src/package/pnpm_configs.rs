use serde::Deserialize;
use starbase_utils::yaml;
use std::path::Path;

// https://pnpm.io/pnpm-workspace_yaml
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PnpmWorkspace {
    pub packages: Vec<String>,
}

impl PnpmWorkspace {
    pub fn load<T: AsRef<Path>>(file: T) -> miette::Result<Self> {
        Ok(yaml::read_file(file.as_ref())?)
    }
}
