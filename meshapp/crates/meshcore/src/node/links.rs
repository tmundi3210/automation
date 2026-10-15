//! Per-link state and the byte-stream API (docs/DESIGN.md §7 "Link
//! lifecycle in core": `link_up` → Noise NN handshake (initiator = central)
//! → `Hello` → optional `ContactProof` → summary exchange (`dtn/sync`) →
//! scheduled push; `Throttle` frames carry backpressure; §2.3 link frame;
//! BRIEF_v2 §5.5 per-link limits and lanes, §5.6 strict class order;
//! DECISIONS D4 PER INBOUND LINK, LANES (L2 by a pairwise tag in the link
//! handshake), LINK SLOTS).
//!
//! Each [`LinkEnd`] owns a [`PlatformLink`] (byte pipe), a [`NoiseLink`]
//! (one NN session per link), a [`LinkSync`] (anti-entropy), the inbound
//! [`LinkBudget`] (BRIEF_v2 §5.5 "rate limits per inbound link"), an
//! outbound mirror budget that paces what we push so the peer rarely has
//! to throttle, and a [`Scheduler`] (strict class order, fair across source
//! links). The L2 contact proof: after the handshake, a side that was told
//! which contact it is talking to (`hint`) and holds a `k_pair` for it
//! sends `ContactProof`; the other side matches the proof against every
//! contact's `k_pair` and, on a match, lifts the link to
//! `LinkAuth::VerifiedContact` and answers with its own proof.

use std::collections::{BTreeMap, VecDeque};

use rand::RngCore;

use crate::codec::{Encode, decode};
use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::Hash32;
use crate::dtn::dedupe::DedupeCache;
use crate::dtn::store::CellStore;
use crate::dtn::sync::{LinkSync, Out, SyncConfig, SyncState};
use crate::relay::Lane;
use crate::relay::limits::{Admit, LinkBudget};
use crate::relay::sched::{Band, Scheduler};
use crate::transport::link::{
    Link, LinkAuth, LinkError, LinkId, LinkInfo, PeerKind, Role, Validation,
};
use crate::transport::platform_link::PlatformLink;
use crate::wire::cell::CELL_LEN;
use crate::wire::frame::Class;
use crate::wire::frame::{ContactProof, Frame, LinkFrame};
use crate::wire::noise::{HANDSHAKE_HASH_LEN, NoiseLink, contact_proof, verify_contact_proof};

use super::node::{Node, NodeStats};
use super::status::DeliveryState;
use super::{Event, NodeError, ObjId};

/// Ids pulled from `LinkSync::next_to_send` per call (the store scan is
/// linear, so pulls are batched; engine performance only).
pub const PULL_BATCH: usize = 256;

/// Why a link died (the node removes it and `feed` reports
/// [`NodeError::LinkDead`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkFail {
    /// The Noise session failed (handshake rejected, decrypt failed,
    /// nonce exhausted).
    Noise,
    /// The byte stream was corrupt or a queue overflowed.
    Framing,
    /// An authenticated frame did not decode.
    Malformed,
}

