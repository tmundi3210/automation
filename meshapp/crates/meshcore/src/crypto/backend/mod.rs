//! Pluggable crypto backends (docs/DESIGN.md §1 "Backend trait").
//!
//! The protocol core reaches the Signal-style primitives, PQXDH session start,
//! Double Ratchet pairwise messages and Sender Keys for groups (BRIEF_v2 §5.2
//! items 1–3; DECISIONS D3 K1–K2; D6 rules 1–7), only through
//! [`PairwiseBackend`] and [`GroupBackend`]. [`libsignal::LibsignalBackend`]
//! (feature `backend-libsignal`, the default) implements them over the pinned
//! `libsignal-protocol` crate; [`vodozemac::VodozemacBackend`] (feature
//! `backend-vodozemac`) is a compile-only stub until its skipped-key cap is
//! forked from 40 to 2000 (D6 rule 1). Everything above the traits (SKDM
//! bundles, rotation triggers, crossed initiation, header encryption) is
//! composition in `groups/` and `pairwise/` (DESIGN §1).
//!
//! Rules every backend follows:
//!
//! - **All state lives in the caller's transaction.** Sessions, prekeys and
//!   sender-key records are read from and written to the [`KvTxn`] namespaces
//!   in [`crate::storage::kv::ns`] on every call; the backend object caches
//!   nothing. "Decrypt + advance ratchet + store plaintext + delete message
//!   key" therefore commit together or not at all (BRIEF §5.3 "State is
//!   committed atomically before Delivered", D6 rule 4); a caller that drops
//!   its transaction loses the ratchet step and can decrypt the same
//!   ciphertext again (DESIGN §6 "Atomic commit": a crash between decrypt and
//!   commit replays the cell).
//! - **No wall clock.** Time arrives as `now_mono`, powered-on milliseconds
//!   from [`crate::clock::Clock`]; no backend calls `SystemTime::now` (BRIEF §4
//!   "no timestamps decide anything", D6 rule 6 "no wall-clock expiry on
//!   established sessions").
//! - **TOFU is off.** A peer's X25519 key must be pinned from its roster
//!   [`crate::crypto::keycard::KeyCard`] with [`PairwiseBackend::pin_peer_x25519`]
//!   before any bundle, session or message from it is accepted, and the key in
//!   use must still match the pin on every operation (BRIEF §5.2 "a Sender Key
//!   is accepted only over a pairwise session whose key matches the sender's
//!   roster entry", D3 K1 atomic identity binding).
//! - **The backend holds X25519 and Kyber keys only.** The long-term identity
//!   is the Ed25519 [`crate::crypto::ed25519::IdentityKey`] ([`PeerId`]), and
//!   the KeyCard binds the backend's X25519 identity key to it (DESIGN §1
//!   "`PeerId` = Ed25519 identity. The backend holds X25519 keys only; the
//!   KeyCard binds them").
//! - **Determinism.** Given the same transaction contents, inputs and RNG
//!   stream a backend produces the same bytes and the same decisions.

use core::fmt;

use rand::CryptoRng;

use crate::codec::{CodecError, Decode, Encode, ListItem, Reader, Writer, unknown_variant};
use crate::crypto::ed25519::IdentityKey;
use crate::storage::kv::KvTxn;

#[cfg(feature = "backend-libsignal")]
pub mod libsignal;
#[cfg(feature = "backend-vodozemac")]
pub mod vodozemac;

/// A peer is named by its Ed25519 identity key (DESIGN §1 "`PeerId` = Ed25519
/// identity"). Backends that need a textual address use its lowercase hex
/// (the [`fmt::Display`] of [`IdentityKey`]) with device id 1.
pub type PeerId = IdentityKey;

/// A group's 16-byte id (DESIGN §2.4 `group_id [16]`). Backends that key
/// sender-key records by a "distribution id" (libsignal: a UUID) use these
/// bytes verbatim, so the id is the same on every member.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GroupId(pub [u8; GroupId::LEN]);

impl GroupId {
    /// Byte length of a group id.
    pub const LEN: usize = 16;

    pub const fn as_bytes(&self) -> &[u8; GroupId::LEN] {
        &self.0
    }
}

impl fmt::Debug for GroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GroupId(")?;
        for b in &self.0 {
            write!(f, "{b:02x}")?;
        }
        f.write_str(")")
    }
}

impl Encode for GroupId {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.bytes(&self.0);
        Ok(())
    }
}

