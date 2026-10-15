//! One simulated phone or station: a real [`meshcore::node::Node`] on an
//! in-memory database, plus the parts of the device the core does not
//! model (DESIGN §7 `SimLink` row, §10 "Simulator scenarios"; docs/testing.md
//! "Assumptions baked into the simulator"):
//!
//! * the node itself: identity, roster, pairwise sessions, DTN store, link
//!   lifecycle, lanes and budgets all live in `meshcore::node` (DESIGN §1
//!   `node/*`); the engine drives it with `link_up`/`feed`/`poll_outbound`/
//!   `link_down`/`tick` and the commands, nothing else;
//! * the GATT slot policy used to decide dialling (`relay::slots`, D4 LINK
//!   SLOTS), mirrored here because the node accepts whatever link the
//!   platform brings up;
//! * the battery model (BRIEF_v2 §5.7: below the floor the phone stops
//!   forwarding others' cells but still sends its own; fed to the node with
//!   `set_power`) and the iPhone foreground/locked state (DECISIONS D10
//!   Q1–Q4; docs/testing.md);
//! * a [`SimClock`]: the display hook handed to the node with
//!   `set_display_clock`. The node's powered-on meter runs on the `now_mono`
//!   the engine passes in; the wall clock is reachable only through this
//!   hook and every read of it is counted (`Node::wall_reads` plus the
//!   trait reads counted here), so the `zero_wall_clock_reads` criterion
//!   can be asserted (DESIGN §10 S4 (e)).
//!
//! The [`ObjRegistry`] is the simulator's omniscient record of every text
//! object created in the run (group posts and DMs by `ObjId`, the same id
//! on sender and receivers): who was expected to see it and when each node
//! first *showed* it (`Event::Post` / `Event::Dm` after the decrypting
//! transaction committed). "Delivered" in every criterion means shown at
//! the node, never store presence. The [`Trace`] folds every node event
//! into what the invariant checker needs (`assert.rs`).
//!
//! Nothing here reads the OS clock: `now_ms` arrives from the engine.

use std::cell::Cell as StdCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use meshcore::clock::Clock;
use meshcore::crypto::backend::GroupId;
use meshcore::crypto::ed25519::IdentityKey;
use meshcore::crypto::hash::Hash32;
use meshcore::node::node::NodeStats as CoreStats;
use meshcore::node::{
    DeliveryState, Event, Node, NodeConfig as CoreConfig, NodeError, ObjId, VoteId,
};
use meshcore::objects::task_op::TaskId;
use meshcore::relay::slots::SlotPolicy;
use meshcore::storage::Db;
use meshcore::transport::link::{LinkAuth, LinkId, Role};
use meshcore::votes::tally::VoteState;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::scenario::NodeKind;

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
    /// The node declared the link dead (`NodeError::LinkDead`: Noise
    /// failure, corrupt stream or a malformed authenticated frame).
    Dead,
    /// The node refused to bring the link up.
    Refused,
}

impl DropReason {
    pub fn name(self) -> &'static str {
        match self {
            DropReason::FrameLoss => "frame_loss",
            DropReason::WakeEnd => "wake_end",
            DropReason::Partition => "partition",
            DropReason::Dead => "link_dead",
            DropReason::Refused => "refused",
        }
    }
}

// ---- clock ------------------------------------------------------------------------

#[derive(Debug, Default)]
struct ClockState {
    mono: StdCell<u64>,
    wall_offset_ms: StdCell<i64>,
    reads: StdCell<u64>,
}

/// The display clock of one node (module docs): the engine moves it, the
/// node reaches it only through `set_display_clock`, and every
/// `wall_for_display` call made through the trait is counted. The handle
/// is shared (`Rc`), so the engine keeps one and the node owns another.
#[derive(Debug, Clone, Default)]
pub struct SimClock {
    state: Rc<ClockState>,
}

impl SimClock {
    pub fn new(wall_offset_ms: i64) -> Self {
        let state = ClockState::default();
        state.wall_offset_ms.set(wall_offset_ms);
        SimClock {
            state: Rc::new(state),
        }
    }

    /// The engine sets powered-on time before every call into the node.
    pub fn set(&self, mono_ms: u64) {
        self.state.mono.set(mono_ms);
    }

    /// Calls of [`Clock::wall_for_display`] made through the trait.
    pub fn wall_reads(&self) -> u64 {
        self.state.reads.get()
    }

