//! [`MeshNode`]: the plain-Rust facade a later uniffi export wraps
//! (docs/DESIGN.md §1 row `api/node.rs`; §7 "The core never blocks"; §8
//! `MeshApplication.kt` creates `MeshNode` with the keystore DB key;
//! BRIEF_v2 §2 screens; §5.3 transactions; §5.7 battery mode).
//!
//! One [`MeshNode`] owns one [`Node`](crate::node::Node) behind a
//! `std::sync::Mutex`. Every method takes `&self`, owned FFI-safe arguments
//! and returns owned values or [`ApiError`], so the uniffi object maps one
//! to one. The platform's threads (BLE callbacks, the UI, a tick timer) may
//! call concurrently; the lock serialises them and is never held while a
//! callback into the platform runs:
//!
//! * events produced by a call are drained from the node **after** its
//!   transaction committed and **before** the lock is released, then
//!   handed to the sink (or queued for [`MeshNode::poll_events`]) with the
//!   lock released, so a sink may call back into the node without
//!   deadlocking;
//! * the display clock ([`MeshNode::set_display_clock`]) is the only path
//!   to wall time and every read of it is counted
//!   ([`MeshNode::wall_reads`]); protocol decisions use the `now_mono`
//!   the platform passes to each call (BRIEF_v2 §4 "no timestamps decide
//!   anything").
//!
//! Lifecycle: [`MeshNode::open`] (stopped) → [`MeshNode::start`] →
//! radio calls (`link_up`, `feed`, `poll_outbound`, `tick`) →
//! [`MeshNode::stop`] (every link down). Commands and queries work in
//! either state: a text sent while stopped waits in the outbox
//! ("Offline – will send later", BRIEF_v2 §7).

use std::collections::VecDeque;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use zeroize::Zeroizing;

use crate::clock::Clock;
use crate::codec;
use crate::crypto::hash::hkdf32;
use crate::node::{AddDraft, Event, Node};
use crate::pairwise::pairing::QrCode;
use crate::storage::Db;

use super::errors::ApiError;
use super::events::ApiEvent;
use super::types::{
    ApiAddDraft, ApiConfig, ApiContact, ApiDeliveryState, ApiEndorsement, ApiImportedContact,
    ApiIssueContext, ApiLink, ApiLinkRole, ApiMemberEntry, ApiNeedsSomeone, ApiOutboxEntry,
    ApiPairingStep, ApiRole, ApiRoster, ApiSessionState, ApiStats, ApiStatusStrip, ApiTally,
    ApiTask, ApiTaskState, ApiVote, WallClock, group_bytes, group_from, hash_bytes, hash_from,
    key_bytes, key_from, keycard_from, link_id, task_id_from,
};

/// HKDF salt under which [`MeshNode::open`] derives its secrets from the
/// platform entropy.
const ENTROPY_SALT: &[u8] = b"meshapp.v1.api.entropy";
/// Fewest entropy bytes [`MeshNode::open`] accepts.
pub const MIN_ENTROPY_BYTES: usize = 32;

/// An event sink: called once per event, outside the node lock (uniffi:
/// a callback interface with one method).
pub type EventSink = Box<dyn Fn(ApiEvent) + Send>;

struct Inner {
    node: Node,
    running: bool,
}

/// The installed sink and a counter bumped by every
/// [`MeshNode::set_event_sink`], so a dispatch in progress can tell that
/// the sink it holds was replaced or removed from inside the callback.
struct SinkSlot {
    sink: Option<EventSink>,
    generation: u64,
}

/// The facade (module docs). uniffi: `Object` with a constructor
/// ([`MeshNode::open`]) and `&self` methods.
pub struct MeshNode {
    inner: Mutex<Inner>,
    /// The sink is *taken out* of this slot while it runs, so a re-entrant
    /// [`MeshNode::set_event_sink`] from inside the sink never blocks.
    sink: Mutex<SinkSlot>,
    /// Events produced while no sink was installed (or while it was busy).
    queue: Mutex<VecDeque<ApiEvent>>,
}

impl core::fmt::Debug for MeshNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut d = f.debug_struct("MeshNode");
        match self.inner.try_lock() {
            Ok(g) => {
                d.field("node", &g.node).field("running", &g.running);
            }
            Err(_) => {
                d.field("node", &"<locked>");
            }
        }
        d.finish_non_exhaustive()
    }
}

/// The display-clock adaptor: the node only ever asks it for wall time.
struct WallAdaptor(Box<dyn WallClock>);

impl Clock for WallAdaptor {
    fn mono_ms(&self) -> u64 {
        // Never consulted: powered-on time comes from the `now_mono` the
        // platform passes to every call.
        0
    }
    fn wall_for_display(&self) -> i64 {
        self.0.wall_now_ms()
    }
}

impl MeshNode {
    // ---- lifecycle --------------------------------------------------------------

