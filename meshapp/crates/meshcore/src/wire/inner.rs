//! The inner fragment layer: the 476-byte body plaintext of a cell
//! (docs/DESIGN.md §2.2 "Inner plaintext"; BRIEF_v2 §5.5, §5.6 "segmented,
//! objects up to 64 kB").
//!
//! | off | len | field |
//! |---|---|---|
//! | 0 | 1 | [`InnerKind`] |
//! | 1 | 4 | `obj_seq` u32: per-chain object counter |
//! | 5 | 2 | `frag_idx` u16 |
//! | 7 | 2 | `frag_total` u16 (1..=[`MAX_FRAGS`]) |
//! | 9 | 2 | `len` u16: bytes of data in this fragment (≤ [`DATA_LEN`]) |
//! | 11 | 465 | data, zero-padded |
//!
//! [`fragment`] splits one object into its fragments and [`Reassembler`]
//! collects fragments back into objects, keyed by `(chain, obj_seq)` where
//! the chain is whatever the caller recognised the cell under (the tag table
//! in `wire/recog.rs` or the seal trial in `wire/seal.rs`). The fragment
//! layer is below end-to-end decryption: a fragment's data is opaque here,
//! including the `e_pub` that a [`InnerKind::SealedFirst`] object carries
//! in its first [`E_PUB_LEN`] bytes ([`sealed_first_split`]).
//!
//! Rules (all deterministic; the same inputs give the same bytes/verdict):
//! * `frag_total` in `1..=MAX_FRAGS`, `frag_idx < frag_total`;
//! * `len == DATA_LEN` for every fragment but the last, `1..=DATA_LEN` for
//!   the last, so an object has exactly one fragmentation and no fragment is
//!   empty (DESIGN §2.1 "canonical by construction");
//! * padding after the data is zero (a decoder rejects anything else, like
//!   trailing bytes in the codec);
//! * duplicates are ignored, a fragment whose kind or `frag_total` disagrees
//!   with the in-progress object rejects that object (a `frag_total` of 1
//!   included: one `(chain, obj_seq)` is one object), and the number of
//!   in-progress objects is bounded (oldest by insertion evicted first), so
//!   a peer cannot make the receiver hold unbounded partial state (BRIEF_v2
//!   §5.5 "every relay checks format, size").
//!
//! Nothing here reads a clock (BRIEF_v2 §4).

use std::collections::VecDeque;

use crate::codec::{CodecError, Decode, Encode, Reader, Writer, decode, unknown_variant};
use crate::wire::cell::BODY_PLAINTEXT_LEN;

/// Encoded size of [`InnerHeader`].
pub const INNER_HEADER_LEN: usize = 11;
/// Data bytes per fragment (DESIGN §2.2: 465).
pub const DATA_LEN: usize = BODY_PLAINTEXT_LEN - INNER_HEADER_LEN;
/// Largest `frag_total` (DESIGN §2.2 "≤ 141 → 64 kB C1 cap").
pub const MAX_FRAGS: usize = 141;
/// Largest object the fragment layer carries: `MAX_FRAGS * DATA_LEN` =
/// 65 565 bytes, just above the 64 kB C1 object cap (BRIEF_v2 §5.6).
pub const MAX_OBJECT_LEN: usize = MAX_FRAGS * DATA_LEN;
/// Length of the clear `e_pub` at the start of a [`InnerKind::SealedFirst`]
/// object (DESIGN §2.2 "bytes 11..43 = e_pub").
pub const E_PUB_LEN: usize = 32;
/// In-progress objects a [`Reassembler::default`] holds before evicting the
/// oldest. 64 objects × 141 fragments × 465 B bounds the partial state at
/// about 4 MB per reassembler (assumption; tune in the D10 spike).
pub const DEFAULT_MAX_IN_PROGRESS: usize = 64;

