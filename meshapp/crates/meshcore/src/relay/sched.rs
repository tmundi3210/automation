//! Relay send scheduler (BRIEF_v2 §5.6 "Strict priority on every queue";
//! DECISIONS §3 row (4,8,9,10) scheduling order; D4 PER INBOUND LINK "fair
//! queueing by input link"; DESIGN §1 `relay/sched.rs`).
//!
//! Seven strict priority bands ([`Band`]), highest first:
//! C0 > C1 > C3 verified at hop 1 (lane L1; the P2 News lane, slot kept so
//! the order is wired now) > C2 on L1/L2 > C2 on L3 > other C3 > C4.
//! Inside a band the scheduler round-robins over the inbound link each
//! message arrived on, so one chatty link cannot starve the others
//! (D4 "fair queueing by input link"; GossipSub D_out analogue, Q5).
//!
//! The scheduler holds message ids only ([`Hash32`], DESIGN §2.2 `msg_id`);
//! the DTN store owns the bytes. [`Scheduler::busy_hint`] is the signal
//! behind "Waiting · network busy" (BRIEF_v2 §5.9, D4 UI): something is
//! queued and a budget is holding it.

use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::crypto::hash::Hash32;
use crate::relay::Lane;
use crate::transport::link::LinkId;
use crate::wire::frame::Class;

/// Priority band (BRIEF_v2 §5.6 order as resolved by DECISIONS §3 row
/// (4,8,9,10)). Lower discriminant = served first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Band {
    /// C0 SOS (reserved at P0).
    Sos = 0,
    /// C1 control: roster ops, Sender Key bundles, vote-opens, ballots,
    /// task ops, revocations.
    Control = 1,
    /// C3 News whose chain the first hop checked (lane L1). P2; the slot is
    /// kept so the order never changes.
    NewsVerified = 2,
    /// C2 text on L1/L2.
    TextVouched = 3,
    /// C2 text on L3.
    TextOpaque = 4,
    /// Other C3 News (L2/L3).
    NewsOther = 5,
    /// C4 preview.
    Preview = 6,
}

impl Band {
    /// All bands, highest priority first.
    pub const ALL: [Band; 7] = [
        Band::Sos,
        Band::Control,
        Band::NewsVerified,
        Band::TextVouched,
        Band::TextOpaque,
        Band::NewsOther,
        Band::Preview,
    ];

    /// Band of a message by class and lane.
    pub const fn of(class: Class, lane: Lane) -> Band {
        match (class, lane) {
            (Class::C0Sos, _) => Band::Sos,
            (Class::C1Control, _) => Band::Control,
            (Class::C3News, Lane::L1) => Band::NewsVerified,
            (Class::C2Text, Lane::L1 | Lane::L2) => Band::TextVouched,
            (Class::C2Text, Lane::L3) => Band::TextOpaque,
            (Class::C3News, Lane::L2 | Lane::L3) => Band::NewsOther,
            (Class::C4Preview, _) => Band::Preview,
        }
    }

    const fn index(self) -> usize {
        self as usize
    }
}

/// Queue key inside a band: the inbound link, or `None` for this phone's
/// own traffic (which takes a fair turn like any link).
type Source = Option<LinkId>;

/// Where a queued id lives. The ticket is unique per `enqueue`, so a queue
/// entry left behind by a lazy `remove` can never be mistaken for a later
/// re-queue of the same id under the same band and source (that would serve
/// the re-queued id from its old, earlier position and break FIFO within
/// the source).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Slot {
    band: Band,
    source: Source,
    ticket: u64,
}

/// One priority band: a FIFO per source plus the round-robin cursor.
#[derive(Debug, Default)]
struct BandQueue {
    queues: BTreeMap<Source, VecDeque<(Hash32, u64)>>,
    /// Source served last; the next pick starts after it.
    cursor: Option<Source>,
}

impl BandQueue {
    /// The source whose turn is next: the first key after the cursor, or
    /// the first key when the cursor is past the end (or unset).
    fn next_source(&self) -> Option<Source> {
        if self.queues.is_empty() {
            return None;
        }
        let after = self.cursor.and_then(|c| {
            self.queues
                .range((std::ops::Bound::Excluded(c), std::ops::Bound::Unbounded))
                .next()
                .map(|(k, _)| *k)
        });
        after.or_else(|| self.queues.keys().next().copied())
    }
}

