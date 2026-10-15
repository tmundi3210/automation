//! SKDM bundle: one sender's new Sender Key, pairwise-encrypted to every
//! member of its roster view (docs/DESIGN.md §2.4 row "SKDM bundle", §1 row
//! `objects/skdm_bundle.rs`; DECISIONS D3 K2 "one bundle of n−1 pairwise
//! ciphertexts", D3 branch B "bundles are padded to 8, 16, 32 or 64
//! recipients"; §3 row (3,4,6,9,10) "each bundle is its own C1 object").
//!
//! Layout: `chain_id u32 ‖ n u8 ‖ (hint [8] ‖ ct_len u16 ‖ ct)×n`, with `n`
//! one of [`PAD_SIZES`] (the smallest that fits the real recipients; the
//! rest are random dummies, applied in both builds per DECISIONS §3 "single
//! wire" row, so the recipient count is hidden from everyone who can open
//! the envelope). Entries are ordered by hint, so real and dummy entries
//! are interleaved and equal bundles are byte-identical.
//!
//! * `hint = Trunc8(SHA256("meshapp.v1.skdm.hint" ‖ recipient.ed ‖
//!   LE32(chain_id)))` ([`hint`]): lets a recipient find its entry without
//!   trial-decrypting the others (a pairwise trial decrypt would cost a
//!   ratchet step), while telling a non-recipient nothing it does not
//!   already know from the roster.
//! * `ct` = the recipient's pairwise encryption ([`PairwiseCt`] encoded) of
//!   a 1:1 post of kind [`kind::POST_SKDM_BUNDLE`] whose body is the
//!   [`SkdmPayload`] `group_id [16] ‖ skdm bytes16 ‖ epoch_seed [32]`: the
//!   backend's SenderKeyDistributionMessage and the recognition seed of the
//!   new chain travel together (DECISIONS §3 row (3,4,6,10) "K_tag comes from
//!   each sender's chain and is delivered in the SKDM"). Framing the entry
//!   as a post means the same pairwise receive path handles an entry
//!   delivered inside a bundle under the sender's old chain tags and one
//!   delivered as a plain 1:1 message when the sender has no old chain yet
//!   (DESIGN §2.4 "a sender with no old chain sends per-recipient pairwise
//!   SKDMs").
//!
//! Deviation from DESIGN §2.4, which hashes `recipient.ed ‖ chain_id` with
//! no version label and carries no chain id: the recipient must know the
//! chain id to compute its hint before it can decrypt anything, so the id
//! travels in the clear at the front of the bundle. It is inside the §5.5
//! envelope (relays never see it) and is cleartext in the SKDM itself (D3
//! Q1), so nothing new is exposed.
//!
//! Nothing here reads a clock.

use rand::CryptoRng;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::codec::{CodecError, Decode, Encode, ListItem, Reader, Writer};
use crate::crypto::backend::{ChainId, GroupId, PeerId};
use crate::crypto::hash::sha256_concat;

/// Length of an entry hint.
pub const HINT_LEN: usize = 8;
/// Domain of the hint hash.
pub const HINT_DOMAIN: &[u8] = b"meshapp.v1.skdm.hint";
/// Allowed entry counts (D3 branch B padding).
pub const PAD_SIZES: [usize; 4] = [8, 16, 32, 64];
/// Largest number of entries: the group cap of 50 (BRIEF §5.4) minus the
/// sender, padded, fits.
pub const MAX_ENTRIES: usize = 64;
/// Length of the recognition seed carried with the SKDM.
pub const SEED_LEN: usize = 32;
/// Ciphertext length of a dummy entry when there is no real entry to copy
/// (assumption; a Normal ratchet message of a ~130-byte payload).
pub const DUMMY_CT_LEN_ASM: usize = 232;

/// Why a bundle could not be built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum SkdmError {
    #[error("too many recipients: {0} (max {MAX_ENTRIES})")]
    TooManyRecipients(usize),
    #[error("two entries share a hint (same recipient twice)")]
    DuplicateRecipient,
}

/// `Trunc8(SHA256("meshapp.v1.skdm.hint" ‖ recipient.ed ‖ LE32(chain_id)))`.
pub fn hint(recipient: &PeerId, chain: ChainId) -> [u8; HINT_LEN] {
    sha256_concat(&[HINT_DOMAIN, recipient.as_bytes(), &chain.0.to_le_bytes()]).truncate8()
}

