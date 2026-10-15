//! The node's own bounded fragment collector (docs/DESIGN.md §1 row
//! `wire/inner.rs` "Reassembler (bounded, per chain)"; §6 `reassembly`
//! table).
//!
//! The node keeps reassembly here rather than inside the group and
//! pairwise managers so that the state can be snapshotted (it is `Clone`)
//! before each transaction and restored when the transaction fails: the
//! managers themselves are replayed from the database on a rollback, and
//! partial objects are the one piece of protocol state that is deliberately
//! not persisted (a restart starts reassembly afresh; the DTN store keeps
//! the cells). Semantics follow [`crate::wire::inner::Reassembler`]: one
//! partial object per `(chain, obj_seq)`, every header checked by
//! [`InnerHeader::parse`], a mismatching fragment rejected, a duplicate
//! fragment ignored, at most [`MAX_IN_PROGRESS`] partial objects with the
//! oldest evicted first, so a flood of first fragments cannot grow memory.

use std::collections::{BTreeMap, VecDeque};

use crate::wire::cell::BODY_PLAINTEXT_LEN;
use crate::wire::inner::{InnerError, InnerHeader, InnerKind};

/// Partial objects kept at once (`wire::inner::DEFAULT_MAX_IN_PROGRESS`).
pub const MAX_IN_PROGRESS: usize = crate::wire::inner::DEFAULT_MAX_IN_PROGRESS;

#[derive(Debug, Clone)]
struct Partial {
    kind: InnerKind,
    frags: Vec<Option<Vec<u8>>>,
    have: usize,
}

/// Bounded reassembly of inner objects keyed by `(chain, obj_seq)`.
#[derive(Debug, Clone, Default)]
pub struct Reasm {
    partials: BTreeMap<(u64, u32), Partial>,
    order: VecDeque<(u64, u32)>,
}

impl Reasm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_progress(&self) -> usize {
        self.partials.len()
    }

    /// Drops every partial object of `chain`.
    pub fn forget_chain(&mut self, chain: u64) {
        self.partials.retain(|(c, _), _| *c != chain);
        self.order.retain(|(c, _)| *c != chain);
    }

    /// Feeds one body plaintext; returns the completed object once.
    pub fn push(
        &mut self,
        chain: u64,
        plain: &[u8; BODY_PLAINTEXT_LEN],
    ) -> Result<Option<(InnerKind, u32, Vec<u8>)>, InnerError> {
        let (header, data) = InnerHeader::parse(plain)?;
        let total = usize::from(header.frag_total);
        let slot = usize::from(header.frag_idx);
        if total == 1 {
            // A single-fragment object completes at once; an open partial
            // under the same key contradicts it and is dropped.
            self.remove(&(chain, header.obj_seq));
            return Ok(Some((header.kind, header.obj_seq, data.to_vec())));
        }
        let key = (chain, header.obj_seq);
        if let Some(p) = self.partials.get_mut(&key) {
            if p.kind != header.kind || p.frags.len() != total {
                return Err(InnerError::Mismatch);
            }
            match p.frags.get_mut(slot) {
                Some(entry @ None) => {
                    *entry = Some(data.to_vec());
                    p.have = p.have.saturating_add(1);
                }
                Some(Some(_)) => return Ok(None),
                None => return Err(InnerError::Mismatch),
            }
            if p.have == total {
                let Some(p) = self.remove(&key) else {
                    return Ok(None);
                };
                let mut out = Vec::new();
                for d in p.frags.iter().flatten() {
                    out.extend_from_slice(d);
                }
                return Ok(Some((p.kind, header.obj_seq, out)));
            }
            return Ok(None);
        }
        while self.partials.len() >= MAX_IN_PROGRESS {
            match self.order.pop_front() {
                Some(old) => {
                    self.partials.remove(&old);
                }
                None => break,
            }
        }
        let mut frags = vec![None; total];
        if let Some(entry) = frags.get_mut(slot) {
            *entry = Some(data.to_vec());
        } else {
            return Err(InnerError::Mismatch);
        }
        self.partials.insert(
            key,
            Partial {
                kind: header.kind,
                frags,
                have: 1,
            },
        );
        self.order.push_back(key);
        Ok(None)
    }

    fn remove(&mut self, key: &(u64, u32)) -> Option<Partial> {
        let p = self.partials.remove(key);
        if p.is_some() {
            self.order.retain(|k| k != key);
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::inner::fragment;
    use proptest::prelude::*;

    #[test]
    fn reassembles_in_any_order_and_bounds_partials() {
        let object: Vec<u8> = (0..2000u32).map(|i| i as u8).collect();
        let frags = fragment(InnerKind::SenderKey, 7, &object).unwrap();
        let mut r = Reasm::new();
        let mut done = None;
        for f in frags.iter().rev() {
            if let Some(o) = r.push(1, f).unwrap() {
                done = Some(o);
            }
        }
        assert_eq!(done, Some((InnerKind::SenderKey, 7, object.clone())));
        assert_eq!(r.in_progress(), 0);
        // a duplicate fragment is ignored, a single-fragment object completes at once
        let one = fragment(InnerKind::PairwiseDr, 9, b"x").unwrap();
        assert!(r.push(1, &one[0]).unwrap().is_some());
        // bounded
        for seq in 0..(MAX_IN_PROGRESS as u32 + 10) {
            let f = fragment(InnerKind::SenderKey, seq, &object).unwrap();
            assert!(r.push(2, &f[0]).unwrap().is_none());
        }
        assert_eq!(r.in_progress(), MAX_IN_PROGRESS);
        r.forget_chain(2);
        assert_eq!(r.in_progress(), 0);
    }

    proptest! {
        #[test]
        fn never_panics_on_random_bodies(bytes in prop::collection::vec(any::<u8>(), BODY_PLAINTEXT_LEN)) {
            let mut r = Reasm::new();
            let mut arr = [0u8; BODY_PLAINTEXT_LEN];
            arr.copy_from_slice(&bytes);
            let _ = r.push(0, &arr);
        }
    }
}
