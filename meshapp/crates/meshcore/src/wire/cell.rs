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
//! This file holds the fixed layout, the accessors every other module relies
//! on (DTN store, dedupe, relay limits), the sender-side builders (TTL draw,
//! [`build`], [`seal_cell`]) and the body AEAD ([`seal_body`], [`open_body`]).
//!
//! **Body encryption (DESIGN §2.2, shared byte-for-byte with `wire/recog.rs`):**
//!
//! * `cell_key = HKDF-SHA256(ikm = k_hdr, salt = "", info = "meshapp.v1.cell" ‖ LE64(ctr))`
//!   ([`cell_key`]); one recognition counter per cell, so every cell has its
//!   own key and relays cannot group the cells of one object (§2.2 "one cell
//!   = one recognition counter");
//! * `nonce = LE32(frag_idx) ‖ 0⁸` ([`body_nonce`]);
//! * `AAD = cell[0..1] ‖ cell[2..20]`: the whole clear header except the TTL
//!   ([`HeaderAad`]), so a relay's decrement does not break authentication
//!   while any other header change does;
//! * AEAD = ChaCha20-Poly1305 ([`crate::crypto::aead::seal`] / [`open`]).
//!
//! Nothing here reads a clock (BRIEF_v2 §4 "no timestamps decide anything").

use core::fmt;

use crate::crypto::aead::{self, Key32, NONCE_LEN};
use crate::crypto::hash::{Hash32, hkdf32, sha256_concat};

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

/// Bytes of the header that the body AEAD authenticates: everything except
/// the TTL byte (DESIGN §2.2 "AAD = bytes 0, 2..20").
pub const HEADER_AAD_LEN: usize = HEADER_LEN - 1;

/// HKDF `info` prefix of the per-cell body key (DESIGN §2.2 `key =
/// HKDF(k_hdr, "cell", ctr)`, spelled with the protocol's domain prefix).
/// The full info is `CELL_KEY_INFO ‖ LE64(ctr)`.
pub const CELL_KEY_INFO: &[u8] = b"meshapp.v1.cell";

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

/// Why a byte string is not a cell, or why a body did not open.
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
    /// The body did not authenticate under the given key, counter and
    /// fragment index: wrong chain or counter, a replayed cell under another
    /// counter, or a header/body byte changed in transit (DESIGN §2.2 AAD;
    /// BRIEF_v2 §5.5 "replays fail AEAD").
    #[error("cell body failed authentication")]
    BodyAuthFailed,
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

    /// The body as a fixed array (ciphertext ‖ 16-byte auth tag).
    pub fn body_array(&self) -> [u8; BODY_LEN] {
        let mut b = [0u8; BODY_LEN];
        b.copy_from_slice(&self.0[OFF_BODY..]);
        b
    }

    /// The header fields the body AEAD authenticates (everything but the
    /// TTL), as a struct.
    pub fn header_aad(&self) -> HeaderAad {
        HeaderAad {
            ver: self.ver(),
            token_type: self.token_type(),
            token_len: self.token_len(),
            tag: self.tag(),
        }
    }

    /// `cell[0..1] ‖ cell[2..20]`: the AAD bytes of the body AEAD (DESIGN
    /// §2.2).
    pub fn aad_bytes(&self) -> [u8; HEADER_AAD_LEN] {
        let mut aad = [0u8; HEADER_AAD_LEN];
        aad[0] = self.0[OFF_VER];
        aad[1..].copy_from_slice(&self.0[OFF_TOKEN_TYPE..HEADER_LEN]);
        aad
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

    /// Opens this cell's body under the chain header key, recognition
    /// counter and fragment index the tag table resolved. See [`open_body`].
    pub fn open_body(
        &self,
        k_hdr: &[u8; 32],
        ctr: u64,
        frag_idx: u16,
    ) -> Result<[u8; BODY_PLAINTEXT_LEN], CellReject> {
        open_body(k_hdr, ctr, frag_idx, self)
    }
}

/// The clear header without its TTL: the associated data of the body AEAD
/// (DESIGN §2.2 "AAD = bytes 0, 2..20"). A sender fills it before sealing;
/// a receiver takes it from the cell with [`Cell::header_aad`]. The TTL is
/// left out so that relays can decrement it without re-authenticating,
/// while any other header change (version, token slot, tag) fails
/// [`open_body`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeaderAad {
    pub ver: u8,
    pub token_type: u8,
    pub token_len: u8,
    pub tag: [u8; TAG_LEN],
}

