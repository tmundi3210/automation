//! Public-API tests for `meshcore::relay` (docs/DESIGN.md §10
//! "limits/ramp/lanes/scheduler order; buffer eviction order"; BRIEF_v2
//! §5.5–5.6; DECISIONS D4 PER INBOUND LINK / BUFFER / LINK SLOTS and §3 row
//! (4,8,9,10)).

use std::collections::{BTreeMap, BTreeSet};

use meshcore::clock::ms;
use meshcore::crypto::ed25519::IdentitySigner;
use meshcore::crypto::hash::Hash32;
use meshcore::dtn::store::{EvictionEntry, EvictionPolicy};
use meshcore::relay::Lane;
use meshcore::relay::buffers::BufferPolicy;
use meshcore::relay::lanes::classify;
use meshcore::relay::limits::{
    Admit, BULK_BYTES_PER_MIN_ASM, CONTROL_CELLS_PER_MIN_ASM, LinkBudget, Penalty, RAMP_MS_ASM,
    SCORE_RECOVERY_MS_ASM, TEXT_CELLS_PER_MIN_ASM, THROTTLE_SCORE_ASM,
};
use meshcore::relay::sched::{Band, Scheduler};
use meshcore::relay::slots::{PlatformHint, SlotPolicy};
use meshcore::transport::link::{
    Link, LinkAuth, LinkError, LinkId, LinkInfo, PeerKind, Role, Validation,
};
use meshcore::wire::cell::CELL_LEN;
use meshcore::wire::frame::Class;
use proptest::prelude::*;

const T0: u64 = 5 * ms::MINUTE;

fn id(n: u64) -> Hash32 {
    let mut b = [0u8; 32];
    b[..8].copy_from_slice(&n.to_le_bytes());
    Hash32::from(b)
}

fn cells(b: &mut LinkBudget, class: Class, lane: Lane, n: usize, now: u64) -> usize {
    (0..n)
        .filter(|_| b.admit(class, lane, CELL_LEN, now) == Admit::Accept)
        .count()
}

// ---- lanes ----------------------------------------------------------------------

struct FakeLink {
    auth: LinkAuth,
    info: LinkInfo,
}

impl Link for FakeLink {
    fn id(&self) -> LinkId {
        LinkId(1)
    }
    fn info(&self) -> &LinkInfo {
        &self.info
    }
    fn auth(&self) -> LinkAuth {
        self.auth
    }
    fn send(&mut self, _frame_bytes: &[u8]) -> Result<(), LinkError> {
        Ok(())
    }
    fn recv(&mut self) -> Option<Vec<u8>> {
        None
    }
}

#[test]
fn link_lane_for_delegates_to_classify() {
    let info = LinkInfo {
        role: Role::Central,
        peer_kind_hint: PeerKind::Unknown,
        mtu_hint: 185,
    };
    let contact = LinkAuth::VerifiedContact(IdentitySigner::from_seed(&[3u8; 32]).public());
    let anon = FakeLink {
        auth: LinkAuth::Anonymous,
        info,
    };
    let verified = FakeLink {
        auth: contact,
        info,
    };
    for v in [Validation::Validated, Validation::Opaque] {
        assert_eq!(anon.lane_for(v), classify(v, LinkAuth::Anonymous));
        assert_eq!(verified.lane_for(v), classify(v, contact));
    }
    assert_eq!(anon.lane_for(Validation::Opaque), Lane::L3);
    assert_eq!(verified.lane_for(Validation::Opaque), Lane::L2);
    assert_eq!(verified.lane_for(Validation::Validated), Lane::L1);
}

// ---- limits ---------------------------------------------------------------------

