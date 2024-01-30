pub mod bun;
pub mod npm;
pub mod pnpm;
pub mod yarn;

#[derive(Clone, Copy, PartialEq)]
pub enum PackageManager {
    Bun,
    Npm,
    Pnpm,
    Yarn,
    YarnLegacy,
}