    /// The display hook the report reads; not counted (the one legitimate
    /// reader).
    pub fn display_hook_wall(&self) -> i64 {
        i64::try_from(self.state.mono.get())
            .unwrap_or(i64::MAX)
            .saturating_add(self.state.wall_offset_ms.get())
    }
}

impl Clock for SimClock {
    fn mono_ms(&self) -> u64 {
        self.state.mono.get()
    }

    fn wall_for_display(&self) -> i64 {
        self.state
            .reads
            .set(self.state.reads.get().saturating_add(1));
        self.display_hook_wall()
    }
}

// ---- object registry --------------------------------------------------------------

/// What a text object is (docs/scenarios.md `[traffic]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjKind {
    /// A text post to scenario group `g`.
    Post { group: usize },
    /// A 1:1 text from the creator to `to`.
    Dm { to: usize },
}

/// The sim's omniscient record of one created object.
#[derive(Debug, Clone)]
pub struct ObjRecord {
    pub obj: ObjId,
    pub creator: usize,
    pub created_ms: u64,
    /// Index into the resolved partition schedule in force at creation.
    pub partition_idx: usize,
    /// The creator's fragment at creation.
    pub fragment: usize,
    pub kind: ObjKind,
    /// Nodes expected to show the object (the creator shows its own at
    /// creation and is not listed).
    pub expected: BTreeSet<usize>,
    /// Powered-on time each node first showed the object; `u64::MAX` = never.
    pub shown: Vec<u64>,
    /// Non-empty label lists per node at the time it was shown.
    pub labels: BTreeMap<usize, Vec<String>>,
    /// A DM the `withhold_dms` adversary held back and released at
    /// `created_ms` (docs/scenarios.md S3).
    pub withheld: bool,
    /// A setup hello sent during the in-person setup phase before the run
    /// (engine docs "Provisioning"); shown everywhere at t=0 and excluded
    /// from every delivery criterion.
    pub setup: bool,
}

/// What recording a shown object found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shown {
    First,
    Duplicate,
    Unknown,
}

/// Every text object created in the run, by index and by `ObjId`.
#[derive(Debug, Default)]
pub struct ObjRegistry {
    pub records: Vec<ObjRecord>,
    by_id: BTreeMap<ObjId, usize>,
    n_nodes: usize,
    /// Times a node showed the same object twice (`no_duplicate_ui_delivery`).
    pub duplicate_shown: u64,
    /// Objects shown that the sim never created (would be a forgery).
    pub unknown_shown: u64,
}

impl ObjRegistry {
    pub fn new(n_nodes: usize) -> Self {
        ObjRegistry {
            n_nodes,
            ..Self::default()
        }
    }

    /// Records a new object; the creator shows it at once.
    #[allow(clippy::too_many_arguments)]
    pub fn register(
        &mut self,
        obj: ObjId,
        creator: usize,
        created_ms: u64,
        partition_idx: usize,
        fragment: usize,
        kind: ObjKind,
        expected: BTreeSet<usize>,
        withheld: bool,
        setup: bool,
    ) -> usize {
        let idx = self.records.len();
        let mut shown = vec![u64::MAX; self.n_nodes];
        if let Some(s) = shown.get_mut(creator) {
            *s = created_ms;
        }
        self.records.push(ObjRecord {
            obj,
            creator,
            created_ms,
            partition_idx,
            fragment,
            kind,
            expected,
            shown,
            labels: BTreeMap::new(),
            withheld,
            setup,
        });
        self.by_id.insert(obj, idx);
        idx
    }

    pub fn index_of(&self, obj: &ObjId) -> Option<usize> {
        self.by_id.get(obj).copied()
    }

    /// Records that `node` showed `obj` at `now`.
    pub fn record_shown(&mut self, obj: &ObjId, node: usize, now: u64, labels: &[String]) -> Shown {
        let Some(i) = self.index_of(obj) else {
            self.unknown_shown = self.unknown_shown.saturating_add(1);
            return Shown::Unknown;
        };
        let rec = &mut self.records[i];
        let Some(slot) = rec.shown.get_mut(node) else {
            return Shown::Unknown;
        };
        if *slot != u64::MAX {
            self.duplicate_shown = self.duplicate_shown.saturating_add(1);
            return Shown::Duplicate;
        }
        *slot = now;
        if !labels.is_empty() {
            rec.labels.insert(node, labels.to_vec());
        }
        Shown::First
    }
}

// ---- per-node statistics -------------------------------------------------------------

