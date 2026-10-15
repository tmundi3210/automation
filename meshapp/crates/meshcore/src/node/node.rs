//! The [`Node`]: construction, the transaction plumbing, the tick and the
//! queries (docs/DESIGN.md §1 row `node/node.rs`; §6 "Powered-on meter",
//! "Atomic commit"; BRIEF_v2 §2 status strip, §4 "Needs someone", §5.3,
//! §5.7 battery mode; DECISIONS D6 rules 3–4).
//!
//! The in-memory protocol state lives in [`Proto`]; everything in it is
//! persisted by [`super::persist`] and reloaded by the same loader at open
//! and after a failed transaction. The relay buffers ([`Relay`]) are not
//! persisted: the DTN store, the dedupe cache and the links start empty at
//! every open (the simulator restarts nothing; `storage::kv::ns` names the
//! namespaces that are the persistence contract).

use std::cell::Cell as StdCell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::clock::{Clock, PoweredOnMeter};
use crate::crypto::backend::{Backend, GroupId, PeerId, default_backend};
use crate::crypto::ed25519::{IdentityKey, IdentitySigner};
use crate::crypto::hash::Hash32;
use crate::crypto::keycard::KeyCard;
use crate::dtn::dedupe::{DEDUPE_MAX_ENTRIES_ASM, DedupeCache};
use crate::dtn::store::CellStore;
use crate::groups::group::GroupState;
use crate::objects::ballot::Ballot;
use crate::objects::sender_chain::SenderChainIndex;
use crate::objects::task_op::TaskId;
use crate::objects::vote_open::VoteOpen;
use crate::pairwise::bundles::BundleManager;
use crate::pairwise::pairing::PairingSession;
use crate::pairwise::session::PairwiseManager;
use crate::people::samename::SameNameIndex;
use crate::relay::buffers::BufferPolicy;
use crate::relay::slots::SlotPolicy;
use crate::roster::dag::RosterDag;
use crate::roster::eval::RosterView;
use crate::roster::rotation::RotationPolicy;
use crate::storage::db::{Db, meta_keys};
use crate::storage::txn::Txn;
use crate::tasks::pool::EvidenceAge;
use crate::tasks::state::{OpRecord, TaskState};
use crate::transport::link::LinkId;
use crate::votes::open::{OpenReject, ValidOpen};
use crate::votes::tally::Tally;
use crate::wire::seal::EnvelopeKeyPair;

use super::links::LinkEnd;
use super::persist;
use super::reasm::Reasm;
use super::status::{DeliveryState, StatusStrip, age_transition};
use super::{Event, NodeConfig, NodeError, NodeRole, ObjId, VoteId};

/// Powered-on time between dedupe sweeps and meter checkpoints (DESIGN §6
/// "all expiries use the powered-on meter"; the sweep is cheap).
pub const SWEEP_MS_ASM: u64 = 60_000;
/// The receipt batching window the simulator configures
/// (`Node::set_receipt_batch_ms`): after a receipt to a peer for a
/// conversation, the next cumulative receipt waits this long (DECISIONS D4
/// MERGE "receipts are cumulative per recipient per conversation;
/// per-message receipts would total 93,600 and take about 26 h"; D4 pick B
/// (5) "receipts batched and randomly delayed", B defaults 5–15 min; the
/// upper bound, no jitter: the opaque control lane carries about 5
/// cells/min per link, so a 40-member group cannot carry one receipt per
/// (member, author) much more often; assumption).
pub const RECEIPT_BATCH_MS_ASM: u64 = 15 * crate::clock::ms::MINUTE;

/// This node's identity (a member; stations have none).
pub(crate) struct Identity {
    pub signer: IdentitySigner,
    pub card: KeyCard,
    pub display_name: String,
    /// The current envelope key for sealed first flights (D6 pick B (2)); at
    /// P0 one key, never rotated (open issue: rotation needs the new key to
    /// travel inside group encryption).
    pub envelope: EnvelopeKeyPair,
}

impl core::fmt::Debug for Identity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Identity")
            .field("key", &self.card.ed)
            .field("display_name", &self.display_name)
            .finish_non_exhaustive()
    }
}

/// What the node remembers about a pinned contact between transactions
/// (a cache of `people::contacts`; the store is authoritative).
#[derive(Debug, Clone)]
pub(crate) struct ContactLite {
    pub display_name: String,
    pub link_key: Option<[u8; 32]>,
    pub envelope_pub: Option<[u8; 32]>,
    pub verified: bool,
}

/// One task as this node folds it.
#[derive(Debug, Clone, Default)]
pub(crate) struct TaskCtx {
    pub records: Vec<OpRecord>,
    /// Powered-on time this node first held a Take (for the pool rule).
    pub taken_at: Option<u64>,
    /// Cells of this node's own Take, to learn whether it reached beyond
    /// the people near us (`tasks::state::ReachEvidence`).
    pub own_take_cells: Vec<Hash32>,
    pub seen_beyond_reach: bool,
    pub state: Option<TaskState>,
    pub ui: String,
}

/// One group on this node.
#[derive(Debug)]
pub(crate) struct GroupCtx {
    pub state: GroupState,
    pub dag: RosterDag,
    pub policy: RotationPolicy,
    pub view: RosterView,
    /// Members that received this node's current chain (its SKDM).
    pub holders: BTreeSet<PeerId>,
    /// Next `author_seq` and `prev_own` of this node's identity-signed
    /// objects in the group (`objects::sender_chain`).
    pub own_seq: (u32, Hash32),
    pub tasks: BTreeMap<TaskId, TaskCtx>,
    /// Objects under a sender's chain that the backend could not decrypt
    /// yet (`NoSenderKey`: "Waiting · can't read yet"), retried when its
    /// SKDM arrives. Bounded per sender.
    pub parked: BTreeMap<(PeerId, u32), Vec<u8>>,
}

