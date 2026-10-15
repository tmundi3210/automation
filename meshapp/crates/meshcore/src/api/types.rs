//! FFI-friendly value types of the facade (docs/DESIGN.md §1 row
//! `api/types.rs`; BRIEF_v2 §2 "Information architecture": the Chats, Tasks,
//! People and Votes screens drive the query shapes; §7 vocabulary for every
//! user-facing string).
//!
//! Every type here is owned data a uniffi `Record` or `Enum` can carry:
//! `Vec<u8>` for keys, ids and hashes (32-byte identity keys, 16-byte group
//! and task ids, 32-byte object/vote ids, 128-byte KeyCards), `String`,
//! `u64`/`i64`/`u32`/`u8`, `bool`, `Option<_>` and `Vec<_>` of those. No
//! references, lifetimes or generics appear in any signature. Conversions
//! to and from the core types live in this module (`pub(crate)` helpers)
//! so that `node.rs` reads as the command/query list.

use core::fmt;

use zeroize::{Zeroize, Zeroizing};

use crate::crypto::backend::GroupId;
use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::Hash32;
use crate::crypto::keycard::{KEYCARD_LEN, KeyCard};
use crate::node::node::NodeStats;
use crate::node::status::{DeliveryState, StatusStrip};
use crate::node::{ImportBadge, ImportedContact, NodeConfig, NodeRole};
use crate::objects::roster_op::{Endorsement, MemberEntry};
use crate::objects::task_op::TaskId;
use crate::pairwise::bundles::IssueContext;
use crate::pairwise::pairing::PairingStep;
use crate::pairwise::session::SessionState;
use crate::relay::buffers::BufferPolicy;
use crate::roster::eval::RosterView;
use crate::roster::labels::{LabelKind, RosterLabel};
use crate::tasks::pool::EvidenceAge;
use crate::tasks::state::TaskState;
use crate::transport::link::{LinkAuth, LinkId, Role};
use crate::votes::open::Branch;
use crate::votes::tally::{Tally, VoteState};

use super::errors::ApiError;

/// Parts per million in one whole (`absent_peer_reserve_ppm`).
pub const PPM: u32 = 1_000_000;

/// Longest display name accepted at onboarding (BRIEF_v2 §6 step 1: a
/// name; assumption, well under the roster entry's `str16` limit).
pub const MAX_DISPLAY_NAME_BYTES: usize = 256;

// ---- configuration ------------------------------------------------------------

/// Who this phone is (uniffi: enum with one named-field variant).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiRole {
    /// A person's phone, shown as `display_name` (BRIEF_v2 §6 step 1). The
    /// identity seed is derived from the entropy passed to
    /// [`MeshNode::open`](super::MeshNode::open) on the first open and
    /// persisted; later opens ignore both the entropy and this name (the
    /// stored name wins; see [`MeshNode::display_name`](super::MeshNode::display_name)).
    Member { display_name: String },
    /// A keyless relay station (BRIEF_v2 §5.7; DESIGN §8 "Station mode").
    Station,
}

/// Build branch (BRIEF_v2 §0; `votes::open::Branch`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiBranch {
    A,
    B,
}

/// Static node configuration (uniffi: record). Mirrors
/// [`NodeConfig`](crate::node::NodeConfig) with FFI-safe fields; the
/// relay limits are the `relay::limits` constants and are not configurable
/// (DECISIONS D4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiConfig {
    pub role: ApiRole,
    pub branch: ApiBranch,
    /// Whole relay buffer, bytes (DECISIONS D4 BUFFER).
    pub total_cap_bytes: u64,
    /// Per inbound link, bytes.
    pub per_link_cap_bytes: u64,
    /// Share of the buffer only absent-peer traffic may use, in parts per
    /// million of [`PPM`] (the `f32` fraction of `BufferPolicy`).
    pub absent_peer_reserve_ppm: u32,
    /// Dedupe retention in powered-on milliseconds.
    pub dedupe_retention_ms: u64,
    /// Sender Key rotation interval in powered-on milliseconds (D3 K2).
    pub rotation_interval_ms: u64,
    /// Device-wide GATT link slots (D4 LINK SLOTS).
    pub max_links: u32,
    /// Battery floor in parts per million below which the phone stops
    /// forwarding others' cells (BRIEF_v2 §5.7).
    pub battery_floor_ppm: u32,
}

