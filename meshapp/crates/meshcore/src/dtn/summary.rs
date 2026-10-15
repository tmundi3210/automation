//! Bloom summary vectors (docs/DESIGN.md §1 row `dtn/summary.rs`; §2.3
//! `Summary chunk = {salt u32, n_bits u32, k u8, chunk_idx u16, chunk_total
//! u16, bits}`; DECISIONS D10 (4) "anti-entropy by summary-vector
//! exchange"; §3 row (1,2,3,4,10) (iv) "summary vectors carry the hashes
//! of rejected items so they are not re-offered").
//!
//! A [`SummaryVector`] is a salted Bloom filter over `msg_id`s (DESIGN §2.2
//! "summary vectors … key on the derived value"). The on-air rule, which
//! both ends must compute identically:
//!
//! ```text
//! position_i(id) = LE64(SHA256(LE32(salt) ‖ LE8(i) ‖ id)[0..8]) mod n_bits,  i in 0..k
//! bit b lives in bits[b / 8], mask 1 << (b % 8)
//! ```
//!
//! * `salt` is fresh per encounter (DESIGN §10 risk 4 "salt rotation per
//!   encounter"): a false positive under one salt is independent of the
//!   next encounter's, so rarely-met peers do not accumulate misses.
//! * `n_bits` is a multiple of 8 in `[MIN_N_BITS, MAX_N_BITS]` and `k` in
//!   `[1, MAX_K]`; [`sizing`] picks them from the item count and a target
//!   false-positive rate with the standard formulas.
//! * A vector travels as [`SummaryChunk`]s of at most
//!   [`MAX_SUMMARY_BITS_LEN`] bytes each (DESIGN §1 "chunked into
//!   frames"); chunks may arrive in any order and
//!   [`SummaryVector::from_chunks`] / [`ChunkAssembler`] rebuild the vector
//!   or report why not ([`SummaryError`]). Decoding never panics on any
//!   input (BRIEF_v2 §5.5 "checks format, size"; DESIGN §10 fuzz target
//!   `summary_decode`).
//!
//! No false negatives: an id that was inserted is always reported as
//! contained. Everything is a pure function of its inputs.

use std::collections::BTreeSet;

use crate::crypto::hash::{Hash32, sha256_concat};
use crate::wire::frame::{MAX_SUMMARY_BITS_LEN, SummaryChunk};

/// Smallest vector: one byte.
pub const MIN_N_BITS: u32 = 8;
/// Largest vector: 64 kB (`65_536 × 8` bits), about 55 k ids at 1 %.
pub const MAX_N_BITS: u32 = 65_536 * 8;
/// Largest number of hash functions.
pub const MAX_K: u8 = 16;
/// Smallest chunk `to_chunks` makes, so that a vector of [`MAX_N_BITS`]
/// never needs more than [`MAX_CHUNK_TOTAL`] chunks.
pub const MIN_CHUNK_BYTES: usize = 16;
/// Largest `chunk_total` a decoder accepts (`MAX_N_BITS / 8 /
/// MIN_CHUNK_BYTES`), bounding what a peer can make us buffer.
pub const MAX_CHUNK_TOTAL: u16 = 4096;
/// Target false-positive rate of a summary (DESIGN §10 risk 4 "1 % false
/// negatives per encounter"; an assumption the simulator retunes).
pub const SUMMARY_TARGET_FPR_ASM: f64 = 0.01;

const _: () = assert!((MAX_N_BITS as usize) / 8 / MIN_CHUNK_BYTES == MAX_CHUNK_TOTAL as usize);
const _: () = assert!(MIN_CHUNK_BYTES <= MAX_SUMMARY_BITS_LEN);

/// Why chunks did not make a vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum SummaryError {
    #[error("no chunks")]
    NoChunks,
    /// `n_bits` not a byte multiple or out of range, `k` out of range,
    /// `chunk_total` zero or above [`MAX_CHUNK_TOTAL`], or `chunk_idx ≥
    /// chunk_total`.
    #[error("bad summary parameters: {0}")]
    BadParams(&'static str),
    /// Chunks disagree on `salt`, `n_bits`, `k` or `chunk_total`, or two
    /// copies of one index carry different bits.
    #[error("summary chunks do not belong to one vector")]
    Mismatch,
    /// Not every index in `0..chunk_total` has arrived.
    #[error("summary incomplete: {have} of {total} chunks")]
    Incomplete { have: u16, total: u16 },
    /// The chunks together do not hold exactly `n_bits / 8` bytes.
    #[error("summary bits length {got}, expected {expected}")]
    BadLength { expected: usize, got: usize },
}