/// One recipient's slot: `hint [8] ‖ ct bytes16`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkdmEntry {
    pub hint: [u8; HINT_LEN],
    /// A [`PairwiseCt`] encoding, or random bytes for a dummy.
    pub ct: Vec<u8>,
}

impl SkdmEntry {
    /// A real entry for `recipient`.
    pub fn new(recipient: &PeerId, chain: ChainId, ct: Vec<u8>) -> Self {
        SkdmEntry {
            hint: hint(recipient, chain),
            ct,
        }
    }
}

impl Encode for SkdmEntry {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.bytes(&self.hint);
        w.bytes16(&self.ct)
    }
}

impl Decode for SkdmEntry {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(SkdmEntry {
            hint: r.bytes()?,
            ct: r.bytes16()?.to_vec(),
        })
    }
}

impl ListItem for SkdmEntry {}

/// The padded bundle (module docs).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkdmBundle {
    /// The chain the SKDMs inside describe.
    pub chain: ChainId,
    /// Exactly one of [`PAD_SIZES`] entries, ordered by hint.
    pub entries: Vec<SkdmEntry>,
}

impl SkdmBundle {
    /// The padded size for `n` real recipients, `None` above [`MAX_ENTRIES`].
    pub fn padded_len(n: usize) -> Option<usize> {
        PAD_SIZES.iter().copied().find(|&p| p >= n)
    }

    /// Pads `real` entries with random dummies (a random hint and a random
    /// ciphertext as long as the longest real one) to the next size in
    /// [`PAD_SIZES`] and orders them by hint. Deterministic for a given RNG
    /// stream. Two real entries with the same hint are refused: a recipient
    /// can hold only one slot.
    pub fn build(
        chain: ChainId,
        mut real: Vec<SkdmEntry>,
        rng: &mut dyn CryptoRng,
    ) -> Result<Self, SkdmError> {
        let target =
            Self::padded_len(real.len()).ok_or(SkdmError::TooManyRecipients(real.len()))?;
        let dummy_len = real
            .iter()
            .map(|e| e.ct.len())
            .max()
            .unwrap_or(DUMMY_CT_LEN_ASM);
        while real.len() < target {
            let mut h = [0u8; HINT_LEN];
            rng.fill_bytes(&mut h);
            let mut ct = vec![0u8; dummy_len];
            rng.fill_bytes(&mut ct);
            real.push(SkdmEntry { hint: h, ct });
        }
        real.sort_by(|a, b| a.hint.cmp(&b.hint).then_with(|| a.ct.cmp(&b.ct)));
        if real.windows(2).any(|w| w[0].hint == w[1].hint) {
            return Err(SkdmError::DuplicateRecipient);
        }
        Ok(SkdmBundle {
            chain,
            entries: real,
        })
    }

    /// The entry addressed to `my_key` for `chain_id`, by hint.
    pub fn find_mine(&self, my_key: &PeerId, chain_id: ChainId) -> Option<&SkdmEntry> {
        let h = hint(my_key, chain_id);
        self.entries.iter().find(|e| e.hint == h)
    }

    /// [`find_mine`](Self::find_mine) for this bundle's own chain id.
    pub fn find_for(&self, my_key: &PeerId) -> Option<&SkdmEntry> {
        self.find_mine(my_key, self.chain)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn check_shape(entries: &[SkdmEntry]) -> Result<(), CodecError> {
        if !PAD_SIZES.contains(&entries.len()) {
            return Err(CodecError::Invalid("bundle size"));
        }
        if entries.windows(2).any(|w| w[0].hint >= w[1].hint) {
            return Err(CodecError::Invalid("bundle order"));
        }
        Ok(())
    }
}

impl Encode for SkdmBundle {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        Self::check_shape(&self.entries)?;
        self.chain.encode_to(w)?;
        self.entries.encode_to(w)
    }
}

impl Decode for SkdmBundle {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let chain = ChainId::decode_from(r)?;
        let entries: Vec<SkdmEntry> = Vec::decode_from(r)?;
        Self::check_shape(&entries)?;
        Ok(SkdmBundle { chain, entries })
    }
}

