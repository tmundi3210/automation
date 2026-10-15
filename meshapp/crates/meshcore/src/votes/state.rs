//! Vote display: card strings, state labels, the "Changed after reconnect"
//! flag, annotations and the notification rule (BRIEF_v2 §4 "Votes ·
//! States", "Card example", §7 vocabulary; DECISIONS D1 rules 5–9, D2 rule
//! 6; docs/DESIGN.md §5).
//!
//! Everything here is a pure function of tallies and caller-supplied
//! display strings. The target time ("Target 18:00") and the "as of"
//! date are wall-clock renderings the caller makes with
//! `Clock::wall_for_display`; nothing here reads a clock and nothing here
//! changes a count (D2 rule 6: annotations only).

use std::collections::BTreeSet;

use crate::crypto::ed25519::IdentityKey;
use crate::votes::open::{Branch, ValidOpen};
use crate::votes::tally::{Tally, VoteState, top_two_gap};

/// Shown while `H` cannot be resolved on this phone (DESIGN §5; D2 rule
/// 3); such a vote is never Final here.
pub const WAITING_LABEL: &str = "Waiting · needs this vote's member list";

/// "Target 18:00 · late votes still count" (§7 vocabulary "Vote close
/// time"; D1 rule 6: a label that never disables Vote or filters ballots).
pub fn target_label(display_time: &str) -> String {
    format!("Target {display_time} · late votes still count")
}

/// Branch A disclosure (D2 rule 6).
pub const VISIBILITY_A: &str = "Group members can see how you voted";
/// Branch B disclosure (D1 "Branch B additions").
pub const VISIBILITY_B: &str =
    "Open ballot: members, and anyone holding a member's unlocked phone, can see your vote";

/// The card header (BRIEF_v2 §4 "Card example"): "Final when 7 of 12 back
/// one option · Target 18:00 · late votes still count · Ballots can't be
/// changed · Group members can see how you voted · 2 members are
/// Unverified". `target` is a [`target_label`] (omitted when `None`);
/// `unverified` is the number of roster keys shown as Unverified (D1 rule 1
/// "before anyone votes, the card shows how many roster keys are
/// Unverified"); omitted when zero.
pub fn card_header(
    open: &ValidOpen,
    branch: Branch,
    target: Option<&str>,
    unverified: usize,
) -> String {
    let mut parts: Vec<String> = vec![format!(
        "Final when {} of {} back one option",
        open.t(),
        open.n()
    )];
    if let Some(t) = target {
        parts.push(t.to_string());
    }
    parts.push("Ballots can't be changed".to_string());
    parts.push(
        match branch {
            Branch::A => VISIBILITY_A,
            Branch::B => VISIBILITY_B,
        }
        .to_string(),
    );
    match unverified {
        0 => {}
        1 => parts.push("1 member is Unverified".to_string()),
        n => parts.push(format!("{n} members are Unverified")),
    }
    parts.join(" · ")
}

/// The state line, icon word first (BRIEF_v2 §4 "States, each with icon +
/// word"; §7 "Provisional + people near you only", "Final", "Disputed · n
/// members sent conflicting ballots").
pub fn state_label(tally: &Tally) -> String {
    match &tally.state {
        VoteState::Final { .. } => "Final".to_string(),
        VoteState::Disputed { members } => format!(
            "Disputed · {} members sent conflicting ballots",
            members.len()
        ),
        VoteState::LeadingProvisional { k_not_heard, .. } => {
            if tally.heard == 0 {
                "Voting open".to_string()
            } else {
                format!(
                    "Leading · Provisional (people near you only) · {k_not_heard} not heard from"
                )
            }
        }
        VoteState::ResultProvisional {
            needs_t,
            k_not_heard,
            ..
        } => {
            format!(
                "Result (provisional) · needs {needs_t} for Final · {k_not_heard} not heard from"
            )
        }
    }
}