impl HeaderAad {
    /// Encoded length: `ver ‖ token_type ‖ token_len ‖ tag`.
    pub const LEN: usize = HEADER_AAD_LEN;

    /// A header with the given version and tag and the v1 empty token slot.
    pub const fn new(ver: u8, tag: [u8; TAG_LEN]) -> Self {
        HeaderAad {
            ver,
            token_type: 0,
            token_len: 0,
            tag,
        }
    }

    /// A v1 header for `tag`.
    pub const fn v1(tag: [u8; TAG_LEN]) -> Self {
        Self::new(VERSION, tag)
    }

    /// The AAD a received cell carries.
    pub fn from_cell(cell: &Cell) -> Self {
        cell.header_aad()
    }

    /// `ver ‖ token_type ‖ token_len ‖ tag`, i.e. `cell[0..1] ‖ cell[2..20]`.
    pub fn to_bytes(&self) -> [u8; HEADER_AAD_LEN] {
        let mut aad = [0u8; HEADER_AAD_LEN];
        aad[0] = self.ver;
        aad[1] = self.token_type;
        aad[2] = self.token_len;
        aad[3..].copy_from_slice(&self.tag);
        aad
    }
}

/// Draws the initial TTL uniformly from [`TTL_INITIAL_CHOICES`] (DESIGN
/// §2.2; DECISIONS §3 "single wire" row: a random initial TTL in [6,8] is
/// mandatory so that a cell at its starting value does not mark the
/// originator to the first hop).
///
/// Uniformity: rejection sampling on `next_u32` (the top `2^32 mod 3`
/// values are redrawn), so no choice is favoured by the modulo. The same
/// RNG state always gives the same draw.
pub fn draw_initial_ttl(rng: &mut dyn rand::RngCore) -> u8 {
    const N: u32 = 3;
    const _: () = assert!(TTL_INITIAL_CHOICES.len() == N as usize);
    // Largest multiple of N that fits in a u32; draws at or above it are
    // rejected so that every residue class is equally likely.
    const LIMIT: u32 = u32::MAX - (u32::MAX % N);
    fn choice(residue: u32) -> u8 {
        match residue {
            0 => TTL_INITIAL_CHOICES[0],
            1 => TTL_INITIAL_CHOICES[1],
            _ => TTL_INITIAL_CHOICES[2],
        }
    }
    // A sound RNG rejects with probability 2^-32 per draw; the bound only
    // keeps a broken RNG (e.g. one stuck on u32::MAX) from hanging the
    // sender, at the cost of a negligible bias in that case.
    for _ in 0..64 {
        let v = rng.next_u32();
        if v < LIMIT {
            return choice(v % N);
        }
    }
    choice(rng.next_u32() % N)
}

/// Assembles a cell from its clear fields and an already-sealed body. The
/// token fields are zero (DESIGN §2.2 "0x00 in v1"). The version is written
/// as given; senders pass [`VERSION`].
pub fn build(ver: u8, ttl: u8, tag: &[u8; TAG_LEN], body: &[u8; BODY_LEN]) -> Cell {
    let mut bytes = [0u8; CELL_LEN];
    bytes[OFF_VER] = ver;
    bytes[OFF_TTL] = ttl;
    bytes[OFF_TOKEN_TYPE] = 0;
    bytes[OFF_TOKEN_LEN] = 0;
    bytes[OFF_TAG..OFF_TAG + TAG_LEN].copy_from_slice(tag);
    bytes[OFF_BODY..].copy_from_slice(body);
    Cell(bytes)
}

/// The per-cell body key: `HKDF-SHA256(ikm = k_hdr, salt = "", info =
/// "meshapp.v1.cell" ‖ LE64(ctr))` (DESIGN §2.2). `ctr` is the recognition
/// counter of this cell, so each cell of an object has its own key.
pub fn cell_key(k_hdr: &[u8; 32], ctr: u64) -> Key32 {
    let mut info = [0u8; CELL_KEY_INFO.len() + 8];
    info[..CELL_KEY_INFO.len()].copy_from_slice(CELL_KEY_INFO);
    info[CELL_KEY_INFO.len()..].copy_from_slice(&ctr.to_le_bytes());
    Key32::from(hkdf32(k_hdr, b"", &info))
}