/// Sums the node counters of every node for the report.
#[derive(Debug, Default, Clone, Copy)]
pub struct NodeStats {
    pub cells_received: u64,
    pub cells_sent: u64,
    pub malformed: u64,
    pub duplicates: u64,
    pub throttled_in: u64,
    pub rejected_in: u64,
    pub no_room: u64,
    pub decrypted: u64,
    pub decrypt_failures: u64,
    pub rejected_posts: u64,
    pub parked: u64,
    pub handshakes: u64,
    pub commits: u64,
    pub rollbacks: u64,
    pub penalties: u64,
}

impl NodeStats {
    pub fn add(&mut self, o: &CoreStats) {
        self.cells_received += o.cells_received;
        self.cells_sent += o.cells_sent;
        self.malformed += o.malformed;
        self.duplicates += o.duplicates;
        self.throttled_in += o.throttled_in;
        self.rejected_in += o.rejected_in;
        self.no_room += o.no_room;
        self.decrypted += o.decrypted;
        self.decrypt_failures += o.decrypt_failures;
        self.rejected_posts += o.rejected_posts;
        self.parked += o.parked;
        self.handshakes += o.handshakes;
        self.commits += o.commits;
        self.rollbacks += o.rollbacks;
        self.penalties += o.penalties;
    }
}

// ---- node ----------------------------------------------------------------------------

/// Static configuration of one node, from the scenario and the seed.
#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub idx: usize,
    pub kind: NodeKind,
    /// Display name (BRIEF_v2 §6 step 1; never used to look anything up).
    pub name: String,
    /// GATT link slots.
    pub slots: u32,
    pub battery_floor_ppm: u64,
    pub battery_start_ppm: u64,
    /// iPhone foreground duty in parts per million.
    pub foreground_duty_ppm: u64,
    pub wall_offset_ms: i64,
    pub dedupe_retention_ms: u64,
}

/// The outcome of one node tick the engine acts on.
#[derive(Debug, Default)]
pub struct TickOutcome {
    /// The iPhone just left the locked state (the engine starts its dial
    /// attempts).
    pub became_foreground: bool,
}

/// One simulated phone or station. See the module docs.
pub struct SimNode {
    cfg: NodeConfig,
    pub clock: SimClock,
    node: Node,
    rng: ChaCha20Rng,
    slots: SlotPolicy,
    roles: BTreeMap<u32, Role>,
    foreground: bool,
    next_duty_window_ms: u64,
    powered_ms: u64,
    relay_ms: u64,
    last_tick_ms: u64,
    /// Events the node produced, stamped with the time of the call, not yet
    /// folded into the trace.
    events: Vec<(u64, Event)>,
    /// Radio batches this side handed to the air.
    pub batches_sent: u64,
    /// Posts the generator skipped because the node did not hold the group
    /// yet or was no longer a member.
    pub posts_skipped: u64,
    /// Rows in the kv store right after provisioning.
    pub kv_rows_at_start: u64,
    /// Display reads the report itself made through the node
    /// ([`SimNode::last_sync_set`]); not protocol reads.
    report_reads: StdCell<u64>,
}

impl core::fmt::Debug for SimNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SimNode")
            .field("idx", &self.cfg.idx)
            .field("kind", &self.cfg.kind)
            .field("foreground", &self.foreground)
            .field("links", &self.roles.len())
            .finish_non_exhaustive()
    }
}

impl SimNode {
    /// Opens the node at `now` on a fresh in-memory database keyed from the
    /// node's seed (identity seed, db key and entropy all derive from it, so
    /// the same seed gives the same node). `foreground` for an iPhone is
    /// drawn for the first duty window here.
    pub fn new(cfg: NodeConfig, seed: u64, now: u64) -> Result<Self, NodeError> {
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let mut id_seed = [0u8; 32];
        rng.fill_bytes(&mut id_seed);
        let mut db_key = [0u8; 32];
        rng.fill_bytes(&mut db_key);
        let mut entropy = [0u8; 32];
        rng.fill_bytes(&mut entropy);
        let node_rng = ChaCha20Rng::seed_from_u64(rng.next_u64());
        let db = Db::open_in_memory(db_key, entropy)?;
        let mut core = match cfg.kind {
            NodeKind::Station => CoreConfig::station(),
            NodeKind::Android | NodeKind::IPhone => CoreConfig::member(id_seed, cfg.name.clone()),
        };
        core.dedupe_retention_ms = cfg.dedupe_retention_ms;
        core.max_links = cfg.slots;
        core.battery_floor_ppm = u32::try_from(cfg.battery_floor_ppm).unwrap_or(u32::MAX);

        let mut node = Node::open(db, core, now, node_rng)?;
        node.set_receipt_batch_ms(meshcore::node::node::RECEIPT_BATCH_MS_ASM);
        let slots = SlotPolicy::new(cfg.slots);
        let clock = SimClock::new(cfg.wall_offset_ms);
        clock.set(now);
        let mut sim = SimNode {
            cfg,
            clock,
            node,
            rng,
            slots,
            roles: BTreeMap::new(),
            foreground: true,
            next_duty_window_ms: DUTY_WINDOW_MS,
            powered_ms: 0,
            relay_ms: 0,
            last_tick_ms: now,
            events: Vec::new(),
            batches_sent: 0,
            posts_skipped: 0,
            kv_rows_at_start: 0,
            report_reads: StdCell::new(0),
        };
        sim.draw_duty();
        Ok(sim)
    }

