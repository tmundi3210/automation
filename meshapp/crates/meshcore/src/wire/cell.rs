//! The on-air cell (docs/DESIGN.md §2.2; BRIEF_v2 §5.5 "On air").
//!
//! Every relay-visible unit is exactly [`CELL_LEN`] bytes:
//!
//! | off | len | field | rule |
//! |---|---|---|---|
//! | 0 | 1 | `ver` | 0x01; relays forward any `ver` in 1..=15 but interpret only 1 |
//! | 1 | 1 | `ttl` | sender draws from {6,7,8}; a relay forwards iff `ttl ≥ 2`, with `ttl−1`; excluded from `msg_id` |
//! | 2 | 1 | `token_type` | 0x00 in v1 (reserved for P2 rate tokens) |
//! | 3 | 1 | `token_len` | 0x00 in v1 |
//! | 4 | 16 | `tag` | `prf16(k_tag, "tag", ctr)` for chain cells; a seal hash for first flights |
//! | 20 | 492 | `body` | ChaCha20-Poly1305 ciphertext (476 B plaintext + 16 B auth tag) |
//!
//! `msg_id = SHA256(cell[0] ‖ cell[2..])`: derived, never transmitted, and
//! independent of the TTL so that every hop sees the same id.
//!
//! This file holds the fixed layout and the accessors every other module
//! relies on (DTN store, dedupe, relay limits). Building and validating cells
//! (TTL draw, v1 field rules, body encryption) is in the same module and is
//! completed in milestone 1 wave 2.

use core::fmt;

use crate::crypto::hash::{Hash32, sha256_concat};

/// Total cell size on the air. One constant; the inner layout derives from it.
pub const CELL_LEN: usize = 512;
/// Clear header: ver, ttl, token_type, token_len, tag.
pub const HEADER_LEN: usize = 20;
/// AEAD body (ciphertext + 16-byte auth tag).
pub const BODY_LEN: usize = CELL_LEN - HEADER_LEN;
/// Plaintext carried by one body.
pub const BODY_PLAINTEXT_LEN: usize = BODY_LEN - 16;

pub const OFF_VER: usize = 0;
pub const OFF_TTL: usize = 1;
pub const OFF_TOKEN_TYPE: usize = 2;
pub const OFF_TOKEN_LEN: usize = 3;
pub const OFF_TAG: usize = 4;
pub const TAG_LEN: usize = 16;
pub const OFF_BODY: usize = HEADER_LEN;

/// The only version this implementation interprets.
pub const VERSION: u8 = 1;
/// Relays forward unknown versions in this range (DESIGN §2.2) so that a
/// later format can cross older relays.
pub const FORWARDABLE_VERSIONS: core::ops::RangeInclusive<u8> = 1..=15;
/// TTL start values the sender draws from uniformly (DESIGN §2.2, §3 row
/// "single wire": random initial TTL is mandatory).
pub const TTL_INITIAL_CHOICES: [u8; 3] = [6, 7, 8];
/// A relay forwards a cell only if its TTL is at least this, after
/// decrementing.
pub const TTL_FORWARD_MIN: u8 = 2;

/// Why a byte string is not a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CellReject {
    #[error("cell must be exactly {CELL_LEN} bytes, got {0}")]
    BadLength(usize),
    #[error("cell version {0} is outside the forwardable range")]
    BadVersion(u8),
    #[error("cell token fields must be zero in v1")]
    TokenNotEmpty,
    #[error("cell ttl {0} is above the maximum start value")]
    BadTtl(u8),
}

/// One on-air cell, exactly [`CELL_LEN`] bytes. Comparison and hashing are
/// by content, so two copies of the same cell with different TTLs differ as
/// values but share a [`Cell::msg_id`].
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Cell(pub(crate) [u8; CELL_LEN]);

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cell")
            .field("ver", &self.ver())
            .field("ttl", &self.ttl())
            .field("msg_id", &self.msg_id())
            .finish()
    }
}