const _: () = assert!(DATA_LEN == 465);
const _: () = assert!(MAX_OBJECT_LEN >= 64 * 1024);
const _: () = assert!(MAX_FRAGS <= u16::MAX as usize);

/// What the object inside a chain's cells is (DESIGN §2.2 inner `kind`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InnerKind {
    /// Pairwise Double Ratchet message (BRIEF_v2 §5.1, §5.2).
    PairwiseDr = 0x01,
    /// Group Sender Key message (BRIEF_v2 §5.2).
    SenderKey = 0x02,
    /// Sealed first flight to a gossiped envelope key: `e_pub [32]` clear,
    /// then the AEAD under `k_seal` (D6 pick B (2); DESIGN §2.4).
    SealedFirst = 0x03,
    /// Padded SKDM bundle (D3 K2; DESIGN §2.4).
    SkdmBundle = 0x04,
    /// Cumulative receipt batch, pairwise (D4 MERGE).
    Receipt = 0x05,
    /// SOS (C0; reserved at P0, owner decision 5).
    Sos = 0x06,
}

impl InnerKind {
    pub const ALL: [InnerKind; 6] = [
        InnerKind::PairwiseDr,
        InnerKind::SenderKey,
        InnerKind::SealedFirst,
        InnerKind::SkdmBundle,
        InnerKind::Receipt,
        InnerKind::Sos,
    ];

    pub fn from_u8(v: u8) -> Result<Self, CodecError> {
        Ok(match v {
            0x01 => InnerKind::PairwiseDr,
            0x02 => InnerKind::SenderKey,
            0x03 => InnerKind::SealedFirst,
            0x04 => InnerKind::SkdmBundle,
            0x05 => InnerKind::Receipt,
            0x06 => InnerKind::Sos,
            other => return Err(unknown_variant("inner kind", other)),
        })
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

impl Encode for InnerKind {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.as_u8());
        Ok(())
    }
}

impl Decode for InnerKind {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        InnerKind::from_u8(r.u8()?)
    }
}

/// Why a fragment or an object was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InnerError {
    /// The 11-byte header did not decode (unknown kind, short input).
    #[error("inner header: {0}")]
    Codec(#[from] CodecError),
    /// [`fragment`] was given an empty object; every fragment carries data.
    #[error("object is empty")]
    EmptyObject,
    /// [`fragment`] was given more than [`MAX_OBJECT_LEN`] bytes.
    #[error("object of {len} bytes exceeds the {max}-byte fragmentation cap")]
    ObjectTooLarge { len: usize, max: usize },
    /// `frag_total` is 0 or above [`MAX_FRAGS`].
    #[error("frag_total {0} is outside 1..={MAX_FRAGS}")]
    BadFragTotal(u16),
    /// `frag_idx` is not below `frag_total`.
    #[error("frag_idx {idx} is not below frag_total {total}")]
    BadFragIdx { idx: u16, total: u16 },
    /// `len` is above [`DATA_LEN`], not exactly [`DATA_LEN`] on a non-final
    /// fragment, or 0 on the final one.
    #[error("fragment len {len} is invalid for a {} fragment", if *.is_final { "final" } else { "non-final" })]
    BadLen { len: u16, is_final: bool },
    /// Bytes after the fragment's data are not all zero.
    #[error("padding after the fragment data is not zero")]
    NonZeroPadding,
    /// The fragment's kind or `frag_total` differs from the fragments of the
    /// same `(chain, obj_seq)` already held; that object is dropped.
    #[error("fragment disagrees with the in-progress object's kind or frag_total")]
    Mismatch,
}

/// The fragment header (DESIGN §2.2 inner plaintext, bytes 0..11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InnerHeader {
    pub kind: InnerKind,
    /// Per-chain object counter.
    pub obj_seq: u32,
    pub frag_idx: u16,
    pub frag_total: u16,
    /// Data bytes in this fragment.
    pub len: u16,
}