    /// Opens (creating on the first open) the node's database and state.
    /// uniffi: the object's constructor.
    ///
    /// * `db_path`: the SQLite file (`None` = a private in-memory database
    ///   for tests and the simulator);
    /// * `db_key`: the 32-byte application key from the platform keystore
    ///   (DESIGN §6; wiped before this returns);
    /// * `entropy`: at least [`MIN_ENTROPY_BYTES`] of fresh OS randomness
    ///   on **every** open (the core never reads the OS); the database
    ///   nonce stream, the node's RNG and, on the first open of a member,
    ///   the identity seed are derived from it (HKDF, DESIGN §6);
    /// * `config`: see [`ApiConfig`]; on a reopen the stored identity and
    ///   display name win over `config.role`;
    /// * `now_mono`: the platform's monotonic milliseconds at this call
    ///   (the powered-on meter resumes from the stored total).
    pub fn open(
        db_path: Option<String>,
        db_key: Vec<u8>,
        entropy: Vec<u8>,
        config: ApiConfig,
        now_mono: u64,
    ) -> Result<MeshNode, ApiError> {
        let db_key = Zeroizing::new(db_key);
        let entropy = Zeroizing::new(entropy);
        let key: Zeroizing<[u8; 32]> = Zeroizing::new(
            db_key
                .as_slice()
                .try_into()
                .map_err(|_| ApiError::invalid("db_key must be 32 bytes"))?,
        );
        if entropy.len() < MIN_ENTROPY_BYTES {
            return Err(ApiError::invalid("entropy must be at least 32 bytes"));
        }
        let db_entropy = Zeroizing::new(hkdf32(&entropy, ENTROPY_SALT, b"db"));
        let identity_seed = Zeroizing::new(hkdf32(&entropy, ENTROPY_SALT, b"identity"));
        let rng_seed = Zeroizing::new(hkdf32(&entropy, ENTROPY_SALT, b"rng"));
        let cfg = config.to_node_config(&identity_seed)?;
        let db = match db_path {
            Some(p) => Db::open(Path::new(&p), *key, *db_entropy),
            None => Db::open_in_memory(*key, *db_entropy),
        }
        .map_err(|e| ApiError::Storage { msg: e.to_string() })?;
        let node = Node::open(db, cfg, now_mono, ChaCha20Rng::from_seed(*rng_seed))?;
        Ok(MeshNode {
            inner: Mutex::new(Inner {
                node,
                running: false,
            }),
            sink: Mutex::new(SinkSlot {
                sink: None,
                generation: 0,
            }),
            queue: Mutex::new(VecDeque::new()),
        })
    }

    /// Accepts radio links from now on. uniffi: method.
    pub fn start(&self, now_mono: u64) -> Result<(), ApiError> {
        self.with_events(|i| {
            i.running = true;
            Ok(((), i.node.tick(now_mono)))
        })
    }

    /// Takes every link down and stops accepting radio calls; the
    /// database stays open and commands still queue. uniffi: method.
    pub fn stop(&self, now_mono: u64) -> Result<(), ApiError> {
        self.with(|i| {
            for (link, _, _) in i.node.links() {
                i.node.link_down(link, now_mono);
            }
            i.running = false;
            Ok(())
        })
    }

    /// Whether [`MeshNode::start`] was called since the last stop. uniffi: method.
    pub fn is_running(&self) -> Result<bool, ApiError> {
        Ok(self.lock()?.running)
    }

    /// Periodic maintenance (the platform calls it about once a second
    /// while running; DESIGN §1 "tick scheduler"): dedupe sweeps, meter
    /// checkpoints, rotation due, delivery ageing, link service. Events go
    /// to the sink or the queue (`Node::tick` *returns* the events it
    /// produced, draining the node's queue; they are handed on like every
    /// other event, so "May have expired · Resend" reaches the UI).
    /// uniffi: method.
    pub fn tick(&self, now_mono: u64) -> Result<(), ApiError> {
        self.with_events(|i| Ok(((), i.node.tick(now_mono))))
    }

    /// Installs (or with `None` removes) the event sink. Events produced
    /// while no sink is installed wait in the poll queue. uniffi: method
    /// taking an optional callback interface.
    pub fn set_event_sink(&self, sink: Option<EventSink>) {
        let mut slot = self.sink_slot();
        slot.sink = sink;
        slot.generation = slot.generation.wrapping_add(1);
    }

    /// Drains the events not yet handed to a sink. uniffi: method.
    pub fn poll_events(&self) -> Vec<ApiEvent> {
        self.queue_slot().drain(..).collect()
    }

    /// The display hook, the only path to wall time (BRIEF_v2 §4). uniffi:
    /// method taking a callback interface.
    pub fn set_display_clock(&self, clock: Box<dyn WallClock>) -> Result<(), ApiError> {
        self.lock()?
            .node
            .set_display_clock(Box::new(WallAdaptor(clock)));
        Ok(())
    }

    /// Reads of the display clock so far (tests assert zero for protocol
    /// runs, DESIGN §10 S4 (e)). uniffi: method.
    pub fn wall_reads(&self) -> Result<u64, ApiError> {
        Ok(self.lock()?.node.wall_reads())
    }

