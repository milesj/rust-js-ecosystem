use crate::yarn::{YarnLock, YarnLockError};
use crate::LockfileDependency;

pub type BunLockbError = YarnLockError;

pub struct BunLockb;

impl BunLockb {
    pub fn parse<T: AsRef<str>>(content: T) -> Result<Vec<LockfileDependency>, BunLockbError> {
        // Bun's lockfile is in yarn v1 format, so this is ok!
        YarnLock::parse(content)
    }
}
