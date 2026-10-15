//! Per-chain recognition: tag windows and per-counter cell keys
//! (docs/DESIGN.md §1 row `wire/recog`, §2.2 `tag` and body key;
//! DECISIONS.md §3 row (3,4,6,10); BRIEF_v2 §5.5 "a tag = PRF(tag key,
//! counter), fresh for every message").
//!
//! A *chain* is one stream of cells that share a tag key: a group member's
//! sender chain, or one direction of a 1:1 Double Ratchet. The caller names
//! each chain with a local [`ChainKey`]. A chain's keys change at an
//! *epoch* boundary (an SKDM rotation for groups, a DH step for 1:1; §3 row
//! (3,4,6,10) "re-seeded at each DH step"), and a receiver keeps the last
//! [`MAX_EPOCHS`] epochs of every chain so that cells delayed across a
//! rotation still match (D6 pick B (2) keeps three live envelope keys for
//! the same reason).
//!
//! **Derivation** from the 32-byte epoch seed the chain delivers (inside the
//! SKDM for groups, from the ratchet root for 1:1):
//!
//! * `k_tag = HKDF-SHA256(ikm = seed, salt = "", info = "meshapp.v1.recog.tag")`
//! * `k_hdr = HKDF-SHA256(ikm = seed, salt = "", info = "meshapp.v1.recog.hdr")`
//! * `tag(ctr) = prf16(k_tag, "tag", ctr)` (DESIGN §2.2 offset 4)
//! * `cell_key(ctr) = HKDF-SHA256(ikm = k_hdr, salt = "", info =
//!   "meshapp.v1.cell" ‖ LE64(ctr))` (DESIGN §2.2 offset 20). This is the
//!   same bytes as [`crate::wire::cell::cell_key`]; the tests pin both to one
//!   fixed vector.
//!
//! One counter per cell (DESIGN §2.2 "one cell = one recognition counter"),
//! so every tag and every body key is fresh and a relay sees only 16
//! uniformly random bytes per cell.
//!
//! **Receiver window** (§3 row (3,4,6,10) "receivers keep a 50-tag window per
//! chain"). For every registered `(chain, epoch)` the [`TagTable`]
//! precomputes the tags of [`WINDOW`] consecutive counters starting at
//! `base`, the next expected counter, and keeps all of them in one map
//! `tag → (chain, epoch, ctr)`, so recognising a cell is one lookup whatever
//! the number of chains. [`TagTable::consume`] marks a counter used, removes
//! its tag and slides the window forward:
//!
//! 1. `base` is raised to at least `ctr − (WINDOW − 1 − LOOKAHEAD)`, so that
//!    at least [`LOOKAHEAD`] counters beyond the newest hit are always
//!    precomputed even when an earlier counter never arrives (a cell whose
//!    TTL ran out, or one evicted by a relay). Counters that fall below
//!    `base` this way are given up: a cell that late is no longer
//!    recognisable, the same price the ratchet's skipped-key cap charges;
//! 2. `base` then advances past the run of consecutive used counters at the
//!    bottom of the window, so the window never starts on a consumed
//!    counter and in-order delivery always keeps the full `WINDOW` of
//!    lookahead.
//!
//! Used counters are a bitmap relative to `base`, and a consumed tag is
//! removed from the map, so a replayed cell never hits twice and the state
//! per epoch is at most `WINDOW` map entries plus two words. While a hole
//! pins `base`, consumed counters above it leave the map without the
//! window moving, so the map holds between `WINDOW − KEEP_BEHIND` and
//! `WINDOW` tags for the epoch.
//!
//! **Memory bound.** The map holds at most `chains × MAX_EPOCHS × WINDOW`
//! entries (a 16-byte tag plus a 24-byte [`Hit`], about 64 B each with tree
//! overhead): 6 000 entries, about 400 kB, for a 40-member group with every
//! member on three epochs; 150 000 entries, about 10 MB, for 1 000 chains.
//! The caller bounds the number of chains by its roster and contact list;
//! this table never grows on its own and never allocates on a lookup.
//!
//! The sending side ([`TagTable::register_sender`], [`TagTable::sender_next`])
//! keeps one monotone counter per chain and epoch; the caller persists it
//! and re-registers after a restart, because reusing a counter would reuse a
//! cell key.
//!
//! Nothing here reads a clock (BRIEF_v2 §4 "no timestamps decide anything").

use core::fmt;
use std::collections::{BTreeMap, VecDeque};

use crate::crypto::aead::Key32;
use crate::crypto::hash::{hkdf32, prf16};
use crate::wire::cell::TAG_LEN;

/// Counters precomputed per `(chain, epoch)` (DECISIONS §3 row (3,4,6,10)
/// "a 50-tag window per chain").
pub const WINDOW: usize = 50;
/// Minimum number of precomputed counters beyond the newest consumed one.
/// After a hit at `ctr` the window covers at least `ctr + 1 ..= ctr +
/// LOOKAHEAD`, so a chain with a permanent hole still makes progress; the
/// remaining [`KEEP_BEHIND`] slots keep late cells recognisable.
pub const LOOKAHEAD: usize = WINDOW / 2;
/// How far behind the newest hit the window may start (rule 1 above):
/// `WINDOW − 1 − LOOKAHEAD`.
pub const KEEP_BEHIND: usize = WINDOW - 1 - LOOKAHEAD;
/// Epochs retained per chain: the current one and the two before it.
pub const MAX_EPOCHS: usize = 3;