impl ApiConfig {
    /// A build defaults for a member phone named `display_name`.
    pub fn member(display_name: impl Into<String>) -> Self {
        ApiConfig {
            role: ApiRole::Member {
                display_name: display_name.into(),
            },
            ..Self::station()
        }
    }

    /// A build defaults for a keyless station.
    pub fn station() -> Self {
        Self::from_node_config(&NodeConfig::station())
    }

    /// The facade view of a core configuration (the reserve fraction is
    /// rounded to parts per million).
    pub fn from_node_config(cfg: &NodeConfig) -> Self {
        let role = match &cfg.role {
            NodeRole::Member { display_name, .. } => ApiRole::Member {
                display_name: display_name.clone(),
            },
            NodeRole::Station => ApiRole::Station,
        };
        ApiConfig {
            role,
            branch: match cfg.branch {
                Branch::A => ApiBranch::A,
                Branch::B => ApiBranch::B,
            },
            total_cap_bytes: usize_to_u64(cfg.buffer.total_cap_bytes),
            per_link_cap_bytes: usize_to_u64(cfg.buffer.per_link_cap_bytes),
            absent_peer_reserve_ppm: fraction_to_ppm(cfg.buffer.absent_peer_reserve),
            dedupe_retention_ms: cfg.dedupe_retention_ms,
            rotation_interval_ms: cfg.rotation_interval_ms,
            max_links: cfg.max_links,
            battery_floor_ppm: cfg.battery_floor_ppm,
        }
    }

    /// The core configuration for this facade configuration;
    /// `identity_seed` is used on the first open only (see [`ApiRole`]).
    pub fn to_node_config(&self, identity_seed: &[u8; 32]) -> Result<NodeConfig, ApiError> {
        let role = match &self.role {
            ApiRole::Member { display_name } => {
                if display_name.is_empty() {
                    return Err(ApiError::invalid("display name is empty"));
                }
                if display_name.len() > MAX_DISPLAY_NAME_BYTES {
                    return Err(ApiError::invalid("display name is too long"));
                }
                NodeRole::Member {
                    identity_seed: *identity_seed,
                    display_name: display_name.clone(),
                }
            }
            ApiRole::Station => NodeRole::Station,
        };
        if self.absent_peer_reserve_ppm > PPM {
            return Err(ApiError::invalid(
                "absent_peer_reserve_ppm exceeds 1 000 000",
            ));
        }
        if self.battery_floor_ppm > PPM {
            return Err(ApiError::invalid("battery_floor_ppm exceeds 1 000 000"));
        }
        if self.max_links == 0 {
            return Err(ApiError::invalid("max_links is zero"));
        }
        let buffer = BufferPolicy {
            total_cap_bytes: u64_to_usize(self.total_cap_bytes, "total_cap_bytes")?,
            per_link_cap_bytes: u64_to_usize(self.per_link_cap_bytes, "per_link_cap_bytes")?,
            absent_peer_reserve: ppm_to_fraction(self.absent_peer_reserve_ppm),
        };
        Ok(NodeConfig {
            role,
            branch: match self.branch {
                ApiBranch::A => Branch::A,
                ApiBranch::B => Branch::B,
            },
            buffer,
            dedupe_retention_ms: self.dedupe_retention_ms,
            rotation_interval_ms: self.rotation_interval_ms,
            max_links: self.max_links,
            battery_floor_ppm: self.battery_floor_ppm,
        })
    }
}

fn usize_to_u64(v: usize) -> u64 {
    u64::try_from(v).unwrap_or(u64::MAX)
}

fn u64_to_usize(v: u64, what: &'static str) -> Result<usize, ApiError> {
    usize::try_from(v).map_err(|_| ApiError::invalid(what))
}

