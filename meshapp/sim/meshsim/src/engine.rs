//! The deterministic discrete-event loop (DESIGN §7 `SimLink` row, §10
//! "Simulator scenarios"; docs/scenarios.md). One [`Sim`] runs one scenario
//! for one seed over real [`meshcore::node::Node`]s: the event queue is
//! ordered by `(time, sequence)`, every random draw comes from a
//! `ChaCha20Rng` seeded from the run's seed (or from a per-node RNG seeded
//! from it), and all iteration is over ordered collections, so the same
//! seed gives byte-identical reports.
//!
//! Event kinds: `NodeTick(n)` (the node's `tick_ms` tick), `LinkAttempt(n)`
//! (a node tries to connect to a random reachable node, DESIGN §7 "link
//! establishment"), `LinkUp`/`LinkDown`, `Deliver` (a batch of bytes
//! arrives at one end of a link), `Traffic` (one node's next post, DM or
//! junk batch), `Scripted(idx)` (a `[[events]]` entry), `Cast` (one
//! scheduled ballot), `Carry` (the courier crosses into another fragment),
//! `Release` (withheld DMs go out), `Replay`, `Snapshot` (a vote line is
//! read on every node for `expected_vote_states`), `PartitionChange(idx)`
//! and `End`.
//!
//! # Provisioning (docs/scenarios.md `[groups]`, `[pairs]`)
//!
//! Before the first event every member imports every co-member and every
//! pair peer as a contact (`Node::import_contact`: KeyCard, prekey bundle,
//! envelope key; pairs also share a `k_pair` link key and the "Verified in
//! person" badge). Co-member sessions stand in for the in-group bundle
//! distribution the core does not carry yet (DECISIONS §3 row (4,5,6,7)).
//! Each group is then created by its first admin with every member in the
//! genesis (`Node::create_group`), which the node shares **over the mesh**
//! as sealed first flights: no roster op is pre-loaded, so every node's
//! roster DAG is the one the real ops produce. A member whose genesis has
//! not arrived yet skips its scheduled post (counted) and tries again at
//! the next period.
//!
//! # What the generator and the events do
//!
//! Posts are `Node::send_text`, DMs `Node::send_dm`, receipts happen inside
//! the node. Scenario events act through the real commands: `remove_member`
//! / `add_member` (with a second admin's endorsement), `task_*`,
//! `vote_open`, `ballot` (spread over `spread_ms`), and the adversary hooks
//! (`keep_keys_after_removal` → `set_adversary_keep_keys`; `sockpuppet_adds`
//! → three fresh KeyCards added by the insider with an endorsement from an
//! admin in its fragment; `withhold_dms` → the DMs are composed at the event
//! and only sent `hold_ms` later; `replay_cells` → `adversary_replay_held`;
//! `equivocate_ballot` → one ballot now, then at `carry_at_ms` the courier
//! is moved into `carry_to_fragment` and casts the other choice; `flood` →
//! `adversary_inject_junk` with the throttle-ignoring hook). A Take or a
//! ballot whose task or vote has not reached the node yet is retried on
//! every tick of that node until it applies.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use meshcore::crypto::backend::GroupId;
use meshcore::crypto::ed25519::{IdentityKey, IdentitySigner};
use meshcore::crypto::hash::Hash32;
use meshcore::crypto::keycard::KeyCard;
use meshcore::dtn::dedupe::{KEY_RETENTION_ASM_MS, dedupe_retention_ms};
use meshcore::node::{ImportBadge, ImportedContact, NodeError, ObjId, VoteId};
use meshcore::objects::roster_op::MemberEntry;
use meshcore::objects::task_op::TaskId;
use meshcore::pairwise::bundles::IssueContext;
use meshcore::transport::link::{LinkId, Role};
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::assert::{self, EvalInput, FloodInfo, VoteSnapshot};
use crate::radio::{Radio, RadioParams};
use crate::report::{
    BytesOnLinks, CellTraffic, CellsCreated, Links, NodeReport, Report, VoteSnapshotReport,
};
use crate::scenario::{
    self, Behaviour, NodeKind, Resolved, ResolvedPartition, Scenario, ScenarioError,
};
use crate::simnode::{
    DropReason, IPHONE_SLOTS, NodeConfig, NodeStats, ObjKind, SimNode, TickOutcome, Trace,
};

/// Largest batch of bytes one `Deliver` event carries (assumption; the
/// node's stream framer takes any chunking).
pub const BATCH_BYTES: usize = 4096;
/// Bytes of one cell-carrying Noise frame on the wire (length prefix +
/// frame header + 512-byte cell + 16-byte tag; assumption): the per-frame
/// loss is rolled once per this many bytes of a batch.
pub const FRAME_BYTES_ASM: usize = 531;
/// Connect latency from dial to link up (assumption: a BLE connection
/// takes a few connection intervals).
pub const CONNECT_DELAY_MS_ASM: u64 = 200;
/// Dial period per node: "every 2 s ± jitter".
pub const DIAL_PERIOD_MS: u64 = 2_000;
pub const DIAL_JITTER_MS: u64 = 500;
/// DMs the `withhold_dms` adversary composes when the event names no
/// count (docs/scenarios.md S3 "withholds 20 DMs").
pub const WITHHELD_DMS_DEFAULT: u32 = 20;
/// The `replay_cells` adversary re-offers this many held cells per replay
/// and repeats every [`REPLAY_PERIOD_MS`] (assumption).
pub const REPLAY_BATCH: usize = 256;
pub const REPLAY_PERIOD_MS: u64 = 30 * 60_000;
/// Junk is injected once per second in batches (`cells_per_min / 60`).
pub const FLOOD_PERIOD_MS: u64 = 1_000;
/// Link ids of the setup phase (above any id the radio hands out).
const SETUP_LINK_BASE: u32 = 0x8000_0000;
/// Setup rounds over every co-member/pair link (a round that moves nothing
/// ends the phase early).
const SETUP_ROUNDS_MAX: usize = 4;
/// Poll/feed iterations per setup link.
const SETUP_PUMP_MAX: usize = 100_000;

/// What a `Traffic` event generates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrafficKind {
    /// A post to group `g`.
    GroupPost(usize),
    /// A DM in pair `p` (the event's node is the sender; the reply comes
    /// from the other end).
    Dm(usize),
    /// A junk batch from a flooding node every [`FLOOD_PERIOD_MS`].
    Flood { per_batch: u32 },
}

/// One queued event (module docs).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    NodeTick(usize),
    LinkAttempt(usize),
    LinkUp {
        a: usize,
        b: usize,
    },
    LinkDown {
        link: u32,
        reason: DropReason,
    },
    Deliver {
        link: u32,
        from: usize,
        bytes: Vec<u8>,
    },
    Traffic {
        node: usize,
        kind: TrafficKind,
    },
    Scripted(usize),
    Cast {
        node: usize,
        vote: String,
        choice: usize,
    },
    Carry {
        node: usize,
        fragment: usize,
        vote: String,
        choice: usize,
    },
    Release(usize),
    Replay(usize),
    Snapshot(usize),
    PartitionChange(usize),
    End,
}

/// A task op or ballot waiting for its task or vote to reach the node.
#[derive(Debug, Clone)]
enum PendingKind {
    Take(String),
    Release(String),
    Done(String),
    Ballot { vote: String, choice: usize },
}

#[derive(Debug, Clone)]
struct Pending {
    node: usize,
    since_ms: u64,
    kind: PendingKind,
}

impl Pending {
    fn name(&self) -> &'static str {
        match self.kind {
            PendingKind::Take(_) => "task_take",
            PendingKind::Release(_) => "task_release",
            PendingKind::Done(_) => "task_done",
            PendingKind::Ballot { .. } => "ballot",
        }
    }
}

#[derive(Debug, Default)]
struct Metrics {
    established: u64,
    wake_links: u64,
    attempts: u64,
    attempts_failed: u64,
    dropped_by_reason: BTreeMap<DropReason, u64>,
    events_applied: BTreeMap<String, u64>,
    events_failed: BTreeMap<String, u64>,
    cells_created: CellsCreated,
    frames_sent: u64,
}

/// One run. Build with [`Sim::new`], drive with [`Sim::run_to_end`], read
/// with [`Sim::report`].
pub struct Sim {
    scenario: Scenario,
    resolved: Resolved,
    seed: u64,
    duration_ms: u64,
    tick_ms: u64,
    now_ms: u64,
    queue: BinaryHeap<Reverse<(u64, u64, Event)>>,
    seq: u64,
    rng: ChaCha20Rng,
    nodes: Vec<SimNode>,
    radio: Radio,
    trace: Trace,
    /// Live link ids per node.
    links_of: Vec<BTreeSet<u32>>,
    pairs: Vec<(usize, usize)>,
    pair_set: BTreeSet<(usize, usize)>,
    ids: Vec<Option<IdentityKey>>,
    group_ids: Vec<Option<GroupId>>,
    vote_ids: BTreeMap<String, VoteId>,
    task_ids: BTreeMap<String, (usize, TaskId)>,
    pending: Vec<Pending>,
    /// Nodes the courier event moved out of their scheduled fragment.
    fragment_override: BTreeMap<usize, usize>,
    current_partition: usize,
    puppets: Vec<IdentityKey>,
    withheld: BTreeMap<usize, Vec<(usize, String)>>,
    flood: Option<(usize, u64)>,
    junk_ids: BTreeSet<Hash32>,
    replay_offers: u64,
    vote_snapshots: Vec<VoteSnapshot>,
    /// Honest ballots cast through the generator: label → (node, at).
    ballots_cast: BTreeMap<String, Vec<(usize, u64)>>,
    equivocators: BTreeSet<usize>,
    adversaries: BTreeSet<usize>,
    removal_subjects: BTreeSet<usize>,
    metrics: Metrics,
    rejoin_at: Option<u64>,
    split_at: Option<u64>,
    baseline_latency_ms: Option<f64>,
    ended: bool,
}

impl core::fmt::Debug for Sim {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Sim")
            .field("scenario", &self.scenario.scenario.name)
            .field("seed", &self.seed)
            .field("now_ms", &self.now_ms)
            .field("nodes", &self.nodes.len())
            .finish_non_exhaustive()
    }
}

fn provisioning(e: impl core::fmt::Display) -> ScenarioError {
    ScenarioError::Invalid(format!("provisioning: {e}"))
}

