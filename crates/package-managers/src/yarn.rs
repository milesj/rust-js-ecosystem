use rustc_hash::FxHashMap;
use yarn_lock_parser::{parse_str, Entry};

pub use yarn_lock_parser::YarnLockError;

pub struct YarnLockDependency {
    pub name: String,
    pub version: Option<String>,
    pub integrity: Option<String>,
    pub dependencies: FxHashMap<String, String>,
}

pub struct YarnLock;

impl YarnLock {
    pub fn parse<T: AsRef<str>>(content: T) -> Result<Vec<YarnLockDependency>, YarnLockError> {
        let entries: Vec<Entry> = parse_str(content.as_ref())?;

        Ok(entries
            .into_iter()
            .map(|entry| YarnLockDependency {
                name: entry.name.to_owned(),
                version: if entry.version.is_empty() {
                    None
                } else {
                    Some(entry.version.to_owned())
                },
                integrity: if entry.integrity.is_empty() {
                    None
                } else {
                    Some(entry.integrity.to_owned())
                },
                dependencies: FxHashMap::from_iter(
                    entry
                        .dependencies
                        .into_iter()
                        .map(|(k, v)| (k.to_owned(), v.to_owned())),
                ),
            })
            .collect())
    }
}