impl InnerHeader {
    /// Encoded size.
    pub const LEN: usize = INNER_HEADER_LEN;

    /// Whether this is the object's last fragment.
    pub fn is_final(&self) -> bool {
        u32::from(self.frag_idx) + 1 == u32::from(self.frag_total)
    }

    /// The header's own consistency rules (module docs): `frag_total` in
    /// `1..=MAX_FRAGS`, `frag_idx < frag_total`, `len` exactly [`DATA_LEN`]
    /// unless final, then `1..=DATA_LEN`.
    pub fn check(&self) -> Result<(), InnerError> {
        let total = usize::from(self.frag_total);
        if total == 0 || total > MAX_FRAGS {
            return Err(InnerError::BadFragTotal(self.frag_total));
        }
        if self.frag_idx >= self.frag_total {
            return Err(InnerError::BadFragIdx {
                idx: self.frag_idx,
                total: self.frag_total,
            });
        }
        let len = usize::from(self.len);
        let is_final = self.is_final();
        let ok = if is_final {
            (1..=DATA_LEN).contains(&len)
        } else {
            len == DATA_LEN
        };
        if !ok {
            return Err(InnerError::BadLen {
                len: self.len,
                is_final,
            });
        }
        Ok(())
    }

    /// Parses a body plaintext into its header and data slice, applying
    /// [`InnerHeader::check`] and the zero-padding rule. Never panics on any
    /// input: the header is read through the bounds-checked codec and the
    /// data slice is cut only after `len ≤ DATA_LEN` is known.
    pub fn parse(plain: &[u8; BODY_PLAINTEXT_LEN]) -> Result<(Self, &[u8]), InnerError> {
        let (head, rest) = plain.split_at(INNER_HEADER_LEN);
        let header: InnerHeader = decode(head)?;
        header.check()?;
        let len = usize::from(header.len);
        let (data, padding) = rest.split_at(len.min(rest.len()));
        if data.len() != len || padding.iter().any(|&b| b != 0) {
            return Err(InnerError::NonZeroPadding);
        }
        Ok((header, data))
    }

    /// Writes `self ‖ data ‖ zero padding` as one body plaintext. `data`
    /// must be exactly `self.len` bytes and the header must pass
    /// [`InnerHeader::check`].
    pub fn to_plaintext(&self, data: &[u8]) -> Result<[u8; BODY_PLAINTEXT_LEN], InnerError> {
        self.check()?;
        if data.len() != usize::from(self.len) {
            return Err(InnerError::BadLen {
                len: self.len,
                is_final: self.is_final(),
            });
        }
        let mut out = [0u8; BODY_PLAINTEXT_LEN];
        let head = self.try_encode()?;
        out[..INNER_HEADER_LEN].copy_from_slice(&head);
        out[INNER_HEADER_LEN..INNER_HEADER_LEN + data.len()].copy_from_slice(data);
        Ok(out)
    }
}

impl Encode for InnerHeader {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.kind.as_u8())
            .u32(self.obj_seq)
            .u16(self.frag_idx)
            .u16(self.frag_total)
            .u16(self.len);
        Ok(())
    }
}

impl Decode for InnerHeader {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(InnerHeader {
            kind: InnerKind::decode_from(r)?,
            obj_seq: r.u32()?,
            frag_idx: r.u16()?,
            frag_total: r.u16()?,
            len: r.u16()?,
        })
    }
}

