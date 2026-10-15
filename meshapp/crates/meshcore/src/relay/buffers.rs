//! Bounded relay buffers (BRIEF_v2 §5.5 "Buffers: bounded, with per-link
//! and per-class quotas; within a class L3 is evicted first; a lower class
//! in the §5.6 order never evicts a higher one"; DECISIONS D4 BUFFER
//! "50 MB, at most 5 MB per inbound link, evict L3 first then oldest or
//! lowest TTL, reserve at least 20 % for traffic queued for absent
//! (pocketed) peers"; D3 M1 roster ops never evicted until superseded;
//! DESIGN §1 `relay/buffers.rs`).
//!
//! [`BufferPolicy`] is pure. The DTN store ([`crate::dtn::store::CellStore`])
//! owns the rows and asks it two questions: [`BufferPolicy::fits`] before
//! admitting a cell from a link (this is where the absent-peer reserve
//! lives; the store itself only knows the two hard caps in
//! [`StoreCaps`]) and [`EvictionPolicy::victims`] when a cap would be
//! exceeded. Every row is one fixed [`CELL_LEN`]-byte cell (DESIGN §1
//! `dtn/store.rs` "fixed rows"), so byte needs convert to whole cells.

use crate::crypto::hash::Hash32;
use crate::dtn::store::{
    EvictionEntry, EvictionPolicy, STORE_MAX_BYTES_ASM, STORE_MAX_BYTES_PER_LINK_ASM, StoreCaps,
};
use crate::wire::cell::CELL_LEN;
use crate::wire::frame::Class;

/// Whole relay buffer: D4 BUFFER "50 MB" (assumption; MiB). One value with
/// the store's cap.
pub const TOTAL_CAP_BYTES_ASM: usize = STORE_MAX_BYTES_ASM;
/// Per inbound link: D4 BUFFER "at most 5 MB per inbound link" (assumption;
/// MiB). One value with the store's cap.
pub const PER_LINK_CAP_BYTES_ASM: usize = STORE_MAX_BYTES_PER_LINK_ASM;
/// Fraction of the buffer kept for cells held for absent (pocketed) peers:
/// D4 BUFFER "reserve at least 20 %" (assumption).
pub const ABSENT_PEER_RESERVE_ASM: f32 = 0.20;

/// Buffer caps, the absent-peer reserve and the eviction order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BufferPolicy {
    /// Whole buffer, bytes.
    pub total_cap_bytes: usize,
    /// Per inbound link, bytes.
    pub per_link_cap_bytes: usize,
    /// Fraction of `total_cap_bytes` only absent-peer traffic may use, in
    /// [0, 1].
    pub absent_peer_reserve: f32,
}

impl Default for BufferPolicy {
    fn default() -> Self {
        BufferPolicy {
            total_cap_bytes: TOTAL_CAP_BYTES_ASM,
            per_link_cap_bytes: PER_LINK_CAP_BYTES_ASM,
            absent_peer_reserve: ABSENT_PEER_RESERVE_ASM,
        }
    }
}

/// `fraction` of `bytes`, rounded down, without floating point on the size
/// (the fraction is a configuration constant, so it is first rounded to a
/// per-mille integer).
fn fraction_of(bytes: usize, fraction: f32) -> usize {
    let scaled = (fraction.clamp(0.0, 1.0) * 1000.0).round();
    // `scaled` is in [0, 1000] (NaN clamps to NaN and casts to 0).
    let permille = scaled as usize;
    bytes / 1000 * permille + (bytes % 1000) * permille / 1000
}

impl BufferPolicy {
    /// The hard caps for [`crate::dtn::store::CellStore::new`].
    pub fn caps(&self) -> StoreCaps {
        StoreCaps {
            max_bytes: self.total_cap_bytes,
            max_bytes_per_link: self.per_link_cap_bytes,
        }
    }

    /// Bytes kept for absent-peer traffic.
    pub fn reserve_bytes(&self) -> usize {
        fraction_of(self.total_cap_bytes, self.absent_peer_reserve)
    }

    /// Ceiling for ordinary inbound relay traffic: the buffer minus the
    /// absent-peer reserve.
    pub fn general_cap_bytes(&self) -> usize {
        self.total_cap_bytes.saturating_sub(self.reserve_bytes())
    }

    /// Whole buffer in cells.
    pub fn max_cells(&self) -> usize {
        self.total_cap_bytes / CELL_LEN
    }

    /// `true` if `incoming_bytes` from a link fits without eviction:
    /// `total_bytes` (whole buffer, after) stays within the general cap and
    /// `link_bytes` (that link's share, after) within the per-link cap.
    pub fn fits(&self, total_bytes: usize, link_bytes: usize, incoming_bytes: usize) -> bool {
        total_bytes.saturating_add(incoming_bytes) <= self.general_cap_bytes()
            && link_bytes.saturating_add(incoming_bytes) <= self.per_link_cap_bytes
    }

    /// `true` if a cell held for an absent peer fits: such traffic may use
    /// the reserve too (D4 BUFFER), and has no inbound link to count
    /// against.
    pub fn fits_for_absent_peer(&self, total_bytes: usize, incoming_bytes: usize) -> bool {
        total_bytes.saturating_add(incoming_bytes) <= self.total_cap_bytes
    }

