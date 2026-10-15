//! Sender Key rotation triggers and the removed-author post verdict
//! (docs/DESIGN.md §3 "Sender Key rotation" and "Posts by removed members";
//! DECISIONS.md D3 K2 "A sender that learns of any removal rotates its own
//! sender key before its next post ... Also rotate every 24 h", branch B
//! "every 12 h with a random 0-10 min delay"; DECISIONS §3 row (1,2,3,4,10)
//! (ii) "M4 becomes deterministic ... No local timer is used").
//!
//! [`RotationPolicy`] only decides *whether* a rotation is due; the group
//! layer (`groups::group`) performs it. Time arrives as powered-on
//! milliseconds (`now_mono`, DESIGN §6 "Powered-on meter"); nothing here
//! reads a clock.
//!
//! [`post_verdict`] is the deterministic M4: a post whose own carried roster
//! heads already contain the author's removal is rejected everywhere; any
//! other post by a non-member is kept and labelled "Sent by [name], who was
//! removed" (BRIEF_v2 §5.4 "No local timer decides whether a removed
//! member's posts are accepted").

use crate::clock::ms;
use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::Hash32;
use crate::objects::roster_op::RosterBody;
use crate::roster::dag::RosterDag;
use crate::roster::eval::{RosterView, evaluate};

/// Build A: rotate every 24 h of powered-on time (D3 K2; assumption, tuned
/// at the gate-2 spike).
pub const ROTATION_INTERVAL_A_ASM: u64 = 24 * ms::HOUR;
/// Build B: rotate every 12 h of powered-on time (D3 branch B).
pub const ROTATION_INTERVAL_B_ASM: u64 = 12 * ms::HOUR;
/// Build B: largest random delay added to the interval (D3 branch B "0-10
/// min"). The caller draws the delay and passes it to
/// [`RotationPolicy::set_jitter_ms`]; this module holds no RNG.
pub const ROTATION_JITTER_B_ASM: u64 = 10 * ms::MINUTE;

/// When this phone's own sender chain must be replaced (DESIGN §3
/// "`pending_rotation` is set when (a) any valid Remove enters the DAG (of
/// anyone), (b) `powered_on − last_rotation ≥ 24 h` (B: 12 h + 0–10 min)").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RotationPolicy {
    interval_ms: u64,
    jitter_ms: u64,
    last_rotation_mono: u64,
    pending: bool,
}

impl RotationPolicy {
    /// A policy whose last rotation happened at `now_mono` (the chain
    /// created when the group was joined counts as the first rotation).
    pub fn new(interval_ms: u64, now_mono: u64) -> Self {
        Self {
            interval_ms,
            jitter_ms: 0,
            last_rotation_mono: now_mono,
            pending: false,
        }
    }

    /// Build A defaults ([`ROTATION_INTERVAL_A_ASM`], no jitter).
    pub fn build_a(now_mono: u64) -> Self {
        Self::new(ROTATION_INTERVAL_A_ASM, now_mono)
    }

    /// Build B defaults ([`ROTATION_INTERVAL_B_ASM`]); the caller sets the
    /// jitter from its RNG after every rotation.
    pub fn build_b(now_mono: u64) -> Self {
        Self::new(ROTATION_INTERVAL_B_ASM, now_mono)
    }

    /// Restores a persisted policy (`meta.last_rotation`, DESIGN §6).
    pub fn restore(interval_ms: u64, last_rotation_mono: u64, pending: bool) -> Self {
        Self {
            interval_ms,
            jitter_ms: 0,
            last_rotation_mono,
            pending,
        }
    }

    /// Random delay added to the interval (B build; clamped to
    /// [`ROTATION_JITTER_B_ASM`]).
    pub fn set_jitter_ms(&mut self, jitter_ms: u64) {
        self.jitter_ms = jitter_ms.min(ROTATION_JITTER_B_ASM);
    }

    pub fn interval_ms(&self) -> u64 {
        self.interval_ms
    }

    pub fn jitter_ms(&self) -> u64 {
        self.jitter_ms
    }

    pub fn last_rotation_mono(&self) -> u64 {
        self.last_rotation_mono
    }

    /// Set by a removal; cleared by [`RotationPolicy::on_rotated`].
    pub fn is_pending(&self) -> bool {
        self.pending
    }

    /// Trigger (a): a valid Remove (of anyone) entered the DAG (D3 K2 "Once
    /// a sender knows about a removal, none of its new posts uses a key the
    /// removed member holds").
    pub fn on_removal_seen(&mut self) {
        self.pending = true;
    }