/// Splits `object` into its body plaintexts: `ceil(len / DATA_LEN)`
/// fragments, each full except the last, zero-padded, with
/// `frag_idx = 0..frag_total`. Errors on an empty object or one above
/// [`MAX_OBJECT_LEN`] (the C1 64 kB segmentation ceiling, BRIEF_v2 §5.6).
pub fn fragment(
    kind: InnerKind,
    obj_seq: u32,
    object: &[u8],
) -> Result<Vec<[u8; BODY_PLAINTEXT_LEN]>, InnerError> {
    if object.is_empty() {
        return Err(InnerError::EmptyObject);
    }
    if object.len() > MAX_OBJECT_LEN {
        return Err(InnerError::ObjectTooLarge {
            len: object.len(),
            max: MAX_OBJECT_LEN,
        });
    }
    let frag_total =
        u16::try_from(object.len().div_ceil(DATA_LEN)).map_err(|_| InnerError::ObjectTooLarge {
            len: object.len(),
            max: MAX_OBJECT_LEN,
        })?;
    let mut out = Vec::with_capacity(usize::from(frag_total));
    for (i, chunk) in object.chunks(DATA_LEN).enumerate() {
        let frag_idx = u16::try_from(i).map_err(|_| InnerError::BadFragTotal(frag_total))?;
        let len = u16::try_from(chunk.len()).map_err(|_| InnerError::BadLen {
            len: u16::MAX,
            is_final: false,
        })?;
        let header = InnerHeader {
            kind,
            obj_seq,
            frag_idx,
            frag_total,
            len,
        };
        out.push(header.to_plaintext(chunk)?);
    }
    Ok(out)
}

/// For a [`InnerKind::SealedFirst`] object: the clear `e_pub` (first
/// [`E_PUB_LEN`] bytes) and the sealed payload after it (DESIGN §2.2, §2.4).
/// `None` if the object is too short to hold a key.
pub fn sealed_first_split(object: &[u8]) -> Option<(&[u8; E_PUB_LEN], &[u8])> {
    let (e_pub, sealed) = object.split_at_checked(E_PUB_LEN)?;
    let e_pub: &[u8; E_PUB_LEN] = e_pub.try_into().ok()?;
    Some((e_pub, sealed))
}

/// One object being collected.
#[derive(Debug)]
struct Partial {
    chain: u64,
    obj_seq: u32,
    kind: InnerKind,
    frag_total: u16,
    /// `frag_total` slots; a slot is filled once with that fragment's data.
    frags: Vec<Option<Vec<u8>>>,
    have: usize,
}

impl Partial {
    fn is_complete(&self) -> bool {
        self.have == self.frags.len()
    }

    fn assemble(self) -> Vec<u8> {
        let total: usize = self.frags.iter().flatten().map(Vec::len).sum();
        let mut out = Vec::with_capacity(total);
        for data in self.frags.iter().flatten() {
            out.extend_from_slice(data);
        }
        out
    }
}

/// Collects fragments into objects, keyed by `(chain, obj_seq)`. A completed
/// object is returned once and forgotten; partial objects are bounded by
/// `max_in_progress` with oldest-by-insertion eviction, so a flood of first
/// fragments can only push out other partial objects, never grow memory.
///
/// The reassembler is not the dedupe layer: a fragment of an object that was
/// already completed starts a new partial object, and the msg_id cache in
/// `dtn/dedupe.rs` is what stops a replayed cell from reaching it (BRIEF_v2
/// §5.5 "dedupe cache").
#[derive(Debug)]
pub struct Reassembler {
    max_in_progress: usize,
    /// Oldest first.
    partials: VecDeque<Partial>,
}

impl Default for Reassembler {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_IN_PROGRESS)
    }
}

impl Reassembler {
    /// A reassembler holding at most `max_in_progress` partial objects (at
    /// least 1).
    pub fn new(max_in_progress: usize) -> Self {
        Reassembler {
            max_in_progress: max_in_progress.max(1),
            partials: VecDeque::new(),
        }
    }

    pub fn max_in_progress(&self) -> usize {
        self.max_in_progress
    }

    /// Objects currently being collected.
    pub fn in_progress(&self) -> usize {
        self.partials.len()
    }

    /// Whether `(chain, obj_seq)` is being collected.
    pub fn contains(&self, chain: u64, obj_seq: u32) -> bool {
        self.position(chain, obj_seq).is_some()
    }

