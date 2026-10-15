//! The one error type of the facade (docs/DESIGN.md §1 row
//! `api/errors.rs`). uniffi: a flat `Error` enum; every variant carries
//! only owned data and `Display` gives the string the platform shows
//! (BRIEF_v2 §7: error messages give a reason, never "Failed"; "Needs their
//! key" is the §7 term for a 1:1 message without a usable session).

use crate::node::NodeError;

use super::types::key_bytes;

/// Why a facade call did not apply.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ApiError {
    /// A byte argument had the wrong length or a value was out of range
    /// (checked before the lock is taken; nothing changed).
    #[error("invalid argument: {msg}")]
    InvalidArgument { msg: String },
    /// The database refused (DESIGN §6).
    #[error("storage: {msg}")]
    Storage { msg: String },
    /// The crypto backend refused (BRIEF_v2 §5.2).
    #[error("crypto backend: {msg}")]
    Backend { msg: String },
    /// A protocol object or rule refused (roster, group, session, post,
    /// task, vote, bundle, pairing, codec); `msg` names which.
    #[error("{msg}")]
    Protocol { msg: String },
    /// The link failed and was removed; the platform drops the connection
    /// and makes a new one (DESIGN §7).
    #[error("link {link} dropped")]
    LinkDead { link: u32 },
    #[error("unknown link {link}")]
    UnknownLink { link: u32 },
    #[error("unknown group")]
    UnknownGroup,
    #[error("unknown task")]
    UnknownTask,
    #[error("unknown vote")]
    UnknownVote,
    #[error("unknown message")]
    UnknownObject,
    #[error("not a member of this group")]
    NotMember,
    #[error("not an admin of this group")]
    NotAdmin,
    /// No usable 1:1 session with `peer` (D6 rule 7).
    #[error("Needs their key")]
    NeedsKey { peer: Vec<u8> },
    /// This phone is a keyless station and has no identity.
    #[error("this phone is a keyless station")]
    NoIdentity,
    /// The radio-facing calls (`link_up`, `feed`, `poll_outbound`) need
    /// [`MeshNode::start`](super::MeshNode::start) first.
    #[error("not started")]
    NotRunning,
    /// The node lock was poisoned by a panic in an earlier call.
    #[error("internal: {msg}")]
    Internal { msg: String },
}

impl ApiError {
    pub(crate) fn invalid(msg: &'static str) -> Self {
        ApiError::InvalidArgument {
            msg: msg.to_owned(),
        }
    }

    /// The display string (uniffi: platforms read `message()` on the
    /// thrown error).
    pub fn message(&self) -> String {
        self.to_string()
    }
}

impl From<NodeError> for ApiError {
    fn from(e: NodeError) -> Self {
        match e {
            NodeError::Storage(s) => ApiError::Storage { msg: s.to_string() },
            NodeError::Backend(b) => ApiError::Backend { msg: b.to_string() },
            NodeError::LinkDead(l) => ApiError::LinkDead { link: l.0 },
            NodeError::UnknownLink(l) => ApiError::UnknownLink { link: l.0 },
            NodeError::UnknownGroup => ApiError::UnknownGroup,
            NodeError::UnknownTask => ApiError::UnknownTask,
            NodeError::UnknownVote => ApiError::UnknownVote,
            NodeError::UnknownObject => ApiError::UnknownObject,
            NodeError::NotMember => ApiError::NotMember,
            NodeError::NotAdmin => ApiError::NotAdmin,
            NodeError::NeedsKey(peer) => ApiError::NeedsKey {
                peer: key_bytes(&peer),
            },
            NodeError::NoIdentity => ApiError::NoIdentity,
            NodeError::Invalid(msg) => ApiError::InvalidArgument {
                msg: msg.to_owned(),
            },
            other @ (NodeError::Group(_)
            | NodeError::Pairwise(_)
            | NodeError::RosterOp(_)
            | NodeError::Roster(_)
            | NodeError::Post(_)
            | NodeError::TaskOp(_)
            | NodeError::VoteOpen(_)
            | NodeError::OpenRejected(_)
            | NodeError::Codec(_)
            | NodeError::Inner(_)
            | NodeError::KeyCard(_)
            | NodeError::Contact(_)
            | NodeError::Bundle(_)
            | NodeError::Pairing(_)) => ApiError::Protocol {
                msg: other.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::link::LinkId;

    #[test]
    fn node_errors_map_to_flat_variants() {
        assert_eq!(
            ApiError::from(NodeError::LinkDead(LinkId(9))),
            ApiError::LinkDead { link: 9 }
        );
        assert_eq!(ApiError::from(NodeError::NoIdentity), ApiError::NoIdentity);
        assert_eq!(
            ApiError::from(NodeError::Invalid("x")).message(),
            "invalid argument: x"
        );
        let p = ApiError::from(NodeError::Pairwise(
            crate::pairwise::session::PairwiseError::NeedsTheirKey,
        ));
        assert!(matches!(p, ApiError::Protocol { .. }));
        assert_eq!(p.message(), "pairwise: needs their key");
    }
}