/// Option initial used in the compact tally line ("Y", "N").
fn initial(label: &str) -> String {
    label
        .chars()
        .next()
        .map(|c| c.to_uppercase().collect())
        .unwrap_or_else(|| "?".to_string())
}

/// The compact count line of DESIGN §10 S4: "Y10 N10 · 1 void ·
/// Provisional". Displayed counts (equivocators excluded), the void count
/// when non-zero, then the state word.
pub fn tally_line(open: &ValidOpen, tally: &Tally) -> String {
    let counts: Vec<String> = open
        .options()
        .iter()
        .enumerate()
        .map(|(i, o)| {
            format!(
                "{}{}",
                initial(o),
                tally.displayed.get(i).copied().unwrap_or(0)
            )
        })
        .collect();
    let mut parts = vec![counts.join(" ")];
    if !tally.void_keys.is_empty() {
        parts.push(format!("{} void", tally.void_keys.len()));
    }
    parts.push(
        match &tally.state {
            VoteState::Final { .. } => "Final",
            VoteState::Disputed { .. } => "Disputed",
            VoteState::LeadingProvisional { .. } | VoteState::ResultProvisional { .. } => {
                "Provisional"
            }
        }
        .to_string(),
    );
    parts.join(" · ")
}

/// A provisional lead that changed between two tallies of the same vote
/// (BRIEF_v2 §4 "Changed after reconnect (applies only to a provisional
/// lead; the reason is shown)").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangedAfterReconnect {
    pub from: Option<u8>,
    pub to: Option<u8>,
    /// Members heard from in `after` but not in `before`: the reason.
    pub newly_heard: usize,
    /// Keys that became void between the two tallies.
    pub newly_void: usize,
}

/// Detects a changed provisional lead. `None` unless `before` was
/// provisional with a lead and `after` is provisional with a different
/// lead (or none); a move to Final or Disputed is a notification, not a
/// reconnect change.
pub fn changed_after_reconnect(before: &Tally, after: &Tally) -> Option<ChangedAfterReconnect> {
    if !before.state.is_provisional() || !after.state.is_provisional() {
        return None;
    }
    let from = before.state.leading()?;
    let to = after.state.leading();
    if to == Some(from) {
        return None;
    }
    let newly_heard = before
        .not_heard
        .iter()
        .filter(|k| !after.not_heard.contains(k))
        .count();
    let newly_void = after.void_keys.difference(&before.void_keys).count();
    Some(ChangedAfterReconnect {
        from: Some(from),
        to,
        newly_heard,
        newly_void,
    })
}

/// "Changed after reconnect · was Yes, now No · 3 more heard from".
pub fn changed_label(open: &ValidOpen, change: &ChangedAfterReconnect) -> String {
    let name = |o: Option<u8>| match o.and_then(|i| open.options().get(usize::from(i))) {
        Some(label) => label.clone(),
        None => "tied".to_string(),
    };
    let mut s = format!(
        "Changed after reconnect · was {}, now {}",
        name(change.from),
        name(change.to)
    );
    if change.newly_heard > 0 {
        s.push_str(&format!(" · {} more heard from", change.newly_heard));
    }
    if change.newly_void > 0 {
        s.push_str(&format!(" · {} void", change.newly_void));
    }
    s
}

/// Annotations for roster changes not contained in `H` (BRIEF_v2 §4
/// "Changes not contained in H ... only add annotations"; D2 rule 6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotations {
    /// Display lines, in order.
    pub lines: Vec<String>,
    /// Ballots counted from members removed after the vote opened (`k`).
    pub removed_ballots: usize,
    /// Offer "Propose a new vote" on the current roster head: `k ≥` the gap
    /// between the top two options (and `k > 0`).
    pub propose_new_vote: bool,
}