/// Strict-priority, link-fair scheduler over message ids.
#[derive(Debug, Default)]
pub struct Scheduler {
    bands: [BandQueue; 7],
    /// Where each queued id lives. Lazy deletion: an id removed from here
    /// may still sit in a band queue until a pick skips it; the ticket in
    /// the queue entry must match for it to be live.
    index: HashMap<Hash32, Slot>,
    held: bool,
    /// Ticket for the next `enqueue`.
    next_ticket: u64,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues `msg_id`. Returns `false` (and changes nothing) if it is
    /// already queued: a message is scheduled once however many links
    /// re-offer it (dedupe is the store's job; this only keeps the queue
    /// consistent).
    pub fn enqueue(
        &mut self,
        msg_id: Hash32,
        class: Class,
        lane: Lane,
        from_link: Option<LinkId>,
    ) -> bool {
        if self.index.contains_key(&msg_id) {
            return false;
        }
        let band = Band::of(class, lane);
        let ticket = self.next_ticket;
        self.next_ticket = self.next_ticket.wrapping_add(1);
        self.bands[band.index()]
            .queues
            .entry(from_link)
            .or_default()
            .push_back((msg_id, ticket));
        self.index.insert(
            msg_id,
            Slot {
                band,
                source: from_link,
                ticket,
            },
        );
        true
    }

    /// Next message to send: the first non-empty band, then the next source
    /// in round-robin order within it.
    pub fn pick(&mut self) -> Option<Hash32> {
        for band in Band::ALL {
            let bq = &mut self.bands[band.index()];
            while let Some(source) = bq.next_source() {
                let Some(queue) = bq.queues.get_mut(&source) else {
                    break;
                };
                // Pop until a live entry or the queue is drained.
                let mut served = None;
                while let Some((id, ticket)) = queue.pop_front() {
                    let live = Slot {
                        band,
                        source,
                        ticket,
                    };
                    if self.index.get(&id) == Some(&live) {
                        served = Some(id);
                        break;
                    }
                }
                let drained = queue.is_empty();
                if drained {
                    bq.queues.remove(&source);
                }
                if let Some(id) = served {
                    self.index.remove(&id);
                    bq.cursor = Some(source);
                    return Some(id);
                }
                // Only stale entries under this source: move on (the
                // source is gone now, so the loop cannot spin on it).
            }
        }
        None
    }

    /// Drops `msg_id` from the queue (expired, evicted, or sent by another
    /// path). Returns `true` if it was queued.
    pub fn remove(&mut self, msg_id: &Hash32) -> bool {
        self.index.remove(msg_id).is_some()
    }

    /// `true` if `msg_id` is queued.
    pub fn contains(&self, msg_id: &Hash32) -> bool {
        self.index.contains_key(msg_id)
    }

    /// Band a queued id is in, if any.
    pub fn band_of(&self, msg_id: &Hash32) -> Option<Band> {
        self.index.get(msg_id).map(|s| s.band)
    }

    /// Number of queued messages.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Queued messages in one band.
    pub fn len_in(&self, band: Band) -> usize {
        self.index.values().filter(|s| s.band == band).count()
    }

    /// The caller sets this from `relay::limits` when the last `admit` on
    /// the outbound side came back `Throttle` (or the link said `Busy`).
    pub fn set_held(&mut self, held: bool) {
        self.held = held;
    }

    pub fn is_held(&self) -> bool {
        self.held
    }

    /// `true` when something is queued but a budget holds it: drives
    /// "Waiting · network busy" (BRIEF_v2 §5.9; DECISIONS D4 UI). Never true
    /// for an empty queue, so an idle relay cannot show busy.
    pub fn busy_hint(&self) -> bool {
        self.held && !self.index.is_empty()
    }

    /// Forgets queued entries whose source link went down, if the caller
    /// wants to stop carrying them (the DTN store still has the bytes).
    /// Returns how many were dropped.
    pub fn remove_from_link(&mut self, link: LinkId) -> usize {
        let before = self.index.len();
        self.index.retain(|_, s| s.source != Some(link));
        before - self.index.len()
    }

