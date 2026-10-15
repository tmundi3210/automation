//! Per-link anti-entropy (docs/DESIGN.md §1 row `dtn/sync.rs`; §7 link
//! lifecycle "Hello → optional ContactProof → summary exchange → scheduled
//! push"; DECISIONS D4 MERGE "member-to-member set reconciliation runs on
//! its own budget"; D10 (4) "anti-entropy by summary-vector exchange;
//! priority: control ops > text > media").
//!
//! One [`LinkSync`] lives for one link (one encounter, one Noise session;
//! DESIGN §7). Its states:
//!
//! ```text
//! Idle ──start──▶ HelloSent ──peer Hello──▶ AwaitSummary ──first chunk──▶ Exchanging ──last chunk──▶ Pushing
//! ```
//!
//! * `start` sends our [`Hello`]; once the peer's Hello is in, our summary
//!   goes out (as chunks); once the peer's summary is complete and ours was
//!   sent, [`Out::Push`] says the push phase may begin and
//!   [`LinkSync::next_to_send`] yields cells the peer lacks.
//! * **What we offer** (D10 (4) priority; BRIEF_v2 §5.6 strict class
//!   order): the store's forwardable cells in class order, oldest first,
//!   minus what the peer's summary claims to hold, minus what the peer
//!   pushed to us this encounter, minus what we already pushed, minus
//!   rejected ids (§3 row (1,2,3,4,10) (iv)). Roster ops are C1 and so go
//!   first; member reconciliation needs nothing else here (D4 MERGE's
//!   separate budget is the scheduler's `budget_cells`).
//! * **What we summarise**: every id in the store plus the dedupe cache's
//!   rejected ids, so that peers stop re-offering what we refused (§3 row
//!   (1,2,3,4,10) (iv)).
//! * **Salt per encounter** (DESIGN §10 risk 4): the caller draws a random
//!   salt for `start`; a false positive under one salt does not repeat
//!   under the next, so a rarely-met peer is not starved for ever.
//! * **"Last sync" digest match** (DESIGN §1 `dtn/summary.rs`; §3 task
//!   `reach_set` "members synced in the last 10 min"):
//!   [`LinkSync::digest_matched`] is true when every id we *hold* is in
//!   the peer's vector and nothing the peer pushed was outside ours. The
//!   rejected ids we only remember are carried in our vector for the
//!   peer's sake but are not part of the match: otherwise two phones
//!   holding the same cells would never match after one of them refused a
//!   post the other never saw. Both inputs are judged once both summaries
//!   exist, whichever of `start` and the peer's chunks came first.
//! * **`never_offered`** (DESIGN §10 risk 4 "'never offered' counter in the
//!   sim"): the forwardable cells withheld because the peer's summary said
//!   it held them (true holdings and false positives alike, since this side
//!   cannot tell them apart); the simulator intersects
//!   [`LinkSync::withheld_ids`] with the peer's true holdings to measure
//!   false-positive suppression.
//!
//! Nothing here reads a clock: `now_mono` comes in from the caller
//! (BRIEF_v2 §4). Every decision is a deterministic function of the calls
//! made.

use std::collections::BTreeSet;

use crate::crypto::hash::Hash32;
use crate::dtn::dedupe::DedupeCache;
use crate::dtn::store::CellStore;
use crate::dtn::summary::{
    Assembled, ChunkAssembler, SUMMARY_TARGET_FPR_ASM, SummaryError, SummaryVector, sizing,
};
use crate::wire::frame::{Frame, Hello, SummaryChunk};

/// `Hello.proto` this implementation speaks (DESIGN §2.3).
pub const PROTO_VERSION: u8 = 1;
/// Default chunk payload: `15 + 496 = 511` encoded bytes, so a summary
/// frame is never larger than a cell frame on the stream (DESIGN §2.2 "Why
/// 512 B"; an assumption the spike retunes, DESIGN §10 risk 4 "8 kB filters
/// eat the wake").
pub const SUMMARY_CHUNK_BYTES_ASM: usize = 496;

/// State of one encounter (module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SyncState {
    Idle,
    HelloSent,
    AwaitSummary,
    Exchanging,
    Pushing,
}