    /// Victims for a newcomer of class `newcomer`: only rows the newcomer
    /// does not outrank are eligible (BRIEF_v2 §5.5 "a lower class never
    /// evicts a higher one"), pinned rows never are, and the rest go in
    /// eviction order (lowest class first; within a class L3, then L2, then
    /// L1; then oldest `first_seen_mono`; then lowest TTL; then id) until
    /// `need_bytes` is covered. When that is impossible the result is what
    /// could be freed and the caller rejects the newcomer.
    pub fn victims_for(
        &self,
        entries: &[EvictionEntry],
        need_bytes: usize,
        newcomer: Class,
    ) -> Vec<Hash32> {
        let need_cells = need_bytes.div_ceil(CELL_LEN);
        if need_cells == 0 {
            return Vec::new();
        }
        let mut candidates: Vec<&EvictionEntry> = entries
            .iter()
            .filter(|e| !e.pinned && !e.class.outranks(newcomer))
            .collect();
        candidates.sort_by(|a, b| Self::eviction_order(a, b));
        candidates
            .into_iter()
            .take(need_cells)
            .map(|e| e.id)
            .collect()
    }

    /// The eviction order (first = evicted first): lowest-priority class,
    /// then L3 before L2 before L1, then oldest, then lowest TTL, then id
    /// so the order is total and the same on every phone.
    pub fn eviction_order(a: &EvictionEntry, b: &EvictionEntry) -> std::cmp::Ordering {
        b.class
            .cmp(&a.class)
            .then(b.lane.cmp(&a.lane))
            .then(a.first_seen_mono.cmp(&b.first_seen_mono))
            .then(a.ttl.cmp(&b.ttl))
            .then(a.id.cmp(&b.id))
    }
}

impl EvictionPolicy for BufferPolicy {
    fn victims(
        &self,
        entries: &[EvictionEntry],
        incoming: &EvictionEntry,
        need_bytes: usize,
    ) -> Vec<Hash32> {
        self.victims_for(entries, need_bytes, incoming.class)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relay::Lane;

    fn id(n: u8) -> Hash32 {
        Hash32::from([n; 32])
    }

    fn row(
        n: u8,
        class: Class,
        lane: Lane,
        ttl: u8,
        first_seen: u64,
        pinned: bool,
    ) -> EvictionEntry {
        EvictionEntry {
            id: id(n),
            class,
            lane,
            first_seen_mono: first_seen,
            pinned,
            ttl,
        }
    }

    #[test]
    fn caps_and_reserve() {
        let p = BufferPolicy::default();
        assert_eq!(p.caps(), StoreCaps::DEFAULT);
        assert_eq!(p.reserve_bytes(), TOTAL_CAP_BYTES_ASM / 5);
        assert_eq!(
            p.general_cap_bytes(),
            TOTAL_CAP_BYTES_ASM - TOTAL_CAP_BYTES_ASM / 5
        );
        assert!(p.fits(0, 0, CELL_LEN));
        assert!(p.fits(p.general_cap_bytes() - CELL_LEN, 0, CELL_LEN));
        assert!(!p.fits(p.general_cap_bytes() - CELL_LEN + 1, 0, CELL_LEN));
        assert!(!p.fits(0, PER_LINK_CAP_BYTES_ASM - CELL_LEN + 1, CELL_LEN));
        assert!(p.fits_for_absent_peer(p.general_cap_bytes(), CELL_LEN));
        assert!(!p.fits_for_absent_peer(TOTAL_CAP_BYTES_ASM - CELL_LEN + 1, CELL_LEN));
        assert!(!p.fits(usize::MAX, 0, 1));
        let odd = BufferPolicy {
            total_cap_bytes: 1_001,
            per_link_cap_bytes: 1,
            absent_peer_reserve: f32::NAN,
        };
        assert_eq!(odd.reserve_bytes(), 0);
        let all = BufferPolicy {
            absent_peer_reserve: 2.0,
            ..BufferPolicy::default()
        };
        assert_eq!(all.general_cap_bytes(), 0);
    }

    #[test]
    fn eviction_order_lowest_class_then_l3_then_oldest_then_ttl() {
        let p = BufferPolicy::default();
        let rows = [
            row(1, Class::C2Text, Lane::L1, 7, 10, false),
            row(2, Class::C2Text, Lane::L3, 7, 50, false),
            row(3, Class::C4Preview, Lane::L1, 2, 90, false),
            row(4, Class::C1Control, Lane::L3, 1, 1, true),
            row(5, Class::C2Text, Lane::L3, 3, 50, false),
            row(6, Class::C2Text, Lane::L3, 7, 20, false),
            row(7, Class::C1Control, Lane::L3, 7, 5, false),
        ];
        let all = p.victims_for(&rows, 7 * CELL_LEN, Class::C0Sos);
        assert_eq!(all, vec![id(3), id(6), id(5), id(2), id(1), id(7)]);
        // A C2 newcomer cannot touch the C1 row even if the need is unmet.
        let for_text = p.victims_for(&rows, 7 * CELL_LEN, Class::C2Text);
        assert_eq!(for_text, vec![id(3), id(6), id(5), id(2), id(1)]);
        // Need is in bytes, rounded up to cells.
        assert_eq!(p.victims_for(&rows, 1, Class::C2Text), vec![id(3)]);
        assert_eq!(
            p.victims_for(&rows, CELL_LEN + 1, Class::C2Text),
            vec![id(3), id(6)]
        );
        assert!(p.victims_for(&rows, 0, Class::C4Preview).is_empty());
        // A C4 newcomer can only displace C4.
        assert_eq!(
            p.victims_for(&rows, 3 * CELL_LEN, Class::C4Preview),
            vec![id(3)]
        );
        // The trait entry point keys on the incoming cell's class.
        let incoming = row(9, Class::C2Text, Lane::L3, 8, 100, false);
        assert_eq!(p.victims(&rows, &incoming, 7 * CELL_LEN), for_text);
    }
}