    /// Drops the stale entries lazy removal left behind, bounding memory
    /// after many `remove` calls. Order is preserved.
    pub fn compact(&mut self) {
        for (i, bq) in self.bands.iter_mut().enumerate() {
            let band = Band::ALL[i];
            bq.queues.retain(|source, queue| {
                queue.retain(|(id, ticket)| {
                    let live = Slot {
                        band,
                        source: *source,
                        ticket: *ticket,
                    };
                    self.index.get(id) == Some(&live)
                });
                !queue.is_empty()
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u8) -> Hash32 {
        Hash32::from([n; 32])
    }

    #[test]
    fn band_order_matches_decisions_row() {
        assert_eq!(Band::of(Class::C0Sos, Lane::L3), Band::Sos);
        assert_eq!(Band::of(Class::C1Control, Lane::L3), Band::Control);
        assert_eq!(Band::of(Class::C3News, Lane::L1), Band::NewsVerified);
        assert_eq!(Band::of(Class::C2Text, Lane::L1), Band::TextVouched);
        assert_eq!(Band::of(Class::C2Text, Lane::L2), Band::TextVouched);
        assert_eq!(Band::of(Class::C2Text, Lane::L3), Band::TextOpaque);
        assert_eq!(Band::of(Class::C3News, Lane::L2), Band::NewsOther);
        assert_eq!(Band::of(Class::C3News, Lane::L3), Band::NewsOther);
        assert_eq!(Band::of(Class::C4Preview, Lane::L1), Band::Preview);
        assert!(Band::ALL.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn strict_priority_then_round_robin() {
        let mut s = Scheduler::new();
        let a = Some(LinkId(1));
        let b = Some(LinkId(2));
        assert!(s.enqueue(id(1), Class::C2Text, Lane::L1, a));
        assert!(s.enqueue(id(2), Class::C2Text, Lane::L1, a));
        assert!(s.enqueue(id(3), Class::C2Text, Lane::L1, b));
        assert!(s.enqueue(id(4), Class::C1Control, Lane::L3, b));
        assert!(!s.enqueue(id(4), Class::C4Preview, Lane::L3, a));
        assert_eq!(s.len(), 4);
        assert_eq!(s.pick(), Some(id(4)));
        assert_eq!(s.pick(), Some(id(1)));
        assert_eq!(s.pick(), Some(id(3)));
        assert_eq!(s.pick(), Some(id(2)));
        assert_eq!(s.pick(), None);
        assert!(s.is_empty());
    }

    #[test]
    fn remove_is_lazy_but_invisible() {
        let mut s = Scheduler::new();
        s.enqueue(id(1), Class::C2Text, Lane::L1, None);
        s.enqueue(id(2), Class::C2Text, Lane::L1, None);
        assert!(s.remove(&id(1)));
        assert!(!s.remove(&id(1)));
        assert_eq!(s.len(), 1);
        // Re-queued in another band after removal: served from the new one.
        assert!(s.enqueue(id(1), Class::C4Preview, Lane::L3, Some(LinkId(9))));
        assert_eq!(s.band_of(&id(1)), Some(Band::Preview));
        assert_eq!(s.pick(), Some(id(2)));
        assert_eq!(s.pick(), Some(id(1)));
        assert_eq!(s.pick(), None);
        s.enqueue(id(3), Class::C2Text, Lane::L1, None);
        s.remove(&id(3));
        s.compact();
        assert!(s.bands.iter().all(|b| b.queues.is_empty()));
    }

    /// A lazily removed entry re-queued under the same band and source must
    /// take its new place at the back, not its old place at the front
    /// (reviewer regression: the stale queue entry used to match the index
    /// again), and `compact` must keep exactly one live entry for it.
    #[test]
    fn requeue_after_remove_keeps_fifo_within_a_source() {
        let mut s = Scheduler::new();
        let a = Some(LinkId(1));
        assert!(s.enqueue(id(1), Class::C2Text, Lane::L1, a));
        assert!(s.enqueue(id(2), Class::C2Text, Lane::L1, a));
        assert!(s.remove(&id(1)));
        assert!(s.enqueue(id(1), Class::C2Text, Lane::L1, a));
        assert_eq!(s.pick(), Some(id(2)));
        assert_eq!(s.pick(), Some(id(1)));
        assert_eq!(s.pick(), None);
        // Same through remove_from_link.
        assert!(s.enqueue(id(1), Class::C2Text, Lane::L1, a));
        assert!(s.enqueue(id(2), Class::C2Text, Lane::L1, None));
        assert_eq!(s.remove_from_link(LinkId(1)), 1);
        assert!(s.enqueue(id(1), Class::C2Text, Lane::L1, a));
        assert_eq!(s.pick(), Some(id(2)));
        assert_eq!(s.pick(), Some(id(1)));
        assert_eq!(s.pick(), None);
        // compact() drops the stale copy, not the live one.
        assert!(s.enqueue(id(3), Class::C2Text, Lane::L1, a));
        assert!(s.remove(&id(3)));
        assert!(s.enqueue(id(3), Class::C2Text, Lane::L1, a));
        s.compact();
        let queue = &s.bands[Band::TextVouched.index()].queues[&a];
        assert_eq!(queue.len(), 1);
        assert_eq!(s.pick(), Some(id(3)));
        assert_eq!(s.pick(), None);
    }

    #[test]
    fn busy_hint_needs_both_a_queue_and_a_hold() {
        let mut s = Scheduler::new();
        assert!(!s.busy_hint());
        s.set_held(true);
        assert!(!s.busy_hint());
        s.enqueue(id(1), Class::C2Text, Lane::L3, Some(LinkId(1)));
        assert!(s.busy_hint());
        s.set_held(false);
        assert!(!s.busy_hint());
    }

    #[test]
    fn remove_from_link_drops_only_that_source() {
        let mut s = Scheduler::new();
        s.enqueue(id(1), Class::C2Text, Lane::L3, Some(LinkId(1)));
        s.enqueue(id(2), Class::C2Text, Lane::L3, Some(LinkId(2)));
        s.enqueue(id(3), Class::C2Text, Lane::L3, None);
        assert_eq!(s.remove_from_link(LinkId(1)), 1);
        assert_eq!(s.len_in(Band::TextOpaque), 2);
        assert_eq!(s.pick(), Some(id(3)));
        assert_eq!(s.pick(), Some(id(2)));
    }
}