    /// Drops every partial object.
    pub fn clear(&mut self) {
        self.partials.clear();
    }

    /// Drops the partial objects of one chain (for a retired recognition
    /// chain); returns how many were dropped.
    pub fn forget_chain(&mut self, chain: u64) -> usize {
        let before = self.partials.len();
        self.partials.retain(|p| p.chain != chain);
        before - self.partials.len()
    }

    /// Adds one decrypted body plaintext recognised under `chain`. Returns
    /// the whole object `(kind, obj_seq, bytes)` when this fragment completes
    /// it, `None` while fragments are still missing or for a duplicate, and
    /// an error for a malformed fragment or one that contradicts the
    /// fragments already held (which drops that object). The key `(chain,
    /// obj_seq)` *is* the object: a one-fragment object arriving while a
    /// multi-fragment partial is open under the same key contradicts its
    /// `frag_total` and is rejected the same way, rather than completing
    /// beside it.
    #[allow(clippy::type_complexity)]
    pub fn push(
        &mut self,
        chain: u64,
        plain: &[u8; BODY_PLAINTEXT_LEN],
    ) -> Result<Option<(InnerKind, u32, Vec<u8>)>, InnerError> {
        let (header, data) = InnerHeader::parse(plain)?;
        let slot = usize::from(header.frag_idx);
        match self.position(chain, header.obj_seq) {
            // A partial always has frag_total >= 2, so a one-fragment object
            // arriving under an open key is a mismatch like any other and
            // never takes the fast path below.
            Some(pos) => {
                let mismatch = self
                    .partials
                    .get(pos)
                    .is_none_or(|p| p.kind != header.kind || p.frag_total != header.frag_total);
                if mismatch {
                    self.partials.remove(pos);
                    return Err(InnerError::Mismatch);
                }
                let Some(partial) = self.partials.get_mut(pos) else {
                    return Err(InnerError::Mismatch);
                };
                match partial.frags.get_mut(slot) {
                    Some(entry @ None) => {
                        *entry = Some(data.to_vec());
                        partial.have += 1;
                    }
                    // duplicate fragment: ignored
                    Some(Some(_)) => return Ok(None),
                    // frag_idx < frag_total was checked by parse(); a miss
                    // here means a stale partial, which is dropped
                    None => {
                        self.partials.remove(pos);
                        return Err(InnerError::Mismatch);
                    }
                }
                if partial.is_complete() {
                    let done = self.partials.remove(pos).map(Partial::assemble);
                    return Ok(done.map(|bytes| (header.kind, header.obj_seq, bytes)));
                }
                Ok(None)
            }
            None => {
                if header.frag_total == 1 {
                    // whole object in one fragment: nothing to hold
                    return Ok(Some((header.kind, header.obj_seq, data.to_vec())));
                }
                while self.partials.len() >= self.max_in_progress {
                    self.partials.pop_front();
                }
                let mut frags: Vec<Option<Vec<u8>>> = Vec::new();
                frags.resize_with(usize::from(header.frag_total), || None);
                if let Some(entry) = frags.get_mut(slot) {
                    *entry = Some(data.to_vec());
                }
                self.partials.push_back(Partial {
                    chain,
                    obj_seq: header.obj_seq,
                    kind: header.kind,
                    frag_total: header.frag_total,
                    frags,
                    have: 1,
                });
                Ok(None)
            }
        }
    }