    /// Battery input (BRIEF_v2 §5.7): below the floor and not charging the
    /// phone stops forwarding others' cells. uniffi: method.
    pub fn set_power(&self, level_ppm: u32, charging: bool) -> Result<(), ApiError> {
        self.lock()?.node.set_power(level_ppm, charging);
        Ok(())
    }

    /// Fault-injection hook (DESIGN §10 "kill between decrypt and commit"):
    /// the next transaction is dropped instead of committed. uniffi: method
    /// (debug builds of the app only).
    pub fn set_fail_before_commit(&self, fail: bool) -> Result<(), ApiError> {
        self.lock()?.node.set_fail_before_commit(fail);
        Ok(())
    }

    // ---- links (DESIGN §7) ----------------------------------------------------------

    /// A BLE link came up; `hint` is the contact the platform believes is
    /// on the other end (32-byte key), used only to offer a contact proof.
    /// uniffi: method.
    pub fn link_up(
        &self,
        link: u32,
        role: ApiLinkRole,
        hint: Option<Vec<u8>>,
        now_mono: u64,
    ) -> Result<(), ApiError> {
        let hint = hint.as_deref().map(key_from).transpose()?;
        self.with(|i| {
            if !i.running {
                return Err(ApiError::NotRunning);
            }
            Ok(i.node.link_up(link_id(link), role.into(), hint, now_mono)?)
        })
    }

    /// The link is gone. uniffi: method.
    pub fn link_down(&self, link: u32, now_mono: u64) -> Result<(), ApiError> {
        self.with(|i| {
            i.node.link_down(link_id(link), now_mono);
            Ok(())
        })
    }

    /// Inbound radio bytes for `link`, in any chunking. `Err(LinkDead)`
    /// means the link was removed and the platform drops the connection.
    /// uniffi: method taking bytes.
    pub fn feed(&self, link: u32, bytes: Vec<u8>, now_mono: u64) -> Result<(), ApiError> {
        self.with(|i| {
            if !i.running {
                return Err(ApiError::NotRunning);
            }
            Ok(i.node.feed(link_id(link), &bytes, now_mono)?)
        })
    }

    /// Bytes the radio should carry from this side of `link`, at most
    /// `max_bytes`. uniffi: method returning bytes.
    pub fn poll_outbound(
        &self,
        link: u32,
        max_bytes: u32,
        now_mono: u64,
    ) -> Result<Vec<u8>, ApiError> {
        let max = usize::try_from(max_bytes).map_err(|_| ApiError::invalid("max_bytes"))?;
        self.with(|i| {
            if !i.running {
                return Err(ApiError::NotRunning);
            }
            Ok(i.node.poll_outbound(link_id(link), max, now_mono))
        })
    }

    /// Links the core holds, with their authentication. uniffi: method.
    pub fn links(&self) -> Result<Vec<ApiLink>, ApiError> {
        Ok(self
            .lock()?
            .node
            .links()
            .into_iter()
            .map(|(l, auth, up)| ApiLink {
                link: l.0,
                auth: auth.into(),
                up,
            })
            .collect())
    }

    // ---- identity (BRIEF_v2 §3) --------------------------------------------------------

    /// This phone's 32-byte identity key (`None` for a station). uniffi: method.
    pub fn identity(&self) -> Result<Option<Vec<u8>>, ApiError> {
        Ok(self.lock()?.node.identity().map(|k| key_bytes(&k)))
    }

    /// This phone's 128-byte KeyCard (BRIEF_v2 §3 "one contact = one key").
    /// uniffi: method returning bytes.
    pub fn my_keycard(&self) -> Result<Vec<u8>, ApiError> {
        Ok(self.lock()?.node.my_keycard()?.to_bytes().to_vec())
    }

    /// This phone's 32-byte envelope public key for sealed first flights.
    /// uniffi: method returning bytes.
    pub fn my_envelope_pub(&self) -> Result<Vec<u8>, ApiError> {
        Ok(self.lock()?.node.my_envelope_pub()?.to_vec())
    }

    /// The stored display name (`None` for a station). uniffi: method.
    pub fn display_name(&self) -> Result<Option<String>, ApiError> {
        Ok(self.lock()?.node.display_name().map(str::to_owned))
    }

    /// Display name for a key: the roster or contact name, else
    /// "Unverified" (never invented from the key). uniffi: method.
    pub fn name_of(&self, key: Vec<u8>) -> Result<String, ApiError> {
        let key = key_from(&key)?;
        Ok(self.lock()?.node.name_of(&key))
    }

    /// Powered-on milliseconds for the platform reading `now_mono`
    /// (DESIGN §6 "Powered-on meter"). uniffi: method.
    pub fn powered_on_ms(&self, now_mono: u64) -> Result<u64, ApiError> {
        Ok(self.lock()?.node.powered_on_ms(now_mono))
    }

    /// The effective configuration: what [`MeshNode::open`] was given,
    /// except that a member's `display_name` is the stored one (on a reopen
    /// the stored name wins over `config.role`, see [`ApiRole`]; the two
    /// never disagree here). uniffi: method.
    pub fn config(&self) -> Result<ApiConfig, ApiError> {
        let g = self.lock()?;
        let mut cfg = ApiConfig::from_node_config(g.node.config());
        if let (ApiRole::Member { display_name }, Some(stored)) =
            (&mut cfg.role, g.node.display_name())
        {
            *display_name = stored.to_owned();
        }
        Ok(cfg)
    }