/// The body nonce: `LE32(frag_idx) ‖ 0⁸` (DESIGN §2.2). The key already
/// changes per cell, so the nonce only needs to differ per fragment index.
pub fn body_nonce(frag_idx: u16) -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    nonce[..4].copy_from_slice(&u32::from(frag_idx).to_le_bytes());
    nonce
}

/// Seals one body: ChaCha20-Poly1305 under [`cell_key`]`(k_hdr, ctr)`,
/// nonce [`body_nonce`]`(frag_idx)`, AAD = `header_without_ttl` (DESIGN
/// §2.2). The output is exactly [`BODY_LEN`] bytes: ciphertext ‖ tag.
pub fn seal_body(
    k_hdr: &[u8; 32],
    ctr: u64,
    frag_idx: u16,
    header_without_ttl: &HeaderAad,
    plaintext: &[u8; BODY_PLAINTEXT_LEN],
) -> [u8; BODY_LEN] {
    let key = cell_key(k_hdr, ctr);
    let nonce = body_nonce(frag_idx);
    let aad = header_without_ttl.to_bytes();
    let ct = aead::seal(&key, &nonce, &aad, plaintext);
    debug_assert_eq!(ct.len(), BODY_LEN);
    let mut body = [0u8; BODY_LEN];
    body.copy_from_slice(&ct);
    body
}

/// Opens a received cell's body with the chain header key, counter and
/// fragment index that the recognition table resolved from its tag. The AAD
/// is taken from the cell itself (everything but the TTL), so a relay's
/// decrement is fine while a changed version, token slot, tag or body byte,
/// a wrong counter or a wrong fragment index all fail with
/// [`CellReject::BodyAuthFailed`].
pub fn open_body(
    k_hdr: &[u8; 32],
    ctr: u64,
    frag_idx: u16,
    cell: &Cell,
) -> Result<[u8; BODY_PLAINTEXT_LEN], CellReject> {
    let key = cell_key(k_hdr, ctr);
    let nonce = body_nonce(frag_idx);
    let aad = cell.aad_bytes();
    let pt = aead::open(&key, &nonce, &aad, cell.body()).map_err(|_| CellReject::BodyAuthFailed)?;
    let arr: [u8; BODY_PLAINTEXT_LEN] = pt
        .as_slice()
        .try_into()
        .map_err(|_| CellReject::BodyAuthFailed)?;
    Ok(arr)
}