impl Sim {
    /// Sets up nodes, contacts, groups, pairs, the radio and the initial
    /// events (module docs "Provisioning").
    pub fn new(
        scenario: &Scenario,
        seed: u64,
        duration_override_ms: Option<u64>,
    ) -> Result<Self, ScenarioError> {
        let resolved = scenario.resolve()?;
        let n = resolved.n_nodes;
        let duration_ms = duration_override_ms.unwrap_or(scenario.scenario.duration_ms);
        if duration_ms == 0 {
            return Err(ScenarioError::Invalid(
                "duration override must be positive".into(),
            ));
        }
        let mut rng = ChaCha20Rng::seed_from_u64(seed);

        // Pairs: explicit ones, then random ones drawn without replacement
        // from the remaining phone pairs (stations have no identity).
        let mut pairs = resolved.explicit_pairs.clone();
        let taken: BTreeSet<(usize, usize)> = pairs.iter().copied().collect();
        let phones: Vec<usize> = (0..n)
            .filter(|&i| resolved.kinds[i] != NodeKind::Station)
            .collect();
        let mut candidates: Vec<(usize, usize)> = Vec::new();
        for (i, &a) in phones.iter().enumerate() {
            for &b in &phones[i + 1..] {
                if !taken.contains(&(a, b)) {
                    candidates.push((a, b));
                }
            }
        }
        let want = resolved.random_pairs.min(candidates.len());
        for i in 0..want {
            let j = rng.random_range(i..candidates.len());
            candidates.swap(i, j);
            pairs.push(candidates[i]);
        }
        pairs.sort_unstable();
        let pair_set: BTreeSet<(usize, usize)> = pairs.iter().copied().collect();

        let dedupe_retention_ms = dedupe_retention_ms(Some(duration_ms), KEY_RETENTION_ASM_MS);
        let [lo, hi] = scenario.radio.wall_clock_offset_ms;
        let duty_ppm = fraction_ppm(scenario.nodes.iphone_foreground_duty);
        let floor_ppm = fraction_ppm(scenario.nodes.battery_floor);
        let mut nodes = Vec::with_capacity(n);
        for i in 0..n {
            let kind = resolved.kinds[i];
            let battery_start_ppm: u64 = rng.random_range(400_000..=1_000_000);
            let wall_offset_ms: i64 = if lo == hi {
                lo
            } else {
                rng.random_range(lo..=hi)
            };
            let node_seed = rng.next_u64();
            let cfg = NodeConfig {
                idx: i,
                kind,
                name: format!("Node {i}"),
                slots: match kind {
                    NodeKind::IPhone => IPHONE_SLOTS,
                    NodeKind::Android | NodeKind::Station => scenario.nodes.android_slots,
                },
                battery_floor_ppm: floor_ppm,
                battery_start_ppm,
                foreground_duty_ppm: duty_ppm,
                wall_offset_ms,
                dedupe_retention_ms,
            };
            nodes.push(SimNode::new(cfg, node_seed, 0).map_err(provisioning)?);
        }
        let ids: Vec<Option<IdentityKey>> = nodes.iter().map(SimNode::identity).collect();

        let params = RadioParams {
            latency_ms: scenario.radio.latency_ms,
            throughput_bytes_per_s: scenario.radio.throughput_bytes_per_s,
            frame_loss: scenario.radio.frame_loss,
            iphone_wake_ms: scenario.radio.iphone_wake_ms,
        };
        let mut radio = Radio::new(params, resolved.kinds.clone());
        radio.set_partition(resolved.partition_at(0));
        let group_count = resolved.groups.len();

        let mut sim = Sim {
            scenario: scenario.clone(),
            resolved,
            seed,
            duration_ms,
            tick_ms: scenario.scenario.tick_ms,
            now_ms: 0,
            queue: BinaryHeap::new(),
            seq: 0,
            rng,
            nodes,
            radio,
            trace: Trace::new(n),
            links_of: vec![BTreeSet::new(); n],
            pairs,
            pair_set,
            ids,
            group_ids: vec![None; group_count],
            vote_ids: BTreeMap::new(),
            task_ids: BTreeMap::new(),
            pending: Vec::new(),
            fragment_override: BTreeMap::new(),
            current_partition: 0,
            puppets: Vec::new(),
            withheld: BTreeMap::new(),
            flood: None,
            junk_ids: BTreeSet::new(),
            replay_offers: 0,
            vote_snapshots: Vec::new(),
            ballots_cast: BTreeMap::new(),
            equivocators: BTreeSet::new(),
            adversaries: BTreeSet::new(),
            removal_subjects: BTreeSet::new(),
            metrics: Metrics::default(),
            rejoin_at: None,
            split_at: None,
            baseline_latency_ms: None,
            ended: false,
        };
        sim.provision()?;
        sim.schedule_initial();
        Ok(sim)
    }

    // ---- provisioning ------------------------------------------------------------

    fn export_contact(
        &mut self,
        i: usize,
        context: IssueContext,
        link_key: Option<[u8; 32]>,
    ) -> Result<ImportedContact, NodeError> {
        let n = self.nodes[i].node_mut();
        Ok(ImportedContact {
            keycard: n.my_keycard()?,
            display_name: n.display_name().unwrap_or_default().to_owned(),
            bundle: n.my_bundle(context, 0)?,
            link_key,
            envelope_pub: n.my_envelope_pub()?,
            badge: if link_key.is_some() {
                ImportBadge::VerifiedInPerson
            } else {
                ImportBadge::Unverified
            },
        })
    }

    /// Module docs "Provisioning".
    fn provision(&mut self) -> Result<(), ScenarioError> {
        // Which unordered pairs need a session: co-members (no link key)
        // and verified pairs (shared link key, in-person badge).
        let mut wanted: BTreeMap<(usize, usize), bool> = BTreeMap::new();
        for g in &self.resolved.groups {
            let members: Vec<usize> = g.members.iter().copied().collect();
            for (k, &a) in members.iter().enumerate() {
                for &b in &members[k + 1..] {
                    wanted.entry((a, b)).or_insert(false);
                }
            }
        }
        for &(a, b) in &self.pairs {
            wanted.insert((a, b), true);
        }
        // A fresh bundle per import: every bundle carries a one-time Kyber
        // prekey that the backend deletes on first use (PQXDH last resort;
        // `backend::libsignal`), so a bundle can serve exactly one peer.
        // Co-members get an in-group bundle without one-time EC prekeys (D6
        // rule 2); a pair gets a direct-encounter bundle with its reserve.
        let group_ctx = IssueContext::InsideGroup(GroupId([0u8; 16]));
        for ((a, b), pair) in wanted {
            let (ca, cb) = if pair {
                let mut link_key = [0u8; 32];
                self.rng.fill_bytes(&mut link_key);
                (
                    self.export_contact(a, IssueContext::DirectEncounter, Some(link_key))
                        .map_err(provisioning)?,
                    self.export_contact(b, IssueContext::DirectEncounter, Some(link_key))
                        .map_err(provisioning)?,
                )
            } else {
                (
                    self.export_contact(a, group_ctx, None)
                        .map_err(provisioning)?,
                    self.export_contact(b, group_ctx, None)
                        .map_err(provisioning)?,
                )
            };
            self.nodes[b]
                .node_mut()
                .import_contact(ca, 0)
                .map_err(provisioning)?;
            self.nodes[a]
                .node_mut()
                .import_contact(cb, 0)
                .map_err(provisioning)?;
        }
        // Groups: genesis by the first admin, shared over the mesh.
        let groups = self.resolved.groups.clone();
        for (g, rg) in groups.iter().enumerate() {
            let creator = *rg.admins.iter().next().ok_or_else(|| {
                ScenarioError::Invalid(format!("group {:?} has no admins", rg.name))
            })?;
            let mut admins = Vec::new();
            let mut members = Vec::new();
            for &m in &rg.members {
                if m == creator {
                    continue;
                }
                let card = self.nodes[m].node().my_keycard().map_err(provisioning)?;
                if rg.admins.contains(&m) {
                    admins.push(card);
                }
                members.push(MemberEntry {
                    keycard: card,
                    name: self.nodes[m].name().to_owned(),
                    in_person: true,
                });
            }
            let gid = self.nodes[creator]
                .node_mut()
                .create_group(&rg.name, &admins, &members, 0, 0)
                .map_err(provisioning)?;
            self.group_ids[g] = Some(gid);
        }
        self.setup_phase()?;
        for n in 0..self.nodes.len() {
            self.drain(n);
            self.nodes[n].attach_display_clock();
            let rows = self.nodes[n].node().kv_len().map_err(provisioning)?;
            self.nodes[n].kv_rows_at_start = rows;
        }
        // Scenario removal subjects are tracked from the start.
        for e in &self.scenario.events {
            if let scenario::Event::RemoveMember { subject, .. } = e {
                self.removal_subjects.insert(*subject);
                if let Some(k) = self.ids[*subject] {
                    self.trace.tracked_keys.insert(k);
                }
            }
        }
        Ok(())
    }

