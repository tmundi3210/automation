//! One shell node of the simulator (DESIGN §7 "link lifecycle in core":
//! `link_up` → Noise NN handshake (initiator = central) → `Hello` → summary
//! exchange (`dtn/sync`) → scheduled push; `Throttle` frames carry
//! backpressure). In this wave the node carries *opaque* cells: random
//! bodies, no roster and no end-to-end crypto, so that the engine, the
//! radio model and the link lifecycle can be exercised before the real node
//! drops in (DESIGN §10 "Simulator scenarios").
//!
//! What a node owns:
//!
//! * per link a [`PlatformLink`] (byte pipe), a [`NoiseLink`] (one NN
//!   session per link, DESIGN §2.3), a [`LinkSync`] (anti-entropy), an
//!   inbound [`LinkBudget`] (BRIEF_v2 §5.5 "rate limits per inbound link"),
//!   an outbound mirror budget that paces what we push, and a [`Scheduler`]
//!   (BRIEF_v2 §5.6 strict class order, fair across inbound links);
//! * a [`CellStore`] with the [`BufferPolicy`] caps (DECISIONS D4 BUFFER), a
//!   [`DedupeCache`] (BRIEF_v2 §5.5 "dedupe cache of message-id hashes") and
//!   a [`SlotPolicy`] (D4 LINK SLOTS);
//! * the battery model (BRIEF_v2 §5.7: below the floor the phone stops
//!   forwarding others' cells but still sends its own) and the iPhone
//!   foreground/locked state (DECISIONS D10 Q1–Q4; docs/testing.md);
//! * a [`SimClock`]: a `ManualClock` the engine advances, with every
//!   `wall_for_display` call counted so the `zero_wall_clock_reads`
//!   criterion can be asserted (DESIGN §10 S4 (e)).
//!
//! Lanes (BRIEF_v2 §5.5): the cell body is random, so "validation" is
//! simulated out of band: a cell counts as [`Validation::Validated`] at a
//! node that is in its audience (the group's members, or the two ends of a
//! DM pair), which is what the real node would find when the tag matches a
//! chain it holds. Everything else is opaque and takes L2 on a link to a
//! QR-verified contact (the sim sets `LinkAuth::VerifiedContact` for pairs
//! instead of running the ContactProof exchange) or L3 otherwise.
//!
//! The `flood` adversary (docs/scenarios.md S-flood) is a node with
//! [`SimNode::set_flooding`]: it pushes its junk without the outbound
//! mirror budget and ignores `Throttle` holds, so what reaches the honest
//! nodes is bounded only by their inbound budgets (BRIEF_v2 §5.5).
//!
//! Nothing here reads the OS clock: `now_ms` arrives from the engine.

use std::cell::Cell as StdCell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use meshcore::clock::{Clock, ManualClock};
use meshcore::codec::{Encode, decode};
use meshcore::crypto::ed25519::{IdentityKey, IdentitySigner};
use meshcore::crypto::hash::Hash32;
use meshcore::dtn::dedupe::{DEDUPE_MAX_ENTRIES_ASM, DedupeCache};
use meshcore::dtn::store::{CellMeta, CellStore, Put};
use meshcore::dtn::sync::{LinkSync, Out, SyncConfig, SyncState};
use meshcore::relay::Lane;
use meshcore::relay::buffers::BufferPolicy;
use meshcore::relay::limits::{Admit, LinkBudget};
use meshcore::relay::sched::Scheduler;
use meshcore::relay::slots::SlotPolicy;
use meshcore::transport::link::{
    Link, LinkAuth, LinkError, LinkId, LinkInfo, PeerKind, Role, Validation,
};
use meshcore::transport::platform_link::PlatformLink;
use meshcore::wire::cell::{self, BODY_LEN, CELL_LEN, Cell, TAG_LEN};
use meshcore::wire::frame::{Class, Frame, LinkFrame, Throttle};
use meshcore::wire::noise::NoiseLink;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::scenario::NodeKind;

/// Ids pulled from `LinkSync::next_to_send` per call: the store scan it
/// runs is linear in the store (one Bloom check per id), so pulls are
/// batched rather than one id at a time (assumption; engine performance
/// only). A call that returns fewer ids than this reached the end of the
/// store, so no further pull is needed this encounter.
pub const PULL_BATCH: usize = 256;
/// Powered-on time between dedupe sweeps (DESIGN §6 "all expiries use the
/// powered-on meter"; the sweep itself is cheap).
pub const DEDUPE_SWEEP_MS: u64 = 60_000;
/// Length of one iPhone foreground/locked window (simulation assumption:
/// "windows of 5 min drawn from the rng").
pub const DUTY_WINDOW_MS: u64 = 5 * 60_000;
/// Battery drain per powered-on hour, in parts per million of a full
/// battery (simulation assumption: 1 %/h idle).
pub const DRAIN_PPM_PER_H: u64 = 10_000;
/// Extra drain per hour while relaying (simulation assumption: +2 %/h).
pub const RELAY_DRAIN_PPM_PER_H: u64 = 20_000;
/// iPhone GATT link slots (DECISIONS D4 Q4/Q9: "iOS unpublished"; assumption).
pub const IPHONE_SLOTS: u32 = 4;
/// Class of every opaque cell this wave generates (posts, DMs and junk are
/// all text-sized, BRIEF_v2 §5.6 C2).
pub const OPAQUE_CLASS: Class = Class::C2Text;

const PPM: u64 = 1_000_000;
const MS_PER_H: u64 = 3_600_000;

/// Why a link went down (DESIGN §7 `SimLink` row; reported per reason).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DropReason {
    /// A Noise frame was lost on the air (DESIGN §7: a lost chunk tears the
    /// link down and a new link re-handshakes).
    FrameLoss,
    /// A locked iPhone's wake ended (DECISIONS D10).
    WakeEnd,
    /// The partition schedule separated the two nodes.
    Partition,
    /// The Noise session failed (handshake rejected, decrypt failed).
    Noise,
    /// The byte stream was corrupt or a queue overflowed.
    Framing,
    /// An authenticated frame did not decode (a peer bug in this wave).
    Malformed,
}

impl DropReason {
    pub fn name(self) -> &'static str {
        match self {
            DropReason::FrameLoss => "frame_loss",
            DropReason::WakeEnd => "wake_end",
            DropReason::Partition => "partition",
            DropReason::Noise => "noise",
            DropReason::Framing => "framing",
            DropReason::Malformed => "malformed",
        }
    }
}

// ---- clock ------------------------------------------------------------------------

/// The engine-driven clock of one node: a [`ManualClock`] whose
/// `wall_for_display` calls are counted, so the run can assert that the
/// protocol never read wall time (DESIGN §10 S4 (e): "the sim's protocol
/// clock is a counter and `wall_for_display` is instrumented").
#[derive(Debug, Default)]
pub struct SimClock {
    inner: ManualClock,
    wall_reads: StdCell<u64>,
}