    fn draw_duty(&mut self) {
        if self.cfg.kind == NodeKind::IPhone {
            let roll: u64 = self.rng.random_range(0..PPM);
            self.foreground = roll < self.cfg.foreground_duty_ppm;
        } else {
            self.foreground = true;
        }
    }

    /// Attaches the display hook (after provisioning, so that the badge
    /// dates of the provisioned contacts never count as protocol reads).
    pub fn attach_display_clock(&mut self) {
        self.node.set_display_clock(Box::new(self.clock.clone()));
    }

    /// Turns this node into the `flood` adversary (docs/scenarios.md
    /// `behaviour = "flood"`): it keeps pushing through its own outbound
    /// pacing and through the peer's `Throttle` frames; containing it is the
    /// receiver's job (BRIEF_v2 §5.5 "rate limits per inbound link").
    pub fn set_flooding(&mut self, flooding: bool) {
        self.node.set_adversary_ignore_throttle(flooding);
    }

    // ---- accessors -----------------------------------------------------------

    pub fn idx(&self) -> usize {
        self.cfg.idx
    }

    pub fn kind(&self) -> NodeKind {
        self.cfg.kind
    }

    pub fn name(&self) -> &str {
        &self.cfg.name
    }

    /// The real node (commands and queries).
    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// This node's identity key (`None` for a station).
    pub fn identity(&self) -> Option<IdentityKey> {
        self.node.identity()
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
        self.roles.len()
    }

    pub fn holds(&self, id: &Hash32) -> bool {
        self.node.holds(id)
    }

    pub fn store_len(&self) -> usize {
        self.node.store_len()
    }

    pub fn store_bytes(&self) -> usize {
        self.node.store_bytes()
    }

    /// Wall-clock reads made during the run: every read the node makes
    /// through its display hook reaches the clock trait (the node's own
    /// counter and the clock's count the same call, so only the clock's is
    /// summed), plus any made through the trait directly, minus the
    /// report's own [`SimNode::last_sync_set`] reads.
    pub fn wall_reads(&self) -> u64 {
        self.clock
            .wall_reads()
            .saturating_sub(self.report_reads.get())
    }

    /// Whether "last sync" is set (BRIEF_v2 §2: the message digest matched
    /// a neighbour's; DESIGN §10 S0 "last-sync set"). The status strip reads
    /// the display clock for the "N min ago" when it is set; that read is
    /// the report's, not the protocol's, and is excluded from
    /// [`SimNode::wall_reads`].
    pub fn last_sync_set(&self, now: u64) -> bool {
        self.clock.set(now);
        let before = self.clock.wall_reads();
        let set = self.node.status_strip(now).last_sync_age_ms.is_some();
        let made = self.clock.wall_reads().saturating_sub(before);
        self.report_reads
            .set(self.report_reads.get().saturating_add(made));
        set
    }

