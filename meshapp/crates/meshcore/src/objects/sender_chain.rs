//! Per-author hash chain of identity-signed objects (docs/DESIGN.md §2.4
//! "`author_seq`/`prev_own` form the author's per-sender hash chain";
//! DECISIONS.md D2 rule 5 "every member's signed messages are hash-chained
//! per sender"; BRIEF_v2 §4 votes).
//!
//! Every identity-signed object (roster ops, vote-opens) carries `(author,
//! author_seq, prev_own, heads)`. This index records what each author has
//! signed so that
//!
//! * **equivocation** is provable: two objects with the same `(author,
//!   author_seq)` and different hashes mean the author signed two histories;
//!   callers ignore both (DESIGN §3 "any" row: "both ignored, event
//!   `Equivocation(author)`"). At most two hashes per slot are kept as the
//!   proof; further ones are dropped (D1's "keep ≤2 payloads as proof");
//! * the **vote-open fork rule** has its evidence (DESIGN §3 "Vote-open fork
//!   rule hook"; DECISIONS §3 row (1,2,3,7) "Evidence for V2's stale-head fork
//!   rule is the roster heads the opener carried in its own earlier posts"):
//!   [`SenderChainIndex::heads_cited_before`] returns every head an author
//!   cited in objects with a lower sequence number;
//! * **gaps** are visible: an object whose sequence number jumps past the
//!   next expected one tells the caller that earlier objects are still in
//!   flight (anti-entropy fetches them; the object itself is still
//!   recorded).
//!
//! Memory is bounded: at most [`SenderChainIndex::max_authors`] authors and
//! [`SenderChainIndex::max_per_author`] entries per author. When an author
//! overflows, the lowest sequence numbers are forgotten (they are the oldest
//! objects, and roster ops themselves are never evicted from the DAG, D3 M1).
//! When the author table overflows, the author touched least recently (by
//! observation order, never by time) is forgotten. Nothing here reads a clock.

use std::collections::BTreeMap;

use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::Hash32;

/// Default cap on distinct authors (50 members per group, several groups).
pub const DEFAULT_MAX_AUTHORS: usize = 1024;
/// Default cap on recorded objects per author.
pub const DEFAULT_MAX_PER_AUTHOR: usize = 4096;

/// What the index concluded about one observed object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainVerdict {
    /// Consistent with everything seen from this author so far (or an exact
    /// repeat of an object already recorded).
    Ok,
    /// Another object by the same author with the same `author_seq` and a
    /// different hash is known; `other` is the first one recorded. Callers
    /// ignore both objects (DESIGN §3).
    Equivocation { other: Hash32 },
    /// The sequence number jumps past the next expected one
    /// (`expected_seq`): earlier objects have not been seen yet. The object
    /// is recorded anyway.
    Gap { expected_seq: u32 },
}

/// One recorded object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainEntry {
    pub hash: Hash32,
    pub prev_own: Hash32,
    /// Roster heads the object cited (sorted, deduplicated).
    pub heads: Vec<Hash32>,
}

/// One author's recorded chain.
#[derive(Debug, Clone, Default)]
struct AuthorChain {
    entries: BTreeMap<u32, ChainEntry>,
    /// `seq → (first hash, second hash)` for slots the author equivocated on.
    equivocations: BTreeMap<u32, (Hash32, Hash32)>,
    /// Highest sequence number ever observed, plus one (saturating).
    next_expected: u32,
    /// Observation counter at the last touch, for deterministic eviction.
    last_touch: u64,
}

/// The index over every author (see the module docs).
#[derive(Debug, Clone)]
pub struct SenderChainIndex {
    authors: BTreeMap<IdentityKey, AuthorChain>,
    max_authors: usize,
    max_per_author: usize,
    ticks: u64,
}

impl Default for SenderChainIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl SenderChainIndex {
    /// An index with the default caps.
    pub fn new() -> Self {
        Self::with_limits(DEFAULT_MAX_AUTHORS, DEFAULT_MAX_PER_AUTHOR)
    }

    /// An index that keeps at most `max_authors` authors and `max_per_author`
    /// objects per author (both at least 1).
    pub fn with_limits(max_authors: usize, max_per_author: usize) -> Self {
        Self {
            authors: BTreeMap::new(),
            max_authors: max_authors.max(1),
            max_per_author: max_per_author.max(1),
            ticks: 0,
        }
    }

    pub fn max_authors(&self) -> usize {
        self.max_authors
    }

    pub fn max_per_author(&self) -> usize {
        self.max_per_author
    }