impl Decode for GroupId {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(GroupId(r.bytes()?))
    }
}

impl ListItem for GroupId {}

/// Identifies one sender chain inside a group: every call to
/// [`GroupBackend::new_sender_chain`] (= a rotation, D3 K2) yields a new one
/// and receivers keep the previous chains so late posts still decrypt
/// (BRIEF §5.4 "Receivers keep every sender's previous chains ... until the
/// library's built-in caps are reached").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChainId(pub u32);

impl Encode for ChainId {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u32(self.0);
        Ok(())
    }
}

impl Decode for ChainId {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(ChainId(r.u32()?))
    }
}

impl ListItem for ChainId {}

/// Which kind of pairwise ciphertext a [`PairwiseCt`] carries (DESIGN §1
/// `PairwiseCt {kind: PreKey|Normal, bytes}`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PairwiseKind {
    /// A first-flight message that also carries the PQXDH handshake (the
    /// sender processed the peer's prekey bundle and has not yet heard back).
    /// BRIEF §5.3: "Strangers use signed prekey bundles ... First messages are
    /// sealed" — the sealing is the caller's envelope, not this object.
    PreKey,
    /// An ordinary Double Ratchet message on an established session.
    Normal,
}

impl PairwiseKind {
    /// Wire discriminant (`0x01` PreKey, `0x02` Normal).
    pub const fn to_u8(self) -> u8 {
        match self {
            PairwiseKind::PreKey => 0x01,
            PairwiseKind::Normal => 0x02,
        }
    }

    /// Inverse of [`PairwiseKind::to_u8`]; unknown values are rejected
    /// (DESIGN §2.1 "decoders reject ... unknown enum values").
    pub fn from_u8(v: u8) -> Result<Self, CodecError> {
        match v {
            0x01 => Ok(PairwiseKind::PreKey),
            0x02 => Ok(PairwiseKind::Normal),
            other => Err(unknown_variant("pairwise kind", other)),
        }
    }
}

/// A pairwise (1:1) ciphertext: the backend's serialized message plus the
/// kind the receiver needs to parse it. Layout: `kind u8 ‖ bytes bytes16`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairwiseCt {
    pub kind: PairwiseKind,
    pub bytes: Vec<u8>,
}

impl Encode for PairwiseCt {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.kind.to_u8());
        w.bytes16(&self.bytes)
    }
}

impl Decode for PairwiseCt {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(PairwiseCt {
            kind: PairwiseKind::from_u8(r.u8()?)?,
            bytes: r.bytes16()?.to_vec(),
        })
    }
}

impl ListItem for PairwiseCt {}

/// Why a backend call failed. Decryption failures are deliberately opaque
/// (BRIEF §5.9 / D6 rule 7: the UI never says "Failed"; the caller shows
/// "Needs their key" or a counter gap instead).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BackendError {
    /// No pairwise session with this peer exists in the transaction (D6 rule 7
    /// "'Needs their key': no bundle yet").
    #[error("no session with this peer")]
    NoSession,
    /// The peer's X25519 key is not pinned, or the key in the bundle/session
    /// differs from the pinned roster key (BRIEF §5.2, D3 K1). Nothing was
    /// stored.
    #[error("peer key is not the pinned roster key")]
    UntrustedIdentity,
    /// No sender-key chain for this (group, sender, chain id): the SKDM has
    /// not arrived yet or the chain was evicted by the library's state cap
    /// (BRIEF §5.4; D3 K2 "5 chain states per sender").
    #[error("no sender key for this group and sender")]
    NoSenderKey,
    /// The ciphertext did not decrypt (bad MAC/signature, duplicate counter,
    /// wrong chain, missing prekey). Intentionally carries no detail.
    #[error("decryption failed")]
    Decrypt,
    /// The bytes are not a well-formed bundle, SKDM or message.
    #[error("malformed backend object")]
    Encoding,
    /// This backend does not implement the operation (the vodozemac stub).
    #[error("not implemented by this backend")]
    Unimplemented,
    /// A broken invariant inside the backend or its storage (corrupt record,
    /// missing identity, library error that should not happen). Never caused
    /// by peer input alone.
    #[error("backend internal error: {0}")]
    Internal(String),
}

impl From<CodecError> for BackendError {
    fn from(_: CodecError) -> Self {
        BackendError::Encoding
    }
}