/// Standard Bloom sizing: `m = ceil(-n ln p / ln²2)` bits rounded up to a
/// byte multiple and clamped to `[MIN_N_BITS, MAX_N_BITS]`, `k =
/// round(m/n · ln 2)` clamped to `[1, MAX_K]`. `n_items = 0` sizes for one
/// item; a non-finite or out-of-range `target_fpr` falls back to
/// [`SUMMARY_TARGET_FPR_ASM`]. The result only parameterises the local
/// vector (the peer reads `n_bits` and `k` from the chunks), so float
/// rounding differences between platforms cannot desynchronise anything.
pub fn sizing(n_items: usize, target_fpr: f64) -> (u32, u8) {
    let p = if target_fpr.is_finite() && target_fpr > 0.0 && target_fpr < 1.0 {
        target_fpr
    } else {
        SUMMARY_TARGET_FPR_ASM
    };
    let n = n_items.max(1) as f64;
    let ln2 = core::f64::consts::LN_2;
    let m = (-n * p.ln() / (ln2 * ln2)).ceil();
    // f64 → u32 `as` saturates (and maps NaN to 0); the clamp below fixes
    // the range either way.
    let m_bits = m.clamp(f64::from(MIN_N_BITS), f64::from(MAX_N_BITS)) as u32;
    let n_bits = round_up_to_byte(m_bits).clamp(MIN_N_BITS, MAX_N_BITS);
    let k = (f64::from(n_bits) / n * ln2).round();
    let k = k.clamp(1.0, f64::from(MAX_K)) as u8;
    (n_bits, k)
}

fn round_up_to_byte(bits: u32) -> u32 {
    bits.saturating_add(7) / 8 * 8
}

/// Whether `(n_bits, k, chunk_total)` are acceptable on the wire.
fn check_params(n_bits: u32, k: u8, chunk_total: u16) -> Result<(), SummaryError> {
    if !(MIN_N_BITS..=MAX_N_BITS).contains(&n_bits) || n_bits % 8 != 0 {
        return Err(SummaryError::BadParams("n_bits"));
    }
    if k == 0 || k > MAX_K {
        return Err(SummaryError::BadParams("k"));
    }
    if chunk_total == 0 || chunk_total > MAX_CHUNK_TOTAL {
        return Err(SummaryError::BadParams("chunk_total"));
    }
    Ok(())
}

/// `position_i(id)` of the module docs. `n_bits` of 0 is treated as 1.
pub fn position(salt: u32, i: u8, id: &Hash32, n_bits: u32) -> u32 {
    let digest = sha256_concat(&[&salt.to_le_bytes(), &[i], id.as_bytes()]);
    let mut first = [0u8; 8];
    first.copy_from_slice(&digest.as_bytes()[..8]);
    let v = u64::from_le_bytes(first);
    let m = u64::from(n_bits.max(1));
    // v % m < m ≤ u32::MAX, so the conversion cannot fail.
    u32::try_from(v % m).unwrap_or(0)
}

/// A salted Bloom filter over `msg_id`s. See the module docs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SummaryVector {
    salt: u32,
    n_bits: u32,
    k: u8,
    bits: Vec<u8>,
}

impl SummaryVector {
    /// An empty vector. `n_bits` is rounded up to a byte multiple and
    /// clamped to `[MIN_N_BITS, MAX_N_BITS]`, `k` to `[1, MAX_K]`.
    pub fn empty(salt: u32, n_bits: u32, k: u8) -> Self {
        let n_bits = round_up_to_byte(n_bits).clamp(MIN_N_BITS, MAX_N_BITS);
        let k = k.clamp(1, MAX_K);
        let len = usize::try_from(n_bits / 8).unwrap_or(0);
        SummaryVector {
            salt,
            n_bits,
            k,
            bits: vec![0u8; len],
        }
    }

    /// A vector containing every id of `ids` (parameters as in
    /// [`empty`](Self::empty)).
    pub fn build(ids: impl IntoIterator<Item = Hash32>, salt: u32, n_bits: u32, k: u8) -> Self {
        let mut v = Self::empty(salt, n_bits, k);
        for id in ids {
            v.insert(&id);
        }
        v
    }

    pub fn salt(&self) -> u32 {
        self.salt
    }

