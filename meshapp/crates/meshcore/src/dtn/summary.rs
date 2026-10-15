//! Bloom summary vectors (docs/DESIGN.md §1 row `dtn/summary.rs`; §2.3
//! `Summary chunk = {salt u32, n_bits u32, k u8, chunk_idx u16, chunk_total
//! u16, bits}`; DECISIONS D10 (4) "anti-entropy by summary-vector
//! exchange"; §3 row (1,2,3,4,10) (iv) "summary vectors carry the hashes
//! of rejected items so they are not re-offered"; BRIEF_v2 §5.7).
//!
//! A [`SummaryVector`] is a salted Bloom filter over `msg_id`s (DESIGN §2.2
//! "summary vectors … key on the derived value"). The chunk format on the
//! wire is unchanged from version 1; the *position rule* is
//! [`SUMMARY_VERSION`] 2, which both ends must compute identically:
//!
//! ```text
//! h1 = LE64(id[0..8]),  h2 = LE64(id[8..16])                 (per cell, cached: IdWords)
//! c1 = mix64(salt ⊕ 0x9E37_79B9_7F4A_7C15)                   (per encounter: SaltMix)
//! c2 = mix64(c1 ⊕ 0xD1B5_4A32_D192_ED03)
//! g1 = mix64(h1 ⊕ c1),  g2 = mix64(h2 ⊕ c2)                  (per cell per encounter: Probe)
//! position_i(id) = ((g1 + i·g2) mod 2^64) · n_bits >> 64,   i in 0..k
//! mix64(z): z ^= z >> 30; z *= 0xBF58_476D_1CE4_E5B9; z ^= z >> 27;
//!           z *= 0x94D0_49BB_1331_11EB; z ^= z >> 31        (SplitMix64 finaliser, mod 2^64)
//! bit b lives in bits[b / 8], mask 1 << (b % 8)
//! ```
//!
//! Why this rule (the version-1 rule was `SHA256(salt ‖ i ‖ id)` per
//! position): a `msg_id` is already SHA-256 output, so its first sixteen
//! bytes are two uniform 64-bit words. Those are the only hash-derived
//! input; the salt is folded in with integer mixing, and the `k` positions
//! come from double hashing (Kirsch–Mitzenmacher: `g1 + i·g2`) with the
//! range reduction `x · n_bits >> 64` (uniform for uniform `x`, one
//! multiply, no division). Summarising a store therefore costs **no hash
//! function at all** beyond the one SHA-256 that derived each cell's id
//! when it was stored, and testing an id against a peer's vector is a few
//! dozen integer operations. Platform independence: every step is fixed
//! width `mod 2^64` arithmetic, so no float or endianness question arises.
//!
//! * `salt` makes the positions of one encounter's vector independent of
//!   the next's (DESIGN §10 risk 4 "salt rotation per encounter"): a false
//!   positive under one salt does not repeat under the next, so rarely-met
//!   peers do not accumulate misses. How often a store rotates its salt is
//!   decided by [`crate::dtn::store::CellStore::summary_salt_for`] (the
//!   privacy and cost trade-off is documented there); the rule here only
//!   needs the salt that travels in the chunks.
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
//! * [`SummaryIndex`] is the incremental form a [`crate::dtn::store::CellStore`]
//!   owns: the cached words of every held cell plus a *counting* filter
//!   under the store's current salt, updated in `O(k)` on put and evict
//!   (a counter per bit, so removal is exact and the bit vector never
//!   carries a false negative). A salt change or a capacity change
//!   re-derives the filter from the cached words in `O(n·k)` integer
//!   operations; an encounter that uses the store's salt sends the
//!   maintained bits as they are (an `O(n_bits/8)` copy).
//!
//! No false negatives: an id that was inserted is always reported as
//! contained. Everything is a pure function of its inputs.

use std::collections::BTreeSet;

use crate::crypto::hash::Hash32;
use crate::wire::frame::{MAX_SUMMARY_BITS_LEN, SummaryChunk};

/// Version of the position rule in the module docs (not carried on the
/// wire: the chunk layout did not change; peers must agree out of band,
/// which `Hello.proto` covers, DESIGN §2.3).
pub const SUMMARY_VERSION: u8 = 2;
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