/// Pairwise PQXDH + Double Ratchet sessions (BRIEF §5.2 item 2 "1:1: pairwise
/// PQXDH/X3DH + Double Ratchet"; D6). Object-safe; the RNG is injected so tests
/// and the simulator are reproducible.
pub trait PairwiseBackend {
    /// Generates and stores (namespace [`crate::storage::kv::ns::IDENTITY`])
    /// this node's X25519 identity key pair and returns its 32-byte public key
    /// for the [`crate::crypto::keycard::KeyCard`]. Idempotent: returns the
    /// stored key if one exists.
    fn init_identity(
        &self,
        txn: &mut dyn KvTxn,
        rng: &mut dyn CryptoRng,
    ) -> Result<[u8; 32], BackendError>;

    /// The stored X25519 identity public key, if [`Self::init_identity`] ran.
    fn local_x25519(&self, txn: &dyn KvTxn) -> Option<[u8; 32]>;

    /// Pins a peer's X25519 key from its roster KeyCard. TOFU is OFF: bundles,
    /// sessions and messages from a peer are refused with
    /// [`BackendError::UntrustedIdentity`] unless its key was pinned here and
    /// matches (BRIEF §5.2 "accepted only over a pairwise session whose key
    /// matches the sender's roster entry"; D3 K1). Re-pinning a different key
    /// is the roster's decision (D5 same-name marker); it makes an existing
    /// session with the old key unusable, which is the intended effect.
    fn pin_peer_x25519(
        &self,
        txn: &mut dyn KvTxn,
        peer: &PeerId,
        x25519: &[u8; 32],
    ) -> Result<(), BackendError>;

    /// The X25519 identity key the current session with `peer` is bound to,
    /// used for the SKDM atomic check against the roster KeyCard (DESIGN §1
    /// "X25519 used for the SKDM atomic check"; D3 K1).
    fn remote_session_key(&self, txn: &dyn KvTxn, peer: &PeerId) -> Option<[u8; 32]>;

    /// Builds and stores a prekey bundle: the current signed prekey, a fresh
    /// Kyber prekey and `one_time` fresh one-time prekeys, serialized as
    /// [`PreKeyBundleWire`]. The signed prekey is rotated only by
    /// [`Self::rotate_signed_prekey`]; the caller decides when (every 24 h of
    /// powered-on time, BRIEF §5.3, D6 rule 3). D6 rule 2: hand one-time
    /// prekeys only to a specific peer on a direct encounter, never gossip
    /// them.
    fn make_prekey_bundle(
        &self,
        txn: &mut dyn KvTxn,
        one_time: u8,
        rng: &mut dyn CryptoRng,
    ) -> Result<Vec<u8>, BackendError>;