/// `f32` fraction in `[0, 1]` to parts per million (clamped; the `as`
/// conversion from a clamped float saturates, it never truncates silently).
fn fraction_to_ppm(f: f32) -> u32 {
    let scaled = (f64::from(f) * f64::from(PPM)).round();
    if scaled.is_nan() {
        return 0;
    }
    scaled.clamp(0.0, f64::from(PPM)) as u32
}

fn ppm_to_fraction(ppm: u32) -> f32 {
    (f64::from(ppm.min(PPM)) / f64::from(PPM)) as f32
}

// ---- links ----------------------------------------------------------------------

/// Who initiated the BLE connection (`transport::link::Role`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiLinkRole {
    Central,
    Peripheral,
}

impl From<ApiLinkRole> for Role {
    fn from(r: ApiLinkRole) -> Role {
        match r {
            ApiLinkRole::Central => Role::Central,
            ApiLinkRole::Peripheral => Role::Peripheral,
        }
    }
}

/// What this phone knows about the peer on a link (DESIGN §2.3
/// ContactProof): a stranger running the app, or a QR-verified contact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiLinkAuth {
    Anonymous,
    VerifiedContact { peer: Vec<u8> },
}

impl From<LinkAuth> for ApiLinkAuth {
    fn from(a: LinkAuth) -> Self {
        match a {
            LinkAuth::Anonymous => ApiLinkAuth::Anonymous,
            LinkAuth::VerifiedContact(k) => ApiLinkAuth::VerifiedContact {
                peer: key_bytes(&k),
            },
        }
    }
}

/// One link as the core sees it (`MeshNode::links`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiLink {
    pub link: u32,
    pub auth: ApiLinkAuth,
    /// Handshake finished.
    pub up: bool,
}

// ---- status and delivery --------------------------------------------------------

/// Delivery state of a message this phone sent (BRIEF_v2 §5.9; §7
/// "Waiting / Passed along / Delivered").
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApiDeliveryState {
    Waiting,
    WaitingNetworkBusy,
    PassedAlong,
    Delivered,
    MayHaveExpired,
    NeedsKey,
}

impl ApiDeliveryState {
    /// The §7 vocabulary string shown next to the icon.
    pub fn text(self) -> String {
        DeliveryState::from(self).text().to_owned()
    }

    /// Solid states never revert (BRIEF_v2 §8): only Delivered.
    pub fn is_solid(self) -> bool {
        DeliveryState::from(self).is_solid()
    }

    /// Whether "Resend" is offered.
    pub fn can_resend(self) -> bool {
        DeliveryState::from(self).can_resend()
    }
}

impl From<DeliveryState> for ApiDeliveryState {
    fn from(s: DeliveryState) -> Self {
        match s {
            DeliveryState::Waiting => ApiDeliveryState::Waiting,
            DeliveryState::WaitingNetworkBusy => ApiDeliveryState::WaitingNetworkBusy,
            DeliveryState::PassedAlong => ApiDeliveryState::PassedAlong,
            DeliveryState::Delivered => ApiDeliveryState::Delivered,
            DeliveryState::MayHaveExpired => ApiDeliveryState::MayHaveExpired,
            DeliveryState::NeedsKey => ApiDeliveryState::NeedsKey,
        }
    }
}

impl From<ApiDeliveryState> for DeliveryState {
    fn from(s: ApiDeliveryState) -> Self {
        match s {
            ApiDeliveryState::Waiting => DeliveryState::Waiting,
            ApiDeliveryState::WaitingNetworkBusy => DeliveryState::WaitingNetworkBusy,
            ApiDeliveryState::PassedAlong => DeliveryState::PassedAlong,
            ApiDeliveryState::Delivered => DeliveryState::Delivered,
            ApiDeliveryState::MayHaveExpired => DeliveryState::MayHaveExpired,
            ApiDeliveryState::NeedsKey => DeliveryState::NeedsKey,
        }
    }
}

/// One message this phone sent, with its delivery state (Chats screen).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiOutboxEntry {
    pub obj: Vec<u8>,
    pub state: ApiDeliveryState,
    /// The §7 string for `state`.
    pub text: String,
}

