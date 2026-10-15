//! The in-memory cell store (docs/DESIGN.md §1 row `dtn/store.rs`; §6
//! `cells(msg_id PK, bytes[512], class, ttl, lane, first_seen_mono, …)`).
//!
//! Rules implemented here:
//!
//! * **Fixed rows keyed by `msg_id`** (DESIGN §2.2 "Dedupe, summary vectors
//!   and storage all key on the derived value"): [`CellStore::put`] derives
//!   the id from the cell and reports a second copy as
//!   [`Put::Duplicate`]; a copy with a different TTL is the same cell.
//! * **Bounded buffers with per-link and per-class quotas** (BRIEF_v2 §5.5
//!   "Buffers"; DECISIONS D4 BUFFER "50 MB, at most 5 MB per inbound link"):
//!   [`StoreCaps`] holds the two caps, the store keeps counts per class and
//!   per inbound link, and `put` asks the [`EvictionPolicy`] for victims
//!   when a cap would be exceeded. The 20 % reserve for absent peers and the
//!   production policy belong to `relay/buffers.rs` (DESIGN §1); the store
//!   only provides the hook and the accounting.
//! * **Eviction order** (BRIEF_v2 §5.5 "within a class L3 is evicted first;
//!   a lower class in the §5.6 order never evicts a higher one"; D4 "Evict
//!   L3 first, then oldest or lowest TTL"): [`LowestClassL3First`] is the
//!   default policy. Two of the rules are enforced by the store itself, so
//!   no policy can break them: a pinned entry is never evicted (D3 M1 /
//!   DESIGN §3 "roster ops are never evicted until superseded"), and an
//!   entry whose class outranks the incoming cell's class is never evicted.
//! * **Forwarding candidates** (BRIEF_v2 §5.6 strict class priority; §5.7
//!   DTN store-and-forward): [`CellStore::scan_forwardable`] (and the
//!   id-only [`CellStore::candidates_for`]) lists cells in class order,
//!   oldest first within a class, skipping cells a relay may no longer
//!   forward (`ttl < 2`, DESIGN §2.2). Each candidate carries its *slot*
//!   and *generation* so a link can remember what it offered in a bitset
//!   instead of a set of ids.
//! * **Summary index** (BRIEF_v2 §5.7 anti-entropy; DESIGN §7 "summary
//!   vectors"; D4 MERGE): the store owns a
//!   [`SummaryIndex`](crate::dtn::summary::SummaryIndex) that caches each
//!   cell's two hash words at `put` (the cell's `msg_id` is the one
//!   SHA-256 a cell ever costs for summary purposes) and keeps a counting
//!   Bloom filter under the store's current salt exact on every put and
//!   evict. [`CellStore::summary_salt_for`] is the **salt rotation
//!   policy**: DESIGN §10 risk 4 names "salt rotation per encounter" as
//!   the knob against accumulated false positives, and every encounter
//!   still carries its own salt on the wire; but the store hands out a
//!   *fresh* salt (re-deriving the filter, `O(n·k)` integer operations)
//!   at most once per [`SUMMARY_SALT_ROTATE_MS_ASM`] of powered-on time
//!   and otherwise the current one (an `O(n_bits/8)` copy). Trade-off: two
//!   peers met inside one window receive vectors under the same salt, so
//!   (a) a false positive of ours repeats for both until the next
//!   rotation, bounded by the window, and (b) the two vectors are
//!   bit-comparable, which lets colluding peers link the two encounters
//!   to one phone and count what arrived in between. The Noise session
//!   hides the vector from everyone else (DESIGN §7), and the platform's
//!   BLE address rotation already links encounters over a similar window,
//!   so the default is a 10-minute window; `set_summary_salt_interval_ms(0)`
//!   restores per-encounter rotation at `O(n·k)` per encounter.
//!
//! Everything is deterministic: the same sequence of calls gives the same
//! contents, the same candidate order and the same victims. Time only
//! arrives as `first_seen_mono` inside [`CellMeta`] (BRIEF_v2 §4 "no
//! timestamps decide anything"; powered-on milliseconds from
//! [`crate::clock::Clock`]). Milestone 1b replaces the maps with the SQLite
//! `cells` table behind the same API.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::crypto::hash::Hash32;
use crate::dtn::summary::{IdWords, SUMMARY_TARGET_FPR_ASM, SummaryIndex};
use crate::relay::Lane;
use crate::transport::link::LinkId;
use crate::wire::cell::{CELL_LEN, Cell};
use crate::wire::frame::Class;