    /// Creates a new signed prekey, stamped `now_mono`, and makes it current
    /// for future bundles. Earlier signed prekeys stay stored so sessions
    /// started from older bundles still complete; deleting them is the
    /// caller's powered-on-time rule (BRIEF §5.3 "deleted by a powered-on-time
    /// rule"; D6 rule 3).
    fn rotate_signed_prekey(
        &self,
        txn: &mut dyn KvTxn,
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<(), BackendError>;

    /// Starts a session with `peer` from its bundle (PQXDH initiator side).
    /// Fails with [`BackendError::UntrustedIdentity`] unless the bundle's
    /// identity key equals the pinned one, and with [`BackendError::Encoding`]
    /// if the bundle or its signatures are malformed. `now_mono` stamps the
    /// unacknowledged session (see the backend's notes on how the library
    /// interprets it).
    fn process_prekey_bundle(
        &self,
        txn: &mut dyn KvTxn,
        peer: &PeerId,
        bundle: &[u8],
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<(), BackendError>;

    /// Encrypts `plain` to `peer`, advancing the sending chain in `txn`. The
    /// result is [`PairwiseKind::PreKey`] until the peer's first reply was
    /// decrypted, then [`PairwiseKind::Normal`].
    fn encrypt(
        &self,
        txn: &mut dyn KvTxn,
        peer: &PeerId,
        plain: &[u8],
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<PairwiseCt, BackendError>;

    /// Decrypts `ct` from `peer`, consuming the message key in `txn` (D6 rule 4
    /// "message keys are deleted on decrypt"). Out-of-order messages use the
    /// library's skipped-key store (2000 per chain, D6 rule 1).
    fn decrypt(
        &self,
        txn: &mut dyn KvTxn,
        peer: &PeerId,
        ct: &PairwiseCt,
        rng: &mut dyn CryptoRng,
    ) -> Result<Vec<u8>, BackendError>;

    /// Whether a session with `peer` exists and can send: a sending chain is
    /// present and the session's remote key is still the pinned one (after a
    /// re-pin, [`Self::encrypt`] refuses the session, so this is `false` and
    /// the pairwise layer re-runs the bundle). Reads no clock.
    fn has_session(&self, txn: &dyn KvTxn, peer: &PeerId) -> bool;
}

/// Sender Keys over the pairwise sessions (BRIEF §5.2 item 2 "Groups: Sender
/// Keys over those pairwise sessions"; D3 K1–K2). The SKDM bytes returned by
/// [`GroupBackend::new_sender_chain`] carry no identity signature; the caller
/// sends them inside a pairwise session and the receiver accepts them only
/// after the atomic KeyCard check (D3 K1; DESIGN §1 `GroupState::accept_skdm`).
pub trait GroupBackend {
    /// Creates a fresh sender chain for this node in `group`, deleting any
    /// existing own record first (= rotation, D3 K2 "called again =
    /// rotation"), and returns its chain id and the SenderKeyDistributionMessage
    /// bytes to distribute.
    fn new_sender_chain(
        &self,
        txn: &mut dyn KvTxn,
        group: &GroupId,
        rng: &mut dyn CryptoRng,
    ) -> Result<(ChainId, Vec<u8>), BackendError>;

    /// Stores `sender`'s chain from its SKDM and returns the chain id. Keeps the
    /// sender's previous chains up to the library's cap (BRIEF §5.4).
    fn process_skdm(
        &self,
        txn: &mut dyn KvTxn,
        group: &GroupId,
        sender: &PeerId,
        skdm: &[u8],
    ) -> Result<ChainId, BackendError>;

    /// Encrypts a group post under this node's current chain: a
    /// SenderKeyMessage including the per-chain signature (BRIEF §5.1 "Group
    /// posts are signed with a per-chain signing key").
    fn encrypt(
        &self,
        txn: &mut dyn KvTxn,
        group: &GroupId,
        plain: &[u8],
        rng: &mut dyn CryptoRng,
    ) -> Result<Vec<u8>, BackendError>;

    /// Decrypts a SenderKeyMessage from `sender`, returning the chain it was
    /// sent under and the plaintext. [`BackendError::NoSenderKey`] means the
    /// SKDM for that chain has not arrived or was evicted ("Waiting · can't
    /// read yet", DESIGN §1 rotation paragraph).
    fn decrypt(
        &self,
        txn: &mut dyn KvTxn,
        group: &GroupId,
        sender: &PeerId,
        ct: &[u8],
    ) -> Result<(ChainId, Vec<u8>), BackendError>;

    /// This node's current chain in `group`, if [`Self::new_sender_chain`] ran.
    fn own_chain(&self, txn: &dyn KvTxn, group: &GroupId) -> Option<ChainId>;
}

/// Both halves of a backend behind one object.
pub trait Backend: PairwiseBackend + GroupBackend {}

impl<T: PairwiseBackend + GroupBackend> Backend for T {}

/// The backend selected by the crate features: libsignal when
/// `backend-libsignal` is on (the default), otherwise the vodozemac stub
/// (docs/decisions-assumed.md row 2).
pub fn default_backend() -> Box<dyn Backend> {
    #[cfg(feature = "backend-libsignal")]
    {
        Box::new(libsignal::LibsignalBackend::new())
    }
    #[cfg(all(not(feature = "backend-libsignal"), feature = "backend-vodozemac"))]
    {
        Box::new(vodozemac::VodozemacBackend::new())
    }
    #[cfg(not(any(feature = "backend-libsignal", feature = "backend-vodozemac")))]
    {
        compile_error!("meshcore needs feature backend-libsignal or backend-vodozemac");
    }
}

// ---- prekey bundle wire format ---------------------------------------------

/// Version byte of [`PreKeyBundleWire`]; decoders reject any other value
/// (DESIGN §2.1 "every object needs a `ver` byte and a reject-unknown-version
/// rule").
pub const BUNDLE_VERSION: u8 = 1;

/// Length of a serialized Curve25519 public key as the backend emits it:
/// a `0x05` type byte followed by the 32-byte key.
pub const EC_PUBLIC_KEY_LEN: usize = 33;

/// One one-time prekey inside a [`PreKeyBundleWire`]: `id u32 ‖ key [33]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OneTimePreKey {
    pub id: u32,
    pub key: [u8; EC_PUBLIC_KEY_LEN],
}

impl Encode for OneTimePreKey {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u32(self.id).bytes(&self.key);
        Ok(())
    }
}

impl Decode for OneTimePreKey {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(OneTimePreKey {
            id: r.u32()?,
            key: r.bytes()?,
        })
    }
}