    /// Records one identity-signed object and reports how it relates to the
    /// author's chain so far. Idempotent for an object already recorded.
    pub fn observe(
        &mut self,
        author: IdentityKey,
        author_seq: u32,
        op_hash: Hash32,
        prev_own: Hash32,
        heads: &[Hash32],
    ) -> ChainVerdict {
        self.ticks = self.ticks.saturating_add(1);
        let tick = self.ticks;
        if !self.authors.contains_key(&author) && self.authors.len() >= self.max_authors {
            self.evict_one_author();
        }
        let chain = self.authors.entry(author).or_default();
        chain.last_touch = tick;

        if let Some(&(first, second)) = chain.equivocations.get(&author_seq) {
            // Slot already proven equivocated; a third hash is not kept.
            let other = if op_hash == first { second } else { first };
            return ChainVerdict::Equivocation { other };
        }
        if let Some(existing) = chain.entries.get(&author_seq) {
            if existing.hash == op_hash {
                return ChainVerdict::Ok;
            }
            let other = existing.hash;
            chain.equivocations.insert(author_seq, (other, op_hash));
            return ChainVerdict::Equivocation { other };
        }

        let gap = author_seq > chain.next_expected;
        let expected_seq = chain.next_expected;
        let mut sorted = heads.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        chain.entries.insert(
            author_seq,
            ChainEntry {
                hash: op_hash,
                prev_own,
                heads: sorted,
            },
        );
        chain.next_expected = chain.next_expected.max(author_seq.saturating_add(1));
        while chain.entries.len() > self.max_per_author {
            chain.entries.pop_first();
        }
        if gap {
            ChainVerdict::Gap { expected_seq }
        } else {
            ChainVerdict::Ok
        }
    }

    fn evict_one_author(&mut self) {
        let victim = self
            .authors
            .iter()
            .min_by_key(|(k, c)| (c.last_touch, **k))
            .map(|(k, _)| *k);
        if let Some(k) = victim {
            self.authors.remove(&k);
        }
    }

    /// Union of the heads cited by `author`'s recorded objects with a
    /// sequence number below `seq` (sorted, deduplicated). This is the
    /// evidence for the vote-open fork rule (DESIGN §3): a VoteOpen at
    /// `seq` citing `H_v` is invalid if any of these heads is outside
    /// `closure(H_v)`.
    pub fn heads_cited_before(&self, author: &IdentityKey, seq: u32) -> Vec<Hash32> {
        let mut out: Vec<Hash32> = self
            .authors
            .get(author)
            .map(|c| {
                c.entries
                    .range(..seq)
                    .flat_map(|(_, e)| e.heads.iter().copied())
                    .collect()
            })
            .unwrap_or_default();
        out.sort_unstable();
        out.dedup();
        out
    }

    /// The recorded object at `(author, seq)`, if any (the first one seen
    /// when the slot was equivocated).
    pub fn get(&self, author: &IdentityKey, seq: u32) -> Option<&ChainEntry> {
        self.authors.get(author)?.entries.get(&seq)
    }

    /// Has this author ever signed two different objects with one sequence
    /// number?
    pub fn is_equivocator(&self, author: &IdentityKey) -> bool {
        self.authors
            .get(author)
            .is_some_and(|c| !c.equivocations.is_empty())
    }