impl SimClock {
    pub fn new(wall_offset_ms: i64) -> Self {
        let inner = ManualClock::new(0);
        inner.set_wall_offset_ms(wall_offset_ms);
        SimClock {
            inner,
            wall_reads: StdCell::new(0),
        }
    }

    /// The engine sets powered-on time before every call into the node.
    pub fn set(&self, mono_ms: u64) {
        self.inner.set(mono_ms);
    }

    /// Calls of [`Clock::wall_for_display`] made through the trait.
    pub fn wall_reads(&self) -> u64 {
        self.wall_reads.get()
    }

    /// The display hook: the report reads wall time here, and this path is
    /// not counted (it is the one legitimate reader).
    pub fn display_hook_wall(&self) -> i64 {
        self.inner.wall_for_display()
    }
}

impl Clock for SimClock {
    fn mono_ms(&self) -> u64 {
        self.inner.mono_ms()
    }

    fn wall_for_display(&self) -> i64 {
        self.wall_reads.set(self.wall_reads.get().saturating_add(1));
        self.inner.wall_for_display()
    }
}

// ---- cell registry ------------------------------------------------------------------

/// What kind of traffic a cell is (docs/scenarios.md `[traffic]` and the
/// `flood` adversary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CellKind {
    /// A group post.
    Post,
    /// A DM between a verified pair.
    Dm,
    /// Junk from a flooding node: counted, never expected to be delivered.
    Junk,
}

/// Who could validate a cell (module docs "Lanes").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Audience {
    Group(usize),
    Pair(usize, usize),
    Nobody,
}

/// The sim's omniscient record of one created cell: creator, creation
/// time, fragment at creation and the first-seen time at every node.
#[derive(Debug, Clone)]
pub struct CellRecord {
    pub id: Hash32,
    pub creator: usize,
    pub created_ms: u64,
    pub fragment: usize,
    pub kind: CellKind,
    pub audience: Audience,
    /// Powered-on time each node first held the cell; `u64::MAX` = never.
    pub first_seen: Vec<u64>,
}

/// Every cell created in the run, by index and by `msg_id`.
#[derive(Debug, Default)]
pub struct CellRegistry {
    pub records: Vec<CellRecord>,
    by_id: BTreeMap<Hash32, usize>,
    group_members: Vec<BTreeSet<usize>>,
    n_nodes: usize,
    /// Times a node recorded the same `msg_id` as newly seen twice
    /// (`no_duplicate_ui_delivery`).
    pub duplicate_first_seen: u64,
}

impl CellRegistry {
    pub fn new(n_nodes: usize, group_members: Vec<BTreeSet<usize>>) -> Self {
        CellRegistry {
            records: Vec::new(),
            by_id: BTreeMap::new(),
            group_members,
            n_nodes,
            duplicate_first_seen: 0,
        }
    }

    fn register(
        &mut self,
        id: Hash32,
        creator: usize,
        created_ms: u64,
        fragment: usize,
        kind: CellKind,
        audience: Audience,
    ) -> usize {
        let idx = self.records.len();
        self.records.push(CellRecord {
            id,
            creator,
            created_ms,
            fragment,
            kind,
            audience,
            first_seen: vec![u64::MAX; self.n_nodes],
        });
        self.by_id.insert(id, idx);
        idx
    }

    pub fn index_of(&self, id: &Hash32) -> Option<usize> {
        self.by_id.get(id).copied()
    }

    pub fn creator_of(&self, id: &Hash32) -> Option<usize> {
        self.index_of(id).map(|i| self.records[i].creator)
    }

    /// Whether `node` could validate the cell (module docs "Lanes").
    pub fn in_audience(&self, node: usize, id: &Hash32) -> bool {
        let Some(i) = self.index_of(id) else {
            return false;
        };
        match self.records[i].audience {
            Audience::Group(g) => self.group_members.get(g).is_some_and(|m| m.contains(&node)),
            Audience::Pair(a, b) => node == a || node == b,
            Audience::Nobody => false,
        }
    }

    /// Records that `node` holds the cell from `now` on. A second record
    /// for the same node is the duplicate-delivery violation the checker
    /// reports.
    pub fn record_first_seen(&mut self, id: &Hash32, node: usize, now: u64) {
        let Some(i) = self.index_of(id) else {
            return;
        };
        if let Some(slot) = self.records[i].first_seen.get_mut(node) {
            if *slot == u64::MAX {
                *slot = now;
            } else {
                self.duplicate_first_seen = self.duplicate_first_seen.saturating_add(1);
            }
        }
    }
}

// ---- per-node statistics ---------------------------------------------------------------

/// Counters one node keeps; summed into the report.
#[derive(Debug, Default, Clone, Copy)]
pub struct NodeStats {
    pub handshakes: u64,
    pub frames_sent: u64,
    pub cells_sent: u64,
    pub cells_received: u64,
    pub duplicates: u64,
    pub malformed: u64,
    pub throttled_in: u64,
    pub throttled_out: u64,
    pub rejected_in: u64,
    pub rejected_out: u64,
    pub no_room: u64,
    pub throttles_received: u64,
    pub withheld_low_battery: u64,
    pub digest_matches: u64,
    pub summary_errors: u64,
}

impl NodeStats {
    pub fn add(&mut self, o: &NodeStats) {
        self.handshakes += o.handshakes;
        self.frames_sent += o.frames_sent;
        self.cells_sent += o.cells_sent;
        self.cells_received += o.cells_received;
        self.duplicates += o.duplicates;
        self.malformed += o.malformed;
        self.throttled_in += o.throttled_in;
        self.throttled_out += o.throttled_out;
        self.rejected_in += o.rejected_in;
        self.rejected_out += o.rejected_out;
        self.no_room += o.no_room;
        self.throttles_received += o.throttles_received;
        self.withheld_low_battery += o.withheld_low_battery;
        self.digest_matches += o.digest_matches;
        self.summary_errors += o.summary_errors;
    }
}

// ---- link end ------------------------------------------------------------------------

/// One node's side of one link.
#[derive(Debug)]
struct LinkEnd {
    id: u32,
    role: Role,
    platform: PlatformLink,
    /// `None` only transiently while the handshake state is converted.
    noise: Option<NoiseLink>,
    sync: LinkSync,
    inbound_budget: LinkBudget,
    /// Mirror of the peer's inbound budget, created when the push phase
    /// starts; paces what we push so that the peer rarely has to throttle.
    outbound_budget: Option<LinkBudget>,
    sched: Scheduler,
    /// The id picked from the scheduler but not yet sent (budget or queue
    /// held it).
    head: Option<Hash32>,
    /// Encrypted frames the platform queue did not take yet (`Busy`).
    pending: VecDeque<Vec<u8>>,
    /// Powered-on time until which a `Throttle` holds this link.
    hold_until: u64,
    /// `true` once `next_to_send` returned nothing: new cells then go
    /// through the scheduler directly.
    pull_exhausted: bool,
    /// A cell was stored since the encounter started and before the pull
    /// finished, so the store differs from what the summary described.
    dirty: bool,
    /// Frames handed to the platform since the engine last rolled losses.
    frames_unrolled: u32,
}