impl ListItem for OneTimePreKey {}

/// Canonical encoding of a prekey bundle (BRIEF §5.3 "signed prekey bundles";
/// D6 rule 2), in the §2.1 codec so equal bundles are byte-identical:
///
/// ```text
/// ver u8 (=1)
/// registration_id u32
/// one_time list_u8 of (id u32 ‖ key [33])      -- 0..=255 one-time prekeys
/// signed_prekey_id u32 ‖ signed_prekey [33] ‖ signed_prekey_sig bytes16
/// kyber_prekey_id u32 ‖ kyber_prekey bytes16 ‖ kyber_prekey_sig bytes16
/// identity_key [33]                              -- X25519 identity, bound by the KeyCard
/// ```
///
/// Signatures are the backend's XEdDSA signatures by `identity_key` over the
/// serialized prekey; the receiver verifies them before starting a session.
/// Only the first one-time prekey is used by
/// [`PairwiseBackend::process_prekey_bundle`]; the others are reserve keys for
/// a later re-establishment (D6 rule 2 "hand over 10 reserve OPKs").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreKeyBundleWire {
    pub registration_id: u32,
    pub one_time: Vec<OneTimePreKey>,
    pub signed_prekey_id: u32,
    pub signed_prekey: [u8; EC_PUBLIC_KEY_LEN],
    pub signed_prekey_sig: Vec<u8>,
    pub kyber_prekey_id: u32,
    pub kyber_prekey: Vec<u8>,
    pub kyber_prekey_sig: Vec<u8>,
    pub identity_key: [u8; EC_PUBLIC_KEY_LEN],
}

impl Encode for PreKeyBundleWire {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(BUNDLE_VERSION).u32(self.registration_id);
        self.one_time.encode_to(w)?;
        w.u32(self.signed_prekey_id).bytes(&self.signed_prekey);
        w.bytes16(&self.signed_prekey_sig)?;
        w.u32(self.kyber_prekey_id);
        w.bytes16(&self.kyber_prekey)?;
        w.bytes16(&self.kyber_prekey_sig)?;
        w.bytes(&self.identity_key);
        Ok(())
    }
}