/// One vote on this node (P1 types, DESIGN §5).
#[derive(Debug, Clone)]
pub(crate) struct VoteCtx {
    pub group: GroupId,
    pub open: VoteOpen,
    pub valid: Option<ValidOpen>,
    pub reject: Option<OpenReject>,
    pub members_of_h: Option<BTreeSet<IdentityKey>>,
    /// Up to two distinct payloads per key (D1 rule 3 proof retention).
    pub ballots: BTreeMap<IdentityKey, Vec<Ballot>>,
    pub tally: Option<Tally>,
}

/// Where an object this node sent was addressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    Group(GroupId),
    Peer(PeerId),
}

/// One object this node sent and tracks (BRIEF_v2 §5.9).
#[derive(Debug, Clone)]
pub(crate) struct OutboxEntry {
    pub conv: [u8; 16],
    pub obj_seq: u32,
    pub target: Target,
    pub kind: u8,
    pub created_t: u64,
    pub state: DeliveryState,
    pub cells: Vec<Hash32>,
    /// Every cell went out on at least one link.
    pub passed: bool,
}

/// Dirty flags: which blobs the next flush must write.
#[derive(Debug, Default, Clone)]
pub(crate) struct Dirty {
    pub recog: bool,
    pub pm: bool,
    pub groups: BTreeSet<GroupId>,
    pub outbox: bool,
    pub received: bool,
    pub heard: bool,
    pub bundles: bool,
}

/// Every piece of protocol state, all of it persisted (module docs).
#[derive(Debug)]
pub(crate) struct Proto {
    pub identity: Option<Identity>,
    pub recog: crate::wire::recog::TagTable,
    pub pm: Option<PairwiseManager>,
    pub groups: BTreeMap<GroupId, GroupCtx>,
    pub chains: SenderChainIndex,
    pub votes: BTreeMap<VoteId, VoteCtx>,
    /// Ballots whose opening has not arrived (≤ 2 per key).
    pub pending_ballots: BTreeMap<VoteId, BTreeMap<IdentityKey, Vec<Ballot>>>,
    pub outbox: BTreeMap<ObjId, OutboxEntry>,
    /// Own cell → object, for "Passed along".
    pub own_cells: BTreeMap<Hash32, ObjId>,
    /// Per conversation: object counters received, for cumulative receipts.
    pub received: BTreeMap<[u8; 16], BTreeSet<u32>>,
    pub bundles: BundleManager,
    pub reasm: Reasm,
    /// Powered-on time this node last recognised anything from a member.
    pub last_heard: BTreeMap<PeerId, u64>,
    pub contacts: BTreeMap<PeerId, ContactLite>,
    pub samename: SameNameIndex,
    /// SKDM entries that arrived pairwise before the roster named their
    /// sender (or before the session existed), per group and sender. Each
    /// plaintext holds a chain key and an epoch seed: wiped on drop.
    pub pending_skdm: BTreeMap<GroupId, BTreeMap<PeerId, zeroize::Zeroizing<Vec<u8>>>>,
    /// Dedupe marks not yet written to the `dedupe` namespace.
    pub pending_dedupe: Vec<(Hash32, u64, bool)>,
    pub dirty: Dirty,
}

impl Proto {
    pub fn me(&self) -> Option<PeerId> {
        self.identity.as_ref().map(|i| i.card.ed)
    }
}

/// Battery model input (BRIEF_v2 §5.7).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Power {
    pub level_ppm: u32,
    pub charging: bool,
}

/// Counters for tests and the simulator report.
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
    /// Link penalties applied (DECISIONS §3 row (1,2,3,4,10) (iii)).
    pub penalties: u64,
}

/// Relay-side state, not persisted.
pub(crate) struct Relay {
    pub store: CellStore,
    /// Bumped whenever a cell is stored: a link that was out of the map
    /// while cells were held (the link whose frames were being processed)
    /// re-pulls when it sees the change.
    pub store_version: u64,
    pub dedupe: DedupeCache,
    pub buffer: BufferPolicy,
    pub slots: SlotPolicy,
    pub links: BTreeMap<LinkId, LinkEnd>,
    pub power: Power,
    pub last_sync_t: Option<u64>,
    pub is_station: bool,
    pub battery_floor_ppm: u32,
}

impl Relay {
    /// BRIEF_v2 §5.7: below the floor the phone stops forwarding others'
    /// cells; stations are mains powered.
    pub fn relaying(&self) -> bool {
        self.is_station || self.power.charging || self.power.level_ppm >= self.battery_floor_ppm
    }
}

/// The single-threaded node (module docs).
pub struct Node {
    pub(crate) cfg: NodeConfig,
    pub(crate) db: Db,
    pub(crate) backend: Box<dyn Backend>,
    pub(crate) rng: ChaCha20Rng,
    pub(crate) proto: Proto,
    pub(crate) relay: Relay,
    pub(crate) meter: PoweredOnMeter,
    pub(crate) last_sweep_t: u64,
    pub(crate) events: VecDeque<Event>,
    pub(crate) stats: NodeStats,
    pub(crate) pairing: BTreeMap<LinkId, PairingSession>,
    /// The last `now_mono` the platform passed in.
    pub(crate) last_now: u64,
    /// Simulator adversary hook (docs/scenarios.md `keep_keys_after_removal`).
    pub(crate) adversary_keep_keys: bool,
    /// Simulator adversary hook (docs/scenarios.md `flood`): this node pushes
    /// through its outbound mirror budget and through the peer's `Throttle`
    /// holds (`links::LinkEnd::ignore_throttle`).
    pub(crate) adversary_ignore_throttle: bool,
    /// Simulator provisioning hook (`Node::set_provisioning`): in-person
    /// setup, every inbound cell admitted, no pacing.
    pub(crate) provisioning: bool,
    /// Penalties for a link that was out of the link map when its
    /// transaction committed (the link whose frames are being processed):
    /// applied by `process_link` once the link is back.
    pub(crate) pending_penalties: Vec<(LinkId, crate::relay::limits::Penalty)>,
    /// Receipt batching window (`Node::set_receipt_batch_ms`); 0 sends one
    /// receipt per delivered text.
    pub(crate) receipt_batch_ms: u64,
    /// Receipts due per (peer, conversation) at a powered-on time; not
    /// persisted (a restart sends the next receipt when the next object of
    /// the conversation arrives).
    pub(crate) pending_receipts: BTreeMap<(PeerId, [u8; 16]), u64>,
    /// Powered-on time of the last receipt sent per (peer, conversation).
    pub(crate) last_receipt: BTreeMap<(PeerId, [u8; 16]), u64>,
    display: Option<Box<dyn Clock>>,
    wall_reads: StdCell<u64>,
}