#[test]
fn control_bucket_boundary_twentieth_accepted_twenty_first_throttled_refill_after_a_minute() {
    let mut b = LinkBudget::new(T0, false);
    let n = usize::try_from(CONTROL_CELLS_PER_MIN_ASM).unwrap_or(0);
    for i in 0..n {
        assert_eq!(
            b.admit(Class::C1Control, Lane::L1, CELL_LEN, T0),
            Admit::Accept,
            "cell {i}"
        );
    }
    let throttled = b.admit(Class::C1Control, Lane::L1, CELL_LEN, T0);
    assert_eq!(throttled, Admit::Throttle { retry_ms: 3_000 });
    // Still held one millisecond short of the refill.
    assert!(matches!(
        b.admit(Class::C1Control, Lane::L1, CELL_LEN, T0 + 2_999),
        Admit::Throttle { retry_ms: 1 }
    ));
    assert_eq!(
        b.admit(Class::C0Sos, Lane::L1, CELL_LEN, T0 + 3_000),
        Admit::Accept
    );
    // Sixty seconds after the burst the whole minute is back, no more.
    let t1 = T0 + 3_000 + ms::MINUTE;
    assert_eq!(cells(&mut b, Class::C1Control, Lane::L1, n + 5, t1), n);
    // Other classes have their own buckets.
    assert_eq!(
        cells(&mut b, Class::C2Text, Lane::L1, 100, t1),
        usize::try_from(TEXT_CELLS_PER_MIN_ASM).unwrap_or(0)
    );
}

#[test]
fn bulk_bucket_is_bytes_per_minute() {
    let mut b = LinkBudget::new(T0, false);
    let cap = usize::try_from(BULK_BYTES_PER_MIN_ASM).unwrap_or(0);
    assert_eq!(
        b.admit(Class::C4Preview, Lane::L1, cap - 1, T0),
        Admit::Accept
    );
    assert_eq!(b.admit(Class::C4Preview, Lane::L1, 1, T0), Admit::Accept);
    assert_eq!(
        b.admit(Class::C4Preview, Lane::L1, 1, T0),
        Admit::Throttle { retry_ms: 1 }
    );
    assert_eq!(
        b.admit(Class::C4Preview, Lane::L1, cap + 1, T0),
        Admit::Reject
    );
    assert_eq!(
        b.admit(Class::C4Preview, Lane::L1, cap, T0 + ms::MINUTE),
        Admit::Accept
    );
}

#[test]
fn l3_is_capped_at_a_quarter_except_on_stations() {
    let full = T0 + RAMP_MS_ASM;
    let mut relay = LinkBudget::new(T0, false);
    assert_eq!(cells(&mut relay, Class::C1Control, Lane::L3, 20, full), 5);
    assert!(matches!(
        relay.admit(Class::C1Control, Lane::L3, CELL_LEN, full),
        Admit::Throttle { .. }
    ));
    // The other three quarters stay open to validated traffic.
    assert_eq!(cells(&mut relay, Class::C1Control, Lane::L1, 20, full), 15);
    // Text too: 60/min → 15 opaque.
    let mut relay = LinkBudget::new(T0, false);
    assert_eq!(cells(&mut relay, Class::C2Text, Lane::L3, 100, full), 15);

    let mut station = LinkBudget::new(T0, true);
    assert!(station.is_station());
    assert_eq!(
        cells(&mut station, Class::C1Control, Lane::L3, 25, full),
        20
    );
    let mut station = LinkBudget::new(T0, true);
    assert_eq!(cells(&mut station, Class::C2Text, Lane::L3, 100, full), 60);
}

#[test]
fn ramp_limits_only_opaque_traffic() {
    // At link creation L3 gets 25 % of its share; L1 and L2 get everything.
    let mut b = LinkBudget::new(T0, false);
    assert!(b.in_ramp(T0));
    assert_eq!(cells(&mut b, Class::C1Control, Lane::L3, 10, T0), 1);
    let mut b = LinkBudget::new(T0, false);
    assert_eq!(cells(&mut b, Class::C1Control, Lane::L1, 25, T0), 20);
    let mut b = LinkBudget::new(T0, false);
    assert_eq!(cells(&mut b, Class::C2Text, Lane::L2, 1_000, T0), 120);
    // Half way through the ramp: 0.625 × 25 % × 20 = 3.125.
    let mut b = LinkBudget::new(T0, false);
    assert_eq!(
        cells(&mut b, Class::C1Control, Lane::L3, 10, T0 + RAMP_MS_ASM / 2),
        3
    );
    // After the ramp: the full quarter.
    let mut b = LinkBudget::new(T0, false);
    assert!(!b.in_ramp(T0 + RAMP_MS_ASM));
    assert_eq!(
        cells(&mut b, Class::C1Control, Lane::L3, 10, T0 + RAMP_MS_ASM),
        5
    );
    // The ramp is per link: a link made later ramps later.
    let mut late = LinkBudget::new(T0 + RAMP_MS_ASM, false);
    assert_eq!(
        cells(&mut late, Class::C1Control, Lane::L3, 10, T0 + RAMP_MS_ASM),
        1
    );
    // A station is all L3, so the ramp applies there too (DECISIONS §3 row
    // (4,8,9,10) problem statement), from 25 % of the whole bucket.
    let mut s = LinkBudget::new(T0, true);
    assert_eq!(cells(&mut s, Class::C1Control, Lane::L3, 25, T0), 5);
}

