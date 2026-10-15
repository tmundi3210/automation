//! The node: wiring of every module into one single-threaded state machine
//! (docs/DESIGN.md §1 rows `node/node.rs`, `node/inbound.rs`,
//! `node/outbound.rs`, `node/status.rs`; §6 "Atomic commit"; §7 "Link
//! lifecycle in core"; BRIEF_v2 §5.3, §5.5 "Every relay", §5.9; DECISIONS
//! D3 K1–K2, D4, D6).
//!
//! A [`Node`] owns one [`Db`](crate::storage::Db), the crypto backend, the
//! relay buffers and every protocol manager. It has no threads, reads no OS
//! clock (time arrives as `now_mono: u64` from the platform's monotonic
//! clock; the wall clock is reachable only through the display hook,
//! [`Node::set_display_clock`], and every read of it is counted by
//! [`Node::wall_reads`]) and does no I/O beyond the database it was handed.
//! It is the integration target of the simulator and of the later uniffi
//! facade, so its API is plain Rust with owned types.
//!
//! # Transactions (BRIEF_v2 §5.3; DESIGN §6; D6 rule 4)
//!
//! Everything that touches the database goes through one
//! [`Txn`](crate::storage::Txn) per inbound cell or per command, committed
//! before any [`Event`] is emitted. Decrypt, ratchet advance, plaintext
//! store, message-key deletion and the recognition-window move land in one
//! commit. On an error, or a commit refused by the fault-injection hook
//! (`Db::set_fail_before_commit`), the transaction is dropped (rollback)
//! and the in-memory protocol state is **replayed from the database**, so
//! the node is exactly where the last commit left it: a re-fed frame then
//! yields exactly one delivery. (The persisted state *is* the snapshot: the
//! same loader serves restart and rollback, see [`persist`].)
//!
//! # Module map
//!
//! * [`node`]: the [`Node`] struct, `open`, `tick`, the queries and the
//!   transaction plumbing;
//! * [`links`]: per-link state (Noise NN session, Hello, ContactProof,
//!   summary exchange, budgets, scheduler) and the byte-stream API;
//! * [`inbound`]: the cell pipeline (format → dedupe → admit → store →
//!   recognise → reassemble → decrypt in a transaction → dispatch);
//! * [`outbound`]: commands (groups, roster ops, texts, DMs, tasks, votes,
//!   contacts, pairing);
//! * [`status`]: delivery states and the status strip;
//! * [`persist`]: the persisted layout of every in-memory manager;
//! * [`reasm`]: the node's own bounded fragment collector.

pub mod inbound;
pub mod links;
#[allow(clippy::module_inception)] // DESIGN §1 names the file `node/node.rs`
pub mod node;
pub mod outbound;
pub mod persist;
pub mod reasm;
pub mod status;

use core::fmt;

use crate::crypto::backend::{BackendError, GroupId};
use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::Hash32;
use crate::crypto::keycard::{KeyCard, KeyCardError};
use crate::groups::group::GroupError;
use crate::objects::chat_post::PostError;
use crate::objects::roster_op::RosterOpError;
use crate::objects::task_op::{TaskId, TaskOpError};
use crate::objects::vote_open::VoteOpenError;
use crate::pairwise::bundles::BundleError;
use crate::pairwise::pairing::PairingError;
use crate::pairwise::session::PairwiseError;
use crate::people::contacts::ContactError;
use crate::relay::buffers::BufferPolicy;
use crate::roster::dag::RejectReason;
use crate::roster::labels::RosterLabel;
use crate::storage::StorageError;
use crate::transport::link::{LinkAuth, LinkId};
use crate::votes::open::{Branch, OpenReject};
use crate::votes::tally::VoteState;

pub use node::Node;
pub use status::{DeliveryState, StatusStrip};

/// A delivered or sent object's id, the same on every phone that holds it:
/// `SHA256("meshapp.v1.obj" ‖ conv ‖ author.ed ‖ LE32(obj_seq))`
/// ([`object_id`]). The simulator's "no object delivered twice" check keys
/// on it.
pub type ObjId = Hash32;

/// A vote's id (`VoteOpen::vote_id`).
pub type VoteId = Hash32;