/// One node's side of one link (module docs).
pub(crate) struct LinkEnd {
    pub id: LinkId,
    pub role: Role,
    pub platform: PlatformLink,
    /// `None` only transiently while the handshake state is converted.
    pub noise: Option<NoiseLink>,
    pub sync: LinkSync,
    pub inbound_budget: LinkBudget,
    pub outbound_budget: Option<LinkBudget>,
    pub sched: Scheduler,
    pub head: Option<Hash32>,
    pub pending: VecDeque<Vec<u8>>,
    /// Per (priority band, mirror lane): powered-on time until which a
    /// budget (ours or, through a `Throttle` frame, the peer's) holds that
    /// queue. Holds are per band and lane, never per link (BRIEF_v2 §5.6
    /// strict class order gives a higher class the first turn, not the
    /// right to stall everything else while its own lane is out of tokens:
    /// a paced opaque control cell must stop neither member-validated text
    /// nor the member-validated control cells, ballots and roster ops,
    /// that share its band).
    pub band_hold: BTreeMap<(Band, Lane), u64>,
    /// Cells picked from the scheduler while their queue was held, oldest
    /// first; served before the scheduler once released, so a hold never
    /// reorders a queue behind what arrived meanwhile.
    pub deferred: BTreeMap<(Band, Lane), VecDeque<Hash32>>,
    /// Mirror lane of the last cell sent per class: the queue a peer's
    /// class-level `Throttle` most likely means.
    pub last_lane: BTreeMap<u8, Lane>,
    pub pull_exhausted: bool,
    pub dirty: bool,
    pub auth: LinkAuth,
    pub hint: Option<IdentityKey>,
    pub proof_sent: bool,
    pub announced: bool,
    pub hh: Option<[u8; HANDSHAKE_HASH_LEN]>,
    pub digest_counted: bool,
    /// Simulator adversary hook (`Node::set_adversary_ignore_throttle`):
    /// push through the mirror budget and the peer's `Throttle` holds.
    pub ignore_throttle: bool,
}

impl core::fmt::Debug for LinkEnd {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LinkEnd")
            .field("id", &self.id)
            .field("role", &self.role)
            .field("auth", &self.auth)
            .field("sync", &self.sync.state())
            .finish_non_exhaustive()
    }
}

impl LinkEnd {
    pub fn is_up(&self) -> bool {
        self.noise.as_ref().is_some_and(NoiseLink::is_transport)
    }

    pub fn is_pushing(&self) -> bool {
        self.sync.state() == SyncState::Pushing
    }

    pub fn has_work(&self) -> bool {
        self.head.is_some()
            || !self.pending.is_empty()
            || !self.sched.is_empty()
            || !self.deferred.is_empty()
            || (!self.pull_exhausted && self.is_pushing())
    }

    /// Whether servicing now can send something: work that is not behind a
    /// band hold, or a hold that ended.
    pub fn ready(&self, now: u64) -> bool {
        if self.ignore_throttle {
            return self.has_work();
        }
        self.head.is_some()
            || !self.pending.is_empty()
            || !self.sched.is_empty()
            || (!self.pull_exhausted && self.is_pushing())
            || self
                .deferred
                .keys()
                .any(|k| self.band_hold.get(k).is_none_or(|&until| now >= until))
    }

    /// Ends the band holds that expired.
    fn release_holds(&mut self, now: u64) {
        let ignore = self.ignore_throttle;
        self.band_hold.retain(|_, until| now < *until && !ignore);
    }

    /// The oldest deferred cell of the highest (band, lane) queue that is
    /// not held.
    fn next_deferred(&mut self) -> Option<Hash32> {
        for band in Band::ALL {
            for lane in [Lane::L1, Lane::L2, Lane::L3] {
                let key = (band, lane);
                if self.band_hold.contains_key(&key) {
                    continue;
                }
                if let Some(q) = self.deferred.get_mut(&key) {
                    let id = q.pop_front();
                    if q.is_empty() {
                        self.deferred.remove(&key);
                    }
                    if id.is_some() {
                        return id;
                    }
                }
            }
        }
        None
    }

    /// Holds the queue until `until` (the later of an existing hold and this).
    fn hold_band(&mut self, key: (Band, Lane), until: u64) {
        let e = self.band_hold.entry(key).or_insert(0);
        *e = (*e).max(until);
    }

    /// A peer `Throttle` for `class` (BRIEF_v2 §5.5 backpressure): the
    /// queue of that class on the lane this link last sent it with waits
    /// `retry_ms` (the frame names the class only; the peer's opaque
    /// sub-bucket is the usual cause).
    pub fn throttle_class(&mut self, class: Class, retry_ms: u32, now: u64) {
        let until = now.saturating_add(u64::from(retry_ms));
        let lane = self
            .last_lane
            .get(&class.as_u8())
            .copied()
            .unwrap_or(Lane::L3);
        self.hold_band((Band::of(class, lane), lane), until);
    }