impl LinkEnd {
    fn has_work(&self) -> bool {
        self.head.is_some()
            || !self.pending.is_empty()
            || !self.sched.is_empty()
            || (!self.pull_exhausted && self.sync.state() == SyncState::Pushing)
    }

    fn is_pushing(&self) -> bool {
        self.sync.state() == SyncState::Pushing
    }

    /// Hands pending encrypted frames to the platform until it says `Busy`.
    fn flush_pending(&mut self, stats: &mut NodeStats) -> Result<(), DropReason> {
        while let Some(frame) = self.pending.front() {
            match self.platform.send(frame) {
                Ok(()) => {
                    self.pending.pop_front();
                    self.frames_unrolled = self.frames_unrolled.saturating_add(1);
                    stats.frames_sent = stats.frames_sent.saturating_add(1);
                }
                Err(LinkError::Busy) => return Ok(()),
                Err(LinkError::Closed | LinkError::TooLarge(_)) => {
                    return Err(DropReason::Framing);
                }
            }
        }
        Ok(())
    }

    /// Encrypts one link frame and queues it (DESIGN §7 layering: frame →
    /// `encrypt_frame` → `send`).
    fn stage(&mut self, plain: &[u8], stats: &mut NodeStats) -> Result<(), DropReason> {
        let noise = self.noise.as_mut().ok_or(DropReason::Noise)?;
        let ct = noise.encrypt_frame(plain).map_err(|_| DropReason::Noise)?;
        self.pending.push_back(ct);
        self.flush_pending(stats)
    }

    fn apply_outs(&mut self, outs: Vec<Out>, stats: &mut NodeStats) -> Result<(), DropReason> {
        for out in outs {
            match out {
                Out::SendHello(h) => {
                    let bytes = LinkFrame::hello(&h)
                        .try_encode()
                        .map_err(|_| DropReason::Malformed)?;
                    self.stage(&bytes, stats)?;
                }
                Out::SendSummary(chunks) => {
                    for c in &chunks {
                        let bytes = LinkFrame::summary(c)
                            .and_then(|f| f.try_encode())
                            .map_err(|_| DropReason::Malformed)?;
                        self.stage(&bytes, stats)?;
                    }
                }
                Out::Push => {}
            }
        }
        Ok(())
    }

    /// The push loop (module docs): drain pending frames, then pick ids
    /// from the scheduler (fed by `next_to_send` until exhausted, then by
    /// live forwarding), admit each against the outbound mirror budget and
    /// send its TTL-decremented copy (DESIGN §2.2 `Cell::forwarded`).
    ///
    /// `flooding`: the node is the `flood` adversary and skips the mirror
    /// budget and any `Throttle` hold (docs/scenarios.md S-flood "injecting
    /// 600 junk cells/min": containment is the receiver's inbound budget,
    /// BRIEF_v2 §5.5, never the sender's good manners).
    #[allow(clippy::too_many_arguments)]
    fn service(
        &mut self,
        store: &CellStore,
        dedupe: &DedupeCache,
        reg: &CellRegistry,
        me: usize,
        relaying: bool,
        flooding: bool,
        now: u64,
        stats: &mut NodeStats,
    ) -> Result<(), DropReason> {
        self.flush_pending(stats)?;
        if !self.is_pushing() {
            return Ok(());
        }
        loop {
            if !self.pending.is_empty() {
                return Ok(());
            }
            if now < self.hold_until && !flooding {
                self.sched.set_held(true);
                return Ok(());
            }
            let Some(id) = self.head else {
                if let Some(id) = self.sched.pick() {
                    self.head = Some(id);
                    continue;
                }
                if self.pull_exhausted {
                    self.sched.set_held(false);
                    return Ok(());
                }
                if self.sync.digest_matched() && !self.dirty {
                    // Every id we hold is in the peer's vector and nothing
                    // arrived since: a pull would withhold all of them
                    // (DESIGN §10 risk 4 `never_offered`), so the scan is
                    // skipped and live forwarding takes over.
                    self.pull_exhausted = true;
                    continue;
                }
                let ids = self.sync.next_to_send(store, dedupe, PULL_BATCH);
                if ids.len() < PULL_BATCH {
                    // The scan reached the end of the store: nothing more to
                    // pull this encounter.
                    self.pull_exhausted = true;
                }
                if ids.is_empty() {
                    continue;
                }
                for id in ids {
                    let Some((_, meta)) = store.get(&id) else {
                        continue;
                    };
                    if relaying || reg.creator_of(&id) == Some(me) {
                        self.sched
                            .enqueue(id, meta.class, meta.lane, meta.from_link);
                    } else {
                        stats.withheld_low_battery = stats.withheld_low_battery.saturating_add(1);
                    }
                }
                continue;
            };
            let Some((cell, meta)) = store.get(&id) else {
                self.head = None;
                continue;
            };
            let peer_is_station = self.sync.peer_hello().is_some_and(|h| h.is_station);
            let budget = self
                .outbound_budget
                .get_or_insert_with(|| LinkBudget::new(now, peer_is_station));
            let verdict = if flooding {
                Admit::Accept
            } else {
                budget.admit(meta.class, meta.lane, CELL_LEN, now)
            };
            match verdict {
                Admit::Accept => {}
                Admit::Throttle { retry_ms } => {
                    self.hold_until = now.saturating_add(u64::from(retry_ms));
                    self.sched.set_held(true);
                    stats.throttled_out = stats.throttled_out.saturating_add(1);
                    return Ok(());
                }
                Admit::Reject => {
                    self.head = None;
                    stats.rejected_out = stats.rejected_out.saturating_add(1);
                    continue;
                }
            }
            let Some(fwd) = cell.forwarded() else {
                self.head = None;
                continue;
            };
            let bytes = LinkFrame::cell(meta.class, &fwd)
                .try_encode()
                .map_err(|_| DropReason::Malformed)?;
            self.stage(&bytes, stats)?;
            self.head = None;
            stats.cells_sent = stats.cells_sent.saturating_add(1);
        }
    }
}

// ---- node ----------------------------------------------------------------------------

/// Static configuration of one node, from the scenario and the seed.
#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub idx: usize,
    pub kind: NodeKind,
    /// GATT link slots.
    pub slots: u32,
    pub battery_floor_ppm: u64,
    pub battery_start_ppm: u64,
    /// iPhone foreground duty in parts per million.
    pub foreground_duty_ppm: u64,
    pub wall_offset_ms: i64,
    pub dedupe_retention_ms: u64,
    /// QR-verified contacts (node indices).
    pub contacts: BTreeSet<usize>,
}