/// Domain of [`object_id`].
pub const OBJ_ID_DOMAIN: &[u8] = b"meshapp.v1.obj";

/// [`ObjId`] of the object `obj_seq` by `author` in conversation `conv`.
pub fn object_id(conv: &[u8; 16], author: &IdentityKey, obj_seq: u32) -> ObjId {
    crate::crypto::hash::sha256_concat(&[
        OBJ_ID_DOMAIN,
        &[0u8],
        conv,
        author.as_bytes(),
        &obj_seq.to_le_bytes(),
    ])
}

/// Who this node is (DESIGN §8 "Station mode": a station has no identity,
/// relays only, and its L3 cap is the whole link budget).
#[derive(Clone, PartialEq, Eq)]
pub enum NodeRole {
    /// A person's phone: identity key from `identity_seed`, shown as
    /// `display_name` (BRIEF_v2 §6 step 1).
    Member {
        identity_seed: [u8; 32],
        display_name: String,
    },
    /// A keyless relay station (BRIEF_v2 §5.7).
    Station,
}

impl fmt::Debug for NodeRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeRole::Member { display_name, .. } => f
                .debug_struct("Member")
                .field("display_name", display_name)
                .field("identity_seed", &"<redacted>")
                .finish(),
            NodeRole::Station => f.write_str("Station"),
        }
    }
}

/// Static configuration of a node.
#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub role: NodeRole,
    /// Build branch (A: `f_min = 1`, no delays; BRIEF_v2 §0).
    pub branch: Branch,
    /// Relay buffer caps and the absent-peer reserve (DECISIONS D4 BUFFER).
    pub buffer: BufferPolicy,
    /// Dedupe retention in powered-on milliseconds
    /// (`dtn::dedupe::dedupe_retention_ms`).
    pub dedupe_retention_ms: u64,
    /// Sender Key rotation interval in powered-on milliseconds
    /// (`roster::rotation::ROTATION_INTERVAL_A_ASM` for the A build).
    pub rotation_interval_ms: u64,
    /// Device-wide GATT link slots (D4 LINK SLOTS; Android 14: 7).
    pub max_links: u32,
    /// Battery floor in parts per million below which the phone stops
    /// forwarding others' cells (BRIEF_v2 §5.7 "about 30%, assumption").
    pub battery_floor_ppm: u32,
}

impl NodeConfig {
    /// A build defaults for a member node with the given identity.
    pub fn member(identity_seed: [u8; 32], display_name: impl Into<String>) -> Self {
        NodeConfig {
            role: NodeRole::Member {
                identity_seed,
                display_name: display_name.into(),
            },
            ..Self::station()
        }
    }

    /// A build defaults for a keyless station.
    pub fn station() -> Self {
        NodeConfig {
            role: NodeRole::Station,
            branch: Branch::A,
            buffer: BufferPolicy::default(),
            dedupe_retention_ms: crate::dtn::dedupe::DEDUPE_RETENTION_ASM_MS,
            rotation_interval_ms: crate::roster::rotation::ROTATION_INTERVAL_A_ASM,
            max_links: 7,
            battery_floor_ppm: status::BATTERY_FLOOR_PPM_ASM,
        }
    }
}

/// Trust level a provisioned contact arrives with (BRIEF_v2 §3 levels 0
/// and 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportBadge {
    VerifiedInPerson,
    Unverified,
}

/// What a completed pairing, or a simulator provisioning step, yields for
/// one contact (DECISIONS §3 row (4,5,6,7): bundles and envelope keys
/// travel at the QR scan, on a direct encounter or inside group
/// encryption, never as gossip).
#[derive(Clone)]
pub struct ImportedContact {
    pub keycard: KeyCard,
    /// The delivered display name (never used to look anything up).
    pub display_name: String,
    /// The contact's prekey bundle (`PairwiseBackend::make_prekey_bundle`).
    pub bundle: Vec<u8>,
    /// `k_pair` for L2 contact proofs, when the contact was QR-verified.
    pub link_key: Option<[u8; 32]>,
    /// The contact's envelope public key for sealed first flights.
    pub envelope_pub: [u8; 32],
    pub badge: ImportBadge,
}