    /// The lane the outbound mirror charges for a held cell (BRIEF_v2 §5.5
    /// lanes): member-validated traffic (cells recognised here and this
    /// node's own group texts, L1) is what a member peer validates too and
    /// admits at the full rate; traffic opaque here is paced as the peer
    /// will see it (L2 to a verified contact, else L3), so that the peer
    /// rarely has to throttle and drop. This node's own unpinned control
    /// cells are paced as opaque: they are mostly pairwise (receipts, SKDM
    /// entries, first flights), which only their recipient validates; the
    /// pinned roster-op carriers (D3 M1) every member validates. A station
    /// validates nothing: everything is L3 there (its L3 cap is the whole
    /// link budget, DESIGN §8).
    fn mirror_lane(&self, meta: &crate::dtn::store::CellMeta, peer_is_station: bool) -> Lane {
        let own_control =
            meta.from_link.is_none() && meta.class == Class::C1Control && !meta.pinned;
        let member_validated = meta.lane == Lane::L1 && !own_control;
        match (member_validated, peer_is_station, self.auth) {
            (true, false, _) => Lane::L1,
            (_, false, LinkAuth::VerifiedContact(_)) => Lane::L2,
            _ => Lane::L3,
        }
    }

    /// Hands pending encrypted frames to the platform until it says `Busy`.
    fn flush_pending(&mut self) -> Result<(), LinkFail> {
        while let Some(frame) = self.pending.front() {
            match self.platform.send(frame) {
                Ok(()) => {
                    self.pending.pop_front();
                }
                Err(LinkError::Busy) => return Ok(()),
                Err(LinkError::Closed | LinkError::TooLarge(_)) => return Err(LinkFail::Framing),
            }
        }
        Ok(())
    }

    /// Encrypts one link frame and queues it (DESIGN §7 layering).
    pub fn stage(&mut self, plain: &[u8]) -> Result<(), LinkFail> {
        let noise = self.noise.as_mut().ok_or(LinkFail::Noise)?;
        let ct = noise.encrypt_frame(plain).map_err(|_| LinkFail::Noise)?;
        self.pending.push_back(ct);
        self.flush_pending()
    }

    pub fn stage_frame(&mut self, frame: &LinkFrame) -> Result<(), LinkFail> {
        let bytes = frame.try_encode().map_err(|_| LinkFail::Malformed)?;
        self.stage(&bytes)
    }