    pub fn stats(&self) -> CoreStats {
        self.node.stats()
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

    /// Takes the events the node produced since the last call.
    pub fn take_events(&mut self) -> Vec<(u64, Event)> {
        std::mem::take(&mut self.events)
    }

    /// Moves the node's events into the pending list, stamped `now`.
    pub fn collect(&mut self, now: u64) {
        for e in self.node.poll_events() {
            self.events.push((now, e));
        }
    }

    // ---- links ----------------------------------------------------------------

    /// A link came up (DESIGN §7). `hint`: the contact the platform
    /// believes is on the other end (a QR-verified pair in the sim), so the
    /// node offers its L2 contact proof.
    pub fn link_up(
        &mut self,
        id: u32,
        role: Role,
        hint: Option<IdentityKey>,
        now: u64,
    ) -> Result<(), DropReason> {
        self.clock.set(now);
        self.node
            .link_up(LinkId(id), role, hint, now)
            .map_err(|_| DropReason::Refused)?;
        self.slots.on_link_up(role);
        self.roles.insert(id, role);
        self.collect(now);
        Ok(())
    }

    /// The link is gone: its queues go with it (the DTN store keeps the
    /// bytes; the next encounter re-summarises).
    pub fn link_down(&mut self, id: u32, now: u64) {
        self.clock.set(now);
        self.node.link_down(LinkId(id), now);
        self.forget_link(id);
        self.collect(now);
    }

    fn forget_link(&mut self, id: u32) {
        if let Some(role) = self.roles.remove(&id) {
            self.slots.on_link_down(role);
        }
    }

    /// Bytes the radio wants to carry from this side of `id`, at most
    /// `max_bytes` (the node services the link first).
    pub fn poll_outbound(&mut self, id: u32, max_bytes: usize, now: u64) -> Vec<u8> {
        self.clock.set(now);
        let bytes = self.node.poll_outbound(LinkId(id), max_bytes, now);
        if !bytes.is_empty() {
            self.batches_sent = self.batches_sent.saturating_add(1);
        }
        self.collect(now);
        bytes
    }

    /// Inbound radio bytes. `Err` means the node declared the link dead and
    /// removed it.
    pub fn feed(&mut self, id: u32, bytes: &[u8], now: u64) -> Result<(), DropReason> {
        self.clock.set(now);
        let r = self.node.feed(LinkId(id), bytes, now);
        self.collect(now);
        match r {
            Ok(()) => Ok(()),
            Err(NodeError::LinkDead(_)) => {
                self.forget_link(id);
                Err(DropReason::Dead)
            }
            Err(_) => Ok(()),
        }
    }

    // ---- tick ------------------------------------------------------------------

    /// One node tick (scenario `tick_ms`): battery meter, iPhone duty
    /// window, then the node's own tick (dedupe sweep, rotations due,
    /// delivery ageing, held links serviced).
    pub fn tick(&mut self, now: u64) -> TickOutcome {
        self.clock.set(now);
        let mut outcome = TickOutcome::default();
        let dt = now.saturating_sub(self.last_tick_ms);
        self.last_tick_ms = now;
        let relaying_now = !self.roles.is_empty() && self.is_relaying() && !self.is_station();
        self.powered_ms = self.powered_ms.saturating_add(dt);
        if relaying_now {
            self.relay_ms = self.relay_ms.saturating_add(dt);
        }
        if self.cfg.kind == NodeKind::IPhone && now >= self.next_duty_window_ms {
            let was = self.foreground;
            self.draw_duty();
            self.next_duty_window_ms = now.saturating_add(DUTY_WINDOW_MS);
            outcome.became_foreground = self.foreground && !was;
        }
        let level = u32::try_from(self.battery_ppm()).unwrap_or(u32::MAX);
        self.node.set_power(level, self.is_station());
        for e in self.node.tick(now) {
            self.events.push((now, e));
        }
        outcome
    }
}

// ---- trace ---------------------------------------------------------------------------

/// One `VoteChanged` event in the run.
#[derive(Debug, Clone)]
pub struct VoteEvent {
    pub at_ms: u64,
    pub node: usize,
    pub vote_id: VoteId,
    pub line: String,
    pub state: Option<VoteState>,
}

/// Everything the invariant checker reads, folded from node events as
/// they happen (module docs).
#[derive(Debug, Default)]
pub struct Trace {
    pub registry: ObjRegistry,
    last_delivery: BTreeMap<(usize, ObjId), DeliveryState>,
    /// A solid delivery state (`Delivered`) that reverted (S1 (f)).
    pub solid_reverts: Vec<String>,
    pub delivery_events: u64,
    pub votes: Vec<VoteEvent>,
    /// The first `Final` decision per (node, vote): the winning option and
    /// the line it was shown with.
    final_seen: BTreeMap<(usize, VoteId), (u8, String)>,
    /// A `Final` decision that changed afterwards (S4 (c): the state or the
    /// winner; the displayed counts may still grow as late ballots arrive).
    pub final_changes: Vec<String>,
    /// `TaskChanged` events seen.
    pub task_events: u64,
    done_seen: BTreeSet<(usize, GroupId, TaskId)>,
    /// A `Done` task that reverted (S1 (f)).
    pub done_reverts: Vec<String>,
    /// Keys whose removal from a roster the trace tracks (scenario
    /// removals and sock-puppet keys).
    pub tracked_keys: BTreeSet<IdentityKey>,
    /// `(node, key)` → powered-on time the node's `members(H)` first lacked
    /// `key` after holding it (the node applied the removal, or voided the Add).
    pub removal_applied: BTreeMap<(usize, IdentityKey), u64>,
    member_seen: BTreeSet<(usize, IdentityKey)>,
    pub roster_changes: u64,
    pub needs_key: u64,
    pub same_name: u64,
    pub link_ups: u64,
    pub l2_links: u64,
}

impl Trace {
    pub fn new(n_nodes: usize) -> Self {
        Trace {
            registry: ObjRegistry::new(n_nodes),
            ..Self::default()
        }
    }