// ---- the position rule -------------------------------------------------------

/// `mix64` of the module docs: the SplitMix64 finaliser, a bijection on
/// `u64` with good avalanche, so `mix64(h ⊕ c)` behaves like a fresh
/// uniform word for every salt constant `c`.
pub fn mix64(mut z: u64) -> u64 {
    z ^= z >> 30;
    z = z.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z ^= z >> 27;
    z = z.wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    z
}

/// `x · m >> 64`: maps a uniform 64-bit `x` to `[0, m)` with one widening
/// multiply (Lemire's range reduction). The result is below `m`, so the
/// narrowing cannot fail; `m = 0` yields 0.
fn range(x: u64, m: u32) -> u32 {
    let wide = u128::from(x).wrapping_mul(u128::from(m)) >> 64;
    u32::try_from(wide).unwrap_or(0)
}

/// The two 64-bit words of an id, `h1 = LE64(id[0..8])`, `h2 =
/// LE64(id[8..16])` (module docs). They are the only input a vector needs
/// about a cell, so a store caches them once per cell ([`SummaryIndex`])
/// and never touches the hash function again for summary purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IdWords {
    pub h1: u64,
    pub h2: u64,
}

impl IdWords {
    pub fn of(id: &Hash32) -> Self {
        let b = id.as_bytes();
        let mut w1 = [0u8; 8];
        let mut w2 = [0u8; 8];
        w1.copy_from_slice(&b[..8]);
        w2.copy_from_slice(&b[8..16]);
        IdWords {
            h1: u64::from_le_bytes(w1),
            h2: u64::from_le_bytes(w2),
        }
    }
}

/// The salt-derived constants `(c1, c2)` of one vector (module docs),
/// computed once per vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SaltMix {
    c1: u64,
    c2: u64,
}

impl SaltMix {
    pub fn new(salt: u32) -> Self {
        let c1 = mix64(u64::from(salt) ^ 0x9E37_79B9_7F4A_7C15);
        let c2 = mix64(c1 ^ 0xD1B5_4A32_D192_ED03);
        SaltMix { c1, c2 }
    }

    /// `(g1, g2)` of an id under this salt.
    pub fn probe(&self, w: &IdWords) -> Probe {
        Probe {
            g1: mix64(w.h1 ^ self.c1),
            g2: mix64(w.h2 ^ self.c2),
        }
    }
}

/// The mixed words of one id under one salt; `position(i, n_bits)` gives
/// the `i`-th bit position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Probe {
    g1: u64,
    g2: u64,
}

impl Probe {
    /// `position_i` of the module docs. `n_bits` of 0 is treated as 1.
    pub fn position(&self, i: u8, n_bits: u32) -> u32 {
        let x = self.g1.wrapping_add(self.g2.wrapping_mul(u64::from(i)));
        range(x, n_bits.max(1))
    }

    /// Writes the first `k` positions into `out` and returns how many were
    /// written (`min(k, out.len())`); no allocation.
    fn positions_into(&self, k: u8, n_bits: u32, out: &mut [u32; MAX_K as usize]) -> usize {
        let k = usize::from(k.min(MAX_K));
        for (i, slot) in out.iter_mut().enumerate().take(k) {
            *slot = self.position(u8::try_from(i).unwrap_or(u8::MAX), n_bits);
        }
        k
    }
}

/// `position_i(id)` of the module docs. `n_bits` of 0 is treated as 1.
pub fn position(salt: u32, i: u8, id: &Hash32, n_bits: u32) -> u32 {
    SaltMix::new(salt)
        .probe(&IdWords::of(id))
        .position(i, n_bits)
}

fn bit_is_set(bits: &[u8], pos: u32) -> bool {
    let byte = usize::try_from(pos / 8).unwrap_or(usize::MAX);
    bits.get(byte).is_some_and(|b| b & (1u8 << (pos % 8)) != 0)
}

fn set_bit(bits: &mut [u8], pos: u32) {
    let byte = usize::try_from(pos / 8).unwrap_or(usize::MAX);
    if let Some(b) = bits.get_mut(byte) {
        *b |= 1u8 << (pos % 8);
    }
}

