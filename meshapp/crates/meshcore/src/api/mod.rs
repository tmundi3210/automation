//! Plain-Rust facade shaped for a later uniffi export (docs/DESIGN.md §1
//! row `api/*`: `node.rs` ([`MeshNode`]), `types.rs`, `events.rs`,
//! `errors.rs`; §11 milestone M2 wraps it, no uniffi dependency in
//! milestone 1).
//!
//! What the platform (DESIGN §8 Android, §9 iOS) sees:
//!
//! * [`MeshNode`]: `open` / `start` / `stop` / `tick`, the byte-stream
//!   link API (`link_up`, `feed`, `poll_outbound`, `link_down`), the
//!   commands (groups, roster, texts, 1:1, tasks, votes, contacts,
//!   pairing) and the queries the BRIEF_v2 §2 screens need (Chats, Tasks,
//!   People, Votes, the status strip);
//! * [`ApiEvent`]: what changed, delivered after the commit that caused it,
//!   through `poll_events` or a sink called outside the lock;
//! * [`ApiError`]: one flat error with a display string;
//! * `types`: owned, FFI-safe records and enums (`Vec<u8>` ids and keys,
//!   `String`, integers, `bool`), with the §7 vocabulary strings attached
//!   where the UI shows them.
//!
//! Every method's doc comment ends with its uniffi mapping intent.

pub mod errors;
pub mod events;
pub mod node;
pub mod types;

pub use errors::ApiError;
pub use events::ApiEvent;
pub use node::{EventSink, MIN_ENTROPY_BYTES, MeshNode};
pub use types::{
    ApiAddDraft, ApiBranch, ApiConfig, ApiContact, ApiDeliveryState, ApiEndorsement,
    ApiImportBadge, ApiImportedContact, ApiIssueContext, ApiLabelKind, ApiLink, ApiLinkAuth,
    ApiLinkRole, ApiMember, ApiMemberEntry, ApiNeedsSomeone, ApiOutboxEntry, ApiPairingStep,
    ApiRole, ApiRoster, ApiRosterLabel, ApiSessionState, ApiStats, ApiStatusStrip, ApiTally,
    ApiTask, ApiTaskState, ApiVote, ApiVoteState, WallClock,
};
