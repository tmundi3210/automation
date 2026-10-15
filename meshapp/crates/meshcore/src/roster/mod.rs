//! Roster DAG (docs/DESIGN.md §3): op storage, closure(H), members(H)/admins(H), rotation triggers, UI labels.

pub mod dag;
pub mod eval;
pub mod labels;
pub mod rotation;

pub use dag::{Insert, RejectReason, RosterDag};
pub use eval::{MemberInfo, RosterView, VoidReason, evaluate, members_of_head_set_hash};
pub use labels::{LabelKind, RosterLabel, labels};
pub use rotation::{PostVerdict, RotationPolicy, post_verdict};