fn clear_bit(bits: &mut [u8], pos: u32) {
    let byte = usize::try_from(pos / 8).unwrap_or(usize::MAX);
    if let Some(b) = bits.get_mut(byte) {
        *b &= !(1u8 << (pos % 8));
    }
}

// ---- the vector ----------------------------------------------------------------

/// A salted Bloom filter over `msg_id`s. See the module docs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SummaryVector {
    salt: u32,
    n_bits: u32,
    k: u8,
    mix: SaltMix,
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
            mix: SaltMix::new(salt),
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

    /// A vector containing every id whose words are in `words`.
    pub fn build_words<'a>(
        words: impl IntoIterator<Item = &'a IdWords>,
        salt: u32,
        n_bits: u32,
        k: u8,
    ) -> Self {
        let mut v = Self::empty(salt, n_bits, k);
        for w in words {
            v.insert_words(w);
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

    /// The salt constants of this vector, for callers that probe many ids.
    pub fn mix(&self) -> &SaltMix {
        &self.mix
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
    pub fn positions(&self, id: &Hash32) -> impl Iterator<Item = u32> + use<> {
        self.positions_of(&IdWords::of(id))
    }

    /// The `k` positions of an id given its words.
    pub fn positions_of(&self, w: &IdWords) -> impl Iterator<Item = u32> + use<> {
        let probe = self.mix.probe(w);
        let (k, n_bits) = (self.k, self.n_bits);
        (0..k).map(move |i| probe.position(i, n_bits))
    }

    pub fn insert(&mut self, id: &Hash32) {
        self.insert_words(&IdWords::of(id));
    }

    /// Inserts an id given its cached words: `k` integer probes, no
    /// allocation.
    pub fn insert_words(&mut self, w: &IdWords) {
        let probe = self.mix.probe(w);
        for i in 0..self.k {
            set_bit(&mut self.bits, probe.position(i, self.n_bits));
        }
    }

    /// `true` if every position of `id` is set: always for an inserted id,
    /// with probability about the sized false-positive rate otherwise.
    pub fn contains(&self, id: &Hash32) -> bool {
        self.contains_words(&IdWords::of(id))
    }

    /// [`contains`](Self::contains) given the id's cached words.
    pub fn contains_words(&self, w: &IdWords) -> bool {
        let probe = self.mix.probe(w);
        (0..self.k).all(|i| bit_is_set(&self.bits, probe.position(i, self.n_bits)))
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
            mix: SaltMix::new(first.salt),
            bits,
        })
    }
}

// ---- the incremental index -----------------------------------------------------

/// The incremental summary of a store (module docs; DESIGN §7 "summary
/// vectors"; D4 MERGE): one slot per held cell holding its cached
/// [`IdWords`], and a counting Bloom filter under the index's current
/// salt that `insert` and `remove` keep exact in `O(k)`.
///
/// **Slots** are dense indices the store hands to per-link bookkeeping
/// (`dtn/sync.rs` marks "already offered" and "withheld" per slot instead
/// of hashing ids). A removed slot is reused; every (re)use bumps the
/// slot's *generation*, so a mark made for an earlier occupant never
/// applies to a later one.
///
/// **Capacity**: the filter is sized for `capacity ≥ len` with the index's
/// target false-positive rate, so at any load up to the capacity the rate
/// is at or below target. It grows by an eighth ([`Self::grow_step`]) when
/// the length exceeds it and shrinks when the length drops below a quarter
/// of it; each change re-derives the filter in `O(n·k)`, amortised `O(k)`
/// per insert. The wire vector is therefore at most `~12 %` larger than
/// the exact sizing of a growing store (and smaller than `2.25×` after a
/// mass deletion).
///
/// **Counters** are `u8` and saturate; a saturated counter is never
/// decremented, so its bit stays set (a standard counting-filter rule:
/// never a false negative, at worst a stale bit). At the designed load
/// (`k·n/m ≈ 0.7` per bit) saturation does not happen in practice.
#[derive(Debug, Clone)]
pub struct SummaryIndex {
    target_fpr: f64,
    words: Vec<IdWords>,
    live: Vec<bool>,
    gens: Vec<u32>,
    free: Vec<usize>,
    len: usize,
    capacity: usize,
    vector: SummaryVector,
    counts: Vec<u8>,
}