    // ---- groups and roster (BRIEF_v2 §5.4; DESIGN §3) -----------------------------------

    /// Creates a group: this phone is an admin and a member; `admins` are
    /// further admins' KeyCards, `members` further members. Returns the
    /// 16-byte group id. uniffi: method.
    pub fn create_group(
        &self,
        name: String,
        admins: Vec<Vec<u8>>,
        members: Vec<ApiMemberEntry>,
        settings: u8,
        now_mono: u64,
    ) -> Result<Vec<u8>, ApiError> {
        let admins = admins
            .iter()
            .map(|c| keycard_from(c))
            .collect::<Result<Vec<_>, _>>()?;
        let members = members
            .iter()
            .map(ApiMemberEntry::to_core)
            .collect::<Result<Vec<_>, _>>()?;
        self.with(|i| {
            let gid = i
                .node
                .create_group(&name, &admins, &members, settings, now_mono)?;
            Ok(group_bytes(&gid))
        })
    }

    /// A proposed Add for a second admin to endorse. uniffi: method.
    pub fn propose_add(
        &self,
        group: Vec<u8>,
        member: ApiMemberEntry,
    ) -> Result<ApiAddDraft, ApiError> {
        let gid = group_from(&group)?;
        let entry = member.to_core()?;
        let draft = self.lock()?.node.propose_add(gid, entry)?;
        Ok(ApiAddDraft {
            group: group_bytes(&draft.group),
            member: ApiMemberEntry::from_core(&draft.subject),
        })
    }

    /// This admin's endorsement of a draft (`in_person`: this admin
    /// verified the newcomer in person). uniffi: method.
    pub fn endorse(&self, draft: ApiAddDraft, in_person: bool) -> Result<ApiEndorsement, ApiError> {
        let core = AddDraft {
            group: group_from(&draft.group)?,
            subject: draft.member.to_core()?,
        };
        let e = self.lock()?.node.endorse(&core, in_person)?;
        Ok(ApiEndorsement::from_core(&e))
    }

    /// Adds a member with a second admin's endorsement (two distinct
    /// admins, BRIEF_v2 §5.4). uniffi: method.
    pub fn add_member(
        &self,
        group: Vec<u8>,
        member: ApiMemberEntry,
        endorsement: ApiEndorsement,
        now_mono: u64,
    ) -> Result<(), ApiError> {
        let gid = group_from(&group)?;
        let entry = member.to_core()?;
        let e = endorsement.to_core()?;
        self.with(|i| Ok(i.node.add_member(gid, entry, e, now_mono)?))
    }

    /// Removes a member (any one admin, D3 M2). uniffi: method.
    pub fn remove_member(
        &self,
        group: Vec<u8>,
        member: Vec<u8>,
        now_mono: u64,
    ) -> Result<(), ApiError> {
        let gid = group_from(&group)?;
        let key = key_from(&member)?;
        self.with(|i| Ok(i.node.remove_member(gid, key, now_mono)?))
    }

    /// Groups this phone holds a roster for (16-byte ids). uniffi: method.
    pub fn groups(&self) -> Result<Vec<Vec<u8>>, ApiError> {
        Ok(self.lock()?.node.groups().iter().map(group_bytes).collect())
    }

    /// `members(H)` at the group's current heads. uniffi: method.
    pub fn roster(&self, group: Vec<u8>) -> Result<ApiRoster, ApiError> {
        let gid = group_from(&group)?;
        let view = self.lock()?.node.roster(&gid)?;
        Ok(ApiRoster::from_view(&gid, &view))
    }

    /// Whether this phone's next post in the group rotates its Sender Key.
    /// uniffi: method.
    pub fn rotation_pending(&self, group: Vec<u8>) -> Result<bool, ApiError> {
        let gid = group_from(&group)?;
        Ok(self.lock()?.node.rotation_pending(&gid)?)
    }

    // ---- messages (BRIEF_v2 §5.9) ----------------------------------------------------------

    /// Posts a text in the group; returns the 32-byte message id whose
    /// delivery state the UI follows. uniffi: method.
    pub fn send_text(
        &self,
        group: Vec<u8>,
        text: String,
        now_mono: u64,
    ) -> Result<Vec<u8>, ApiError> {
        let gid = group_from(&group)?;
        self.with(|i| Ok(hash_bytes(&i.node.send_text(gid, &text, now_mono)?)))
    }

    /// Sends a 1:1 text (`NeedsKey` when the peer's bundle has not arrived,
    /// D6 rule 7). uniffi: method.
    pub fn send_dm(&self, peer: Vec<u8>, text: String, now_mono: u64) -> Result<Vec<u8>, ApiError> {
        let key = key_from(&peer)?;
        self.with(|i| Ok(hash_bytes(&i.node.send_dm(key, &text, now_mono)?)))
    }

