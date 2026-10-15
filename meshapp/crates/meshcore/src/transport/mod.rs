//! Transport abstraction (docs/DESIGN.md §7): a `Link` carries whole Noise
//! frames between two phones; the platform (or the simulator) owns the radio.

pub mod chunk;
pub mod link;
pub mod platform_link;

pub use link::{Link, LinkAuth, LinkError, LinkId, LinkInfo, PeerKind, Role};
