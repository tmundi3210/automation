//! Public-API tests for `meshcore::dtn` (docs/DESIGN.md §10 "dedupe
//! expiry; summary vectors; buffer eviction order" and the stable-proptest
//! version of the `summary_decode` fuzz target; BRIEF_v2 §5.5 relay
//! bullets and §5.7; DECISIONS D4 MERGE, D10 (4), §3 rows (4,6,7) and
//! (1,2,3,4,10) (iv)).

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use meshcore::clock::ms;
use meshcore::codec::{decode, encode};
use meshcore::crypto::hash::{Hash32, sha256};
use meshcore::dtn::dedupe::{
    DEDUPE_CARRY_MS, DEDUPE_RETENTION_ASM_MS, DedupeCache, KEY_RETENTION_ASM_MS,
    dedupe_retention_ms,
};
use meshcore::dtn::store::{
    CellMeta, CellStore, EvictionEntry, EvictionPolicy, LowestClassL3First, Put, StoreCaps,
};
use meshcore::dtn::summary::{
    Assembled, ChunkAssembler, MAX_CHUNK_TOTAL, MAX_K, MAX_N_BITS, MIN_N_BITS, SummaryError,
    SummaryVector, sizing,
};
use meshcore::dtn::sync::{LinkSync, Out, PROTO_VERSION, SyncConfig, SyncState};
use meshcore::relay::Lane;
use meshcore::transport::LinkId;
use meshcore::wire::cell::{BODY_LEN, CELL_LEN, Cell, TAG_LEN, VERSION, build};
use meshcore::wire::frame::{Class, Hello, SummaryChunk};
use proptest::prelude::*;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha20Rng;

// ---- helpers ----------------------------------------------------------------

/// A distinct cell per seed; the TTL does not enter the `msg_id`.
fn cell(seed: u64, ttl: u8) -> Cell {
    let mut body = [0u8; BODY_LEN];
    body[..8].copy_from_slice(&seed.to_le_bytes());
    for (i, b) in body[8..].iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(13) ^ (seed as u8);
    }
    build(VERSION, ttl, &[0x42; TAG_LEN], &body)
}

fn id_of(seed: u64) -> Hash32 {
    cell(seed, 7).msg_id()
}