    /// "Resend" (BRIEF_v2 §5.9 "May have expired · Resend"): the stored
    /// text goes out again as a new message. uniffi: method.
    pub fn resend(&self, obj: Vec<u8>, now_mono: u64) -> Result<Vec<u8>, ApiError> {
        let id = hash_from(&obj, "message id must be 32 bytes")?;
        self.with(|i| Ok(hash_bytes(&i.node.resend(&id, now_mono)?)))
    }

    /// Delivery state of a message this phone sent. uniffi: method.
    pub fn delivery(&self, obj: Vec<u8>) -> Result<ApiDeliveryState, ApiError> {
        let id = hash_from(&obj, "message id must be 32 bytes")?;
        Ok(self.lock()?.node.delivery(&id)?.into())
    }

    /// Every message this phone sent, with its delivery state. uniffi: method.
    pub fn outbox(&self) -> Result<Vec<ApiOutboxEntry>, ApiError> {
        let g = self.lock()?;
        let mut out = Vec::new();
        for obj in g.node.outbox() {
            let state = g.node.delivery(&obj)?;
            out.push(ApiOutboxEntry {
                obj: hash_bytes(&obj),
                state: state.into(),
                text: state.text().to_owned(),
            });
        }
        Ok(out)
    }

    /// The decrypted text of a stored message (sent or received), if it is
    /// a text. uniffi: method.
    pub fn plaintext(&self, obj: Vec<u8>) -> Result<Option<String>, ApiError> {
        let id = hash_from(&obj, "message id must be 32 bytes")?;
        Ok(self.lock()?.node.plaintext(&id)?)
    }

    // ---- tasks (BRIEF_v2 §4; DESIGN §4) ----------------------------------------------------

    /// Creates a task; returns its 16-byte id. uniffi: method.
    pub fn create_task(
        &self,
        group: Vec<u8>,
        title: String,
        details: String,
        now_mono: u64,
    ) -> Result<Vec<u8>, ApiError> {
        let gid = group_from(&group)?;
        self.with(|i| {
            Ok(i.node
                .create_task(gid, &title, &details, now_mono)?
                .to_vec())
        })
    }

    /// "Take" (§7 vocabulary). uniffi: method.
    pub fn take_task(
        &self,
        group: Vec<u8>,
        task_id: Vec<u8>,
        now_mono: u64,
    ) -> Result<(), ApiError> {
        let gid = group_from(&group)?;
        let tid = task_id_from(&task_id)?;
        self.with(|i| Ok(i.node.take_task(gid, tid, now_mono)?))
    }

    /// "Release". uniffi: method.
    pub fn release_task(
        &self,
        group: Vec<u8>,
        task_id: Vec<u8>,
        now_mono: u64,
    ) -> Result<(), ApiError> {
        let gid = group_from(&group)?;
        let tid = task_id_from(&task_id)?;
        self.with(|i| Ok(i.node.release_task(gid, tid, now_mono)?))
    }

    /// "Done". uniffi: method.
    pub fn done_task(
        &self,
        group: Vec<u8>,
        task_id: Vec<u8>,
        now_mono: u64,
    ) -> Result<(), ApiError> {
        let gid = group_from(&group)?;
        let tid = task_id_from(&task_id)?;
        self.with(|i| Ok(i.node.done_task(gid, tid, now_mono)?))
    }

    /// One task's folded state and UI string. uniffi: method.
    pub fn task(&self, group: Vec<u8>, task_id: Vec<u8>) -> Result<ApiTask, ApiError> {
        let gid = group_from(&group)?;
        let tid = task_id_from(&task_id)?;
        let (state, ui) = self.lock()?.node.task(&gid, &tid)?;
        Ok(ApiTask {
            group,
            task_id,
            state: ApiTaskState::from(&state),
            ui,
        })
    }

    /// Every task of a group whose state is known. uniffi: method.
    pub fn tasks(&self, group: Vec<u8>) -> Result<Vec<ApiTask>, ApiError> {
        let gid = group_from(&group)?;
        let g = self.lock()?;
        let mut out = Vec::new();
        for tid in g.node.tasks(&gid)? {
            if let Ok((state, ui)) = g.node.task(&gid, &tid) {
                out.push(ApiTask {
                    group: group.clone(),
                    task_id: tid.to_vec(),
                    state: ApiTaskState::from(&state),
                    ui,
                });
            }
        }
        Ok(out)
    }

    /// The "Needs someone" pool of a group (BRIEF_v2 §4). uniffi: method.
    pub fn needs_someone(
        &self,
        group: Vec<u8>,
        now_mono: u64,
    ) -> Result<Vec<ApiNeedsSomeone>, ApiError> {
        let gid = group_from(&group)?;
        Ok(self
            .lock()?
            .node
            .needs_someone(&gid, now_mono)?
            .iter()
            .map(|(tid, age)| ApiNeedsSomeone::from_core(tid, age))
            .collect())
    }

    // ---- votes (BRIEF_v2 §4; DESIGN §5) ----------------------------------------------------

