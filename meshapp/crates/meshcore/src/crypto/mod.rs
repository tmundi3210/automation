//! Cryptographic primitives and the pluggable backends (docs/DESIGN.md §1).

pub mod aead;
pub mod backend;
pub mod ed25519;
pub mod hash;
pub mod keycard;