/// Total buffer, DECISIONS D4 BUFFER "50 MB" (unmeasured; D4 "all
/// parameters are unmeasured and need the Decision 10 field spike").
pub const STORE_MAX_BYTES_ASM: usize = 50 * 1024 * 1024;
/// Per inbound link, D4 BUFFER "at most 5 MB per inbound link".
pub const STORE_MAX_BYTES_PER_LINK_ASM: usize = 5 * 1024 * 1024;
/// Powered-on time between summary salt rotations (module docs; DESIGN
/// §10 risk 4; an assumption: the window of BLE address rotation, within
/// which encounters are linkable anyway). `0` rotates on every encounter.
pub const SUMMARY_SALT_ROTATE_MS_ASM: u64 = 10 * 60_000;

/// What the store knows about a cell besides its bytes (DESIGN §6 `cells`
/// row: class, lane, first_seen_mono; the TTL is read from the cell).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellMeta {
    /// Message class from the link frame (BRIEF_v2 §5.6).
    pub class: Class,
    /// Lane on the link it arrived on (BRIEF_v2 §5.5 lanes).
    pub lane: Lane,
    /// Inbound link, or `None` for a cell this phone originated. Only cells
    /// with a link count against the per-link cap.
    pub from_link: Option<LinkId>,
    /// Powered-on milliseconds when this phone first held the cell.
    pub first_seen_mono: u64,
    /// Roster ops: never evicted until superseded (D3 M1; DESIGN §3). The
    /// roster module clears the pin (or removes the cell) when a newer op
    /// makes it redundant.
    pub pinned: bool,
}

/// Result of [`CellStore::put`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Put {
    /// The cell is now in the store (victims may have been evicted first).
    Stored,
    /// A cell with this `msg_id` was already held; nothing changed.
    Duplicate,
    /// The caps are exceeded and the policy could not free enough bytes
    /// without evicting a pinned entry or a higher class (BRIEF_v2 §5.5 "a
    /// lower class never evicts a higher one"). Nothing changed.
    NoRoom,
}

/// Cap set of a store (DECISIONS D4 BUFFER). Both caps are in bytes; each
/// cell costs [`CELL_LEN`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreCaps {
    /// Whole-store cap.
    pub max_bytes: usize,
    /// Cap on cells from any one inbound link (`from_link == Some(_)`).
    pub max_bytes_per_link: usize,
}

impl StoreCaps {
    /// D4 BUFFER: 50 MB total, 5 MB per inbound link.
    pub const DEFAULT: StoreCaps = StoreCaps {
        max_bytes: STORE_MAX_BYTES_ASM,
        max_bytes_per_link: STORE_MAX_BYTES_PER_LINK_ASM,
    };

    /// Caps expressed in cells rather than bytes (tests, simulator).
    pub const fn cells(max_cells: usize, max_cells_per_link: usize) -> StoreCaps {
        StoreCaps {
            max_bytes: max_cells.saturating_mul(CELL_LEN),
            max_bytes_per_link: max_cells_per_link.saturating_mul(CELL_LEN),
        }
    }
}

impl Default for StoreCaps {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The view of one stored cell an [`EvictionPolicy`] decides on: exactly
/// the fields the eviction rules mention (id, class, lane, age, pin, TTL)
/// and nothing that would let a policy see link identities or bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvictionEntry {
    pub id: Hash32,
    pub class: Class,
    pub lane: Lane,
    pub first_seen_mono: u64,
    pub pinned: bool,
    pub ttl: u8,
}

/// Chooses what to drop when a cap would be exceeded (BRIEF_v2 §5.5
/// "Buffers"; D4 BUFFER). `relay/buffers.rs` provides the production
/// policy; [`LowestClassL3First`] is the default.
pub trait EvictionPolicy {
    /// Returns ids from `entries` to evict so that at least `need_bytes`
    /// become free for `incoming` (the cell being stored; its
    /// `first_seen_mono` is "now"). The store ignores ids that are not in
    /// `entries`, pinned entries and entries whose class outranks
    /// `incoming.class`, and returns [`Put::NoRoom`] if what remains frees
    /// less than `need_bytes`. Each entry frees [`CELL_LEN`] bytes. Must be
    /// deterministic in its inputs.
    fn victims(
        &self,
        entries: &[EvictionEntry],
        incoming: &EvictionEntry,
        need_bytes: usize,
    ) -> Vec<Hash32>;
}

