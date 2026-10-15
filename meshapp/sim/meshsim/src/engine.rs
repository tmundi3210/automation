//! The deterministic discrete-event loop (DESIGN §7 `SimLink` row, §10
//! "Simulator scenarios"; docs/scenarios.md). One [`Sim`] runs one scenario
//! for one seed: the event queue is ordered by `(time, sequence)`, every
//! random draw comes from a `ChaCha20Rng` seeded from the run's seed (or
//! from a per-node RNG seeded from it), and all iteration is over ordered
//! collections, so the same seed gives byte-identical reports.
//!
//! Event kinds: `NodeTick(n)` (the node's `tick_ms` tick), `LinkAttempt(n)`
//! (a node tries to connect to a random reachable node, DESIGN §7 "link
//! establishment"), `LinkUp`/`LinkDown`, `Deliver` (a batch of bytes
//! arrives at one end of a link), `Traffic` (one node's next post, DM or
//! junk cell), `Scripted(idx)` (a `[[events]]` entry), `PartitionChange
//! (idx)` and `End`.
//!
//! Per-node work is event driven: link attempts, traffic and deliveries are
//! events; a node tick only sweeps its own links for throttle retries.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use meshcore::dtn::dedupe::{KEY_RETENTION_ASM_MS, dedupe_retention_ms};
use meshcore::transport::link::{PeerKind, Role};
use meshcore::wire::cell::BODY_PLAINTEXT_LEN;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::assert;
use crate::radio::{Radio, RadioParams};
use crate::report::{BytesOnLinks, CellTraffic, CellsCreated, Links, NodeReport, Report};
use crate::scenario::{self, Behaviour, NodeKind, Resolved, Scenario, ScenarioError};
use crate::simnode::{
    Audience, CellKind, CellRegistry, DropReason, IPHONE_SLOTS, NodeConfig, NodeStats, SimNode,
    TickOutcome,
};

/// Largest batch of bytes one `Deliver` event carries (assumption; the
/// receiver still feeds it to the framer in MTU-sized chunks).
pub const BATCH_BYTES: usize = 4096;
/// Connect latency from dial to link up (assumption: a BLE connection
/// takes a few connection intervals).
pub const CONNECT_DELAY_MS_ASM: u64 = 200;
/// Dial period per node: "every 2 s ± jitter".
pub const DIAL_PERIOD_MS: u64 = 2_000;
pub const DIAL_JITTER_MS: u64 = 500;

/// What a `Traffic` event generates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrafficKind {
    /// A post to group `g`.
    GroupPost(usize),
    /// A DM in pair `p` (the event's node is the sender; the reply comes
    /// from the other end).
    Dm(usize),
    /// A junk cell from a flooding node every `period_ms`.
    Flood { period_ms: u64 },
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
    PartitionChange(usize),
    End,
}

#[derive(Debug, Default)]
struct Metrics {
    established: u64,
    wake_links: u64,
    attempts: u64,
    attempts_failed: u64,
    dropped_by_reason: BTreeMap<DropReason, u64>,
    events_ignored: BTreeMap<String, u64>,
    cells_created: CellsCreated,
}

/// One run. Build with [`Sim::new`], drive with [`Sim::run_to_end`], read
/// with [`Sim::report`].
#[derive(Debug)]
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
    registry: CellRegistry,
    /// Live link ids per node.
    links_of: Vec<BTreeSet<u32>>,
    pairs: Vec<(usize, usize)>,
    metrics: Metrics,
    rejoin_at: Option<u64>,
    ended: bool,
}