/// HKDF `info` of the tag key.
pub const TAG_KEY_INFO: &[u8] = b"meshapp.v1.recog.tag";
/// HKDF `info` of the header (cell body) key.
pub const HDR_KEY_INFO: &[u8] = b"meshapp.v1.recog.hdr";
/// `prf16` label of the tag stream (DESIGN §2.2 `prf16(k_tag, "tag", ctr)`).
pub const TAG_LABEL: &str = "tag";
/// HKDF `info` prefix of the per-cell body key, followed by `LE64(ctr)`.
/// Must equal [`crate::wire::cell::CELL_KEY_INFO`]; a test checks it.
pub const CELL_KEY_INFO: &[u8] = b"meshapp.v1.cell";

/// Length of an epoch seed.
pub const SEED_LEN: usize = 32;

const WINDOW_SPAN: u64 = WINDOW as u64;
const KEEP_BEHIND_SPAN: u64 = KEEP_BEHIND as u64;
/// Bits of the used bitmap that can ever be set.
const USED_MASK: u64 = (1u64 << WINDOW) - 1;

const _: () = assert!(WINDOW <= 64, "the used bitmap is a u64");
const _: () = assert!(LOOKAHEAD < WINDOW);

/// Caller-chosen local id of one chain: a `(group, sender chain)` or one
/// direction of a 1:1 session. Never on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChainKey(pub u64);

/// Caller-chosen id of one epoch of a chain (an SKDM rotation or a DH
/// step). Never on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EpochId(pub u32);

/// The two keys of one epoch of one chain.
#[derive(Clone, PartialEq, Eq)]
pub struct RecogEpoch {
    k_tag: Key32,
    k_hdr: Key32,
}

impl fmt::Debug for RecogEpoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RecogEpoch(<redacted>)")
    }
}

impl RecogEpoch {
    /// Derives both keys from the epoch seed (module docs).
    pub fn from_seed(seed: &[u8; SEED_LEN]) -> Self {
        RecogEpoch {
            k_tag: Key32::from(hkdf32(seed, b"", TAG_KEY_INFO)),
            k_hdr: Key32::from(hkdf32(seed, b"", HDR_KEY_INFO)),
        }
    }

    /// Wraps keys the caller already derived or persisted.
    pub fn from_keys(k_tag: Key32, k_hdr: Key32) -> Self {
        RecogEpoch { k_tag, k_hdr }
    }

    pub fn k_tag(&self) -> &Key32 {
        &self.k_tag
    }

    /// The header key: what [`crate::wire::cell::open_body`] takes.
    pub fn k_hdr(&self) -> &Key32 {
        &self.k_hdr
    }

    /// `prf16(k_tag, "tag", ctr)`: the clear 16-byte tag of the cell with
    /// recognition counter `ctr` (DESIGN §2.2 offset 4).
    pub fn tag_for(&self, ctr: u64) -> [u8; TAG_LEN] {
        prf16(self.k_tag.as_bytes(), TAG_LABEL, ctr)
    }

    /// `HKDF-SHA256(ikm = k_hdr, salt = "", info = "meshapp.v1.cell" ‖
    /// LE64(ctr))`: the body key of the cell with counter `ctr` (DESIGN
    /// §2.2 offset 20), byte-identical to [`crate::wire::cell::cell_key`].
    pub fn cell_key(&self, ctr: u64) -> Key32 {
        let mut info = [0u8; CELL_KEY_INFO.len() + 8];
        info[..CELL_KEY_INFO.len()].copy_from_slice(CELL_KEY_INFO);
        info[CELL_KEY_INFO.len()..].copy_from_slice(&ctr.to_le_bytes());
        Key32::from(hkdf32(self.k_hdr.as_bytes(), b"", &info))
    }
}

/// A recognised tag: which chain and epoch it belongs to and the counter
/// of the cell. The caller opens the body with
/// `table.epoch(hit.chain, hit.epoch_id).k_hdr()` and `hit.ctr`, then
/// [`TagTable::consume`]s the hit once the body authenticated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hit {
    pub chain: ChainKey,
    pub epoch_id: EpochId,
    pub ctr: u64,
}

/// Persistable position of one epoch's window: `base` is the next expected
/// counter and bit `i` of `used` says whether `base + i` was consumed.
/// Bits at or above [`WINDOW`] are always zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WindowState {
    pub base: u64,
    pub used: u64,
}

/// What the sender attaches to its next cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issued {
    pub epoch_id: EpochId,
    pub ctr: u64,
    pub tag: [u8; TAG_LEN],
    pub cell_key: Key32,
}

/// Why a [`TagTable`] operation did not apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum RecogError {
    #[error("chain is not registered")]
    UnknownChain,
    #[error("epoch is not registered on this chain")]
    UnknownEpoch,
    #[error("counter is outside the epoch's window")]
    OutOfWindow,
    #[error("counter was already consumed")]
    AlreadyUsed,
    #[error("chain has no sender state")]
    NoSender,
    #[error("sender counter exhausted")]
    CounterExhausted,
}

/// One epoch's receive window.
#[derive(Debug)]
struct EpochWindow {
    id: EpochId,
    epoch: RecogEpoch,
    base: u64,
    used: u64,
}

impl EpochWindow {
    /// The counters currently in the window, lowest first. Never overflows:
    /// a window that reaches `u64::MAX` is simply shorter.
    fn counters(&self) -> impl Iterator<Item = u64> {
        (self.base..=u64::MAX).take(WINDOW)
    }