    /// The in-person setup phase (module docs "Provisioning"): with every
    /// node's budgets off (`Node::set_provisioning`), the creator of each
    /// group meets every member directly so the genesis arrives, every
    /// member posts one setup hello (its first Sender Key rotation, so its
    /// chain reaches every co-member) and every pair exchanges a setup DM
    /// (first flights both ways), then every co-member/pair link is pumped
    /// until nothing new moves. No radio, no latency, no loss: this is the
    /// key exchange a setup session does in the same room. Everything that
    /// follows runs over the modelled radio with every budget on.
    fn setup_phase(&mut self) -> Result<(), ScenarioError> {
        for n in &mut self.nodes {
            n.node_mut().set_provisioning(true);
        }
        let mut next_link = SETUP_LINK_BASE;
        let groups = self.resolved.groups.clone();
        // 1. the genesis, creator to member
        for rg in &groups {
            let Some(&creator) = rg.admins.iter().next() else {
                continue;
            };
            for &m in &rg.members {
                if m != creator {
                    self.setup_pump(creator, m, next_link)?;
                    next_link = next_link.wrapping_add(1);
                }
            }
        }
        // 2. setup hellos
        for (g, rg) in groups.iter().enumerate() {
            let Some(gid) = self.group_ids[g] else {
                continue;
            };
            for &m in &rg.members {
                let hello = format!("hello from {}", self.nodes[m].name());
                match self.nodes[m].node_mut().send_text(gid, &hello, 0) {
                    Ok(obj) => {
                        let expected: BTreeSet<usize> =
                            rg.members.iter().copied().filter(|&x| x != m).collect();
                        self.trace.registry.register(
                            obj,
                            m,
                            0,
                            0,
                            0,
                            ObjKind::Post { group: g },
                            expected,
                            false,
                            true,
                        );
                        self.metrics.cells_created.setup += 1;
                    }
                    Err(e) => return Err(provisioning(format!("setup hello: {e}"))),
                }
            }
        }
        let pairs = self.pairs.clone();
        for &(a, b) in &pairs {
            for (from, to) in [(a, b), (b, a)] {
                let Some(peer) = self.ids[to] else {
                    continue;
                };
                let hello = format!("hello from {}", self.nodes[from].name());
                match self.nodes[from].node_mut().send_dm(peer, &hello, 0) {
                    Ok(obj) => {
                        self.trace.registry.register(
                            obj,
                            from,
                            0,
                            0,
                            0,
                            ObjKind::Dm { to },
                            BTreeSet::from([to]),
                            false,
                            true,
                        );
                        self.metrics.cells_created.setup += 1;
                    }
                    Err(e) => return Err(provisioning(format!("setup dm: {e}"))),
                }
            }
        }
        // 3. everyone meets everyone they share a group or a pair with,
        //    until a full round moves nothing new.
        let mut links: BTreeSet<(usize, usize)> = self.pair_set.clone();
        for rg in &groups {
            let members: Vec<usize> = rg.members.iter().copied().collect();
            for (k, &a) in members.iter().enumerate() {
                for &b in &members[k + 1..] {
                    links.insert((a, b));
                }
            }
        }
        for _round in 0..SETUP_ROUNDS_MAX {
            let before: Vec<usize> = self.nodes.iter().map(SimNode::store_len).collect();
            for &(a, b) in &links {
                self.setup_pump(a, b, next_link)?;
                next_link = next_link.wrapping_add(1);
            }
            let after: Vec<usize> = self.nodes.iter().map(SimNode::store_len).collect();
            if before == after {
                break;
            }
        }
        for n in &mut self.nodes {
            n.node_mut().set_provisioning(false);
        }
        Ok(())
    }

    /// One direct setup link between `a` and `b`, pumped until quiescent.
    fn setup_pump(&mut self, a: usize, b: usize, link: u32) -> Result<(), ScenarioError> {
        let (ha, hb) = (self.hint_for(a, b), self.hint_for(b, a));
        self.nodes[a]
            .link_up(link, Role::Central, ha, 0)
            .map_err(|r| provisioning(format!("setup link {a}-{b}: {r:?}")))?;
        self.nodes[b]
            .link_up(link, Role::Peripheral, hb, 0)
            .map_err(|r| provisioning(format!("setup link {a}-{b}: {r:?}")))?;
        let mut idle = 0u32;
        for _ in 0..SETUP_PUMP_MAX {
            let mut moved = false;
            let ab = self.nodes[a].poll_outbound(link, usize::MAX, 0);
            if !ab.is_empty() {
                moved = true;
                self.nodes[b]
                    .feed(link, &ab, 0)
                    .map_err(|r| provisioning(format!("setup link {a}-{b} died: {r:?}")))?;
            }
            let ba = self.nodes[b].poll_outbound(link, usize::MAX, 0);
            if !ba.is_empty() {
                moved = true;
                self.nodes[a]
                    .feed(link, &ba, 0)
                    .map_err(|r| provisioning(format!("setup link {a}-{b} died: {r:?}")))?;
            }
            if moved {
                idle = 0;
            } else {
                idle += 1;
                if idle >= 2 {
                    break;
                }
            }
        }
        self.nodes[a].link_down(link, 0);
        self.nodes[b].link_down(link, 0);
        self.drain(a);
        self.drain(b);
        Ok(())
    }