    /// Opens a vote with 2..=8 options and tolerance `f`; returns its
    /// 32-byte id. uniffi: method.
    pub fn open_vote(
        &self,
        group: Vec<u8>,
        question: String,
        options: Vec<String>,
        f: u8,
        now_mono: u64,
    ) -> Result<Vec<u8>, ApiError> {
        let gid = group_from(&group)?;
        let opts: Vec<&str> = options.iter().map(String::as_str).collect();
        self.with(|i| {
            Ok(hash_bytes(
                &i.node.open_vote(gid, &question, &opts, f, now_mono)?,
            ))
        })
    }

    /// Casts a ballot (no revotes: a second payload voids the key, D1
    /// rule 3). uniffi: method.
    pub fn cast_ballot(&self, vote_id: Vec<u8>, choice: u8, now_mono: u64) -> Result<(), ApiError> {
        let vid = hash_from(&vote_id, "vote id must be 32 bytes")?;
        self.with(|i| Ok(i.node.cast_ballot(vid, choice, now_mono)?))
    }

    /// One vote's line and tally. uniffi: method.
    pub fn vote(&self, vote_id: Vec<u8>) -> Result<ApiVote, ApiError> {
        let vid = hash_from(&vote_id, "vote id must be 32 bytes")?;
        let (tally, line) = self.lock()?.node.vote(&vid)?;
        Ok(ApiVote {
            vote_id,
            line,
            tally: tally.as_ref().map(ApiTally::from),
        })
    }

    /// Every vote this phone holds. uniffi: method.
    pub fn votes(&self) -> Result<Vec<ApiVote>, ApiError> {
        let g = self.lock()?;
        let mut out = Vec::new();
        for vid in g.node.votes() {
            let (tally, line) = g.node.vote(&vid)?;
            out.push(ApiVote {
                vote_id: hash_bytes(&vid),
                line,
                tally: tally.as_ref().map(ApiTally::from),
            });
        }
        Ok(out)
    }

    // ---- people (BRIEF_v2 §3; DECISIONS D5, D6) -------------------------------------------

    /// Provisions a contact from a completed pairing or an out-of-band
    /// exchange (verifies the KeyCard, pins the key, starts the session).
    /// uniffi: method.
    pub fn import_contact(
        &self,
        contact: ApiImportedContact,
        now_mono: u64,
    ) -> Result<(), ApiError> {
        let core = contact.to_core()?;
        self.with(|i| Ok(i.node.import_contact(core, now_mono)?))
    }

    /// Issues this phone's prekey bundle for `context` (one-time prekeys
    /// only at a scan or a direct encounter, D6 rule 2). uniffi: method
    /// returning bytes.
    pub fn my_bundle(&self, context: ApiIssueContext, now_mono: u64) -> Result<Vec<u8>, ApiError> {
        let ctx = context.to_core()?;
        self.with(|i| Ok(i.node.my_bundle(ctx, now_mono)?))
    }

    /// The bytes to render as the QR code for "Verify in person" (they
    /// include the per-display secret; show them, never send them).
    /// uniffi: method returning bytes.
    pub fn my_qr_code(&self, now_mono: u64) -> Result<Vec<u8>, ApiError> {
        self.with(|i| {
            let code = i.node.my_qr_code(now_mono)?;
            codec::try_encode(&code).map_err(|e| ApiError::Protocol { msg: e.to_string() })
        })
    }

    /// Starts the scanner side of a pairing over `link` after scanning
    /// `scanned` (`shown` is this phone's own displayed code; both carry a
    /// display secret and are wiped before this returns). uniffi: method.
    pub fn start_pairing_scanner(
        &self,
        link: u32,
        shown: Vec<u8>,
        scanned: Vec<u8>,
    ) -> Result<(), ApiError> {
        let shown = Zeroizing::new(shown);
        let scanned = Zeroizing::new(scanned);
        let shown = decode_qr(&shown)?;
        let scanned = decode_qr(&scanned)?;
        self.with(|i| {
            Ok(i.node
                .start_pairing_scanner(link_id(link), &shown, scanned)?)
        })
    }

    /// Starts the displayer side of a pairing over `link` for the code shown.
    /// uniffi: method.
    pub fn start_pairing_displayer(&self, link: u32, shown: Vec<u8>) -> Result<(), ApiError> {
        let shown = Zeroizing::new(shown);
        let shown = decode_qr(&shown)?;
        self.with(|i| Ok(i.node.start_pairing_displayer(link_id(link), shown)?))
    }

    /// Advances the pairing on `link` with the peer's message (the platform
    /// ferries the bytes over the direct link). uniffi: method.
    pub fn pairing_step(
        &self,
        link: u32,
        incoming: Option<Vec<u8>>,
        now_mono: u64,
    ) -> Result<ApiPairingStep, ApiError> {
        self.with(|i| {
            Ok(i.node
                .pairing_step(link_id(link), incoming.as_deref(), now_mono)?
                .into())
        })
    }

    /// The pinned contacts, in key order, with their session state and
    /// same-name marker (People screen). uniffi: method.
    pub fn contacts(&self) -> Result<Vec<ApiContact>, ApiError> {
        let g = self.lock()?;
        Ok(g.node
            .contacts()
            .iter()
            .map(|k| ApiContact {
                key: key_bytes(k),
                name: g.node.name_of(k),
                session: g.node.session_state(k).into(),
                same_name_marker: g.node.same_name_marker(k),
            })
            .collect())
    }