/// The global status strip (BRIEF_v2 §2: "6 nearby · last sync 2 min ago"
/// or "Offline – will send later").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiStatusStrip {
    /// Links whose handshake finished.
    pub nearby: u32,
    /// This phone forwards others' cells.
    pub passing_along: bool,
    /// Wall-clock millisecond of the last digest match, display only
    /// (`None` without a display clock or before any sync).
    pub last_sync_display_ms: Option<i64>,
    /// Powered-on milliseconds since the last digest match.
    pub last_sync_age_ms: Option<u64>,
    /// Below the battery floor (BRIEF_v2 §5.7).
    pub battery_floor: bool,
    /// A relay limit is holding traffic ("network busy", DECISIONS D4 UI).
    pub network_busy: bool,
    /// The strip text in §7 vocabulary.
    pub text: String,
}

impl From<StatusStrip> for ApiStatusStrip {
    fn from(s: StatusStrip) -> Self {
        ApiStatusStrip {
            nearby: u32::try_from(s.nearby).unwrap_or(u32::MAX),
            passing_along: s.passing_along,
            last_sync_display_ms: s.last_sync_display,
            last_sync_age_ms: s.last_sync_age_ms,
            battery_floor: s.battery_floor,
            network_busy: s.network_busy,
            text: s.text(),
        }
    }
}

/// Counters for diagnostics and the simulator report (`node::NodeStats`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ApiStats {
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

impl From<NodeStats> for ApiStats {
    fn from(s: NodeStats) -> Self {
        ApiStats {
            cells_received: s.cells_received,
            cells_sent: s.cells_sent,
            malformed: s.malformed,
            duplicates: s.duplicates,
            throttled_in: s.throttled_in,
            rejected_in: s.rejected_in,
            no_room: s.no_room,
            decrypted: s.decrypted,
            decrypt_failures: s.decrypt_failures,
            rejected_posts: s.rejected_posts,
            parked: s.parked,
            handshakes: s.handshakes,
            commits: s.commits,
            rollbacks: s.rollbacks,
            penalties: s.penalties,
        }
    }
}

// ---- roster -----------------------------------------------------------------------

/// A member to add or list in a genesis (DESIGN §2.4 member entry):
/// the 128-byte KeyCard, the delivered name, and whether an admin verified
/// the person in person (BRIEF_v2 §5.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiMemberEntry {
    pub keycard: Vec<u8>,
    pub name: String,
    pub in_person: bool,
}

impl ApiMemberEntry {
    pub(crate) fn to_core(&self) -> Result<MemberEntry, ApiError> {
        Ok(MemberEntry {
            keycard: keycard_from(&self.keycard)?,
            name: self.name.clone(),
            in_person: self.in_person,
        })
    }

    pub(crate) fn from_core(m: &MemberEntry) -> Self {
        ApiMemberEntry {
            keycard: m.keycard.to_bytes().to_vec(),
            name: m.name.clone(),
            in_person: m.in_person,
        }
    }
}

/// One current member of a group (`members(H)`, DESIGN §3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiMember {
    pub key: Vec<u8>,
    pub keycard: Vec<u8>,
    pub name: String,
    pub in_person: bool,
    pub is_admin: bool,
}

/// `members(H)` at the group's current heads (DESIGN §3; BRIEF_v2 §5.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiRoster {
    pub group: Vec<u8>,
    /// Group name from the Genesis.
    pub name: String,
    /// Current heads, sorted.
    pub heads: Vec<Vec<u8>>,
    /// Heads this phone does not hold yet; non-empty means the view is
    /// provisional ("Waiting · needs this vote's member list").
    pub unresolved: Vec<Vec<u8>>,
    pub members: Vec<ApiMember>,
    pub admins: Vec<Vec<u8>>,
}

impl ApiRoster {
    pub(crate) fn from_view(group: &GroupId, v: &RosterView) -> Self {
        ApiRoster {
            group: group_bytes(group),
            name: v.name.clone(),
            heads: v.heads.iter().map(hash_bytes).collect(),
            unresolved: v.unresolved.iter().map(hash_bytes).collect(),
            members: v
                .members
                .iter()
                .map(|(k, m)| ApiMember {
                    key: key_bytes(k),
                    keycard: m.keycard.to_bytes().to_vec(),
                    name: m.name.clone(),
                    in_person: m.in_person,
                    is_admin: v.admins.contains(k),
                })
                .collect(),
            admins: v.admins.iter().map(key_bytes).collect(),
        }
    }
}