impl Sim {
    /// Sets up nodes, pairs, the radio and the initial events.
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
            let contacts: BTreeSet<usize> = pairs
                .iter()
                .filter_map(|&(a, b)| {
                    if a == i {
                        Some(b)
                    } else if b == i {
                        Some(a)
                    } else {
                        None
                    }
                })
                .collect();
            let cfg = NodeConfig {
                idx: i,
                kind,
                slots: match kind {
                    NodeKind::IPhone => IPHONE_SLOTS,
                    NodeKind::Android | NodeKind::Station => scenario.nodes.android_slots,
                },
                battery_floor_ppm: floor_ppm,
                battery_start_ppm,
                foreground_duty_ppm: duty_ppm,
                wall_offset_ms,
                dedupe_retention_ms,
                contacts,
            };
            nodes.push(SimNode::new(cfg, node_seed));
        }

        let params = RadioParams {
            latency_ms: scenario.radio.latency_ms,
            throughput_bytes_per_s: scenario.radio.throughput_bytes_per_s,
            frame_loss: scenario.radio.frame_loss,
            iphone_wake_ms: scenario.radio.iphone_wake_ms,
        };
        let mut radio = Radio::new(params, resolved.kinds.clone());
        radio.set_partition(resolved.partition_at(0));
        let registry = CellRegistry::new(
            n,
            resolved.groups.iter().map(|g| g.members.clone()).collect(),
        );

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
            registry,
            links_of: vec![BTreeSet::new(); n],
            pairs,
            metrics: Metrics::default(),
            rejoin_at: None,
            ended: false,
        };
        sim.schedule_initial();
        Ok(sim)
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

    pub fn registry(&self) -> &CellRegistry {
        &self.registry
    }

    pub fn rejoin_at(&self) -> Option<u64> {
        self.rejoin_at
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
            Event::PartitionChange(i) => self.partition_change(i),
            Event::End => self.ended = true,
        }
    }

    // ---- handlers ------------------------------------------------------------------

    fn apply_outcome(&mut self, outcome: TickOutcome, node: usize) {
        for link in outcome.serviced {
            self.pump(link, node);
        }
        for (link, reason) in outcome.dead_links {
            self.link_down(link, reason);
        }
        if outcome.became_foreground {
            let at = self
                .now_ms
                .saturating_add(self.rng.random_range(0..DIAL_PERIOD_MS));
            self.push(at, Event::LinkAttempt(node));
        }
    }

    fn node_tick(&mut self, n: usize) {
        let now = self.now_ms;
        let outcome = self.nodes[n].tick(now, &mut self.registry);
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

    fn hint_for(&self, m: usize) -> PeerKind {
        let node = &self.nodes[m];
        if node.is_station() {
            PeerKind::Station
        } else if node.is_locked_iphone() {
            PeerKind::AppleOverflow
        } else {
            PeerKind::AppUuid
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
        let chunk = self.radio.chunk_for(a, b);
        self.links_of[a].insert(id);
        self.links_of[b].insert(id);
        let (hint_a, hint_b) = (self.hint_for(b), self.hint_for(a));
        let contact_a = self.nodes[a]
            .is_contact(b)
            .then(|| self.nodes[b].identity());
        let contact_b = self.nodes[b]
            .is_contact(a)
            .then(|| self.nodes[a].identity());
        if let Err(reason) = self.nodes[a].link_up(id, Role::Central, chunk, hint_a, contact_a, now)
        {
            self.link_down(id, reason);
            return;
        }
        if let Err(reason) =
            self.nodes[b].link_up(id, Role::Peripheral, chunk, hint_b, contact_b, now)
        {
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
            self.links_of[link.a].remove(&id);
            self.links_of[link.b].remove(&id);
            *self.metrics.dropped_by_reason.entry(reason).or_insert(0) += 1;
        }
    }

    /// Moves what `side` queued on `link` onto the air as `Deliver` events
    /// and rolls the per-frame loss (DESIGN §7 "loss 0.5 %/frame → link
    /// drop").
    fn pump(&mut self, link: u32, side: usize) {
        let now = self.now_ms;
        loop {
            let bytes = self.nodes[side].poll_outbound(link, BATCH_BYTES);
            if bytes.is_empty() {
                break;
            }
            let Some(arrive) = self.radio.transmit(link, side, bytes.len(), now) else {
                break;
            };
            self.push(
                arrive,
                Event::Deliver {
                    link,
                    from: side,
                    bytes,
                },
            );
        }
        let frames = self.nodes[side].take_unrolled_frames(link);
        let loss = self.radio.params().frame_loss;
        if loss > 0.0 {
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
    }

    fn deliver(&mut self, link: u32, from: usize, bytes: &[u8]) {
        let now = self.now_ms;
        let Some(l) = self.radio.link(link) else {
            return;
        };
        let to = l.peer_of(from);
        let chunk = l.chunk;
        if let Err(reason) = self.nodes[to].on_bytes(link, bytes, chunk, now, &mut self.registry) {
            self.link_down(link, reason);
            return;
        }
        self.pump(link, to);
        if self.radio.link(link).is_none() {
            return;
        }
        // The sender's queue drained: let it continue pushing.
        if let Err(reason) = self.nodes[from].service_link(link, now, &mut self.registry) {
            self.link_down(link, reason);
            return;
        }
        self.pump(link, from);
    }

    fn jitter(&mut self, period_ms: u64) -> u64 {
        let lo = period_ms.saturating_mul(3) / 4;
        let hi = period_ms.saturating_mul(5) / 4;
        self.rng.random_range(lo.max(1)..=hi.max(lo.max(1)))
    }

    /// Cells one text of the scenario's size range takes.
    fn text_cells(&mut self) -> usize {
        let [lo, hi] = self.scenario.traffic.text_bytes;
        let text: u32 = self.rng.random_range(lo..=hi);
        usize::try_from(text)
            .unwrap_or(usize::MAX)
            .div_ceil(BODY_PLAINTEXT_LEN)
            .max(1)
    }

    fn create_cells(&mut self, node: usize, kind: CellKind, audience: Audience, count: usize) {
        let now = self.now_ms;
        let fragment = self.radio.fragment_of(node);
        for _ in 0..count {
            self.nodes[node].create_cell(kind, audience, fragment, now, &mut self.registry);
        }
        let c = u64::try_from(count).unwrap_or(u64::MAX);
        self.metrics.cells_created.total += c;
        match kind {
            CellKind::Post => self.metrics.cells_created.posts += c,
            CellKind::Dm => self.metrics.cells_created.dms += c,
            CellKind::Junk => self.metrics.cells_created.junk += c,
        }
        let mut outcome = TickOutcome::default();
        self.nodes[node].service_links(now, &self.registry, &mut outcome);
        self.apply_outcome(outcome, node);
    }

    fn traffic(&mut self, node: usize, kind: TrafficKind) {
        let now = self.now_ms;
        match kind {
            TrafficKind::GroupPost(g) => {
                let count = self.text_cells();
                self.create_cells(node, CellKind::Post, Audience::Group(g), count);
                let next =
                    now.saturating_add(self.jitter(self.scenario.traffic.group_post_every_ms));
                self.push(next, Event::Traffic { node, kind });
            }
            TrafficKind::Dm(p) => {
                let (a, b) = self.pairs[p];
                let other = if node == a { b } else { a };
                let count = self.text_cells();
                self.create_cells(node, CellKind::Dm, Audience::Pair(a, b), count);
                let next = now.saturating_add(self.jitter(self.scenario.traffic.dm_every_ms));
                self.push(next, Event::Traffic { node: other, kind });
            }
            TrafficKind::Flood { period_ms } => {
                self.create_cells(node, CellKind::Junk, Audience::Nobody, 1);
                self.push(now.saturating_add(period_ms), Event::Traffic { node, kind });
            }
        }
    }

    fn scenario_event(&mut self, i: usize) {
        let now = self.now_ms;
        let Some(ev) = self.scenario.events.get(i) else {
            return;
        };
        match ev {
            scenario::Event::Adversary {
                node,
                behaviour: Behaviour::Flood,
                cells_per_min: Some(cpm),
                ..
            } => {
                let period_ms = (60_000 / u64::from(*cpm).max(1)).max(1);
                let node = *node;
                if let Some(n) = self.nodes.get_mut(node) {
                    n.set_flooding(true);
                }
                self.push(
                    now,
                    Event::Traffic {
                        node,
                        kind: TrafficKind::Flood { period_ms },
                    },
                );
            }
            other => {
                let key = match other {
                    scenario::Event::Adversary { behaviour, .. } => {
                        format!("adversary:{}", behaviour_name(*behaviour))
                    }
                    e => e.kind_name().to_string(),
                };
                *self.metrics.events_ignored.entry(key).or_insert(0) += 1;
            }
        }
    }

    fn partition_change(&mut self, i: usize) {
        let now = self.now_ms;
        let Some(p) = self.resolved.partitions.get(i).cloned() else {
            return;
        };
        let before = self.radio.fragment_count();
        let cut = self.radio.set_partition(&p);
        for id in cut {
            self.link_down(id, DropReason::Partition);
        }
        if p.fragment_count == 1 && before > 1 {
            self.rejoin_at = Some(now);
        }
    }

    // ---- report --------------------------------------------------------------------

    /// Evaluates the pass criteria and builds the report.
    pub fn report(&self) -> Report {
        let eval = assert::evaluate(
            &self.scenario,
            self.nodes(),
            self.registry(),
            self.rejoin_at(),
            self.duration_ms,
        );
        let mut stats = NodeStats::default();
        for n in self.nodes() {
            stats.add(&n.stats);
        }
        // Every node's clock is brought to the end of the run first: the
        // display offset is read as wall − mono, and a node's clock only
        // moves when the engine calls into it.
        for n in &self.nodes {
            n.clock.set(self.now_ms);
        }
        let nodes = self
            .nodes
            .iter()
            .map(|n| NodeReport {
                idx: n.idx(),
                kind: n.kind().name(),
                battery_pct: (n.battery_ppm() / 1_000) as f64 / 10.0,
                foreground: n.is_foreground(),
                cells_held: n.store_len(),
                links_up: n.link_count(),
                wall_offset_ms: n.clock.display_hook_wall()
                    - i64::try_from(self.now_ms()).unwrap_or(0),
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
        Report {
            scenario: self.scenario.scenario.name.clone(),
            seed: self.seed,
            duration_ms: self.duration_ms,
            nodes,
            cells_created: self.metrics.cells_created.clone(),
            delivery: eval.delivery,
            drain_ms: eval.drain_ms,
            rejoin_at_ms: self.rejoin_at,
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
                digest_matches: stats.digest_matches,
            },
            cells: CellTraffic {
                frames_sent: stats.frames_sent,
                cells_sent: stats.cells_sent,
                cells_received: stats.cells_received,
                duplicates: stats.duplicates,
                malformed: stats.malformed,
                throttled_in: stats.throttled_in,
                throttled_out: stats.throttled_out,
                rejected_in: stats.rejected_in,
                rejected_out: stats.rejected_out,
                no_room: stats.no_room,
                throttles_received: stats.throttles_received,
                withheld_low_battery: stats.withheld_low_battery,
                summary_errors: stats.summary_errors,
            },
            wall_clock_reads: self.nodes.iter().map(SimNode::wall_reads).sum(),
            events_ignored: self.metrics.events_ignored.clone(),
            criteria: eval.criteria,
            host_seconds: 0.0,
        }
    }
}

fn behaviour_name(b: Behaviour) -> &'static str {
    match b {
        Behaviour::KeepKeysAfterRemoval => "keep_keys_after_removal",
        Behaviour::EquivocateBallot => "equivocate_ballot",
        Behaviour::SockpuppetAdds => "sockpuppet_adds",
        Behaviour::WithholdDms => "withhold_dms",
        Behaviour::ReplayCells => "replay_cells",
        Behaviour::Flood => "flood",
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

/// Runs and reports.
pub fn run(
    scenario: &Scenario,
    seed: u64,
    duration_override_ms: Option<u64>,
) -> Result<Report, ScenarioError> {
    Ok(run_sim(scenario, seed, duration_override_ms)?.report())
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
            "[pass]\nevery_post_decrypts_after_rejoin = true\nno_duplicate_ui_delivery = true\nzero_wall_clock_reads = true",
        ))
        .unwrap();
        let sim = run_sim(&scn, 3, None).unwrap();
        let r = sim.report();
        assert!(r.passed(), "{}", r.summary_line());
        assert!(r.cells_created.total >= 10);
        assert_eq!(r.delivery.ratio, 1.0);
        assert_eq!(r.links.established, 1);
        assert_eq!(r.links.handshakes, 1);
        assert_eq!(r.wall_clock_reads, 0);
        assert!(r.bytes_on_links.max_link_bytes > 0);
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
    fn flood_event_generates_junk_and_other_events_are_counted() {
        let scn = parse(&toml(
            "android = 2\niphone = 0",
            "frame_loss = 0.0",
            "[[events]]\nat_ms = 1000\nkind = \"adversary\"\nnode = 1\nbehaviour = \"flood\"\ncells_per_min = 600\n[[events]]\nat_ms = 2000\nkind = \"task_create\"\nby = 0\ntask = \"w\"",
        ))
        .unwrap();
        let r = run(&scn, 2, Some(10_000)).unwrap();
        assert!(r.cells_created.junk >= 80, "{}", r.cells_created.junk);
        assert_eq!(r.events_ignored.get("task_create"), Some(&1));
        // docs/scenarios.md S-flood "injecting 600 junk cells/min": the
        // flooder does not pace itself, so over a minute nearly all of its
        // junk reaches the honest node's inbound budget (where it is
        // throttled at the L3 cap, BRIEF_v2 §5.5).
        let r = run(&scn, 2, Some(60_000)).unwrap();
        assert!(r.cells_created.junk >= 580, "{}", r.cells_created.junk);
        assert!(
            r.cells.cells_received >= 500,
            "only {} cells arrived at the honest node of {} junk: {}",
            r.cells.cells_received,
            r.cells_created.junk,
            r.summary_line()
        );
        assert!(r.cells.throttled_in > 300, "{}", r.cells.throttled_in);
        assert_eq!(r.cells.throttled_out, 0, "the flooder never holds back");
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
}