fn hash(n: u64) -> Hash32 {
    sha256(&n.to_le_bytes())
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

/// Class of a seed in the property tests: all five classes appear.
fn class_of(seed: u64) -> Class {
    Class::ALL[(seed % 5) as usize]
}

/// TTL of a seed in the property tests: every eleventh is spent.
fn ttl_of(seed: u64) -> u8 {
    if seed % 11 == 0 { 1 } else { 7 }
}

fn store_of(seeds: &BTreeSet<u64>) -> CellStore {
    let mut s = CellStore::with_default_policy(StoreCaps::cells(1_000, 1_000));
    for (i, seed) in seeds.iter().enumerate() {
        assert_eq!(
            s.put(
                cell(*seed, ttl_of(*seed)),
                meta(class_of(*seed), Lane::L1, None, i as u64)
            ),
            Put::Stored
        );
    }
    s
}

/// Feeds one side's outputs into the other; returns the answers.
fn deliver(outs: Vec<Out>, to: &mut LinkSync) -> Vec<Out> {
    let mut back = Vec::new();
    for o in outs {
        match o {
            Out::SendHello(h) => back.extend(to.on_hello(h)),
            Out::SendSummary(chunks) => {
                // every chunk must be a legal frame payload
                for c in &chunks {
                    assert_eq!(decode::<SummaryChunk>(&encode(c)).as_ref(), Ok(c));
                }
                for c in chunks {
                    back.extend(to.on_summary_chunk(c));
                }
            }
            Out::Push => {}
        }
    }
    back
}

/// Runs the Hello/summary exchange on both sides until both push.
fn exchange(
    a: &mut LinkSync,
    sa: &CellStore,
    da: &DedupeCache,
    b: &mut LinkSync,
    sb: &CellStore,
    db: &DedupeCache,
) {
    let a0 = a.start(0x1111_1111, 10, sa, da);
    let b0 = b.start(0x2222_2222, 10, sb, db);
    let a1 = deliver(b0, a);
    let b1 = deliver(a0, b);
    let a2 = deliver(b1, a);
    let b2 = deliver(a1, b);
    assert!(deliver(a2, b).is_empty());
    assert!(deliver(b2, a).is_empty());
    assert_eq!(a.state(), SyncState::Pushing);
    assert_eq!(b.state(), SyncState::Pushing);
}

/// Pushes `ids` from `from`'s store into `to`'s store as a relay would:
/// TTL decremented, recorded as received on the peer's sync.
fn push(ids: &[Hash32], from: &CellStore, to: &mut CellStore, to_sync: &mut LinkSync, now: u64) {
    for id in ids {
        let (c, m) = from.get(id).expect("pushed id is held");
        let fwd = c.forwarded().expect("only forwardable cells are pushed");
        to_sync.on_cell_received(*id);
        let r = to.put(fwd, meta(m.class, Lane::L3, Some(7), now));
        assert_ne!(r, Put::NoRoom);
    }
}

// ---- store ------------------------------------------------------------------

#[test]
fn store_orders_candidates_by_class_then_age_and_skips_spent_cells() {
    let mut s = CellStore::with_default_policy(StoreCaps::cells(50, 50));
    let rows = [
        (1u64, Class::C4Preview, 7u8, 5u64),
        (2, Class::C2Text, 7, 9),
        (3, Class::C2Text, 7, 3),
        (4, Class::C1Control, 7, 100),
        (5, Class::C0Sos, 7, 200),
        (6, Class::C1Control, 1, 1), // spent: never a candidate
        (7, Class::C3News, 2, 4),
    ];
    for (seed, class, ttl, t) in rows {
        assert_eq!(
            s.put(cell(seed, ttl), meta(class, Lane::L2, Some(1), t)),
            Put::Stored
        );
    }
    let want: Vec<Hash32> = [5u64, 4, 3, 2, 7, 1].iter().map(|s| id_of(*s)).collect();
    assert_eq!(s.candidates_for(&|_| false, 100), want);
    assert_eq!(s.candidates_for(&|_| false, 3), want[..3].to_vec());
    let skip = id_of(4);
    assert_eq!(
        s.candidates_for(&|id| *id == skip, 100),
        want.iter()
            .copied()
            .filter(|i| *i != skip)
            .collect::<Vec<_>>()
    );
    assert!(s.candidates_for(&|_| true, 100).is_empty());
    assert_eq!(s.len(), 7);
    assert_eq!(s.bytes(), 7 * CELL_LEN);
    assert_eq!(s.len_for_class(Class::C2Text), 2);
    assert_eq!(s.len_for_class(Class::C1Control), 2);
    assert_eq!(s.bytes_for_link(LinkId(1)), 7 * CELL_LEN);
    assert_eq!(s.iter_ids().count(), 7);
    assert!(
        s.iter()
            .all(|(id, c, m)| c.msg_id() == *id && m.from_link == Some(LinkId(1)))
    );
}

#[test]
fn store_keys_on_msg_id_so_a_relayed_copy_is_a_duplicate() {
    let mut s = CellStore::default();
    assert_eq!(s.caps(), StoreCaps::DEFAULT);
    let c = cell(1, 8);
    assert_eq!(
        s.put(c.clone(), meta(Class::C2Text, Lane::L1, None, 0)),
        Put::Stored
    );
    let relayed = c.forwarded().unwrap();
    assert_eq!(relayed.ttl(), 7);
    assert_eq!(
        s.put(relayed, meta(Class::C2Text, Lane::L3, Some(3), 50)),
        Put::Duplicate
    );
    assert_eq!(s.len(), 1);
    assert_eq!(s.get(&c.msg_id()).map(|(c, _)| c.ttl()), Some(8));
    assert_eq!(s.len_for_link(LinkId(3)), 0, "a duplicate is not accounted");
    assert_eq!(s.remove(&c.msg_id()).map(|(_, m)| m.lane), Some(Lane::L1));
    assert!(s.is_empty());
    assert_eq!(s.free_bytes(), StoreCaps::DEFAULT.max_bytes);
}

#[test]
fn store_evicts_lowest_class_l3_oldest_first_and_never_a_pin_or_a_higher_class() {
    let mut s = CellStore::with_default_policy(StoreCaps::cells(4, 4));
    s.put(cell(1, 7), meta(Class::C3News, Lane::L1, None, 10));
    s.put(cell(2, 7), meta(Class::C3News, Lane::L3, None, 20)); // L3: first to go
    s.put(cell(3, 7), meta(Class::C3News, Lane::L3, None, 5)); // L3 and older: before 2
    let mut pin = meta(Class::C1Control, Lane::L3, None, 1);
    pin.pinned = true;
    s.put(cell(4, 7), pin);
    assert_eq!(s.len(), 4);

    assert_eq!(
        s.put(cell(5, 7), meta(Class::C2Text, Lane::L1, None, 30)),
        Put::Stored
    );
    assert!(
        !s.contains(&id_of(3)),
        "oldest L3 of the lowest class goes first"
    );
    assert!(s.contains(&id_of(2)));
    assert_eq!(
        s.put(cell(6, 7), meta(Class::C2Text, Lane::L1, None, 31)),
        Put::Stored
    );
    assert!(!s.contains(&id_of(2)));
    assert_eq!(
        s.put(cell(7, 7), meta(Class::C2Text, Lane::L1, None, 32)),
        Put::Stored
    );
    assert!(!s.contains(&id_of(1)), "then the L1 C3");
    assert_eq!(s.evicted_total(), 3);
    // store = {4 pinned C1, 5, 6, 7 (C2)}: a C3 arrival may not evict C2
    assert_eq!(
        s.put(cell(8, 7), meta(Class::C3News, Lane::L1, None, 40)),
        Put::NoRoom
    );
    assert_eq!(s.len(), 4);
    // a C1 arrival evicts the oldest C2, never the pinned C1
    assert_eq!(
        s.put(cell(9, 7), meta(Class::C1Control, Lane::L1, None, 41)),
        Put::Stored
    );
    assert!(!s.contains(&id_of(5)));
    assert!(s.contains(&id_of(4)));
    // unpin, and the C1 can go for a C0
    assert!(s.set_pinned(&id_of(4), false));
    assert_eq!(
        s.put(cell(10, 7), meta(Class::C0Sos, Lane::L1, None, 42)),
        Put::Stored
    );
    assert!(
        !s.contains(&id_of(6)),
        "C2 is the lowest class present; its oldest goes"
    );
    assert!(s.contains(&id_of(4)) && s.contains(&id_of(7)) && s.contains(&id_of(9)));
    assert!(s.contains(&id_of(10)));
    assert_eq!(s.len_for_class(Class::C0Sos), 1);
}

/// The policy hook sees exactly the minimal view and cannot override the
/// store's own rules (pins, class rank).
#[test]
fn store_calls_the_policy_with_the_minimal_view_and_overrules_bad_choices() {
    #[derive(Debug, Default)]
    struct Spy {
        calls: RefCell<Vec<(Vec<EvictionEntry>, EvictionEntry, usize)>>,
        answer: RefCell<Vec<Hash32>>,
    }
    impl EvictionPolicy for Spy {
        fn victims(
            &self,
            entries: &[EvictionEntry],
            incoming: &EvictionEntry,
            need_bytes: usize,
        ) -> Vec<Hash32> {
            self.calls
                .borrow_mut()
                .push((entries.to_vec(), *incoming, need_bytes));
            self.answer.borrow().clone()
        }
    }
    let spy = Spy::default();
    // the policy must be owned by the store; keep a shared answer through a static-free trick:
    // answer the pinned id and the higher-class id, then nothing valid.
    let pinned_id = id_of(1);
    let high_id = id_of(2);
    let low_id = id_of(3);
    *spy.answer.borrow_mut() = vec![pinned_id, high_id, hash(999), low_id];
    let calls = std::rc::Rc::new(spy);
    struct Shared(std::rc::Rc<Spy>);
    impl EvictionPolicy for Shared {
        fn victims(
            &self,
            entries: &[EvictionEntry],
            incoming: &EvictionEntry,
            need_bytes: usize,
        ) -> Vec<Hash32> {
            self.0.victims(entries, incoming, need_bytes)
        }
    }
    let mut s = CellStore::new(StoreCaps::cells(3, 2), Box::new(Shared(calls.clone())));
    let mut pin = meta(Class::C2Text, Lane::L1, Some(1), 1);
    pin.pinned = true;
    assert_eq!(s.put(cell(1, 7), pin), Put::Stored);
    assert_eq!(
        s.put(cell(2, 5), meta(Class::C1Control, Lane::L2, Some(2), 2)),
        Put::Stored
    );
    assert_eq!(
        s.put(cell(3, 3), meta(Class::C3News, Lane::L3, Some(2), 3)),
        Put::Stored
    );
    assert!(calls.calls.borrow().is_empty(), "under the caps: no call");

    // link 2 is at its cap (2): the per-link pool is consulted first
    assert_eq!(
        s.put(cell(4, 7), meta(Class::C2Text, Lane::L1, Some(2), 4)),
        Put::Stored
    );
    {
        let c = calls.calls.borrow();
        assert_eq!(c.len(), 1, "one per-link call; the total cap was then met");
        let (entries, incoming, need) = &c[0];
        assert_eq!(need, &CELL_LEN);
        assert_eq!(incoming.id, id_of(4));
        assert_eq!(incoming.class, Class::C2Text);
        assert_eq!(incoming.ttl, 7);
        assert_eq!(incoming.first_seen_mono, 4);
        let ids: Vec<Hash32> = entries.iter().map(|e| e.id).collect();
        assert_eq!(
            ids,
            vec![high_id, low_id],
            "link 2's cells in forwarding order"
        );
        assert_eq!(entries[1].ttl, 3);
        assert_eq!(entries[1].lane, Lane::L3);
        assert!(!entries[0].pinned);
    }
    assert!(
        s.contains(&pinned_id),
        "not in the consulted (per-link) pool: the policy's choice is ignored"
    );
    assert!(s.contains(&high_id), "outranks the arrival: ignored");
    assert!(!s.contains(&low_id), "the one legal victim");
    assert!(s.contains(&id_of(4)));
    assert_eq!(s.len_for_link(LinkId(2)), 2);

    // now nothing legal remains for a C2 arrival on link 2: NoRoom, nothing changes
    *calls.answer.borrow_mut() = vec![pinned_id, high_id, id_of(4)];
    let before: Vec<Hash32> = s.iter_ids().collect();
    assert_eq!(
        s.put(cell(5, 7), meta(Class::C3News, Lane::L1, Some(2), 5)),
        Put::NoRoom
    );
    assert_eq!(s.iter_ids().collect::<Vec<_>>(), before);
    // the default policy is still usable on its own
    let entries = vec![EvictionEntry {
        id: low_id,
        class: Class::C4Preview,
        lane: Lane::L3,
        first_seen_mono: 0,
        pinned: false,
        ttl: 7,
    }];
    let incoming = EvictionEntry {
        id: id_of(9),
        class: Class::C2Text,
        lane: Lane::L1,
        first_seen_mono: 1,
        pinned: false,
        ttl: 7,
    };
    assert_eq!(
        LowestClassL3First.victims(&entries, &incoming, 1),
        vec![low_id]
    );
    assert!(
        LowestClassL3First
            .victims(&entries, &incoming, 0)
            .is_empty()
    );
    s.set_policy(Box::new(LowestClassL3First));
}

/// The store's own pin rule (D3 M1 "roster ops are never evicted until
/// superseded") holds even against a policy that names a pinned entry that
/// *is* in the consulted pool. The default policy never names one, so only
/// a hostile policy exercises the store's enforcement.
#[test]
fn store_never_evicts_a_pinned_entry_even_when_the_policy_names_it() {
    struct NameEverything;
    impl EvictionPolicy for NameEverything {
        fn victims(
            &self,
            entries: &[EvictionEntry],
            _incoming: &EvictionEntry,
            _need_bytes: usize,
        ) -> Vec<Hash32> {
            entries.iter().map(|e| e.id).collect()
        }
    }
    let mut s = CellStore::new(StoreCaps::cells(2, 2), Box::new(NameEverything));
    let mut pin = meta(Class::C4Preview, Lane::L3, Some(1), 1);
    pin.pinned = true;
    assert_eq!(s.put(cell(1, 7), pin), Put::Stored);
    assert_eq!(
        s.put(cell(2, 7), meta(Class::C4Preview, Lane::L3, Some(1), 2)),
        Put::Stored
    );
    // per-link cap hit; the policy names both; only the unpinned one may go
    assert_eq!(
        s.put(cell(3, 7), meta(Class::C0Sos, Lane::L1, Some(1), 3)),
        Put::Stored
    );
    assert!(
        s.contains(&id_of(1)),
        "pinned survives a policy that named it"
    );
    assert!(!s.contains(&id_of(2)));
    assert!(s.contains(&id_of(3)));
    assert_eq!(s.len(), 2);
    assert_eq!(s.evicted_total(), 1);
    // with everything pinned the policy's choice frees nothing: NoRoom
    assert!(s.set_pinned(&id_of(3), true));
    let before: Vec<Hash32> = s.iter_ids().collect();
    assert_eq!(
        s.put(cell(4, 7), meta(Class::C0Sos, Lane::L1, Some(1), 4)),
        Put::NoRoom
    );
    assert_eq!(s.iter_ids().collect::<Vec<_>>(), before);
    assert_eq!(s.evicted_total(), 1);
    // the same through the total cap (own cells, no link)
    let mut t = CellStore::new(StoreCaps::cells(1, 1), Box::new(NameEverything));
    let mut pin = meta(Class::C4Preview, Lane::L3, None, 1);
    pin.pinned = true;
    assert_eq!(t.put(cell(1, 7), pin), Put::Stored);
    assert_eq!(
        t.put(cell(2, 7), meta(Class::C0Sos, Lane::L1, None, 2)),
        Put::NoRoom
    );
    assert!(t.contains(&id_of(1)) && !t.contains(&id_of(2)));
}

// ---- dedupe -----------------------------------------------------------------

#[test]
fn dedupe_semantics_expiry_in_powered_on_time_and_boundedness() {
    assert_eq!(DEDUPE_RETENTION_ASM_MS, 10 * ms::DAY);
    assert_eq!(
        dedupe_retention_ms(None, KEY_RETENTION_ASM_MS),
        KEY_RETENTION_ASM_MS + DEDUPE_CARRY_MS
    );
    assert_eq!(
        dedupe_retention_ms(Some(20 * ms::DAY), KEY_RETENTION_ASM_MS),
        23 * ms::DAY
    );

    let retention = ms::HOUR;
    let mut d = DedupeCache::new(retention, 4);
    // first sight at powered-on t=0; the same id is "seen" until t+retention
    assert!(!d.seen_or_insert(hash(1), 0));
    assert!(d.seen_or_insert(hash(1), 0));
    assert!(d.seen_or_insert(hash(1), retention - 1));
    assert_eq!(d.expires_at(&hash(1)), Some(retention));
    // the window is powered-on time: a sweep at t = retention drops it
    assert!(!d.seen_or_insert(hash(2), 5));
    assert_eq!(d.expire(retention - 1), 0);
    assert_eq!(d.expire(retention), 1);
    assert!(!d.contains(&hash(1)));
    assert!(d.contains(&hash(2)));
    assert!(
        !d.seen_or_insert(hash(1), retention),
        "forgotten: a late copy is new again"
    );

    // bounded at 4: the oldest goes when a fifth arrives
    for i in 10..13 {
        assert!(!d.seen_or_insert(hash(i), i));
    }
    assert_eq!(d.len(), 4);
    assert!(!d.contains(&hash(2)), "oldest evicted");
    assert!(d.contains(&hash(1)));
    assert!(!d.seen_or_insert(hash(20), 20));
    assert_eq!(d.len(), 4);
    assert!(!d.contains(&hash(10)));
    assert_eq!(d.max_entries(), 4);
    assert_eq!(d.retention_ms(), retention);
}

#[test]
fn dedupe_rejected_ids_are_remembered_until_expiry() {
    let mut d = DedupeCache::new(100, 10);
    d.mark_rejected(hash(1), 0);
    assert!(d.is_rejected(&hash(1)));
    assert!(d.seen_or_insert(hash(1), 50), "rejected implies seen");
    d.seen_or_insert(hash(2), 0);
    assert!(!d.is_rejected(&hash(2)));
    d.mark_rejected(hash(2), 10);
    assert!(d.is_rejected(&hash(2)));
    assert_eq!(d.rejected_ids().count(), 2);
    assert_eq!(d.rejected_len(), 2);
    assert_eq!(d.expire(100), 2);
    assert!(!d.is_rejected(&hash(1)) && !d.is_rejected(&hash(2)));
    assert!(d.rejected_ids().next().is_none());
}

// ---- summary ----------------------------------------------------------------

#[test]
fn bloom_fpr_is_near_target_over_10k_ids_with_no_false_negatives() {
    let n = 10_000u64;
    let target = 0.01;
    let (n_bits, k) = sizing(n as usize, target);
    assert_eq!(n_bits % 8, 0);
    assert!((1..=MAX_K).contains(&k));
    let members: Vec<Hash32> = (0..n).map(hash).collect();
    let v = SummaryVector::build(members.iter().copied(), 0xC0FF_EE00, n_bits, k);
    assert_eq!(v.len_bytes(), (n_bits / 8) as usize);
    assert!(members.iter().all(|m| v.contains(m)), "no false negatives");
    let false_positives = (n..2 * n).map(hash).filter(|x| v.contains(x)).count();
    let fpr = false_positives as f64 / n as f64;
    assert!(fpr < 2.0 * target, "fpr {fpr} vs target {target}");
    assert!(v.estimated_fpr(n as usize) < 2.0 * target);
    // a different salt moves the false positives (the per-encounter salt rule)
    let w = SummaryVector::build(members.iter().copied(), 0xC0FF_EE01, n_bits, k);
    let fp_w: BTreeSet<u64> = (n..2 * n).filter(|x| w.contains(&hash(*x))).collect();
    let fp_v: BTreeSet<u64> = (n..2 * n).filter(|x| v.contains(&hash(*x))).collect();
    assert!(
        fp_v.intersection(&fp_w).count() * 10 < fp_v.len().max(1) * 3 + 10,
        "salts decorrelate: {} common of {}",
        fp_v.intersection(&fp_w).count(),
        fp_v.len()
    );
    // same inputs → same bytes
    assert_eq!(
        SummaryVector::build(members.iter().copied(), 0xC0FF_EE00, n_bits, k),
        v
    );
}

#[test]
fn summary_sizing_caps() {
    assert_eq!(sizing(0, 0.01).0, 16);
    assert!(sizing(1_000_000, 0.01).0 == MAX_N_BITS);
    assert!(sizing(1, 1e-12).1 == MAX_K);
    assert!(sizing(5, 0.5).0 >= MIN_N_BITS);
}

#[test]
fn chunks_round_trip_in_random_order_and_a_missing_chunk_is_incomplete() {
    let ids: Vec<Hash32> = (0..2_000).map(hash).collect();
    let (n_bits, k) = sizing(ids.len(), 0.01);
    let v = SummaryVector::build(ids.iter().copied(), 77, n_bits, k);
    let chunks = v.to_chunks(496);
    assert!(chunks.len() > 2);
    assert!(
        chunks
            .iter()
            .all(|c| c.bits.len() <= 496 && c.chunk_total as usize == chunks.len())
    );
    let mut rng = ChaCha20Rng::seed_from_u64(7);
    for _ in 0..5 {
        let mut shuffled = chunks.clone();
        shuffled.shuffle(&mut rng);
        // through the wire codec, as the link would carry them
        let wire: Vec<SummaryChunk> = shuffled
            .iter()
            .map(|c| decode::<SummaryChunk>(&encode(c)).unwrap())
            .collect();
        assert_eq!(SummaryVector::from_chunks(&wire), Ok(v.clone()));
        let mut a = ChunkAssembler::new();
        let mut done = None;
        for c in wire {
            match a.push(c).unwrap() {
                Assembled::Incomplete { .. } => assert!(done.is_none()),
                Assembled::Complete(got) => done = Some(got),
            }
        }
        assert_eq!(done, Some(v.clone()));
        // drop one: incomplete
        let mut missing = shuffled.clone();
        missing.remove(rng.next_u32_usize(missing.len()));
        assert_eq!(
            SummaryVector::from_chunks(&missing),
            Err(SummaryError::Incomplete {
                have: (chunks.len() - 1) as u16,
                total: chunks.len() as u16
            })
        );
    }
    // the fields of a chunk beyond the bits are the vector's
    assert!(
        chunks
            .iter()
            .all(|c| c.salt == 77 && c.n_bits == n_bits && c.k == k)
    );
    assert!(MAX_CHUNK_TOTAL as usize >= chunks.len());
}

trait RngUsize {
    fn next_u32_usize(&mut self, bound: usize) -> usize;
}
impl RngUsize for ChaCha20Rng {
    fn next_u32_usize(&mut self, bound: usize) -> usize {
        (rand::RngCore::next_u32(self) as usize) % bound.max(1)
    }
}

proptest! {
    /// `summary_decode` on stable (DESIGN §10 fuzz targets): arbitrary
    /// bytes through the frame codec and the vector builders never panic.
    #[test]
    fn summary_parsers_survive_arbitrary_bytes(
        raws in proptest::collection::vec(proptest::collection::vec(any::<u8>(), 0..700), 1..4)
    ) {
        let chunks: Vec<SummaryChunk> = raws.iter().filter_map(|r| decode::<SummaryChunk>(r).ok()).collect();
        let _ = SummaryVector::from_chunks(&chunks);
        let mut a = ChunkAssembler::new();
        for c in chunks {
            let _ = a.push(c);
        }
        let mut sync = LinkSync::new(SyncConfig::default());
        for r in &raws {
            if let Ok(c) = decode::<SummaryChunk>(r) {
                let _ = sync.on_summary_chunk(c);
            }
        }
    }

    /// Arbitrary chunk structs (fields the codec would never produce
    /// included) never panic the builder.
    #[test]
    fn summary_builder_survives_arbitrary_chunk_structs(
        chunks in proptest::collection::vec(
            (any::<u32>(), any::<u32>(), any::<u8>(), any::<u16>(), any::<u16>(), proptest::collection::vec(any::<u8>(), 0..200)),
            0..5,
        )
    ) {
        let chunks: Vec<SummaryChunk> = chunks
            .into_iter()
            .map(|(salt, n_bits, k, chunk_idx, chunk_total, bits)| SummaryChunk { salt, n_bits, k, chunk_idx, chunk_total, bits })
            .collect();
        let _ = SummaryVector::from_chunks(&chunks);
        let mut sync = LinkSync::new(SyncConfig::default());
        for c in chunks {
            let _ = sync.on_summary_chunk(c);
        }
    }
}

// ---- sync -------------------------------------------------------------------

#[test]
fn rejected_ids_are_never_re_offered_in_either_direction() {
    // A holds 1,2,3; B holds 2 and has rejected 3 (a removed member's post).
    let sa = store_of(&BTreeSet::from([1u64, 2, 3]));
    let sb = store_of(&BTreeSet::from([2u64]));
    let mut da = DedupeCache::new(ms::DAY, 100);
    let mut db = DedupeCache::new(ms::DAY, 100);
    db.mark_rejected(id_of(3), 0);
    // A itself rejected 4 but still holds the cell (not yet evicted)
    let mut sa = sa;
    sa.put(cell(4, 7), meta(Class::C2Text, Lane::L1, None, 99));
    da.mark_rejected(id_of(4), 0);

    let mut a = LinkSync::new(SyncConfig::default());
    let mut b = LinkSync::new(SyncConfig::default());
    exchange(&mut a, &sa, &da, &mut b, &sb, &db);
    assert!(
        b.local_summary().unwrap().contains(&id_of(3)),
        "B's vector carries the rejected hash"
    );
    let a_push = a.next_to_send(&sa, &da, 100);
    assert_eq!(
        a_push,
        vec![id_of(1)],
        "3 is in B's summary, 4 is rejected locally"
    );
    assert!(!a_push.contains(&id_of(3)));
    assert!(!a_push.contains(&id_of(4)));
    assert_eq!(
        a.never_offered(),
        2,
        "2 (held by B) and 3 (rejected by B) are both in B's vector"
    );
    assert_eq!(
        a.withheld_ids().collect::<BTreeSet<_>>(),
        BTreeSet::from([id_of(2), id_of(3)])
    );
    assert!(b.next_to_send(&sb, &db, 100).is_empty());
}

#[test]
fn two_syncs_converge_on_a_fixed_example() {
    let a_ids = BTreeSet::from([1u64, 2, 3, 5, 8, 13]);
    let b_ids = BTreeSet::from([2u64, 3, 21, 34]);
    let mut sa = store_of(&a_ids);
    let mut sb = store_of(&b_ids);
    let da = DedupeCache::new(ms::DAY, 100);
    let db = DedupeCache::new(ms::DAY, 100);
    let mut a = LinkSync::new(SyncConfig::default());
    let mut b = LinkSync::new(SyncConfig::default());
    exchange(&mut a, &sa, &da, &mut b, &sb, &db);
    assert!(!a.digest_matched() && !b.digest_matched());
    let a_push = a.next_to_send(&sa, &da, 100);
    let mut want: Vec<Hash32> = [1u64, 5, 8, 13].iter().map(|s| id_of(*s)).collect();
    let mut got = a_push.clone();
    want.sort();
    got.sort();
    assert_eq!(got, want);
    // class order first: 5 (C0), then 1 (C1), 8 (C3), 13 (C3)
    assert_eq!(a_push[0], id_of(5));
    assert_eq!(a_push[1], id_of(1));
    push(&a_push, &sa, &mut sb, &mut b, 20);
    let b_push = b.next_to_send(&sb, &db, 100);
    let mut got: Vec<Hash32> = b_push.clone();
    got.sort();
    let mut want: Vec<Hash32> = [21u64, 34].iter().map(|s| id_of(*s)).collect();
    want.sort();
    assert_eq!(got, want);
    push(&b_push, &sb, &mut sa, &mut a, 20);
    assert!(a.next_to_send(&sa, &da, 100).is_empty());
    assert!(b.next_to_send(&sb, &db, 100).is_empty());
    let union: BTreeSet<Hash32> = a_ids.union(&b_ids).map(|s| id_of(*s)).collect();
    assert_eq!(sa.iter_ids().collect::<BTreeSet<_>>(), union);
    assert_eq!(sb.iter_ids().collect::<BTreeSet<_>>(), union);
    assert_eq!(a.pushed_len(), 4);
    assert_eq!(b.received_len(), 4);
    assert_eq!(b.pushed_len(), 2);
}

#[test]
fn equal_stores_match_digests_and_push_nothing() {
    let ids = BTreeSet::from([3u64, 4, 5]);
    let sa = store_of(&ids);
    let sb = store_of(&ids);
    let d = DedupeCache::new(ms::DAY, 100);
    let mut a = LinkSync::new(SyncConfig::default());
    let mut b = LinkSync::new(SyncConfig {
        is_station: true,
        ..SyncConfig::default()
    });
    exchange(&mut a, &sa, &d, &mut b, &sb, &d);
    assert!(a.digest_matched());
    assert!(b.digest_matched());
    assert!(a.next_to_send(&sa, &d, 10).is_empty());
    assert!(b.next_to_send(&sb, &d, 10).is_empty());
    assert_eq!(
        a.peer_hello().map(|h| (h.proto, h.is_station)),
        Some((PROTO_VERSION, true))
    );
    // every cell was withheld on the peer's vector, all of them true holdings
    assert_eq!(a.never_offered(), 3);
    assert_eq!(
        a.withheld_ids().collect::<BTreeSet<_>>(),
        ids.iter().map(|s| id_of(*s)).collect()
    );
    assert_eq!(a.summary_errors(), 0);
}

/// The peer's chunks can complete before `start` (a pocketed phone's link
/// comes up with the peer already talking). The digest match must still be
/// judged against what we actually summarise, and a cell the peer pushed
/// before we started still counts as "outside ours".
#[test]
fn digest_match_is_judged_against_our_ids_even_when_the_peer_summary_came_first() {
    let s = store_of(&BTreeSet::from([1u64]));
    let d = DedupeCache::new(ms::DAY, 100);
    let hello = Hello {
        proto: PROTO_VERSION,
        caps: 0,
        is_station: false,
        buffer_free: 0,
    };
    // peer holds only cell 2 (and its vector does not accidentally claim 1)
    let peer = SummaryVector::build([id_of(2)], 9, 256, 3);
    assert!(!peer.contains(&id_of(1)));
    let mut a = LinkSync::new(SyncConfig::default());
    for c in peer.to_chunks(16) {
        assert!(a.on_summary_chunk(c).is_empty());
    }
    assert!(a.peer_summary().is_some());
    assert!(matches!(
        a.start(1, 0, &s, &d).as_slice(),
        [Out::SendHello(_)]
    ));
    assert!(matches!(
        a.on_hello(hello).as_slice(),
        [Out::SendSummary(_), Out::Push]
    ));
    assert_eq!(a.state(), SyncState::Pushing);
    assert!(
        !a.digest_matched(),
        "cell 1 is ours and not in the peer's vector"
    );
    assert_eq!(a.next_to_send(&s, &d, 10), vec![id_of(1)]);

    // same holdings on both sides, but a cell the peer pushed before we
    // started (never stored here) is outside our summary
    let same = SummaryVector::build([id_of(1)], 9, 256, 3);
    assert!(!same.contains(&id_of(3)));
    let mut b = LinkSync::new(SyncConfig::default());
    for c in same.to_chunks(16) {
        b.on_summary_chunk(c);
    }
    b.on_cell_received(id_of(3));
    b.start(2, 0, &s, &d);
    assert!(matches!(
        b.on_hello(hello).as_slice(),
        [Out::SendSummary(_), Out::Push]
    ));
    assert!(
        !b.digest_matched(),
        "3 was pushed to us but we never summarised it"
    );
    assert!(b.next_to_send(&s, &d, 10).is_empty());

    // and with nothing pushed, equal holdings do match in this ordering too
    let mut c = LinkSync::new(SyncConfig::default());
    for ch in same.to_chunks(16) {
        c.on_summary_chunk(ch);
    }
    c.start(3, 0, &s, &d);
    c.on_hello(hello);
    assert!(c.digest_matched());
}

/// A rejected post one side remembers but the other never saw must not
/// stop two phones with the same cells from matching (the "last sync"
/// strip would otherwise stay dark for the dedupe retention).
#[test]
fn a_rejection_the_peer_never_saw_does_not_break_the_digest_match() {
    let ids = BTreeSet::from([3u64, 4]);
    let sa = store_of(&ids);
    let sb = store_of(&ids);
    let mut da = DedupeCache::new(ms::DAY, 100);
    da.mark_rejected(id_of(50), 0);
    let db = DedupeCache::new(ms::DAY, 100);
    let mut a = LinkSync::new(SyncConfig::default());
    let mut b = LinkSync::new(SyncConfig::default());
    exchange(&mut a, &sa, &da, &mut b, &sb, &db);
    assert!(a.local_summary().unwrap().contains(&id_of(50)));
    assert!(!b.local_summary().unwrap().contains(&id_of(50)));
    assert!(a.digest_matched());
    assert!(b.digest_matched());
    assert!(a.next_to_send(&sa, &da, 10).is_empty());
    assert!(b.next_to_send(&sb, &db, 10).is_empty());
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(96))]

    /// After one exchange each side's `next_to_send` yields exactly the
    /// cells the other lacks (forwardable, not rejected, not hidden by a
    /// false positive of the other's vector), in class-then-age order, and
    /// after both pushes the stores agree.
    #[test]
    fn two_syncs_exchange_exactly_the_missing_cells(
        a_ids in proptest::collection::btree_set(0u64..70, 0..45),
        b_ids in proptest::collection::btree_set(0u64..70, 0..45),
        b_rejects in proptest::collection::btree_set(0u64..70, 0..4),
        a_rejects in proptest::collection::btree_set(0u64..70, 0..4),
        salt_a in any::<u32>(),
        salt_b in any::<u32>(),
    ) {
        let mut sa = store_of(&a_ids);
        let mut sb = store_of(&b_ids);
        let mut da = DedupeCache::new(ms::DAY, 1_000);
        let mut db = DedupeCache::new(ms::DAY, 1_000);
        for r in &a_rejects { da.mark_rejected(id_of(*r), 0); }
        for r in &b_rejects { db.mark_rejected(id_of(*r), 0); }
        let mut a = LinkSync::new(SyncConfig::default());
        let mut b = LinkSync::new(SyncConfig::default());
        let a0 = a.start(salt_a, 0, &sa, &da);
        let b0 = b.start(salt_b, 0, &sb, &db);
        let a1 = deliver(b0, &mut a);
        let b1 = deliver(a0, &mut b);
        deliver(b1, &mut a);
        deliver(a1, &mut b);
        prop_assert_eq!(a.state(), SyncState::Pushing);
        prop_assert_eq!(b.state(), SyncState::Pushing);
        let sum_a = a.local_summary().unwrap().clone();
        let sum_b = b.local_summary().unwrap().clone();
        prop_assert_eq!(sum_a.salt(), salt_a);
        prop_assert_eq!(sum_b.salt(), salt_b);

        // what A should push: in A, forwardable, not rejected by A, not in B's vector
        let order_key = |store: &CellStore, id: &Hash32| {
            let (c, m) = store.get(id).unwrap();
            (m.class, m.first_seen_mono, c.msg_id())
        };
        let mut want_a: Vec<Hash32> = a_ids
            .iter()
            .filter(|s| !b_ids.contains(s) && ttl_of(**s) >= 2 && !a_rejects.contains(s))
            .map(|s| id_of(*s))
            .filter(|id| !sum_b.contains(id))
            .collect();
        want_a.sort_by_key(|id| order_key(&sa, id));
        let got_a = a.next_to_send(&sa, &da, 1_000);
        prop_assert_eq!(&got_a, &want_a);
        // withheld = forwardable, not rejected by A, in B's vector (B's
        // holdings, B's rejections and false positives alike)
        let withheld_a: BTreeSet<Hash32> = a_ids
            .iter()
            .filter(|s| ttl_of(**s) >= 2 && !a_rejects.contains(s))
            .map(|s| id_of(*s))
            .filter(|id| sum_b.contains(id))
            .collect();
        prop_assert_eq!(a.never_offered(), withheld_a.len() as u64);
        prop_assert_eq!(a.withheld_ids().collect::<BTreeSet<_>>(), withheld_a.clone());
        // the simulator's metric: withheld although B really lacked it (a
        // false positive of B's vector, or B's rejection)
        let suppressed: BTreeSet<Hash32> = withheld_a
            .iter()
            .copied()
            .filter(|id| !sb.contains(id))
            .collect();
        for id in &suppressed {
            prop_assert!(sum_b.contains(id));
            prop_assert!(!want_a.contains(id));
        }
        prop_assert!(a.next_to_send(&sa, &da, 1_000).is_empty());

        push(&got_a, &sa, &mut sb, &mut b, 5);

        let mut want_b: Vec<Hash32> = b_ids
            .iter()
            .filter(|s| !a_ids.contains(s) && ttl_of(**s) >= 2 && !b_rejects.contains(s))
            .map(|s| id_of(*s))
            .filter(|id| !sum_a.contains(id))
            .collect();
        want_b.sort_by_key(|id| order_key(&sb, id));
        let got_b = b.next_to_send(&sb, &db, 1_000);
        prop_assert_eq!(&got_b, &want_b);
        push(&got_b, &sb, &mut sa, &mut a, 5);
        prop_assert!(a.next_to_send(&sa, &da, 1_000).is_empty());
        prop_assert!(b.next_to_send(&sb, &db, 1_000).is_empty());

        // stores converge up to spent cells, rejections and false positives
        let a_final: BTreeSet<Hash32> = sa.iter_ids().collect();
        let b_final: BTreeSet<Hash32> = sb.iter_ids().collect();
        for s in a_ids.union(&b_ids) {
            let id = id_of(*s);
            let in_a = a_ids.contains(s);
            let in_b = b_ids.contains(s);
            prop_assert!(a_final.contains(&id) == (in_a || want_b.contains(&id)));
            prop_assert!(b_final.contains(&id) == (in_b || want_a.contains(&id)));
            if in_a && in_b {
                prop_assert!(a_final.contains(&id) && b_final.contains(&id));
            }
        }
        // digest match, exactly: every held id in the other's vector (true
        // holdings, rejections and false positives alike) and nothing
        // pushed to us; rejected ids we only remember are not judged
        let a_in_b = a_ids.iter().all(|s| sum_b.contains(&id_of(*s)));
        let b_in_a = b_ids.iter().all(|s| sum_a.contains(&id_of(*s)));
        prop_assert_eq!(a.digest_matched(), a_in_b && got_b.is_empty());
        prop_assert_eq!(b.digest_matched(), b_in_a && got_a.is_empty());
        // same ids on both sides ⇒ both digests match (vectors have no false negatives)
        if a_ids == b_ids {
            prop_assert!(a.digest_matched() && b.digest_matched());
        }
        // a forwarded copy on the other side carries one hop less
        for id in &got_a {
            let (orig, _) = sa.get(id).unwrap();
            let (copy, m) = sb.get(id).unwrap();
            prop_assert_eq!(copy.ttl() + 1, orig.ttl());
            prop_assert_eq!(m.from_link, Some(LinkId(7)));
        }
        // determinism: the same run again gives the same pushes
        let mut a2 = LinkSync::new(SyncConfig::default());
        let mut b2 = LinkSync::new(SyncConfig::default());
        let sa2 = store_of(&a_ids);
        let sb2 = store_of(&b_ids);
        let a0 = a2.start(salt_a, 0, &sa2, &da);
        let b0 = b2.start(salt_b, 0, &sb2, &db);
        let a1 = deliver(b0, &mut a2);
        let b1 = deliver(a0, &mut b2);
        deliver(b1, &mut a2);
        deliver(a1, &mut b2);
        prop_assert_eq!(a2.next_to_send(&sa2, &da, 1_000), got_a);
        prop_assert_eq!(a2.local_chunks(), a.local_chunks());
    }

    /// The dedupe cache's answers depend only on the call sequence and the
    /// powered-on times given.
    #[test]
    fn dedupe_is_deterministic_and_bounded(
        ops in proptest::collection::vec((0u64..40, 0u64..500, any::<bool>()), 0..200),
        max in 1usize..32,
    ) {
        let run = || {
            let mut d = DedupeCache::new(100, max);
            let mut log = Vec::new();
            let mut map: BTreeMap<Hash32, u64> = BTreeMap::new();
            for (n, t, reject) in &ops {
                if *reject {
                    d.mark_rejected(hash(*n), *t);
                } else {
                    log.push(d.seen_or_insert(hash(*n), *t));
                }
                map.insert(hash(*n), *t);
                assert!(d.len() <= max);
                assert!(d.rejected_len() <= d.len());
            }
            (log, d.len(), d.rejected_ids().collect::<Vec<_>>())
        };
        prop_assert_eq!(run(), run());
    }
}