    /// The chain was replaced at `now_mono`.
    pub fn on_rotated(&mut self, now_mono: u64) {
        self.pending = false;
        self.last_rotation_mono = now_mono;
    }

    /// Trigger (b) or a pending removal: must the next post go out under a
    /// new chain? Uses saturating arithmetic so a restored `last_rotation`
    /// in the future (a reset meter) means "not due yet", never a panic.
    pub fn due(&self, now_mono: u64) -> bool {
        if self.pending {
            return true;
        }
        let elapsed = now_mono.saturating_sub(self.last_rotation_mono);
        elapsed >= self.interval_ms.saturating_add(self.jitter_ms)
    }

    /// Powered-on time at which trigger (b) fires on its own.
    pub fn next_scheduled_mono(&self) -> u64 {
        self.last_rotation_mono
            .saturating_add(self.interval_ms)
            .saturating_add(self.jitter_ms)
    }

    /// Feeds newly applied ops (`RosterDag::drain_applied`) through trigger
    /// (a): any op among `applied` that is a Remove and valid in `view`
    /// (the view at the current heads) sets `pending`. Returns whether a
    /// removal was seen.
    pub fn observe_applied(
        &mut self,
        dag: &RosterDag,
        applied: &[Hash32],
        view: &RosterView,
    ) -> bool {
        let seen = applied.iter().any(|h| {
            matches!(dag.get(h).map(|o| &o.body), Some(RosterBody::Remove { .. }))
                && view.is_valid(h)
        });
        if seen {
            self.on_removal_seen();
        }
        seen
    }
}

/// What to do with a decrypted post by `author` (DESIGN §3 "Posts by removed
/// members"; DECISIONS §3 row (1,2,3,4,10) (ii)).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostVerdict {
    /// The author is a member at the current heads: show normally.
    Accept,
    /// The post's own carried heads already contain a valid removal of its
    /// author: hidden everywhere, its id goes to the "rejected" set so
    /// summary vectors stop re-offering it, and the link is penalised.
    RejectRemovedInHeads,
    /// The author is not a member at the current heads but the post was
    /// made without the removal in its past: keep, collapse and label
    /// "Sent by [name], who was removed" (`roster::labels`).
    KeepLabelledRemoved,
}