impl Decode for PreKeyBundleWire {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let ver = r.u8()?;
        if ver != BUNDLE_VERSION {
            return Err(unknown_variant("prekey bundle ver", ver));
        }
        Ok(PreKeyBundleWire {
            registration_id: r.u32()?,
            one_time: Vec::decode_from(r)?,
            signed_prekey_id: r.u32()?,
            signed_prekey: r.bytes()?,
            signed_prekey_sig: r.bytes16()?.to_vec(),
            kyber_prekey_id: r.u32()?,
            kyber_prekey: r.bytes16()?.to_vec(),
            kyber_prekey_sig: r.bytes16()?.to_vec(),
            identity_key: r.bytes()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{decode, encode};
    use proptest::prelude::*;

    fn sample_bundle() -> PreKeyBundleWire {
        PreKeyBundleWire {
            registration_id: 0x0102_0304,
            one_time: vec![
                OneTimePreKey {
                    id: 7,
                    key: [0x05; 33],
                },
                OneTimePreKey {
                    id: 8,
                    key: [0x06; 33],
                },
            ],
            signed_prekey_id: 3,
            signed_prekey: [0x07; 33],
            signed_prekey_sig: vec![0xaa; 64],
            kyber_prekey_id: 9,
            kyber_prekey: vec![0xbb; 1569],
            kyber_prekey_sig: vec![0xcc; 64],
            identity_key: [0x05; 33],
        }
    }

    #[test]
    fn bundle_known_layout_and_roundtrip() {
        let b = sample_bundle();
        let bytes = encode(&b);
        let mut expect = vec![1u8, 4, 3, 2, 1];
        expect.push(2);
        expect.extend_from_slice(&7u32.to_le_bytes());
        expect.extend_from_slice(&[0x05; 33]);
        expect.extend_from_slice(&8u32.to_le_bytes());
        expect.extend_from_slice(&[0x06; 33]);
        expect.extend_from_slice(&3u32.to_le_bytes());
        expect.extend_from_slice(&[0x07; 33]);
        expect.extend_from_slice(&[64, 0]);
        expect.extend_from_slice(&[0xaa; 64]);
        expect.extend_from_slice(&9u32.to_le_bytes());
        expect.extend_from_slice(&1569u16.to_le_bytes());
        expect.extend_from_slice(&[0xbb; 1569]);
        expect.extend_from_slice(&[64, 0]);
        expect.extend_from_slice(&[0xcc; 64]);
        expect.extend_from_slice(&[0x05; 33]);
        assert_eq!(bytes, expect);
        assert_eq!(decode::<PreKeyBundleWire>(&bytes), Ok(b.clone()));
        assert_eq!(encode(&b), encode(&b.clone()), "deterministic");
    }

    #[test]
    fn bundle_rejects_version_prefixes_and_trailing() {
        let good = encode(&sample_bundle());
        let mut bad = good.clone();
        bad[0] = 2;
        assert_eq!(
            decode::<PreKeyBundleWire>(&bad),
            Err(unknown_variant("prekey bundle ver", 2u8))
        );
        for cut in 0..good.len() {
            assert!(
                matches!(
                    decode::<PreKeyBundleWire>(&good[..cut]),
                    Err(CodecError::UnexpectedEof { .. })
                ),
                "prefix {cut}"
            );
        }
        let mut longer = good.clone();
        longer.push(0);
        assert_eq!(
            decode::<PreKeyBundleWire>(&longer),
            Err(CodecError::TrailingBytes(1))
        );
        // no one-time prekeys is a valid bundle
        let mut none = sample_bundle();
        none.one_time.clear();
        assert_eq!(decode::<PreKeyBundleWire>(&encode(&none)), Ok(none));
    }

    #[test]
    fn pairwise_ct_and_kind_roundtrip() {
        for (kind, byte) in [(PairwiseKind::PreKey, 1u8), (PairwiseKind::Normal, 2u8)] {
            assert_eq!(kind.to_u8(), byte);
            assert_eq!(PairwiseKind::from_u8(byte), Ok(kind));
            let ct = PairwiseCt {
                kind,
                bytes: vec![1, 2, 3],
            };
            let bytes = encode(&ct);
            assert_eq!(bytes, [byte, 3, 0, 1, 2, 3]);
            assert_eq!(decode::<PairwiseCt>(&bytes), Ok(ct));
        }
        assert_eq!(
            PairwiseKind::from_u8(0),
            Err(unknown_variant("pairwise kind", 0u8))
        );
        assert_eq!(
            decode::<PairwiseCt>(&[9, 0, 0]),
            Err(unknown_variant("pairwise kind", 9u8))
        );
    }

    #[test]
    fn group_and_chain_ids_encode_fixed_width() {
        let g = GroupId([0xab; 16]);
        assert_eq!(encode(&g), vec![0xab; 16]);
        assert_eq!(decode::<GroupId>(&[0xab; 16]), Ok(g));
        assert_eq!(format!("{g:?}"), format!("GroupId({})", "ab".repeat(16)));
        assert_eq!(encode(&ChainId(0x0102_0304)), [4, 3, 2, 1]);
        assert_eq!(decode::<ChainId>(&[4, 3, 2, 1]), Ok(ChainId(0x0102_0304)));
        assert_eq!(
            encode(&vec![ChainId(1), ChainId(2)]),
            [2, 1, 0, 0, 0, 2, 0, 0, 0]
        );
    }

    #[test]
    fn errors_display_and_convert() {
        let e: BackendError = CodecError::TrailingBytes(1).into();
        assert_eq!(e, BackendError::Encoding);
        assert_eq!(BackendError::Decrypt.to_string(), "decryption failed");
        assert_eq!(
            BackendError::Internal("x".into()).to_string(),
            "backend internal error: x"
        );
        let _: &dyn std::error::Error = &BackendError::NoSession;
    }

    proptest! {
        #[test]
        fn bundle_decoder_survives_random_bytes(bytes in proptest::collection::vec(any::<u8>(), 0..4096)) {
            if let Ok(b) = decode::<PreKeyBundleWire>(&bytes) {
                prop_assert_eq!(encode(&b), bytes, "accepted input re-encodes identically");
            }
        }

        #[test]
        fn pairwise_ct_decoder_survives_random_bytes(bytes in proptest::collection::vec(any::<u8>(), 0..512)) {
            if let Ok(ct) = decode::<PairwiseCt>(&bytes) {
                prop_assert_eq!(encode(&ct), bytes);
            }
        }
    }
}
