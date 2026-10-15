//! Deterministic, non-grindable conflict tie-break (docs/DESIGN.md §4
//! "Tie-break"; docs/decisions-assumed.md row 6; BRIEF_v2 §4 "Conflict:
//! Ravi also took this · a deterministic rule picks one").
//!
//! For the conflict set `C` of concurrent Take ops, each named by its
//! author-bound op hash ([`crate::objects::task_op::task_op_hash`], DESIGN
//! §2.4):
//!
//! ```text
//! R        = SHA256(sorted op hashes of C)
//! score(k) = SHA256("meshapp.v1.tiebreak" ‖ task_id ‖ R ‖ k)
//! winner   = the key with the lowest score
//! ```
//!
//! Why this shape (DESIGN §4): keys are fixed in the roster before any task
//! exists and every task re-randomises the ranking through `task_id` and
//! `R`, so no key choice wins in general; `R` depends on every rival's op
//! hash, so neither the task creator nor any single taker knows the
//! ranking before the conflict exists. Because the op hash binds the author,
//! two rivals whose Take posts are byte-identical still contribute two
//! distinct hashes to `R`, one per claim. Every phone computes the same
//! winner from the same set (`tests/tasks_votes.rs` checks permutation
//! invariance and the fairness bound).

use crate::crypto::ed25519::IdentityKey;
use crate::crypto::hash::{Hash32, sha256, sha256_concat};
use crate::objects::task_op::TaskId;

/// Domain prefix of the score hash.
pub const TIEBREAK_DOMAIN: &[u8] = b"meshapp.v1.tiebreak";

/// `R = SHA256(sorted op hashes)`: duplicates are collapsed so a set, not a
/// list, defines the ranking (a repeated hash is the same op listed twice).
pub fn ranking_seed(concurrent_takes: &[(Hash32, IdentityKey)]) -> Hash32 {
    let mut hashes: Vec<Hash32> = concurrent_takes.iter().map(|(h, _)| *h).collect();
    hashes.sort_unstable();
    hashes.dedup();
    let mut bytes = Vec::with_capacity(hashes.len() * 32);
    for h in &hashes {
        bytes.extend_from_slice(h.as_bytes());
    }
    sha256(&bytes)
}

/// `score(k) = SHA256("meshapp.v1.tiebreak" ‖ task_id ‖ R ‖ k)`.
pub fn score(task_id: &TaskId, r: &Hash32, key: &IdentityKey) -> Hash32 {
    sha256_concat(&[TIEBREAK_DOMAIN, task_id, r.as_bytes(), key.as_bytes()])
}

/// The winner of the conflict set, or `None` for an empty set. Ties in the
/// score (a SHA-256 collision between two distinct keys) fall to the lower
/// key so the result is total; the same key listed twice is one contender.
pub fn winner(task_id: &TaskId, concurrent_takes: &[(Hash32, IdentityKey)]) -> Option<IdentityKey> {
    let r = ranking_seed(concurrent_takes);
    concurrent_takes
        .iter()
        .map(|(_, k)| (score(task_id, &r, k), *k))
        .min()
        .map(|(_, k)| k)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;

    fn key(i: u8) -> IdentityKey {
        IdentitySigner::from_seed(&[i; 32]).public()
    }

    fn h(i: u8) -> Hash32 {
        Hash32::from_bytes([i; 32])
    }

    #[test]
    fn empty_set_has_no_winner_and_singleton_wins() {
        assert_eq!(winner(&[0; 16], &[]), None);
        assert_eq!(winner(&[0; 16], &[(h(1), key(1))]), Some(key(1)));
    }

    #[test]
    fn permutation_and_duplicate_invariant() {
        let a = [(h(1), key(1)), (h(2), key(2)), (h(3), key(3))];
        let b = [
            (h(3), key(3)),
            (h(1), key(1)),
            (h(2), key(2)),
            (h(1), key(1)),
        ];
        assert_eq!(ranking_seed(&a), ranking_seed(&b));
        assert_eq!(winner(&[7; 16], &a), winner(&[7; 16], &b));
        assert!(a.iter().any(|(_, k)| Some(*k) == winner(&[7; 16], &a)));
    }

    #[test]
    fn lowest_score_wins_and_r_is_the_sorted_concatenation() {
        // docs/decisions-assumed.md row 6: R = SHA256(sorted op hashes of C),
        // lowest score wins.
        let set = [
            (h(9), key(1)),
            (h(2), key(2)),
            (h(5), key(3)),
            (h(2), key(4)),
        ];
        let mut pre = Vec::new();
        for i in [2u8, 5, 9] {
            pre.extend_from_slice(h(i).as_bytes());
        }
        let r = sha256(&pre);
        assert_eq!(ranking_seed(&set), r);
        for tid in [[0u8; 16], [7; 16], [0xff; 16]] {
            let w = winner(&tid, &set).unwrap();
            let min = set.iter().map(|(_, k)| score(&tid, &r, k)).min().unwrap();
            assert_eq!(
                score(&tid, &r, &w),
                min,
                "winner must have the lowest score"
            );
            assert!(set.iter().all(|(_, k)| score(&tid, &r, k) >= min));
        }
        // the ranking is not constant over task ids (it re-randomises)
        let winners: std::collections::BTreeSet<IdentityKey> =
            (0u8..32).map(|i| winner(&[i; 16], &set).unwrap()).collect();
        assert!(winners.len() >= 2);
    }

    #[test]
    fn seed_and_task_id_re_randomise() {
        let a = [(h(1), key(1)), (h(2), key(2))];
        let r1 = ranking_seed(&a);
        let r2 = ranking_seed(&[(h(1), key(1)), (h(9), key(2))]);
        assert_ne!(r1, r2);
        assert_ne!(score(&[0; 16], &r1, &key(1)), score(&[1; 16], &r1, &key(1)));
        assert_ne!(score(&[0; 16], &r1, &key(1)), score(&[0; 16], &r2, &key(1)));
        // known structure: the score is a plain SHA-256 over the documented concatenation
        let mut pre = TIEBREAK_DOMAIN.to_vec();
        pre.extend_from_slice(&[0; 16]);
        pre.extend_from_slice(r1.as_bytes());
        pre.extend_from_slice(key(1).as_bytes());
        assert_eq!(score(&[0; 16], &r1, &key(1)), sha256(&pre));
    }
}
