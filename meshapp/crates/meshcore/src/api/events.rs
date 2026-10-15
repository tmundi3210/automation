//! What the node tells the UI (docs/DESIGN.md §1 row `api/events.rs`;
//! BRIEF_v2 §5.3 "Delivered means decrypted and committed": no event is
//! produced before the transaction that caused it committed; §5.9 delivery
//! states; D3 UI labels; D6 rule 7).
//!
//! uniffi: one `Enum` with named-field variants, delivered either through
//! [`MeshNode::poll_events`](super::MeshNode::poll_events) or through the
//! sink set with [`MeshNode::set_event_sink`](super::MeshNode::set_event_sink)
//! (a callback interface), which the facade invokes only after it released
//! the node lock, so a sink may call back into the node.

use crate::node::Event;

use super::types::{
    ApiDeliveryState, ApiLinkAuth, ApiRosterLabel, ApiVoteState, group_bytes, hash_bytes, key_bytes,
};

/// One UI event (`node::Event` with FFI-safe fields).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiEvent {
    /// A group text was decrypted and stored. `labels` carries "Sent by X,
    /// who was removed" when the author is no longer a member (DESIGN §3).
    Post {
        group: Vec<u8>,
        author: Vec<u8>,
        author_name: String,
        obj: Vec<u8>,
        text: String,
        labels: Vec<String>,
    },
    /// A 1:1 text arrived.
    Dm {
        peer: Vec<u8>,
        obj: Vec<u8>,
        text: String,
    },
    /// The delivery state of a message this phone sent changed
    /// (`state_text` is the §7 string).
    Delivery {
        obj: Vec<u8>,
        state: ApiDeliveryState,
        state_text: String,
    },
    /// `members(H)` of a group changed; `labels` are the D3 UI labels.
    RosterChanged {
        group: Vec<u8>,
        labels: Vec<ApiRosterLabel>,
    },
    /// A task's folded state changed; `ui` is the brief's state string.
    TaskChanged {
        group: Vec<u8>,
        task_id: Vec<u8>,
        ui: String,
    },
    /// A vote's tally changed; `line` is the compact count line.
    VoteChanged {
        vote_id: Vec<u8>,
        line: String,
        state: Option<ApiVoteState>,
    },
    /// A message to `peer` cannot be sent until its bundle arrives
    /// ("Needs their key", D6 rule 7).
    NeedsKey { peer: Vec<u8> },
    /// "2 phones use the name X" for `key` (BRIEF_v2 §7).
    SameName { key: Vec<u8>, text: String },
    /// A link finished its handshake (`up`) with the given authentication,
    /// or went down.
    LinkState {
        link: u32,
        up: bool,
        auth: ApiLinkAuth,
    },
}

impl From<Event> for ApiEvent {
    fn from(e: Event) -> Self {
        match e {
            Event::Post {
                group,
                author,
                author_name,
                obj,
                text,
                labels,
            } => ApiEvent::Post {
                group: group_bytes(&group),
                author: key_bytes(&author),
                author_name,
                obj: hash_bytes(&obj),
                text,
                labels,
            },
            Event::Dm { peer, obj, text } => ApiEvent::Dm {
                peer: key_bytes(&peer),
                obj: hash_bytes(&obj),
                text,
            },
            Event::Delivery { obj, state } => ApiEvent::Delivery {
                obj: hash_bytes(&obj),
                state: state.into(),
                state_text: state.text().to_owned(),
            },
            Event::RosterChanged { group, labels } => ApiEvent::RosterChanged {
                group: group_bytes(&group),
                labels: labels.into_iter().map(ApiRosterLabel::from).collect(),
            },
            Event::TaskChanged { group, task_id, ui } => ApiEvent::TaskChanged {
                group: group_bytes(&group),
                task_id: task_id.to_vec(),
                ui,
            },
            Event::VoteChanged {
                vote_id,
                line,
                state,
            } => ApiEvent::VoteChanged {
                vote_id: hash_bytes(&vote_id),
                line,
                state: state.as_ref().map(ApiVoteState::from),
            },
            Event::NeedsKey { peer } => ApiEvent::NeedsKey {
                peer: key_bytes(&peer),
            },
            Event::SameName { key, text } => ApiEvent::SameName {
                key: key_bytes(&key),
                text,
            },
            Event::LinkState { link, up, auth } => ApiEvent::LinkState {
                link: link.0,
                up,
                auth: auth.into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hash::Hash32;
    use crate::node::DeliveryState;
    use crate::transport::link::{LinkAuth, LinkId};

    #[test]
    fn events_flatten_to_owned_fields() {
        let e = ApiEvent::from(Event::Delivery {
            obj: Hash32::from_bytes([3; 32]),
            state: DeliveryState::PassedAlong,
        });
        assert_eq!(
            e,
            ApiEvent::Delivery {
                obj: vec![3; 32],
                state: ApiDeliveryState::PassedAlong,
                state_text: "Passed along".to_owned(),
            }
        );
        let e = ApiEvent::from(Event::LinkState {
            link: LinkId(4),
            up: true,
            auth: LinkAuth::Anonymous,
        });
        assert_eq!(
            e,
            ApiEvent::LinkState {
                link: 4,
                up: true,
                auth: ApiLinkAuth::Anonymous
            }
        );
    }
}