/// Builds and seals a whole cell: header `{ver, ttl, 0, 0, tag}` and the
/// body of `plaintext` under `(k_hdr, ctr, frag_idx)`. Deterministic: the
/// same inputs give the same bytes.
pub fn seal_cell(
    ver: u8,
    ttl: u8,
    tag: &[u8; TAG_LEN],
    k_hdr: &[u8; 32],
    ctr: u64,
    frag_idx: u16,
    plaintext: &[u8; BODY_PLAINTEXT_LEN],
) -> Cell {
    let header = HeaderAad::new(ver, *tag);
    let body = seal_body(k_hdr, ctr, frag_idx, &header, plaintext);
    build(ver, ttl, tag, &body)
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

    // ---- wave 2: builders and body AEAD --------------------------------

    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    fn k_hdr() -> [u8; 32] {
        let mut k = [0u8; 32];
        for (i, b) in k.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(7).wrapping_add(3);
        }
        k
    }

    fn plaintext(fill: u8) -> [u8; BODY_PLAINTEXT_LEN] {
        let mut p = [fill; BODY_PLAINTEXT_LEN];
        p[0] = 0x01;
        p[BODY_PLAINTEXT_LEN - 1] = 0xFE;
        p
    }

    #[test]
    fn build_writes_header_and_body_with_empty_token_slot() {
        let body = [0x5Au8; BODY_LEN];
        let c = build(VERSION, 7, &[0x11; TAG_LEN], &body);
        assert_eq!(c.ver(), 1);
        assert_eq!(c.ttl(), 7);
        assert_eq!(c.token_type(), 0);
        assert_eq!(c.token_len(), 0);
        assert_eq!(c.tag(), [0x11; TAG_LEN]);
        assert_eq!(c.body(), &body[..]);
        assert_eq!(c.body_array(), body);
        assert!(c.validate().is_ok());
        assert_eq!(c.header_aad(), HeaderAad::v1([0x11; TAG_LEN]));
        assert_eq!(c.aad_bytes(), c.header_aad().to_bytes());
    }

    #[test]
    fn header_aad_layout_is_ver_then_bytes_2_to_20() {
        let mut raw = [0u8; CELL_LEN];
        for (i, b) in raw.iter_mut().enumerate() {
            *b = i as u8;
        }
        let c = Cell::from_array(raw);
        let aad = c.aad_bytes();
        assert_eq!(aad.len(), 19);
        assert_eq!(aad[0], raw[0]);
        assert_eq!(&aad[1..], &raw[2..20]);
        assert_eq!(HeaderAad::from_cell(&c).to_bytes(), aad);
        assert_eq!(HeaderAad::LEN, HEADER_AAD_LEN);
    }

    #[test]
    fn cell_key_and_nonce_follow_the_design_derivation() {
        let k = k_hdr();
        let mut info = CELL_KEY_INFO.to_vec();
        info.extend_from_slice(&9u64.to_le_bytes());
        assert_eq!(cell_key(&k, 9).as_bytes(), &hkdf32(&k, b"", &info));
        assert_ne!(cell_key(&k, 9), cell_key(&k, 10));
        assert_eq!(
            body_nonce(0x0102),
            [0x02, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(body_nonce(0), [0u8; NONCE_LEN]);
    }

    #[test]
    fn seal_then_open_round_trips_and_is_deterministic() {
        let k = k_hdr();
        let pt = plaintext(0x33);
        let tag = [0x22u8; TAG_LEN];
        let c1 = seal_cell(VERSION, 8, &tag, &k, 5, 2, &pt);
        let c2 = seal_cell(VERSION, 8, &tag, &k, 5, 2, &pt);
        assert_eq!(c1, c2, "same inputs, same bytes");
        assert_eq!(open_body(&k, 5, 2, &c1), Ok(pt));
        assert_eq!(c1.open_body(&k, 5, 2), Ok(pt));
        // the body alone matches seal_body with the same header
        let body = seal_body(&k, 5, 2, &HeaderAad::v1(tag), &pt);
        assert_eq!(c1.body(), &body[..]);
    }

    #[test]
    fn open_survives_ttl_decrement_but_not_other_changes() {
        let k = k_hdr();
        let pt = plaintext(0x44);
        let c = seal_cell(VERSION, 7, &[0x01; TAG_LEN], &k, 77, 0, &pt);
        let fwd = c.forwarded().expect("ttl 7 forwards");
        assert_eq!(open_body(&k, 77, 0, &fwd), Ok(pt));
        assert_eq!(open_body(&k, 78, 0, &c), Err(CellReject::BodyAuthFailed));
        assert_eq!(open_body(&k, 77, 1, &c), Err(CellReject::BodyAuthFailed));
        let mut other_key = k;
        other_key[0] ^= 1;
        assert_eq!(
            open_body(&other_key, 77, 0, &c),
            Err(CellReject::BodyAuthFailed)
        );
        for off in [
            OFF_VER,
            OFF_TOKEN_TYPE,
            OFF_TOKEN_LEN,
            OFF_TAG,
            OFF_TAG + 15,
            OFF_BODY,
            CELL_LEN - 1,
        ] {
            let mut t = c.clone();
            t.0[off] ^= 0x80;
            assert_eq!(
                open_body(&k, 77, 0, &t),
                Err(CellReject::BodyAuthFailed),
                "flip at offset {off}"
            );
        }
    }

    #[test]
    fn ttl_draw_is_within_choices_and_deterministic() {
        let mut a = ChaCha20Rng::seed_from_u64(1);
        let mut b = ChaCha20Rng::seed_from_u64(1);
        let mut seen = [0usize; 3];
        for _ in 0..300 {
            let x = draw_initial_ttl(&mut a);
            assert_eq!(x, draw_initial_ttl(&mut b));
            let i = TTL_INITIAL_CHOICES
                .iter()
                .position(|&c| c == x)
                .expect("draw is one of the choices");
            seen[i] += 1;
        }
        assert!(seen.iter().all(|&n| n > 0), "every choice drawn: {seen:?}");
    }

    /// An RNG stuck on the rejected value must not hang the sender.
    #[test]
    fn ttl_draw_terminates_on_a_stuck_rng() {
        struct Stuck;
        impl rand::RngCore for Stuck {
            fn next_u32(&mut self) -> u32 {
                u32::MAX
            }
            fn next_u64(&mut self) -> u64 {
                u64::MAX
            }
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                dest.fill(0xFF);
            }
        }
        let t = draw_initial_ttl(&mut Stuck);
        assert!(TTL_INITIAL_CHOICES.contains(&t));
    }
}
