//! The `msg_id` dedupe cache (docs/DESIGN.md §1 row `dtn/dedupe.rs`; §6
//! `dedupe(msg_id PK, expires_mono)` and "Dedupe retention"; BRIEF_v2 §5.5
//! "Every relay … keeps a dedupe cache of message-id hashes for
//! max(message retention, key retention) + 72 h"; DECISIONS §3 row (4,6,7)
//! and row (1,2,3,4,10) (iv)).
//!
//! Rules implemented here:
//!
//! * **Seen-before test** ([`DedupeCache::seen_or_insert`]): the relay
//!   pipeline (DESIGN §1 `node/inbound.rs`: "cell → dedupe → store") asks
//!   once per received cell; `true` means drop it (BRIEF_v2 §5.5 "replays
//!   are dropped by dedupe/AEAD"; DESIGN §10 S3 (d)).
//! * **Retention in powered-on time** (BRIEF_v2 §4 "no timestamps decide
//!   anything"; DESIGN §6 "all expiries use the powered-on meter"): an
//!   entry expires `retention_ms` after its first sighting; `now_mono` is a
//!   powered-on millisecond count from [`crate::clock::Clock`]. The expiry
//!   is fixed at insertion (DESIGN §6 stores one `expires_mono` per id) so
//!   re-sightings do not extend it.
//! * **Retention value** (§3 row (4,6,7): "max(retention, key retention)
//!   plus 72 h of carry time"): the caller computes it with
//!   [`dedupe_retention_ms`]; [`DEDUPE_RETENTION_ASM_MS`] is the A-build
//!   value (DESIGN §6: chats unbounded, so the cache is bounded by the key
//!   retention of 7 d plus 72 h = 10 d).
//! * **Rejected items are remembered** (§3 row (1,2,3,4,10) (iv) "summary
//!   vectors carry the hashes of rejected items so they are not
//!   re-offered"; DESIGN §3 "its msg_id is added to the 'rejected' set"):
//!   [`DedupeCache::mark_rejected`], [`is_rejected`](DedupeCache::is_rejected)
//!   and [`rejected_ids`](DedupeCache::rejected_ids), which the summary
//!   builder in `dtn/sync.rs` folds into the local vector.
//! * **Bounded** (`max_entries`): when full, the entry closest to expiry
//!   (the oldest) is dropped first, so memory is capped even if `expire`
//!   is never called.
//!
//! The cache is deterministic: the same calls with the same `now_mono`
//! values give the same answers.

use std::collections::{BTreeMap, BTreeSet};

use crate::clock::ms;
use crate::crypto::hash::Hash32;

/// DTN carry time added to every retention window (BRIEF_v2 §5.5 "+ 72 h";
/// §3 row (4,6,7) "72 h of carry time").
pub const DEDUPE_CARRY_MS: u64 = 72 * ms::HOUR;
/// Key retention assumption (BRIEF_v2 §5.5 "(assumption: 7 days)"; DESIGN
/// §6 `KEY_RETENTION_ASM = 7 d`).
pub const KEY_RETENTION_ASM_MS: u64 = 7 * ms::DAY;
/// Dedupe retention for the A build: `max(MSG_RETENTION, KEY_RETENTION_ASM)
/// + 72 h` with chats unbounded (so the message term contributes nothing
/// and the cache is bounded by the key retention), i.e. `7 d + 72 h = 10 d`
/// (DESIGN §6 "A: … 10 d"). The B build passes `72 h + 7 d` itself.
pub const DEDUPE_RETENTION_ASM_MS: u64 = KEY_RETENTION_ASM_MS + DEDUPE_CARRY_MS;
/// Default entry bound: 2^18 ids (≈ 8 MB at 32 B plus map overhead), well
/// above a 10-day A-build backlog at BLE rates (assumption, retuned by the
/// spike).
pub const DEDUPE_MAX_ENTRIES_ASM: usize = 1 << 18;

const _: () = assert!(DEDUPE_RETENTION_ASM_MS == 10 * ms::DAY);