#[test]
fn penalties_throttle_then_reject_and_recover_over_time() {
    let mut b = LinkBudget::new(T0, false);
    assert_eq!(b.score(), 1.0);
    b.penalise(Penalty::BadSignature, T0);
    b.penalise(Penalty::NeverMember, T0);
    assert!(b.score() >= THROTTLE_SCORE_ASM);
    assert_eq!(cells(&mut b, Class::C2Text, Lane::L1, 100, T0), 60);
    b.penalise(Penalty::RemovedInHeads, T0);
    assert!(b.score() < THROTTLE_SCORE_ASM);
    assert!(b.is_throttled_by_score(T0));
    // Budget halved: control gives 10 instead of 20.
    assert_eq!(cells(&mut b, Class::C1Control, Lane::L1, 20, T0), 10);
    let Admit::Throttle { retry_ms } = b.admit(Class::C1Control, Lane::L1, CELL_LEN, T0) else {
        panic!("expected throttle");
    };
    assert_eq!(retry_ms, 6_000); // a doubled cost at the normal rate
    // Overflow penalties are lighter but add up to a graylist.
    b.penalise(Penalty::Overflow, T0);
    b.penalise(Penalty::Overflow, T0);
    assert_eq!(b.admit(Class::C4Preview, Lane::L1, 1, T0), Admit::Reject);
    // Recovery is linear in powered-on time; half way the score is back
    // above the throttle line.
    let half = T0 + SCORE_RECOVERY_MS_ASM / 2;
    assert!(b.score_at(half) > THROTTLE_SCORE_ASM);
    assert!(!b.is_throttled_by_score(half));
    assert_eq!(b.score_at(T0 + SCORE_RECOVERY_MS_ASM), 1.0);
    // Time going backwards never raises the score.
    assert!(b.score_at(0) < THROTTLE_SCORE_ASM);
}

// ---- scheduler ------------------------------------------------------------------

#[test]
fn scheduler_serves_all_seven_bands_in_order() {
    let mut s = Scheduler::new();
    // (class, lane) per band, enqueued lowest priority first.
    let bands = [
        (Band::Preview, Class::C4Preview, Lane::L1),
        (Band::NewsOther, Class::C3News, Lane::L3),
        (Band::TextOpaque, Class::C2Text, Lane::L3),
        (Band::TextVouched, Class::C2Text, Lane::L2),
        (Band::NewsVerified, Class::C3News, Lane::L1),
        (Band::Control, Class::C1Control, Lane::L3),
        (Band::Sos, Class::C0Sos, Lane::L3),
    ];
    for (i, (band, class, lane)) in bands.iter().enumerate() {
        let n = u64::try_from(i).unwrap_or(0);
        assert!(s.enqueue(id(n), *class, *lane, Some(LinkId(7))));
        assert_eq!(s.band_of(&id(n)), Some(*band));
    }
    assert_eq!(s.len(), 7);
    let picked: Vec<Hash32> = std::iter::from_fn(|| s.pick()).collect();
    let expected: Vec<Hash32> = (0..7).rev().map(id).collect();
    assert_eq!(picked, expected);
    assert!(s.is_empty());
}

