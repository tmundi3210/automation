//! meshcore: protocol core for the offline BLE mesh coordination app.
//!
//! Scope and rules come from `BRIEF_v2.md` (P0) and `DECISIONS.md` (run 2); the
//! module map is in `docs/DESIGN.md`. Protocol code never reads a wall clock:
//! everything time-related goes through [`clock::Clock`].

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod clock;
pub mod codec;
pub mod crypto;
pub mod dtn;
pub mod relay;
pub mod storage;
pub mod transport;
pub mod wire;

pub use clock::Clock;