    /// Folds one event of node `n` into the trace.
    pub fn on_event(&mut self, n: usize, now: u64, ev: &Event, node: &SimNode) {
        match ev {
            Event::Post { obj, labels, .. } => {
                self.registry.record_shown(obj, n, now, labels);
            }
            Event::Dm { obj, .. } => {
                self.registry.record_shown(obj, n, now, &[]);
            }
            Event::Delivery { obj, state } => {
                self.delivery_events = self.delivery_events.saturating_add(1);
                if let Some(prev) = self.last_delivery.get(&(n, *obj))
                    && prev.is_solid()
                    && !state.is_solid()
                {
                    self.solid_reverts.push(format!(
                        "node {n} obj {obj} {} -> {} at {now} ms",
                        prev.text(),
                        state.text()
                    ));
                }
                self.last_delivery.insert((n, *obj), *state);
            }
            Event::RosterChanged { group, .. } => {
                self.roster_changes = self.roster_changes.saturating_add(1);
                self.note_roster(n, now, group, node);
            }
            Event::TaskChanged { group, task_id, ui } => {
                let key = (n, *group, *task_id);
                if self.done_seen.contains(&key) && ui != "Done" {
                    self.done_reverts.push(format!(
                        "node {n} task {} Done -> {ui} at {now} ms",
                        hex::encode(task_id)
                    ));
                }
                if ui == "Done" {
                    self.done_seen.insert(key);
                }
                self.task_events = self.task_events.saturating_add(1);
            }
            Event::VoteChanged {
                vote_id,
                line,
                state,
            } => {
                let key = (n, *vote_id);
                let option = match state {
                    Some(VoteState::Final { option }) => Some(*option),
                    _ => None,
                };
                if let Some((first_option, first_line)) = self.final_seen.get(&key)
                    && option != Some(*first_option)
                {
                    self.final_changes.push(format!(
                        "node {n}: \"{first_line}\" -> \"{line}\" at {now} ms"
                    ));
                }
                if let Some(option) = option
                    && !self.final_seen.contains_key(&key)
                {
                    self.final_seen.insert(key, (option, line.clone()));
                }
                self.votes.push(VoteEvent {
                    at_ms: now,
                    node: n,
                    vote_id: *vote_id,
                    line: line.clone(),
                    state: state.clone(),
                });
            }
            Event::NeedsKey { .. } => self.needs_key = self.needs_key.saturating_add(1),
            Event::SameName { .. } => self.same_name = self.same_name.saturating_add(1),
            Event::LinkState { up, auth, .. } => {
                if *up {
                    self.link_ups = self.link_ups.saturating_add(1);
                    if matches!(auth, LinkAuth::VerifiedContact(_)) {
                        self.l2_links = self.l2_links.saturating_add(1);
                    }
                }
            }
        }
    }

    /// Checks the tracked keys against the node's current `members(H)`.
    fn note_roster(&mut self, n: usize, now: u64, group: &GroupId, node: &SimNode) {
        if self.tracked_keys.is_empty() {
            return;
        }
        let Ok(view) = node.node().roster(group) else {
            return;
        };
        for key in &self.tracked_keys {
            let member = view.is_member(key);
            let k = (n, *key);
            if member {
                self.member_seen.insert(k);
            } else if self.member_seen.contains(&k) && !self.removal_applied.contains_key(&k) {
                self.removal_applied.insert(k, now);
            }
        }
    }