impl core::fmt::Debug for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Node")
            .field("role", &self.cfg.role)
            .field("groups", &self.proto.groups.len())
            .field("links", &self.relay.links.len())
            .field("store", &self.relay.store.len())
            .finish_non_exhaustive()
    }
}

/// Everything a transaction-scoped handler may touch.
pub(crate) struct Ctx<'a, 'b> {
    pub txn: &'a mut Txn<'b>,
    pub proto: &'a mut Proto,
    pub backend: &'a dyn Backend,
    pub rng: &'a mut ChaCha20Rng,
    pub cfg: &'a NodeConfig,
    pub stats: &'a mut NodeStats,
    /// Simulator adversary hook (`Node::set_adversary_keep_keys`).
    pub adversary_keep_keys: bool,
    /// Powered-on now.
    pub t: u64,
    /// Events to emit after the commit.
    pub events: Vec<Event>,
    /// Relay-side effects to apply after the commit (never before: a
    /// rolled-back cell must not be deduped or stored, so a re-fed copy
    /// yields exactly one delivery).
    pub after: Vec<After>,
    /// Cells originated in this transaction so far: own cells get strictly
    /// increasing `first_seen_mono` stamps so the push order within one
    /// command is the order they were cut (a first flight before the chain
    /// cells that follow it).
    pub seq: u64,
}

/// A relay-side effect deferred to after the commit.
#[derive(Debug)]
pub(crate) enum After {
    /// Store the cell as held by this node (and schedule it on pushing links).
    Store {
        cell: Box<crate::wire::cell::Cell>,
        meta: crate::dtn::store::CellMeta,
    },
    /// Dedupe mark (`rejected` → the summary vector carries it so peers stop
    /// re-offering it).
    Dedupe { id: Hash32, rejected: bool },
    /// Pin a held cell (a roster op: never evicted, D3 M1).
    Pin(Hash32),
    /// Unpin a held cell (an own object that was delivered).
    Unpin(Hash32),
    /// Penalise the source link (DECISIONS §3 row (1,2,3,4,10) (iii)).
    Penalise(LinkId, crate::relay::limits::Penalty),
    /// Acknowledge delivered objects to `peer` in `conv` (BRIEF_v2 §5.3
    /// "only then the receipt is scheduled").
    Receipt { peer: PeerId, conv: [u8; 16] },
    /// A pairwise session with `peer` became usable: groups owing it a
    /// chain rotate before their next post.
    SessionReady(PeerId),
    /// A receive epoch was registered: held cells that were opaque until
    /// now may be recognisable (DESIGN §3 "unrecognised cells stay in the
    /// DTN store and are re-tried whenever a chain/epoch is registered").
    Rescan,
}

impl Ctx<'_, '_> {
    pub fn me(&self) -> Result<PeerId, NodeError> {
        self.proto.me().ok_or(NodeError::NoIdentity)
    }

    pub fn identity(&self) -> Result<&Identity, NodeError> {
        self.proto.identity.as_ref().ok_or(NodeError::NoIdentity)
    }
}

impl Node {
    /// Opens a node on `db` (DESIGN §1 `Node::new`): creates the backend
    /// identity and the KeyCard on the first open (`backend.init_identity`,
    /// `KeyCard::issue`), restores every persisted manager on later opens
    /// ([`super::persist`]) and resumes the powered-on meter from
    /// `meta.powered_on_ms` at `now_mono`.
    pub fn open(
        mut db: Db,
        cfg: NodeConfig,
        now_mono: u64,
        mut rng: ChaCha20Rng,
    ) -> Result<Node, NodeError> {
        let backend = default_backend();
        let powered = db.meta_get_u64(meta_keys::POWERED_ON_MS)?.unwrap_or(0);
        let meter = PoweredOnMeter::resume(powered, now_mono);
        let t = meter.total_ms(now_mono);
        // First open of a member: identity + KeyCard + envelope key, in one
        // transaction.
        if let NodeRole::Member {
            identity_seed,
            display_name,
        } = &cfg.role
        {
            let mut txn = db.begin()?;
            if persist::load_identity(&txn)?.is_none() {
                let x25519 = backend.init_identity(&mut txn, &mut rng)?;
                let signer = IdentitySigner::from_seed(identity_seed);
                let card = KeyCard::issue(&signer, x25519);
                let envelope = EnvelopeKeyPair::generate(&mut rng);
                persist::save_identity(&mut txn, identity_seed, display_name, &envelope);
                persist::save_bundles(&mut txn, &BundleManager::new(0));
                let _ = card;
                txn.commit()?;
            } else {
                drop(txn);
            }
        }
        let proto = persist::load(&mut db, &*backend, &cfg, t)?;
        let is_station = matches!(cfg.role, NodeRole::Station);
        let mut dedupe = DedupeCache::new(cfg.dedupe_retention_ms, DEDUPE_MAX_ENTRIES_ASM);
        {
            // The persisted dedupe marks (an unexpired mark is re-armed for
            // a full window: the cache cannot take an explicit expiry).
            let txn = db.begin()?;
            for (id, expires, rejected) in persist::load_dedupe(&txn) {
                if expires > t {
                    if rejected {
                        dedupe.mark_rejected(id, t);
                    } else {
                        dedupe.seen_or_insert(id, t);
                    }
                }
            }
            drop(txn);
        }
        let relay = Relay {
            store: CellStore::new(cfg.buffer.caps(), Box::new(cfg.buffer)),
            store_version: 0,
            dedupe,
            buffer: cfg.buffer,
            slots: SlotPolicy::new(cfg.max_links),
            links: BTreeMap::new(),
            power: Power {
                level_ppm: 1_000_000,
                charging: false,
            },
            last_sync_t: None,
            is_station,
            battery_floor_ppm: cfg.battery_floor_ppm,
        };
        // A different stream per open so two opens never reuse nonces.
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        let rng = ChaCha20Rng::from_seed(seed);
        Ok(Node {
            cfg,
            db,
            backend,
            rng,
            proto,
            relay,
            meter,
            last_sweep_t: t,
            events: VecDeque::new(),
            stats: NodeStats::default(),
            pairing: BTreeMap::new(),
            last_now: now_mono,
            adversary_keep_keys: false,
            adversary_ignore_throttle: false,
            provisioning: false,
            pending_penalties: Vec::new(),
            receipt_batch_ms: 0,
            pending_receipts: BTreeMap::new(),
            last_receipt: BTreeMap::new(),
            display: None,
            wall_reads: StdCell::new(0),
        })
    }

