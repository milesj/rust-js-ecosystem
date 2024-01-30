#[cfg(feature = "protocols")]
mod version;

#[cfg(feature = "protocols")]
mod workspace;

#[cfg(feature = "protocols")]
pub use version::*;

#[cfg(feature = "protocols")]
pub use workspace::*;

#[cfg(not(feature = "protocols"))]
pub type VersionProtocol = String;

#[cfg(not(feature = "protocols"))]
pub type WorkspaceProtocol = String;