    /// The 1:1 session state with a key. uniffi: method.
    pub fn session_state(&self, peer: Vec<u8>) -> Result<ApiSessionState, ApiError> {
        let key = key_from(&peer)?;
        Ok(self.lock()?.node.session_state(&key).into())
    }

    // ---- status (BRIEF_v2 §2) -------------------------------------------------------------

    /// The global status strip. uniffi: method.
    pub fn status_strip(&self, now_mono: u64) -> Result<ApiStatusStrip, ApiError> {
        Ok(self.lock()?.node.status_strip(now_mono).into())
    }

    /// Diagnostic counters. uniffi: method.
    pub fn stats(&self) -> Result<ApiStats, ApiError> {
        Ok(self.lock()?.node.stats().into())
    }

    /// Cells held in the relay store. uniffi: method.
    pub fn store_len(&self) -> Result<u64, ApiError> {
        Ok(u64::try_from(self.lock()?.node.store_len()).unwrap_or(u64::MAX))
    }

    /// Bytes held in the relay store. uniffi: method.
    pub fn store_bytes(&self) -> Result<u64, ApiError> {
        Ok(u64::try_from(self.lock()?.node.store_bytes()).unwrap_or(u64::MAX))
    }

    // ---- plumbing ----------------------------------------------------------------------------

    fn lock(&self) -> Result<MutexGuard<'_, Inner>, ApiError> {
        self.inner.lock().map_err(|_| ApiError::Internal {
            msg: "node lock poisoned".to_owned(),
        })
    }

    /// The sink slot (poisoning is harmless here: the slot holds no state
    /// the node depends on).
    fn sink_slot(&self) -> MutexGuard<'_, SinkSlot> {
        self.sink.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn queue_slot(&self) -> MutexGuard<'_, VecDeque<ApiEvent>> {
        self.queue.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Runs `f` under the node lock, drains the events it produced, releases
    /// the lock and only then dispatches them (module docs).
    fn with<R>(&self, f: impl FnOnce(&mut Inner) -> Result<R, ApiError>) -> Result<R, ApiError> {
        self.with_events(|i| f(i).map(|r| (r, Vec::new())))
    }

    /// [`MeshNode::with`] for a call that *returns* events as well
    /// (`Node::tick` drains the node's queue itself): the returned events
    /// come first, then whatever is still queued in the node.
    fn with_events<R>(
        &self,
        f: impl FnOnce(&mut Inner) -> Result<(R, Vec<Event>), ApiError>,
    ) -> Result<R, ApiError> {
        let (result, events) = {
            let mut g = self.lock()?;
            let (result, mut produced) = match f(&mut g) {
                Ok((r, ev)) => (Ok(r), ev),
                Err(e) => (Err(e), Vec::new()),
            };
            produced.extend(g.node.poll_events());
            let events: Vec<ApiEvent> = produced.into_iter().map(ApiEvent::from).collect();
            (result, events)
        };
        self.dispatch(events);
        result
    }

    /// Hands events to the sink with no lock held, or queues them. The sink
    /// is taken out of its slot while it runs; events produced re-entrantly
    /// meanwhile land in the queue and are drained through the same sink
    /// before it is put back, so nothing is lost or reordered. A sink
    /// replaced or removed from inside the callback is not put back.
    fn dispatch(&self, events: Vec<ApiEvent>) {
        if events.is_empty() {
            return;
        }
        let (sink, generation) = {
            // The queue is filled while the slot is still held: a dispatch
            // on another thread decides "nothing pending, put the sink
            // back" under the same slot lock, so no event can slip into the
            // queue between that check and the sink's return.
            let mut slot = self.sink_slot();
            match slot.sink.take() {
                Some(sink) => (sink, slot.generation),
                None => {
                    self.queue_slot().extend(events);
                    return;
                }
            }
        };
        let mut batch = events;
        loop {
            for e in batch.drain(..) {
                sink(e);
            }
            let mut slot = self.sink_slot();
            if slot.generation != generation {
                // Replaced or removed from inside the callback: whatever
                // was queued meanwhile stays pollable.
                return;
            }
            let pending: Vec<ApiEvent> = self.queue_slot().drain(..).collect();
            if pending.is_empty() {
                slot.sink = Some(sink);
                return;
            }
            drop(slot);
            batch = pending;
        }
    }
}

fn decode_qr(bytes: &[u8]) -> Result<QrCode, ApiError> {
    codec::decode::<QrCode>(bytes).map_err(|e| ApiError::Protocol {
        msg: format!("qr code: {e}"),
    })
}