/// Plaintext inside one real entry (the body of its
/// [`kind::POST_SKDM_BUNDLE`] post): `group_id [16] ‖ skdm bytes16 ‖
/// epoch_seed [32]`. The SKDM and the seed are wiped on drop.
#[derive(Clone, PartialEq, Eq, Hash, Zeroize, ZeroizeOnDrop)]
pub struct SkdmPayload {
    #[zeroize(skip)]
    pub group: GroupId,
    /// The backend's SenderKeyDistributionMessage bytes.
    pub skdm: Vec<u8>,
    /// Seed of the new chain's recognition epoch
    /// ([`crate::wire::recog::RecogEpoch::from_seed`]).
    pub seed: [u8; SEED_LEN],
}

impl core::fmt::Debug for SkdmPayload {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "SkdmPayload({:?}, skdm {} bytes, <seed redacted>)",
            self.group,
            self.skdm.len()
        )
    }
}

impl Encode for SkdmPayload {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.group.encode_to(w)?;
        w.bytes16(&self.skdm)?;
        w.bytes(&self.seed);
        Ok(())
    }
}

impl Decode for SkdmPayload {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(SkdmPayload {
            group: GroupId::decode_from(r)?,
            skdm: r.bytes16()?.to_vec(),
            seed: r.bytes()?,
        })
    }
}