/// What happened to one person between two roster views (D3 UI labels).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiLabelKind {
    /// "[Name] added · Provisional".
    AddedProvisional,
    /// "[Name] removed" (waiting for the rest of the group).
    RemovedWaiting,
    /// "Changed after reconnect" (an Add voided by the merge, D3 M3).
    ChangedAfterReconnect,
}

/// One roster label to show, with its optional detail line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiRosterLabel {
    pub key: Vec<u8>,
    pub kind: ApiLabelKind,
    pub text: String,
    pub detail: Option<String>,
}

impl From<RosterLabel> for ApiRosterLabel {
    fn from(l: RosterLabel) -> Self {
        ApiRosterLabel {
            key: key_bytes(&l.key),
            kind: match l.kind {
                LabelKind::AddedProvisional => ApiLabelKind::AddedProvisional,
                LabelKind::RemovedWaiting => ApiLabelKind::RemovedWaiting,
                LabelKind::ChangedAfterReconnect => ApiLabelKind::ChangedAfterReconnect,
            },
            text: l.text,
            detail: l.detail,
        }
    }
}

/// A proposed Add handed to a second admin (BRIEF_v2 §5.4 "Add or re-add:
/// 2 distinct admins").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiAddDraft {
    pub group: Vec<u8>,
    pub member: ApiMemberEntry,
}

/// The second admin's signed endorsement of a draft (`objects::roster_op`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiEndorsement {
    pub admin: Vec<u8>,
    pub in_person: bool,
    /// 64-byte Ed25519 signature.
    pub sig: Vec<u8>,
}

impl ApiEndorsement {
    pub(crate) fn to_core(&self) -> Result<Endorsement, ApiError> {
        let sig: [u8; 64] = self
            .sig
            .as_slice()
            .try_into()
            .map_err(|_| ApiError::invalid("endorsement signature must be 64 bytes"))?;
        Ok(Endorsement {
            admin: key_from(&self.admin)?,
            in_person: self.in_person,
            sig,
        })
    }

    pub(crate) fn from_core(e: &Endorsement) -> Self {
        ApiEndorsement {
            admin: key_bytes(&e.admin),
            in_person: e.in_person,
            sig: e.sig.to_vec(),
        }
    }
}

// ---- tasks ------------------------------------------------------------------------

/// A task's folded state (DESIGN §4; BRIEF_v2 §4 "Tasks").
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiTaskState {
    Open,
    /// Live claims in claim order ("N people on this" when more than one).
    Taken {
        holders: Vec<Vec<u8>>,
    },
    /// Concurrent claims: the tie-break winner, the losers (offered "Keep
    /// helping" or "Release") and the second claims.
    Conflict {
        winner: Vec<u8>,
        losers: Vec<Vec<u8>>,
        second: Vec<Vec<u8>>,
    },
    Done,
}

impl From<&TaskState> for ApiTaskState {
    fn from(s: &TaskState) -> Self {
        match s {
            TaskState::Open => ApiTaskState::Open,
            TaskState::Taken { holders } => ApiTaskState::Taken {
                holders: holders.iter().map(key_bytes).collect(),
            },
            TaskState::Conflict {
                winner,
                losers,
                second,
            } => ApiTaskState::Conflict {
                winner: key_bytes(winner),
                losers: losers.iter().map(key_bytes).collect(),
                second: second.iter().map(key_bytes).collect(),
            },
            TaskState::Done => ApiTaskState::Done,
        }
    }
}

/// One task on the Tasks screen (`MeshNode::task`, `MeshNode::tasks`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiTask {
    pub group: Vec<u8>,
    pub task_id: Vec<u8>,
    pub state: ApiTaskState,
    /// The brief's state string ("Taken by you · Provisional", ...).
    pub ui: String,
}