#[test]
fn scheduler_is_fair_across_inbound_links() {
    let mut s = Scheduler::new();
    let links = [LinkId(10), LinkId(20), LinkId(30)];
    // Three links, two text cells each, enqueued link by link.
    for (li, link) in links.iter().enumerate() {
        for k in 0..2u64 {
            let n = u64::try_from(li).unwrap_or(0) * 10 + k;
            assert!(s.enqueue(id(n), Class::C2Text, Lane::L3, Some(*link)));
        }
    }
    // A later-arriving link gets a turn in the same round.
    let mut seen_first_round: BTreeMap<LinkId, u64> = BTreeMap::new();
    for _ in 0..3 {
        let m = s.pick().expect("queued");
        let n = u64::from_le_bytes(<[u8; 8]>::try_from(&m.as_ref()[..8]).expect("8 bytes"));
        let link = links[usize::try_from(n / 10).expect("index")];
        assert!(
            seen_first_round.insert(link, n).is_none(),
            "{link:?} served twice before all had a turn"
        );
    }
    assert_eq!(seen_first_round.len(), 3);
    // Second round: the remaining cell of each, again one per link.
    let second: BTreeSet<u64> = (0..3)
        .map(|_| {
            let m = s.pick().expect("queued");
            u64::from_le_bytes(<[u8; 8]>::try_from(&m.as_ref()[..8]).expect("8 bytes")) / 10
        })
        .collect();
    assert_eq!(second.len(), 3);
    assert_eq!(s.pick(), None);
    // Own traffic (no inbound link) is one more fair-share source, and a
    // higher band always wins first.
    s.enqueue(id(100), Class::C2Text, Lane::L1, None);
    s.enqueue(id(101), Class::C2Text, Lane::L1, Some(LinkId(10)));
    s.enqueue(id(102), Class::C1Control, Lane::L3, Some(LinkId(20)));
    assert_eq!(s.pick(), Some(id(102)));
    let rest: BTreeSet<Hash32> = std::iter::from_fn(|| s.pick()).collect();
    assert_eq!(rest, BTreeSet::from([id(100), id(101)]));
}

#[test]
fn scheduler_busy_hint_and_remove() {
    let mut s = Scheduler::new();
    assert!(!s.busy_hint());
    s.set_held(true);
    assert!(!s.busy_hint(), "an empty queue is never busy");
    s.enqueue(id(1), Class::C2Text, Lane::L3, Some(LinkId(1)));
    assert!(s.busy_hint());
    assert!(s.remove(&id(1)));
    assert!(!s.busy_hint());
    assert_eq!(s.pick(), None);
    s.set_held(false);
    s.enqueue(id(2), Class::C2Text, Lane::L3, Some(LinkId(1)));
    assert!(!s.busy_hint());
}

/// Reviewer regression: a message removed (evicted, expired) and later
/// re-queued under the same band and inbound link takes its place at the
/// back of that link's FIFO, not the slot its stale entry left at the front.
#[test]
fn scheduler_keeps_fifo_within_a_link_after_a_requeue() {
    let mut s = Scheduler::new();
    let link = Some(LinkId(4));
    assert!(s.enqueue(id(1), Class::C2Text, Lane::L3, link));
    assert!(s.enqueue(id(2), Class::C2Text, Lane::L3, link));
    assert!(s.enqueue(id(3), Class::C2Text, Lane::L3, link));
    assert!(s.remove(&id(1)));
    assert!(s.enqueue(id(1), Class::C2Text, Lane::L3, link));
    let picked: Vec<Hash32> = std::iter::from_fn(|| s.pick()).collect();
    assert_eq!(picked, vec![id(2), id(3), id(1)]);
    // The same after the link's entries were dropped wholesale.
    assert!(s.enqueue(id(1), Class::C2Text, Lane::L3, link));
    assert!(s.enqueue(id(2), Class::C2Text, Lane::L3, None));
    assert_eq!(s.remove_from_link(LinkId(4)), 1);
    assert!(s.enqueue(id(1), Class::C2Text, Lane::L3, link));
    assert_eq!(s.pick(), Some(id(2)));
    assert_eq!(s.pick(), Some(id(1)));
    assert_eq!(s.pick(), None);
}

/// Reviewer regression: `available` is the number the status strip shows,
/// so it must equal what `admit` would grant, including L2's 2x quota, the
/// halved budget below `THROTTLE_SCORE_ASM` and the graylist.
#[test]
fn available_reports_what_the_lane_can_draw() {
    let t = T0 + RAMP_MS_ASM;
    let mut b = LinkBudget::new(T0, false);
    assert_eq!(b.available(Class::C2Text, Lane::L1, t), 60);
    assert_eq!(b.available(Class::C2Text, Lane::L2, t), 120);
    assert_eq!(b.available(Class::C2Text, Lane::L3, t), 15);
    assert_eq!(cells(&mut b, Class::C2Text, Lane::L2, 1_000, t), 120);
    assert_eq!(b.available(Class::C2Text, Lane::L2, t), 0);
    let mut b = LinkBudget::new(T0, false);
    b.penalise(Penalty::BadSignature, t);
    b.penalise(Penalty::NeverMember, t);
    b.penalise(Penalty::RemovedInHeads, t);
    assert!(b.is_throttled_by_score(t));
    assert_eq!(b.available(Class::C1Control, Lane::L1, t), 10);
    assert_eq!(b.available(Class::C1Control, Lane::L2, t), 20);
    assert_eq!(b.available(Class::C1Control, Lane::L3, t), 2);
    assert_eq!(cells(&mut b, Class::C1Control, Lane::L1, 100, t), 10);
    b.penalise(Penalty::Overflow, t);
    b.penalise(Penalty::Overflow, t);
    assert_eq!(
        b.admit(Class::C1Control, Lane::L1, CELL_LEN, t),
        Admit::Reject
    );
    assert_eq!(b.available(Class::C1Control, Lane::L1, t), 0);
    assert_eq!(b.available(Class::C4Preview, Lane::L2, t), 0);
}