/// What the link layer must send next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Out {
    SendHello(Hello),
    /// Our whole summary, one frame per chunk, in index order.
    SendSummary(Vec<SummaryChunk>),
    /// Both summaries are exchanged: start calling
    /// [`LinkSync::next_to_send`] on the link's schedule.
    Push,
}

/// Per-link parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SyncConfig {
    /// Advertised in `Hello.is_station` (BRIEF_v2 §5.7 keyless stations).
    pub is_station: bool,
    /// Target false-positive rate of our summary
    /// ([`SUMMARY_TARGET_FPR_ASM`]).
    pub target_fpr: f64,
    /// Largest chunk `bits` payload ([`SUMMARY_CHUNK_BYTES_ASM`]).
    pub max_chunk_bytes: usize,
}

impl Default for SyncConfig {
    fn default() -> Self {
        SyncConfig {
            is_station: false,
            target_fpr: SUMMARY_TARGET_FPR_ASM,
            max_chunk_bytes: SUMMARY_CHUNK_BYTES_ASM,
        }
    }
}

/// The anti-entropy state machine of one link. See the module docs.
#[derive(Debug)]
pub struct LinkSync {
    cfg: SyncConfig,
    state: SyncState,
    salt: u32,
    started_at_mono: Option<u64>,
    local: Option<SummaryVector>,
    local_chunks: Vec<SummaryChunk>,
    /// The ids the store held at `start` (ascending): what the digest
    /// match is judged over. The vector also carries the rejected ids.
    local_ids: Vec<Hash32>,
    summary_sent: bool,
    peer_hello: Option<Hello>,
    assembler: ChunkAssembler,
    peer: Option<SummaryVector>,
    ours_in_theirs: Option<bool>,
    received: BTreeSet<Hash32>,
    received_outside_local: u64,
    pushed: BTreeSet<Hash32>,
    withheld: BTreeSet<Hash32>,
    summary_errors: u64,
}

impl LinkSync {
    pub fn new(cfg: SyncConfig) -> Self {
        LinkSync {
            cfg,
            state: SyncState::Idle,
            salt: 0,
            started_at_mono: None,
            local: None,
            local_chunks: Vec::new(),
            local_ids: Vec::new(),
            summary_sent: false,
            peer_hello: None,
            assembler: ChunkAssembler::new(),
            peer: None,
            ours_in_theirs: None,
            received: BTreeSet::new(),
            received_outside_local: 0,
            pushed: BTreeSet::new(),
            withheld: BTreeSet::new(),
            summary_errors: 0,
        }
    }

    // ---- accessors ------------------------------------------------------

    pub fn config(&self) -> &SyncConfig {
        &self.cfg
    }

    pub fn state(&self) -> SyncState {
        self.state
    }

    /// The salt of our summary this encounter (0 before `start`).
    pub fn salt(&self) -> u32 {
        self.salt
    }

    pub fn started_at_mono(&self) -> Option<u64> {
        self.started_at_mono
    }

    pub fn peer_hello(&self) -> Option<&Hello> {
        self.peer_hello.as_ref()
    }

    /// Our summary, once `start` built it.
    pub fn local_summary(&self) -> Option<&SummaryVector> {
        self.local.as_ref()
    }

    /// The chunks `start` prepared (what `Out::SendSummary` carries).
    pub fn local_chunks(&self) -> &[SummaryChunk] {
        &self.local_chunks
    }

    /// The peer's summary, once complete.
    pub fn peer_summary(&self) -> Option<&SummaryVector> {
        self.peer.as_ref()
    }

    /// Ids pushed to the peer this encounter.
    pub fn pushed_len(&self) -> usize {
        self.pushed.len()
    }

    /// Ids the peer pushed to us this encounter.
    pub fn received_len(&self) -> usize {
        self.received.len()
    }

    /// Forwardable, unrejected cells this encounter did not offer because
    /// the peer's vector claims them: true holdings and false positives
    /// alike, since only the simulator knows which is which (module docs).
    pub fn never_offered(&self) -> u64 {
        u64::try_from(self.withheld.len()).unwrap_or(u64::MAX)
    }