impl Cell {
    /// Wraps raw bytes after the length check only. Use [`Cell::validate`]
    /// for the v1 field rules a relay applies.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CellReject> {
        let arr: [u8; CELL_LEN] = bytes
            .try_into()
            .map_err(|_| CellReject::BadLength(bytes.len()))?;
        Ok(Self(arr))
    }

    /// Wraps an owned array; infallible because the length is right by type.
    pub const fn from_array(bytes: [u8; CELL_LEN]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; CELL_LEN] {
        &self.0
    }

    pub fn into_array(self) -> [u8; CELL_LEN] {
        self.0
    }

    pub fn ver(&self) -> u8 {
        self.0[OFF_VER]
    }

    pub fn ttl(&self) -> u8 {
        self.0[OFF_TTL]
    }

    pub fn token_type(&self) -> u8 {
        self.0[OFF_TOKEN_TYPE]
    }

    pub fn token_len(&self) -> u8 {
        self.0[OFF_TOKEN_LEN]
    }

    pub fn tag(&self) -> [u8; TAG_LEN] {
        let mut t = [0u8; TAG_LEN];
        t.copy_from_slice(&self.0[OFF_TAG..OFF_TAG + TAG_LEN]);
        t
    }

    pub fn body(&self) -> &[u8] {
        &self.0[OFF_BODY..]
    }

    /// The relay's format check (BRIEF_v2 §5.5 "checks format, size and
    /// TTL"): version forwardable, v1 token fields empty, TTL not above the
    /// largest start value.
    pub fn validate(&self) -> Result<(), CellReject> {
        if !FORWARDABLE_VERSIONS.contains(&self.ver()) {
            return Err(CellReject::BadVersion(self.ver()));
        }
        if self.ver() == VERSION && (self.token_type() != 0 || self.token_len() != 0) {
            return Err(CellReject::TokenNotEmpty);
        }
        let max_ttl = TTL_INITIAL_CHOICES[TTL_INITIAL_CHOICES.len() - 1];
        if self.ttl() > max_ttl {
            return Err(CellReject::BadTtl(self.ttl()));
        }
        Ok(())
    }

    /// `SHA256(cell[0] ‖ cell[2..])`: everything except the TTL byte.
    pub fn msg_id(&self) -> Hash32 {
        sha256_concat(&[&self.0[OFF_VER..OFF_VER + 1], &self.0[OFF_TOKEN_TYPE..]])
    }

    /// Whether a relay may forward this cell at all (`ttl ≥ 2`).
    pub fn is_forwardable(&self) -> bool {
        self.ttl() >= TTL_FORWARD_MIN
    }

    /// The copy a relay forwards: same bytes, TTL decremented. `None` when the
    /// cell has reached the end of its hop budget.
    pub fn forwarded(&self) -> Option<Self> {
        if !self.is_forwardable() {
            return None;
        }
        let mut next = self.0;
        next[OFF_TTL] -= 1;
        Some(Self(next))
    }

    /// Sets the TTL in place (used by builders and tests).
    pub fn set_ttl(&mut self, ttl: u8) {
        self.0[OFF_TTL] = ttl;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell_with(ver: u8, ttl: u8, fill: u8) -> Cell {
        let mut b = [fill; CELL_LEN];
        b[OFF_VER] = ver;
        b[OFF_TTL] = ttl;
        b[OFF_TOKEN_TYPE] = 0;
        b[OFF_TOKEN_LEN] = 0;
        Cell::from_array(b)
    }

    #[test]
    fn layout_constants_add_up() {
        assert_eq!(HEADER_LEN, 4 + TAG_LEN);
        assert_eq!(HEADER_LEN + BODY_LEN, CELL_LEN);
        assert_eq!(BODY_PLAINTEXT_LEN, 476);
    }

    #[test]
    fn length_is_enforced() {
        assert_eq!(
            Cell::from_bytes(&[0u8; 511]),
            Err(CellReject::BadLength(511))
        );
        assert!(Cell::from_bytes(&[0u8; CELL_LEN]).is_ok());
    }

    #[test]
    fn msg_id_ignores_ttl_only() {
        let a = cell_with(1, 8, 0xAB);
        let mut b = a.clone();
        b.set_ttl(3);
        assert_eq!(a.msg_id(), b.msg_id());
        let mut c = a.clone();
        c.0[OFF_BODY] ^= 1;
        assert_ne!(a.msg_id(), c.msg_id());
        let mut d = a.clone();
        d.0[OFF_VER] = 2;
        assert_ne!(a.msg_id(), d.msg_id());
    }

    #[test]
    fn forwarding_decrements_and_stops_at_one() {
        let c = cell_with(1, 2, 0);
        let f = c.forwarded().expect("ttl 2 forwards");
        assert_eq!(f.ttl(), 1);
        assert!(f.forwarded().is_none());
        assert!(!f.is_forwardable());
    }

    #[test]
    fn validate_applies_v1_rules() {
        assert_eq!(
            cell_with(0, 7, 0).validate(),
            Err(CellReject::BadVersion(0))
        );
        assert_eq!(
            cell_with(16, 7, 0).validate(),
            Err(CellReject::BadVersion(16))
        );
        assert!(
            cell_with(15, 7, 0).validate().is_ok(),
            "unknown but forwardable"
        );
        assert_eq!(cell_with(1, 9, 0).validate(), Err(CellReject::BadTtl(9)));
        let mut t = cell_with(1, 7, 0);
        t.0[OFF_TOKEN_LEN] = 1;
        assert_eq!(t.validate(), Err(CellReject::TokenNotEmpty));
        assert!(cell_with(1, 8, 0).validate().is_ok());
    }
}