/// `max(message retention, key retention) + 72 h` (BRIEF_v2 §5.5; §3 row
/// (4,6,7)). `message_retention_ms = None` means messages are never deleted
/// for age (A build chats), so only the key retention bounds the window
/// (DESIGN §6).
pub fn dedupe_retention_ms(message_retention_ms: Option<u64>, key_retention_ms: u64) -> u64 {
    message_retention_ms
        .unwrap_or(0)
        .max(key_retention_ms)
        .saturating_add(DEDUPE_CARRY_MS)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Entry {
    expires_mono: u64,
    rejected: bool,
}

/// The dedupe cache. See the module docs.
#[derive(Debug, Clone)]
pub struct DedupeCache {
    retention_ms: u64,
    max_entries: usize,
    entries: BTreeMap<Hash32, Entry>,
    /// `(expires_mono, id)`: expiry order, oldest first.
    by_expiry: BTreeSet<(u64, Hash32)>,
    rejected: BTreeSet<Hash32>,
}

impl DedupeCache {
    /// A cache keeping ids for `retention_ms` of powered-on time, holding at
    /// most `max_entries` ids (at least 1).
    pub fn new(retention_ms: u64, max_entries: usize) -> Self {
        DedupeCache {
            retention_ms,
            max_entries: max_entries.max(1),
            entries: BTreeMap::new(),
            by_expiry: BTreeSet::new(),
            rejected: BTreeSet::new(),
        }
    }

    /// The A-build defaults: [`DEDUPE_RETENTION_ASM_MS`],
    /// [`DEDUPE_MAX_ENTRIES_ASM`].
    pub fn with_defaults() -> Self {
        Self::new(DEDUPE_RETENTION_ASM_MS, DEDUPE_MAX_ENTRIES_ASM)
    }

    pub fn retention_ms(&self) -> u64 {
        self.retention_ms
    }

    pub fn max_entries(&self) -> usize {
        self.max_entries
    }

    /// Ids currently remembered (expired ones included until swept).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Ids remembered as rejected.
    pub fn rejected_len(&self) -> usize {
        self.rejected.len()
    }

    /// Whether `id` is remembered (no expiry check; see
    /// [`expire`](Self::expire)).
    pub fn contains(&self, id: &Hash32) -> bool {
        self.entries.contains_key(id)
    }

    /// When `id` expires, if remembered.
    pub fn expires_at(&self, id: &Hash32) -> Option<u64> {
        self.entries.get(id).map(|e| e.expires_mono)
    }

    /// `true` if `id` was seen before and has not expired at `now_mono`
    /// (the relay drops the cell); otherwise remembers it until
    /// `now_mono + retention` and returns `false`. An entry whose window
    /// has passed counts as unseen and is re-armed.
    pub fn seen_or_insert(&mut self, id: Hash32, now_mono: u64) -> bool {
        if let Some(e) = self.entries.get(&id) {
            if e.expires_mono > now_mono {
                return true;
            }
            self.drop_entry(&id);
        }
        self.insert_new(id, now_mono, false);
        false
    }

    /// Remembers `id` as rejected (§3 row (1,2,3,4,10) (iv)): a post whose
    /// heads include its author's removal, a bad signature, a never-member.
    /// Inserts the id if it was unknown; keeps the existing expiry if it
    /// was known.
    pub fn mark_rejected(&mut self, id: Hash32, now_mono: u64) {
        match self.entries.get_mut(&id) {
            Some(e) if e.expires_mono > now_mono => {
                e.rejected = true;
                self.rejected.insert(id);
            }
            Some(_) => {
                self.drop_entry(&id);
                self.insert_new(id, now_mono, true);
            }
            None => self.insert_new(id, now_mono, true),
        }
    }

    pub fn is_rejected(&self, id: &Hash32) -> bool {
        self.rejected.contains(id)
    }

    /// Rejected ids in ascending order, for the summary vector (so peers
    /// stop re-offering them).
    pub fn rejected_ids(&self) -> impl Iterator<Item = Hash32> + '_ {
        self.rejected.iter().copied()
    }

    /// Drops every entry whose window has passed at `now_mono` (powered-on
    /// time) and returns how many went. Called on the powered-on tick.
    pub fn expire(&mut self, now_mono: u64) -> usize {
        let mut dropped = 0usize;
        while let Some(&(exp, id)) = self.by_expiry.iter().next() {
            if exp > now_mono {
                break;
            }
            self.drop_entry(&id);
            dropped = dropped.saturating_add(1);
        }
        dropped
    }

    fn insert_new(&mut self, id: Hash32, now_mono: u64, rejected: bool) {
        while self.entries.len() >= self.max_entries {
            match self.by_expiry.iter().next().map(|(_, id)| *id) {
                Some(oldest) => self.drop_entry(&oldest),
                None => break,
            }
        }
        let expires_mono = now_mono.saturating_add(self.retention_ms);
        self.entries.insert(
            id,
            Entry {
                expires_mono,
                rejected,
            },
        );
        self.by_expiry.insert((expires_mono, id));
        if rejected {
            self.rejected.insert(id);
        }
    }

    fn drop_entry(&mut self, id: &Hash32) {
        if let Some(e) = self.entries.remove(id) {
            self.by_expiry.remove(&(e.expires_mono, *id));
            self.rejected.remove(id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hash::sha256;

    fn id(n: u64) -> Hash32 {
        sha256(&n.to_le_bytes())
    }

    #[test]
    fn retention_formula() {
        assert_eq!(DEDUPE_RETENTION_ASM_MS, 10 * ms::DAY);
        assert_eq!(
            dedupe_retention_ms(None, KEY_RETENTION_ASM_MS),
            DEDUPE_RETENTION_ASM_MS
        );
        // B build: 72 h chats, 7 d keys → 7 d + 72 h
        assert_eq!(
            dedupe_retention_ms(Some(72 * ms::HOUR), KEY_RETENTION_ASM_MS),
            10 * ms::DAY
        );
        // longer message retention wins
        assert_eq!(
            dedupe_retention_ms(Some(30 * ms::DAY), KEY_RETENTION_ASM_MS),
            33 * ms::DAY
        );
        assert_eq!(dedupe_retention_ms(Some(u64::MAX), 1), u64::MAX);
    }

    #[test]
    fn seen_or_insert_and_powered_on_expiry() {
        let mut c = DedupeCache::new(100, 10);
        assert!(!c.seen_or_insert(id(1), 0));
        assert!(c.seen_or_insert(id(1), 0));
        assert!(c.seen_or_insert(id(1), 99), "still inside the window");
        assert_eq!(c.expires_at(&id(1)), Some(100));
        assert!(c.contains(&id(1)));
        // the window passes without a sweep: treated as unseen and re-armed
        assert!(!c.seen_or_insert(id(1), 100));
        assert_eq!(c.expires_at(&id(1)), Some(200));
        assert_eq!(c.len(), 1);
        // the sweep drops only what has expired
        assert!(!c.seen_or_insert(id(2), 150));
        assert_eq!(c.expire(199), 0);
        assert_eq!(c.expire(200), 1);
        assert!(!c.contains(&id(1)));
        assert!(c.contains(&id(2)));
        assert_eq!(c.expire(250), 1);
        assert!(c.is_empty());
    }

    #[test]
    fn rejected_ids_are_remembered_and_listed() {
        let mut c = DedupeCache::new(100, 10);
        c.mark_rejected(id(3), 10);
        assert!(c.is_rejected(&id(3)));
        assert!(c.contains(&id(3)));
        assert!(c.seen_or_insert(id(3), 20), "a rejected id is a seen id");
        assert!(!c.is_rejected(&id(4)));
        c.seen_or_insert(id(4), 20);
        c.mark_rejected(id(4), 30);
        assert!(c.is_rejected(&id(4)));
        assert_eq!(c.expires_at(&id(4)), Some(120), "existing expiry kept");
        let mut listed: Vec<Hash32> = c.rejected_ids().collect();
        listed.sort();
        let mut want = vec![id(3), id(4)];
        want.sort();
        assert_eq!(listed, want);
        assert_eq!(c.rejected_len(), 2);
        // rejection expires with the entry
        assert_eq!(c.expire(110), 1);
        assert!(!c.is_rejected(&id(3)));
        assert!(c.is_rejected(&id(4)));
        // marking an expired id re-arms it as rejected
        c.mark_rejected(id(4), 500);
        assert_eq!(c.expires_at(&id(4)), Some(600));
        assert!(c.is_rejected(&id(4)));
    }

    #[test]
    fn bounded_evicts_oldest_first() {
        let mut c = DedupeCache::new(1_000, 3);
        for i in 0..3 {
            assert!(!c.seen_or_insert(id(i), i));
        }
        assert_eq!(c.len(), 3);
        assert!(!c.seen_or_insert(id(3), 3));
        assert_eq!(c.len(), 3);
        assert!(!c.contains(&id(0)), "oldest went");
        assert!(c.contains(&id(1)) && c.contains(&id(2)) && c.contains(&id(3)));
        // a rejected id is bounded like any other
        c.mark_rejected(id(4), 4);
        assert!(!c.contains(&id(1)));
        assert_eq!(c.rejected_len(), 1);
        c.seen_or_insert(id(5), 5);
        c.seen_or_insert(id(6), 6);
        c.seen_or_insert(id(7), 7);
        assert_eq!(c.rejected_len(), 0, "evicted rejected id is forgotten");
        assert_eq!(c.len(), 3);
        assert_eq!(DedupeCache::new(10, 0).max_entries(), 1);
    }

    #[test]
    fn defaults_and_accessors() {
        let c = DedupeCache::with_defaults();
        assert_eq!(c.retention_ms(), DEDUPE_RETENTION_ASM_MS);
        assert_eq!(c.max_entries(), DEDUPE_MAX_ENTRIES_ASM);
        assert!(c.is_empty());
        assert_eq!(c.expires_at(&id(1)), None);
    }

    #[test]
    fn saturates_at_the_end_of_time() {
        let mut c = DedupeCache::new(u64::MAX, 2);
        assert!(!c.seen_or_insert(id(1), u64::MAX - 1));
        assert_eq!(c.expires_at(&id(1)), Some(u64::MAX));
        assert!(c.seen_or_insert(id(1), u64::MAX - 1));
        assert_eq!(c.expire(u64::MAX), 1);
    }
}
