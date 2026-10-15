//! Roster UI label strings (BRIEF_v2 §7 controlled vocabulary; DECISIONS.md
//! D3 "UI labels (criterion 3)"; DESIGN §3 "Posts by removed members").
//!
//! The exact strings live here so every screen says the same thing
//! (BRIEF §7 "one term per concept"; the banned word "Pending" never
//! appears). Labels are derived from two [`RosterView`]s, the one before and
//! the one after new ops arrived; nothing here reads a clock or decides
//! protocol state.
//!
//! | Event | Label |
//! |---|---|
//! | member left the view through a Remove | `[Name] removed · Waiting for people out of range` (never reverts) |
//! | its detail text | `[Name] can still read what people out of range send until this reaches them` |
//! | member entered the view | `[Name] added · Provisional (people near you only)` |
//! | an Add was voided at a merge | `Changed after reconnect` |
//! | a kept post by a non-member | `Sent by [Name], who was removed` |
//! | a cell whose key has not arrived | `Waiting · can't read yet` |

use crate::crypto::ed25519::IdentityKey;
use crate::roster::eval::RosterView;

/// D3 M3 / UI labels: an Add that a merge voided.
pub const CHANGED_AFTER_RECONNECT: &str = "Changed after reconnect";
/// D3 UI labels: a message whose Sender Key has not arrived yet.
pub const WAITING_CANT_READ_YET: &str = "Waiting · can't read yet";
/// Shown for a key the roster has no name for (BRIEF §7 identity
/// vocabulary: "Unverified").
pub const UNNAMED: &str = "Unverified";

/// "Sent by [name], who was removed" (BRIEF §5.4; DESIGN §3).
pub fn sent_by_removed(name: &str) -> String {
    format!("Sent by {name}, who was removed")
}

/// "[name] removed · Waiting for people out of range" (D3 UI labels).
pub fn removed_waiting(name: &str) -> String {
    format!("{name} removed · Waiting for people out of range")
}

/// Detail text under [`removed_waiting`].
pub fn removed_detail(name: &str) -> String {
    format!("{name} can still read what people out of range send until this reaches them")
}

/// "[name] added · Provisional (people near you only)" (D3 UI labels;
/// BRIEF §7 "Provisional + people near you only").
pub fn added_provisional(name: &str) -> String {
    format!("{name} added · Provisional (people near you only)")
}

/// The display name for `key`: its name in `after`, else in `before`, else
/// [`UNNAMED`] (a name is never invented from the key bytes).
pub fn display_name(before: &RosterView, after: &RosterView, key: &IdentityKey) -> String {
    after
        .members
        .get(key)
        .or_else(|| before.members.get(key))
        .map(|m| m.name.as_str())
        .filter(|n| !n.is_empty())
        .unwrap_or(UNNAMED)
        .to_owned()
}

/// What happened to one person between two views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LabelKind {
    /// Entered the member list.
    AddedProvisional,
    /// Left the member list through a valid Remove.
    RemovedWaiting,
    /// An Add of this person was voided by the merge (D3 M3).
    ChangedAfterReconnect,
}

/// One label to show, with its optional detail line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RosterLabel {
    pub key: IdentityKey,
    pub kind: LabelKind,
    pub text: String,
    pub detail: Option<String>,
}