/// Builds the annotations. `members_of_h` is the electorate,
/// `current_members` the roster as this phone sees it now, `as_of_date`
/// the caller's rendering of when `H` was signed, `me` this phone's key.
pub fn annotations(
    open: &ValidOpen,
    tally: &Tally,
    members_of_h: &BTreeSet<IdentityKey>,
    current_members: &BTreeSet<IdentityKey>,
    as_of_date: &str,
    me: Option<&IdentityKey>,
) -> Annotations {
    let mut lines = vec![format!("{} members as of {as_of_date}", open.n())];
    if let Some(me) = me
        && !members_of_h.contains(me)
        && current_members.contains(me)
    {
        lines.push("Not on this vote's list (joined after it opened)".to_string());
    }
    let removed_ballots = members_of_h
        .iter()
        .filter(|k| !current_members.contains(k) && !tally.not_heard.contains(k))
        .count();
    let propose_new_vote = removed_ballots > 0
        && removed_ballots >= usize::try_from(top_two_gap(&tally.displayed)).unwrap_or(usize::MAX);
    if removed_ballots > 0 {
        lines.push(format!(
            "Includes {removed_ballots} ballot(s) from people removed after this vote opened"
        ));
        if propose_new_vote {
            lines.push("Propose a new vote".to_string());
        }
    }
    Annotations {
        lines,
        removed_ballots,
        propose_new_vote,
    }
}

/// The only events that notify (BRIEF_v2 §4 "Notify people only when a
/// vote becomes Final or Disputed, or when the organiser posts a
/// Decision"; D1 rule 9).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Notification {
    Final { option: u8 },
    Disputed,
    Decision,
}

/// Decides whether the change from `before` (`None` when the vote was just
/// seen) to `after` notifies. Provisional changes never do, nor does a
/// Final or Disputed state that was already shown.
pub fn notification(before: Option<&VoteState>, after: &VoteState) -> Option<Notification> {
    match after {
        VoteState::Final { option } => match before {
            Some(VoteState::Final { option: prev }) if prev == option => None,
            _ => Some(Notification::Final { option: *option }),
        },
        VoteState::Disputed { .. } => match before {
            Some(VoteState::Disputed { .. }) => None,
            _ => Some(Notification::Disputed),
        },
        VoteState::LeadingProvisional { .. } | VoteState::ResultProvisional { .. } => None,
    }
}

