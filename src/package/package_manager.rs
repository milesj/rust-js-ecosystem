#[derive(Clone, Copy, PartialEq)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    YarnLegacy,
}