/// Judges a post by `author` carrying `post_heads` against the DAG and the
/// view at the current heads (`current`). No timers (DECISIONS §3 (ii)).
///
/// "Removed in heads" means: `closure(post_heads)` holds a valid
/// `Remove(author)` and the author is not a member at those heads (a member
/// re-added causally after the removal is not removed). If `post_heads` are
/// not all held yet the verdict is provisional: the caller displays it and
/// re-evaluates when the ops arrive (`RosterDag::resolves`); the final state
/// is identical on all phones.
pub fn post_verdict(
    dag: &RosterDag,
    author: &IdentityKey,
    post_heads: &[Hash32],
    current: &RosterView,
) -> PostVerdict {
    let own = evaluate(dag, post_heads);
    if own.removed.contains(author) {
        return PostVerdict::RejectRemovedInHeads;
    }
    if current.is_member(author) {
        PostVerdict::Accept
    } else {
        PostVerdict::KeepLabelledRemoved
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::backend::GroupId;
    use crate::crypto::ed25519::IdentitySigner;
    use crate::crypto::keycard::KeyCard;
    use crate::objects::roster_op::{MemberEntry, RosterOp};
    use crate::objects::{SignedHeader, kind};
    use crate::roster::dag::Insert;

    #[test]
    fn interval_trigger() {
        let mut p = RotationPolicy::build_a(1_000);
        assert_eq!(p.interval_ms(), ROTATION_INTERVAL_A_ASM);
        assert!(!p.due(1_000));
        assert!(!p.due(1_000 + ROTATION_INTERVAL_A_ASM - 1));
        assert!(p.due(1_000 + ROTATION_INTERVAL_A_ASM));
        assert_eq!(p.next_scheduled_mono(), 1_000 + ROTATION_INTERVAL_A_ASM);
        p.on_rotated(5_000);
        assert_eq!(p.last_rotation_mono(), 5_000);
        assert!(!p.due(5_000 + ROTATION_INTERVAL_A_ASM - 1));
        // a meter that went backwards never panics and is "not due"
        assert!(!p.due(0));
        let q = RotationPolicy::restore(10, u64::MAX, false);
        assert!(!q.due(5));
        assert_eq!(q.next_scheduled_mono(), u64::MAX);
    }

    #[test]
    fn removal_trigger_and_jitter() {
        let mut p = RotationPolicy::build_b(0);
        assert_eq!(p.interval_ms(), ROTATION_INTERVAL_B_ASM);
        p.set_jitter_ms(ms::HOUR);
        assert_eq!(p.jitter_ms(), ROTATION_JITTER_B_ASM);
        p.set_jitter_ms(5 * ms::MINUTE);
        assert!(!p.due(ROTATION_INTERVAL_B_ASM + 5 * ms::MINUTE - 1));
        assert!(p.due(ROTATION_INTERVAL_B_ASM + 5 * ms::MINUTE));
        assert!(!p.is_pending());
        p.on_removal_seen();
        assert!(p.is_pending());
        assert!(p.due(1));
        p.on_rotated(1);
        assert!(!p.is_pending());
        assert!(!p.due(1));
        let r = RotationPolicy::restore(7, 3, true);
        assert!(r.is_pending() && r.due(3));
    }

    fn signer(i: u8) -> IdentitySigner {
        IdentitySigner::from_seed(&[i; 32])
    }

    fn card(i: u8) -> KeyCard {
        KeyCard::issue(&signer(i), [i; 32])
    }

    fn genesis() -> RosterOp {
        let s = signer(1);
        RosterOp::sign(
            &s,
            SignedHeader::new(
                kind::ROSTER_GENESIS,
                GroupId([0; 16]),
                s.public(),
                0,
                Hash32::ZERO,
                vec![],
            )
            .unwrap(),
            RosterBody::Genesis {
                nonce: [1; 16],
                name: "g".into(),
                admins: vec![card(1), card(2)],
                members: vec![MemberEntry {
                    keycard: card(3),
                    name: "three".into(),
                    in_person: true,
                }],
                settings: 0,
            },
        )
        .unwrap()
    }

    fn remove(author: u8, seq: u32, gid: GroupId, heads: Vec<Hash32>, subject: u8) -> RosterOp {
        let s = signer(author);
        RosterOp::sign(
            &s,
            SignedHeader::new(
                kind::ROSTER_REMOVE,
                gid,
                s.public(),
                seq,
                Hash32::ZERO,
                heads,
            )
            .unwrap(),
            RosterBody::Remove {
                subject: signer(subject).public(),
            },
        )
        .unwrap()
    }

    #[test]
    fn post_verdicts_and_observe_applied() {
        let g = genesis();
        let gid = g.group_id();
        let gh = g.op_hash();
        let mut dag = RosterDag::from_genesis(g).unwrap();
        let three = signer(3).public();
        let current = evaluate(&dag, &dag.heads());
        assert_eq!(
            post_verdict(&dag, &three, &[gh], &current),
            PostVerdict::Accept
        );
        let r = remove(1, 1, gid, vec![gh], 3);
        let rh = r.op_hash();
        assert_eq!(dag.insert(r), Insert::Applied(rh));
        let _ = dag.drain_applied();
        let current = evaluate(&dag, &dag.heads());
        let mut p = RotationPolicy::build_a(0);
        assert!(p.observe_applied(&dag, &[gh, rh], &current));
        assert!(p.is_pending());
        // a post made before the removal: kept and labelled
        assert_eq!(
            post_verdict(&dag, &three, &[gh], &current),
            PostVerdict::KeepLabelledRemoved
        );
        // a post whose heads include the removal: rejected
        assert_eq!(
            post_verdict(&dag, &three, &[rh], &current),
            PostVerdict::RejectRemovedInHeads
        );
        // an honest member is unaffected
        let two = signer(2).public();
        assert_eq!(
            post_verdict(&dag, &two, &[rh], &current),
            PostVerdict::Accept
        );
        // an invalid Remove (by a non-admin) does not trigger rotation
        let bad = remove(3, 0, gid, vec![rh], 2);
        let bh = bad.op_hash();
        assert_eq!(dag.insert(bad), Insert::Applied(bh));
        let current = evaluate(&dag, &dag.heads());
        let mut q = RotationPolicy::build_a(0);
        assert!(!q.observe_applied(&dag, &[bh], &current));
        assert!(!q.is_pending());
        // unknown heads: provisional, judged on what is held
        let ghost = Hash32::from_bytes([7; 32]);
        assert_eq!(
            post_verdict(&dag, &three, &[ghost], &current),
            PostVerdict::KeepLabelledRemoved
        );
    }
}