/// A task in the "Needs someone" pool (BRIEF_v2 §4; docs/decisions-assumed.md
/// row 7): the newest evidence from a holder, in powered-on milliseconds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiNeedsSomeone {
    pub task_id: Vec<u8>,
    /// Powered-on time of the newest evidence (the caller converts it for
    /// display against [`MeshNode::powered_on_ms`](super::MeshNode::powered_on_ms)).
    pub since_powered_on_ms: u64,
    pub holder: Vec<u8>,
}

impl ApiNeedsSomeone {
    pub(crate) fn from_core(task_id: &TaskId, age: &EvidenceAge) -> Self {
        ApiNeedsSomeone {
            task_id: task_id.to_vec(),
            since_powered_on_ms: age.since_mono,
            holder: key_bytes(&age.holder),
        }
    }
}

// ---- votes ------------------------------------------------------------------------

/// A vote's state (DESIGN §5; BRIEF_v2 §4 "Votes"; §7 "Provisional /
/// Final / Disputed").
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiVoteState {
    Final {
        option: u8,
    },
    Disputed {
        members: Vec<Vec<u8>>,
    },
    LeadingProvisional {
        leading: Option<u8>,
        k_not_heard: u16,
    },
    ResultProvisional {
        leading: Option<u8>,
        needs_t: u16,
        k_not_heard: u16,
    },
}

impl From<&VoteState> for ApiVoteState {
    fn from(s: &VoteState) -> Self {
        match s {
            VoteState::Final { option } => ApiVoteState::Final { option: *option },
            VoteState::Disputed { members } => ApiVoteState::Disputed {
                members: members.iter().map(key_bytes).collect(),
            },
            VoteState::LeadingProvisional {
                leading,
                k_not_heard,
            } => ApiVoteState::LeadingProvisional {
                leading: *leading,
                k_not_heard: *k_not_heard,
            },
            VoteState::ResultProvisional {
                leading,
                needs_t,
                k_not_heard,
            } => ApiVoteState::ResultProvisional {
                leading: *leading,
                needs_t: *needs_t,
                k_not_heard: *k_not_heard,
            },
        }
    }
}

/// A vote's tally as the card shows it (`votes::tally::Tally`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiTally {
    /// Per option, without the void keys.
    pub displayed: Vec<u32>,
    /// Keys with conflicting ballots (D1 rule 3).
    pub void_keys: Vec<Vec<u8>>,
    /// Members with any valid ballot.
    pub heard: u32,
    /// Members with no valid ballot ("k not heard from").
    pub not_heard: Vec<Vec<u8>>,
    pub rejected: u32,
    pub state: ApiVoteState,
}

impl From<&Tally> for ApiTally {
    fn from(t: &Tally) -> Self {
        ApiTally {
            displayed: t.displayed.clone(),
            void_keys: t.void_keys.iter().map(key_bytes).collect(),
            heard: u32::try_from(t.heard).unwrap_or(u32::MAX),
            not_heard: t.not_heard.iter().map(key_bytes).collect(),
            rejected: u32::try_from(t.rejected).unwrap_or(u32::MAX),
            state: ApiVoteState::from(&t.state),
        }
    }
}

/// One vote on the Votes screen: the compact line ("Y10 N10 · 1 void ·
/// Provisional", or the waiting label while its member list is unknown)
/// and the tally when one exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiVote {
    pub vote_id: Vec<u8>,
    pub line: String,
    pub tally: Option<ApiTally>,
}

// ---- people -----------------------------------------------------------------------

/// The 1:1 session state with a contact (`pairwise::session::SessionState`;
/// D6 rule 7 "Needs their key").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiSessionState {
    NeedsTheirKey,
    Ready,
    Crossed,
}

impl From<SessionState> for ApiSessionState {
    fn from(s: SessionState) -> Self {
        match s {
            SessionState::NeedsTheirKey => ApiSessionState::NeedsTheirKey,
            SessionState::Ready => ApiSessionState::Ready,
            SessionState::Crossed => ApiSessionState::Crossed,
        }
    }
}

/// One pinned contact on the People screen (BRIEF_v2 §2, §3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiContact {
    pub key: Vec<u8>,
    /// The delivered display name ("Unverified" when none).
    pub name: String,
    pub session: ApiSessionState,
    /// "2 phones use the name X" when another key carries this name.
    pub same_name_marker: Option<String>,
}