    /// The display hook: the only path to wall time. Protocol code never
    /// calls it; every read is counted ([`Node::wall_reads`]).
    pub fn set_display_clock(&mut self, clock: Box<dyn Clock>) {
        self.display = Some(clock);
    }

    /// Reads of the display clock so far (the simulator asserts zero for
    /// protocol runs, DESIGN §10 S4 (e)).
    pub fn wall_reads(&self) -> u64 {
        self.wall_reads.get()
    }

    /// Wall-clock milliseconds for display, if a display clock is set.
    pub(crate) fn wall_for_display(&self) -> Option<i64> {
        let clock = self.display.as_ref()?;
        self.wall_reads.set(self.wall_reads.get().saturating_add(1));
        Some(clock.wall_for_display())
    }

    /// Powered-on time for the platform reading `now_mono`.
    pub fn powered_on_ms(&self, now_mono: u64) -> u64 {
        self.meter.total_ms(now_mono)
    }

    pub(crate) fn t(&self, now_mono: u64) -> u64 {
        self.meter.total_ms(now_mono)
    }

    pub fn config(&self) -> &NodeConfig {
        &self.cfg
    }

    pub fn stats(&self) -> NodeStats {
        self.stats
    }

    /// This node's identity key (`None` for a station).
    pub fn identity(&self) -> Option<IdentityKey> {
        self.proto.me()
    }

    /// This node's KeyCard (BRIEF_v2 §3 "one contact = one key").
    pub fn my_keycard(&self) -> Result<KeyCard, NodeError> {
        Ok(self
            .proto
            .identity
            .as_ref()
            .ok_or(NodeError::NoIdentity)?
            .card)
    }

    /// This node's envelope public key for sealed first flights.
    pub fn my_envelope_pub(&self) -> Result<[u8; 32], NodeError> {
        Ok(self
            .proto
            .identity
            .as_ref()
            .ok_or(NodeError::NoIdentity)?
            .envelope
            .public())
    }

    pub fn display_name(&self) -> Option<&str> {
        self.proto
            .identity
            .as_ref()
            .map(|i| i.display_name.as_str())
    }

    /// Fault-injection passthrough (DESIGN §10 "kill between decrypt and
    /// commit").
    pub fn set_fail_before_commit(&mut self, fail: bool) {
        self.db.set_fail_before_commit(fail);
    }

    /// Hands the database back (tests reopen the same store with
    /// [`Node::open`] to check persistence).
    pub fn into_db(self) -> Db {
        self.db
    }

    /// Simulator adversary hook (docs/scenarios.md `adversary` behaviour
    /// `keep_keys_after_removal`; DESIGN §10 S3): this node ignores the
    /// rotation trigger on removals and keeps posting under its old chain
    /// even when the roster no longer lists it. Honest nodes are expected to
    /// reject its posts whose heads include its removal and to label the
    /// others; nothing in a stock client enables this.
    pub fn set_adversary_keep_keys(&mut self, on: bool) {
        self.adversary_keep_keys = on;
    }

    /// Simulator adversary hook (docs/scenarios.md `flood`; DESIGN §10
    /// S-flood): holds `n` junk cells (random tag and body, drawn TTL) as
    /// this node's own, so they are pushed over every link. Returns their
    /// ids.
    pub fn adversary_inject_junk(&mut self, n: usize, now_mono: u64) -> Vec<Hash32> {
        let t = self.t(now_mono);
        let mut ids = Vec::with_capacity(n);
        for _ in 0..n {
            let ttl = crate::wire::cell::draw_initial_ttl(&mut self.rng);
            let mut tag = [0u8; crate::wire::cell::TAG_LEN];
            self.rng.fill_bytes(&mut tag);
            let mut body = [0u8; crate::wire::cell::BODY_LEN];
            self.rng.fill_bytes(&mut body);
            let cell = crate::wire::cell::build(crate::wire::cell::VERSION, ttl, &tag, &body);
            let id = cell.msg_id();
            let meta = crate::dtn::store::CellMeta {
                class: crate::wire::frame::Class::C2Text,
                lane: crate::relay::Lane::L1,
                from_link: None,
                first_seen_mono: t,
                pinned: false,
            };
            self.relay.dedupe.seen_or_insert(id, t);
            if self.relay.store.put(cell, meta) == crate::dtn::store::Put::Stored {
                self.relay.store_version = self.relay.store_version.wrapping_add(1);
                self.forward_live(id, meta.class, meta.lane, None);
                ids.push(id);
            }
        }
        ids
    }