/// Re-export so callers of this module see the post kind the entries use.
pub use crate::objects::chat_post::kind;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{decode, encode};
    use crate::crypto::ed25519::IdentitySigner;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    fn peer(seed: u8) -> PeerId {
        IdentitySigner::from_seed(&[seed; 32]).public()
    }

    fn rng(seed: u64) -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(seed)
    }

    #[test]
    fn hint_is_the_documented_hash() {
        let p = peer(1);
        let mut bytes = b"meshapp.v1.skdm.hint".to_vec();
        bytes.extend_from_slice(p.as_bytes());
        bytes.extend_from_slice(&0x0102_0304u32.to_le_bytes());
        assert_eq!(
            hint(&p, ChainId(0x0102_0304)),
            crate::crypto::hash::sha256(&bytes).truncate8()
        );
        assert_ne!(hint(&p, ChainId(1)), hint(&p, ChainId(2)));
        assert_ne!(hint(&p, ChainId(1)), hint(&peer(2), ChainId(1)));
    }

    #[test]
    fn build_pads_orders_and_finds() {
        let chain = ChainId(77);
        let real: Vec<SkdmEntry> = (1u8..=3)
            .map(|i| SkdmEntry::new(&peer(i), chain, vec![i; 10 + usize::from(i)]))
            .collect();
        let b = SkdmBundle::build(chain, real, &mut rng(1)).unwrap();
        assert_eq!(b.len(), 8);
        assert!(b.entries.windows(2).all(|w| w[0].hint < w[1].hint));
        for i in 1u8..=3 {
            let e = b.find_mine(&peer(i), chain).expect("found");
            assert_eq!(e.ct, vec![i; 10 + usize::from(i)]);
            assert_eq!(b.find_for(&peer(i)), Some(e));
        }
        assert!(b.find_mine(&peer(4), chain).is_none());
        assert!(b.find_mine(&peer(1), ChainId(78)).is_none());
        // dummies are as long as the longest real entry
        assert!(b.entries.iter().all(|e| e.ct.len() <= 13));
        assert_eq!(b.entries.iter().filter(|e| e.ct.len() == 13).count(), 6);
        // deterministic for the rng stream, different for another
        let real2: Vec<SkdmEntry> = (1u8..=3)
            .map(|i| SkdmEntry::new(&peer(i), chain, vec![i; 10 + usize::from(i)]))
            .collect();
        assert_eq!(
            SkdmBundle::build(chain, real2.clone(), &mut rng(1)).unwrap(),
            b
        );
        assert_ne!(SkdmBundle::build(chain, real2, &mut rng(2)).unwrap(), b);
        // sizes
        for (n, want) in [
            (0, 8),
            (8, 8),
            (9, 16),
            (16, 16),
            (17, 32),
            (33, 64),
            (64, 64),
        ] {
            assert_eq!(SkdmBundle::padded_len(n), Some(want), "{n}");
        }
        assert_eq!(SkdmBundle::padded_len(65), None);
        let too_many: Vec<SkdmEntry> = (0..65u16)
            .map(|i| SkdmEntry {
                hint: [i as u8, (i >> 8) as u8, 0, 0, 0, 0, 0, 0],
                ct: vec![],
            })
            .collect();
        assert_eq!(
            SkdmBundle::build(chain, too_many, &mut rng(1)),
            Err(SkdmError::TooManyRecipients(65))
        );
        let dup = vec![
            SkdmEntry::new(&peer(1), chain, vec![1]),
            SkdmEntry::new(&peer(1), chain, vec![2]),
        ];
        assert_eq!(
            SkdmBundle::build(chain, dup, &mut rng(1)),
            Err(SkdmError::DuplicateRecipient)
        );
        let empty = SkdmBundle::build(chain, vec![], &mut rng(3)).unwrap();
        assert_eq!(empty.len(), 8);
        assert!(empty.entries.iter().all(|e| e.ct.len() == DUMMY_CT_LEN_ASM));
    }

    #[test]
    fn bundle_layout_roundtrip_and_shape_checks() {
        let chain = ChainId(5);
        let b = SkdmBundle::build(
            chain,
            vec![SkdmEntry::new(&peer(1), chain, vec![9, 9])],
            &mut rng(4),
        )
        .unwrap();
        let bytes = encode(&b);
        assert_eq!(&bytes[..4], &[5, 0, 0, 0]);
        assert_eq!(bytes[4], 8);
        assert_eq!(decode::<SkdmBundle>(&bytes), Ok(b.clone()));
        // a 7-entry bundle is not a valid shape
        let mut seven = b.clone();
        seven.entries.truncate(7);
        assert!(seven.try_encode().is_err());
        let mut w = Writer::new();
        w.u32(5);
        seven.entries.encode_to(&mut w).unwrap();
        assert_eq!(
            decode::<SkdmBundle>(w.as_bytes()),
            Err(CodecError::Invalid("bundle size"))
        );
        // out of order
        let mut swapped = b.clone();
        swapped.entries.swap(0, 1);
        let mut w = Writer::new();
        w.u32(5);
        swapped.entries.encode_to(&mut w).unwrap();
        assert_eq!(
            decode::<SkdmBundle>(w.as_bytes()),
            Err(CodecError::Invalid("bundle order"))
        );
        let mut longer = bytes.clone();
        longer.push(0);
        assert_eq!(
            decode::<SkdmBundle>(&longer),
            Err(CodecError::TrailingBytes(1))
        );
    }

    #[test]
    fn payload_is_zeroized_on_drop() {
        fn assert_zeroize_on_drop<T: zeroize::ZeroizeOnDrop>() {}
        assert_zeroize_on_drop::<SkdmPayload>();
    }

    #[test]
    fn payload_roundtrip_and_redaction() {
        let p = SkdmPayload {
            group: GroupId([3; 16]),
            skdm: vec![1, 2, 3],
            seed: [7; 32],
        };
        let bytes = encode(&p);
        assert_eq!(bytes.len(), 16 + 2 + 3 + 32);
        assert_eq!(&bytes[16..18], &[3, 0]);
        assert_eq!(decode::<SkdmPayload>(&bytes), Ok(p.clone()));
        assert!(!format!("{p:?}").contains("07070707"));
        assert_eq!(kind::POST_SKDM_BUNDLE, 0x36);
    }

    proptest! {
        #[test]
        fn decoders_survive_random_bytes(bytes in prop::collection::vec(any::<u8>(), 0..600)) {
            if let Ok(b) = decode::<SkdmBundle>(&bytes) {
                prop_assert_eq!(encode(&b), bytes.clone());
            }
            if let Ok(p) = decode::<SkdmPayload>(&bytes) {
                prop_assert_eq!(encode(&p), bytes.clone());
            }
            if let Ok(e) = decode::<SkdmEntry>(&bytes) {
                prop_assert_eq!(encode(&e), bytes.clone());
            }
        }

        #[test]
        fn any_recipient_count_up_to_64_builds_and_roundtrips(n in 0usize..=64, seed in any::<u64>()) {
            let chain = ChainId(9);
            let real: Vec<SkdmEntry> = (0..n).map(|i| {
                let mut s = [0u8; 32];
                s[0] = i as u8;
                s[1] = 1;
                SkdmEntry::new(&IdentitySigner::from_seed(&s).public(), chain, vec![0xaa; 20])
            }).collect();
            let b = SkdmBundle::build(chain, real, &mut rng(seed)).unwrap();
            prop_assert_eq!(b.len(), SkdmBundle::padded_len(n).unwrap());
            prop_assert_eq!(decode::<SkdmBundle>(&encode(&b)), Ok(b));
        }
    }
}