/// Trust level a provisioned contact arrives with (BRIEF_v2 §3; §7
/// "Unverified / Verified in person").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiImportBadge {
    VerifiedInPerson,
    Unverified,
}

/// What a completed pairing or an out-of-band provisioning yields for one
/// contact (DECISIONS §3 row (4,5,6,7)).
#[derive(Clone, PartialEq, Eq)]
pub struct ApiImportedContact {
    pub keycard: Vec<u8>,
    pub display_name: String,
    pub bundle: Vec<u8>,
    /// 32-byte `k_pair` for L2 contact proofs, when QR-verified.
    pub link_key: Option<Vec<u8>>,
    /// 32-byte envelope public key for sealed first flights.
    pub envelope_pub: Vec<u8>,
    pub badge: ApiImportBadge,
}

/// `link_key` is `k_pair`, a secret: wiped when the record is dropped.
impl Drop for ApiImportedContact {
    fn drop(&mut self) {
        if let Some(k) = self.link_key.as_mut() {
            k.zeroize();
        }
    }
}

impl fmt::Debug for ApiImportedContact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiImportedContact")
            .field("display_name", &self.display_name)
            .field("bundle_len", &self.bundle.len())
            .field("link_key", &self.link_key.as_ref().map(|_| "<redacted>"))
            .field("badge", &self.badge)
            .finish_non_exhaustive()
    }
}

impl ApiImportedContact {
    pub(crate) fn to_core(&self) -> Result<ImportedContact, ApiError> {
        let link_key = match &self.link_key {
            Some(k) => {
                let k = Zeroizing::new(arr32(k, "link_key")?);
                Some(*k)
            }
            None => None,
        };
        Ok(ImportedContact {
            keycard: keycard_from(&self.keycard)?,
            display_name: self.display_name.clone(),
            bundle: self.bundle.clone(),
            link_key,
            envelope_pub: arr32(&self.envelope_pub, "envelope_pub")?,
            badge: match self.badge {
                ApiImportBadge::VerifiedInPerson => ImportBadge::VerifiedInPerson,
                ApiImportBadge::Unverified => ImportBadge::Unverified,
            },
        })
    }
}

/// Where a bundle is about to be handed out (`pairwise::bundles::IssueContext`;
/// D6 rule 2). `Gossip` is always refused by the core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiIssueContext {
    QrScan,
    DirectEncounter,
    InsideGroup { group: Vec<u8> },
    Gossip,
}

impl ApiIssueContext {
    pub(crate) fn to_core(&self) -> Result<IssueContext, ApiError> {
        Ok(match self {
            ApiIssueContext::QrScan => IssueContext::QrScan,
            ApiIssueContext::DirectEncounter => IssueContext::DirectEncounter,
            ApiIssueContext::InsideGroup { group } => IssueContext::InsideGroup(group_from(group)?),
            ApiIssueContext::Gossip => IssueContext::Gossip,
        })
    }
}

/// What the caller does after a pairing step (`pairwise::pairing::PairingStep`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiPairingStep {
    /// Send these bytes over the direct link.
    Send { bytes: Vec<u8> },
    /// Pairing completed; `peer` is now a "Verified in person" contact.
    Done { peer: Vec<u8> },
    /// Nothing to send; wait for the peer's next message.
    Wait,
}

impl From<PairingStep> for ApiPairingStep {
    fn from(s: PairingStep) -> Self {
        match s {
            PairingStep::Send(bytes) => ApiPairingStep::Send { bytes },
            PairingStep::Done(paired) => ApiPairingStep::Done {
                peer: key_bytes(&paired.peer),
            },
            PairingStep::Wait => ApiPairingStep::Wait,
        }
    }
}

/// Wall time for display strings only (`clock::Clock::wall_for_display`;
/// BRIEF_v2 §4 "no timestamps decide anything"). uniffi: callback
/// interface implemented by the platform. The core counts every read
/// ([`MeshNode::wall_reads`](super::MeshNode::wall_reads)).
pub trait WallClock: Send {
    /// Unix milliseconds, user-settable, may jump.
    fn wall_now_ms(&self) -> i64;
}