/// The outcome of one node tick the engine acts on.
#[derive(Debug, Default)]
pub struct TickOutcome {
    /// The iPhone just left the locked state (the engine starts its dial
    /// attempts).
    pub became_foreground: bool,
    /// Links that were serviced and may have queued bytes for the radio.
    pub serviced: Vec<u32>,
    /// Links whose service failed; the engine tears them down.
    pub dead_links: Vec<(u32, DropReason)>,
}

/// One simulated phone or station. See the module docs.
#[derive(Debug)]
pub struct SimNode {
    cfg: NodeConfig,
    pub clock: SimClock,
    rng: ChaCha20Rng,
    identity: IdentityKey,
    store: CellStore,
    dedupe: DedupeCache,
    slots: SlotPolicy,
    links: BTreeMap<u32, LinkEnd>,
    foreground: bool,
    next_duty_window_ms: u64,
    next_dedupe_sweep_ms: u64,
    powered_ms: u64,
    relay_ms: u64,
    last_tick_ms: u64,
    /// The `flood` adversary: pushes without its own pacing (module docs).
    flooding: bool,
    pub stats: NodeStats,
}

impl SimNode {
    /// A node seeded from the engine's RNG. `foreground` for an iPhone is
    /// drawn for the first duty window here.
    pub fn new(cfg: NodeConfig, seed: u64) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let mut id_seed = [0u8; 32];
        rng.fill_bytes(&mut id_seed);
        let identity = IdentitySigner::from_seed(&id_seed).public();
        let policy = BufferPolicy::default();
        let store = CellStore::new(policy.caps(), Box::new(policy));
        let dedupe = DedupeCache::new(cfg.dedupe_retention_ms, DEDUPE_MAX_ENTRIES_ASM);
        let slots = SlotPolicy::new(cfg.slots);
        let clock = SimClock::new(cfg.wall_offset_ms);
        let mut node = SimNode {
            cfg,
            clock,
            rng,
            identity,
            store,
            dedupe,
            slots,
            links: BTreeMap::new(),
            foreground: true,
            next_duty_window_ms: DUTY_WINDOW_MS,
            next_dedupe_sweep_ms: DEDUPE_SWEEP_MS,
            powered_ms: 0,
            relay_ms: 0,
            last_tick_ms: 0,
            flooding: false,
            stats: NodeStats::default(),
        };
        node.draw_duty();
        node
    }

    fn draw_duty(&mut self) {
        if self.cfg.kind == NodeKind::IPhone {
            let roll: u64 = self.rng.random_range(0..PPM);
            self.foreground = roll < self.cfg.foreground_duty_ppm;
        } else {
            self.foreground = true;
        }
    }

    /// Replaces the store with an empty one holding at most `max_cells`
    /// (tests of the no-room path).
    #[cfg(test)]
    fn shrink_store(&mut self, max_cells: usize) {
        use meshcore::dtn::store::StoreCaps;
        self.store = CellStore::new(
            StoreCaps::cells(max_cells, max_cells),
            Box::new(BufferPolicy::default()),
        );
    }

    /// Turns this node into the `flood` adversary (docs/scenarios.md
    /// `behaviour = "flood"`): it keeps pushing through its own outbound
    /// pacing and through the peer's `Throttle` frames; containing it is the
    /// receiver's job (BRIEF_v2 §5.5 "rate limits per inbound link").
    pub fn set_flooding(&mut self, flooding: bool) {
        self.flooding = flooding;
    }

    // ---- accessors -----------------------------------------------------------

    pub fn idx(&self) -> usize {
        self.cfg.idx
    }

    pub fn kind(&self) -> NodeKind {
        self.cfg.kind
    }

    pub fn identity(&self) -> IdentityKey {
        self.identity
    }

    pub fn is_station(&self) -> bool {
        self.cfg.kind == NodeKind::Station
    }

    /// A locked iPhone never initiates and never links to another locked
    /// iPhone (DECISIONS D10 Q1–Q4).
    pub fn is_locked_iphone(&self) -> bool {
        self.cfg.kind == NodeKind::IPhone && !self.foreground
    }

    pub fn is_foreground(&self) -> bool {
        self.foreground
    }

    /// Whether this node would dial right now (a slot is free and it is not
    /// a locked iPhone).
    pub fn can_dial(&self) -> bool {
        self.slots.can_dial() && !self.is_locked_iphone()
    }

    /// Whether this node takes an inbound connect (D4 LINK SLOTS: the
    /// policy keeps half the slots for links it chooses itself).
    pub fn accepting_inbound(&self) -> bool {
        self.slots.accepting_inbound()
    }

    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    pub fn is_contact(&self, peer: usize) -> bool {
        self.cfg.contacts.contains(&peer)
    }

    pub fn holds(&self, id: &Hash32) -> bool {
        self.store.contains(id)
    }

    pub fn store_len(&self) -> usize {
        self.store.len()
    }

    pub fn wall_reads(&self) -> u64 {
        self.clock.wall_reads()
    }

    /// Battery level in parts per million (stations are mains powered).
    pub fn battery_ppm(&self) -> u64 {
        if self.is_station() {
            return PPM;
        }
        let start = u128::from(self.cfg.battery_start_ppm) * u128::from(MS_PER_H);
        let used = u128::from(DRAIN_PPM_PER_H) * u128::from(self.powered_ms)
            + u128::from(RELAY_DRAIN_PPM_PER_H) * u128::from(self.relay_ms);
        let left = start.saturating_sub(used) / u128::from(MS_PER_H);
        u64::try_from(left).unwrap_or(0)
    }

    /// BRIEF_v2 §5.7: below the floor the phone stops forwarding others'
    /// cells (it still sends its own and still receives).
    pub fn is_relaying(&self) -> bool {
        self.is_station() || self.battery_ppm() >= self.cfg.battery_floor_ppm
    }

    // ---- links ----------------------------------------------------------------

    /// A link came up. The central runs the Noise initiator and writes the
    /// first handshake message at once (DESIGN §7 "initiator = central").
    pub fn link_up(
        &mut self,
        id: u32,
        role: Role,
        chunk: usize,
        hint: PeerKind,
        peer_contact: Option<IdentityKey>,
        now: u64,
    ) -> Result<(), DropReason> {
        self.clock.set(now);
        let mtu_hint = u16::try_from(chunk.saturating_add(3)).unwrap_or(u16::MAX);
        let info = LinkInfo {
            role,
            peer_kind_hint: hint,
            mtu_hint,
        };
        let mut platform = PlatformLink::new(LinkId(id), info);
        if let Some(key) = peer_contact {
            platform.set_auth(LinkAuth::VerifiedContact(key));
        }
        let mut noise = match role {
            Role::Central => NoiseLink::initiator(&mut self.rng),
            Role::Peripheral => NoiseLink::responder(&mut self.rng),
        }
        .map_err(|_| DropReason::Noise)?;
        let mut frames_unrolled = 0u32;
        if role == Role::Central {
            let msg = noise.write_handshake().map_err(|_| DropReason::Noise)?;
            platform.send(&msg).map_err(|_| DropReason::Framing)?;
            frames_unrolled = 1;
            self.stats.frames_sent = self.stats.frames_sent.saturating_add(1);
        }
        let end = LinkEnd {
            id,
            role,
            platform,
            noise: Some(noise),
            sync: LinkSync::new(SyncConfig {
                is_station: self.is_station(),
                ..SyncConfig::default()
            }),
            inbound_budget: LinkBudget::new(now, self.is_station()),
            outbound_budget: None,
            sched: Scheduler::new(),
            head: None,
            pending: VecDeque::new(),
            hold_until: 0,
            pull_exhausted: false,
            dirty: false,
            frames_unrolled,
        };
        self.slots.on_link_up(role);
        self.links.insert(id, end);
        Ok(())
    }

    /// The link is gone: its queues go with it (the DTN store keeps the
    /// bytes; the next encounter re-summarises).
    pub fn link_down(&mut self, id: u32, now: u64) {
        self.clock.set(now);
        if let Some(end) = self.links.remove(&id) {
            self.slots.on_link_down(end.role);
            if end.sync.digest_matched() {
                self.stats.digest_matches = self.stats.digest_matches.saturating_add(1);
            }
            self.stats.summary_errors = self
                .stats
                .summary_errors
                .saturating_add(end.sync.summary_errors());
        }
    }

    /// Bytes the radio wants to carry from this side of `id`, at most
    /// `max_bytes`.
    pub fn poll_outbound(&mut self, id: u32, max_bytes: usize) -> Vec<u8> {
        self.links
            .get_mut(&id)
            .map(|l| l.platform.poll_outbound(max_bytes))
            .unwrap_or_default()
    }

    /// Frames handed to the platform since the last call (the radio rolls
    /// one loss per frame).
    pub fn take_unrolled_frames(&mut self, id: u32) -> u32 {
        self.links
            .get_mut(&id)
            .map(|l| std::mem::take(&mut l.frames_unrolled))
            .unwrap_or(0)
    }

    /// Inbound radio bytes, fed to the platform link in `chunk`-sized
    /// pieces (the ATT MTU−3 the platform would use) and processed.
    pub fn on_bytes(
        &mut self,
        id: u32,
        bytes: &[u8],
        chunk: usize,
        now: u64,
        reg: &mut CellRegistry,
    ) -> Result<(), DropReason> {
        self.clock.set(now);
        let Some(end) = self.links.get_mut(&id) else {
            return Ok(());
        };
        for piece in bytes.chunks(chunk.max(1)) {
            match end.platform.feed(piece) {
                Ok(()) => {}
                Err(LinkError::Busy) => return Err(DropReason::Framing),
                Err(_) => return Err(DropReason::Framing),
            }
        }
        self.with_link(id, now, reg, |node, end, now, reg| {
            node.process_inbound(end, now, reg)
        })
    }

    /// Continues pushing on `id` (the radio drained some of its queue).
    pub fn service_link(
        &mut self,
        id: u32,
        now: u64,
        reg: &mut CellRegistry,
    ) -> Result<(), DropReason> {
        self.clock.set(now);
        self.with_link(id, now, reg, |node, end, now, reg| {
            let relaying = node.is_relaying();
            end.service(
                &node.store,
                &node.dedupe,
                reg,
                node.cfg.idx,
                relaying,
                node.flooding,
                now,
                &mut node.stats,
            )
        })
    }

    /// Runs `f` with the link taken out of the map, so that the handler can
    /// touch every other link (live forwarding never sends back on the
    /// source link).
    fn with_link<F>(
        &mut self,
        id: u32,
        now: u64,
        reg: &mut CellRegistry,
        f: F,
    ) -> Result<(), DropReason>
    where
        F: FnOnce(&mut SimNode, &mut LinkEnd, u64, &mut CellRegistry) -> Result<(), DropReason>,
    {
        let Some(mut end) = self.links.remove(&id) else {
            return Ok(());
        };
        let r = f(self, &mut end, now, reg);
        self.links.insert(id, end);
        r
    }

    fn process_inbound(
        &mut self,
        end: &mut LinkEnd,
        now: u64,
        reg: &mut CellRegistry,
    ) -> Result<(), DropReason> {
        while let Some(msg) = end.platform.recv() {
            let noise = end.noise.as_mut().ok_or(DropReason::Noise)?;
            if !noise.is_transport() {
                noise.read_handshake(&msg).map_err(|_| DropReason::Noise)?;
                if noise.is_my_turn() {
                    let reply = noise.write_handshake().map_err(|_| DropReason::Noise)?;
                    end.platform.send(&reply).map_err(|_| DropReason::Framing)?;
                    end.frames_unrolled = end.frames_unrolled.saturating_add(1);
                    self.stats.frames_sent = self.stats.frames_sent.saturating_add(1);
                }
                if noise.is_handshake_finished() {
                    let hs = end.noise.take().ok_or(DropReason::Noise)?;
                    end.noise = Some(hs.into_transport().map_err(|_| DropReason::Noise)?);
                    self.stats.handshakes = self.stats.handshakes.saturating_add(1);
                    let salt = self.rng.next_u32();
                    let outs = end.sync.start(salt, now, &self.store, &self.dedupe);
                    end.apply_outs(outs, &mut self.stats)?;
                }
                continue;
            }
            let plain = noise.decrypt_frame(&msg).map_err(|_| DropReason::Noise)?;
            let frame: Frame = decode(&plain).map_err(|_| DropReason::Malformed)?;
            match frame {
                Frame::Hello(h) => {
                    let outs = end.sync.on_hello(h);
                    end.apply_outs(outs, &mut self.stats)?;
                }
                Frame::Summary(s) => {
                    let outs = end.sync.on_summary_chunk(s);
                    end.apply_outs(outs, &mut self.stats)?;
                }
                Frame::Cell { class, cell } => self.on_cell(end, class, cell, now, reg)?,
                Frame::Throttle(t) => {
                    end.hold_until = now.saturating_add(u64::from(t.retry_ms));
                    end.sched.set_held(true);
                    self.stats.throttles_received = self.stats.throttles_received.saturating_add(1);
                }
                Frame::ContactProof(_) => {}
            }
        }
        let relaying = self.is_relaying();
        end.service(
            &self.store,
            &self.dedupe,
            reg,
            self.cfg.idx,
            relaying,
            self.flooding,
            now,
            &mut self.stats,
        )
    }

    /// The inbound pipeline of one cell (DESIGN §1 `node/inbound.rs`:
    /// "stream → frame → cell → dedupe → store"; BRIEF_v2 §5.5 "checks
    /// format, size and TTL", per-link rate limits, dedupe).
    fn on_cell(
        &mut self,
        end: &mut LinkEnd,
        class: Class,
        cell: Cell,
        now: u64,
        reg: &mut CellRegistry,
    ) -> Result<(), DropReason> {
        self.stats.cells_received = self.stats.cells_received.saturating_add(1);
        if cell.validate().is_err() {
            self.stats.malformed = self.stats.malformed.saturating_add(1);
            return Ok(());
        }
        let id = cell.msg_id();
        end.sync.on_cell_received(id);
        if self.dedupe.expires_at(&id).is_some_and(|exp| exp > now) {
            self.stats.duplicates = self.stats.duplicates.saturating_add(1);
            return Ok(());
        }
        let validated = if reg.in_audience(self.cfg.idx, &id) {
            Validation::Validated
        } else {
            Validation::Opaque
        };
        let lane = end.platform.lane_for(validated);
        match end.inbound_budget.admit(class, lane, CELL_LEN, now) {
            Admit::Accept => {}
            Admit::Throttle { retry_ms } => {
                self.stats.throttled_in = self.stats.throttled_in.saturating_add(1);
                let bytes = LinkFrame::throttle(&Throttle::for_class(class, retry_ms))
                    .try_encode()
                    .map_err(|_| DropReason::Malformed)?;
                end.stage(&bytes, &mut self.stats)?;
                return Ok(());
            }
            Admit::Reject => {
                self.stats.rejected_in = self.stats.rejected_in.saturating_add(1);
                return Ok(());
            }
        }
        let meta = CellMeta {
            class,
            lane,
            from_link: Some(LinkId(end.id)),
            first_seen_mono: now,
            pinned: false,
        };
        // The dedupe entry is written only for a cell the node actually
        // holds (DESIGN §1 "dedupe → store" is one step): a cell refused for
        // lack of room was never delivered, and remembering it as seen would
        // drop every later offer as a duplicate for the whole retention
        // window, long after room came back.
        match self.store.put(cell, meta) {
            Put::Stored => {
                self.dedupe.seen_or_insert(id, now);
                reg.record_first_seen(&id, self.cfg.idx, now);
                if self.is_relaying() {
                    self.forward_live(id, class, lane, Some(LinkId(end.id)));
                }
            }
            Put::Duplicate => {
                self.dedupe.seen_or_insert(id, now);
                self.stats.duplicates = self.stats.duplicates.saturating_add(1);
            }
            Put::NoRoom => self.stats.no_room = self.stats.no_room.saturating_add(1),
        }
        Ok(())
    }

    /// Queues a newly held cell on every pushing link whose initial pull is
    /// over (links still pulling will find it in the store). The source
    /// link is never in the map while its cell is handled.
    fn forward_live(&mut self, id: Hash32, class: Class, lane: Lane, from: Option<LinkId>) {
        for end in self.links.values_mut() {
            if end.is_pushing() && end.pull_exhausted {
                end.sched.enqueue(id, class, lane, from);
            } else {
                end.dirty = true;
            }
        }
    }

    // ---- traffic ---------------------------------------------------------------

    /// Creates one opaque cell of `kind` (random tag and body, TTL drawn
    /// per DESIGN §2.2) and holds it as this node's own (lane L1, no inbound
    /// link). Own cells are forwarded even below the battery floor
    /// (BRIEF_v2 §5.7 "still sends its own").
    pub fn create_cell(
        &mut self,
        kind: CellKind,
        audience: Audience,
        fragment: usize,
        now: u64,
        reg: &mut CellRegistry,
    ) -> Hash32 {
        self.clock.set(now);
        let ttl = cell::draw_initial_ttl(&mut self.rng);
        let mut tag = [0u8; TAG_LEN];
        self.rng.fill_bytes(&mut tag);
        let mut body = [0u8; BODY_LEN];
        self.rng.fill_bytes(&mut body);
        let c = cell::build(cell::VERSION, ttl, &tag, &body);
        let id = c.msg_id();
        self.dedupe.seen_or_insert(id, now);
        let meta = CellMeta {
            class: OPAQUE_CLASS,
            lane: Lane::L1,
            from_link: None,
            first_seen_mono: now,
            pinned: false,
        };
        reg.register(id, self.cfg.idx, now, fragment, kind, audience);
        if self.store.put(c, meta) == Put::Stored {
            reg.record_first_seen(&id, self.cfg.idx, now);
            self.forward_live(id, OPAQUE_CLASS, Lane::L1, None);
        }
        id
    }

    // ---- tick ------------------------------------------------------------------

    /// One node tick (scenario `tick_ms`): battery meter, dedupe sweep,
    /// iPhone duty window, and a service pass over links with pending work
    /// (throttle retries).
    pub fn tick(&mut self, now: u64, reg: &mut CellRegistry) -> TickOutcome {
        self.clock.set(now);
        let mut outcome = TickOutcome::default();
        let dt = now.saturating_sub(self.last_tick_ms);
        self.last_tick_ms = now;
        let relaying_now = !self.links.is_empty() && self.is_relaying() && !self.is_station();
        self.powered_ms = self.powered_ms.saturating_add(dt);
        if relaying_now {
            self.relay_ms = self.relay_ms.saturating_add(dt);
        }
        if now >= self.next_dedupe_sweep_ms {
            self.dedupe.expire(now);
            self.next_dedupe_sweep_ms = now.saturating_add(DEDUPE_SWEEP_MS);
        }
        if self.cfg.kind == NodeKind::IPhone && now >= self.next_duty_window_ms {
            let was = self.foreground;
            self.draw_duty();
            self.next_duty_window_ms = now.saturating_add(DUTY_WINDOW_MS);
            outcome.became_foreground = self.foreground && !was;
        }
        self.service_links(now, reg, &mut outcome);
        outcome
    }

    /// Services every pushing link that has work and is not held
    /// (throttle retries, live forwards queued since the last pass).
    pub fn service_links(&mut self, now: u64, reg: &CellRegistry, outcome: &mut TickOutcome) {
        self.clock.set(now);
        let relaying = self.is_relaying();
        let flooding = self.flooding;
        for end in self.links.values_mut() {
            if end.is_pushing() && (flooding || now >= end.hold_until) && end.has_work() {
                match end.service(
                    &self.store,
                    &self.dedupe,
                    reg,
                    self.cfg.idx,
                    relaying,
                    flooding,
                    now,
                    &mut self.stats,
                ) {
                    Ok(()) => outcome.serviced.push(end.id),
                    Err(reason) => outcome.dead_links.push((end.id, reason)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(idx: usize, kind: NodeKind) -> NodeConfig {
        NodeConfig {
            idx,
            kind,
            slots: 7,
            battery_floor_ppm: 300_000,
            battery_start_ppm: 1_000_000,
            foreground_duty_ppm: 1_000_000,
            wall_offset_ms: 0,
            dedupe_retention_ms: 10 * 24 * 3_600_000,
            contacts: BTreeSet::new(),
        }
    }

    fn registry() -> CellRegistry {
        CellRegistry::new(2, vec![BTreeSet::from([0, 1])])
    }

    /// Carries bytes both ways until nothing moves, like the radio does.
    fn pump(a: &mut SimNode, b: &mut SimNode, id: u32, now: u64, reg: &mut CellRegistry) {
        for _ in 0..200 {
            let ab = a.poll_outbound(id, usize::MAX);
            let ba = b.poll_outbound(id, usize::MAX);
            if ab.is_empty() && ba.is_empty() {
                return;
            }
            if !ab.is_empty() {
                b.on_bytes(id, &ab, 185, now, reg).unwrap();
            }
            if !ba.is_empty() {
                a.on_bytes(id, &ba, 512, now, reg).unwrap();
            }
            a.service_link(id, now, reg).unwrap();
            b.service_link(id, now, reg).unwrap();
        }
        panic!("the link never settled");
    }

    #[test]
    fn clock_counts_trait_reads_but_not_the_display_hook() {
        let c = SimClock::new(-5);
        c.set(100);
        assert_eq!(c.mono_ms(), 100);
        assert_eq!(c.display_hook_wall(), 95);
        assert_eq!(c.wall_reads(), 0);
        assert_eq!(c.wall_for_display(), 95);
        assert_eq!(c.wall_reads(), 1);
    }

    #[test]
    fn handshake_hello_summary_and_push_deliver_cells_both_ways() {
        let mut reg = registry();
        let mut a = SimNode::new(cfg(0, NodeKind::Android), 1);
        let mut b = SimNode::new(cfg(1, NodeKind::Android), 2);
        let ida = a.create_cell(CellKind::Post, Audience::Group(0), 0, 10, &mut reg);
        let idb = b.create_cell(CellKind::Post, Audience::Group(0), 0, 10, &mut reg);
        a.link_up(7, Role::Central, 512, PeerKind::AppUuid, None, 100)
            .unwrap();
        b.link_up(7, Role::Peripheral, 512, PeerKind::AppUuid, None, 100)
            .unwrap();
        pump(&mut a, &mut b, 7, 200, &mut reg);
        assert_eq!(a.stats.handshakes, 1);
        assert_eq!(b.stats.handshakes, 1);
        assert!(a.holds(&idb), "a received b's cell");
        assert!(b.holds(&ida), "b received a's cell");
        assert_eq!(reg.records[0].first_seen, vec![10, 200]);
        assert_eq!(reg.duplicate_first_seen, 0);
        // a live cell created after the sync goes through the scheduler
        let live = a.create_cell(CellKind::Post, Audience::Group(0), 0, 300, &mut reg);
        a.service_link(7, 300, &mut reg).unwrap();
        pump(&mut a, &mut b, 7, 300, &mut reg);
        assert!(b.holds(&live));
        assert_eq!(reg.records[2].first_seen[1], 300);
        // the TTL was decremented on the way (DESIGN §2.2 `Cell::forwarded`)
        let ttl_at_a = a.store.get(&live).unwrap().0.ttl();
        let ttl_at_b = b.store.get(&live).unwrap().0.ttl();
        assert_eq!(ttl_at_b, ttl_at_a - 1, "one hop, one TTL decrement");
        a.link_down(7, 400);
        b.link_down(7, 400);
        assert_eq!(a.link_count(), 0);
        assert_eq!(
            a.stats.digest_matches + b.stats.digest_matches,
            0,
            "stores differed at start"
        );
        // a second encounter with equal stores: both sides see a digest match
        b.link_up(8, Role::Central, 185, PeerKind::AppUuid, None, 500)
            .unwrap();
        a.link_up(8, Role::Peripheral, 185, PeerKind::AppUuid, None, 500)
            .unwrap();
        pump(&mut b, &mut a, 8, 600, &mut reg);
        a.link_down(8, 700);
        b.link_down(8, 700);
        assert_eq!(a.stats.digest_matches + b.stats.digest_matches, 2);
        assert_eq!(reg.duplicate_first_seen, 0);
    }

    #[test]
    fn corrupt_bytes_tear_the_link_down() {
        let mut reg = registry();
        let mut a = SimNode::new(cfg(0, NodeKind::Android), 1);
        let mut b = SimNode::new(cfg(1, NodeKind::Android), 2);
        a.link_up(1, Role::Central, 512, PeerKind::AppUuid, None, 0)
            .unwrap();
        b.link_up(1, Role::Peripheral, 512, PeerKind::AppUuid, None, 0)
            .unwrap();
        let mut msg1 = a.poll_outbound(1, usize::MAX);
        assert_eq!(msg1.len(), 2 + 32);
        msg1[5] ^= 0xFF;
        let r = b.on_bytes(1, &msg1, 512, 0, &mut reg);
        // a flipped ephemeral byte is still a valid handshake message; the
        // initiator then fails on the responder's reply tag.
        if r.is_ok() {
            let msg2 = b.poll_outbound(1, usize::MAX);
            assert_eq!(
                a.on_bytes(1, &msg2, 512, 0, &mut reg),
                Err(DropReason::Noise)
            );
        }
        // a short frame on the stream closes the platform link
        let mut c = SimNode::new(cfg(0, NodeKind::Android), 3);
        c.link_up(2, Role::Peripheral, 512, PeerKind::AppUuid, None, 0)
            .unwrap();
        assert_eq!(
            c.on_bytes(2, &[1, 0, 9], 512, 0, &mut reg),
            Err(DropReason::Framing)
        );
    }

    #[test]
    fn battery_drains_and_the_floor_stops_relaying_but_not_own_cells() {
        let mut reg = registry();
        let mut c = cfg(0, NodeKind::Android);
        c.battery_start_ppm = 400_000;
        let mut n = SimNode::new(c, 9);
        assert!(n.is_relaying());
        // 1 %/h idle: after 11 h the level is 29 % < 30 % floor
        n.tick(11 * MS_PER_H, &mut reg);
        assert_eq!(n.battery_ppm(), 290_000);
        assert!(!n.is_relaying());
        let own = n.create_cell(
            CellKind::Post,
            Audience::Group(0),
            0,
            11 * MS_PER_H,
            &mut reg,
        );
        assert!(n.holds(&own));
        let mut s = SimNode::new(cfg(1, NodeKind::Station), 10);
        s.tick(100 * MS_PER_H, &mut reg);
        assert_eq!(s.battery_ppm(), PPM, "stations are mains powered");
    }

    #[test]
    fn locked_iphone_never_dials_and_duty_windows_redraw() {
        let mut reg = registry();
        let mut c = cfg(0, NodeKind::IPhone);
        c.foreground_duty_ppm = 0;
        c.slots = IPHONE_SLOTS;
        let mut n = SimNode::new(c, 4);
        assert!(n.is_locked_iphone());
        assert!(!n.can_dial());
        assert!(n.accepting_inbound());
        let mut c = cfg(0, NodeKind::IPhone);
        c.foreground_duty_ppm = PPM;
        let mut m = SimNode::new(c, 4);
        assert!(m.is_foreground() && m.can_dial());
        let o = n.tick(DUTY_WINDOW_MS, &mut reg);
        assert!(!o.became_foreground);
        let o = m.tick(DUTY_WINDOW_MS, &mut reg);
        assert!(!o.became_foreground, "already foreground: no transition");
    }

    #[test]
    fn verified_contacts_get_l2_links() {
        let mut reg = registry();
        let mut ca = cfg(0, NodeKind::Android);
        ca.contacts.insert(1);
        let a = SimNode::new(ca, 1);
        let b = SimNode::new(cfg(1, NodeKind::Android), 2);
        let mut a = a;
        a.link_up(
            3,
            Role::Central,
            512,
            PeerKind::AppUuid,
            Some(b.identity()),
            0,
        )
        .unwrap();
        let end = a.links.get(&3).unwrap();
        assert_eq!(end.platform.auth(), LinkAuth::VerifiedContact(b.identity()));
        assert_eq!(end.platform.lane_for(Validation::Opaque), Lane::L2);
        assert!(a.is_contact(1));
        let _ = &mut reg;
    }
    #[test]
    fn below_the_floor_a_node_withholds_others_cells_but_pushes_its_own() {
        let mut reg = CellRegistry::new(3, vec![BTreeSet::from([0, 1, 2])]);
        let mut c = cfg(0, NodeKind::Android);
        c.battery_start_ppm = 400_000;
        let mut a = SimNode::new(c, 1);
        let mut b = SimNode::new(cfg(1, NodeKind::Android), 2);
        let mut d = SimNode::new(cfg(2, NodeKind::Android), 3);
        // b hands a its cell while a is still relaying
        let theirs = b.create_cell(CellKind::Post, Audience::Group(0), 0, 10, &mut reg);
        b.link_up(1, Role::Central, 512, PeerKind::AppUuid, None, 100)
            .unwrap();
        a.link_up(1, Role::Peripheral, 512, PeerKind::AppUuid, None, 100)
            .unwrap();
        pump(&mut b, &mut a, 1, 200, &mut reg);
        assert!(a.holds(&theirs));
        a.link_down(1, 300);
        b.link_down(1, 300);
        // 11 h later a is at 29 % < 30 %: BRIEF_v2 §5.7 "stops relaying
        // for others" but "keeps own traffic"
        let t = 11 * MS_PER_H;
        a.tick(t, &mut reg);
        assert!(!a.is_relaying());
        let own = a.create_cell(CellKind::Post, Audience::Group(0), 0, t, &mut reg);
        a.link_up(2, Role::Central, 512, PeerKind::AppUuid, None, t)
            .unwrap();
        d.link_up(2, Role::Peripheral, 512, PeerKind::AppUuid, None, t)
            .unwrap();
        pump(&mut a, &mut d, 2, t + 100, &mut reg);
        assert!(d.holds(&own), "own cells still go out below the floor");
        assert!(
            !d.holds(&theirs),
            "others' cells are withheld below the floor"
        );
        assert!(a.stats.withheld_low_battery >= 1);
    }

    #[test]
    fn a_cell_refused_for_lack_of_room_is_not_remembered_as_seen() {
        let mut reg = registry();
        let mut a = SimNode::new(cfg(0, NodeKind::Android), 1);
        let mut b = SimNode::new(cfg(1, NodeKind::Android), 2);
        a.shrink_store(1);
        let first = b.create_cell(CellKind::Post, Audience::Group(0), 0, 10, &mut reg);
        b.link_up(1, Role::Central, 512, PeerKind::AppUuid, None, 100)
            .unwrap();
        a.link_up(1, Role::Peripheral, 512, PeerKind::AppUuid, None, 100)
            .unwrap();
        pump(&mut b, &mut a, 1, 200, &mut reg);
        assert!(a.holds(&first));
        // pinned, so the policy cannot evict it for the next cell
        assert!(a.store.set_pinned(&first, true));
        let second = b.create_cell(CellKind::Post, Audience::Group(0), 0, 300, &mut reg);
        b.service_link(1, 300, &mut reg).unwrap();
        pump(&mut b, &mut a, 1, 300, &mut reg);
        assert!(!a.holds(&second), "no room for a second cell");
        assert_eq!(a.stats.no_room, 1);
        // DESIGN §1 inbound pipeline "dedupe → store": a cell the store
        // refused was never delivered, so the dedupe cache must not claim it
        // was seen, or a later re-offer (when room exists) is dropped as a
        // duplicate for the whole retention window.
        assert!(
            !a.dedupe.contains(&second),
            "a refused cell must stay acceptable on a later offer"
        );
        assert_eq!(reg.records[1].first_seen[0], u64::MAX);
    }

    #[test]
    fn a_flooding_node_ignores_its_outbound_budget_and_throttles() {
        // docs/scenarios.md S-flood: the adversary "injects 600 junk
        // cells/min"; it is the honest receiver's inbound budget (BRIEF_v2
        // §5.5) that must contain it, not the sender's own pacing.
        let mut reg = CellRegistry::new(2, vec![BTreeSet::from([0])]);
        let mut honest = SimNode::new(cfg(0, NodeKind::Android), 1);
        let mut flooder = SimNode::new(cfg(1, NodeKind::Android), 2);
        flooder.set_flooding(true);
        flooder
            .link_up(1, Role::Central, 512, PeerKind::AppUuid, None, 0)
            .unwrap();
        honest
            .link_up(1, Role::Peripheral, 512, PeerKind::AppUuid, None, 0)
            .unwrap();
        pump(&mut flooder, &mut honest, 1, 10, &mut reg);
        let n = 200u64;
        for i in 0..n {
            let now = 100 + i * 100;
            flooder.create_cell(CellKind::Junk, Audience::Nobody, 0, now, &mut reg);
            flooder.service_link(1, now, &mut reg).unwrap();
            pump(&mut flooder, &mut honest, 1, now, &mut reg);
        }
        assert!(
            flooder.stats.cells_sent >= n - 5,
            "the flooder sent {} of {n} junk cells: its own mirror budget paced it",
            flooder.stats.cells_sent
        );
        assert_eq!(flooder.stats.throttled_out, 0);
        assert!(honest.stats.throttles_received == 0);
        assert!(
            honest.stats.throttled_in > n / 2,
            "the honest side throttled only {} of {n} L3 junk cells",
            honest.stats.throttled_in
        );
        assert!(honest.store_len() < usize::try_from(n / 2).unwrap());
    }
}