// ---- buffers ----------------------------------------------------------------------

fn arb_class() -> impl Strategy<Value = Class> {
    prop::sample::select(Class::ALL.to_vec())
}

fn arb_lane() -> impl Strategy<Value = Lane> {
    prop::sample::select(Lane::ALL.to_vec())
}

/// An eviction row without its id (ids are assigned by position).
fn arb_row() -> impl Strategy<Value = (Class, Lane, u8, u64, bool)> {
    (arb_class(), arb_lane(), 0u8..=8, 0u64..1_000, any::<bool>())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// D4 BUFFER / BRIEF_v2 §5.5 eviction order as a property: never a
    /// pinned row, never a class the newcomer does not outrank, L3 of a
    /// class before any L1/L2 of it, and the same answer whatever the
    /// input order.
    #[test]
    fn eviction_order_property(
        rows in prop::collection::vec(arb_row(), 0..40),
        need_cells in 0usize..45,
        newcomer in arb_class(),
        shuffle_seed in any::<u64>(),
    ) {
        let policy = BufferPolicy::default();
        let entries: Vec<EvictionEntry> = rows
            .iter()
            .enumerate()
            .map(|(i, (class, lane, ttl, first_seen_mono, pinned))| EvictionEntry {
                id: id(u64::try_from(i).unwrap_or(0)),
                class: *class,
                lane: *lane,
                first_seen_mono: *first_seen_mono,
                pinned: *pinned,
                ttl: *ttl,
            })
            .collect();
        let need_bytes = need_cells * CELL_LEN;
        let victims = policy.victims_for(&entries, need_bytes, newcomer);
        let by_id: BTreeMap<Hash32, &EvictionEntry> = entries.iter().map(|e| (e.id, e)).collect();
        let set: BTreeSet<Hash32> = victims.iter().copied().collect();
        prop_assert_eq!(set.len(), victims.len(), "duplicates");
        for v in &victims {
            let e = by_id.get(v).expect("victim is an entry");
            prop_assert!(!e.pinned, "pinned row evicted");
            prop_assert!(!e.class.outranks(newcomer), "{:?} evicted for a {:?} newcomer", e.class, newcomer);
        }
        // Within a class, every eligible L3 row goes before any L1/L2 row.
        for class in Class::ALL {
            let vouched_victim = victims.iter().any(|v| {
                let e = by_id[v];
                e.class == class && e.lane != Lane::L3
            });
            if vouched_victim {
                for e in &entries {
                    if e.class == class && e.lane == Lane::L3 && !e.pinned {
                        prop_assert!(set.contains(&e.id), "L3 row {:?} survived while an L1/L2 row of {:?} was evicted", e.id, class);
                    }
                }
            }
        }
        // A lower class is always exhausted before a higher one is touched.
        for e in &entries {
            if e.pinned || e.class.outranks(newcomer) || set.contains(&e.id) {
                continue;
            }
            // Survivor: no victim may outrank it.
            for v in &victims {
                prop_assert!(!by_id[v].class.outranks(e.class), "evicted {:?} while {:?} survived", by_id[v].class, e.class);
            }
        }
        // Size: exactly the need when possible, else everything eligible.
        let eligible = entries.iter().filter(|e| !e.pinned && !e.class.outranks(newcomer)).count();
        prop_assert_eq!(victims.len(), need_cells.min(eligible));
        // Determinism under permutation.
        let mut shuffled = entries.clone();
        let mut x = shuffle_seed | 1;
        for i in (1..shuffled.len()).rev() {
            x ^= x << 13; x ^= x >> 7; x ^= x << 17;
            let j = usize::try_from(x % (u64::try_from(i).unwrap_or(1) + 1)).unwrap_or(0);
            shuffled.swap(i, j);
        }
        prop_assert_eq!(policy.victims_for(&shuffled, need_bytes, newcomer), victims.clone());
        // The store's trait entry point keys on the incoming cell's class.
        let incoming = EvictionEntry {
            id: id(u64::MAX),
            class: newcomer,
            lane: Lane::L3,
            first_seen_mono: 1_000,
            pinned: false,
            ttl: 8,
        };
        prop_assert_eq!(policy.victims(&entries, &incoming, need_bytes), victims);
    }
}