    pub fn n_bits(&self) -> u32 {
        self.n_bits
    }

    pub fn k(&self) -> u8 {
        self.k
    }

    /// The bit array, `n_bits / 8` bytes.
    pub fn bits(&self) -> &[u8] {
        &self.bits
    }

    pub fn len_bytes(&self) -> usize {
        self.bits.len()
    }

    /// Set bits.
    pub fn count_ones(&self) -> u32 {
        self.bits.iter().map(|b| b.count_ones()).sum()
    }

    /// The `k` positions of `id` under this vector's salt and size.
    pub fn positions(&self, id: &Hash32) -> impl Iterator<Item = u32> + '_ {
        let id = *id;
        (0..self.k).map(move |i| position(self.salt, i, &id, self.n_bits))
    }

    pub fn insert(&mut self, id: &Hash32) {
        for pos in self.positions(id).collect::<Vec<_>>() {
            let byte = usize::try_from(pos / 8).unwrap_or(usize::MAX);
            if let Some(b) = self.bits.get_mut(byte) {
                *b |= 1u8 << (pos % 8);
            }
        }
    }

    /// `true` if every position of `id` is set: always for an inserted id,
    /// with probability about the sized false-positive rate otherwise.
    pub fn contains(&self, id: &Hash32) -> bool {
        self.positions(id).all(|pos| {
            let byte = usize::try_from(pos / 8).unwrap_or(usize::MAX);
            self.bits
                .get(byte)
                .is_some_and(|b| b & (1u8 << (pos % 8)) != 0)
        })
    }

    /// `Some(true)` if every set bit of `self` is set in `other` (so every
    /// id in `self` is contained in `other`); `None` if the two are not
    /// comparable (different salt, size or `k`).
    pub fn is_subset_of(&self, other: &SummaryVector) -> Option<bool> {
        if self.salt != other.salt || self.n_bits != other.n_bits || self.k != other.k {
            return None;
        }
        Some(
            self.bits
                .iter()
                .zip(other.bits.iter())
                .all(|(a, b)| a & !b == 0),
        )
    }

    /// Expected false-positive rate after `n_items` insertions,
    /// `(1 − e^(−k·n/m))^k`.
    pub fn estimated_fpr(&self, n_items: usize) -> f64 {
        let n = n_items as f64;
        let m = f64::from(self.n_bits);
        let k = f64::from(self.k);
        (1.0 - (-k * n / m).exp()).powf(k)
    }

    /// Splits the vector into chunks of at most `max_chunk_bytes` (clamped
    /// to `[MIN_CHUNK_BYTES, MAX_SUMMARY_BITS_LEN]`), indexed in order.
    pub fn to_chunks(&self, max_chunk_bytes: usize) -> Vec<SummaryChunk> {
        let size = max_chunk_bytes.clamp(MIN_CHUNK_BYTES, MAX_SUMMARY_BITS_LEN);
        let total = self.bits.len().div_ceil(size).max(1);
        // ≤ MAX_CHUNK_TOTAL by the size clamp; the fallback cannot trigger.
        let chunk_total = u16::try_from(total).unwrap_or(u16::MAX);
        self.bits
            .chunks(size)
            .enumerate()
            .map(|(i, b)| SummaryChunk {
                salt: self.salt,
                n_bits: self.n_bits,
                k: self.k,
                chunk_idx: u16::try_from(i).unwrap_or(u16::MAX),
                chunk_total,
                bits: b.to_vec(),
            })
            .collect()
    }

    /// Rebuilds a vector from its chunks, in any order; duplicates of a
    /// chunk are fine if identical. Strict about the wire parameters (no
    /// clamping): see [`SummaryError`].
    pub fn from_chunks(chunks: &[SummaryChunk]) -> Result<Self, SummaryError> {
        let first = chunks.first().ok_or(SummaryError::NoChunks)?;
        check_params(first.n_bits, first.k, first.chunk_total)?;
        let total = first.chunk_total;
        let mut slots: Vec<Option<&[u8]>> = vec![None; usize::from(total)];
        for c in chunks {
            if c.salt != first.salt
                || c.n_bits != first.n_bits
                || c.k != first.k
                || c.chunk_total != total
            {
                return Err(SummaryError::Mismatch);
            }
            let slot = slots
                .get_mut(usize::from(c.chunk_idx))
                .ok_or(SummaryError::BadParams("chunk_idx"))?;
            match slot {
                Some(prev) if *prev != c.bits.as_slice() => return Err(SummaryError::Mismatch),
                Some(_) => {}
                None => *slot = Some(c.bits.as_slice()),
            }
        }
        let have = slots.iter().filter(|s| s.is_some()).count();
        if have < usize::from(total) {
            return Err(SummaryError::Incomplete {
                have: u16::try_from(have).unwrap_or(u16::MAX),
                total,
            });
        }
        let expected = usize::try_from(first.n_bits / 8).unwrap_or(usize::MAX);
        let got: usize = slots
            .iter()
            .map(|s| s.map_or(0, <[u8]>::len))
            .fold(0usize, usize::saturating_add);
        if got != expected {
            return Err(SummaryError::BadLength { expected, got });
        }
        let mut bits = Vec::with_capacity(expected);
        for s in slots.iter().flatten() {
            bits.extend_from_slice(s);
        }
        Ok(SummaryVector {
            salt: first.salt,
            n_bits: first.n_bits,
            k: first.k,
            bits,
        })
    }
}