impl fmt::Debug for ImportedContact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImportedContact")
            .field("key", &self.keycard.ed)
            .field("display_name", &self.display_name)
            .field("bundle_len", &self.bundle.len())
            .field("link_key", &self.link_key.map(|_| "<redacted>"))
            .field("badge", &self.badge)
            .finish()
    }
}

/// A proposed Add, handed to a second admin for endorsement (BRIEF_v2 §5.4
/// "Add or re-add: 2 distinct admins").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDraft {
    pub group: GroupId,
    pub subject: crate::objects::roster_op::MemberEntry,
}

/// What the node tells the UI (never before the transaction that produced
/// it committed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// A group text post was decrypted and stored (BRIEF_v2 §5.9
    /// "Delivered"). `labels` carries "Sent by X, who was removed" when the
    /// author is no longer a member (DESIGN §3).
    Post {
        group: GroupId,
        author: IdentityKey,
        author_name: String,
        obj: ObjId,
        text: String,
        labels: Vec<String>,
    },
    /// A 1:1 text arrived.
    Dm {
        peer: IdentityKey,
        obj: ObjId,
        text: String,
    },
    /// The delivery state of an object this node sent changed.
    Delivery { obj: ObjId, state: DeliveryState },
    /// `members(H)` at the current heads changed (D3 UI labels).
    RosterChanged {
        group: GroupId,
        labels: Vec<RosterLabel>,
    },
    /// A task's folded state changed; `ui` is the brief's state string.
    TaskChanged {
        group: GroupId,
        task_id: TaskId,
        ui: String,
    },
    /// A vote's tally changed; `line` is the compact count line.
    VoteChanged {
        vote_id: VoteId,
        line: String,
        state: Option<VoteState>,
    },
    /// A message to `peer` cannot be sent until its bundle arrives (D6 rule 7).
    NeedsKey { peer: IdentityKey },
    /// The same-name marker for `key` ("2 phones use the name X").
    SameName { key: IdentityKey, text: String },
    /// A link finished its handshake (`up`) with the given authentication,
    /// or went down.
    LinkState {
        link: LinkId,
        up: bool,
        auth: LinkAuth,
    },
}

/// Why a node operation did not apply.
#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("storage: {0}")]
    Storage(#[from] StorageError),
    #[error("backend: {0}")]
    Backend(#[from] BackendError),
    #[error("group: {0}")]
    Group(#[from] GroupError),
    #[error("pairwise: {0}")]
    Pairwise(#[from] PairwiseError),
    #[error("roster op: {0}")]
    RosterOp(#[from] RosterOpError),
    #[error("roster: {0}")]
    Roster(#[from] RejectReason),
    #[error("post: {0}")]
    Post(#[from] PostError),
    #[error("task op: {0}")]
    TaskOp(#[from] TaskOpError),
    #[error("vote open: {0}")]
    VoteOpen(#[from] VoteOpenError),
    #[error("vote opening rejected: {0:?}")]
    OpenRejected(OpenReject),
    #[error("codec: {0}")]
    Codec(#[from] crate::codec::CodecError),
    #[error("fragment layer: {0}")]
    Inner(#[from] crate::wire::inner::InnerError),
    #[error("keycard: {0}")]
    KeyCard(#[from] KeyCardError),
    #[error("contact: {0}")]
    Contact(#[from] ContactError),
    #[error("bundle: {0}")]
    Bundle(#[from] BundleError),
    #[error("pairing: {0}")]
    Pairing(#[from] PairingError),
    #[error("link {0:?} is dead and was removed")]
    LinkDead(LinkId),
    #[error("unknown link {0:?}")]
    UnknownLink(LinkId),
    #[error("unknown group")]
    UnknownGroup,
    #[error("unknown task")]
    UnknownTask,
    #[error("unknown vote")]
    UnknownVote,
    #[error("unknown object")]
    UnknownObject,
    #[error("not a member of this group")]
    NotMember,
    #[error("not an admin of this group")]
    NotAdmin,
    #[error("needs their key")]
    NeedsKey(IdentityKey),
    #[error("this node is a keyless station")]
    NoIdentity,
    #[error("{0}")]
    Invalid(&'static str),
}
