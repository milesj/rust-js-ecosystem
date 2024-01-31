use crate::LockfileDependency;
use package_lock_json_parser::parse;
use rustc_hash::FxHashMap;

pub use package_lock_json_parser::PackageLockJsonError;

pub struct PackageLockJson;

impl PackageLockJson {
    pub fn parse<T: AsRef<str>>(
        content: T,
    ) -> Result<Vec<LockfileDependency>, PackageLockJsonError> {
        let data = parse(content.as_ref())?;

        // v1
        if let Some(entries) = data.packages {
            return Ok(entries
                .into_iter()
                .map(|(name, entry)| {
                    let mut dependencies = FxHashMap::default();

                    if let Some(prod_deps) = entry.dependencies {
                        dependencies.extend(prod_deps);
                    }

                    if let Some(dev_deps) = entry.dev_dependencies {
                        dependencies.extend(dev_deps);
                    }

                    LockfileDependency {
                        name: entry.name.unwrap_or(name),
                        version: if entry.version.is_empty() {
                            None
                        } else {
                            Some(entry.version.to_owned())
                        },
                        integrity: entry.integrity,
                        dependencies,
                    }
                })
                .collect());
        }

        // v1
        if let Some(entries) = data.dependencies {
            return Ok(entries
                .into_iter()
                .map(|(name, entry)| LockfileDependency {
                    name,
                    version: if entry.version.is_empty() {
                        None
                    } else {
                        Some(entry.version.to_owned())
                    },
                    integrity: entry.integrity,
                    dependencies: FxHashMap::from_iter(
                        entry
                            .dependencies
                            .unwrap_or_default()
                            .into_iter()
                            .map(|(k, v)| (k, v.version)),
                    ),
                })
                .collect());
        }

        Ok(vec![])
    }
}
