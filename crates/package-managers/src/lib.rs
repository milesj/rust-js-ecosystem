pub mod bun;
pub mod npm;
pub mod pnpm;
pub mod yarn;

use rustc_hash::FxHashMap;

#[derive(Clone, Copy, PartialEq)]
pub enum PackageManager {
    Bun,
    Npm,
    Pnpm,
    Yarn,
    YarnLegacy,
}

pub struct LockfileDependency {
    pub name: String,
    pub version: Option<String>,
    pub integrity: Option<String>,
    pub dependencies: FxHashMap<String, String>,
}