#[test]
fn buffer_fits_applies_both_caps_and_the_reserve() {
    let p = BufferPolicy::default();
    let general = p.general_cap_bytes();
    assert_eq!(p.reserve_bytes() * 5, p.total_cap_bytes);
    assert!(p.fits(
        general - CELL_LEN,
        p.per_link_cap_bytes - CELL_LEN,
        CELL_LEN
    ));
    assert!(!p.fits(general - CELL_LEN + 1, 0, CELL_LEN));
    assert!(!p.fits(0, p.per_link_cap_bytes - CELL_LEN + 1, CELL_LEN));
    assert!(p.fits_for_absent_peer(general, CELL_LEN));
    assert!(!p.fits_for_absent_peer(p.total_cap_bytes, 1));
}

// ---- slots --------------------------------------------------------------------------

#[test]
fn slot_hints_keep_half_the_slots_for_outbound() {
    let mut p = SlotPolicy::new(7);
    assert_eq!(p.outbound_reserved(), 4);
    assert_eq!(p.inbound_cap(), 3);
    let mut hints = Vec::new();
    for _ in 0..3 {
        hints.push(p.on_link_up(Role::Peripheral));
    }
    assert_eq!(
        hints,
        vec![None, None, Some(PlatformHint::StopAcceptingInbound)]
    );
    assert!(!p.accepting_inbound());
    // Outbound links are unaffected by the inbound stop.
    assert!(p.can_dial());
    assert_eq!(p.on_link_up(Role::Central), None);
    assert_eq!(
        p.on_link_down(Role::Peripheral),
        Some(PlatformHint::ResumeInbound)
    );
    assert_eq!(p.on_link_down(Role::Peripheral), None);
    // Filling the device cap with outbound links also stops inbound.
    for _ in 0..4 {
        assert_eq!(p.on_link_up(Role::Central), None);
    }
    assert_eq!(p.total(), 6);
    assert_eq!(
        p.on_link_up(Role::Central),
        Some(PlatformHint::StopAcceptingInbound)
    );
    assert_eq!(
        p.on_link_down(Role::Central),
        Some(PlatformHint::ResumeInbound)
    );
}

// ---- never panics --------------------------------------------------------------------

#[derive(Debug, Clone)]
enum BudgetOp {
    Admit {
        class: Class,
        lane: Lane,
        bytes: usize,
        now: u64,
    },
    Penalise {
        reason: Penalty,
        now: u64,
    },
    Score {
        now: u64,
    },
}

fn arb_now() -> impl Strategy<Value = u64> {
    prop_oneof![
        5 => 0u64..(2 * ms::HOUR),
        1 => any::<u64>(),
    ]
}

fn arb_bytes() -> impl Strategy<Value = usize> {
    prop_oneof![
        5 => 0usize..(4 * 1024 * 1024),
        1 => any::<usize>(),
    ]
}

fn arb_budget_op() -> impl Strategy<Value = BudgetOp> {
    prop_oneof![
        6 => (arb_class(), arb_lane(), arb_bytes(), arb_now())
            .prop_map(|(class, lane, bytes, now)| BudgetOp::Admit { class, lane, bytes, now }),
        1 => (
            prop::sample::select(vec![
                Penalty::BadSignature,
                Penalty::NeverMember,
                Penalty::RemovedInHeads,
                Penalty::Overflow,
            ]),
            arb_now()
        )
            .prop_map(|(reason, now)| BudgetOp::Penalise { reason, now }),
        1 => arb_now().prop_map(|now| BudgetOp::Score { now }),
    ]
}

#[derive(Debug, Clone)]
enum SchedOp {
    Enqueue {
        n: u64,
        class: Class,
        lane: Lane,
        link: Option<u32>,
    },
    Pick,
    Remove {
        n: u64,
    },
    Held(bool),
    DropLink(u32),
    Compact,
}

