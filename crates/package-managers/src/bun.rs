use crate::yarn::{YarnLock, YarnLockDependency, YarnLockError};

pub type BunLockbError = YarnLockError;
pub type BunLockbDependency = YarnLockDependency;

pub struct BunLockb;

impl BunLockb {
    pub fn parse<T: AsRef<str>>(content: T) -> Result<Vec<BunLockbDependency>, BunLockbError> {
        // Bun's lockfile is in yarn v1 format, so this is ok!
        YarnLock::parse(content)
    }
}