    /// First counter above the window, or `None` when the window reaches
    /// the top of the counter space.
    fn end(&self) -> Option<u64> {
        self.base.checked_add(WINDOW_SPAN)
    }

    fn hit(&self, chain: ChainKey, ctr: u64) -> Hit {
        Hit {
            chain,
            epoch_id: self.id,
            ctr,
        }
    }

    fn is_used(&self, ctr: u64) -> bool {
        bit_for(self.base, ctr).is_some_and(|b| self.used & b != 0)
    }
}

/// The bitmap bit of `ctr` relative to `base`, or `None` outside the window.
fn bit_for(base: u64, ctr: u64) -> Option<u64> {
    let off = ctr.checked_sub(base)?;
    (off < WINDOW_SPAN).then(|| 1u64 << off)
}

#[derive(Debug, Clone)]
struct SenderState {
    epoch_id: EpochId,
    epoch: RecogEpoch,
    next_ctr: u64,
}

/// One chain as the table sees it: up to [`MAX_EPOCHS`] receive epochs,
/// oldest first, and optionally a sender state. Read-only from outside;
/// mutation goes through [`TagTable`] so the tag map stays in step.
#[derive(Debug, Default)]
pub struct RecogChain {
    epochs: VecDeque<EpochWindow>,
    sender: Option<SenderState>,
}

impl RecogChain {
    /// Receive epochs, oldest first.
    pub fn epochs(&self) -> impl Iterator<Item = (EpochId, &RecogEpoch)> {
        self.epochs.iter().map(|w| (w.id, &w.epoch))
    }

    pub fn epoch(&self, id: EpochId) -> Option<&RecogEpoch> {
        self.epochs.iter().find(|w| w.id == id).map(|w| &w.epoch)
    }

    /// The window of one receive epoch, for persistence.
    pub fn window(&self, id: EpochId) -> Option<WindowState> {
        self.epochs
            .iter()
            .find(|w| w.id == id)
            .map(|w| WindowState {
                base: w.base,
                used: w.used,
            })
    }

    /// The next expected counter of one receive epoch (the window's base).
    pub fn next_expected(&self, id: EpochId) -> Option<u64> {
        self.window(id).map(|w| w.base)
    }

    /// The sender epoch, if this chain sends.
    pub fn sender_epoch(&self) -> Option<(EpochId, &RecogEpoch)> {
        self.sender.as_ref().map(|s| (s.epoch_id, &s.epoch))
    }

    /// The counter the next [`TagTable::sender_next`] will issue, for
    /// persistence.
    pub fn sender_next_ctr(&self) -> Option<u64> {
        self.sender.as_ref().map(|s| s.next_ctr)
    }

    fn is_empty(&self) -> bool {
        self.epochs.is_empty() && self.sender.is_none()
    }

    fn position(&self, id: EpochId) -> Option<usize> {
        self.epochs.iter().position(|w| w.id == id)
    }
}

/// Every registered chain's windows and the one map that recognises tags
/// (module docs). Deterministic: the same sequence of calls gives the same
/// lookups, and the map is a `BTreeMap` so no per-process hash seed is
/// involved. Tags are public values, so lookups need no constant-time
/// comparison; no operation's timing depends on a key.
#[derive(Debug, Default)]
pub struct TagTable {
    chains: BTreeMap<ChainKey, RecogChain>,
    tags: BTreeMap<[u8; TAG_LEN], Hit>,
}

