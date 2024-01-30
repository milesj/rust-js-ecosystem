pub mod pnpm;

#[derive(Clone, Copy, PartialEq)]
pub enum PackageManager {
    Bun,
    Npm,
    Pnpm,
    Yarn,
    YarnLegacy,
}