/// Default policy: lowest-priority class first (never a class that
/// outranks the incoming cell), within a class L3 before L2 before L1,
/// then oldest first, then lowest TTL, then id; pinned entries are skipped
/// (BRIEF_v2 §5.5 "Buffers"; D4 BUFFER "Evict L3 first, then oldest or
/// lowest TTL"; DESIGN §1 `relay/buffers.rs`). Exactly as many victims as
/// `need_bytes` requires are returned.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LowestClassL3First;

impl EvictionPolicy for LowestClassL3First {
    fn victims(
        &self,
        entries: &[EvictionEntry],
        incoming: &EvictionEntry,
        need_bytes: usize,
    ) -> Vec<Hash32> {
        let need_cells = need_bytes.div_ceil(CELL_LEN);
        if need_cells == 0 {
            return Vec::new();
        }
        let mut cands: Vec<&EvictionEntry> = entries
            .iter()
            .filter(|e| !e.pinned && !e.class.outranks(incoming.class))
            .collect();
        cands.sort_by(|a, b| {
            b.class
                .cmp(&a.class)
                .then(b.lane.cmp(&a.lane))
                .then(a.first_seen_mono.cmp(&b.first_seen_mono))
                .then(a.ttl.cmp(&b.ttl))
                .then(a.id.cmp(&b.id))
        });
        cands.iter().take(need_cells).map(|e| e.id).collect()
    }
}

#[derive(Debug, Clone)]
struct Entry {
    cell: Cell,
    meta: CellMeta,
    /// Slot in the summary index and the slot tables.
    slot: usize,
}

/// Forwarding-order key: class (C0 first), then first seen, then id.
type OrderKey = (Class, u64, Hash32);

/// One forwardable cell as [`CellStore::scan_forwardable`] yields it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candidate<'a> {
    /// Dense index of the cell in the store's slot tables; reused after a
    /// removal, hence `generation`.
    pub slot: usize,
    /// The slot's generation while this cell occupies it (never 0).
    pub generation: u32,
    pub id: &'a Hash32,
    /// The cell's cached summary words.
    pub words: &'a IdWords,
}

/// The cell store. See the module docs for the rules.
pub struct CellStore {
    caps: StoreCaps,
    policy: Box<dyn EvictionPolicy>,
    entries: BTreeMap<Hash32, Entry>,
    order: BTreeMap<OrderKey, usize>,
    /// Slot tables parallel to the index: id and forwardability per slot.
    slot_ids: Vec<Hash32>,
    slot_fwd: Vec<bool>,
    index: SummaryIndex,
    salt_rotated_at: Option<u64>,
    salt_interval_ms: u64,
    per_class: [usize; Class::ALL.len()],
    per_link: BTreeMap<LinkId, usize>,
    evicted_total: u64,
}

impl fmt::Debug for CellStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CellStore")
            .field("caps", &self.caps)
            .field("len", &self.len())
            .field("bytes", &self.bytes())
            .field("per_class", &self.per_class)
            .field("per_link", &self.per_link)
            .field("evicted_total", &self.evicted_total)
            .field("summary_salt", &self.index.salt())
            .field("summary_capacity", &self.index.capacity())
            .finish_non_exhaustive()
    }
}

impl Default for CellStore {
    fn default() -> Self {
        Self::with_default_policy(StoreCaps::DEFAULT)
    }
}

fn class_index(class: Class) -> usize {
    usize::from(class.as_u8())
}

impl CellStore {
    /// A store with the given caps and eviction policy.
    pub fn new(caps: StoreCaps, policy: Box<dyn EvictionPolicy>) -> Self {
        CellStore {
            caps,
            policy,
            entries: BTreeMap::new(),
            order: BTreeMap::new(),
            slot_ids: Vec::new(),
            slot_fwd: Vec::new(),
            index: SummaryIndex::new(0, SUMMARY_TARGET_FPR_ASM),
            salt_rotated_at: None,
            salt_interval_ms: SUMMARY_SALT_ROTATE_MS_ASM,
            per_class: [0; Class::ALL.len()],
            per_link: BTreeMap::new(),
            evicted_total: 0,
        }
    }