    /// Simulator adversary hook (docs/scenarios.md `flood`; DESIGN §10
    /// S-flood "injecting 600 junk cells/min"): when on, this node ignores
    /// its outbound mirror budget and every `Throttle` hold its peers send,
    /// so what reaches the honest nodes is bounded only by their inbound
    /// budgets (BRIEF_v2 §5.5 "rate limits per inbound link"), never by the
    /// sender's good manners. Nothing in a stock client enables this.
    pub fn set_adversary_ignore_throttle(&mut self, on: bool) {
        self.adversary_ignore_throttle = on;
        self.set_pacing_off();
    }

    /// Simulator provisioning hook (docs/scenarios.md `[groups]` "created at
    /// t=0 by admin 0 with everyone pre-verified in person", `[pairs]`
    /// "QR-verified 1:1 contacts at t=0"): while on, this node admits every
    /// inbound cell without the per-link budget and pushes without pacing,
    /// as the direct in-person key exchange of a setup session does. The
    /// simulator turns it on for every node during its setup phase (before
    /// the run's first event) and off before the radio takes over; nothing
    /// in a stock client enables it.
    pub fn set_provisioning(&mut self, on: bool) {
        self.provisioning = on;
        self.set_pacing_off();
    }

    /// Whether this node is pacing nothing (either hook on).
    pub(crate) fn pacing_off(&self) -> bool {
        self.adversary_ignore_throttle || self.provisioning
    }

    fn set_pacing_off(&mut self) {
        let off = self.pacing_off();
        for end in self.relay.links.values_mut() {
            end.ignore_throttle = off;
        }
    }

    /// Simulator adversary hook (docs/scenarios.md `replay_cells`; DESIGN
    /// §10 S3 (d) "replays are dropped by dedupe/AEAD, never redelivered"):
    /// re-offers up to `max` of the oldest cells this node received from
    /// peers on every pushing link, bypassing the peer's summary vector as a
    /// replaying node would. Returns the number of (cell, link) offers
    /// queued. Nothing in a stock client calls this.
    pub fn adversary_replay_held(&mut self, max: usize) -> usize {
        let mut cells: Vec<(u64, Hash32, crate::wire::frame::Class, crate::relay::Lane)> = self
            .relay
            .store
            .iter()
            .filter(|(_, _, m)| m.from_link.is_some())
            .map(|(id, _, m)| (m.first_seen_mono, *id, m.class, m.lane))
            .collect();
        cells.sort_unstable();
        cells.truncate(max);
        let mut queued = 0usize;
        for end in self.relay.links.values_mut() {
            if !end.is_pushing() {
                continue;
            }
            for (_, id, class, lane) in &cells {
                if end.sched.enqueue(*id, *class, *lane, None) {
                    queued = queued.saturating_add(1);
                }
            }
        }
        queued
    }

    /// Rows in the key-value store (`Db::kv_len`), for the simulator's
    /// bookkeeping checks (DESIGN §10 S1 (g): per-message keys never
    /// accumulate as rows; the backend consumes them inside its session and
    /// sender-key records on decrypt, D6 rule 4).
    pub fn kv_len(&self) -> Result<u64, NodeError> {
        Ok(self.db.kv_len()?)
    }

    /// Re-runs recognition over every held cell after a receive epoch was
    /// registered (DESIGN §3): cells that were opaque and now match a chain
    /// go through the decrypt path, each in its own transaction.
    fn rescan_store(&mut self, now_mono: u64) {
        let candidates: Vec<(Hash32, crate::wire::cell::Cell, crate::dtn::store::CellMeta)> = self
            .relay
            .store
            .iter()
            .filter(|(_, cell, meta)| {
                meta.from_link.is_some() && self.proto.recog.lookup(&cell.tag()).is_some()
            })
            .map(|(id, cell, meta)| (*id, cell.clone(), *meta))
            .collect();
        for (_, cell, meta) in candidates {
            self.reprocess_stored(cell, meta, now_mono);
        }
    }

    /// Receipt batching window in powered-on milliseconds (DECISIONS D4
    /// MERGE "cumulative per recipient per conversation", D4 pick B (5)
    /// "receipts batched"): after a receipt to a peer for a conversation,
    /// the next one waits this long and acknowledges everything received by
    /// then. 0 (the default) sends one receipt per delivered text; the
    /// simulator sets [`RECEIPT_BATCH_MS_ASM`].
    pub fn set_receipt_batch_ms(&mut self, window_ms: u64) {
        self.receipt_batch_ms = window_ms;
    }

    /// Battery input (BRIEF_v2 §5.7): below the floor and not charging, the
    /// node stops forwarding others' cells but keeps its own traffic.
    pub fn set_power(&mut self, level_ppm: u32, charging: bool) {
        self.relay.power = Power {
            level_ppm,
            charging,
        };
    }

    /// Drains the events produced so far (none is produced before its
    /// transaction committed).
    pub fn poll_events(&mut self) -> Vec<Event> {
        self.events.drain(..).collect()
    }

    /// Cells held in the DTN store.
    pub fn store_len(&self) -> usize {
        self.relay.store.len()
    }

    /// Bytes held in the DTN store.
    pub fn store_bytes(&self) -> usize {
        self.relay.store.bytes()
    }

    /// Whether the store holds the cell `msg_id`.
    pub fn holds(&self, msg_id: &Hash32) -> bool {
        self.relay.store.contains(msg_id)
    }

    /// Held cells that are pinned (roster-op carriers, undelivered own objects).
    pub fn pinned_cells(&self) -> Vec<Hash32> {
        self.relay
            .store
            .iter()
            .filter(|(_, _, m)| m.pinned)
            .map(|(id, _, _)| *id)
            .collect()
    }

    /// Whether the dedupe cache remembers `msg_id` as rejected (a post whose
    /// heads include its author's removal, DECISIONS §3 row (1,2,3,4,10)).
    pub fn is_rejected(&self, msg_id: &Hash32) -> bool {
        self.relay.dedupe.is_rejected(msg_id)
    }

    /// Whether a held cell is pinned (a roster op carrier).
    pub fn is_pinned(&self, msg_id: &Hash32) -> bool {
        self.relay.store.get(msg_id).is_some_and(|(_, m)| m.pinned)
    }