    fn schedule_initial(&mut self) {
        let n = self.nodes.len();
        for i in 0..n {
            self.push(self.tick_ms, Event::NodeTick(i));
        }
        for i in 0..n {
            if !self.nodes[i].is_locked_iphone() {
                let at = self.rng.random_range(0..DIAL_PERIOD_MS);
                self.push(at, Event::LinkAttempt(i));
            }
        }
        let period = self.scenario.traffic.group_post_every_ms;
        let groups: Vec<BTreeSet<usize>> = self
            .resolved
            .groups
            .iter()
            .map(|g| g.members.clone())
            .collect();
        for (g, members) in groups.iter().enumerate() {
            for &m in members {
                let at = self.rng.random_range(0..period.max(1));
                self.push(
                    at,
                    Event::Traffic {
                        node: m,
                        kind: TrafficKind::GroupPost(g),
                    },
                );
            }
        }
        let dm = self.scenario.traffic.dm_every_ms;
        if dm > 0 {
            let pairs = self.pairs.clone();
            for (p, (a, b)) in pairs.into_iter().enumerate() {
                let first = if self.rng.random_bool(0.5) { a } else { b };
                let at = self.rng.random_range(0..dm);
                self.push(
                    at,
                    Event::Traffic {
                        node: first,
                        kind: TrafficKind::Dm(p),
                    },
                );
            }
        }
        let events: Vec<(u64, usize)> = self
            .scenario
            .events
            .iter()
            .enumerate()
            .map(|(i, e)| (e.at_ms(), i))
            .collect();
        for (at, i) in events {
            self.push(at, Event::Scripted(i));
        }
        let snapshots: Vec<(u64, usize)> = self
            .scenario
            .pass
            .expected_vote_states
            .iter()
            .enumerate()
            .map(|(i, v)| (v.at_ms, i))
            .collect();
        for (at, i) in snapshots {
            self.push(at, Event::Snapshot(i));
        }
        let partitions: Vec<(u64, usize)> = self
            .resolved
            .partitions
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, p)| (p.at_ms, i))
            .collect();
        for (at, i) in partitions {
            self.push(at, Event::PartitionChange(i));
        }
        self.push(self.duration_ms, Event::End);
    }

    fn push(&mut self, at: u64, ev: Event) {
        if at > self.duration_ms && ev != Event::End {
            return;
        }
        let seq = self.seq;
        self.seq = self.seq.wrapping_add(1);
        self.queue.push(Reverse((at, seq, ev)));
    }

    pub fn now_ms(&self) -> u64 {
        self.now_ms
    }

    pub fn radio(&self) -> &Radio {
        &self.radio
    }

    pub fn nodes(&self) -> &[SimNode] {
        &self.nodes
    }

    #[cfg(test)]
    pub fn trace(&self) -> &Trace {
        &self.trace
    }

    pub fn group_id(&self, g: usize) -> Option<GroupId> {
        self.group_ids.get(g).copied().flatten()
    }

    /// Runs until the `End` event.
    pub fn run_to_end(&mut self) {
        while !self.ended {
            let Some(Reverse((at, _, ev))) = self.queue.pop() else {
                break;
            };
            self.now_ms = at;
            self.handle(ev);
        }
    }

    fn handle(&mut self, ev: Event) {
        match ev {
            Event::NodeTick(n) => self.node_tick(n),
            Event::LinkAttempt(n) => self.link_attempt(n),
            Event::LinkUp { a, b } => self.link_up(a, b),
            Event::LinkDown { link, reason } => self.link_down(link, reason),
            Event::Deliver { link, from, bytes } => self.deliver(link, from, &bytes),
            Event::Traffic { node, kind } => self.traffic(node, kind),
            Event::Scripted(i) => self.scenario_event(i),
            Event::Cast { node, vote, choice } => {
                let at = self.now_ms;
                self.ballots_cast
                    .entry(vote.clone())
                    .or_default()
                    .push((node, at));
                self.queue_pending(node, PendingKind::Ballot { vote, choice });
            }
            Event::Carry {
                node,
                fragment,
                vote,
                choice,
            } => self.carry(node, fragment, vote, choice),
            Event::Release(node) => self.release(node),
            Event::Replay(node) => self.replay(node),
            Event::Snapshot(i) => self.snapshot(i),
            Event::PartitionChange(i) => self.partition_change(i),
            Event::End => self.ended = true,
        }
    }

    // ---- node plumbing -----------------------------------------------------------

    /// Folds the node's pending events into the trace.
    fn drain(&mut self, n: usize) {
        let events = self.nodes[n].take_events();
        for (t, e) in &events {
            self.trace.on_event(n, *t, e, &self.nodes[n]);
        }
    }

    /// Pumps every live link of `n`.
    fn pump_node(&mut self, n: usize) {
        let links: Vec<u32> = self.links_of[n].iter().copied().collect();
        for link in links {
            self.pump(link, n);
        }
    }

    fn apply_outcome(&mut self, outcome: TickOutcome, node: usize) {
        if outcome.became_foreground {
            let at = self
                .now_ms
                .saturating_add(self.rng.random_range(0..DIAL_PERIOD_MS));
            self.push(at, Event::LinkAttempt(node));
        }
    }

    fn node_tick(&mut self, n: usize) {
        let now = self.now_ms;
        let outcome = self.nodes[n].tick(now);
        self.drain(n);
        if self.pending.iter().any(|p| p.node == n) {
            self.try_pending(n);
        }
        self.pump_node(n);
        self.apply_outcome(outcome, n);
        self.push(now.saturating_add(self.tick_ms), Event::NodeTick(n));
    }

    /// Whether `a` may dial `b` right now (DESIGN §7 `SimLink`: partition,
    /// slots, the locked-iPhone rule).
    fn may_link(&self, a: usize, b: usize) -> bool {
        self.radio.reachable(a, b)
            && !self.radio.is_linked(a, b)
            && self.nodes[a].can_dial()
            && self.nodes[b].accepting_inbound()
            && !(self.nodes[a].is_locked_iphone() && self.nodes[b].is_locked_iphone())
    }

    fn link_attempt(&mut self, n: usize) {
        let now = self.now_ms;
        if self.nodes[n].is_locked_iphone() {
            // Locked iPhones never initiate; the tick that makes them
            // foreground schedules the next attempt.
            return;
        }
        if self.nodes[n].can_dial() {
            let candidates: Vec<usize> = (0..self.nodes.len())
                .filter(|&m| self.may_link(n, m))
                .collect();
            if !candidates.is_empty() {
                let m = candidates[self.rng.random_range(0..candidates.len())];
                self.metrics.attempts += 1;
                self.push(
                    now.saturating_add(CONNECT_DELAY_MS_ASM),
                    Event::LinkUp { a: n, b: m },
                );
            }
        }
        let jitter = self
            .rng
            .random_range(DIAL_PERIOD_MS - DIAL_JITTER_MS..=DIAL_PERIOD_MS + DIAL_JITTER_MS);
        self.push(now.saturating_add(jitter), Event::LinkAttempt(n));
    }

    /// The contact hint for a link from `a` to `b`: the peer's identity
    /// when the two are a QR-verified pair (the platform knows who it
    /// connected to), so the node offers its L2 contact proof.
    fn hint_for(&self, a: usize, b: usize) -> Option<IdentityKey> {
        if self.pair_set.contains(&(a.min(b), a.max(b))) {
            self.ids[b]
        } else {
            None
        }
    }

    fn link_up(&mut self, a: usize, b: usize) {
        let now = self.now_ms;
        if !self.may_link(a, b) {
            self.metrics.attempts_failed += 1;
            return;
        }
        let wake = self.nodes[b].is_locked_iphone();
        let id = self.radio.open(a, b, now, wake);
        self.links_of[a].insert(id);
        self.links_of[b].insert(id);
        let (hint_a, hint_b) = (self.hint_for(a, b), self.hint_for(b, a));
        if let Err(reason) = self.nodes[a].link_up(id, Role::Central, hint_a, now) {
            self.link_down(id, reason);
            return;
        }
        if let Err(reason) = self.nodes[b].link_up(id, Role::Peripheral, hint_b, now) {
            self.link_down(id, reason);
            return;
        }
        self.metrics.established += 1;
        if let Some(deadline) = self.radio.link(id).and_then(|l| l.wake_deadline) {
            self.metrics.wake_links += 1;
            self.push(
                deadline,
                Event::LinkDown {
                    link: id,
                    reason: DropReason::WakeEnd,
                },
            );
        }
        self.pump(id, a);
    }

    fn link_down(&mut self, id: u32, reason: DropReason) {
        let now = self.now_ms;
        if let Some(link) = self.radio.close(id, now, reason) {
            self.nodes[link.a].link_down(id, now);
            self.nodes[link.b].link_down(id, now);
            self.drain(link.a);
            self.drain(link.b);
            self.links_of[link.a].remove(&id);
            self.links_of[link.b].remove(&id);
            *self.metrics.dropped_by_reason.entry(reason).or_insert(0) += 1;
        }
    }

    /// Moves what `side` queued on `link` onto the air as `Deliver` events
    /// and rolls the per-frame loss (DESIGN §7 "loss 0.5 %/frame → link
    /// drop"), once per [`FRAME_BYTES_ASM`] of each batch.
    fn pump(&mut self, link: u32, side: usize) {
        let now = self.now_ms;
        let loss = self.radio.params().frame_loss;
        loop {
            let bytes = self.nodes[side].poll_outbound(link, BATCH_BYTES, now);
            if bytes.is_empty() {
                break;
            }
            self.drain(side);
            let Some(arrive) = self.radio.transmit(link, side, bytes.len(), now) else {
                break;
            };
            self.metrics.frames_sent += 1;
            if loss > 0.0 {
                let frames = bytes.len().div_ceil(FRAME_BYTES_ASM).max(1);
                for _ in 0..frames {
                    if self.rng.random::<f64>() < loss
                        && let Some(at) = self.radio.doom(link, side, now)
                    {
                        self.push(
                            at,
                            Event::LinkDown {
                                link,
                                reason: DropReason::FrameLoss,
                            },
                        );
                    }
                }
            }
            self.push(
                arrive,
                Event::Deliver {
                    link,
                    from: side,
                    bytes,
                },
            );
        }
        self.drain(side);
    }

    fn deliver(&mut self, link: u32, from: usize, bytes: &[u8]) {
        let now = self.now_ms;
        let Some(l) = self.radio.link(link) else {
            return;
        };
        let to = l.peer_of(from);
        let chunk = l.chunk.max(1);
        // Fed in MTU−3 pieces, as the platform would (DESIGN §7 "chunk at
        // MTU−3"); the node's stream framer takes any boundary.
        for piece in bytes.chunks(chunk) {
            if let Err(reason) = self.nodes[to].feed(link, piece, now) {
                self.drain(to);
                self.link_down(link, reason);
                return;
            }
        }
        self.drain(to);
        self.pump(link, to);
        if self.radio.link(link).is_none() {
            return;
        }
        // The sender's queue drained: let it continue pushing.
        self.pump(link, from);
    }

    // ---- traffic -----------------------------------------------------------------

    fn jitter(&mut self, period_ms: u64) -> u64 {
        let lo = period_ms.saturating_mul(3) / 4;
        let hi = period_ms.saturating_mul(5) / 4;
        self.rng.random_range(lo.max(1)..=hi.max(lo.max(1)))
    }

    /// One text of the scenario's size range (lower-case words).
    fn random_text(&mut self) -> String {
        let [lo, hi] = self.scenario.traffic.text_bytes;
        let len: u32 = self.rng.random_range(lo..=hi);
        let len = usize::try_from(len).unwrap_or(0).max(1);
        let mut s = String::with_capacity(len);
        for i in 0..len {
            let c = if i > 0 && i % 7 == 6 {
                ' '
            } else {
                char::from(b'a' + self.rng.random_range(0..26u8))
            };
            s.push(c);
        }
        s
    }

    /// Nodes expected to show a post by `creator` in scenario group `g`
    /// made now: every other member, except a removed node whose removal
    /// the creator had already applied (the post is under a rotated chain
    /// it never received, D3 K2). A post by a removed node made after it
    /// applied its own removal carries that removal in its heads and is
    /// rejected everywhere (DECISIONS §3 row (1,2,3,4,10) (ii); DESIGN §10
    /// S3 (c)), so it is expected nowhere: only the `keep_keys_after_removal`
    /// adversary makes such a post, a stock client refuses to.
    fn expected_for_post(&self, g: usize, creator: usize, now: u64) -> BTreeSet<usize> {
        let mut out = BTreeSet::new();
        if self.removal_subjects.contains(&creator)
            && let Some(k) = self.ids[creator]
            && self
                .trace
                .removal_applied
                .get(&(creator, k))
                .is_some_and(|&t| t <= now)
        {
            return out;
        }
        for &m in &self.resolved.groups[g].members {
            if m == creator {
                continue;
            }
            if self.removal_subjects.contains(&m)
                && let Some(k) = self.ids[m]
                && self
                    .trace
                    .removal_applied
                    .get(&(creator, k))
                    .is_some_and(|&t| t <= now)
            {
                continue;
            }
            out.insert(m);
        }
        out
    }

    fn register(
        &mut self,
        obj: ObjId,
        creator: usize,
        kind: ObjKind,
        mut expected: BTreeSet<usize>,
        withheld: bool,
    ) {
        let now = self.now_ms;
        // The S-flood criteria are about honest traffic (docs/scenarios.md
        // "honest C0/C1 latency"): what the flooder itself sends is
        // throttled with its junk and is expected nowhere.
        if self.flood.is_some_and(|(f, _)| f == creator) {
            expected.clear();
        }
        let fragment = self.radio.fragment_of(creator);
        self.trace.registry.register(
            obj,
            creator,
            now,
            self.current_partition,
            fragment,
            kind,
            expected,
            withheld,
            false,
        );
        self.metrics.cells_created.total += 1;
        match kind {
            ObjKind::Post { .. } => self.metrics.cells_created.posts += 1,
            ObjKind::Dm { .. } => self.metrics.cells_created.dms += 1,
        }
    }

    fn post(&mut self, node: usize, g: usize) {
        let now = self.now_ms;
        let Some(gid) = self.group_id(g) else {
            return;
        };
        let text = self.random_text();
        match self.nodes[node].node_mut().send_text(gid, &text, now) {
            Ok(obj) => {
                let expected = self.expected_for_post(g, node, now);
                self.register(obj, node, ObjKind::Post { group: g }, expected, false);
            }
            Err(NodeError::UnknownGroup | NodeError::NotMember) => {
                self.nodes[node].posts_skipped += 1;
            }
            Err(e) => {
                self.nodes[node].posts_skipped += 1;
                self.failed("send_text", &e);
            }
        }
        self.drain(node);
        self.pump_node(node);
    }

    fn dm(&mut self, node: usize, peer: usize, withheld: bool, text: Option<String>) {
        let now = self.now_ms;
        let Some(peer_id) = self.ids[peer] else {
            return;
        };
        let text = text.unwrap_or_else(|| self.random_text());
        match self.nodes[node].node_mut().send_dm(peer_id, &text, now) {
            Ok(obj) => {
                self.register(
                    obj,
                    node,
                    ObjKind::Dm { to: peer },
                    BTreeSet::from([peer]),
                    withheld,
                );
            }
            Err(e) => self.failed("send_dm", &e),
        }
        self.drain(node);
        self.pump_node(node);
    }

    fn failed(&mut self, what: &str, e: &NodeError) {
        *self
            .metrics
            .events_failed
            .entry(format!("{what}: {e}"))
            .or_insert(0) += 1;
    }

    fn applied(&mut self, what: &str) {
        *self
            .metrics
            .events_applied
            .entry(what.to_owned())
            .or_insert(0) += 1;
    }

    fn traffic(&mut self, node: usize, kind: TrafficKind) {
        let now = self.now_ms;
        match kind {
            TrafficKind::GroupPost(g) => {
                self.post(node, g);
                let next =
                    now.saturating_add(self.jitter(self.scenario.traffic.group_post_every_ms));
                self.push(next, Event::Traffic { node, kind });
            }
            TrafficKind::Dm(p) => {
                let (a, b) = self.pairs[p];
                let other = if node == a { b } else { a };
                self.dm(node, other, false, None);
                let next = now.saturating_add(self.jitter(self.scenario.traffic.dm_every_ms));
                self.push(next, Event::Traffic { node: other, kind });
            }
            TrafficKind::Flood { per_batch } => {
                let n = usize::try_from(per_batch).unwrap_or(1);
                let ids = self.nodes[node].node_mut().adversary_inject_junk(n, now);
                self.metrics.cells_created.junk += u64::try_from(ids.len()).unwrap_or(u64::MAX);
                self.junk_ids.extend(ids);
                self.drain(node);
                self.pump_node(node);
                self.push(
                    now.saturating_add(FLOOD_PERIOD_MS),
                    Event::Traffic { node, kind },
                );
            }
        }
    }

    // ---- scenario events -----------------------------------------------------------

    /// The first scenario group `node` is a member of.
    fn group_of(&self, node: usize) -> Option<(usize, GroupId)> {
        self.resolved
            .groups
            .iter()
            .enumerate()
            .find(|(_, g)| g.members.contains(&node))
            .and_then(|(i, _)| self.group_ids[i].map(|gid| (i, gid)))
    }

    fn queue_pending(&mut self, node: usize, kind: PendingKind) {
        self.pending.push(Pending {
            node,
            since_ms: self.now_ms,
            kind,
        });
        self.try_pending(node);
    }

    /// Retries every pending op of `node`; ops whose task or vote has not
    /// arrived stay queued (module docs).
    fn try_pending(&mut self, node: usize) {
        let now = self.now_ms;
        let mut i = 0;
        while i < self.pending.len() {
            if self.pending[i].node != node {
                i += 1;
                continue;
            }
            let p = self.pending[i].clone();
            let result: Result<(), NodeError> = match &p.kind {
                PendingKind::Take(label)
                | PendingKind::Release(label)
                | PendingKind::Done(label) => match self.task_ids.get(label).copied() {
                    None => Err(NodeError::UnknownTask),
                    Some((_, tid)) => {
                        let gid = self.group_of(node).map(|(_, g)| g);
                        match gid {
                            None => Err(NodeError::UnknownGroup),
                            // The query first: a command on a task the node does
                            // not hold yet fails inside its transaction, and a
                            // failed transaction replays the node's whole state
                            // from the database (`Node::run_txn`), far too dear
                            // to repeat every tick.
                            Some(gid)
                                if !self.nodes[node]
                                    .node()
                                    .tasks(&gid)
                                    .is_ok_and(|t| t.contains(&tid)) =>
                            {
                                Err(NodeError::UnknownTask)
                            }
                            Some(gid) => {
                                let n = self.nodes[node].node_mut();
                                match &p.kind {
                                    PendingKind::Take(_) => n.take_task(gid, tid, now),
                                    PendingKind::Release(_) => n.release_task(gid, tid, now),
                                    _ => n.done_task(gid, tid, now),
                                }
                            }
                        }
                    }
                },
                PendingKind::Ballot { vote, choice } => match self.vote_ids.get(vote).copied() {
                    None => Err(NodeError::UnknownVote),
                    Some(vid) if !self.nodes[node].node().votes().contains(&vid) => {
                        Err(NodeError::UnknownVote)
                    }
                    Some(vid) => {
                        let choice = u8::try_from(*choice).unwrap_or(u8::MAX);
                        self.nodes[node].node_mut().cast_ballot(vid, choice, now)
                    }
                },
            };
            match result {
                Ok(()) => {
                    let name = if p.since_ms == now {
                        p.name().to_owned()
                    } else {
                        format!("{} (retried, {} ms late)", p.name(), now - p.since_ms)
                    };
                    self.applied(&name);
                    self.pending.remove(i);
                }
                Err(NodeError::UnknownTask | NodeError::UnknownVote) => i += 1,
                Err(e) => {
                    self.failed(p.name(), &e);
                    self.pending.remove(i);
                }
            }
        }
        self.drain(node);
        self.pump_node(node);
    }

    fn scenario_event(&mut self, i: usize) {
        let now = self.now_ms;
        let Some(ev) = self.scenario.events.get(i).cloned() else {
            return;
        };
        match ev {
            scenario::Event::RemoveMember { by, subject, .. } => self.remove_member(by, subject),
            scenario::Event::AddMember { by, subject, .. } => self.add_member(by, subject),
            scenario::Event::TaskCreate { by, task, .. } => {
                let Some((g, gid)) = self.group_of(by) else {
                    self.failed("task_create", &NodeError::UnknownGroup);
                    return;
                };
                match self.nodes[by].node_mut().create_task(gid, &task, "", now) {
                    Ok(id) => {
                        self.task_ids.insert(task, (g, id));
                        self.applied("task_create");
                    }
                    Err(e) => self.failed("task_create", &e),
                }
                self.drain(by);
                self.pump_node(by);
            }
            scenario::Event::TaskTake { by, task, .. } => {
                self.queue_pending(by, PendingKind::Take(task));
            }
            scenario::Event::TaskRelease { by, task, .. } => {
                self.queue_pending(by, PendingKind::Release(task));
            }
            scenario::Event::TaskDone { by, task, .. } => {
                self.queue_pending(by, PendingKind::Done(task));
            }
            scenario::Event::VoteOpen {
                by,
                label,
                question,
                options,
                f,
                ..
            } => {
                let Some((_, gid)) = self.group_of(by) else {
                    self.failed("vote_open", &NodeError::UnknownGroup);
                    return;
                };
                let opts: Vec<&str> = options.iter().map(String::as_str).collect();
                let f = u8::try_from(f).unwrap_or(u8::MAX);
                match self.nodes[by]
                    .node_mut()
                    .open_vote(gid, &question, &opts, f, now)
                {
                    Ok(id) => {
                        self.vote_ids.insert(label, id);
                        self.applied("vote_open");
                    }
                    Err(e) => self.failed("vote_open", &e),
                }
                self.drain(by);
                self.pump_node(by);
            }
            scenario::Event::Ballot {
                by,
                vote,
                choice,
                spread_ms,
                ..
            } => {
                for node in by.to_vec() {
                    let at = now.saturating_add(self.rng.random_range(0..=spread_ms));
                    self.push(
                        at,
                        Event::Cast {
                            node,
                            vote: vote.clone(),
                            choice,
                        },
                    );
                }
            }
            scenario::Event::Adversary {
                node,
                behaviour,
                count,
                hold_ms,
                cells_per_min,
                vote,
                choices,
                carry_to_fragment,
                carry_at_ms,
                ..
            } => self.adversary(
                node,
                behaviour,
                count,
                hold_ms,
                cells_per_min,
                vote,
                choices,
                carry_to_fragment,
                carry_at_ms,
            ),
        }
    }

    fn remove_member(&mut self, by: usize, subject: usize) {
        let now = self.now_ms;
        let Some(key) = self.ids[subject] else {
            self.failed("remove_member", &NodeError::NoIdentity);
            return;
        };
        self.trace.tracked_keys.insert(key);
        let groups: Vec<(usize, GroupId)> = self
            .resolved
            .groups
            .iter()
            .enumerate()
            .filter(|(_, g)| g.admins.contains(&by) && g.members.contains(&subject))
            .filter_map(|(i, _)| self.group_ids[i].map(|gid| (i, gid)))
            .collect();
        if groups.is_empty() {
            // A scenario naming a non-admin issuer must not pass by doing
            // nothing (the node would refuse the command, `NotAdmin`).
            self.failed("remove_member", &NodeError::NotAdmin);
        }
        for (_, gid) in groups {
            for n in 0..self.nodes.len() {
                self.trace.seed_membership(n, &gid, &self.nodes[n]);
            }
            match self.nodes[by].node_mut().remove_member(gid, key, now) {
                // The op may be held but void: D3 "Remove(m): m ∉ admins(P),
                // demote first" (`roster::eval`), which a scenario that
                // removes an admin runs into.
                Ok(())
                    if self.nodes[by]
                        .node()
                        .roster(&gid)
                        .is_ok_and(|v| v.is_member(&key)) =>
                {
                    self.failed(
                        "remove_member",
                        &NodeError::Invalid(
                            "void: the subject is still a member at the issuer (an admin must be demoted first, D3)",
                        ),
                    );
                }
                Ok(()) => self.applied("remove_member"),
                Err(e) => self.failed("remove_member", &e),
            }
        }
        self.drain(by);
        self.pump_node(by);
    }

    /// An admin other than `by` in the group, preferably one it can reach
    /// now (the endorsement is given in person).
    fn endorser_for(&self, g: usize, by: usize) -> Option<usize> {
        let admins = &self.resolved.groups[g].admins;
        admins
            .iter()
            .copied()
            .find(|&a| a != by && self.radio.reachable(by, a))
            .or_else(|| admins.iter().copied().find(|&a| a != by))
    }

    fn add_member(&mut self, by: usize, subject: usize) {
        let Some((g, gid)) = self.group_of(by) else {
            self.failed("add_member", &NodeError::UnknownGroup);
            return;
        };
        let card = match self.nodes[subject].node().my_keycard() {
            Ok(c) => c,
            Err(e) => return self.failed("add_member", &e),
        };
        let entry = MemberEntry {
            keycard: card,
            name: self.nodes[subject].name().to_owned(),
            in_person: true,
        };
        self.add_entry(by, g, gid, entry, "add_member");
    }

    fn add_entry(&mut self, by: usize, g: usize, gid: GroupId, entry: MemberEntry, what: &str) {
        let now = self.now_ms;
        let Some(endorser) = self.endorser_for(g, by) else {
            self.failed(what, &NodeError::NotAdmin);
            return;
        };
        let draft = match self.nodes[by].node().propose_add(gid, entry.clone()) {
            Ok(d) => d,
            Err(e) => return self.failed(what, &e),
        };
        let endorsement = match self.nodes[endorser].node().endorse(&draft, true) {
            Ok(e) => e,
            Err(e) => return self.failed(what, &e),
        };
        match self.nodes[by]
            .node_mut()
            .add_member(gid, entry, endorsement, now)
        {
            Ok(()) => self.applied(what),
            Err(e) => self.failed(what, &e),
        }
        self.drain(by);
        self.pump_node(by);
    }

    #[allow(clippy::too_many_arguments)]
    fn adversary(
        &mut self,
        node: usize,
        behaviour: Behaviour,
        count: Option<u32>,
        hold_ms: Option<u64>,
        cells_per_min: Option<u32>,
        vote: Option<String>,
        choices: Option<Vec<usize>>,
        carry_to_fragment: Option<usize>,
        carry_at_ms: Option<u64>,
    ) {
        let now = self.now_ms;
        match behaviour {
            Behaviour::KeepKeysAfterRemoval => {
                self.nodes[node].node_mut().set_adversary_keep_keys(true);
                self.adversaries.insert(node);
                self.applied("adversary:keep_keys_after_removal");
            }
            Behaviour::EquivocateBallot => {
                let (Some(vote), Some(choices), Some(frag), Some(at)) =
                    (vote, choices, carry_to_fragment, carry_at_ms)
                else {
                    return;
                };
                let (first, second) = (
                    choices.first().copied().unwrap_or(0),
                    choices.get(1).copied().unwrap_or(0),
                );
                self.equivocators.insert(node);
                self.queue_pending(
                    node,
                    PendingKind::Ballot {
                        vote: vote.clone(),
                        choice: first,
                    },
                );
                self.push(
                    at.max(now),
                    Event::Carry {
                        node,
                        fragment: frag,
                        vote,
                        choice: second,
                    },
                );
                self.applied("adversary:equivocate_ballot");
            }
            Behaviour::SockpuppetAdds => {
                let Some((g, gid)) = self.group_of(node) else {
                    self.failed("adversary:sockpuppet_adds", &NodeError::UnknownGroup);
                    return;
                };
                let count = usize::try_from(count.unwrap_or(0)).unwrap_or(0);
                for k in 0..count {
                    let mut seed = [0u8; 32];
                    self.rng.fill_bytes(&mut seed);
                    let mut x25519 = [0u8; 32];
                    self.rng.fill_bytes(&mut x25519);
                    let signer = IdentitySigner::from_seed(&seed);
                    let card = KeyCard::issue(&signer, x25519);
                    let key = card.ed;
                    self.puppets.push(key);
                    self.trace.tracked_keys.insert(key);
                    let entry = MemberEntry {
                        keycard: card,
                        name: format!("Puppet {k}"),
                        in_person: true,
                    };
                    self.add_entry(node, g, gid, entry, "adversary:sockpuppet_adds");
                    for n in 0..self.nodes.len() {
                        self.trace.seed_membership(n, &gid, &self.nodes[n]);
                    }
                }
            }
            Behaviour::WithholdDms => {
                let Some(hold) = hold_ms else {
                    return;
                };
                let peers: Vec<usize> = self
                    .pairs
                    .iter()
                    .filter_map(|&(a, b)| {
                        if a == node {
                            Some(b)
                        } else if b == node {
                            Some(a)
                        } else {
                            None
                        }
                    })
                    .collect();
                if peers.is_empty() {
                    self.failed("adversary:withhold_dms", &NodeError::Invalid("no pairs"));
                    return;
                }
                let count = count.unwrap_or(WITHHELD_DMS_DEFAULT);
                let mut composed = Vec::new();
                for k in 0..count {
                    let peer = peers[usize::try_from(k).unwrap_or(0) % peers.len()];
                    let text = self.random_text();
                    composed.push((peer, text));
                }
                self.withheld.insert(node, composed);
                self.push(now.saturating_add(hold), Event::Release(node));
                self.applied("adversary:withhold_dms");
            }
            Behaviour::ReplayCells => {
                self.push(now, Event::Replay(node));
                self.applied("adversary:replay_cells");
            }
            Behaviour::Flood => {
                let Some(cpm) = cells_per_min else {
                    return;
                };
                let per_batch = cpm.div_ceil(60).max(1);
                self.nodes[node].set_flooding(true);
                self.flood = Some((node, now));
                self.push(
                    now,
                    Event::Traffic {
                        node,
                        kind: TrafficKind::Flood { per_batch },
                    },
                );
                self.applied("adversary:flood");
            }
        }
    }

    /// The courier crosses into `fragment` and casts its other ballot.
    fn carry(&mut self, node: usize, fragment: usize, vote: String, choice: usize) {
        self.fragment_override.insert(node, fragment);
        self.apply_partition(self.current_partition);
        self.applied("adversary:equivocate_ballot carry");
        self.queue_pending(node, PendingKind::Ballot { vote, choice });
    }

    fn release(&mut self, node: usize) {
        let Some(list) = self.withheld.remove(&node) else {
            return;
        };
        for (peer, text) in list {
            self.dm(node, peer, true, Some(text));
        }
        self.applied("adversary:withhold_dms release");
    }

    fn replay(&mut self, node: usize) {
        let now = self.now_ms;
        let offers = self.nodes[node]
            .node_mut()
            .adversary_replay_held(REPLAY_BATCH);
        self.replay_offers += u64::try_from(offers).unwrap_or(u64::MAX);
        self.drain(node);
        self.pump_node(node);
        self.push(now.saturating_add(REPLAY_PERIOD_MS), Event::Replay(node));
    }

    fn snapshot(&mut self, i: usize) {
        let Some(v) = self.scenario.pass.expected_vote_states.get(i) else {
            return;
        };
        let lines: Vec<String> = match self.vote_ids.get(&v.vote) {
            None => vec!["(vote never opened)".to_owned(); self.nodes.len()],
            Some(id) => self
                .nodes
                .iter()
                .map(|n| match n.node().vote(id) {
                    Ok((_, line)) => line,
                    Err(_) => "(vote not held)".to_owned(),
                })
                .collect(),
        };
        self.vote_snapshots.push(VoteSnapshot {
            index: i,
            at_ms: self.now_ms,
            lines,
        });
    }

    /// Applies partition `i` with the courier overrides on top.
    fn apply_partition(&mut self, i: usize) {
        let Some(p) = self.resolved.partitions.get(i).cloned() else {
            return;
        };
        let mut p: ResolvedPartition = p;
        if p.fragment_count > 1 {
            for (&n, &f) in &self.fragment_override {
                if let Some(slot) = p.fragment_of.get_mut(n)
                    && f < p.fragment_count
                {
                    *slot = f;
                }
            }
        }
        let cut = self.radio.set_partition(&p);
        for id in cut {
            self.link_down(id, DropReason::Partition);
        }
    }

    fn partition_change(&mut self, i: usize) {
        let now = self.now_ms;
        let before = self.radio.fragment_count();
        self.current_partition = i;
        self.apply_partition(i);
        let after = self.radio.fragment_count();
        if after == 1 && before > 1 {
            self.rejoin_at = Some(now);
        }
        if after > 1 && before == 1 {
            self.split_at = Some(now);
        }
    }

    // ---- report --------------------------------------------------------------------

    /// The inbound-budget scores honest nodes give their live links to the
    /// flooder (`Node::link_score`; 1.0 is clean).
    fn flooder_link_scores(&self, flooder: usize) -> Vec<f32> {
        let now = self.now_ms;
        self.radio
            .links()
            .filter(|l| l.a == flooder || l.b == flooder)
            .filter_map(|l| {
                let peer = l.peer_of(flooder);
                self.nodes[peer].node().link_score(LinkId(l.id), now)
            })
            .collect()
    }

    /// Evaluates the pass criteria and builds the report.
    pub fn report(&self) -> Report {
        let flood = self.flood.map(|(flooder, start_ms)| FloodInfo {
            flooder,
            start_ms,
            junk_ids: &self.junk_ids,
            baseline_latency_ms: self.baseline_latency_ms,
            link_scores: self.flooder_link_scores(flooder),
        });
        let pending: BTreeMap<String, u64> =
            self.pending.iter().fold(BTreeMap::new(), |mut m, p| {
                *m.entry(format!("{} by node {}", p.name(), p.node))
                    .or_insert(0) += 1;
                m
            });
        let input = EvalInput {
            scenario: &self.scenario,
            resolved: &self.resolved,
            nodes: &self.nodes,
            trace: &self.trace,
            rejoin_at: self.rejoin_at,
            split_at: self.split_at,
            duration_ms: self.duration_ms,
            group_ids: &self.group_ids,
            vote_ids: &self.vote_ids,
            vote_snapshots: &self.vote_snapshots,
            ids: &self.ids,
            adversaries: &self.adversaries,
            removal_subjects: &self.removal_subjects,
            puppets: &self.puppets,
            replay_offers: self.replay_offers,
            flood,
            moved: &self.fragment_override.keys().copied().collect(),
            ballots_cast: &self.ballots_cast,
            equivocators: &self.equivocators,
        };
        let eval = assert::evaluate(&input);
        let wall_clock_reads: u64 = self.nodes.iter().map(SimNode::wall_reads).sum();
        let mut stats = NodeStats::default();
        for n in self.nodes() {
            stats.add(&n.stats());
        }
        // Every node's clock is brought to the end of the run first: the
        // display offset is read as wall − mono, and a node's clock only
        // moves when the engine calls into it.
        for n in &self.nodes {
            n.clock.set(self.now_ms);
        }
        let reg = &self.trace.registry;
        let nodes = self
            .nodes
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let s = n.stats();
                let shown = |want_post: bool| -> u64 {
                    reg.records
                        .iter()
                        .filter(|r| {
                            !r.setup
                                && r.creator != i
                                && r.shown.get(i).is_some_and(|&t| t != u64::MAX)
                                && matches!(r.kind, ObjKind::Post { .. }) == want_post
                        })
                        .count() as u64
                };
                NodeReport {
                    idx: n.idx(),
                    name: n.name().to_owned(),
                    kind: n.kind().name(),
                    battery_pct: (n.battery_ppm() / 1_000) as f64 / 10.0,
                    foreground: n.is_foreground(),
                    cells_held: n.store_len(),
                    links_up: n.link_count(),
                    wall_offset_ms: n.clock.display_hook_wall()
                        - i64::try_from(self.now_ms()).unwrap_or(0),
                    posts_shown: shown(true),
                    dms_shown: shown(false),
                    posts_skipped: n.posts_skipped,
                    decrypted: s.decrypted,
                    decrypt_failures: s.decrypt_failures,
                    rejected_posts: s.rejected_posts,
                    duplicates: s.duplicates,
                    parked: s.parked,
                    commits: s.commits,
                    rollbacks: s.rollbacks,
                    penalties: s.penalties,
                    kv_rows: n.node().kv_len().unwrap_or(0),
                    wall_reads: n.wall_reads(),
                    // last: the report's own display read is taken after
                    // the protocol count above
                    last_sync_set: n.last_sync_set(self.now_ms),
                }
            })
            .collect();
        let (max_link_bytes, p95) = self.radio().bottleneck_bytes();
        let live_at_end = u64::try_from(self.radio().links().count()).unwrap_or(u64::MAX);
        let dropped_by_reason: BTreeMap<String, u64> = self
            .metrics
            .dropped_by_reason
            .iter()
            .map(|(r, n)| (r.name().to_string(), *n))
            .collect();
        let vote_snapshots = self
            .vote_snapshots
            .iter()
            .filter_map(|s| {
                let v = self.scenario.pass.expected_vote_states.get(s.index)?;
                let mut lines: BTreeMap<String, u64> = BTreeMap::new();
                for l in &s.lines {
                    *lines.entry(l.clone()).or_insert(0) += 1;
                }
                Some(VoteSnapshotReport {
                    vote: v.vote.clone(),
                    at_ms: s.at_ms,
                    expected: assert::expected_vote_line(v),
                    lines,
                })
            })
            .collect();
        let mut cells_created = self.metrics.cells_created.clone();
        cells_created.delivered_at_sender = self.trace.delivered_at_sender();
        let mut report = Report {
            scenario: self.scenario.scenario.name.clone(),
            seed: self.seed,
            duration_ms: self.duration_ms,
            summary: String::new(),
            nodes,
            cells_created,
            delivery: eval.delivery,
            drain_ms: eval.drain_ms,
            rejoin_at_ms: self.rejoin_at,
            split_at_ms: self.split_at,
            bytes_on_links: BytesOnLinks {
                max_link_bytes,
                p95,
                pairs: u64::try_from(self.radio.pair_bytes.len()).unwrap_or(u64::MAX),
            },
            links: Links {
                established: self.metrics.established,
                live_at_end,
                dropped: dropped_by_reason.values().sum(),
                handshakes: stats.handshakes / 2,
                wake_links: self.metrics.wake_links,
                attempts: self.metrics.attempts,
                attempts_failed: self.metrics.attempts_failed,
                dropped_by_reason,
                l2_links: self.trace.l2_links,
            },
            cells: CellTraffic {
                frames_sent: self.metrics.frames_sent,
                cells_sent: stats.cells_sent,
                cells_received: stats.cells_received,
                duplicates: stats.duplicates,
                malformed: stats.malformed,
                throttled_in: stats.throttled_in,
                rejected_in: stats.rejected_in,
                no_room: stats.no_room,
                decrypted: stats.decrypted,
                decrypt_failures: stats.decrypt_failures,
                rejected_posts: stats.rejected_posts,
                parked: stats.parked,
                commits: stats.commits,
                rollbacks: stats.rollbacks,
                penalties: stats.penalties,
                replay_offers: self.replay_offers,
            },
            wall_clock_reads,
            events_applied: self.metrics.events_applied.clone(),
            events_failed: self.metrics.events_failed.clone(),
            events_pending: pending,
            vote_snapshots,
            honest_latency_ms: eval.honest_latency_ms,
            baseline_latency_ms: self.baseline_latency_ms,
            criteria: eval.criteria,
            host_seconds: 0.0,
        };
        report.summary = report.summary_line();
        report
    }
}