impl TagTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a receive epoch with its window starting at `start_ctr`
    /// (0 for a fresh 1:1 epoch; the SKDM's iteration for a group chain).
    /// Re-registering an existing `(chain, epoch_id)` replaces its window,
    /// which re-accepts the counters consumed under the old one: a duplicate
    /// delivery of the same epoch (a re-sent SKDM) must not be re-registered.
    /// Check [`epoch`](Self::epoch) first, or use [`restore`](Self::restore)
    /// with the persisted [`WindowState`]. A chain keeps at most
    /// [`MAX_EPOCHS`] epochs, in registration order: the earliest registered
    /// is forgotten when a fourth is added, so register (and restore) epochs
    /// oldest first.
    pub fn register(
        &mut self,
        chain: ChainKey,
        epoch_id: EpochId,
        epoch: RecogEpoch,
        start_ctr: u64,
    ) {
        self.restore(
            chain,
            epoch_id,
            epoch,
            WindowState {
                base: start_ctr,
                used: 0,
            },
        );
    }

    /// [`register`](Self::register) from a persisted [`WindowState`], so a
    /// restart neither re-accepts consumed counters nor loses the window's
    /// position. Bits of `used` at or above [`WINDOW`] are ignored.
    pub fn restore(
        &mut self,
        chain: ChainKey,
        epoch_id: EpochId,
        epoch: RecogEpoch,
        state: WindowState,
    ) {
        self.forget_epoch(chain, epoch_id);
        let entry = self.chains.entry(chain).or_default();
        while entry.epochs.len() >= MAX_EPOCHS {
            if let Some(old) = entry.epochs.pop_front() {
                Self::remove_window_tags(&mut self.tags, chain, &old, None);
            }
        }
        let window = EpochWindow {
            id: epoch_id,
            epoch,
            base: state.base,
            used: state.used & USED_MASK,
        };
        for ctr in window.counters() {
            if !window.is_used(ctr) {
                Self::insert_tag(&mut self.tags, chain, &window, ctr);
            }
        }
        entry.epochs.push_back(window);
    }

    /// Drops one receive epoch and its tags. `true` if it existed.
    pub fn forget_epoch(&mut self, chain: ChainKey, epoch_id: EpochId) -> bool {
        let Some(entry) = self.chains.get_mut(&chain) else {
            return false;
        };
        let Some(idx) = entry.position(epoch_id) else {
            return false;
        };
        if let Some(old) = entry.epochs.remove(idx) {
            Self::remove_window_tags(&mut self.tags, chain, &old, None);
        }
        if entry.is_empty() {
            self.chains.remove(&chain);
        }
        true
    }

    /// Drops a chain entirely: every receive epoch and the sender state.
    pub fn forget_chain(&mut self, chain: ChainKey) -> bool {
        let Some(entry) = self.chains.remove(&chain) else {
            return false;
        };
        for old in &entry.epochs {
            Self::remove_window_tags(&mut self.tags, chain, old, None);
        }
        true
    }

    /// Recognises a cell's clear tag. `None` for anything not in a window:
    /// an unknown chain, a counter outside every window, a consumed
    /// counter, or a sealed first flight (which `wire/seal` matches
    /// separately).
    pub fn lookup(&self, tag: &[u8; TAG_LEN]) -> Option<Hit> {
        self.tags.get(tag).copied()
    }

    /// Marks `hit.ctr` consumed and slides its window (module docs). Call
    /// it only after the body opened under the hit's key, so that a junk
    /// cell wearing a real tag cannot burn the counter. A second consume of
    /// the same counter, or of a counter that left the window, is an error
    /// and changes nothing.
    pub fn consume(&mut self, hit: Hit) -> Result<(), RecogError> {
        let entry = self
            .chains
            .get_mut(&hit.chain)
            .ok_or(RecogError::UnknownChain)?;
        let idx = entry
            .position(hit.epoch_id)
            .ok_or(RecogError::UnknownEpoch)?;
        let window = entry.epochs.get_mut(idx).ok_or(RecogError::UnknownEpoch)?;
        let bit = bit_for(window.base, hit.ctr).ok_or(RecogError::OutOfWindow)?;
        if window.used & bit != 0 {
            return Err(RecogError::AlreadyUsed);
        }
        window.used |= bit;
        Self::remove_tag(&mut self.tags, hit.chain, window, hit.ctr);
        // Rule 1: at least LOOKAHEAD counters beyond this hit.
        let centred = hit.ctr.saturating_sub(KEEP_BEHIND_SPAN);
        Self::slide(&mut self.tags, hit.chain, window, centred);
        // Rule 2: past the run of consumed counters at the bottom.
        let run = u64::from(window.used.trailing_ones());
        let after_run = window.base.saturating_add(run);
        Self::slide(&mut self.tags, hit.chain, window, after_run);
        Ok(())
    }

    /// The keys of one receive epoch, to open a recognised cell's body.
    pub fn epoch(&self, chain: ChainKey, epoch_id: EpochId) -> Option<&RecogEpoch> {
        self.chains.get(&chain)?.epoch(epoch_id)
    }

    pub fn chain(&self, chain: ChainKey) -> Option<&RecogChain> {
        self.chains.get(&chain)
    }

    /// Registered chains, in key order.
    pub fn chains(&self) -> impl Iterator<Item = (ChainKey, &RecogChain)> {
        self.chains.iter().map(|(k, c)| (*k, c))
    }

    /// The window of one receive epoch, for persistence.
    pub fn window(&self, chain: ChainKey, epoch_id: EpochId) -> Option<WindowState> {
        self.chains.get(&chain)?.window(epoch_id)
    }

    /// The body key of a recognised cell (`cell_key(hit.ctr)` under the
    /// hit's epoch).
    pub fn cell_key(&self, hit: &Hit) -> Option<Key32> {
        Some(self.epoch(hit.chain, hit.epoch_id)?.cell_key(hit.ctr))
    }

    /// Sets the chain's sending epoch and the next counter to issue (0 for
    /// a new epoch; the persisted value after a restart). Replaces any
    /// previous sender state of the chain.
    pub fn register_sender(
        &mut self,
        chain: ChainKey,
        epoch_id: EpochId,
        epoch: RecogEpoch,
        next_ctr: u64,
    ) {
        self.chains.entry(chain).or_default().sender = Some(SenderState {
            epoch_id,
            epoch,
            next_ctr,
        });
    }

    /// Issues the next counter of the chain's sending epoch with its tag
    /// and body key. Counters are never reused: the one before `u64::MAX`
    /// is the last a chain can issue (rotate long before).
    pub fn sender_next(&mut self, chain: ChainKey) -> Result<Issued, RecogError> {
        let sender = self
            .chains
            .get_mut(&chain)
            .ok_or(RecogError::UnknownChain)?
            .sender
            .as_mut()
            .ok_or(RecogError::NoSender)?;
        let ctr = sender.next_ctr;
        let next = ctr.checked_add(1).ok_or(RecogError::CounterExhausted)?;
        sender.next_ctr = next;
        Ok(Issued {
            epoch_id: sender.epoch_id,
            ctr,
            tag: sender.epoch.tag_for(ctr),
            cell_key: sender.epoch.cell_key(ctr),
        })
    }

    /// The counter the next [`sender_next`](Self::sender_next) will issue,
    /// for persistence.
    pub fn sender_next_ctr(&self, chain: ChainKey) -> Option<u64> {
        self.chains.get(&chain)?.sender_next_ctr()
    }

    /// Number of precomputed tags held (≤ chains × MAX_EPOCHS × WINDOW).
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    pub fn chain_count(&self) -> usize {
        self.chains.len()
    }

    // ---- internals --------------------------------------------------------

    /// Adds the tag of `ctr`. On a 16-byte collision between two epochs
    /// (probability 2^-128 per pair) the earlier entry wins and the later
    /// counter is unrecognisable; nothing else is affected.
    fn insert_tag(
        tags: &mut BTreeMap<[u8; TAG_LEN], Hit>,
        chain: ChainKey,
        window: &EpochWindow,
        ctr: u64,
    ) {
        tags.entry(window.epoch.tag_for(ctr))
            .or_insert(window.hit(chain, ctr));
    }

    /// Removes the tag of `ctr` if it maps to this window's hit (so a
    /// colliding entry of another epoch is left alone).
    fn remove_tag(
        tags: &mut BTreeMap<[u8; TAG_LEN], Hit>,
        chain: ChainKey,
        window: &EpochWindow,
        ctr: u64,
    ) {
        let tag = window.epoch.tag_for(ctr);
        if tags.get(&tag) == Some(&window.hit(chain, ctr)) {
            tags.remove(&tag);
        }
    }

    /// Removes the tags of the window's counters below `below` (all of them
    /// for `None`).
    fn remove_window_tags(
        tags: &mut BTreeMap<[u8; TAG_LEN], Hit>,
        chain: ChainKey,
        window: &EpochWindow,
        below: Option<u64>,
    ) {
        for ctr in window
            .counters()
            .take_while(|c| below.is_none_or(|b| *c < b))
        {
            if !window.is_used(ctr) {
                Self::remove_tag(tags, chain, window, ctr);
            }
        }
    }

    /// Moves the window's base up to `new_base` (never down): drops the
    /// tags that leave, shifts the used bitmap and precomputes the tags
    /// that enter.
    fn slide(
        tags: &mut BTreeMap<[u8; TAG_LEN], Hit>,
        chain: ChainKey,
        window: &mut EpochWindow,
        new_base: u64,
    ) {
        if new_base <= window.base {
            return;
        }
        let old_end = window.end();
        let shift = new_base - window.base;
        Self::remove_window_tags(tags, chain, window, Some(new_base));
        window.used = if shift >= 64 { 0 } else { window.used >> shift };
        window.base = new_base;
        for ctr in window
            .counters()
            .filter(|c| old_end.is_some_and(|e| *c >= e))
        {
            Self::insert_tag(tags, chain, window, ctr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::cell;
    use proptest::prelude::*;

    fn seed(n: u8) -> [u8; SEED_LEN] {
        let mut s = [0u8; SEED_LEN];
        for (i, b) in s.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(13).wrapping_add(n);
        }
        s
    }

    fn epoch(n: u8) -> RecogEpoch {
        RecogEpoch::from_seed(&seed(n))
    }

    const A: ChainKey = ChainKey(1);
    const B: ChainKey = ChainKey(2);
    const E0: EpochId = EpochId(0);
    const E1: EpochId = EpochId(1);
    const E2: EpochId = EpochId(2);
    const E3: EpochId = EpochId(3);

    fn hit(chain: ChainKey, epoch_id: EpochId, ctr: u64) -> Hit {
        Hit {
            chain,
            epoch_id,
            ctr,
        }
    }

    #[test]
    fn derivation_follows_the_module_docs() {
        let s = seed(1);
        let e = RecogEpoch::from_seed(&s);
        assert_eq!(
            e.k_tag().as_bytes(),
            &hkdf32(&s, b"", b"meshapp.v1.recog.tag")
        );
        assert_eq!(
            e.k_hdr().as_bytes(),
            &hkdf32(&s, b"", b"meshapp.v1.recog.hdr")
        );
        assert_ne!(e.k_tag(), e.k_hdr());
        assert_eq!(e.tag_for(7), prf16(e.k_tag().as_bytes(), "tag", 7));
        let mut info = b"meshapp.v1.cell".to_vec();
        info.extend_from_slice(&7u64.to_le_bytes());
        assert_eq!(
            e.cell_key(7).as_bytes(),
            &hkdf32(e.k_hdr().as_bytes(), b"", &info)
        );
        assert_eq!(
            RecogEpoch::from_keys(e.k_tag().clone(), e.k_hdr().clone()),
            e
        );
        assert_eq!(format!("{e:?}"), "RecogEpoch(<redacted>)");
    }

    #[test]
    fn cell_key_matches_wire_cell_byte_for_byte() {
        assert_eq!(CELL_KEY_INFO, cell::CELL_KEY_INFO);
        let e = epoch(2);
        for ctr in [0u64, 1, 49, 50, 1 << 32, u64::MAX] {
            assert_eq!(e.cell_key(ctr), cell::cell_key(e.k_hdr().as_bytes(), ctr));
        }
    }

    #[test]
    fn tags_are_distinct_across_counters_and_epochs() {
        let e = epoch(3);
        let f = epoch(4);
        assert_eq!(e.tag_for(5), e.tag_for(5));
        assert_ne!(e.tag_for(5), e.tag_for(6));
        assert_ne!(e.tag_for(5), f.tag_for(5));
        assert_ne!(e.cell_key(5), e.cell_key(6));
        assert_ne!(e.cell_key(5), f.cell_key(5));
    }

    #[test]
    fn register_precomputes_exactly_one_window() {
        let mut t = TagTable::new();
        t.register(A, E0, epoch(1), 100);
        assert_eq!(t.len(), WINDOW);
        assert_eq!(t.chain_count(), 1);
        let e = epoch(1);
        for ctr in 100..100 + WINDOW_SPAN {
            assert_eq!(t.lookup(&e.tag_for(ctr)), Some(hit(A, E0, ctr)));
        }
        assert_eq!(t.lookup(&e.tag_for(99)), None);
        assert_eq!(t.lookup(&e.tag_for(100 + WINDOW_SPAN)), None);
        assert_eq!(t.window(A, E0), Some(WindowState { base: 100, used: 0 }));
        assert_eq!(t.chain(A).unwrap().next_expected(E0), Some(100));
        assert_eq!(t.epoch(A, E0), Some(&e));
        assert_eq!(t.epoch(A, E1), None);
        assert_eq!(t.epoch(B, E0), None);
    }

    #[test]
    fn in_order_consume_keeps_full_lookahead() {
        let mut t = TagTable::new();
        let e = epoch(1);
        t.register(A, E0, e.clone(), 0);
        for ctr in 0..200u64 {
            let h = t.lookup(&e.tag_for(ctr)).expect("in window");
            assert_eq!(h.ctr, ctr);
            t.consume(h).unwrap();
            assert_eq!(t.len(), WINDOW, "window stays exactly WINDOW");
            assert_eq!(
                t.window(A, E0),
                Some(WindowState {
                    base: ctr + 1,
                    used: 0
                })
            );
        }
    }

    #[test]
    fn replay_never_hits_twice() {
        let mut t = TagTable::new();
        let e = epoch(1);
        t.register(A, E0, e.clone(), 0);
        let h = t.lookup(&e.tag_for(3)).unwrap();
        t.consume(h).unwrap();
        assert_eq!(t.lookup(&e.tag_for(3)), None);
        assert_eq!(t.consume(h), Err(RecogError::AlreadyUsed));
        // counters below the hit are still recognisable (lookbehind)
        for ctr in 0..3 {
            assert_eq!(t.lookup(&e.tag_for(ctr)), Some(hit(A, E0, ctr)));
        }
        assert_eq!(t.window(A, E0).unwrap().used, 0b1000);
    }

    #[test]
    fn out_of_order_within_the_window_loses_nothing() {
        let mut t = TagTable::new();
        let e = epoch(1);
        t.register(A, E0, e.clone(), 0);
        // a 20-cell object arriving in reverse order
        for ctr in (0..20u64).rev() {
            let h = t.lookup(&e.tag_for(ctr)).expect("still in window");
            t.consume(h).unwrap();
        }
        assert_eq!(t.window(A, E0), Some(WindowState { base: 20, used: 0 }));
        assert_eq!(t.len(), WINDOW);
    }

    #[test]
    fn a_hole_does_not_stall_the_chain() {
        let mut t = TagTable::new();
        let e = epoch(1);
        t.register(A, E0, e.clone(), 0);
        // counter 0 never arrives; 1.. keep coming in order
        for ctr in 1..500u64 {
            let h = t
                .lookup(&e.tag_for(ctr))
                .expect("window followed the sender");
            t.consume(h).unwrap();
            let w = t.window(A, E0).unwrap();
            assert!(w.base <= ctr + 1);
            assert!(
                w.base + WINDOW_SPAN > ctr + LOOKAHEAD as u64,
                "at least LOOKAHEAD ahead of the newest hit"
            );
            assert!(t.len() <= WINDOW);
            assert!(t.len() >= WINDOW - KEEP_BEHIND, "{} at {ctr}", t.len());
            assert_eq!(w.used & 1, 0, "window never starts on a consumed counter");
        }
        // the hole was given up once it fell behind the window, and the
        // window is full again
        assert_eq!(t.len(), WINDOW);
        assert_eq!(t.lookup(&e.tag_for(0)), None);
        assert_eq!(t.consume(hit(A, E0, 0)), Err(RecogError::OutOfWindow));
    }

    #[test]
    fn forced_slide_keeps_keep_behind_counters() {
        let mut t = TagTable::new();
        let e = epoch(1);
        t.register(A, E0, e.clone(), 0);
        let top = WINDOW_SPAN - 1;
        t.consume(t.lookup(&e.tag_for(top)).unwrap()).unwrap();
        let w = t.window(A, E0).unwrap();
        assert_eq!(w.base, top - KEEP_BEHIND_SPAN);
        assert_eq!(w.used, 1 << KEEP_BEHIND);
        for ctr in 0..w.base {
            assert_eq!(t.lookup(&e.tag_for(ctr)), None, "given up");
        }
        for ctr in w.base..top {
            assert_eq!(t.lookup(&e.tag_for(ctr)), Some(hit(A, E0, ctr)));
        }
        for ctr in top + 1..w.base + WINDOW_SPAN {
            assert_eq!(t.lookup(&e.tag_for(ctr)), Some(hit(A, E0, ctr)));
        }
        assert_eq!(t.len(), WINDOW - 1);
    }

    #[test]
    fn three_epochs_are_kept_and_the_oldest_is_evicted() {
        let mut t = TagTable::new();
        t.register(A, E0, epoch(0), 0);
        t.register(A, E1, epoch(1), 0);
        t.register(A, E2, epoch(2), 0);
        assert_eq!(t.len(), 3 * WINDOW);
        assert_eq!(
            t.chain(A)
                .unwrap()
                .epochs()
                .map(|(id, _)| id)
                .collect::<Vec<_>>(),
            vec![E0, E1, E2]
        );
        t.register(A, E3, epoch(3), 0);
        assert_eq!(t.len(), 3 * WINDOW);
        assert_eq!(
            t.chain(A)
                .unwrap()
                .epochs()
                .map(|(id, _)| id)
                .collect::<Vec<_>>(),
            vec![E1, E2, E3]
        );
        assert_eq!(t.lookup(&epoch(0).tag_for(0)), None);
        assert_eq!(t.lookup(&epoch(3).tag_for(0)), Some(hit(A, E3, 0)));
        assert_eq!(t.consume(hit(A, E0, 0)), Err(RecogError::UnknownEpoch));
    }

    #[test]
    fn re_registering_an_epoch_replaces_its_window() {
        let mut t = TagTable::new();
        let e = epoch(1);
        t.register(A, E0, e.clone(), 0);
        t.consume(t.lookup(&e.tag_for(0)).unwrap()).unwrap();
        t.register(A, E0, e.clone(), 1000);
        assert_eq!(t.len(), WINDOW);
        assert_eq!(t.lookup(&e.tag_for(1)), None);
        assert_eq!(t.lookup(&e.tag_for(1000)), Some(hit(A, E0, 1000)));
        assert_eq!(t.chain(A).unwrap().epochs().count(), 1);
    }

    #[test]
    fn restore_skips_used_counters_and_masks_stray_bits() {
        let mut t = TagTable::new();
        let e = epoch(1);
        t.restore(
            A,
            E0,
            e.clone(),
            WindowState {
                base: 10,
                used: 0b101 | (1 << 63),
            },
        );
        assert_eq!(t.window(A, E0).unwrap().used, 0b101);
        assert_eq!(t.len(), WINDOW - 2);
        assert_eq!(t.lookup(&e.tag_for(10)), None);
        assert_eq!(t.lookup(&e.tag_for(11)), Some(hit(A, E0, 11)));
        assert_eq!(t.lookup(&e.tag_for(12)), None);
        assert_eq!(t.consume(hit(A, E0, 10)), Err(RecogError::AlreadyUsed));
        // consuming 11 fills the run and slides past all three
        t.consume(hit(A, E0, 11)).unwrap();
        assert_eq!(t.window(A, E0), Some(WindowState { base: 13, used: 0 }));
        assert_eq!(t.len(), WINDOW);
    }

    #[test]
    fn forget_epoch_and_chain_remove_tags() {
        let mut t = TagTable::new();
        t.register(A, E0, epoch(0), 0);
        t.register(A, E1, epoch(1), 0);
        t.register(B, E0, epoch(2), 0);
        assert!(t.forget_epoch(A, E0));
        assert!(!t.forget_epoch(A, E0));
        assert!(!t.forget_epoch(B, E1));
        assert_eq!(t.len(), 2 * WINDOW);
        assert_eq!(t.lookup(&epoch(0).tag_for(0)), None);
        assert!(t.forget_epoch(A, E1));
        assert_eq!(t.chain_count(), 1, "empty chain entry dropped");
        assert!(t.forget_chain(B));
        assert!(!t.forget_chain(B));
        assert!(t.is_empty());
        assert_eq!(t.consume(hit(A, E0, 0)), Err(RecogError::UnknownChain));
    }

    #[test]
    fn two_chains_do_not_interfere() {
        let mut t = TagTable::new();
        let ea = epoch(1);
        let eb = epoch(2);
        t.register(A, E0, ea.clone(), 0);
        t.register(B, E0, eb.clone(), 0);
        for ctr in 0..100u64 {
            let ha = t.lookup(&ea.tag_for(ctr)).unwrap();
            let hb = t.lookup(&eb.tag_for(ctr)).unwrap();
            assert_eq!(ha.chain, A);
            assert_eq!(hb.chain, B);
            t.consume(ha).unwrap();
            t.consume(hb).unwrap();
        }
        assert_eq!(t.len(), 2 * WINDOW);
    }

    /// Two windows whose tags collide (forced here by registering the same
    /// epoch keys on two chains, so every tag collides) never corrupt each
    /// other: the first entry wins, a consume on the loser leaves the
    /// winner's tag in place, and forgetting the loser leaves the winner's
    /// window complete. Fails if `insert_tag` overwrites or `remove_tag`
    /// removes without checking whose hit the entry is.
    #[test]
    fn a_tag_collision_never_corrupts_the_other_window() {
        let mut t = TagTable::new();
        let e = epoch(1);
        t.register(A, E0, e.clone(), 0);
        t.register(B, E0, e.clone(), 0);
        assert_eq!(t.len(), WINDOW, "the loser inserted nothing");
        for ctr in 0..WINDOW_SPAN {
            assert_eq!(t.lookup(&e.tag_for(ctr)), Some(hit(A, E0, ctr)));
        }
        // consuming on the loser marks its window but must not touch A's tag
        assert_eq!(t.consume(hit(B, E0, 3)), Ok(()));
        assert_eq!(t.lookup(&e.tag_for(3)), Some(hit(A, E0, 3)));
        assert_eq!(t.window(B, E0).unwrap().used, 0b1000);
        // forgetting the loser leaves the winner's window complete
        assert!(t.forget_chain(B));
        assert_eq!(t.len(), WINDOW);
        for ctr in 0..WINDOW_SPAN {
            assert_eq!(t.lookup(&e.tag_for(ctr)), Some(hit(A, E0, ctr)));
        }
        // and the winner's own consume still removes exactly its tag
        t.consume(hit(A, E0, 0)).unwrap();
        assert_eq!(t.lookup(&e.tag_for(0)), None);
        assert_eq!(t.len(), WINDOW);
    }

    #[test]
    fn window_near_the_top_of_the_counter_space_is_short_and_safe() {
        let mut t = TagTable::new();
        let e = epoch(1);
        let base = u64::MAX - 10;
        t.register(A, E0, e.clone(), base);
        assert_eq!(t.len(), 11);
        for ctr in base..=u64::MAX {
            let h = t.lookup(&e.tag_for(ctr)).expect("in the short window");
            t.consume(h).unwrap();
        }
        assert!(t.is_empty());
        assert_eq!(t.window(A, E0).unwrap().base, u64::MAX);
        t.register(B, E0, e.clone(), u64::MAX);
        assert_eq!(t.len(), 1);
        t.consume(t.lookup(&e.tag_for(u64::MAX)).unwrap()).unwrap();
        assert!(t.is_empty());
    }

    #[test]
    fn sender_counters_are_monotone_and_agree_with_the_receiver() {
        let mut t = TagTable::new();
        let e = epoch(9);
        assert_eq!(t.sender_next(A), Err(RecogError::UnknownChain));
        t.register(A, E0, e.clone(), 0);
        assert_eq!(t.sender_next(A), Err(RecogError::NoSender));
        t.register_sender(A, E2, e.clone(), 5);
        assert_eq!(t.sender_next_ctr(A), Some(5));
        let i1 = t.sender_next(A).unwrap();
        let i2 = t.sender_next(A).unwrap();
        assert_eq!((i1.epoch_id, i1.ctr), (E2, 5));
        assert_eq!((i2.epoch_id, i2.ctr), (E2, 6));
        assert_eq!(i1.tag, e.tag_for(5));
        assert_eq!(i1.cell_key, e.cell_key(5));
        assert_ne!(i1.tag, i2.tag);
        assert_eq!(t.sender_next_ctr(A), Some(7));
        assert_eq!(
            t.chain(A).unwrap().sender_epoch().map(|(id, _)| id),
            Some(E2)
        );
        // the receiver side of the same epoch resolves the issued tag
        let mut r = TagTable::new();
        r.register(B, E2, e.clone(), 5);
        let h = r.lookup(&i1.tag).unwrap();
        assert_eq!(h, hit(B, E2, 5));
        assert_eq!(r.cell_key(&h), Some(i1.cell_key.clone()));
        assert_eq!(r.cell_key(&hit(B, E3, 5)), None);
        // the chain entry survives forgetting the receive epoch
        assert!(t.forget_epoch(A, E0));
        assert_eq!(t.sender_next_ctr(A), Some(7));
    }

    #[test]
    fn sender_refuses_to_wrap() {
        let mut t = TagTable::new();
        t.register_sender(A, E0, epoch(1), u64::MAX - 1);
        assert_eq!(t.sender_next(A).unwrap().ctr, u64::MAX - 1);
        assert_eq!(t.sender_next(A), Err(RecogError::CounterExhausted));
        assert_eq!(t.sender_next_ctr(A), Some(u64::MAX));
    }

    #[test]
    fn bit_for_bounds() {
        assert_eq!(bit_for(10, 9), None);
        assert_eq!(bit_for(10, 10), Some(1));
        assert_eq!(bit_for(10, 10 + WINDOW_SPAN - 1), Some(1 << (WINDOW - 1)));
        assert_eq!(bit_for(10, 10 + WINDOW_SPAN), None);
        assert_eq!(bit_for(u64::MAX, u64::MAX), Some(1));
        assert_eq!(bit_for(0, u64::MAX), None);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// Any tag bytes and any hit values are safe inputs, and the table
        /// never exceeds its bound.
        #[test]
        fn random_lookups_and_consumes_never_panic_or_overgrow(
            ops in prop::collection::vec(
                (0u8..4, any::<u64>(), any::<u32>(), any::<u64>(), any::<[u8; 16]>()),
                0..200,
            ),
        ) {
            let mut t = TagTable::new();
            for (op, a, b, c, tag) in ops {
                match op {
                    0 => t.register(ChainKey(a % 4), EpochId(b % 5), epoch((b % 7) as u8), c),
                    1 => {
                        let _ = t.lookup(&tag);
                        let _ = t.consume(hit(ChainKey(a % 4), EpochId(b % 5), c));
                    }
                    2 => {
                        if let Some(h) = t.lookup(&epoch((b % 7) as u8).tag_for(c)) {
                            prop_assert!(t.consume(h).is_ok());
                            prop_assert!(t.consume(h).is_err());
                        }
                    }
                    _ => {
                        t.forget_epoch(ChainKey(a % 4), EpochId(b % 5));
                    }
                }
                prop_assert!(t.len() <= t.chain_count() * MAX_EPOCHS * WINDOW);
                for (_, chain) in t.chains() {
                    prop_assert!(chain.epochs().count() <= MAX_EPOCHS);
                }
            }
        }
    }
}