/// The organiser's separately typed Decision (D1 rule 8): "Decision by Ana
/// · vote still Provisional (Y9 N5, 7 not heard from)". Never carries the
/// solid badge; when the vote is already Final or Disputed the state word
/// follows instead.
pub fn decision_label(organiser: &str, open: &ValidOpen, tally: &Tally) -> String {
    let counts: Vec<String> = open
        .options()
        .iter()
        .enumerate()
        .map(|(i, o)| {
            format!(
                "{}{}",
                initial(o),
                tally.displayed.get(i).copied().unwrap_or(0)
            )
        })
        .collect();
    let k = tally.not_heard.len();
    match &tally.state {
        VoteState::Final { .. } => format!(
            "Decision by {organiser} · vote Final ({})",
            counts.join(" ")
        ),
        VoteState::Disputed { members } => format!(
            "Decision by {organiser} · vote Disputed ({} members sent conflicting ballots)",
            members.len()
        ),
        VoteState::LeadingProvisional { .. } | VoteState::ResultProvisional { .. } => {
            format!(
                "Decision by {organiser} · vote still Provisional ({}, {k} not heard from)",
                counts.join(" ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::backend::GroupId;
    use crate::crypto::ed25519::IdentitySigner;
    use crate::crypto::hash::Hash32;
    use crate::objects::vote_open::{VoteOpen, threshold};
    use crate::objects::{SignedHeader, kind};
    use crate::votes::tally::tally_payloads;

    fn key(i: u8) -> IdentityKey {
        IdentitySigner::from_seed(&[i; 32]).public()
    }

    fn members(n: u8) -> BTreeSet<IdentityKey> {
        (1..=n).map(key).collect()
    }

    fn open(n: u16) -> ValidOpen {
        let s = IdentitySigner::from_seed(&[1; 32]);
        let header = SignedHeader::new(
            kind::VOTE_OPEN,
            GroupId([1; 16]),
            s.public(),
            0,
            Hash32::ZERO,
            vec![],
        )
        .unwrap();
        ValidOpen::trusted(
            VoteOpen::sign(
                &s,
                header,
                "q",
                vec!["Yes".into(), "No".into()],
                1,
                threshold(n, 1),
                n,
            )
            .unwrap(),
        )
    }

    fn votes(yes: u8, no: u8) -> Vec<(IdentityKey, u8)> {
        (1..=yes)
            .map(|i| (key(i), 0))
            .chain((yes + 1..=yes + no).map(|i| (key(i), 1)))
            .collect()
    }

    #[test]
    fn card_header_matches_the_brief_example() {
        let o = open(12);
        assert_eq!(
            card_header(&o, Branch::A, Some(&target_label("18:00")), 2),
            "Final when 7 of 12 back one option · Target 18:00 · late votes still count · Ballots can't be changed · Group members can see how you voted · 2 members are Unverified"
        );
        assert_eq!(
            card_header(&o, Branch::B, None, 1),
            format!(
                "Final when 7 of 12 back one option · Ballots can't be changed · {VISIBILITY_B} · 1 member is Unverified"
            )
        );
        assert!(!card_header(&o, Branch::A, None, 0).contains("Unverified"));
        assert_eq!(WAITING_LABEL, "Waiting · needs this vote's member list");
    }

    #[test]
    fn state_labels_and_tally_line() {
        let o = open(12);
        let m = members(12);
        let t = tally_payloads(&o, &m, []);
        assert_eq!(state_label(&t), "Voting open");
        assert_eq!(tally_line(&o, &t), "Y0 N0 · Provisional");
        let t = tally_payloads(&o, &m, votes(3, 1));
        assert_eq!(
            state_label(&t),
            "Leading · Provisional (people near you only) · 8 not heard from"
        );
        let t = tally_payloads(&o, &m, votes(4, 3));
        assert_eq!(
            state_label(&t),
            "Result (provisional) · needs 7 for Final · 5 not heard from"
        );
        assert_eq!(tally_line(&o, &t), "Y4 N3 · Provisional");
        let t = tally_payloads(&o, &m, votes(7, 2));
        assert_eq!(state_label(&t), "Final");
        assert_eq!(tally_line(&o, &t), "Y7 N2 · Final");
        let mut p = votes(7, 2);
        p.push((key(1), 1));
        p.push((key(2), 1));
        let t = tally_payloads(&o, &m, p);
        assert_eq!(
            state_label(&t),
            "Disputed · 2 members sent conflicting ballots"
        );
        assert_eq!(tally_line(&o, &t), "Y5 N2 · 2 void · Disputed");
    }

    #[test]
    fn changed_after_reconnect_only_for_provisional_leads() {
        let o = open(12);
        let m = members(12);
        let before = tally_payloads(&o, &m, votes(2, 1));
        let after = tally_payloads(&o, &m, votes(2, 4));
        let c = changed_after_reconnect(&before, &after).unwrap();
        assert_eq!(
            c,
            ChangedAfterReconnect {
                from: Some(0),
                to: Some(1),
                newly_heard: 3,
                newly_void: 0
            }
        );
        assert_eq!(
            changed_label(&o, &c),
            "Changed after reconnect · was Yes, now No · 3 more heard from"
        );
        // same lead → none; tie → "now tied"; Final → none (that is a notification)
        assert_eq!(
            changed_after_reconnect(&before, &tally_payloads(&o, &m, votes(3, 1))),
            None
        );
        let tie = tally_payloads(&o, &m, votes(2, 2));
        let c = changed_after_reconnect(&before, &tie).unwrap();
        assert_eq!(c.to, None);
        assert_eq!(
            changed_label(&o, &c),
            "Changed after reconnect · was Yes, now tied · 1 more heard from"
        );
        assert_eq!(
            changed_after_reconnect(&before, &tally_payloads(&o, &m, votes(7, 1))),
            None
        );
        assert_eq!(
            changed_after_reconnect(&tally_payloads(&o, &m, []), &after),
            None
        );
        // a void key that flipped the lead is named in the reason
        let mut p = votes(2, 2);
        p.push((key(1), 1));
        let voided = tally_payloads(&o, &m, p);
        let c = changed_after_reconnect(&before, &voided).unwrap();
        assert_eq!(c.newly_void, 1);
        assert_eq!(c.to, Some(1));
        assert!(changed_label(&o, &c).ends_with("· 1 void"));
    }

    #[test]
    fn annotations_never_change_counts() {
        let o = open(12);
        let h = members(12);
        let t = tally_payloads(&o, &h, votes(5, 4));
        // two voters removed since, one newcomer (key 13) joined since
        let mut now = members(13);
        now.remove(&key(1));
        now.remove(&key(6));
        let a = annotations(&o, &t, &h, &now, "3 Oct", Some(&key(13)));
        assert_eq!(a.removed_ballots, 2);
        assert!(a.propose_new_vote, "k = 2 ≥ gap 1");
        assert_eq!(
            a.lines,
            vec![
                "12 members as of 3 Oct".to_string(),
                "Not on this vote's list (joined after it opened)".to_string(),
                "Includes 2 ballot(s) from people removed after this vote opened".to_string(),
                "Propose a new vote".to_string(),
            ]
        );
        // a member of H sees no "not on this list" line; a removed non-voter adds nothing
        let mut now = members(12);
        now.remove(&key(12));
        let a = annotations(&o, &t, &h, &now, "3 Oct", Some(&key(2)));
        assert_eq!(a.lines, vec!["12 members as of 3 Oct".to_string()]);
        assert!(!a.propose_new_vote);
        // k below the gap: annotation without the action
        let t = tally_payloads(&o, &h, votes(6, 1));
        let mut now = members(12);
        now.remove(&key(1));
        let a = annotations(&o, &t, &h, &now, "3 Oct", None);
        assert_eq!(a.removed_ballots, 1);
        assert!(!a.propose_new_vote);
        assert_eq!(a.lines.len(), 2);
    }

    #[test]
    fn notification_rule() {
        let fin = VoteState::Final { option: 0 };
        let disp = VoteState::Disputed {
            members: vec![key(1)],
        };
        let prov = VoteState::LeadingProvisional {
            leading: Some(0),
            k_not_heard: 3,
        };
        assert_eq!(
            notification(None, &fin),
            Some(Notification::Final { option: 0 })
        );
        assert_eq!(
            notification(Some(&prov), &fin),
            Some(Notification::Final { option: 0 })
        );
        assert_eq!(notification(Some(&fin), &fin), None);
        assert_eq!(
            notification(Some(&fin), &disp),
            Some(Notification::Disputed)
        );
        assert_eq!(notification(Some(&disp), &disp), None);
        assert_eq!(notification(None, &prov), None);
        assert_eq!(notification(Some(&fin), &prov), None);
    }

    #[test]
    fn decision_label_never_carries_final_for_a_provisional_vote() {
        let o = open(21);
        let m = members(21);
        let t = tally_payloads(&o, &m, votes(9, 5));
        assert_eq!(
            decision_label("Ana", &o, &t),
            "Decision by Ana · vote still Provisional (Y9 N5, 7 not heard from)"
        );
        let t = tally_payloads(&o, &m, votes(14, 7));
        assert_eq!(
            decision_label("Ana", &o, &t),
            "Decision by Ana · vote Final (Y14 N7)"
        );
    }
}
