//! The "Needs someone" pool: evidence-age rule (docs/DESIGN.md §4 "Needs
//! someone"; BRIEF_v2 §4 "A task whose holder has not been heard from for a
//! while moves to Needs someone"; DECISIONS D10; docs/decisions-assumed.md
//! row 7).
//!
//! No op is emitted and nothing here reads a clock: the caller passes
//! powered-on monotonic milliseconds (`Clock::mono_ms`) for "now", for the
//! Take, and for the last time each holder was heard from (any cell
//! recognised from that member's chains or any identity-signed object). A
//! task needs someone when every holder has been silent for
//! [`NEEDS_SOMEONE_AFTER_ASM`] of powered-on time since the later of the
//! Take and the holder's last evidence. The wall-clock string ("14:05") is
//! display only and is supplied by the caller from `Clock::wall_for_display`.

use std::collections::BTreeMap;

use crate::crypto::ed25519::IdentityKey;
use crate::tasks::state::TaskState;

/// 6 h of powered-on time without hearing from the holder (row 7:
/// "assumption, tune in field tests"). Milliseconds, like `Clock::mono_ms`.
pub const NEEDS_SOMEONE_AFTER_ASM: u64 = 6 * 60 * 60 * 1000;

/// Why a task is in the pool: the newest evidence any holder produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceAge {
    /// Powered-on time of the newest evidence (the Take itself if no holder
    /// has been heard from since); the caller converts it for display.
    pub since_mono: u64,
    /// The holder that evidence came from (named in the B-build label).
    pub holder: IdentityKey,
}

/// Evaluates the pool rule for one task. `None` when the task has no
/// holder, when any holder was heard from recently enough, or when the
/// threshold has not yet elapsed since the Take.
pub fn needs_someone(
    task_state: &TaskState,
    last_heard_mono: &BTreeMap<IdentityKey, u64>,
    taken_at_mono: u64,
    now_mono: u64,
) -> Option<EvidenceAge> {
    let holders = task_state.holders();
    let mut newest: Option<EvidenceAge> = None;
    for holder in holders {
        let heard = last_heard_mono.get(&holder).copied().unwrap_or(0);
        let since_mono = heard.max(taken_at_mono);
        let age = now_mono.saturating_sub(since_mono);
        if age < NEEDS_SOMEONE_AFTER_ASM {
            return None;
        }
        let candidate = EvidenceAge { since_mono, holder };
        newest = Some(match newest {
            Some(cur) if cur.since_mono >= since_mono => cur,
            _ => candidate,
        });
    }
    newest
}

/// Branch A pool label: "not heard from since HH:MM" (BRIEF_v2 §4; D10).
/// `display_time` is the caller's wall-clock rendering of `since_mono`.
pub fn needs_someone_label(display_time: &str) -> String {
    format!("not heard from since {display_time}")
}

/// Branch B label shown in place of the pool: "No update from [name] since
/// HH:MM (phone may be closed)" (BRIEF_v2 §4; D10).
pub fn no_update_label(name: &str, display_time: &str) -> String {
    format!("No update from {name} since {display_time} (phone may be closed)")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;

    fn key(i: u8) -> IdentityKey {
        IdentitySigner::from_seed(&[i; 32]).public()
    }

    const H: u64 = 60 * 60 * 1000;

    #[test]
    fn open_and_done_never_need_someone() {
        let heard = BTreeMap::new();
        assert_eq!(needs_someone(&TaskState::Open, &heard, 0, 100 * H), None);
        assert_eq!(needs_someone(&TaskState::Done, &heard, 0, 100 * H), None);
    }

    #[test]
    fn threshold_counts_from_the_later_of_take_and_last_heard() {
        let taken = TaskState::Taken {
            holders: vec![key(1)],
        };
        let mut heard = BTreeMap::new();
        // never heard: the Take itself is the evidence
        assert_eq!(
            needs_someone(&taken, &heard, 10 * H, 10 * H + NEEDS_SOMEONE_AFTER_ASM - 1),
            None
        );
        assert_eq!(
            needs_someone(&taken, &heard, 10 * H, 10 * H + NEEDS_SOMEONE_AFTER_ASM),
            Some(EvidenceAge {
                since_mono: 10 * H,
                holder: key(1)
            })
        );
        // heard after the Take: the clock restarts from that evidence
        heard.insert(key(1), 13 * H);
        assert_eq!(needs_someone(&taken, &heard, 10 * H, 18 * H), None);
        assert_eq!(
            needs_someone(&taken, &heard, 10 * H, 19 * H),
            Some(EvidenceAge {
                since_mono: 13 * H,
                holder: key(1)
            })
        );
        // heard before the Take: the Take is newer
        heard.insert(key(1), 2 * H);
        assert_eq!(
            needs_someone(&taken, &heard, 10 * H, 16 * H),
            Some(EvidenceAge {
                since_mono: 10 * H,
                holder: key(1)
            })
        );
        // now before the Take (clock reset): no underflow, not in the pool
        assert_eq!(needs_someone(&taken, &heard, 10 * H, 1), None);
    }

    #[test]
    fn any_active_holder_keeps_the_task_out_of_the_pool() {
        let two = TaskState::Taken {
            holders: vec![key(1), key(2)],
        };
        let mut heard = BTreeMap::new();
        heard.insert(key(1), H);
        heard.insert(key(2), 5 * H);
        assert_eq!(needs_someone(&two, &heard, 0, 10 * H), None);
        let got = needs_someone(&two, &heard, 0, 12 * H).unwrap();
        assert_eq!(
            got,
            EvidenceAge {
                since_mono: 5 * H,
                holder: key(2)
            }
        );
        let conflict = TaskState::Conflict {
            winner: key(1),
            losers: vec![key(2)],
            second: vec![],
        };
        assert_eq!(needs_someone(&conflict, &heard, 0, 12 * H), Some(got));
    }

    #[test]
    fn labels_match_the_brief() {
        assert_eq!(needs_someone_label("14:05"), "not heard from since 14:05");
        assert_eq!(
            no_update_label("Ravi", "14:05"),
            "No update from Ravi since 14:05 (phone may be closed)"
        );
    }
}