/// A validated fraction in [0, 1] as parts per million (exact enough for
/// a configuration value; the sim never compares floats afterwards).
fn fraction_ppm(f: f64) -> u64 {
    (f.clamp(0.0, 1.0) * 1_000_000.0).round() as u64
}

/// Runs `scenario` for `seed` to the end and returns the finished sim.
pub fn run_sim(
    scenario: &Scenario,
    seed: u64,
    duration_override_ms: Option<u64>,
) -> Result<Sim, ScenarioError> {
    let mut sim = Sim::new(scenario, seed, duration_override_ms)?;
    sim.run_to_end();
    Ok(sim)
}

/// The scenario without its `flood` events: the baseline of the S-flood
/// latency criterion (docs/scenarios.md `flood_honest_latency_factor_max`:
/// "honest C0/C1 latency ≤ factor × baseline").
fn without_flood(scenario: &Scenario) -> Scenario {
    let mut s = scenario.clone();
    s.events.retain(|e| {
        !matches!(
            e,
            scenario::Event::Adversary {
                behaviour: Behaviour::Flood,
                ..
            }
        )
    });
    s.pass.flood_honest_latency_factor_max = 0.0;
    s.pass.flood_junk_within_l3_cap = false;
    s
}

/// Runs and reports. When the scenario checks the flood latency factor,
/// the same seed is first run without the flooder to measure the baseline.
pub fn run(
    scenario: &Scenario,
    seed: u64,
    duration_override_ms: Option<u64>,
) -> Result<Report, ScenarioError> {
    let has_flood = scenario.events.iter().any(|e| {
        matches!(
            e,
            scenario::Event::Adversary {
                behaviour: Behaviour::Flood,
                ..
            }
        )
    });
    let baseline = if has_flood && scenario.pass.flood_honest_latency_factor_max > 0.0 {
        let base = run_sim(&without_flood(scenario), seed, duration_override_ms)?;
        let start = base.flood_start_hint(scenario);
        assert::honest_latency_ms(&base.trace.registry, start, base.duration_ms, None)
    } else {
        None
    };
    let mut sim = Sim::new(scenario, seed, duration_override_ms)?;
    sim.baseline_latency_ms = baseline;
    sim.run_to_end();
    Ok(sim.report())
}