/// Labels for the change from `before` to `after` (DESIGN §1
/// `roster/labels.rs`: `fn labels(view_before, view_after)`), sorted by key.
///
/// * a key in `after.members` but not `before.members` →
///   [`LabelKind::AddedProvisional`];
/// * a key in `before.members` but not `after.members` →
///   [`LabelKind::ChangedAfterReconnect`] if `after` voided its Add at the
///   merge; else [`LabelKind::RemovedWaiting`] with the detail text when
///   `after` holds a valid Remove of the key (`after.removed`); else (the
///   Add was voided for another reason, e.g. a sender-chain equivocation:
///   nobody removed the person, so the removal string would be false)
///   [`LabelKind::ChangedAfterReconnect`];
/// * a key newly listed in `after.changed_after_reconnect` that was never
///   shown as a member → [`LabelKind::ChangedAfterReconnect`] (the Add and
///   its void arrived in one sync).
pub fn labels(before: &RosterView, after: &RosterView) -> Vec<RosterLabel> {
    let mut out = Vec::new();
    for key in after.members.keys() {
        if !before.members.contains_key(key) {
            out.push(RosterLabel {
                key: *key,
                kind: LabelKind::AddedProvisional,
                text: added_provisional(&display_name(before, after, key)),
                detail: None,
            });
        }
    }
    for key in before.members.keys() {
        if after.members.contains_key(key) {
            continue;
        }
        let name = display_name(before, after, key);
        if after.changed_after_reconnect.contains(key) || !after.removed.contains(key) {
            out.push(RosterLabel {
                key: *key,
                kind: LabelKind::ChangedAfterReconnect,
                text: CHANGED_AFTER_RECONNECT.to_owned(),
                detail: None,
            });
        } else {
            out.push(RosterLabel {
                key: *key,
                kind: LabelKind::RemovedWaiting,
                text: removed_waiting(&name),
                detail: Some(removed_detail(&name)),
            });
        }
    }
    for key in &after.changed_after_reconnect {
        if before.changed_after_reconnect.contains(key) || before.members.contains_key(key) {
            continue;
        }
        out.push(RosterLabel {
            key: *key,
            kind: LabelKind::ChangedAfterReconnect,
            text: CHANGED_AFTER_RECONNECT.to_owned(),
            detail: None,
        });
    }
    out.sort_by(|a, b| a.key.cmp(&b.key).then(a.kind.cmp(&b.kind)));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;
    use crate::crypto::keycard::KeyCard;
    use crate::roster::eval::MemberInfo;

    fn key(i: u8) -> IdentityKey {
        IdentitySigner::from_seed(&[i; 32]).public()
    }

    fn info(i: u8, name: &str) -> MemberInfo {
        MemberInfo {
            keycard: KeyCard::issue(&IdentitySigner::from_seed(&[i; 32]), [i; 32]),
            name: name.into(),
            in_person: true,
        }
    }

    #[test]
    fn strings_use_the_brief_vocabulary() {
        assert_eq!(sent_by_removed("Mo"), "Sent by Mo, who was removed");
        assert_eq!(
            removed_waiting("Mo"),
            "Mo removed · Waiting for people out of range"
        );
        assert_eq!(
            removed_detail("Mo"),
            "Mo can still read what people out of range send until this reaches them"
        );
        assert_eq!(
            added_provisional("Ana"),
            "Ana added · Provisional (people near you only)"
        );
        assert_eq!(CHANGED_AFTER_RECONNECT, "Changed after reconnect");
        for s in [
            sent_by_removed("x"),
            removed_waiting("x"),
            added_provisional("x"),
            CHANGED_AFTER_RECONNECT.to_owned(),
            WAITING_CANT_READ_YET.to_owned(),
        ] {
            assert!(!s.contains("Pending"), "{s}");
        }
    }

    #[test]
    fn labels_from_two_views() {
        let mut before = RosterView::default();
        before.members.insert(key(1), info(1, "Ana"));
        before.members.insert(key(2), info(2, "Mo"));
        before.members.insert(key(3), info(3, ""));
        let mut after = RosterView::default();
        after.members.insert(key(1), info(1, "Ana"));
        after.members.insert(key(4), info(4, "Ben"));
        after.changed_after_reconnect.insert(key(3));
        after.changed_after_reconnect.insert(key(5));
        after.removed.insert(key(2));
        let ls = labels(&before, &after);
        let kinds: Vec<(IdentityKey, LabelKind)> = ls.iter().map(|l| (l.key, l.kind)).collect();
        let mut expect = vec![
            (key(2), LabelKind::RemovedWaiting),
            (key(3), LabelKind::ChangedAfterReconnect),
            (key(4), LabelKind::AddedProvisional),
            (key(5), LabelKind::ChangedAfterReconnect),
        ];
        expect.sort();
        assert_eq!(kinds, expect);
        let mo = ls.iter().find(|l| l.key == key(2)).unwrap();
        assert_eq!(mo.text, "Mo removed · Waiting for people out of range");
        assert_eq!(
            mo.detail.as_deref(),
            Some("Mo can still read what people out of range send until this reaches them")
        );
        let ben = ls.iter().find(|l| l.key == key(4)).unwrap();
        assert_eq!(ben.text, "Ben added · Provisional (people near you only)");
        assert_eq!(display_name(&before, &after, &key(3)), UNNAMED);
        assert_eq!(display_name(&before, &after, &key(9)), UNNAMED);
        // nothing changed → no labels
        assert!(labels(&after, &after).is_empty());
        // a member that left the view without any valid Remove (its Add was
        // voided for another reason) is not reported as removed
        let mut gone = after.clone();
        gone.removed.clear();
        let ls = labels(&before, &gone);
        let mo = ls.iter().find(|l| l.key == key(2)).unwrap();
        assert_eq!(mo.kind, LabelKind::ChangedAfterReconnect);
        assert_eq!(mo.text, CHANGED_AFTER_RECONNECT);
    }
}