/// Result of [`ChunkAssembler::push`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Assembled {
    /// More chunks are needed.
    Incomplete { have: u16, total: u16 },
    /// Every chunk arrived; the vector is complete.
    Complete(SummaryVector),
}

/// Collects one vector's chunks as they arrive (any order, duplicates
/// tolerated) under a byte bound, for `dtn/sync.rs`.
#[derive(Debug, Clone, Default)]
pub struct ChunkAssembler {
    chunks: Vec<SummaryChunk>,
    have: BTreeSet<u16>,
    bytes: usize,
}

impl ChunkAssembler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Chunks held.
    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    /// The header of what is being assembled: `(salt, n_bits, k,
    /// chunk_total)`.
    pub fn header(&self) -> Option<(u32, u32, u8, u16)> {
        self.chunks
            .first()
            .map(|c| (c.salt, c.n_bits, c.k, c.chunk_total))
    }

    pub fn reset(&mut self) {
        self.chunks.clear();
        self.have.clear();
        self.bytes = 0;
    }

    /// Adds a chunk. `Err` leaves the assembler unchanged: bad parameters,
    /// a chunk of another vector ([`SummaryError::Mismatch`]), or more
    /// bytes than `n_bits / 8` ([`SummaryError::BadLength`]). When the last
    /// index arrives the vector is built and the assembler reset.
    pub fn push(&mut self, chunk: SummaryChunk) -> Result<Assembled, SummaryError> {
        check_params(chunk.n_bits, chunk.k, chunk.chunk_total)?;
        if chunk.chunk_idx >= chunk.chunk_total {
            return Err(SummaryError::BadParams("chunk_idx"));
        }
        if let Some((salt, n_bits, k, total)) = self.header()
            && (salt, n_bits, k, total) != (chunk.salt, chunk.n_bits, chunk.k, chunk.chunk_total)
        {
            return Err(SummaryError::Mismatch);
        }
        let total = chunk.chunk_total;
        if self.have.contains(&chunk.chunk_idx) {
            let same = self
                .chunks
                .iter()
                .any(|c| c.chunk_idx == chunk.chunk_idx && c.bits == chunk.bits);
            if !same {
                return Err(SummaryError::Mismatch);
            }
        } else {
            let expected = usize::try_from(chunk.n_bits / 8).unwrap_or(usize::MAX);
            let got = self.bytes.saturating_add(chunk.bits.len());
            if got > expected {
                return Err(SummaryError::BadLength { expected, got });
            }
            self.bytes = got;
            self.have.insert(chunk.chunk_idx);
            self.chunks.push(chunk);
        }
        let have = u16::try_from(self.have.len()).unwrap_or(u16::MAX);
        if have < total {
            return Ok(Assembled::Incomplete { have, total });
        }
        let built = SummaryVector::from_chunks(&self.chunks);
        self.reset();
        built.map(Assembled::Complete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{decode, encode};
    use crate::crypto::hash::sha256;
    use proptest::prelude::*;

    fn id(n: u64) -> Hash32 {
        sha256(&n.to_le_bytes())
    }

    #[test]
    fn positions_follow_the_documented_rule() {
        let v = SummaryVector::empty(7, 1024, 3);
        let x = id(1);
        let want: Vec<u32> = (0..3u8)
            .map(|i| {
                let d = sha256_concat(&[&7u32.to_le_bytes(), &[i], x.as_bytes()]);
                let mut f = [0u8; 8];
                f.copy_from_slice(&d.as_bytes()[..8]);
                u32::try_from(u64::from_le_bytes(f) % 1024).unwrap()
            })
            .collect();
        assert_eq!(v.positions(&x).collect::<Vec<_>>(), want);
        assert!(v.positions(&x).all(|p| p < 1024));
        assert_eq!(position(7, 0, &x, 0), position(7, 0, &x, 1));
        assert_ne!(
            position(7, 0, &x, 1 << 20),
            position(8, 0, &x, 1 << 20),
            "salt changes positions"
        );
    }

    #[test]
    fn no_false_negatives_and_empty_contains_nothing() {
        let ids: Vec<Hash32> = (0..500).map(id).collect();
        let (n_bits, k) = sizing(ids.len(), 0.01);
        let v = SummaryVector::build(ids.iter().copied(), 42, n_bits, k);
        assert!(ids.iter().all(|i| v.contains(i)));
        let e = SummaryVector::empty(42, n_bits, k);
        assert!(ids.iter().all(|i| !e.contains(i)));
        assert_eq!(e.count_ones(), 0);
        assert!(v.count_ones() > 0);
        assert_eq!(v.len_bytes(), usize::try_from(n_bits / 8).unwrap());
        assert_eq!(v.is_subset_of(&v), Some(true));
        assert_eq!(e.is_subset_of(&v), Some(true));
        assert_eq!(v.is_subset_of(&e), Some(false));
        assert_eq!(v.is_subset_of(&SummaryVector::empty(43, n_bits, k)), None);
    }

    #[test]
    fn sizing_is_standard_and_clamped() {
        let (m, k) = sizing(10_000, 0.01);
        assert_eq!(m, 95_856);
        assert_eq!(k, 7);
        let (m0, k0) = sizing(0, 0.01);
        assert_eq!(m0, 16);
        assert_eq!(k0, 11);
        let (mbig, _) = sizing(10_000_000, 0.001);
        assert_eq!(mbig, MAX_N_BITS);
        let (_, kbig) = sizing(1, 1e-9);
        assert_eq!(kbig, MAX_K);
        assert_eq!(sizing(100, f64::NAN), sizing(100, SUMMARY_TARGET_FPR_ASM));
        assert_eq!(sizing(100, 0.0), sizing(100, SUMMARY_TARGET_FPR_ASM));
        assert_eq!(sizing(100, 2.0), sizing(100, SUMMARY_TARGET_FPR_ASM));
        for n in [1usize, 10, 1000, 60_000] {
            let (m, k) = sizing(n, 0.01);
            assert_eq!(m % 8, 0);
            assert!((MIN_N_BITS..=MAX_N_BITS).contains(&m));
            assert!((1..=MAX_K).contains(&k));
        }
    }

    #[test]
    fn empty_clamps_parameters() {
        let v = SummaryVector::empty(1, 0, 0);
        assert_eq!((v.n_bits(), v.k()), (MIN_N_BITS, 1));
        let v = SummaryVector::empty(1, 13, 200);
        assert_eq!((v.n_bits(), v.k()), (16, MAX_K));
        let v = SummaryVector::empty(1, u32::MAX, 3);
        assert_eq!(v.n_bits(), MAX_N_BITS);
        assert_eq!(v.len_bytes(), 65_536);
        assert_eq!(v.salt(), 1);
        assert!(v.estimated_fpr(0) < 1e-12);
    }

    #[test]
    fn chunk_round_trip_any_order_and_errors() {
        let ids: Vec<Hash32> = (0..300).map(id).collect();
        let v = SummaryVector::build(ids.iter().copied(), 9, 2048, 5);
        let mut chunks = v.to_chunks(100);
        assert_eq!(chunks.len(), 3);
        assert!(chunks.iter().all(|c| c.chunk_total == 3));
        chunks.reverse();
        assert_eq!(SummaryVector::from_chunks(&chunks), Ok(v.clone()));
        // duplicates of a chunk are fine when identical
        let mut dup = chunks.clone();
        dup.push(chunks[0].clone());
        assert_eq!(SummaryVector::from_chunks(&dup), Ok(v.clone()));
        // a missing chunk is incomplete
        assert_eq!(
            SummaryVector::from_chunks(&chunks[..2]),
            Err(SummaryError::Incomplete { have: 2, total: 3 })
        );
        assert_eq!(SummaryVector::from_chunks(&[]), Err(SummaryError::NoChunks));
        // mismatched salt / n_bits / k / chunk_total
        for f in [
            |c: &mut SummaryChunk| c.salt ^= 1,
            |c: &mut SummaryChunk| c.n_bits += 8,
            |c: &mut SummaryChunk| c.k += 1,
            |c: &mut SummaryChunk| c.chunk_total += 1,
        ] {
            let mut bad = chunks.clone();
            f(&mut bad[1]);
            assert_eq!(
                SummaryVector::from_chunks(&bad),
                Err(SummaryError::Mismatch)
            );
        }
        // same index, different bits
        let mut bad = chunks.clone();
        let mut twin = chunks[2].clone();
        twin.bits[0] ^= 0xFF;
        bad.push(twin);
        assert_eq!(
            SummaryVector::from_chunks(&bad),
            Err(SummaryError::Mismatch)
        );
        // wrong total length
        let mut short = chunks.clone();
        short[0].bits.pop();
        assert!(matches!(
            SummaryVector::from_chunks(&short),
            Err(SummaryError::BadLength { expected: 256, .. })
        ));
        // one chunk when it fits
        let one = v.to_chunks(100_000);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].bits.len(), 256);
        assert_eq!(SummaryVector::from_chunks(&one), Ok(v.clone()));
        // tiny chunk size is clamped up
        let many = v.to_chunks(1);
        assert_eq!(many.len(), 256 / MIN_CHUNK_BYTES);
        assert_eq!(SummaryVector::from_chunks(&many), Ok(v));
    }

    #[test]
    fn from_chunks_rejects_bad_wire_parameters() {
        let c = |n_bits: u32, k: u8, total: u16, idx: u16| SummaryChunk {
            salt: 0,
            n_bits,
            k,
            chunk_idx: idx,
            chunk_total: total,
            bits: vec![0u8; usize::try_from(n_bits / 8).unwrap()],
        };
        assert_eq!(
            SummaryVector::from_chunks(&[c(12, 1, 1, 0)]),
            Err(SummaryError::BadParams("n_bits"))
        );
        assert_eq!(
            SummaryVector::from_chunks(&[c(0, 1, 1, 0)]),
            Err(SummaryError::BadParams("n_bits"))
        );
        assert_eq!(
            SummaryVector::from_chunks(&[c(MAX_N_BITS + 8, 1, 1, 0)]),
            Err(SummaryError::BadParams("n_bits"))
        );
        assert_eq!(
            SummaryVector::from_chunks(&[c(8, 0, 1, 0)]),
            Err(SummaryError::BadParams("k"))
        );
        assert_eq!(
            SummaryVector::from_chunks(&[c(8, MAX_K + 1, 1, 0)]),
            Err(SummaryError::BadParams("k"))
        );
        assert_eq!(
            SummaryVector::from_chunks(&[c(8, 1, 0, 0)]),
            Err(SummaryError::BadParams("chunk_total"))
        );
        assert_eq!(
            SummaryVector::from_chunks(&[c(8, 1, MAX_CHUNK_TOTAL + 1, 0)]),
            Err(SummaryError::BadParams("chunk_total"))
        );
        assert_eq!(
            SummaryVector::from_chunks(&[c(8, 1, 1, 1)]),
            Err(SummaryError::BadParams("chunk_idx"))
        );
        assert!(SummaryVector::from_chunks(&[c(8, 1, 1, 0)]).is_ok());
        assert!(SummaryVector::from_chunks(&[c(MAX_N_BITS, MAX_K, 1, 0)]).is_ok());
    }

    #[test]
    fn assembler_collects_in_any_order_and_bounds_bytes() {
        let v = SummaryVector::build((0..50).map(id), 3, 512, 4);
        let chunks = v.to_chunks(16);
        assert_eq!(chunks.len(), 4);
        let mut a = ChunkAssembler::new();
        assert!(a.is_empty());
        assert_eq!(
            a.push(chunks[2].clone()),
            Ok(Assembled::Incomplete { have: 1, total: 4 })
        );
        assert_eq!(a.header(), Some((3, 512, 4, 4)));
        assert_eq!(
            a.push(chunks[2].clone()),
            Ok(Assembled::Incomplete { have: 1, total: 4 }),
            "identical duplicate ignored"
        );
        let mut twin = chunks[2].clone();
        twin.bits[0] ^= 1;
        assert_eq!(a.push(twin), Err(SummaryError::Mismatch));
        let mut other = chunks[0].clone();
        other.salt = 99;
        assert_eq!(a.push(other), Err(SummaryError::Mismatch));
        assert_eq!(a.len(), 1);
        assert_eq!(
            a.push(chunks[0].clone()),
            Ok(Assembled::Incomplete { have: 2, total: 4 })
        );
        assert_eq!(
            a.push(chunks[3].clone()),
            Ok(Assembled::Incomplete { have: 3, total: 4 })
        );
        assert_eq!(a.push(chunks[1].clone()), Ok(Assembled::Complete(v)));
        assert!(a.is_empty(), "reset after completion");
        // byte bound: a chunk claiming n_bits=8 but carrying 64 bytes
        let fat = SummaryChunk {
            salt: 0,
            n_bits: 8,
            k: 1,
            chunk_idx: 0,
            chunk_total: 2,
            bits: vec![0u8; 64],
        };
        assert_eq!(
            a.push(fat),
            Err(SummaryError::BadLength {
                expected: 1,
                got: 64
            })
        );
        assert!(a.is_empty());
        // a bad index is refused up front
        let bad = SummaryChunk {
            salt: 0,
            n_bits: 8,
            k: 1,
            chunk_idx: 5,
            chunk_total: 2,
            bits: vec![0u8; 1],
        };
        assert_eq!(a.push(bad), Err(SummaryError::BadParams("chunk_idx")));
        a.reset();
        assert!(a.is_empty());
    }

    proptest! {
        /// `summary_decode` fuzz target on stable (DESIGN §10): any bytes
        /// through the frame decoder and then the vector builder.
        #[test]
        fn decode_then_assemble_never_panics(raw in proptest::collection::vec(any::<u8>(), 0..600)) {
            if let Ok(chunk) = decode::<SummaryChunk>(&raw) {
                let _ = SummaryVector::from_chunks(core::slice::from_ref(&chunk));
                let mut a = ChunkAssembler::new();
                let _ = a.push(chunk);
            }
        }

        /// Arbitrary chunk structs (any field values) never panic the
        /// builder, and a successful build has the chunk's parameters.
        #[test]
        fn from_arbitrary_chunks_never_panics(
            chunks in proptest::collection::vec(
                (any::<u32>(), 0u32..70_000, any::<u8>(), any::<u16>(), 0u16..8, proptest::collection::vec(any::<u8>(), 0..300)),
                0..6,
            )
        ) {
            let chunks: Vec<SummaryChunk> = chunks
                .into_iter()
                .map(|(salt, n_bits, k, chunk_idx, chunk_total, bits)| SummaryChunk { salt, n_bits, k, chunk_idx, chunk_total, bits })
                .collect();
            if let Ok(v) = SummaryVector::from_chunks(&chunks) {
                prop_assert_eq!(v.n_bits(), chunks[0].n_bits);
                prop_assert_eq!(v.k(), chunks[0].k);
                prop_assert_eq!(v.len_bytes(), (chunks[0].n_bits / 8) as usize);
            }
            let mut a = ChunkAssembler::new();
            for c in chunks {
                let _ = a.push(c);
            }
        }

        /// Chunking round-trips under any chunk size and any order.
        #[test]
        fn chunks_round_trip(
            n in 0usize..200,
            salt in any::<u32>(),
            n_bits in 0u32..20_000,
            k in any::<u8>(),
            chunk_bytes in 0usize..5000,
            seed in any::<u64>(),
        ) {
            let v = SummaryVector::build((0..n as u64).map(|i| id(i ^ seed)), salt, n_bits, k);
            let mut chunks = v.to_chunks(chunk_bytes);
            let enc: Vec<Vec<u8>> = chunks.iter().map(encode).collect();
            let dec: Vec<SummaryChunk> = enc.iter().map(|b| decode::<SummaryChunk>(b).unwrap()).collect();
            prop_assert_eq!(&dec, &chunks);
            // deterministic shuffle from the seed
            let mut s = seed;
            for i in (1..chunks.len()).rev() {
                s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                let j = (s >> 33) as usize % (i + 1);
                chunks.swap(i, j);
            }
            prop_assert_eq!(SummaryVector::from_chunks(&chunks), Ok(v.clone()));
            for i in 0..n as u64 {
                prop_assert!(v.contains(&id(i ^ seed)));
            }
        }
    }
}