impl SummaryIndex {
    /// Smallest capacity the filter is sized for.
    pub const MIN_CAPACITY: usize = 64;

    /// An empty index under `salt` with `target_fpr` (an out-of-range rate
    /// falls back to [`SUMMARY_TARGET_FPR_ASM`], as in [`sizing`]).
    pub fn new(salt: u32, target_fpr: f64) -> Self {
        let mut idx = SummaryIndex {
            target_fpr,
            words: Vec::new(),
            live: Vec::new(),
            gens: Vec::new(),
            free: Vec::new(),
            len: 0,
            capacity: 0,
            vector: SummaryVector::empty(salt, MIN_N_BITS, 1),
            counts: Vec::new(),
        };
        idx.rebuild(salt, Self::MIN_CAPACITY);
        idx
    }

    /// Live cells.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Slots in the table (live and free).
    pub fn slots(&self) -> usize {
        self.words.len()
    }

    pub fn salt(&self) -> u32 {
        self.vector.salt
    }

    pub fn target_fpr(&self) -> f64 {
        self.target_fpr
    }

    /// The item count the filter is currently sized for (`≥ len`).
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// The maintained filter: contains every live cell under
    /// [`salt`](Self::salt).
    pub fn vector(&self) -> &SummaryVector {
        &self.vector
    }

    /// The cached words of a live slot.
    pub fn words(&self, slot: usize) -> Option<&IdWords> {
        if self.live.get(slot).copied().unwrap_or(false) {
            self.words.get(slot)
        } else {
            None
        }
    }

    /// The generation of a slot (0 for a slot that never held a cell).
    pub fn generation(&self, slot: usize) -> u32 {
        self.gens.get(slot).copied().unwrap_or(0)
    }

    pub fn is_live(&self, slot: usize) -> bool {
        self.live.get(slot).copied().unwrap_or(false)
    }