    fn position(&self, chain: u64, obj_seq: u32) -> Option<usize> {
        self.partials
            .iter()
            .position(|p| p.chain == chain && p.obj_seq == obj_seq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::encode;

    fn object(len: usize) -> Vec<u8> {
        (0..len).map(|i| (i % 251) as u8).collect()
    }

    fn reassemble_in_order(frags: &[[u8; BODY_PLAINTEXT_LEN]]) -> Vec<u8> {
        let mut r = Reassembler::default();
        let mut out = None;
        for f in frags {
            let res = r.push(1, f).expect("valid fragment");
            assert!(out.is_none(), "object completed twice");
            out = res.map(|(_, _, b)| b);
        }
        out.expect("object completed")
    }

    #[test]
    fn constants_match_the_design_table() {
        assert_eq!(INNER_HEADER_LEN, 11);
        assert_eq!(DATA_LEN, 465);
        assert_eq!(MAX_FRAGS, 141);
        assert_eq!(MAX_OBJECT_LEN, 65_565);
        assert_eq!(InnerHeader::LEN, INNER_HEADER_LEN);
    }

    #[test]
    fn header_is_eleven_bytes_little_endian() {
        let h = InnerHeader {
            kind: InnerKind::SealedFirst,
            obj_seq: 0x0102_0304,
            frag_idx: 0x0506,
            frag_total: 0x0708,
            len: 0x090A,
        };
        let bytes = encode(&h);
        assert_eq!(
            bytes,
            [
                0x03, 0x04, 0x03, 0x02, 0x01, 0x06, 0x05, 0x08, 0x07, 0x0A, 0x09
            ]
        );
        assert_eq!(decode::<InnerHeader>(&bytes), Ok(h));
        assert!(decode::<InnerHeader>(&bytes[..10]).is_err());
        let mut bad_kind = bytes;
        bad_kind[0] = 0x07;
        assert_eq!(
            decode::<InnerHeader>(&bad_kind),
            Err(unknown_variant("inner kind", 7u8))
        );
        for k in InnerKind::ALL {
            assert_eq!(InnerKind::from_u8(k.as_u8()), Ok(k));
        }
        assert!(InnerKind::from_u8(0).is_err());
    }

    #[test]
    fn check_enforces_the_fragment_rules() {
        let base = InnerHeader {
            kind: InnerKind::SenderKey,
            obj_seq: 1,
            frag_idx: 0,
            frag_total: 2,
            len: 465,
        };
        assert_eq!(base.check(), Ok(()));
        assert!(!base.is_final());
        let t0 = InnerHeader {
            frag_total: 0,
            ..base
        };
        assert_eq!(t0.check(), Err(InnerError::BadFragTotal(0)));
        let big = InnerHeader {
            frag_total: 142,
            frag_idx: 0,
            ..base
        };
        assert_eq!(big.check(), Err(InnerError::BadFragTotal(142)));
        let idx = InnerHeader {
            frag_idx: 2,
            ..base
        };
        assert_eq!(
            idx.check(),
            Err(InnerError::BadFragIdx { idx: 2, total: 2 })
        );
        let short_mid = InnerHeader { len: 464, ..base };
        assert_eq!(
            short_mid.check(),
            Err(InnerError::BadLen {
                len: 464,
                is_final: false
            })
        );
        let fin = InnerHeader {
            frag_idx: 1,
            len: 1,
            ..base
        };
        assert!(fin.is_final());
        assert_eq!(fin.check(), Ok(()));
        let fin0 = InnerHeader { len: 0, ..fin };
        assert_eq!(
            fin0.check(),
            Err(InnerError::BadLen {
                len: 0,
                is_final: true
            })
        );
        let fin_big = InnerHeader { len: 466, ..fin };
        assert_eq!(
            fin_big.check(),
            Err(InnerError::BadLen {
                len: 466,
                is_final: true
            })
        );
        // 141 fragments is the ceiling
        let max = InnerHeader {
            frag_idx: 140,
            frag_total: 141,
            len: 7,
            ..base
        };
        assert_eq!(max.check(), Ok(()));
    }

    #[test]
    fn fragment_counts_and_padding() {
        for (len, n) in [
            (1, 1),
            (465, 1),
            (466, 2),
            (930, 2),
            (931, 3),
            (10_000, 22),
            (65_565, 141),
        ] {
            let frags = fragment(InnerKind::PairwiseDr, 9, &object(len)).expect("fits");
            assert_eq!(frags.len(), n, "{len} bytes");
            for (i, f) in frags.iter().enumerate() {
                let (h, data) = InnerHeader::parse(f).expect("valid");
                assert_eq!(h.frag_idx as usize, i);
                assert_eq!(h.frag_total as usize, n);
                assert_eq!(h.obj_seq, 9);
                assert_eq!(h.kind, InnerKind::PairwiseDr);
                if i + 1 < n {
                    assert_eq!(data.len(), DATA_LEN);
                }
                assert!(f[INNER_HEADER_LEN + data.len()..].iter().all(|&b| b == 0));
            }
            assert_eq!(reassemble_in_order(&frags), object(len));
        }
        assert_eq!(
            fragment(InnerKind::Sos, 0, &[]),
            Err(InnerError::EmptyObject)
        );
        assert_eq!(
            fragment(InnerKind::Sos, 0, &object(65_566)),
            Err(InnerError::ObjectTooLarge {
                len: 65_566,
                max: 65_565
            })
        );
        // deterministic
        assert_eq!(
            fragment(InnerKind::Sos, 3, &object(1000)),
            fragment(InnerKind::Sos, 3, &object(1000))
        );
    }

    #[test]
    fn parse_rejects_nonzero_padding() {
        let frags = fragment(InnerKind::Receipt, 1, &[7u8; 10]).expect("fits");
        let mut f = frags[0];
        f[INNER_HEADER_LEN + 10] = 1;
        assert_eq!(
            InnerHeader::parse(&f).map(|(h, _)| h),
            Err(InnerError::NonZeroPadding)
        );
        f[INNER_HEADER_LEN + 10] = 0;
        f[BODY_PLAINTEXT_LEN - 1] = 0x80;
        assert_eq!(
            InnerHeader::parse(&f).map(|(h, _)| h),
            Err(InnerError::NonZeroPadding)
        );
    }

    #[test]
    fn reassembler_handles_out_of_order_duplicates_and_mismatch() {
        let obj = object(2000); // 5 fragments
        let frags = fragment(InnerKind::SkdmBundle, 4, &obj).expect("fits");
        let mut r = Reassembler::new(8);
        for i in [3, 1, 1, 0, 4, 3] {
            assert_eq!(r.push(5, &frags[i]), Ok(None), "fragment {i}");
        }
        assert_eq!(r.in_progress(), 1);
        assert!(r.contains(5, 4));
        // the same obj_seq on another chain is another object
        assert_eq!(r.push(6, &frags[2]), Ok(None));
        assert_eq!(r.in_progress(), 2);
        assert_eq!(
            r.push(5, &frags[2]),
            Ok(Some((InnerKind::SkdmBundle, 4, obj.clone())))
        );
        assert_eq!(r.in_progress(), 1);
        assert!(!r.contains(5, 4));
        // completing again starts over (not a dedupe layer)
        assert_eq!(r.push(5, &frags[0]), Ok(None));
        // a mismatching frag_total drops the object
        let other = fragment(InnerKind::SkdmBundle, 4, &object(3000)).expect("fits");
        assert_eq!(r.push(5, &other[1]), Err(InnerError::Mismatch));
        assert!(!r.contains(5, 4));
        // a mismatching kind too
        assert_eq!(r.push(5, &frags[0]), Ok(None));
        let kind = fragment(InnerKind::Receipt, 4, &obj).expect("fits");
        assert_eq!(r.push(5, &kind[1]), Err(InnerError::Mismatch));
        assert!(!r.contains(5, 4));
        assert_eq!(r.forget_chain(6), 1);
        assert_eq!(r.in_progress(), 0);
    }

    #[test]
    fn reassembler_evicts_oldest_when_over_bound() {
        let mut r = Reassembler::new(2);
        assert_eq!(Reassembler::new(0).max_in_progress(), 1);
        let objs: Vec<Vec<[u8; BODY_PLAINTEXT_LEN]>> = (0..3)
            .map(|s| fragment(InnerKind::SenderKey, s, &object(600)).expect("fits"))
            .collect();
        assert_eq!(r.push(1, &objs[0][0]), Ok(None));
        assert_eq!(r.push(1, &objs[1][0]), Ok(None));
        assert_eq!(r.push(1, &objs[2][0]), Ok(None));
        assert_eq!(r.in_progress(), 2);
        assert!(!r.contains(1, 0), "oldest evicted");
        assert!(r.contains(1, 1) && r.contains(1, 2));
        // the evicted object's second fragment just starts it over, evicting obj 1
        assert_eq!(r.push(1, &objs[0][1]), Ok(None));
        assert!(!r.contains(1, 1));
        assert_eq!(
            r.push(1, &objs[2][1]).map(|o| o.map(|(_, s, _)| s)),
            Ok(Some(2))
        );
        r.clear();
        assert_eq!(r.in_progress(), 0);
    }

    /// A one-fragment object is still "the object `(chain, obj_seq)`": if a
    /// multi-fragment partial is open under that key, its `frag_total`
    /// disagrees and the spec's mismatch rule applies to both (spec:
    /// "mismatched frag_total/kind within one object → reject that object").
    #[test]
    fn single_fragment_object_against_an_open_partial_is_a_mismatch() {
        let mut r = Reassembler::new(4);
        let multi = fragment(InnerKind::SenderKey, 7, &object(1000)).expect("fits"); // 3 frags
        let single = fragment(InnerKind::SenderKey, 7, &object(10)).expect("fits"); // 1 frag
        assert_eq!(single.len(), 1);
        assert_eq!(r.push(1, &multi[0]), Ok(None));
        assert_eq!(r.push(1, &single[0]), Err(InnerError::Mismatch));
        assert!(!r.contains(1, 7), "the contradicted partial is dropped");
        assert_eq!(r.in_progress(), 0);
        // with nothing open under the key the same fragment completes at once
        assert_eq!(
            r.push(1, &single[0]),
            Ok(Some((InnerKind::SenderKey, 7, object(10))))
        );
        assert_eq!(r.in_progress(), 0);
        // and the multi-fragment object can start over afterwards
        assert_eq!(r.push(1, &multi[0]), Ok(None));
        assert_eq!(r.push(1, &multi[2]), Ok(None));
        assert_eq!(
            r.push(1, &multi[1]),
            Ok(Some((InnerKind::SenderKey, 7, object(1000))))
        );
        // another chain's partial under the same obj_seq is unaffected
        assert_eq!(r.push(2, &multi[1]), Ok(None));
        assert_eq!(r.push(1, &single[0]).map(|o| o.is_some()), Ok(true));
        assert!(r.contains(2, 7));
    }

    #[test]
    fn sealed_first_split_is_e_pub_then_payload() {
        let obj = object(40);
        let (e_pub, rest) = sealed_first_split(&obj).expect("long enough");
        assert_eq!(&e_pub[..], &obj[..32]);
        assert_eq!(rest, &obj[32..]);
        assert!(sealed_first_split(&obj[..31]).is_none());
        let (e2, r2) = sealed_first_split(&obj[..32]).expect("exactly a key");
        assert_eq!(&e2[..], &obj[..32]);
        assert!(r2.is_empty());
    }

    #[test]
    fn errors_display() {
        assert_eq!(
            InnerError::BadLen {
                len: 3,
                is_final: false
            }
            .to_string(),
            "fragment len 3 is invalid for a non-final fragment"
        );
        assert_eq!(
            InnerError::BadFragTotal(0).to_string(),
            "frag_total 0 is outside 1..=141"
        );
        let e: InnerError = unknown_variant("inner kind", 9u8).into();
        assert!(matches!(e, InnerError::Codec(_)));
    }
}