#[cfg(all(test, feature = "backend-libsignal"))]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use super::*;
    use crate::crypto::ed25519::IdentitySigner;

    thread_local! {
        static NODE: RefCell<Option<Rc<MeshNode>>> = const { RefCell::new(None) };
        static SEEN: RefCell<Vec<ApiEvent>> = const { RefCell::new(Vec::new()) };
        static REENTERED: Cell<u32> = const { Cell::new(0) };
        static ONCE: Cell<bool> = const { Cell::new(false) };
    }

    fn open_member(seed: u8) -> Rc<MeshNode> {
        Rc::new(
            MeshNode::open(
                None,
                vec![seed; 32],
                vec![seed ^ 0x55; 32],
                ApiConfig::member("Ada"),
                1_000,
            )
            .expect("open"),
        )
    }

    fn stranger(seed: u8) -> Vec<u8> {
        key_bytes(&IdentitySigner::from_seed(&[seed; 32]).public())
    }

    fn install(node: &Rc<MeshNode>) {
        NODE.with(|n| *n.borrow_mut() = Some(node.clone()));
        SEEN.with(|s| s.borrow_mut().clear());
        REENTERED.with(|r| r.set(0));
        ONCE.with(|o| o.set(false));
    }

    fn with_node(f: impl FnOnce(&MeshNode)) {
        NODE.with(|n| {
            let n = n.borrow();
            f(n.as_ref().expect("node installed"));
        });
    }

    fn seen() -> Vec<ApiEvent> {
        SEEN.with(|s| s.borrow().clone())
    }

    /// A DM to a stranger yields `NeedsKey` + `Delivery{NeedsKey}` without
    /// any network (D6 rule 7): the event source of these tests.
    fn poke(node: &MeshNode, seed: u8) {
        node.send_dm(stranger(seed), "hi".to_owned(), 2_000)
            .expect("queued as needs-key");
    }

    #[test]
    fn events_queue_without_a_sink() {
        let node = open_member(1);
        poke(&node, 9);
        let ev = node.poll_events();
        assert_eq!(ev.len(), 2, "{ev:?}");
        assert!(matches!(ev[0], ApiEvent::NeedsKey { .. }));
        assert!(matches!(
            ev[1],
            ApiEvent::Delivery {
                state: ApiDeliveryState::NeedsKey,
                ..
            }
        ));
        assert!(node.poll_events().is_empty());
        assert_eq!(node.wall_reads().unwrap(), 0);
    }

    #[test]
    fn sink_runs_outside_the_lock_and_may_reenter() {
        let node = open_member(2);
        install(&node);
        node.set_event_sink(Some(Box::new(|e| {
            with_node(|n| {
                // The node lock is free while the sink runs (a hang would
                // be the only symptom otherwise, so check before re-entering).
                assert!(
                    n.inner.try_lock().is_ok(),
                    "sink invoked under the node lock"
                );
                // A query, a command-free call and a poll from inside the
                // sink: none of them deadlocks.
                n.groups().expect("re-entrant query");
                n.poll_events();
                n.status_strip(2_000).expect("re-entrant query");
            });
            REENTERED.with(|r| r.set(r.get() + 1));
            SEEN.with(|s| s.borrow_mut().push(e));
        })));
        poke(&node, 9);
        assert_eq!(seen().len(), 2);
        assert_eq!(REENTERED.with(Cell::get), 2);
        assert!(node.poll_events().is_empty(), "the sink took them");
        // the sink is back in its slot
        poke(&node, 10);
        assert_eq!(seen().len(), 4);
    }

    #[test]
    fn sink_removed_inside_the_callback_stays_removed() {
        let node = open_member(3);
        install(&node);
        node.set_event_sink(Some(Box::new(|e| {
            with_node(|n| n.set_event_sink(None));
            SEEN.with(|s| s.borrow_mut().push(e));
        })));
        poke(&node, 9);
        assert_eq!(seen().len(), 2, "the batch in flight is delivered");
        poke(&node, 10);
        assert_eq!(seen().len(), 2, "no sink any more");
        assert_eq!(node.poll_events().len(), 2, "later events are pollable");
    }

    #[test]
    fn events_produced_inside_the_sink_reach_the_sink() {
        let node = open_member(4);
        install(&node);
        node.set_event_sink(Some(Box::new(|e| {
            if !ONCE.with(Cell::get) {
                ONCE.with(|o| o.set(true));
                // A re-entrant command that itself produces events.
                with_node(|n| poke(n, 11));
            }
            SEEN.with(|s| s.borrow_mut().push(e));
        })));
        poke(&node, 9);
        assert_eq!(seen().len(), 4, "{:?}", seen());
        assert!(node.poll_events().is_empty());
    }

    #[test]
    fn radio_calls_need_start() {
        let node = open_member(5);
        assert_eq!(
            node.link_up(1, ApiLinkRole::Central, None, 1_000),
            Err(ApiError::NotRunning)
        );
        assert_eq!(node.feed(1, vec![0], 1_000), Err(ApiError::NotRunning));
        node.start(1_000).unwrap();
        assert!(node.is_running().unwrap());
        node.link_up(1, ApiLinkRole::Central, None, 1_000).unwrap();
        assert_eq!(node.links().unwrap().len(), 1);
        assert!(!node.poll_outbound(1, 4096, 1_001).unwrap().is_empty());
        node.stop(1_002).unwrap();
        assert!(node.links().unwrap().is_empty());
        assert!(!node.is_running().unwrap());
    }
}