// ---- byte conversions ----------------------------------------------------------------

pub(crate) fn key_bytes(k: &IdentityKey) -> Vec<u8> {
    k.as_bytes().to_vec()
}

pub(crate) fn key_from(bytes: &[u8]) -> Result<IdentityKey, ApiError> {
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| ApiError::invalid("identity key must be 32 bytes"))?;
    IdentityKey::try_from(arr).map_err(|_| ApiError::invalid("identity key rejected"))
}

pub(crate) fn group_bytes(g: &GroupId) -> Vec<u8> {
    g.as_bytes().to_vec()
}

pub(crate) fn group_from(bytes: &[u8]) -> Result<GroupId, ApiError> {
    let arr: [u8; GroupId::LEN] = bytes
        .try_into()
        .map_err(|_| ApiError::invalid("group id must be 16 bytes"))?;
    Ok(GroupId(arr))
}

pub(crate) fn hash_bytes(h: &Hash32) -> Vec<u8> {
    h.as_bytes().to_vec()
}

pub(crate) fn hash_from(bytes: &[u8], what: &'static str) -> Result<Hash32, ApiError> {
    let arr: [u8; 32] = bytes.try_into().map_err(|_| ApiError::invalid(what))?;
    Ok(Hash32::from_bytes(arr))
}

pub(crate) fn task_id_from(bytes: &[u8]) -> Result<TaskId, ApiError> {
    bytes
        .try_into()
        .map_err(|_| ApiError::invalid("task id must be 16 bytes"))
}

pub(crate) fn keycard_from(bytes: &[u8]) -> Result<KeyCard, ApiError> {
    let arr: [u8; KEYCARD_LEN] = bytes
        .try_into()
        .map_err(|_| ApiError::invalid("keycard must be 128 bytes"))?;
    KeyCard::from_bytes(&arr).map_err(|e| ApiError::Protocol { msg: e.to_string() })
}

pub(crate) fn arr32(bytes: &[u8], what: &'static str) -> Result<[u8; 32], ApiError> {
    bytes.try_into().map_err(|_| ApiError::invalid(what))
}

pub(crate) fn link_id(link: u32) -> LinkId {
    LinkId(link)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_round_trips_through_the_core_type() {
        let api = ApiConfig::member("Ada");
        let core = api.to_node_config(&[7; 32]).expect("valid");
        assert_eq!(ApiConfig::from_node_config(&core), api);
        let station = ApiConfig::station();
        let core = station.to_node_config(&[0; 32]).expect("valid");
        assert!(matches!(core.role, NodeRole::Station));
        assert_eq!(ApiConfig::from_node_config(&core), station);
    }

    #[test]
    fn config_rejects_bad_values() {
        let mut c = ApiConfig::member("");
        assert!(c.to_node_config(&[0; 32]).is_err());
        c = ApiConfig::member("x");
        c.absent_peer_reserve_ppm = PPM + 1;
        assert!(c.to_node_config(&[0; 32]).is_err());
        c = ApiConfig::member("x");
        c.max_links = 0;
        assert!(c.to_node_config(&[0; 32]).is_err());
    }

    #[test]
    fn ppm_conversions_clamp() {
        assert_eq!(fraction_to_ppm(0.2), 200_000);
        assert_eq!(fraction_to_ppm(-1.0), 0);
        assert_eq!(fraction_to_ppm(7.0), PPM);
        assert_eq!(fraction_to_ppm(f32::NAN), 0);
        assert_eq!(fraction_to_ppm(ppm_to_fraction(123_456)), 123_456);
    }

    #[test]
    fn byte_conversions_check_lengths() {
        assert!(key_from(&[0u8; 31]).is_err());
        assert!(group_from(&[0u8; 15]).is_err());
        assert!(hash_from(&[0u8; 33], "h").is_err());
        assert!(task_id_from(&[0u8; 16]).is_ok());
        assert!(keycard_from(&[0u8; 128]).is_err(), "all-zero card fails");
        assert!(keycard_from(&[0u8; 5]).is_err());
    }
}