    /// A store with the given caps and [`LowestClassL3First`].
    pub fn with_default_policy(caps: StoreCaps) -> Self {
        Self::new(caps, Box::new(LowestClassL3First))
    }

    pub fn caps(&self) -> StoreCaps {
        self.caps
    }

    /// Replaces the eviction policy (the relay installs `BufferPolicy`).
    pub fn set_policy(&mut self, policy: Box<dyn EvictionPolicy>) {
        self.policy = policy;
    }

    // ---- accounting ---------------------------------------------------------

    /// Number of cells held.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Bytes held (`len × CELL_LEN`).
    pub fn bytes(&self) -> usize {
        self.len().saturating_mul(CELL_LEN)
    }

    /// Bytes below the total cap; advertised in `Hello.buffer_free` (DESIGN
    /// §2.3).
    pub fn free_bytes(&self) -> usize {
        self.caps.max_bytes.saturating_sub(self.bytes())
    }

    pub fn len_for_class(&self, class: Class) -> usize {
        self.per_class.get(class_index(class)).copied().unwrap_or(0)
    }

    pub fn bytes_for_class(&self, class: Class) -> usize {
        self.len_for_class(class).saturating_mul(CELL_LEN)
    }

    /// Cells received on `link` (cells this phone originated count for no
    /// link).
    pub fn len_for_link(&self, link: LinkId) -> usize {
        self.per_link.get(&link).copied().unwrap_or(0)
    }

    pub fn bytes_for_link(&self, link: LinkId) -> usize {
        self.len_for_link(link).saturating_mul(CELL_LEN)
    }

    /// Cells evicted by the policy since the store was created.
    pub fn evicted_total(&self) -> u64 {
        self.evicted_total
    }

    // ---- lookup -------------------------------------------------------------

    pub fn contains(&self, id: &Hash32) -> bool {
        self.entries.contains_key(id)
    }

    pub fn get(&self, id: &Hash32) -> Option<(&Cell, &CellMeta)> {
        self.entries.get(id).map(|e| (&e.cell, &e.meta))
    }