    // ---- transactions -------------------------------------------------------

    /// Runs `f` inside one transaction (module docs "Transactions"). On
    /// success the dirty blobs are flushed, the transaction committed, the
    /// deferred relay effects applied and the events emitted, in that order.
    /// On any error the transaction is dropped and the protocol state is
    /// replayed from the database.
    pub(crate) fn run_txn<R>(
        &mut self,
        now_mono: u64,
        f: impl FnOnce(&mut Ctx<'_, '_>) -> Result<R, NodeError>,
    ) -> Result<R, NodeError> {
        self.last_now = now_mono;
        let t = self.t(now_mono);
        let reasm_snapshot = self.proto.reasm.clone();
        let (result, events, after) = {
            let adversary_keep_keys = self.adversary_keep_keys;
            let Node {
                db,
                proto,
                backend,
                rng,
                cfg,
                stats,
                ..
            } = self;
            let mut txn = db.begin()?;
            let mut ctx = Ctx {
                txn: &mut txn,
                proto,
                backend: &**backend,
                rng,
                cfg,
                stats,
                adversary_keep_keys,
                t,
                events: Vec::new(),
                after: Vec::new(),
                seq: 0,
            };
            let result = f(&mut ctx);
            let Ctx { events, after, .. } = ctx;
            let result = result.and_then(|r| {
                persist::flush(&mut txn, proto)?;
                txn.commit()?;
                Ok(r)
            });
            (result, events, after)
        };
        match result {
            Ok(r) => {
                self.stats.commits = self.stats.commits.saturating_add(1);
                self.apply_after(after, now_mono);
                self.events.extend(events);
                Ok(r)
            }
            Err(e) => {
                self.stats.rollbacks = self.stats.rollbacks.saturating_add(1);
                self.rollback(reasm_snapshot, t)?;
                Err(e)
            }
        }
    }

    /// Replays the protocol state from the last commit (module docs).
    fn rollback(&mut self, reasm: Reasm, t: u64) -> Result<(), NodeError> {
        let mut proto = persist::load(&mut self.db, &*self.backend, &self.cfg, t)?;
        proto.reasm = reasm;
        self.proto = proto;
        Ok(())
    }

    /// Applies the deferred relay effects of a committed transaction.
    fn apply_after(&mut self, after: Vec<After>, now_mono: u64) {
        let t = self.t(now_mono);
        let mut receipts = Vec::new();
        let mut ready = Vec::new();
        let mut rescan = false;
        for a in after {
            match a {
                After::Store { cell, meta } => {
                    let cell = *cell;
                    let id = cell.msg_id();
                    self.relay.dedupe.seen_or_insert(id, t);
                    self.proto.pending_dedupe.push((
                        id,
                        t.saturating_add(self.relay.dedupe.retention_ms()),
                        false,
                    ));
                    match self.relay.store.put(cell, meta) {
                        crate::dtn::store::Put::Stored => {
                            self.relay.store_version = self.relay.store_version.wrapping_add(1);
                            self.forward_live(id, meta.class, meta.lane, meta.from_link);
                        }
                        crate::dtn::store::Put::Duplicate => {}
                        crate::dtn::store::Put::NoRoom => {
                            self.stats.no_room = self.stats.no_room.saturating_add(1);
                        }
                    }
                }
                After::Dedupe { id, rejected } => {
                    if rejected {
                        self.relay.dedupe.mark_rejected(id, t);
                    } else {
                        self.relay.dedupe.seen_or_insert(id, t);
                    }
                    self.proto.pending_dedupe.push((
                        id,
                        t.saturating_add(self.relay.dedupe.retention_ms()),
                        rejected,
                    ));
                }
                After::Pin(id) => {
                    self.relay.store.set_pinned(&id, true);
                }
                After::Unpin(id) => {
                    self.relay.store.set_pinned(&id, false);
                }
                After::Penalise(link, why) => {
                    if let Some(end) = self.relay.links.get_mut(&link) {
                        end.inbound_budget.penalise(why, t);
                        self.stats.penalties = self.stats.penalties.saturating_add(1);
                    } else {
                        // The source link is out of the map while its own
                        // frames are processed (`links::process_link`).
                        self.pending_penalties.push((link, why));
                    }
                }
                After::Receipt { peer, conv } => receipts.push((peer, conv)),
                After::SessionReady(peer) => ready.push(peer),
                After::Rescan => rescan = true,
            }
        }
        if rescan {
            self.rescan_store(now_mono);
        }
        for peer in ready {
            self.on_session_ready(peer, now_mono);
        }
        let window = self.receipt_batch_ms;
        for (peer, conv) in receipts {
            // Scheduled only after the delivering transaction committed
            // (BRIEF_v2 §5.3). With a batching window, a receipt that would
            // follow the previous one within the window waits for the
            // window's end and acknowledges everything received by then
            // (`Node::set_receipt_batch_ms`); its own transaction,
            // failures are not fatal.
            let key = (peer, conv);
            let earliest = self
                .last_receipt
                .get(&key)
                .map_or(0, |last| last.saturating_add(window));
            if window == 0 || t >= earliest {
                self.last_receipt.insert(key, t);
                let _ = self.send_receipt(peer, conv, t);
            } else {
                self.pending_receipts.entry(key).or_insert(earliest);
            }
        }
    }

    /// Sends the batched receipts that are due (`Node::set_receipt_batch_ms`).
    fn flush_receipts(&mut self, t: u64) {
        let due: Vec<(PeerId, [u8; 16])> = self
            .pending_receipts
            .iter()
            .filter(|(_, when)| **when <= t)
            .map(|(k, _)| *k)
            .collect();
        for key in due {
            self.pending_receipts.remove(&key);
            self.last_receipt.insert(key, t);
            let _ = self.send_receipt(key.0, key.1, t);
        }
    }