fn arb_sched_op() -> impl Strategy<Value = SchedOp> {
    prop_oneof![
        5 => (0u64..64, arb_class(), arb_lane(), prop::option::of(0u32..5))
            .prop_map(|(n, class, lane, link)| SchedOp::Enqueue { n, class, lane, link }),
        4 => Just(SchedOp::Pick),
        2 => (0u64..64).prop_map(|n| SchedOp::Remove { n }),
        1 => any::<bool>().prop_map(SchedOp::Held),
        1 => (0u32..5).prop_map(SchedOp::DropLink),
        1 => Just(SchedOp::Compact),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn admit_never_panics_and_only_accept_consumes(
        start in arb_now(),
        station in any::<bool>(),
        ops in prop::collection::vec(arb_budget_op(), 0..64),
    ) {
        let mut b = LinkBudget::new(start, station);
        for op in ops {
            match op {
                BudgetOp::Admit { class, lane, bytes, now } => {
                    let before = b.clone();
                    let verdict = b.admit(class, lane, bytes, now);
                    let s = b.score_at(now);
                    prop_assert!((0.0..=1.0).contains(&s));
                    match verdict {
                        Admit::Accept => {}
                        Admit::Throttle { retry_ms } => {
                            prop_assert!(retry_ms >= 1);
                            // Nothing consumed: the same request right away is held again
                            // with the same hint, and what was available is still available.
                            prop_assert_eq!(b.admit(class, lane, bytes, now), verdict);
                            prop_assert_eq!(b.available(class, lane, now), before.available(class, lane, now));
                        }
                        Admit::Reject => {
                            prop_assert_eq!(b.admit(class, lane, bytes, now), verdict);
                            prop_assert_eq!(b.available(class, lane, now), before.available(class, lane, now));
                        }
                    }
                }
                BudgetOp::Penalise { reason, now } => {
                    let before = b.score_at(now);
                    b.penalise(reason, now);
                    prop_assert!(b.score_at(now) <= before);
                }
                BudgetOp::Score { now } => {
                    let s = b.score_at(now);
                    prop_assert!((0.0..=1.0).contains(&s));
                }
            }
        }
    }

    #[test]
    fn scheduler_never_panics_and_stays_consistent(ops in prop::collection::vec(arb_sched_op(), 0..128)) {
        let mut s = Scheduler::new();
        let mut model: BTreeSet<u64> = BTreeSet::new();
        for op in ops {
            match op {
                SchedOp::Enqueue { n, class, lane, link } => {
                    let fresh = s.enqueue(id(n), class, lane, link.map(LinkId));
                    prop_assert_eq!(fresh, model.insert(n));
                    if fresh {
                        prop_assert_eq!(s.band_of(&id(n)), Some(Band::of(class, lane)));
                    } else {
                        prop_assert!(s.band_of(&id(n)).is_some());
                    }
                }
                SchedOp::Pick => {
                    let picked = s.pick();
                    prop_assert_eq!(picked.is_some(), !model.is_empty());
                    if let Some(m) = picked {
                        let n = u64::from_le_bytes(<[u8; 8]>::try_from(&m.as_ref()[..8]).expect("8 bytes"));
                        prop_assert!(model.remove(&n), "picked something not queued");
                        prop_assert!(!s.contains(&m));
                    }
                }
                SchedOp::Remove { n } => {
                    prop_assert_eq!(s.remove(&id(n)), model.remove(&n));
                }
                SchedOp::Held(h) => {
                    s.set_held(h);
                    prop_assert_eq!(s.busy_hint(), h && !model.is_empty());
                }
                SchedOp::DropLink(l) => {
                    let dropped = s.remove_from_link(LinkId(l));
                    prop_assert!(dropped <= model.len());
                    // Rebuild the model from the scheduler's own view.
                    model.retain(|n| s.contains(&id(*n)));
                    prop_assert_eq!(s.len(), model.len());
                }
                SchedOp::Compact => s.compact(),
            }
            prop_assert_eq!(s.len(), model.len());
            prop_assert_eq!(s.is_empty(), model.is_empty());
        }
        // Everything left drains in strict band order: each pick comes from
        // the highest-priority band that still has something queued.
        loop {
            let bands: BTreeMap<u64, Band> = model
                .iter()
                .map(|n| (*n, s.band_of(&id(*n)).expect("queued")))
                .collect();
            let top = bands.values().min().copied();
            let Some(m) = s.pick() else {
                prop_assert!(top.is_none());
                break;
            };
            let n = u64::from_le_bytes(<[u8; 8]>::try_from(&m.as_ref()[..8]).expect("8 bytes"));
            prop_assert_eq!(bands.get(&n).copied(), top);
            model.remove(&n);
        }
        prop_assert!(s.is_empty());
    }

    /// A `Throttle { retry_ms }` is a promise (DESIGN §2.3 Throttle frame):
    /// with nothing else admitted meanwhile, the same request is accepted
    /// `retry_ms` later. Time is monotone here, as a powered-on clock is.
    #[test]
    fn throttle_retry_hint_is_honoured(
        start in 0u64..ms::HOUR,
        station in any::<bool>(),
        ops in prop::collection::vec(arb_timed_op(), 1..40),
    ) {
        let mut b = LinkBudget::new(start, station);
        let mut now = start;
        for op in ops {
            match op {
                TimedOp::Admit { class, lane, bytes, dt } => {
                    now = now.saturating_add(dt);
                    if let Admit::Throttle { retry_ms } = b.admit(class, lane, bytes, now) {
                        let mut later = b.clone();
                        let at = now.saturating_add(u64::from(retry_ms));
                        prop_assert_eq!(
                            later.admit(class, lane, bytes, at),
                            Admit::Accept,
                            "{:?} on {:?}, {} bytes, held at age {} for {} ms",
                            class, lane, bytes, now - start, retry_ms
                        );
                    }
                }
                TimedOp::Penalise { reason, dt } => {
                    now = now.saturating_add(dt);
                    b.penalise(reason, now);
                }
            }
        }
    }

    /// BRIEF_v2 §5.5 "L3 … ≤25 %": at no age of a non-station link can an
    /// opaque burst take more than a quarter of a bucket, while validated
    /// traffic always gets the whole bucket; on a station the ramp alone
    /// bounds L3.
    #[test]
    fn l3_burst_never_exceeds_its_share_at_any_age(
        start in 0u64..ms::HOUR,
        age in 0u64..(2 * RAMP_MS_ASM),
    ) {
        let t = start + age;
        let mut b = LinkBudget::new(start, false);
        let opaque = cells(&mut b, Class::C1Control, Lane::L3, 100, t);
        prop_assert!(opaque <= 5, "L3 control burst {} at age {}", opaque, age);
        // Whatever L3 took, L1 gets the rest of the bucket.
        prop_assert_eq!(cells(&mut b, Class::C1Control, Lane::L1, 100, t), 20 - opaque);
        let mut b = LinkBudget::new(start, false);
        prop_assert_eq!(cells(&mut b, Class::C1Control, Lane::L1, 100, t), 20);
        let mut b = LinkBudget::new(start, false);
        prop_assert!(cells(&mut b, Class::C2Text, Lane::L3, 1_000, t) <= 15);
        let mut s = LinkBudget::new(start, true);
        let station = cells(&mut s, Class::C1Control, Lane::L3, 100, t);
        prop_assert!((5..=20).contains(&station), "station L3 burst {} at age {}", station, age);
        if age >= RAMP_MS_ASM {
            prop_assert_eq!(station, 20);
        }
    }
}

#[derive(Debug, Clone)]
enum TimedOp {
    Admit {
        class: Class,
        lane: Lane,
        bytes: usize,
        dt: u64,
    },
    Penalise {
        reason: Penalty,
        dt: u64,
    },
}

fn arb_timed_op() -> impl Strategy<Value = TimedOp> {
    let bytes = prop_oneof![4 => 0usize..(3 * CELL_LEN), 1 => 0usize..70_000];
    let dt = 0u64..(2 * ms::MINUTE);
    prop_oneof![
        8 => (arb_class(), arb_lane(), bytes, dt.clone())
            .prop_map(|(class, lane, bytes, dt)| TimedOp::Admit { class, lane, bytes, dt }),
        1 => (
            prop::sample::select(vec![
                Penalty::BadSignature,
                Penalty::NeverMember,
                Penalty::RemovedInHeads,
                Penalty::Overflow,
            ]),
            dt
        )
            .prop_map(|(reason, dt)| TimedOp::Penalise { reason, dt }),
    ]
}