    /// Seeds the member-seen set from a node's current roster (called once
    /// a tracked key becomes known, so a later disappearance counts).
    pub fn seed_membership(&mut self, n: usize, group: &GroupId, node: &SimNode) {
        if let Ok(view) = node.node().roster(group) {
            for key in &self.tracked_keys {
                if view.is_member(key) {
                    self.member_seen.insert((n, *key));
                }
            }
        }
    }

    #[cfg(test)]
    pub fn delivery_state(&self, n: usize, obj: &ObjId) -> Option<DeliveryState> {
        self.last_delivery.get(&(n, *obj)).copied()
    }

    /// Objects whose sender's delivery state is `Delivered` now (BRIEF_v2
    /// §5.9; DESIGN §10 S0 "every text Delivered; receipts").
    pub fn delivered_at_sender(&self) -> u64 {
        self.last_delivery
            .values()
            .filter(|s| **s == DeliveryState::Delivered)
            .count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(idx: usize, kind: NodeKind) -> NodeConfig {
        NodeConfig {
            idx,
            kind,
            name: format!("Node {idx}"),
            slots: 7,
            battery_floor_ppm: 300_000,
            battery_start_ppm: 1_000_000,
            foreground_duty_ppm: 1_000_000,
            wall_offset_ms: 0,
            dedupe_retention_ms: 10 * 24 * 3_600_000,
        }
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
        let shared = c.clone();
        shared.set(200);
        assert_eq!(c.mono_ms(), 200, "handles share one state");
    }

    #[test]
    fn registry_records_first_show_duplicates_and_unknowns() {
        let mut reg = ObjRegistry::new(3);
        let obj = Hash32::ZERO;
        reg.register(
            obj,
            0,
            10,
            0,
            0,
            ObjKind::Post { group: 0 },
            BTreeSet::from([1, 2]),
            false,
            false,
        );
        assert_eq!(reg.records[0].shown, vec![10, u64::MAX, u64::MAX]);
        assert_eq!(reg.record_shown(&obj, 1, 50, &["x".into()]), Shown::First);
        assert_eq!(reg.record_shown(&obj, 1, 60, &[]), Shown::Duplicate);
        assert_eq!(reg.duplicate_shown, 1);
        assert_eq!(
            reg.records[0].labels.get(&1).unwrap(),
            &vec!["x".to_owned()]
        );
        let other = meshcore::crypto::hash::sha256(b"other");
        assert_eq!(reg.record_shown(&other, 2, 1, &[]), Shown::Unknown);
        assert_eq!(reg.unknown_shown, 1);
    }

    #[test]
    fn battery_drains_and_the_floor_stops_relaying() {
        let mut c = cfg(0, NodeKind::Android);
        c.battery_start_ppm = 400_000;
        let mut n = SimNode::new(c, 9, 0).unwrap();
        assert!(n.is_relaying());
        // 1 %/h idle: after 11 h the level is 29 % < 30 % floor
        n.tick(11 * MS_PER_H);
        assert_eq!(n.battery_ppm(), 290_000);
        assert!(!n.is_relaying());
        let mut s = SimNode::new(cfg(1, NodeKind::Station), 10, 0).unwrap();
        s.tick(100 * MS_PER_H);
        assert_eq!(s.battery_ppm(), PPM, "stations are mains powered");
        assert!(s.identity().is_none());
        assert_eq!(n.wall_reads(), 0);
    }

    #[test]
    fn locked_iphone_never_dials_and_duty_windows_redraw() {
        let mut c = cfg(0, NodeKind::IPhone);
        c.foreground_duty_ppm = 0;
        c.slots = IPHONE_SLOTS;
        let mut n = SimNode::new(c, 4, 0).unwrap();
        assert!(n.is_locked_iphone());
        assert!(!n.can_dial());
        assert!(n.accepting_inbound());
        let mut c = cfg(0, NodeKind::IPhone);
        c.foreground_duty_ppm = PPM;
        let mut m = SimNode::new(c, 4, 0).unwrap();
        assert!(m.is_foreground() && m.can_dial());
        let o = n.tick(DUTY_WINDOW_MS);
        assert!(!o.became_foreground);
        let o = m.tick(DUTY_WINDOW_MS);
        assert!(!o.became_foreground, "already foreground: no transition");
    }

    #[test]
    fn two_nodes_handshake_over_the_wrapper_and_exchange_a_dm() {
        let mut a = SimNode::new(cfg(0, NodeKind::Android), 1, 0).unwrap();
        let mut b = SimNode::new(cfg(1, NodeKind::Android), 2, 0).unwrap();
        // provisioning as the engine does it
        let ka = [7u8; 32];
        let import_a = meshcore::node::ImportedContact {
            keycard: a.node().my_keycard().unwrap(),
            display_name: a.name().to_owned(),
            bundle: a
                .node_mut()
                .my_bundle(
                    meshcore::pairwise::bundles::IssueContext::DirectEncounter,
                    0,
                )
                .unwrap(),
            link_key: Some(ka),
            envelope_pub: a.node().my_envelope_pub().unwrap(),
            badge: meshcore::node::ImportBadge::VerifiedInPerson,
        };
        let import_b = meshcore::node::ImportedContact {
            keycard: b.node().my_keycard().unwrap(),
            display_name: b.name().to_owned(),
            bundle: b
                .node_mut()
                .my_bundle(
                    meshcore::pairwise::bundles::IssueContext::DirectEncounter,
                    0,
                )
                .unwrap(),
            link_key: Some(ka),
            envelope_pub: b.node().my_envelope_pub().unwrap(),
            badge: meshcore::node::ImportBadge::VerifiedInPerson,
        };
        a.node_mut().import_contact(import_b, 0).unwrap();
        b.node_mut().import_contact(import_a, 0).unwrap();
        a.attach_display_clock();
        b.attach_display_clock();
        a.link_up(1, Role::Central, b.identity(), 10).unwrap();
        b.link_up(1, Role::Peripheral, a.identity(), 10).unwrap();
        assert_eq!(a.link_count(), 1);
        let obj = a
            .node_mut()
            .send_dm(b.identity().unwrap(), "hi", 20)
            .unwrap();
        let mut trace = Trace::new(2);
        trace.registry.register(
            obj,
            0,
            20,
            0,
            0,
            ObjKind::Dm { to: 1 },
            BTreeSet::from([1]),
            false,
            false,
        );
        let mut now = 20;
        for _ in 0..200 {
            now += 2_000;
            let ab = a.poll_outbound(1, usize::MAX, now);
            if !ab.is_empty() {
                b.feed(1, &ab, now).unwrap();
            }
            let ba = b.poll_outbound(1, usize::MAX, now);
            if !ba.is_empty() {
                a.feed(1, &ba, now).unwrap();
            }
            a.tick(now);
            b.tick(now);
            for (t, e) in a.take_events() {
                trace.on_event(0, t, &e, &a);
            }
            for (t, e) in b.take_events() {
                trace.on_event(1, t, &e, &b);
            }
            if trace.registry.records[0].shown[1] != u64::MAX {
                break;
            }
        }
        assert_ne!(
            trace.registry.records[0].shown[1],
            u64::MAX,
            "B showed the DM"
        );
        // the batched receipt (`node::node::RECEIPT_BATCH_MS_ASM`) settles
        // the delivery state once its window passed
        for _ in 0..40 {
            now += 60_000;
            let ab = a.poll_outbound(1, usize::MAX, now);
            if !ab.is_empty() {
                b.feed(1, &ab, now).unwrap();
            }
            let ba = b.poll_outbound(1, usize::MAX, now);
            if !ba.is_empty() {
                a.feed(1, &ba, now).unwrap();
            }
            a.tick(now);
            b.tick(now);
            for (t, e) in a.take_events() {
                trace.on_event(0, t, &e, &a);
            }
            for (t, e) in b.take_events() {
                trace.on_event(1, t, &e, &b);
            }
            if trace.delivery_state(0, &obj) == Some(DeliveryState::Delivered) {
                break;
            }
        }
        assert_eq!(
            trace.delivery_state(0, &obj),
            Some(DeliveryState::Delivered)
        );
        assert!(trace.solid_reverts.is_empty());
        assert!(trace.l2_links >= 1, "the pair's link is L2");
        assert_eq!(a.wall_reads() + b.wall_reads(), 0);
        // a display read through the node counts once (the node's own
        // counter and the clock's count the same call), and the report's
        // own last-sync read is not a protocol read
        let _ = a.last_sync_set(now);
        assert_eq!(a.wall_reads(), 0);
        assert_eq!(a.node().wall_reads(), a.clock.wall_reads());
        let _ = a.node().name_of(&b.identity().unwrap());
        assert_eq!(a.wall_reads(), 0, "name_of is not a wall read");
        a.link_down(1, now);
        b.link_down(1, now);
        assert_eq!(a.link_count(), 0);
    }
}