    /// Queues a newly held cell on every pushing link whose initial pull is
    /// over (links still pulling will find it in the store); own cells are
    /// forwarded even below the battery floor (BRIEF_v2 §5.7).
    pub(crate) fn forward_live(
        &mut self,
        id: Hash32,
        class: crate::wire::frame::Class,
        lane: crate::relay::Lane,
        from: Option<LinkId>,
    ) {
        let own = from.is_none();
        if !own && !self.relay.relaying() {
            return;
        }
        for end in self.relay.links.values_mut() {
            if Some(end.id) == from {
                continue;
            }
            if end.is_pushing() && end.pull_exhausted {
                end.sched.enqueue(id, class, lane, from);
            } else {
                end.dirty = true;
            }
        }
    }

    // ---- tick -------------------------------------------------------------

    /// One node tick (DESIGN §1 "tick scheduler"): the powered-on meter is
    /// checkpointed, the dedupe cache swept, SPK rotation and retirement run
    /// (D6 rule 3), roster rotations become due (D3 K2), delivery states
    /// age (BRIEF_v2 §5.9) and links with held work are serviced. Returns the
    /// events produced.
    pub fn tick(&mut self, now_mono: u64) -> Vec<Event> {
        let t = self.t(now_mono);
        let sweep_due = t.saturating_sub(self.last_sweep_t) >= SWEEP_MS_ASM;
        if sweep_due {
            self.last_sweep_t = t;
            self.relay.dedupe.expire(t);
        }
        // Rotation interval (trigger (b)) per group.
        for g in self.proto.groups.values_mut() {
            if g.policy.due(t) && !g.state.needs_rotation() {
                g.state.mark_rotation_pending();
            }
        }
        // Delivery ageing.
        let held = self.relay.links.values().any(|l| l.sched.busy_hint());
        let mut changed = Vec::new();
        for (obj, e) in self.proto.outbox.iter_mut() {
            let next = age_transition(e.state, t.saturating_sub(e.created_t), e.passed, held);
            if next != e.state {
                e.state = next;
                changed.push((*obj, next));
            }
        }
        if !changed.is_empty() {
            self.proto.dirty.outbox = true;
        }
        self.flush_receipts(t);
        let maintenance = sweep_due || !changed.is_empty() || !self.proto.pending_dedupe.is_empty();
        if maintenance {
            let powered = t;
            let do_spk = sweep_due && self.proto.identity.is_some();
            let res = self.run_txn(now_mono, |ctx| {
                ctx.txn.meta_set_u64(meta_keys::POWERED_ON_MS, powered)?;
                if do_spk {
                    super::outbound::spk_maintenance(ctx)?;
                }
                for (obj, state) in &changed {
                    ctx.events.push(Event::Delivery {
                        obj: *obj,
                        state: *state,
                    });
                }
                Ok(())
            });
            if res.is_err() {
                // The meter checkpoint is retried on the next tick; the
                // in-memory states were reloaded from the last commit.
            }
        }
        self.service_links(now_mono);
        self.poll_events()
    }

    // ---- queries ------------------------------------------------------------

    /// `members(H)` at the group's current heads.
    pub fn roster(&self, group: &GroupId) -> Result<RosterView, NodeError> {
        Ok(self
            .proto
            .groups
            .get(group)
            .ok_or(NodeError::UnknownGroup)?
            .view
            .clone())
    }

    /// Current member keys of the group, sorted.
    pub fn members(&self, group: &GroupId) -> Result<Vec<IdentityKey>, NodeError> {
        Ok(self.roster(group)?.members.keys().copied().collect())
    }

    /// Groups this node holds a roster for.
    pub fn groups(&self) -> Vec<GroupId> {
        self.proto.groups.keys().copied().collect()
    }

    /// The group's current roster heads.
    pub fn roster_heads(&self, group: &GroupId) -> Result<Vec<Hash32>, NodeError> {
        Ok(self
            .proto
            .groups
            .get(group)
            .ok_or(NodeError::UnknownGroup)?
            .dag
            .heads())
    }

    /// Whether this node's next post in the group rotates its Sender Key.
    pub fn rotation_pending(&self, group: &GroupId) -> Result<bool, NodeError> {
        let g = self
            .proto
            .groups
            .get(group)
            .ok_or(NodeError::UnknownGroup)?;
        Ok(g.state.needs_rotation())
    }

    /// A task's folded state and its UI string.
    pub fn task(
        &self,
        group: &GroupId,
        task_id: &TaskId,
    ) -> Result<(TaskState, String), NodeError> {
        let g = self
            .proto
            .groups
            .get(group)
            .ok_or(NodeError::UnknownGroup)?;
        let t = g.tasks.get(task_id).ok_or(NodeError::UnknownTask)?;
        let state = t.state.clone().ok_or(NodeError::UnknownTask)?;
        Ok((state, t.ui.clone()))
    }

    /// Tasks of a group, by id.
    pub fn tasks(&self, group: &GroupId) -> Result<Vec<TaskId>, NodeError> {
        let g = self
            .proto
            .groups
            .get(group)
            .ok_or(NodeError::UnknownGroup)?;
        Ok(g.tasks.keys().copied().collect())
    }

    /// The "Needs someone" pool (BRIEF_v2 §4; D10 (6); docs/decisions-assumed.md
    /// row 7): tasks whose holders have not been heard from for
    /// `NEEDS_SOMEONE_AFTER_ASM` of powered-on time, with the evidence age.
    pub fn needs_someone(
        &self,
        group: &GroupId,
        now_mono: u64,
    ) -> Result<Vec<(TaskId, EvidenceAge)>, NodeError> {
        let t = self.t(now_mono);
        let g = self
            .proto
            .groups
            .get(group)
            .ok_or(NodeError::UnknownGroup)?;
        let mut out = Vec::new();
        for (id, task) in &g.tasks {
            let (Some(state), Some(taken_at)) = (task.state.as_ref(), task.taken_at) else {
                continue;
            };
            if let Some(age) =
                crate::tasks::pool::needs_someone(state, &self.proto.last_heard, taken_at, t)
            {
                out.push((*id, age));
            }
        }
        Ok(out)
    }