    /// The ids behind [`never_offered`](Self::never_offered), ascending;
    /// the simulator intersects them with the peer's true holdings to count
    /// false-positive suppression.
    pub fn withheld_ids(&self) -> impl Iterator<Item = Hash32> + '_ {
        self.withheld.iter().copied()
    }

    /// Chunks refused (bad parameters, a second vector mid-stream, or an
    /// over-long one). A high count marks a misbehaving link.
    pub fn summary_errors(&self) -> u64 {
        self.summary_errors
    }

    /// `true` once both summaries are exchanged, every id we held at
    /// `start` is in the peer's vector and the peer pushed nothing outside
    /// our vector (module docs; rejected ids are not judged). Read it when
    /// the push phase has drained or at link down; it feeds the "last sync"
    /// status strip.
    pub fn digest_matched(&self) -> bool {
        self.state == SyncState::Pushing
            && self.ours_in_theirs == Some(true)
            && self.received_outside_local == 0
    }

    // ---- events ---------------------------------------------------------

    /// Opens the encounter at `now_mono` with a fresh `salt`: builds our
    /// summary from the store's ids and the dedupe cache's rejected ids,
    /// and sends our Hello (plus our summary if the peer's Hello already
    /// arrived). A second call does nothing.
    pub fn start(
        &mut self,
        salt: u32,
        now_mono: u64,
        store: &CellStore,
        dedupe: &DedupeCache,
    ) -> Vec<Out> {
        if self.state != SyncState::Idle {
            return Vec::new();
        }
        self.salt = salt;
        self.started_at_mono = Some(now_mono);
        let held: Vec<Hash32> = store.iter_ids().collect();
        let mut ids: Vec<Hash32> = held.iter().copied().chain(dedupe.rejected_ids()).collect();
        ids.sort_unstable();
        ids.dedup();
        let (n_bits, k) = sizing(ids.len(), self.cfg.target_fpr);
        let local = SummaryVector::build(ids.iter().copied(), salt, n_bits, k);
        self.local_chunks = local.to_chunks(self.cfg.max_chunk_bytes);
        self.local = Some(local);
        self.local_ids = held;
        self.judge_digest();
        self.state = SyncState::HelloSent;
        let mut out = vec![Out::SendHello(self.hello(store))];
        if self.peer_hello.is_some() {
            self.send_summary(&mut out);
        }
        out
    }

    fn hello(&self, store: &CellStore) -> Hello {
        Hello {
            proto: PROTO_VERSION,
            caps: 0,
            is_station: self.cfg.is_station,
            buffer_free: u32::try_from(store.free_bytes()).unwrap_or(u32::MAX),
        }
    }

    fn send_summary(&mut self, out: &mut Vec<Out>) {
        if self.summary_sent || self.state == SyncState::Idle {
            return;
        }
        out.push(Out::SendSummary(self.local_chunks.clone()));
        self.summary_sent = true;
        if self.state == SyncState::HelloSent {
            self.state = if self.assembler.is_empty() {
                SyncState::AwaitSummary
            } else {
                SyncState::Exchanging
            };
        }
        self.maybe_enter_pushing(out);
    }

    fn maybe_enter_pushing(&mut self, out: &mut Vec<Out>) {
        if self.state != SyncState::Pushing && self.summary_sent && self.peer.is_some() {
            self.state = SyncState::Pushing;
            out.push(Out::Push);
        }
    }

    /// Judges the digest-match inputs from what is known now: every id we
    /// held at `start` against the peer's vector, and every id the peer
    /// pushed against our vector. Called whenever either summary becomes known, so the
    /// verdict is the same whether `start` or the peer's chunks came first
    /// (before `start` there is nothing of ours to judge against).
    fn judge_digest(&mut self) {
        let Some(local) = self.local.as_ref() else {
            return;
        };
        self.received_outside_local = u64::try_from(
            self.received
                .iter()
                .filter(|id| !local.contains(id))
                .count(),
        )
        .unwrap_or(u64::MAX);
        self.ours_in_theirs = self
            .peer
            .as_ref()
            .map(|peer| self.local_ids.iter().all(|id| peer.contains(id)));
    }

    /// The peer's Hello arrived. If ours is out, our summary follows.
    pub fn on_hello(&mut self, hello: Hello) -> Vec<Out> {
        self.peer_hello = Some(hello);
        let mut out = Vec::new();
        self.send_summary(&mut out);
        out
    }

    /// One chunk of the peer's summary arrived. Chunks are collected in any
    /// order; a chunk of a different vector restarts the collection (the
    /// peer re-summarised); malformed chunks are counted and ignored. When
    /// the vector completes (and ours was sent) `Out::Push` is returned.
    /// Chunks after completion are ignored.
    pub fn on_summary_chunk(&mut self, chunk: SummaryChunk) -> Vec<Out> {
        let mut out = Vec::new();
        if self.peer.is_some() {
            return out;
        }
        let result = match self.assembler.push(chunk.clone()) {
            Err(SummaryError::Mismatch) => {
                self.summary_errors = self.summary_errors.saturating_add(1);
                self.assembler.reset();
                self.assembler.push(chunk)
            }
            other => other,
        };
        match result {
            Ok(Assembled::Incomplete { .. }) => {
                if self.state == SyncState::AwaitSummary {
                    self.state = SyncState::Exchanging;
                }
            }
            Ok(Assembled::Complete(peer)) => {
                self.peer = Some(peer);
                self.judge_digest();
                if self.state == SyncState::AwaitSummary {
                    self.state = SyncState::Exchanging;
                }
                self.maybe_enter_pushing(&mut out);
            }
            Err(_) => {
                self.summary_errors = self.summary_errors.saturating_add(1);
                self.assembler.reset();
            }
        }
        out
    }

    /// A control frame from the link: Hello and Summary are handled, cell
    /// frames go through the inbound pipeline (which calls
    /// [`on_cell_received`](Self::on_cell_received)), the rest is not ours.
    pub fn on_frame(&mut self, frame: &Frame) -> Vec<Out> {
        match frame {
            Frame::Hello(h) => self.on_hello(*h),
            Frame::Summary(s) => self.on_summary_chunk(s.clone()),
            Frame::Cell { .. } | Frame::Throttle(_) | Frame::ContactProof(_) => Vec::new(),
        }
    }

    /// The peer pushed the cell `msg_id` to us (whether or not we kept
    /// it). It is never offered back, and an id outside our summary means
    /// the peer held something we lacked ([`digest_matched`](Self::digest_matched));
    /// a cell received before `start` is judged when `start` builds ours.
    pub fn on_cell_received(&mut self, msg_id: Hash32) {
        if self.received.insert(msg_id)
            && let Some(local) = &self.local
            && !local.contains(&msg_id)
        {
            self.received_outside_local = self.received_outside_local.saturating_add(1);
        }
    }

    /// Up to `budget_cells` ids to push now (module docs), in class
    /// priority and age order; they are recorded as pushed. Empty before
    /// the push phase.
    pub fn next_to_send(
        &mut self,
        store: &CellStore,
        dedupe: &DedupeCache,
        budget_cells: usize,
    ) -> Vec<Hash32> {
        if self.state != SyncState::Pushing {
            return Vec::new();
        }
        let Some(peer) = self.peer.as_ref() else {
            return Vec::new();
        };
        let withheld = core::cell::RefCell::new(Vec::new());
        let ids = {
            let pushed = &self.pushed;
            let received = &self.received;
            let exclude = |id: &Hash32| -> bool {
                if pushed.contains(id) || received.contains(id) || dedupe.is_rejected(id) {
                    return true;
                }
                if peer.contains(id) {
                    withheld.borrow_mut().push(*id);
                    return true;
                }
                false
            };
            store.candidates_for(&exclude, budget_cells)
        };
        self.withheld.extend(withheld.into_inner());
        self.pushed.extend(ids.iter().copied());
        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtn::store::{CellMeta, Put, StoreCaps};
    use crate::relay::Lane;
    use crate::wire::cell::{BODY_LEN, Cell, TAG_LEN, VERSION, build};
    use crate::wire::frame::{Class, Throttle};

    fn cell(seed: u64) -> Cell {
        let mut body = [0u8; BODY_LEN];
        body[..8].copy_from_slice(&seed.to_le_bytes());
        build(VERSION, 7, &[0x33; TAG_LEN], &body)
    }

    fn meta(class: Class, t: u64) -> CellMeta {
        CellMeta {
            class,
            lane: Lane::L1,
            from_link: None,
            first_seen_mono: t,
            pinned: false,
        }
    }

    fn store_with(seeds: &[(u64, Class)]) -> CellStore {
        let mut s = CellStore::with_default_policy(StoreCaps::cells(100, 100));
        for (i, (seed, class)) in seeds.iter().enumerate() {
            assert_eq!(
                s.put(cell(*seed), meta(*class, u64::try_from(i).unwrap())),
                Put::Stored
            );
        }
        s
    }

    /// Feeds `outs` from one side into the other; returns what the other
    /// side answers.
    fn deliver(outs: Vec<Out>, to: &mut LinkSync) -> Vec<Out> {
        let mut back = Vec::new();
        for o in outs {
            match o {
                Out::SendHello(h) => back.extend(to.on_hello(h)),
                Out::SendSummary(chunks) => {
                    for c in chunks {
                        back.extend(to.on_summary_chunk(c));
                    }
                }
                Out::Push => {}
            }
        }
        back
    }

    #[test]
    fn state_machine_walks_idle_to_pushing_once() {
        let sa = store_with(&[(1, Class::C2Text)]);
        let sb = store_with(&[(2, Class::C2Text)]);
        let da = DedupeCache::new(1_000, 100);
        let db = DedupeCache::new(1_000, 100);
        let mut a = LinkSync::new(SyncConfig::default());
        let mut b = LinkSync::new(SyncConfig {
            is_station: true,
            ..SyncConfig::default()
        });
        assert_eq!(a.state(), SyncState::Idle);
        assert!(a.next_to_send(&sa, &da, 10).is_empty());
        assert!(!a.digest_matched());

        let a0 = a.start(11, 100, &sa, &da);
        assert_eq!(a.state(), SyncState::HelloSent);
        assert_eq!(a.salt(), 11);
        assert_eq!(a.started_at_mono(), Some(100));
        assert!(
            matches!(a0.as_slice(), [Out::SendHello(h)] if h.proto == PROTO_VERSION && !h.is_station)
        );
        assert!(a.start(12, 101, &sa, &da).is_empty(), "start is one-shot");
        assert_eq!(a.salt(), 11);

        // b hears a's Hello before starting: nothing to say yet
        let b0 = deliver(a0, &mut b);
        assert!(b0.is_empty());
        assert_eq!(b.state(), SyncState::Idle);
        assert!(b.peer_hello().is_some());

        // b starts: Hello + its summary at once
        let b1 = b.start(22, 100, &sb, &db);
        assert!(matches!(b1.as_slice(), [Out::SendHello(h), Out::SendSummary(_)] if h.is_station));
        assert_eq!(b.state(), SyncState::AwaitSummary);

        // a gets b's Hello (→ sends its summary) and b's summary (→ Push)
        let a1 = deliver(b1, &mut a);
        assert!(matches!(a1.as_slice(), [Out::SendSummary(_), Out::Push]));
        assert_eq!(a.state(), SyncState::Pushing);
        assert!(a.peer_summary().is_some());

        // b gets a's summary → Push
        let b2 = deliver(a1, &mut b);
        assert_eq!(b2, vec![Out::Push]);
        assert_eq!(b.state(), SyncState::Pushing);

        // each pushes exactly what the other lacks
        assert_eq!(a.next_to_send(&sa, &da, 10), vec![cell(1).msg_id()]);
        assert_eq!(b.next_to_send(&sb, &db, 10), vec![cell(2).msg_id()]);
        assert!(a.next_to_send(&sa, &da, 10).is_empty(), "not pushed twice");
        assert_eq!(a.pushed_len(), 1);
        assert!(!a.digest_matched());
        // extra chunks after completion are ignored
        let extra = b.local_chunks()[0].clone();
        assert!(a.on_summary_chunk(extra).is_empty());
        assert_eq!(a.summary_errors(), 0);
    }

    #[test]
    fn pushes_in_class_order_and_skips_received_rejected_and_held() {
        let sa = store_with(&[
            (10, Class::C4Preview),
            (11, Class::C1Control),
            (12, Class::C2Text),
            (13, Class::C2Text),
            (14, Class::C2Text),
        ]);
        let sb = store_with(&[(13, Class::C2Text)]);
        let mut da = DedupeCache::new(1_000, 100);
        da.mark_rejected(cell(14).msg_id(), 0);
        let db = DedupeCache::new(1_000, 100);
        let mut a = LinkSync::new(SyncConfig::default());
        let mut b = LinkSync::new(SyncConfig::default());
        let a0 = a.start(1, 0, &sa, &da);
        let b0 = b.start(2, 0, &sb, &db);
        let a1 = deliver(b0, &mut a);
        let b1 = deliver(a0, &mut b);
        deliver(a1, &mut b);
        deliver(b1, &mut a);
        assert_eq!(a.state(), SyncState::Pushing);
        assert_eq!(b.state(), SyncState::Pushing);
        let got = a.next_to_send(&sa, &da, 10);
        assert_eq!(
            got,
            vec![cell(11).msg_id(), cell(12).msg_id(), cell(10).msg_id()],
            "C1, then C2, then C4; 13 held by peer, 14 rejected"
        );
        assert_eq!(a.never_offered(), 1, "13 withheld on the summary");
        // the budget is honoured and continues where it left off; a cell
        // the peer pushed to us meanwhile is never offered back
        let mut c = LinkSync::new(SyncConfig::default());
        let mut d = LinkSync::new(SyncConfig::default());
        let c0 = c.start(3, 0, &sa, &da);
        let d0 = d.start(4, 0, &sb, &db);
        deliver(deliver(d0, &mut c), &mut d);
        deliver(deliver(c0, &mut d), &mut c);
        assert_eq!(c.state(), SyncState::Pushing);
        assert_eq!(
            c.next_to_send(&sa, &da, 2),
            vec![cell(11).msg_id(), cell(12).msg_id()]
        );
        c.on_cell_received(cell(10).msg_id());
        assert!(c.next_to_send(&sa, &da, 2).is_empty());
        assert_eq!(c.received_len(), 1);
        assert_eq!(
            c.never_offered(),
            1,
            "received ids are not withheld-by-summary"
        );
    }

    #[test]
    fn digest_matches_only_when_both_sides_hold_the_same_ids() {
        let sa = store_with(&[(1, Class::C2Text), (2, Class::C1Control)]);
        let sb = store_with(&[(2, Class::C1Control), (1, Class::C2Text)]);
        let d = DedupeCache::new(1_000, 100);
        let mut a = LinkSync::new(SyncConfig::default());
        let mut b = LinkSync::new(SyncConfig::default());
        let a0 = a.start(5, 0, &sa, &d);
        let b0 = b.start(6, 0, &sb, &d);
        let a1 = deliver(b0, &mut a);
        let b1 = deliver(a0, &mut b);
        deliver(a1, &mut b);
        deliver(b1, &mut a);
        assert!(a.digest_matched());
        assert!(b.digest_matched());
        assert!(a.next_to_send(&sa, &d, 10).is_empty());
        assert!(b.next_to_send(&sb, &d, 10).is_empty());
        // a cell pushed to us that we did not summarise breaks the match
        a.on_cell_received(cell(99).msg_id());
        assert!(!a.digest_matched());
        assert_eq!(a.received_len(), 1);
        a.on_cell_received(cell(99).msg_id());
        assert_eq!(a.received_len(), 1);

        // a side holding more than the other is not matched
        let sc = store_with(&[
            (1, Class::C2Text),
            (2, Class::C1Control),
            (3, Class::C2Text),
        ]);
        let mut c = LinkSync::new(SyncConfig::default());
        let mut e = LinkSync::new(SyncConfig::default());
        let c0 = c.start(7, 0, &sc, &d);
        let e0 = e.start(8, 0, &sb, &d);
        let c1 = deliver(e0, &mut c);
        let e1 = deliver(c0, &mut e);
        deliver(c1, &mut e);
        deliver(e1, &mut c);
        assert!(!c.digest_matched(), "c has 3, e's vector lacks it");
        assert!(
            e.digest_matched(),
            "e's ids are all in c's vector and c has pushed nothing yet"
        );
        assert_eq!(c.next_to_send(&sc, &d, 10), vec![cell(3).msg_id()]);
        e.on_cell_received(cell(3).msg_id());
        assert!(!e.digest_matched(), "c pushed 3, which e never summarised");
    }

    /// The peer's vector may complete before `start`; the digest is still
    /// judged against what we summarise, not against an empty id list.
    #[test]
    fn digest_is_judged_after_start_when_the_peer_summary_came_first() {
        let s = store_with(&[(1, Class::C2Text)]);
        let d = DedupeCache::new(1_000, 100);
        let hello = Hello {
            proto: 1,
            caps: 0,
            is_station: false,
            buffer_free: 0,
        };
        let peer = SummaryVector::build([cell(2).msg_id()], 9, 256, 3);
        assert!(!peer.contains(&cell(1).msg_id()));
        let mut a = LinkSync::new(SyncConfig::default());
        for c in peer.to_chunks(16) {
            assert!(a.on_summary_chunk(c).is_empty());
        }
        a.start(1, 0, &s, &d);
        assert!(matches!(
            a.on_hello(hello).as_slice(),
            [Out::SendSummary(_), Out::Push]
        ));
        assert!(
            !a.digest_matched(),
            "1 is ours and not in the peer's vector"
        );
        assert_eq!(a.next_to_send(&s, &d, 10), vec![cell(1).msg_id()]);
        // a cell pushed to us before start, never stored, is outside ours
        let same = SummaryVector::build([cell(1).msg_id()], 9, 256, 3);
        assert!(!same.contains(&cell(3).msg_id()));
        let mut b = LinkSync::new(SyncConfig::default());
        for c in same.to_chunks(16) {
            b.on_summary_chunk(c);
        }
        b.on_cell_received(cell(3).msg_id());
        b.start(2, 0, &s, &d);
        b.on_hello(hello);
        assert_eq!(b.state(), SyncState::Pushing);
        assert!(!b.digest_matched());
        assert_eq!(b.received_len(), 1);
    }

    #[test]
    fn malformed_and_foreign_chunks_are_counted_and_the_latest_vector_wins() {
        let s = store_with(&[(1, Class::C2Text)]);
        let d = DedupeCache::new(1_000, 100);
        let mut a = LinkSync::new(SyncConfig::default());
        let peer_hello = Hello {
            proto: 1,
            caps: 0,
            is_station: false,
            buffer_free: 0,
        };
        a.start(1, 0, &s, &d);
        assert!(matches!(
            a.on_hello(peer_hello).as_slice(),
            [Out::SendSummary(_)]
        ));
        assert_eq!(a.state(), SyncState::AwaitSummary);
        let bad = SummaryChunk {
            salt: 1,
            n_bits: 12,
            k: 1,
            chunk_idx: 0,
            chunk_total: 1,
            bits: vec![0],
        };
        assert!(a.on_summary_chunk(bad).is_empty());
        assert_eq!(a.summary_errors(), 1);
        assert_eq!(a.state(), SyncState::AwaitSummary);
        // first vector (salt 9) starts, then a second vector (salt 10)
        // replaces it; 384 bits in 16-byte chunks = 3 chunks each
        let v9 = SummaryVector::build([cell(1).msg_id()], 9, 384, 3).to_chunks(16);
        let v10 = SummaryVector::build([cell(1).msg_id()], 10, 384, 3).to_chunks(16);
        assert_eq!(v10.len(), 3);
        assert!(a.on_summary_chunk(v9[0].clone()).is_empty());
        assert_eq!(a.state(), SyncState::Exchanging);
        assert!(a.on_summary_chunk(v10[1].clone()).is_empty());
        assert_eq!(a.summary_errors(), 2);
        assert!(a.on_summary_chunk(v10[0].clone()).is_empty());
        assert_eq!(a.state(), SyncState::Exchanging);
        assert_eq!(
            a.on_summary_chunk(v10[0].clone()),
            Vec::<Out>::new(),
            "duplicate ignored"
        );
        assert_eq!(a.state(), SyncState::Exchanging);
        assert_eq!(a.on_summary_chunk(v10[2].clone()), vec![Out::Push]);
        assert_eq!(a.state(), SyncState::Pushing);
        assert_eq!(a.peer_summary().map(SummaryVector::salt), Some(10));
        assert!(a.digest_matched());
        // on_frame dispatches control frames and ignores the others
        let mut b = LinkSync::new(SyncConfig::default());
        b.start(2, 0, &s, &d);
        assert!(matches!(
            b.on_frame(&Frame::Hello(peer_hello)).as_slice(),
            [Out::SendSummary(_)]
        ));
        assert!(
            b.on_frame(&Frame::Throttle(Throttle::for_class(Class::C2Text, 5)))
                .is_empty()
        );
        assert!(b.on_frame(&Frame::Summary(v10[0].clone())).is_empty());
        assert_eq!(b.state(), SyncState::Exchanging);
        assert!(b.on_frame(&Frame::Summary(v10[1].clone())).is_empty());
        assert_eq!(b.on_frame(&Frame::Summary(v10[2].clone())), vec![Out::Push]);
    }

    #[test]
    fn chunks_before_our_summary_still_complete_the_exchange() {
        let s = store_with(&[(1, Class::C2Text)]);
        let d = DedupeCache::new(1_000, 100);
        let mut a = LinkSync::new(SyncConfig::default());
        let chunks = SummaryVector::build([cell(1).msg_id()], 9, 256, 3).to_chunks(16);
        // chunks arrive while we are still Idle: collected, no output
        for c in &chunks {
            assert!(a.on_summary_chunk(c.clone()).is_empty());
        }
        assert_eq!(a.state(), SyncState::Idle);
        assert!(a.peer_summary().is_some());
        // start: Hello only (no peer Hello yet)
        let out = a.start(1, 0, &s, &d);
        assert!(matches!(out.as_slice(), [Out::SendHello(_)]));
        assert_eq!(a.state(), SyncState::HelloSent);
        // peer Hello: our summary goes out and Push follows at once
        let out = a.on_hello(Hello {
            proto: 1,
            caps: 0,
            is_station: false,
            buffer_free: 1,
        });
        assert!(matches!(out.as_slice(), [Out::SendSummary(_), Out::Push]));
        assert_eq!(a.state(), SyncState::Pushing);
        assert!(a.digest_matched());
        assert!(a.next_to_send(&s, &d, 10).is_empty());
    }

    /// A rejected id we only remember is in our vector (so the peer stops
    /// offering it) but does not break the match with a peer that never
    /// saw it.
    #[test]
    fn a_remembered_rejection_is_summarised_but_not_judged() {
        let s = store_with(&[(1, Class::C2Text)]);
        let mut da = DedupeCache::new(1_000, 100);
        da.mark_rejected(cell(9).msg_id(), 0);
        let db = DedupeCache::new(1_000, 100);
        let mut a = LinkSync::new(SyncConfig::default());
        let mut b = LinkSync::new(SyncConfig::default());
        let a0 = a.start(5, 0, &s, &da);
        let b0 = b.start(6, 0, &s, &db);
        let a1 = deliver(b0, &mut a);
        let b1 = deliver(a0, &mut b);
        deliver(a1, &mut b);
        deliver(b1, &mut a);
        assert!(a.local_summary().unwrap().contains(&cell(9).msg_id()));
        assert!(!b.local_summary().unwrap().contains(&cell(9).msg_id()));
        assert!(a.digest_matched(), "9 is remembered, not held");
        assert!(b.digest_matched());
        assert!(a.next_to_send(&s, &da, 10).is_empty());
        assert!(b.next_to_send(&s, &db, 10).is_empty());
    }

    #[test]
    fn summary_covers_store_and_rejected_ids_with_the_given_salt() {
        let s = store_with(&[(1, Class::C2Text), (2, Class::C2Text)]);
        let mut d = DedupeCache::new(1_000, 100);
        d.mark_rejected(cell(3).msg_id(), 0);
        let mut a = LinkSync::new(SyncConfig {
            max_chunk_bytes: 16,
            ..SyncConfig::default()
        });
        a.start(0xDEAD_BEEF, 0, &s, &d);
        let local = a.local_summary().expect("built at start");
        assert_eq!(local.salt(), 0xDEAD_BEEF);
        assert!(local.contains(&cell(1).msg_id()));
        assert!(local.contains(&cell(2).msg_id()));
        assert!(
            local.contains(&cell(3).msg_id()),
            "rejected ids are summarised"
        );
        let (n_bits, k) = sizing(3, SUMMARY_TARGET_FPR_ASM);
        assert_eq!((local.n_bits(), local.k()), (n_bits, k));
        assert_eq!(
            SummaryVector::from_chunks(a.local_chunks()).as_ref(),
            Ok(local)
        );
        assert!(a.local_chunks().iter().all(|c| c.bits.len() <= 16));
        assert_eq!(a.config().max_chunk_bytes, 16);
        // the Hello advertises the store's free bytes
        let free = s.free_bytes();
        let mut b = LinkSync::new(SyncConfig::default());
        let out = b.start(1, 0, &s, &d);
        assert!(
            matches!(out.as_slice(), [Out::SendHello(h)] if h.buffer_free == u32::try_from(free).unwrap())
        );
    }
}