    pub fn apply_outs(&mut self, outs: Vec<Out>) -> Result<(), LinkFail> {
        for out in outs {
            match out {
                Out::SendHello(h) => self.stage_frame(&LinkFrame::hello(&h))?,
                Out::SendSummary(chunks) => {
                    for c in &chunks {
                        let f = LinkFrame::summary(c).map_err(|_| LinkFail::Malformed)?;
                        self.stage_frame(&f)?;
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
    /// send its TTL-decremented copy. Returns the own objects whose cells
    /// went out here for the first time (BRIEF_v2 §5.9 "Passed along").
    #[allow(clippy::too_many_arguments)]
    pub fn service(
        &mut self,
        store: &CellStore,
        dedupe: &DedupeCache,
        own_cells: &std::collections::BTreeMap<Hash32, ObjId>,
        relaying: bool,
        now: u64,
        stats: &mut NodeStats,
        sent: &mut Vec<Hash32>,
    ) -> Result<(), LinkFail> {
        self.flush_pending()?;
        if !self.is_pushing() {
            return Ok(());
        }
        let peer_is_station = self.sync.peer_hello().is_some_and(|h| h.is_station);
        loop {
            if !self.pending.is_empty() {
                return Ok(());
            }
            self.release_holds(now);
            let Some(id) = self.head else {
                if let Some(id) = self.next_deferred() {
                    self.head = Some(id);
                    continue;
                }
                if let Some(id) = self.sched.pick() {
                    // A cell whose band is held waits aside, oldest first;
                    // the lower bands get their turn meanwhile (module docs
                    // on `band_hold`).
                    if let Some((_, meta)) = store.get(&id) {
                        let lane = self.mirror_lane(meta, peer_is_station);
                        let key = (Band::of(meta.class, lane), lane);
                        if self.band_hold.contains_key(&key) {
                            self.deferred.entry(key).or_default().push_back(id);
                            continue;
                        }
                    }
                    self.head = Some(id);
                    continue;
                }
                if self.pull_exhausted {
                    // Something queued and only budgets hold it: "Waiting ·
                    // network busy" (BRIEF_v2 §5.9).
                    self.sched.set_held(!self.deferred.is_empty());
                    return Ok(());
                }
                if self.sync.digest_matched() && !self.dirty {
                    self.pull_exhausted = true;
                    continue;
                }
                let ids = self.sync.next_to_send(store, dedupe, PULL_BATCH);
                if ids.len() < PULL_BATCH {
                    self.pull_exhausted = true;
                }
                if ids.is_empty() {
                    continue;
                }
                for id in ids {
                    let Some((_, meta)) = store.get(&id) else {
                        continue;
                    };
                    if relaying || meta.from_link.is_none() {
                        self.sched
                            .enqueue(id, meta.class, meta.lane, meta.from_link);
                    }
                }
                continue;
            };
            let Some((cell, meta)) = store.get(&id) else {
                self.head = None;
                continue;
            };
            let mirror_lane = self.mirror_lane(meta, peer_is_station);
            let budget = self
                .outbound_budget
                .get_or_insert_with(|| LinkBudget::new(now, peer_is_station));
            let verdict = if self.ignore_throttle {
                Admit::Accept
            } else {
                budget.admit(meta.class, mirror_lane, CELL_LEN, now)
            };
            match verdict {
                Admit::Accept => {}
                Admit::Throttle { retry_ms } => {
                    // Only this band waits; the head (the band's oldest)
                    // goes back to the front and the next band gets its
                    // turn.
                    let key = (Band::of(meta.class, mirror_lane), mirror_lane);
                    self.hold_band(key, now.saturating_add(u64::from(retry_ms)));
                    self.deferred.entry(key).or_default().push_front(id);
                    self.head = None;
                    continue;
                }
                Admit::Reject => {
                    self.head = None;
                    continue;
                }
            }
            let Some(fwd) = cell.forwarded() else {
                self.head = None;
                continue;
            };
            self.stage_frame(&LinkFrame::cell(meta.class, &fwd))?;
            self.last_lane.insert(meta.class.as_u8(), mirror_lane);
            self.head = None;
            stats.cells_sent = stats.cells_sent.saturating_add(1);
            if own_cells.contains_key(&id) {
                sent.push(id);
            }
        }
    }
}

impl Node {
    /// A link came up (DESIGN §7). The central runs the Noise initiator and
    /// writes the first handshake message at once. `hint`: the contact the
    /// platform believes is on the other end (from a B-build marker or the
    /// pairing flow), used only to decide whether to offer a contact proof.
    pub fn link_up(
        &mut self,
        link: LinkId,
        role: Role,
        hint: Option<IdentityKey>,
        now_mono: u64,
    ) -> Result<(), NodeError> {
        let t = self.t(now_mono);
        let info = LinkInfo {
            role,
            peer_kind_hint: PeerKind::Unknown,
            mtu_hint: 0,
        };
        let mut platform = PlatformLink::new(link, info);
        platform.set_auth(LinkAuth::Anonymous);
        let mut noise = match role {
            Role::Central => NoiseLink::initiator(&mut self.rng),
            Role::Peripheral => NoiseLink::responder(&mut self.rng),
        }
        .map_err(|_| NodeError::LinkDead(link))?;
        if role == Role::Central {
            let msg = noise
                .write_handshake()
                .map_err(|_| NodeError::LinkDead(link))?;
            platform.send(&msg).map_err(|_| NodeError::LinkDead(link))?;
        }
        let is_station = self.relay.is_station;
        let end = LinkEnd {
            id: link,
            role,
            platform,
            noise: Some(noise),
            sync: LinkSync::new(SyncConfig {
                is_station,
                ..SyncConfig::default()
            }),
            inbound_budget: LinkBudget::new(t, is_station),
            outbound_budget: None,
            sched: Scheduler::new(),
            head: None,
            pending: VecDeque::new(),
            band_hold: BTreeMap::new(),
            deferred: BTreeMap::new(),
            last_lane: BTreeMap::new(),
            pull_exhausted: false,
            dirty: false,
            auth: LinkAuth::Anonymous,
            hint,
            proof_sent: false,
            announced: false,
            hh: None,
            digest_counted: false,
            ignore_throttle: self.pacing_off(),
        };
        self.relay.slots.on_link_up(role);
        self.pending_penalties.retain(|(l, _)| *l != link);
        self.relay.links.insert(link, end);
        Ok(())
    }

    /// The link is gone: its queues go with it (the DTN store keeps the
    /// bytes; the next encounter re-summarises).
    pub fn link_down(&mut self, link: LinkId, now_mono: u64) {
        let t = self.t(now_mono);
        self.pairing.remove(&link);
        self.pending_penalties.retain(|(l, _)| *l != link);
        if let Some(end) = self.relay.links.remove(&link) {
            self.relay.slots.on_link_down(end.role);
            if end.sync.digest_matched() {
                self.relay.last_sync_t = Some(t);
            }
            if end.announced {
                self.events.push_back(Event::LinkState {
                    link,
                    up: false,
                    auth: end.auth,
                });
            }
        }
    }

    /// Bytes the radio should carry from this side of `link`, at most
    /// `max_bytes` (the link is serviced first so held work goes out).
    pub fn poll_outbound(&mut self, link: LinkId, max_bytes: usize, now_mono: u64) -> Vec<u8> {
        self.service_one(link, now_mono);
        self.relay
            .links
            .get_mut(&link)
            .map(|l| l.platform.poll_outbound(max_bytes))
            .unwrap_or_default()
    }

    /// The inbound budget score of `link` at `now_mono` (`relay::limits`:
    /// 1.0 is clean; penalties lower it and it recovers over powered-on
    /// time), for tests and the simulator. `None` for an unknown link.
    pub fn link_score(&self, link: LinkId, now_mono: u64) -> Option<f32> {
        let t = self.t(now_mono);
        self.relay
            .links
            .get(&link)
            .map(|l| l.inbound_budget.score_at(t))
    }

    /// Links currently up (handshake finished), with their authentication.
    pub fn links(&self) -> Vec<(LinkId, LinkAuth, bool)> {
        self.relay
            .links
            .values()
            .map(|l| (l.id, l.auth, l.is_up()))
            .collect()
    }

    /// Inbound radio bytes for `link` (any chunking). Every complete Noise
    /// message is processed: handshake, control frames, cells through the
    /// inbound pipeline. A fatal link error removes the link and returns
    /// [`NodeError::LinkDead`].
    pub fn feed(&mut self, link: LinkId, bytes: &[u8], now_mono: u64) -> Result<(), NodeError> {
        self.last_now = now_mono;
        {
            let end = self
                .relay
                .links
                .get_mut(&link)
                .ok_or(NodeError::UnknownLink(link))?;
            if end.platform.feed(bytes).is_err() {
                self.kill_link(link, now_mono);
                return Err(NodeError::LinkDead(link));
            }
        }
        match self.process_link(link, now_mono) {
            Ok(()) => Ok(()),
            Err(_) => {
                self.kill_link(link, now_mono);
                Err(NodeError::LinkDead(link))
            }
        }
    }

    fn kill_link(&mut self, link: LinkId, now_mono: u64) {
        self.link_down(link, now_mono);
    }

    /// Drains the link's inbound frames, then services it.
    fn process_link(&mut self, link: LinkId, now_mono: u64) -> Result<(), LinkFail> {
        let t = self.t(now_mono);
        let Some(mut end) = self.relay.links.remove(&link) else {
            return Ok(());
        };
        let version = self.relay.store_version;
        let r = self.process_inbound(&mut end, t, now_mono);
        // Penalties earned by this link's own cells (DECISIONS §3 row
        // (1,2,3,4,10) (iii)) land now that the end is in hand.
        for (id, why) in std::mem::take(&mut self.pending_penalties) {
            if id == end.id {
                end.inbound_budget.penalise(why, t);
                self.stats.penalties = self.stats.penalties.saturating_add(1);
            } else if let Some(other) = self.relay.links.get_mut(&id) {
                other.inbound_budget.penalise(why, t);
                self.stats.penalties = self.stats.penalties.saturating_add(1);
            }
        }
        if self.relay.store_version != version && end.pull_exhausted {
            // Cells were held while this link was out of the map (its own
            // frames were being processed): re-pull so that replies such as
            // receipts go back on it.
            end.pull_exhausted = false;
            end.dirty = false;
        }
        let r = r.and_then(|()| self.service_end(&mut end, now_mono));
        self.relay.links.insert(link, end);
        r
    }

    fn process_inbound(
        &mut self,
        end: &mut LinkEnd,
        t: u64,
        now_mono: u64,
    ) -> Result<(), LinkFail> {
        while let Some(msg) = end.platform.recv() {
            let noise = end.noise.as_mut().ok_or(LinkFail::Noise)?;
            if !noise.is_transport() {
                noise.read_handshake(&msg).map_err(|_| LinkFail::Noise)?;
                if noise.is_my_turn() {
                    let reply = noise.write_handshake().map_err(|_| LinkFail::Noise)?;
                    end.platform.send(&reply).map_err(|_| LinkFail::Framing)?;
                }
                if noise.is_handshake_finished() {
                    let hs = end.noise.take().ok_or(LinkFail::Noise)?;
                    let transport = hs.into_transport().map_err(|_| LinkFail::Noise)?;
                    end.hh = Some(transport.handshake_hash());
                    end.noise = Some(transport);
                    self.stats.handshakes = self.stats.handshakes.saturating_add(1);
                    self.offer_contact_proof(end)?;
                    let salt = self.rng.next_u32();
                    let outs = end
                        .sync
                        .start(salt, t, &self.relay.store, &self.relay.dedupe);
                    end.apply_outs(outs)?;
                }
                continue;
            }
            let plain = noise.decrypt_frame(&msg).map_err(|_| LinkFail::Noise)?;
            let frame: Frame = decode(&plain).map_err(|_| LinkFail::Malformed)?;
            match frame {
                Frame::Hello(h) => {
                    let outs = end.sync.on_hello(h);
                    end.apply_outs(outs)?;
                    if !end.announced {
                        end.announced = true;
                        self.events.push_back(Event::LinkState {
                            link: end.id,
                            up: true,
                            auth: end.auth,
                        });
                    }
                }
                Frame::Summary(s) => {
                    let outs = end.sync.on_summary_chunk(s);
                    end.apply_outs(outs)?;
                    self.note_digest(end, t, now_mono);
                }
                Frame::Cell { class, cell } => {
                    self.on_cell(end, class, cell, now_mono);
                }
                Frame::Throttle(th) => {
                    if let Ok(class) = Class::from_u8(th.class) {
                        end.throttle_class(class, th.retry_ms, t);
                    }
                }
                Frame::ContactProof(p) => self.on_contact_proof(end, &p)?,
            }
        }
        Ok(())
    }

    /// "Last sync" (BRIEF_v2 §2, D9): the digest matched a neighbour's.
    fn note_digest(&mut self, end: &mut LinkEnd, t: u64, now_mono: u64) {
        if !end.digest_counted && end.sync.peer_summary().is_some() && end.sync.digest_matched() {
            end.digest_counted = true;
            self.relay.last_sync_t = Some(t);
        }
        if end.sync.peer_summary().is_some() {
            self.note_reach(end, now_mono);
        }
    }

    /// A peer's summary that holds one of our Take cells is evidence the
    /// claim reached beyond the people near us (`tasks::state::ReachEvidence`):
    /// recorded in its own transaction, with the task's UI string refolded
    /// and `TaskChanged` emitted after the commit.
    fn note_reach(&mut self, end: &LinkEnd, now_mono: u64) {
        let Some(summary) = end.sync.peer_summary() else {
            return;
        };
        let mut reached: Vec<(
            crate::crypto::backend::GroupId,
            crate::objects::task_op::TaskId,
        )> = Vec::new();
        for (gid, g) in &self.proto.groups {
            for (task_id, task) in &g.tasks {
                if !task.seen_beyond_reach
                    && task.own_take_cells.iter().any(|c| summary.contains(c))
                {
                    reached.push((*gid, *task_id));
                }
            }
        }
        if reached.is_empty() {
            return;
        }
        let _ = self.run_txn(now_mono, |ctx| {
            let me = ctx.me()?;
            for (gid, task_id) in &reached {
                let names: std::collections::BTreeMap<IdentityKey, String> = ctx
                    .proto
                    .groups
                    .get(gid)
                    .map(|g| {
                        g.view
                            .members
                            .iter()
                            .map(|(k, m)| (*k, m.name.clone()))
                            .collect()
                    })
                    .unwrap_or_default();
                let Some(task) = ctx
                    .proto
                    .groups
                    .get_mut(gid)
                    .and_then(|g| g.tasks.get_mut(task_id))
                else {
                    continue;
                };
                task.seen_beyond_reach = true;
                let before = task.ui.clone();
                let lookup = |k: &IdentityKey| -> String {
                    names
                        .get(k)
                        .cloned()
                        .unwrap_or_else(|| crate::roster::labels::UNNAMED.to_owned())
                };
                super::inbound::refold_task(task_id, task, &me, &lookup);
                let ui = task.ui.clone();
                super::persist::save_task_meta(&mut *ctx.txn, gid, task_id, task)?;
                if ui != before {
                    ctx.events.push(Event::TaskChanged {
                        group: *gid,
                        task_id: *task_id,
                        ui,
                    });
                }
            }
            Ok(())
        });
    }

    /// Sends our L2 proof when the hinted peer is a contact we hold a
    /// `k_pair` for (DESIGN §2.3; D4 LANES L2).
    fn offer_contact_proof(&mut self, end: &mut LinkEnd) -> Result<(), LinkFail> {
        if end.proof_sent {
            return Ok(());
        }
        let (Some(hint), Some(hh)) = (end.hint, end.hh) else {
            return Ok(());
        };
        let Some(k) = self.proto.contacts.get(&hint).and_then(|c| c.link_key) else {
            return Ok(());
        };
        let proof = ContactProof {
            proofs: vec![contact_proof(&k, end.role, &hh)],
        };
        let f = LinkFrame::contact_proof(&proof).map_err(|_| LinkFail::Malformed)?;
        end.stage_frame(&f)?;
        end.proof_sent = true;
        Ok(())
    }

    /// Matches a received proof against every contact's `k_pair`; one match
    /// lifts the link to L2 and, if we have not proved ourselves yet, we
    /// answer with our own proof for that contact.
    fn on_contact_proof(&mut self, end: &mut LinkEnd, p: &ContactProof) -> Result<(), LinkFail> {
        let Some(hh) = end.hh else {
            return Ok(());
        };
        let peer_role = crate::pairwise::pairing::peer_role(end.role);
        let mut matched = None;
        'outer: for proof in &p.proofs {
            for (key, c) in &self.proto.contacts {
                if let Some(k) = c.link_key
                    && verify_contact_proof(&k, peer_role, &hh, proof)
                {
                    matched = Some((*key, k));
                    break 'outer;
                }
            }
        }
        let Some((key, k)) = matched else {
            return Ok(());
        };
        end.auth = LinkAuth::VerifiedContact(key);
        end.platform.set_auth(end.auth);
        if !end.proof_sent {
            let proof = ContactProof {
                proofs: vec![contact_proof(&k, end.role, &hh)],
            };
            let f = LinkFrame::contact_proof(&proof).map_err(|_| LinkFail::Malformed)?;
            end.stage_frame(&f)?;
            end.proof_sent = true;
        }
        self.events.push_back(Event::LinkState {
            link: end.id,
            up: true,
            auth: end.auth,
        });
        Ok(())
    }

    /// Services one link end: pushes what the scheduler offers and records
    /// "Passed along" for own objects whose every cell went out.
    fn service_end(&mut self, end: &mut LinkEnd, now_mono: u64) -> Result<(), LinkFail> {
        let t = self.t(now_mono);
        let relaying = self.relay.relaying();
        let mut sent = Vec::new();
        let r = end.service(
            &self.relay.store,
            &self.relay.dedupe,
            &self.proto.own_cells,
            relaying,
            t,
            &mut self.stats,
            &mut sent,
        );
        if !sent.is_empty() {
            self.note_passed_along(&sent, now_mono);
        }
        r
    }

    /// Own cells went out on a link: objects whose every cell has now been
    /// sent at least once become "Passed along" (BRIEF_v2 §5.9). Recorded
    /// in its own transaction ([`Node::run_txn`]): the state changes and
    /// the event is emitted only once the commit succeeded; a refused
    /// commit leaves the node where the last commit left it (BRIEF_v2
    /// §5.3).
    fn note_passed_along(&mut self, sent: &[Hash32], now_mono: u64) {
        let mut sent_objs: Vec<ObjId> = sent
            .iter()
            .filter_map(|c| self.proto.own_cells.get(c).copied())
            .collect();
        sent_objs.sort_unstable();
        sent_objs.dedup();
        let candidates: Vec<ObjId> = sent_objs
            .into_iter()
            .filter(|obj| {
                self.proto
                    .outbox
                    .get(obj)
                    .is_some_and(|e| !e.passed && !e.state.is_solid())
            })
            .collect();
        if candidates.is_empty() {
            return;
        }
        let _ = self.run_txn(now_mono, |ctx| {
            for obj in &candidates {
                let Some(e) = ctx.proto.outbox.get_mut(obj) else {
                    continue;
                };
                e.passed = true;
                ctx.proto.dirty.outbox = true;
                if matches!(
                    e.state,
                    DeliveryState::Waiting | DeliveryState::WaitingNetworkBusy
                ) {
                    e.state = DeliveryState::PassedAlong;
                    ctx.events.push(Event::Delivery {
                        obj: *obj,
                        state: DeliveryState::PassedAlong,
                    });
                }
            }
            Ok(())
        });
    }

    fn service_one(&mut self, link: LinkId, now_mono: u64) {
        let Some(mut end) = self.relay.links.remove(&link) else {
            return;
        };
        let r = self.service_end(&mut end, now_mono);
        match r {
            Ok(()) => {
                self.relay.links.insert(link, end);
            }
            Err(_) => {
                self.relay.links.insert(link, end);
                self.kill_link(link, now_mono);
            }
        }
    }

    /// Services every pushing link that has work and is not held
    /// (throttle retries, live forwards queued since the last pass).
    pub(crate) fn service_links(&mut self, now_mono: u64) {
        let t = self.t(now_mono);
        let ids: Vec<LinkId> = self
            .relay
            .links
            .values()
            .filter(|l| l.is_pushing() && l.ready(t))
            .map(|l| l.id)
            .collect();
        for id in ids {
            self.service_one(id, now_mono);
        }
    }

    /// The lane a cell on `end` takes (BRIEF_v2 §5.5): L1 if this node
    /// recognised it, L2 over a verified-contact link, else L3.
    pub(crate) fn lane_for(end: &LinkEnd, validated: bool) -> crate::relay::Lane {
        end.platform.lane_for(if validated {
            Validation::Validated
        } else {
            Validation::Opaque
        })
    }
}