    /// A vote's tally and its compact line ("Y10 N10 · 1 void ·
    /// Provisional"), or the waiting label while its member list is unknown.
    pub fn vote(&self, vote_id: &VoteId) -> Result<(Option<Tally>, String), NodeError> {
        let v = self
            .proto
            .votes
            .get(vote_id)
            .ok_or(NodeError::UnknownVote)?;
        Ok((v.tally.clone(), vote_line(v)))
    }

    /// Votes this node holds, by id.
    pub fn votes(&self) -> Vec<VoteId> {
        self.proto.votes.keys().copied().collect()
    }

    /// Delivery state of an object this node sent.
    pub fn delivery(&self, obj: &ObjId) -> Result<DeliveryState, NodeError> {
        Ok(self
            .proto
            .outbox
            .get(obj)
            .ok_or(NodeError::UnknownObject)?
            .state)
    }

    /// Objects this node sent, by id.
    pub fn outbox(&self) -> Vec<ObjId> {
        self.proto.outbox.keys().copied().collect()
    }

    /// The decrypted text of a stored object (sent or received).
    pub fn plaintext(&mut self, obj: &ObjId) -> Result<Option<String>, NodeError> {
        let txn = self.db.begin()?;
        let rec = persist::load_plaintext(&txn, obj)?;
        drop(txn);
        Ok(rec.and_then(|r| {
            if r.kind == crate::objects::chat_post::kind::POST_TEXT {
                crate::codec::decode::<crate::objects::chat_post::PostPlain>(&r.plaintext)
                    .ok()
                    .and_then(|p| p.text_body().ok())
                    .map(|b| b.text)
            } else {
                None
            }
        }))
    }

    /// The pinned contacts, in key order.
    pub fn contacts(&self) -> Vec<IdentityKey> {
        self.proto.contacts.keys().copied().collect()
    }

    /// The session state with a contact (`NeedsTheirKey` for an unknown one).
    pub fn session_state(&self, peer: &IdentityKey) -> crate::pairwise::session::SessionState {
        self.proto.pm.as_ref().map_or(
            crate::pairwise::session::SessionState::NeedsTheirKey,
            |pm| pm.state(peer),
        )
    }

    /// The same-name marker text for a contact, if any.
    pub fn same_name_marker(&self, key: &IdentityKey) -> Option<String> {
        let name = self.proto.contacts.get(key)?.display_name.clone();
        self.proto.samename.marker_for(key).map(|m| m.text(&name))
    }

    /// The status strip (BRIEF_v2 §2).
    pub fn status_strip(&self, now_mono: u64) -> StatusStrip {
        let t = self.t(now_mono);
        let nearby = self.relay.links.values().filter(|l| l.is_up()).count();
        let network_busy = self.relay.links.values().any(|l| l.sched.busy_hint());
        let last_sync_age_ms = self.relay.last_sync_t.map(|s| t.saturating_sub(s));
        let last_sync_display = match (last_sync_age_ms, self.display.as_ref()) {
            (Some(age), Some(_)) => self
                .wall_for_display()
                .map(|w| w.saturating_sub(i64::try_from(age).unwrap_or(i64::MAX))),
            _ => None,
        };
        StatusStrip {
            nearby,
            passing_along: self.relay.relaying() && nearby > 0,
            last_sync_display,
            last_sync_age_ms,
            battery_floor: !self.relay.relaying(),
            network_busy,
        }
    }

    /// Display name for a key: the roster or contact name, else "Unverified"
    /// (`roster::labels::UNNAMED`); never invented from the key bytes.
    pub fn name_of(&self, key: &IdentityKey) -> String {
        if let Some(me) = self.proto.identity.as_ref()
            && me.card.ed == *key
        {
            return me.display_name.clone();
        }
        for g in self.proto.groups.values() {
            if let Some(m) = g.view.members.get(key)
                && !m.name.is_empty()
            {
                return m.name.clone();
            }
        }
        self.proto
            .contacts
            .get(key)
            .map(|c| c.display_name.clone())
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| crate::roster::labels::UNNAMED.to_owned())
    }
}

/// The vote card's compact line for a stored vote.
pub(crate) fn vote_line(v: &VoteCtx) -> String {
    match (&v.valid, &v.tally) {
        (Some(open), Some(tally)) => crate::votes::state::tally_line(open, tally),
        (_, _) if v.reject.is_some() => "Vote void".to_owned(),
        _ => crate::votes::state::WAITING_LABEL.to_owned(),
    }
}

/// Conversation id of a group chain: `Trunc16(SHA256("meshapp.v1.conv.group"
/// ‖ 0x00 ‖ group ‖ LE32(chain)))` (receipts are per chain, so `obj_seq` is
/// unambiguous across rotations).
pub fn group_conv(group: &GroupId, chain: crate::crypto::backend::ChainId) -> [u8; 16] {
    crate::crypto::hash::sha256_concat(&[
        b"meshapp.v1.conv.group",
        &[0u8],
        group.as_bytes(),
        &chain.0.to_le_bytes(),
    ])
    .truncate16()
}

/// Conversation id of a 1:1 thread: `Trunc16(SHA256("meshapp.v1.conv.dm" ‖
/// 0x00 ‖ min(a,b) ‖ max(a,b)))`.
pub fn dm_conv(a: &IdentityKey, b: &IdentityKey) -> [u8; 16] {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    crate::crypto::hash::sha256_concat(&[
        b"meshapp.v1.conv.dm",
        &[0u8],
        lo.as_bytes(),
        hi.as_bytes(),
    ])
    .truncate16()
}

/// Author-bound task op hash (DESIGN §2.4; `objects::task_op`):
/// `SHA256("meshapp.v1.taskop" ‖ 0x00 ‖ author ‖ post plaintext)`, the
/// identity of a task op and the value later ops cite in `task_heads`.
pub use crate::objects::task_op::task_op_hash;