impl Sim {
    /// The flood start of `scenario` (for the baseline run, which has none).
    fn flood_start_hint(&self, scenario: &Scenario) -> u64 {
        scenario
            .events
            .iter()
            .find_map(|e| match e {
                scenario::Event::Adversary {
                    behaviour: Behaviour::Flood,
                    at_ms,
                    ..
                } => Some(*at_ms),
                _ => None,
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::parse;

    pub(crate) fn toml(nodes: &str, radio_extra: &str, rest: &str) -> String {
        format!(
            r#"
[scenario]
name = "t"
duration_ms = 180_000
tick_ms = 100
[nodes]
{nodes}
iphone_foreground_duty = 1.0
android_slots = 7
battery_floor = 0.30
[radio]
latency_ms = 40
throughput_bytes_per_s = 15_000
iphone_wake_ms = 10_000
wall_clock_offset_ms = [0, 0]
{radio_extra}
[groups]
[[groups.list]]
name = "main"
members = "all"
admins = [0]
[traffic]
group_post_every_ms = 20_000
dm_every_ms = 0
text_bytes = [40, 300]
{rest}
"#
        )
    }

    #[test]
    fn event_order_is_by_time_then_sequence() {
        let scn = parse(&toml("android = 2\niphone = 0", "frame_loss = 0.0", "")).unwrap();
        let mut sim = Sim::new(&scn, 1, Some(1_000)).unwrap();
        sim.queue.clear();
        sim.push(50, Event::End);
        sim.push(10, Event::NodeTick(1));
        sim.push(10, Event::NodeTick(0));
        let Reverse((t, _, e)) = sim.queue.pop().unwrap();
        assert_eq!(
            (t, e),
            (10, Event::NodeTick(1)),
            "same time: first pushed first"
        );
        let Reverse((t, _, e)) = sim.queue.pop().unwrap();
        assert_eq!((t, e), (10, Event::NodeTick(0)));
        // events after the end are dropped
        sim.push(2_000, Event::NodeTick(0));
        assert_eq!(sim.queue.len(), 1);
    }

    #[test]
    fn two_nodes_exchange_posts_and_the_report_passes() {
        let scn = parse(&toml(
            "android = 2\niphone = 0",
            "frame_loss = 0.0",
            "[pass]\nevery_post_decrypts_after_rejoin = true\nno_duplicate_ui_delivery = true\nzero_wall_clock_reads = true\nmembers_h_identical = true\nno_solid_state_reverts = true",
        ))
        .unwrap();
        let sim = run_sim(&scn, 3, None).unwrap();
        let r = sim.report();
        assert!(r.passed(), "{}\n{}", r.summary_line(), r.to_json());
        assert!(r.cells_created.posts >= 10, "{}", r.summary_line());
        assert_eq!(r.delivery.ratio, 1.0);
        assert_eq!(r.links.established, 1);
        assert!(r.links.handshakes >= 1, "setup links count too");
        assert_eq!(r.wall_clock_reads, 0);
        assert!(r.bytes_on_links.max_link_bytes > 0);
        assert!(r.cells.decrypted > 0);
        // every member holds the same roster
        let gid = sim.group_id(0).unwrap();
        let a = sim.nodes()[0].node().members(&gid).unwrap();
        let b = sim.nodes()[1].node().members(&gid).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn same_seed_same_report() {
        let scn = parse(&toml("android = 4\niphone = 1", "frame_loss = 0.01", "")).unwrap();
        let a = run(&scn, 11, Some(60_000)).unwrap().deterministic_json();
        let b = run(&scn, 11, Some(60_000)).unwrap().deterministic_json();
        assert_eq!(a, b);
        let c = run(&scn, 12, Some(60_000)).unwrap().deterministic_json();
        assert_ne!(a, c, "another seed draws differently");
    }

    #[test]
    fn frame_loss_drops_links_and_they_re_handshake() {
        let scn = parse(&toml("android = 3\niphone = 0", "frame_loss = 0.05", "")).unwrap();
        let sim = run_sim(&scn, 5, None).unwrap();
        let r = sim.report();
        assert!(
            r.links
                .dropped_by_reason
                .get("frame_loss")
                .copied()
                .unwrap_or(0)
                > 0
        );
        assert!(r.links.established > 3);
        assert!(r.links.handshakes > 3);
    }

    #[test]
    fn flood_event_injects_junk_and_a_task_create_applies() {
        let scn = parse(&toml(
            "android = 2\niphone = 0",
            "frame_loss = 0.0",
            "[[events]]\nat_ms = 1000\nkind = \"adversary\"\nnode = 1\nbehaviour = \"flood\"\ncells_per_min = 600\n[[events]]\nat_ms = 2000\nkind = \"task_create\"\nby = 0\ntask = \"w\"",
        ))
        .unwrap();
        let r = run(&scn, 2, Some(60_000)).unwrap();
        assert!(r.cells_created.junk >= 580, "{}", r.cells_created.junk);
        assert_eq!(r.events_applied.get("task_create"), Some(&1));
        // docs/scenarios.md S-flood "injecting 600 junk cells/min": the
        // flooder ignores its own pacing, so the honest node's inbound
        // budget is what contains it (BRIEF_v2 §5.5): it throttles.
        assert!(r.cells.throttled_in > 100, "{}", r.cells.throttled_in);
        assert!(r.cells.cells_received > 100, "{}", r.summary_line());
    }

    #[test]
    fn reported_wall_offsets_are_the_drawn_offsets() {
        // The per-node wall-clock offset is display only (scenario S4,
        // DESIGN §10 S4 (e)); the report must give the offset itself, not
        // the offset minus however stale that node's clock happened to be.
        let scn = parse(
            &toml("android = 3\niphone = 1", "frame_loss = 0.0", "").replace(
                "wall_clock_offset_ms = [0, 0]",
                "wall_clock_offset_ms = [-5000, -5000]",
            ),
        )
        .unwrap();
        let r = run(&scn, 7, Some(30_050)).unwrap();
        for n in &r.nodes {
            assert_eq!(n.wall_offset_ms, -5000, "node {}: {}", n.idx, r.to_json());
        }
        assert_eq!(r.wall_clock_reads, 0);
        let scn = parse(
            &toml("android = 3\niphone = 1", "frame_loss = 0.0", "").replace(
                "wall_clock_offset_ms = [0, 0]",
                "wall_clock_offset_ms = [-1_200_000, 1_200_000]",
            ),
        )
        .unwrap();
        let r = run(&scn, 7, Some(30_050)).unwrap();
        assert!(
            r.nodes
                .iter()
                .all(|n| (-1_200_000..=1_200_000).contains(&n.wall_offset_ms))
        );
        assert!(
            r.nodes.iter().any(|n| n.wall_offset_ms != 0),
            "offsets are drawn per node"
        );
    }

    #[test]
    fn a_removal_rotates_and_the_removed_node_is_locked_out() {
        let scn = parse(&toml(
            "android = 3\niphone = 0",
            "frame_loss = 0.0",
            "[[events]]\nat_ms = 60_000\nkind = \"remove_member\"\nby = 0\nsubject = 2\n[pass]\nevery_post_decrypts_after_rejoin = true\nremoved_cannot_decrypt_post_removal = true\nremoved_posts_labelled_or_rejected = true\nmembers_h_identical = true",
        ))
        .unwrap();
        let sim = run_sim(&scn, 4, None).unwrap();
        let r = sim.report();
        assert!(r.passed(), "{}\n{}", r.summary_line(), r.to_json());
        let gid = sim.group_id(0).unwrap();
        for n in 0..3 {
            assert_eq!(
                sim.nodes()[n].node().members(&gid).unwrap().len(),
                2,
                "node {n} applied the removal"
            );
        }
        assert!(
            sim.nodes()[2].posts_skipped > 0,
            "a stock client refuses to post"
        );
        let key = sim.nodes()[2].identity().unwrap();
        assert!(sim.trace().removal_applied.contains_key(&(0, key)));
        assert!(sim.trace().removal_applied.contains_key(&(2, key)));
    }

    #[test]
    fn an_insider_that_keeps_keys_has_its_citing_posts_hidden_and_the_rest_labelled() {
        // DESIGN §10 S3 (c) / DECISIONS §3 row (1,2,3,4,10) (ii): a post by
        // the removed member whose own heads include its removal is rejected
        // everywhere, every other is kept and labelled. The registry must
        // expect the citing posts nowhere, or S3 (e) contradicts S3 (c).
        let scn = parse(&toml(
            "android = 3\niphone = 0",
            "frame_loss = 0.0",
            "[[events]]\nat_ms = 60_000\nkind = \"remove_member\"\nby = 0\nsubject = 2\n[[events]]\nat_ms = 60_000\nkind = \"adversary\"\nnode = 2\nbehaviour = \"keep_keys_after_removal\"\n[pass]\nevery_post_decrypts_after_rejoin = true\nremoved_cannot_decrypt_post_removal = true\nremoved_posts_labelled_or_rejected = true\nmembers_h_identical = true",
        ))
        .unwrap();
        let sim = run_sim(&scn, 4, None).unwrap();
        let r = sim.report();
        assert!(r.passed(), "{}\n{}", r.summary_line(), r.to_json());
        let key = sim.nodes()[2].identity().unwrap();
        let applied_at = sim.trace().removal_applied[&(2, key)];
        let citing: Vec<_> = sim
            .trace()
            .registry
            .records
            .iter()
            .filter(|rec| rec.creator == 2 && !rec.setup && rec.created_ms >= applied_at)
            .collect();
        assert!(
            !citing.is_empty(),
            "the insider kept posting after its removal"
        );
        for rec in &citing {
            assert!(
                rec.expected.is_empty(),
                "a citing post is expected nowhere: {rec:?}"
            );
            assert!(
                rec.shown
                    .iter()
                    .enumerate()
                    .all(|(n, &t)| n == 2 || t == u64::MAX),
                "a citing post was shown at an honest node: {rec:?}"
            );
        }
        assert!(
            r.cells.rejected_posts > 0,
            "honest nodes rejected the citing posts: {}",
            r.to_json()
        );
        let c = r
            .criteria
            .iter()
            .find(|c| c.name == "removed_posts_labelled_or_rejected")
            .unwrap();
        assert_eq!(c.status, crate::assert::Status::Pass, "{}", c.detail);
        assert!(
            !c.detail.starts_with("no post by a removed node"),
            "{}",
            c.detail
        );
    }

    #[test]
    fn a_removal_by_a_non_admin_is_a_recorded_failure_not_a_silent_no_op() {
        let scn = parse(&toml(
            "android = 3\niphone = 0",
            "frame_loss = 0.0",
            "[[events]]\nat_ms = 60_000\nkind = \"remove_member\"\nby = 1\nsubject = 2\n[pass]\nremoved_posts_labelled_or_rejected = true",
        ))
        .unwrap();
        let r = run(&scn, 4, Some(90_000)).unwrap();
        assert!(
            r.events_failed
                .keys()
                .any(|k| k.starts_with("remove_member")),
            "{:?}",
            r.events_failed
        );
        assert!(r.events_applied.is_empty(), "{:?}", r.events_applied);
        let c = r
            .criteria
            .iter()
            .find(|c| c.name == "removed_posts_labelled_or_rejected")
            .unwrap();
        assert_eq!(
            c.status,
            crate::assert::Status::NotEvaluated,
            "{}",
            c.detail
        );
    }

    #[test]
    fn a_vote_reaches_final_and_a_task_is_taken() {
        let scn = parse(&toml(
            "android = 3\niphone = 0",
            "frame_loss = 0.0",
            "[[events]]\nat_ms = 20_000\nkind = \"vote_open\"\nby = 0\nlabel = \"v\"\nquestion = \"q\"\noptions = [\"Yes\",\"No\"]\nf = 1\n[[events]]\nat_ms = 40_000\nkind = \"ballot\"\nby = [0, 1, 2]\nvote = \"v\"\nchoice = 0\nspread_ms = 10_000\n[[events]]\nat_ms = 20_000\nkind = \"task_create\"\nby = 1\ntask = \"w\"\n[[events]]\nat_ms = 30_000\nkind = \"task_take\"\nby = 2\ntask = \"w\"\n[pass]\nexpected_vote_states = [{ vote = \"v\", at_ms = 150_000, state = \"Final\", tally = \"Y3 N0\", winner = \"Yes\" }]\nfinal_never_changes = true\nlate_ballot_counts = true\nno_final_while_split = true",
        ))
        .unwrap();
        let sim = run_sim(&scn, 6, None).unwrap();
        let r = sim.report();
        assert!(r.passed(), "{}\n{}", r.summary_line(), r.to_json());
        assert!(r.events_pending.is_empty(), "{:?}", r.events_pending);
        let gid = sim.group_id(0).unwrap();
        let tasks = sim.nodes()[0].node().tasks(&gid).unwrap();
        assert_eq!(tasks.len(), 1);
        let (_, ui) = sim.nodes()[0].node().task(&gid, &tasks[0]).unwrap();
        assert!(ui.contains("Taken by Node 2"), "{ui}");
    }
}

#[cfg(test)]
mod dbg_tests {
    use super::*;
    use crate::scenario::parse;

    /// `MESHSIM_DBG_SCENARIO=<file> MESHSIM_DBG_SEED=<n> MESHSIM_DBG_MS=<ms>
    /// cargo test -p meshsim dbg_timeline -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn dbg_timeline() {
        let path = std::env::var("MESHSIM_DBG_SCENARIO").unwrap();
        let seed: u64 = std::env::var("MESHSIM_DBG_SEED")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let ms: Option<u64> = std::env::var("MESHSIM_DBG_MS")
            .ok()
            .and_then(|s| s.parse().ok());
        let text = std::fs::read_to_string(path).unwrap();
        let scn = parse(&text).unwrap();
        let mut sim = Sim::new(&scn, seed, ms).unwrap();
        let mut lines = Vec::new();
        let n = sim.nodes.len();
        let snap = |sim: &Sim| -> Vec<(u64, u64, u64, usize, u64, usize)> {
            sim.nodes
                .iter()
                .map(|n| {
                    (
                        n.stats().decrypted,
                        n.stats().decrypt_failures,
                        n.stats().parked,
                        n.store_len(),
                        n.posts_skipped,
                        n.node().groups().len(),
                    )
                })
                .collect()
        };
        let mut last = snap(&sim);
        while !sim.ended {
            let Some(Reverse((at, _, ev))) = sim.queue.pop() else {
                break;
            };
            sim.now_ms = at;
            let tag = match &ev {
                Event::Traffic { node, kind } => Some(format!("traffic node={node} {kind:?}")),
                Event::Scripted(i) => Some(format!(
                    "scripted {i} {:?}",
                    sim.scenario.events[*i].kind_name()
                )),
                Event::LinkUp { a, b } => Some(format!("linkup {a}->{b}")),
                Event::LinkDown { link, reason } => Some(format!("linkdown {link} {reason:?}")),
                Event::Deliver { link, from, bytes } => Some(format!(
                    "deliver link={link} from={from} bytes={}",
                    bytes.len()
                )),
                Event::Cast { node, choice, .. } => {
                    Some(format!("cast node={node} choice={choice}"))
                }
                _ => None,
            };
            sim.handle(ev);
            let now = snap(&sim);
            let only_groups = std::env::var("MESHSIM_DBG_ONLY").is_ok_and(|v| v == "groups");
            if only_groups {
                let g_now: Vec<usize> = now.iter().map(|x| x.5).collect();
                let g_last: Vec<usize> = last.iter().map(|x| x.5).collect();
                if g_now != g_last {
                    lines.push(format!("{at:>8} roster held by {:?}", g_now));
                }
                last = now;
                continue;
            }
            if let Some(t) = tag
                && (now != last || !t.starts_with("deliver"))
            {
                lines.push(format!("{at:>8} {t} | {:?}", now));
                last = now;
            }
        }
        for l in lines {
            println!("{l}");
        }
        let r = sim.report();
        println!("{}", r.summary_line());
        for c in &r.criteria {
            println!("  {} {:?}: {}", c.name, c.status, c.detail);
        }
        for m in r.delivery.misses.iter().take(10) {
            println!("  miss {m:?}");
        }
        println!(
            "pending {:?} failed {:?} applied {:?}",
            r.events_pending, r.events_failed, r.events_applied
        );
        let _ = n;
    }
}