    /// The proof of every equivocation by `author`: `(seq, first hash,
    /// second hash)`, in sequence order.
    pub fn equivocations(&self, author: &IdentityKey) -> Vec<(u32, Hash32, Hash32)> {
        self.authors
            .get(author)
            .map(|c| {
                c.equivocations
                    .iter()
                    .map(|(s, (a, b))| (*s, *a, *b))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Sequence number the author's next object is expected to carry
    /// (one past the highest seen; 0 for an unknown author).
    pub fn next_expected(&self, author: &IdentityKey) -> u32 {
        self.authors.get(author).map_or(0, |c| c.next_expected)
    }

    /// Does the recorded chain link at `seq` point at the recorded object at
    /// `seq - 1`? `None` when either side is unknown or `seq == 0`.
    pub fn link_consistent(&self, author: &IdentityKey, seq: u32) -> Option<bool> {
        let chain = self.authors.get(author)?;
        let this = chain.entries.get(&seq)?;
        let prev = chain.entries.get(&seq.checked_sub(1)?)?;
        Some(this.prev_own == prev.hash)
    }

    /// Number of authors recorded.
    pub fn author_count(&self) -> usize {
        self.authors.len()
    }

    /// Number of objects recorded for `author`.
    pub fn entry_count(&self, author: &IdentityKey) -> usize {
        self.authors.get(author).map_or(0, |c| c.entries.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;

    fn key(i: u8) -> IdentityKey {
        IdentitySigner::from_seed(&[i; 32]).public()
    }

    fn h(i: u8) -> Hash32 {
        Hash32::from_bytes([i; 32])
    }

    #[test]
    fn consecutive_objects_are_ok_and_repeats_are_idempotent() {
        let mut idx = SenderChainIndex::new();
        let a = key(1);
        assert_eq!(
            idx.observe(a, 0, h(10), Hash32::ZERO, &[h(1)]),
            ChainVerdict::Ok
        );
        assert_eq!(
            idx.observe(a, 1, h(11), h(10), &[h(2), h(1)]),
            ChainVerdict::Ok
        );
        assert_eq!(
            idx.observe(a, 1, h(11), h(10), &[h(2), h(1)]),
            ChainVerdict::Ok
        );
        assert_eq!(idx.next_expected(&a), 2);
        assert_eq!(idx.link_consistent(&a, 1), Some(true));
        assert_eq!(idx.link_consistent(&a, 0), None);
        assert_eq!(idx.entry_count(&a), 2);
        assert!(!idx.is_equivocator(&a));
    }

    #[test]
    fn same_seq_different_hash_is_equivocation_with_proof() {
        let mut idx = SenderChainIndex::new();
        let a = key(1);
        assert_eq!(
            idx.observe(a, 3, h(1), h(0), &[]),
            ChainVerdict::Gap { expected_seq: 0 }
        );
        assert_eq!(
            idx.observe(a, 3, h(2), h(0), &[]),
            ChainVerdict::Equivocation { other: h(1) }
        );
        // a third hash on the slot is reported but not stored
        assert_eq!(
            idx.observe(a, 3, h(3), h(0), &[]),
            ChainVerdict::Equivocation { other: h(1) }
        );
        // re-observing either proof hash still reports the other one
        assert_eq!(
            idx.observe(a, 3, h(1), h(0), &[]),
            ChainVerdict::Equivocation { other: h(2) }
        );
        assert!(idx.is_equivocator(&a));
        assert_eq!(idx.equivocations(&a), vec![(3, h(1), h(2))]);
        assert!(!idx.is_equivocator(&key(2)));
    }

    #[test]
    fn gaps_are_reported_and_filled() {
        let mut idx = SenderChainIndex::new();
        let a = key(1);
        assert_eq!(idx.observe(a, 0, h(1), Hash32::ZERO, &[]), ChainVerdict::Ok);
        assert_eq!(
            idx.observe(a, 2, h(3), h(2), &[]),
            ChainVerdict::Gap { expected_seq: 1 }
        );
        assert_eq!(idx.observe(a, 1, h(2), h(1), &[]), ChainVerdict::Ok);
        assert_eq!(idx.next_expected(&a), 3);
        assert_eq!(idx.link_consistent(&a, 2), Some(true));
        assert_eq!(idx.observe(a, 3, h(4), h(9), &[]), ChainVerdict::Ok);
        assert_eq!(idx.link_consistent(&a, 3), Some(false));
        assert_eq!(
            idx.observe(a, u32::MAX, h(5), h(9), &[]),
            ChainVerdict::Gap { expected_seq: 4 }
        );
        assert_eq!(idx.next_expected(&a), u32::MAX);
    }

    #[test]
    fn heads_cited_before_is_the_union_of_earlier_heads() {
        let mut idx = SenderChainIndex::new();
        let a = key(1);
        idx.observe(a, 0, h(10), Hash32::ZERO, &[h(3), h(1)]);
        idx.observe(a, 1, h(11), h(10), &[h(2)]);
        idx.observe(a, 2, h(12), h(11), &[h(9)]);
        assert_eq!(idx.heads_cited_before(&a, 2), vec![h(1), h(2), h(3)]);
        assert_eq!(idx.heads_cited_before(&a, 0), Vec::<Hash32>::new());
        assert_eq!(idx.heads_cited_before(&a, 3), vec![h(1), h(2), h(3), h(9)]);
        assert_eq!(idx.heads_cited_before(&key(7), 5), Vec::<Hash32>::new());
    }

    #[test]
    fn per_author_cap_forgets_the_lowest_sequence_numbers() {
        let mut idx = SenderChainIndex::with_limits(8, 3);
        let a = key(1);
        for s in 0..5u32 {
            idx.observe(a, s, h(s as u8 + 1), Hash32::ZERO, &[h(100 + s as u8)]);
        }
        assert_eq!(idx.entry_count(&a), 3);
        assert!(idx.get(&a, 0).is_none());
        assert!(idx.get(&a, 1).is_none());
        assert!(idx.get(&a, 2).is_some());
        assert_eq!(idx.heads_cited_before(&a, 10), vec![h(102), h(103), h(104)]);
        assert_eq!(idx.next_expected(&a), 5);
    }

    #[test]
    fn author_cap_evicts_the_least_recently_touched_author() {
        let mut idx = SenderChainIndex::with_limits(2, 10);
        idx.observe(key(1), 0, h(1), Hash32::ZERO, &[]);
        idx.observe(key(2), 0, h(2), Hash32::ZERO, &[]);
        idx.observe(key(1), 1, h(3), h(1), &[]); // touch 1 again
        idx.observe(key(3), 0, h(4), Hash32::ZERO, &[]); // evicts 2
        assert_eq!(idx.author_count(), 2);
        assert!(idx.get(&key(2), 0).is_none());
        assert!(idx.get(&key(1), 1).is_some());
        assert!(idx.get(&key(3), 0).is_some());
        assert_eq!(idx.max_authors(), 2);
        assert_eq!(idx.max_per_author(), 10);
    }
}