    /// All ids in ascending id order.
    pub fn iter_ids(&self) -> impl Iterator<Item = Hash32> + '_ {
        self.entries.keys().copied()
    }

    /// All entries in ascending id order.
    pub fn iter(&self) -> impl Iterator<Item = (&Hash32, &Cell, &CellMeta)> + '_ {
        self.entries.iter().map(|(id, e)| (id, &e.cell, &e.meta))
    }

    /// The forwarding view of every entry, in forwarding order. The id
    /// comes from the order key (and is handed to `keep` as such): no cell
    /// is re-hashed here.
    fn eviction_entries(&self, keep: impl Fn(&Hash32, &Entry) -> bool) -> Vec<EvictionEntry> {
        self.order
            .keys()
            .filter_map(|(_, _, id)| self.entries.get(id).map(|e| (id, e)))
            .filter(|(id, e)| keep(id, e))
            .map(|(id, e)| EvictionEntry {
                id: *id,
                class: e.meta.class,
                lane: e.meta.lane,
                first_seen_mono: e.meta.first_seen_mono,
                pinned: e.meta.pinned,
                ttl: e.cell.ttl(),
            })
            .collect()
    }

    // ---- summary index ------------------------------------------------------

    /// The incremental summary of the store (module docs): every held
    /// cell's cached words and the filter under [`summary_salt`](Self::summary_salt).
    pub fn summary_index(&self) -> &SummaryIndex {
        &self.index
    }

    /// The salt the maintained filter is under (0 before the first
    /// rotation).
    pub fn summary_salt(&self) -> u32 {
        self.index.salt()
    }

    /// Powered-on time between salt rotations by
    /// [`summary_salt_for`](Self::summary_salt_for).
    pub fn summary_salt_interval_ms(&self) -> u64 {
        self.salt_interval_ms
    }

    pub fn set_summary_salt_interval_ms(&mut self, ms: u64) {
        self.salt_interval_ms = ms;
    }

    /// Re-derives the maintained filter under `salt` (`O(n·k)` integer
    /// operations, no hashing) and records `now_mono` as the rotation time.
    pub fn rotate_summary_salt(&mut self, salt: u32, now_mono: u64) {
        self.index.rotate(salt);
        self.salt_rotated_at = Some(now_mono);
    }

    /// The salt an encounter opening at `now_mono` should summarise under
    /// (module docs, salt rotation policy): `fresh` (drawn by the caller
    /// from its rng) when no rotation happened yet or the last one is at
    /// least [`summary_salt_interval_ms`](Self::summary_salt_interval_ms)
    /// of powered-on time ago, in which case the filter is re-derived
    /// under it; otherwise the current salt, whose filter is ready.
    pub fn summary_salt_for(&mut self, now_mono: u64, fresh: u32) -> u32 {
        let due = match self.salt_rotated_at {
            None => true,
            Some(at) => now_mono.saturating_sub(at) >= self.salt_interval_ms,
        };
        if due {
            self.rotate_summary_salt(fresh, now_mono);
        }
        self.index.salt()
    }

    /// The slot of a held cell.
    pub fn slot_of(&self, id: &Hash32) -> Option<usize> {
        self.entries.get(id).map(|e| e.slot)
    }

    /// The id in `slot`, if it is live.
    pub fn id_at(&self, slot: usize) -> Option<&Hash32> {
        if self.index.is_live(slot) {
            self.slot_ids.get(slot)
        } else {
            None
        }
    }

    /// The generation of `slot` (0 if it never held a cell); see
    /// [`Candidate`].
    pub fn generation(&self, slot: usize) -> u32 {
        self.index.generation(slot)
    }

    // ---- mutation -----------------------------------------------------------

    /// Stores `cell` under its derived `msg_id`. Over a cap, the policy is
    /// asked for victims (per-link pool first, then the whole store); the
    /// store applies only victims that are present, unpinned and not of a
    /// class that outranks `meta.class`. If those free less than needed,
    /// nothing is changed and [`Put::NoRoom`] is returned; otherwise the
    /// victims are removed and the cell stored, atomically from the
    /// caller's view.
    pub fn put(&mut self, cell: Cell, meta: CellMeta) -> Put {
        let id = cell.msg_id();
        if self.entries.contains_key(&id) {
            return Put::Duplicate;
        }
        let incoming = EvictionEntry {
            id,
            class: meta.class,
            lane: meta.lane,
            first_seen_mono: meta.first_seen_mono,
            pinned: meta.pinned,
            ttl: cell.ttl(),
        };
        let mut victims: BTreeSet<Hash32> = BTreeSet::new();

        if let Some(link) = meta.from_link {
            let have = self.bytes_for_link(link);
            if have.saturating_add(CELL_LEN) > self.caps.max_bytes_per_link {
                let need = have
                    .saturating_add(CELL_LEN)
                    .saturating_sub(self.caps.max_bytes_per_link);
                let pool = self.eviction_entries(|_, e| e.meta.from_link == Some(link));
                let chosen = self.policy.victims(&pool, &incoming, need);
                let freed = accept_victims(&pool, &chosen, incoming.class, &mut victims);
                if freed < need {
                    return Put::NoRoom;
                }
            }
        }

        let have = self
            .bytes()
            .saturating_sub(victims.len().saturating_mul(CELL_LEN));
        if have.saturating_add(CELL_LEN) > self.caps.max_bytes {
            let need = have
                .saturating_add(CELL_LEN)
                .saturating_sub(self.caps.max_bytes);
            let already = victims.clone();
            let pool = self.eviction_entries(|id, _| !already.contains(id));
            let chosen = self.policy.victims(&pool, &incoming, need);
            let freed = accept_victims(&pool, &chosen, incoming.class, &mut victims);
            if freed < need {
                return Put::NoRoom;
            }
        }

        for v in &victims {
            if self.remove(v).is_some() {
                self.evicted_total = self.evicted_total.saturating_add(1);
            }
        }
        self.insert(id, cell, meta);
        Put::Stored
    }

    fn insert(&mut self, id: Hash32, cell: Cell, meta: CellMeta) {
        let slot = self.index.insert(&id);
        if self.slot_ids.len() <= slot {
            let want = slot.saturating_add(1);
            self.slot_ids.resize(want, Hash32::ZERO);
            self.slot_fwd.resize(want, false);
        }
        if let (Some(i), Some(f)) = (self.slot_ids.get_mut(slot), self.slot_fwd.get_mut(slot)) {
            *i = id;
            *f = cell.is_forwardable();
        }
        self.order
            .insert((meta.class, meta.first_seen_mono, id), slot);
        if let Some(n) = self.per_class.get_mut(class_index(meta.class)) {
            *n = n.saturating_add(1);
        }
        if let Some(link) = meta.from_link {
            let n = self.per_link.entry(link).or_insert(0);
            *n = n.saturating_add(1);
        }
        self.entries.insert(id, Entry { cell, meta, slot });
    }

    /// Removes a cell (eviction, retention deletion, a superseded roster
    /// op). Returns what was held.
    pub fn remove(&mut self, id: &Hash32) -> Option<(Cell, CellMeta)> {
        let e = self.entries.remove(id)?;
        self.order
            .remove(&(e.meta.class, e.meta.first_seen_mono, *id));
        self.index.remove(e.slot);
        if let Some(n) = self.per_class.get_mut(class_index(e.meta.class)) {
            *n = n.saturating_sub(1);
        }
        if let Some(link) = e.meta.from_link {
            let drop = match self.per_link.get_mut(&link) {
                Some(n) => {
                    *n = n.saturating_sub(1);
                    *n == 0
                }
                None => false,
            };
            if drop {
                self.per_link.remove(&link);
            }
        }
        Some((e.cell, e.meta))
    }

    /// Pins or unpins a held cell (roster ops: pinned until superseded).
    /// Returns `false` if the id is not held.
    pub fn set_pinned(&mut self, id: &Hash32, pinned: bool) -> bool {
        match self.entries.get_mut(id) {
            Some(e) => {
                e.meta.pinned = pinned;
                true
            }
            None => false,
        }
    }

    // ---- forwarding ---------------------------------------------------------

    /// Every cell a relay may forward (`ttl ≥ 2`, DESIGN §2.2) in class
    /// priority order (C0 > C1 > C2 > C3 > C4, BRIEF_v2 §5.6) and oldest
    /// first within a class (ties by id), with its slot, generation and
    /// cached words: one ordered-map step and two table reads per cell, no
    /// lookup by id. The lane split of C2/C3 is the scheduler's job
    /// (`relay/sched.rs`).
    pub fn scan_forwardable(&self) -> impl Iterator<Item = Candidate<'_>> + '_ {
        self.order.iter().filter_map(move |((_, _, id), &slot)| {
            if !self.slot_fwd.get(slot).copied().unwrap_or(false) {
                return None;
            }
            let words = self.index.words(slot)?;
            Some(Candidate {
                slot,
                generation: self.index.generation(slot),
                id,
                words,
            })
        })
    }

    /// Up to `budget_cells` ids to offer a peer, in the order of
    /// [`scan_forwardable`](Self::scan_forwardable), skipping ids for which
    /// `exclude` is true.
    pub fn candidates_for(
        &self,
        exclude: &dyn Fn(&Hash32) -> bool,
        budget_cells: usize,
    ) -> Vec<Hash32> {
        if budget_cells == 0 {
            return Vec::new();
        }
        self.scan_forwardable()
            .filter(|c| !exclude(c.id))
            .map(|c| *c.id)
            .take(budget_cells)
            .collect()
    }
}