    /// Every live slot with its words, in slot order.
    pub fn iter_live(&self) -> impl Iterator<Item = (usize, &IdWords)> + '_ {
        self.words
            .iter()
            .enumerate()
            .filter(|(slot, _)| self.live.get(*slot).copied().unwrap_or(false))
    }

    /// Adds a cell and returns its slot. `O(k)` unless the capacity is
    /// exceeded (then an `O(n·k)` re-derivation).
    pub fn insert(&mut self, id: &Hash32) -> usize {
        let w = IdWords::of(id);
        let slot = match self.free.pop() {
            Some(s) => {
                if let (Some(ws), Some(l), Some(g)) = (
                    self.words.get_mut(s),
                    self.live.get_mut(s),
                    self.gens.get_mut(s),
                ) {
                    *ws = w;
                    *l = true;
                    *g = g.wrapping_add(1).max(1);
                }
                s
            }
            None => {
                self.words.push(w);
                self.live.push(true);
                self.gens.push(1);
                self.words.len().saturating_sub(1)
            }
        };
        self.len = self.len.saturating_add(1);
        if self.len > self.capacity {
            let salt = self.vector.salt;
            self.rebuild(salt, Self::grow_step(self.len));
        } else {
            self.add_to_filter(&w);
        }
        slot
    }

    /// Removes the cell in `slot`; `false` if the slot was not live.
    pub fn remove(&mut self, slot: usize) -> bool {
        let Some(l) = self.live.get_mut(slot) else {
            return false;
        };
        if !*l {
            return false;
        }
        *l = false;
        self.len = self.len.saturating_sub(1);
        self.free.push(slot);
        let w = self.words.get(slot).copied().unwrap_or_default();
        if self.capacity > Self::MIN_CAPACITY && self.len < self.capacity / 4 {
            let salt = self.vector.salt;
            self.rebuild(salt, Self::grow_step(self.len));
        } else {
            self.remove_from_filter(&w);
        }
        true
    }

    /// Re-derives the filter under a new salt (`O(n·k)` integer
    /// operations, no hashing); the capacity is kept.
    pub fn rotate(&mut self, salt: u32) {
        let capacity = self.capacity;
        self.rebuild(salt, capacity);
    }

    /// The capacity chosen for `n` live cells: `n + n/8 + 1`, at least
    /// [`Self::MIN_CAPACITY`].
    pub fn grow_step(n: usize) -> usize {
        n.saturating_add(n / 8)
            .saturating_add(1)
            .max(Self::MIN_CAPACITY)
    }

    fn rebuild(&mut self, salt: u32, capacity: usize) {
        let (n_bits, k) = sizing(capacity, self.target_fpr);
        self.capacity = capacity;
        self.vector = SummaryVector::empty(salt, n_bits, k);
        self.counts = vec![0u8; usize::try_from(self.vector.n_bits).unwrap_or(0)];
        let mut pos = [0u32; MAX_K as usize];
        for slot in 0..self.words.len() {
            if !self.live.get(slot).copied().unwrap_or(false) {
                continue;
            }
            let w = self.words.get(slot).copied().unwrap_or_default();
            let n = self.vector.mix.probe(&w).positions_into(
                self.vector.k,
                self.vector.n_bits,
                &mut pos,
            );
            for &p in pos.iter().take(n) {
                Self::count_up(&mut self.counts, &mut self.vector.bits, p);
            }
        }
    }

    fn add_to_filter(&mut self, w: &IdWords) {
        let mut pos = [0u32; MAX_K as usize];
        let n =
            self.vector
                .mix
                .probe(w)
                .positions_into(self.vector.k, self.vector.n_bits, &mut pos);
        for &p in pos.iter().take(n) {
            Self::count_up(&mut self.counts, &mut self.vector.bits, p);
        }
    }

    fn remove_from_filter(&mut self, w: &IdWords) {
        let mut pos = [0u32; MAX_K as usize];
        let n =
            self.vector
                .mix
                .probe(w)
                .positions_into(self.vector.k, self.vector.n_bits, &mut pos);
        for &p in pos.iter().take(n) {
            Self::count_down(&mut self.counts, &mut self.vector.bits, p);
        }
    }

    fn count_up(counts: &mut [u8], bits: &mut [u8], p: u32) {
        let i = usize::try_from(p).unwrap_or(usize::MAX);
        if let Some(c) = counts.get_mut(i) {
            *c = c.saturating_add(1);
            set_bit(bits, p);
        }
    }

    fn count_down(counts: &mut [u8], bits: &mut [u8], p: u32) {
        let i = usize::try_from(p).unwrap_or(usize::MAX);
        if let Some(c) = counts.get_mut(i) {
            match *c {
                // a saturated counter is never decremented: its bit stays
                0 | u8::MAX => {}
                1 => {
                    *c = 0;
                    clear_bit(bits, p);
                }
                n => *c = n - 1,
            }
        }
    }
}

// ---- chunk assembly ------------------------------------------------------------

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

    /// Whether `chunk` belongs to the vector being assembled (or nothing
    /// is being assembled yet). A caller that wants to restart on a foreign
    /// chunk asks this before [`push`](Self::push), which consumes the
    /// chunk.
    pub fn accepts(&self, chunk: &SummaryChunk) -> bool {
        match self.header() {
            Some(h) => h == (chunk.salt, chunk.n_bits, chunk.k, chunk.chunk_total),
            None => true,
        }
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
        if !self.accepts(&chunk) {
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
        let b = x.as_bytes();
        let h1 = u64::from_le_bytes(b[..8].try_into().unwrap());
        let h2 = u64::from_le_bytes(b[8..16].try_into().unwrap());
        let c1 = mix64(7u64 ^ 0x9E37_79B9_7F4A_7C15);
        let c2 = mix64(c1 ^ 0xD1B5_4A32_D192_ED03);
        let g1 = mix64(h1 ^ c1);
        let g2 = mix64(h2 ^ c2);
        let want: Vec<u32> = (0..3u64)
            .map(|i| {
                let x = g1.wrapping_add(g2.wrapping_mul(i));
                u32::try_from((u128::from(x) * 1024u128) >> 64).unwrap()
            })
            .collect();
        assert_eq!(v.positions(&x).collect::<Vec<_>>(), want);
        assert!(v.positions(&x).all(|p| p < 1024));
        assert_eq!(position(7, 0, &x, 0), position(7, 0, &x, 1));
        assert_eq!(position(7, 0, &x, 1), 0);
        assert_ne!(
            position(7, 0, &x, 1 << 20),
            position(8, 0, &x, 1 << 20),
            "salt changes positions"
        );
        assert_eq!(IdWords::of(&x), IdWords { h1, h2 });
        assert_eq!(SUMMARY_VERSION, 2);
        // mix64 is a bijection in spirit: distinct small inputs spread out
        let outs: BTreeSet<u64> = (0..1000u64).map(mix64).collect();
        assert_eq!(outs.len(), 1000);
        assert_eq!(mix64(0), 0, "SplitMix64's finaliser fixes zero");
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
        // the words API agrees with the id API
        let words: Vec<IdWords> = ids.iter().map(IdWords::of).collect();
        let w = SummaryVector::build_words(words.iter(), 42, n_bits, k);
        assert_eq!(w, v);
        assert!(words.iter().all(|x| v.contains_words(x)));
        assert_eq!(v.mix(), &SaltMix::new(42));
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
        assert!(a.accepts(&chunks[2]));
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
        assert!(a.accepts(&twin), "same header");
        assert_eq!(a.push(twin), Err(SummaryError::Mismatch));
        let mut other = chunks[0].clone();
        other.salt = 99;
        assert!(!a.accepts(&other));
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

    /// The counting index matches a vector built from scratch after every
    /// insert, remove and rotation, and never reports a live id missing.
    #[test]
    fn index_tracks_inserts_removes_and_rotations_exactly() {
        let mut idx = SummaryIndex::new(5, 0.01);
        assert_eq!(idx.salt(), 5);
        assert_eq!(idx.capacity(), SummaryIndex::MIN_CAPACITY);
        assert!(idx.is_empty());
        let ids: Vec<Hash32> = (0..300).map(id).collect();
        let mut slots = Vec::new();
        for x in &ids {
            slots.push(idx.insert(x));
        }
        assert_eq!(idx.len(), 300);
        assert!(idx.capacity() >= 300);
        assert_eq!(idx.slots(), 300);
        assert!(ids.iter().all(|x| idx.vector().contains(x)));
        let check = |idx: &SummaryIndex, live: &[Hash32]| {
            let v = idx.vector();
            let fresh = SummaryVector::build(live.iter().copied(), v.salt(), v.n_bits(), v.k());
            assert_eq!(v, &fresh, "counters keep the bits exact");
            assert!(live.iter().all(|x| v.contains(x)));
        };
        check(&idx, &ids);
        // remove the odd ones: bits of the removed ids go unless shared
        for (i, s) in slots.iter().enumerate() {
            if i % 2 == 1 {
                assert!(idx.remove(*s));
                assert!(!idx.remove(*s), "a second remove is a no-op");
            }
        }
        let live: Vec<Hash32> = ids.iter().step_by(2).copied().collect();
        assert_eq!(idx.len(), 150);
        check(&idx, &live);
        assert_eq!(idx.iter_live().count(), 150);
        assert!(idx.words(slots[0]).is_some());
        assert!(idx.words(slots[1]).is_none());
        assert!(idx.is_live(slots[0]) && !idx.is_live(slots[1]));
        // rotation re-derives under the new salt
        idx.rotate(77);
        assert_eq!(idx.salt(), 77);
        check(&idx, &live);
        // reuse bumps the generation; a free slot comes back first
        let g = idx.generation(slots[299]);
        assert_eq!(g, 1);
        let s = idx.insert(&id(1000));
        assert_eq!(s, slots[299], "LIFO free list");
        assert_eq!(idx.generation(s), 2);
        assert_eq!(idx.generation(usize::MAX), 0);
        let mut live2 = live.clone();
        live2.push(id(1000));
        check(&idx, &live2);
        // shrink: dropping below a quarter of the capacity re-sizes down
        let cap = idx.capacity();
        for s in slots.iter().step_by(2).skip(5) {
            idx.remove(*s);
        }
        assert!(idx.capacity() < cap);
        assert!(idx.capacity() >= idx.len());
        let left: Vec<Hash32> = idx
            .iter_live()
            .map(|(s, _)| if s == slots[299] { id(1000) } else { ids[s] })
            .collect();
        assert_eq!(left.len(), idx.len());
        check(&idx, &left);
        assert_eq!(SummaryIndex::grow_step(0), SummaryIndex::MIN_CAPACITY);
        assert_eq!(SummaryIndex::grow_step(800), 901);
        assert_eq!(idx.target_fpr(), 0.01);
    }

    /// At every load up to the capacity the maintained filter's rate is at
    /// or below the target (the sizing rounds `k`, so the exact-`n` vector
    /// sits at `1.003 %`; the index is sized for `n + n/8` and so has
    /// room, DESIGN §10 risk 4).
    #[test]
    fn index_false_positive_rate_is_at_most_one_percent_at_full_load() {
        let n = 10_000u64;
        let mut idx = SummaryIndex::new(0xC0FF_EE00, 0.01);
        for i in 0..n {
            idx.insert(&id(i));
        }
        let probes = 50_000u64;
        let fp = (n..n + probes)
            .filter(|i| idx.vector().contains(&id(*i)))
            .count();
        let fpr = fp as f64 / probes as f64;
        assert!(fpr <= 0.01, "index fpr {fpr} at n = {n}");
        assert!(idx.vector().estimated_fpr(idx.len()) <= 0.01);
        assert!(idx.vector().estimated_fpr(idx.capacity()) <= 0.0101);
        // the exact-sized vector at the designed load (rounding of k)
        let (n_bits, k) = sizing(n as usize, 0.01);
        let v = SummaryVector::build((0..n).map(id), 0xC0FF_EE00, n_bits, k);
        let fp = (n..n + probes).filter(|i| v.contains(&id(*i))).count();
        let fpr = fp as f64 / probes as f64;
        assert!(fpr <= 0.011, "exact-sized fpr {fpr}");
        assert!(v.estimated_fpr(n as usize) <= 0.0101);
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
                prop_assert_eq!(v.mix(), &SaltMix::new(chunks[0].salt));
            }
            let mut a = ChunkAssembler::new();
            for c in chunks {
                let _ = a.push(c);
            }
        }

        /// The position rule never panics and stays in range for any salt,
        /// id bytes, size and index.
        #[test]
        fn positions_are_in_range_for_any_input(
            salt in any::<u32>(),
            raw in any::<[u8; 32]>(),
            n_bits in any::<u32>(),
            i in any::<u8>(),
        ) {
            let id = Hash32::from_bytes(raw);
            let p = position(salt, i, &id, n_bits);
            prop_assert!(u64::from(p) < u64::from(n_bits.max(1)));
            let v = SummaryVector::empty(salt, n_bits, i);
            for q in v.positions(&id) {
                prop_assert!(q < v.n_bits());
            }
            prop_assert_eq!(IdWords::of(&id), IdWords::of(&Hash32::from_bytes(raw)));
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

        /// Any sequence of inserts, removes and rotations leaves the index
        /// equal to a vector built from its live ids.
        #[test]
        fn index_is_exact_under_random_operations(
            ops in proptest::collection::vec((0u8..3, 0u64..120, any::<u32>()), 0..300),
        ) {
            let mut idx = SummaryIndex::new(1, 0.01);
            let mut held: std::collections::BTreeMap<u64, usize> = std::collections::BTreeMap::new();
            for (op, n, salt) in ops {
                match op {
                    0 => {
                        if let std::collections::btree_map::Entry::Vacant(v) = held.entry(n) {
                            v.insert(idx.insert(&id(n)));
                        }
                    }
                    1 => {
                        if let Some(s) = held.remove(&n) {
                            prop_assert!(idx.remove(s));
                        }
                    }
                    _ => idx.rotate(salt),
                }
                prop_assert_eq!(idx.len(), held.len());
                prop_assert!(idx.capacity() >= idx.len());
            }
            let v = idx.vector();
            let fresh = SummaryVector::build(held.keys().map(|n| id(*n)), v.salt(), v.n_bits(), v.k());
            prop_assert_eq!(v, &fresh);
            for (n, s) in &held {
                prop_assert_eq!(idx.words(*s), Some(&IdWords::of(&id(*n))));
            }
        }
    }
}