/// Filters a policy's choice down to what the store allows to go (present
/// in `pool`, unpinned, not outranking `incoming_class`), adds those to
/// `victims` and returns the bytes they free.
fn accept_victims(
    pool: &[EvictionEntry],
    chosen: &[Hash32],
    incoming_class: Class,
    victims: &mut BTreeSet<Hash32>,
) -> usize {
    let mut freed = 0usize;
    for id in chosen {
        let Some(e) = pool.iter().find(|e| e.id == *id) else {
            continue;
        };
        if e.pinned || e.class.outranks(incoming_class) {
            continue;
        }
        if victims.insert(*id) {
            freed = freed.saturating_add(CELL_LEN);
        }
    }
    freed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::cell::{BODY_LEN, TAG_LEN, VERSION, build};

    fn cell(seed: u64, ttl: u8) -> Cell {
        let mut body = [0u8; BODY_LEN];
        body[..8].copy_from_slice(&seed.to_le_bytes());
        build(VERSION, ttl, &[0x5A; TAG_LEN], &body)
    }

    fn meta(class: Class, lane: Lane, link: Option<u32>, t: u64) -> CellMeta {
        CellMeta {
            class,
            lane,
            from_link: link.map(LinkId),
            first_seen_mono: t,
            pinned: false,
        }
    }

    #[test]
    fn put_get_duplicate_remove_and_accounting() {
        let mut s = CellStore::with_default_policy(StoreCaps::cells(10, 10));
        let c = cell(1, 7);
        let id = c.msg_id();
        assert_eq!(
            s.put(c.clone(), meta(Class::C2Text, Lane::L1, Some(1), 5)),
            Put::Stored
        );
        // same cell, different TTL: same msg_id → duplicate
        let mut c2 = c.clone();
        c2.set_ttl(3);
        assert_eq!(
            s.put(c2, meta(Class::C2Text, Lane::L3, Some(2), 9)),
            Put::Duplicate
        );
        assert_eq!(s.len(), 1);
        assert_eq!(s.bytes(), CELL_LEN);
        assert_eq!(s.free_bytes(), 9 * CELL_LEN);
        assert_eq!(s.len_for_class(Class::C2Text), 1);
        assert_eq!(s.len_for_class(Class::C1Control), 0);
        assert_eq!(s.len_for_link(LinkId(1)), 1);
        assert_eq!(s.bytes_for_link(LinkId(2)), 0);
        let (got, m) = s.get(&id).expect("held");
        assert_eq!(got, &c);
        assert_eq!(m.first_seen_mono, 5);
        assert!(s.contains(&id));
        assert_eq!(s.remove(&id).map(|(c, _)| c.ttl()), Some(7));
        assert!(s.is_empty());
        assert_eq!(s.len_for_link(LinkId(1)), 0);
        assert_eq!(s.len_for_class(Class::C2Text), 0);
        assert!(s.remove(&id).is_none());
    }

    #[test]
    fn candidates_follow_class_then_age_and_skip_dead_or_excluded() {
        let mut s = CellStore::with_default_policy(StoreCaps::cells(10, 10));
        let old_text = cell(1, 6);
        let new_text = cell(2, 6);
        let control = cell(3, 6);
        let dead = cell(4, 1);
        s.put(new_text.clone(), meta(Class::C2Text, Lane::L1, None, 20));
        s.put(old_text.clone(), meta(Class::C2Text, Lane::L3, None, 10));
        s.put(control.clone(), meta(Class::C1Control, Lane::L3, None, 30));
        s.put(dead.clone(), meta(Class::C0Sos, Lane::L1, None, 0));
        let all = s.candidates_for(&|_| false, 10);
        assert_eq!(
            all,
            vec![control.msg_id(), old_text.msg_id(), new_text.msg_id()]
        );
        let budget = s.candidates_for(&|_| false, 2);
        assert_eq!(budget, vec![control.msg_id(), old_text.msg_id()]);
        let skip = old_text.msg_id();
        let excl = s.candidates_for(&|id| *id == skip, 10);
        assert_eq!(excl, vec![control.msg_id(), new_text.msg_id()]);
        assert!(s.candidates_for(&|_| false, 0).is_empty());
    }

    #[test]
    fn eviction_takes_lowest_class_l3_oldest_and_respects_pins_and_rank() {
        let mut s = CellStore::with_default_policy(StoreCaps::cells(3, 3));
        let a = cell(1, 6); // C3, L1, oldest
        let b = cell(2, 6); // C3, L3, newer  → first victim for a C2 arrival
        let c = cell(3, 6); // C1 pinned
        assert_eq!(
            s.put(a.clone(), meta(Class::C3News, Lane::L1, None, 1)),
            Put::Stored
        );
        assert_eq!(
            s.put(b.clone(), meta(Class::C3News, Lane::L3, None, 2)),
            Put::Stored
        );
        let mut pinned = meta(Class::C1Control, Lane::L1, None, 3);
        pinned.pinned = true;
        assert_eq!(s.put(c.clone(), pinned), Put::Stored);

        let d = cell(4, 6);
        assert_eq!(
            s.put(d.clone(), meta(Class::C2Text, Lane::L1, None, 4)),
            Put::Stored
        );
        assert!(!s.contains(&b.msg_id()), "L3 goes first within the class");
        assert!(s.contains(&a.msg_id()));
        assert!(s.contains(&c.msg_id()));
        assert_eq!(s.evicted_total(), 1);

        // a C4 arrival may only evict C4: nothing qualifies → NoRoom
        let e = cell(5, 6);
        assert_eq!(
            s.put(e.clone(), meta(Class::C4Preview, Lane::L1, None, 5)),
            Put::NoRoom
        );
        assert_eq!(s.len(), 3);
        assert!(!s.contains(&e.msg_id()));

        // a C0 arrival evicts the lowest class present (C3 a), never the pin
        let f = cell(6, 6);
        assert_eq!(
            s.put(f.clone(), meta(Class::C0Sos, Lane::L1, None, 6)),
            Put::Stored
        );
        assert!(!s.contains(&a.msg_id()));
        assert!(s.contains(&c.msg_id()));
        assert!(s.contains(&d.msg_id()));
    }

    #[test]
    fn pinned_everything_means_no_room_and_unpin_frees() {
        let mut s = CellStore::with_default_policy(StoreCaps::cells(1, 1));
        let a = cell(1, 6);
        let mut m = meta(Class::C1Control, Lane::L1, None, 1);
        m.pinned = true;
        assert_eq!(s.put(a.clone(), m), Put::Stored);
        let b = cell(2, 6);
        assert_eq!(
            s.put(b.clone(), meta(Class::C1Control, Lane::L1, None, 2)),
            Put::NoRoom
        );
        assert!(s.set_pinned(&a.msg_id(), false));
        assert!(!s.set_pinned(&b.msg_id(), false));
        assert_eq!(
            s.put(b.clone(), meta(Class::C1Control, Lane::L1, None, 2)),
            Put::Stored
        );
        assert!(!s.contains(&a.msg_id()));
        assert!(s.contains(&b.msg_id()));
    }

    #[test]
    fn per_link_cap_evicts_within_the_link_only() {
        let mut s = CellStore::with_default_policy(StoreCaps::cells(10, 2));
        let l1a = cell(1, 6);
        let l1b = cell(2, 6);
        let l2a = cell(3, 6);
        s.put(l1a.clone(), meta(Class::C2Text, Lane::L3, Some(1), 1));
        s.put(l1b.clone(), meta(Class::C2Text, Lane::L3, Some(1), 2));
        s.put(l2a.clone(), meta(Class::C4Preview, Lane::L3, Some(2), 0));
        let l1c = cell(4, 6);
        assert_eq!(
            s.put(l1c.clone(), meta(Class::C2Text, Lane::L1, Some(1), 3)),
            Put::Stored
        );
        assert!(!s.contains(&l1a.msg_id()), "oldest of link 1 goes");
        assert!(s.contains(&l1b.msg_id()));
        assert!(s.contains(&l2a.msg_id()), "link 2's lower class untouched");
        assert_eq!(s.len_for_link(LinkId(1)), 2);
        // own cells (no link) ignore the per-link cap
        let own = cell(5, 6);
        assert_eq!(
            s.put(own, meta(Class::C2Text, Lane::L1, None, 4)),
            Put::Stored
        );
        assert_eq!(s.len(), 4);
    }

    #[test]
    fn store_is_deterministic_in_its_inputs() {
        let run = || {
            let mut s = CellStore::with_default_policy(StoreCaps::cells(4, 4));
            for i in 0..12u64 {
                let class = Class::ALL[(i % 5) as usize];
                let lane = Lane::ALL[(i % 3) as usize];
                s.put(cell(i, 6), meta(class, lane, Some(1), 100 - i));
            }
            (
                s.iter_ids().collect::<Vec<_>>(),
                s.candidates_for(&|_| false, 10),
            )
        };
        assert_eq!(run(), run());
    }

    #[test]
    fn debug_impl_exists() {
        let s = CellStore::default();
        assert!(format!("{s:?}").contains("CellStore"));
        assert_eq!(s.caps(), StoreCaps::DEFAULT);
    }
}
