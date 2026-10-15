//! Alternative backend over `vodozemac` (feature `backend-vodozemac`; DESIGN §1
//! row `crypto/backend/vodozemac.rs`; docs/decisions-assumed.md row 2;
//! docs/testing.md "Backend stub"). Compile-only: every method returns
//! [`BackendError::Unimplemented`], and CI builds it so the Apache-2.0 path
//! never rots (DESIGN §11 risk 5).
//!
//! Why it is not usable yet (D6 rule 1, verified against vodozemac 0.11.1):
//!
//! - Its Olm receiver chain keeps at most `MAX_MESSAGE_KEYS = 40` skipped
//!   message keys (`src/olm/session/receiver_chain.rs`, `pub(crate)`, not
//!   configurable). D6 rule 1: "at 40 a merged backlog of more than 40
//!   reordered messages in one chain permanently loses the early ones
//!   (criterion 1)"; the brief's requirement is 2000 ([`REQUIRED_MAX_MESSAGE_KEYS`]),
//!   which needs a fork of the crate.
//! - Crossed initiation (D6 rule 5) needs an app-side session store that
//!   keeps every Olm session per contact and tries each on decrypt, with the
//!   deterministic sending-session rule ("lower hash of the initiator's base
//!   key"); vodozemac has no session selection of its own.
//! - Group chains would be Megolm inbound sessions, one per (sender, chain),
//!   retained by the caller: Megolm has no equivalent of libsignal's
//!   5-state sender record, so the retention rule (BRIEF §5.4) moves into
//!   `groups/`.
//! - Olm has no post-quantum prekey; the bundle format in
//!   [`super::PreKeyBundleWire`] would carry an empty Kyber field.
//!
//! When the fork lands, this file implements the same two traits over
//! `vodozemac::olm::{Account, Session}` and `vodozemac::megolm::{GroupSession,
//! InboundGroupSession}` with the pickled state in the [`KvTxn`] namespaces.

use rand::CryptoRng;

use super::{BackendError, ChainId, GroupBackend, GroupId, PairwiseBackend, PairwiseCt, PeerId};
use crate::storage::kv::KvTxn;

/// Skipped-key cap of the pinned vodozemac release (crate-private there).
pub const VODOZEMAC_MAX_MESSAGE_KEYS: usize = 40;

/// Skipped-key cap the product needs per receiver chain (D6 rule 1; libsignal
/// has exactly this).
pub const REQUIRED_MAX_MESSAGE_KEYS: usize = 2000;

/// The vodozemac backend stub. Holds no state; all methods are unimplemented.
#[derive(Debug, Default, Clone, Copy)]
pub struct VodozemacBackend;

impl VodozemacBackend {
    pub const fn new() -> Self {
        Self
    }

    /// Type name of the Olm account this backend will wrap once the fork
    /// exists. Referencing `vodozemac::olm::Account` keeps the dependency
    /// really linked so API drift in the crate fails this build, not a later
    /// one.
    pub fn library_account_type() -> &'static str {
        std::any::type_name::<vodozemac::olm::Account>()
    }

    /// Whether the linked vodozemac could satisfy D6 rule 1 unforked. Always
    /// false for the pinned release.
    pub const fn skipped_key_cap_sufficient() -> bool {
        VODOZEMAC_MAX_MESSAGE_KEYS >= REQUIRED_MAX_MESSAGE_KEYS
    }
}

impl PairwiseBackend for VodozemacBackend {
    fn init_identity(
        &self,
        _txn: &mut dyn KvTxn,
        _rng: &mut dyn CryptoRng,
    ) -> Result<[u8; 32], BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn local_x25519(&self, _txn: &dyn KvTxn) -> Option<[u8; 32]> {
        None
    }

    fn pin_peer_x25519(
        &self,
        _txn: &mut dyn KvTxn,
        _peer: &PeerId,
        _x25519: &[u8; 32],
    ) -> Result<(), BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn remote_session_key(&self, _txn: &dyn KvTxn, _peer: &PeerId) -> Option<[u8; 32]> {
        None
    }

    fn make_prekey_bundle(
        &self,
        _txn: &mut dyn KvTxn,
        _one_time: u8,
        _rng: &mut dyn CryptoRng,
    ) -> Result<Vec<u8>, BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn rotate_signed_prekey(
        &self,
        _txn: &mut dyn KvTxn,
        _now_mono: u64,
        _rng: &mut dyn CryptoRng,
    ) -> Result<(), BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn process_prekey_bundle(
        &self,
        _txn: &mut dyn KvTxn,
        _peer: &PeerId,
        _bundle: &[u8],
        _now_mono: u64,
        _rng: &mut dyn CryptoRng,
    ) -> Result<(), BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn encrypt(
        &self,
        _txn: &mut dyn KvTxn,
        _peer: &PeerId,
        _plain: &[u8],
        _now_mono: u64,
        _rng: &mut dyn CryptoRng,
    ) -> Result<PairwiseCt, BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn decrypt(
        &self,
        _txn: &mut dyn KvTxn,
        _peer: &PeerId,
        _ct: &PairwiseCt,
        _rng: &mut dyn CryptoRng,
    ) -> Result<Vec<u8>, BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn has_session(&self, _txn: &dyn KvTxn, _peer: &PeerId) -> bool {
        false
    }
}

impl GroupBackend for VodozemacBackend {
    fn new_sender_chain(
        &self,
        _txn: &mut dyn KvTxn,
        _group: &GroupId,
        _rng: &mut dyn CryptoRng,
    ) -> Result<(ChainId, Vec<u8>), BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn process_skdm(
        &self,
        _txn: &mut dyn KvTxn,
        _group: &GroupId,
        _sender: &PeerId,
        _skdm: &[u8],
    ) -> Result<ChainId, BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn encrypt(
        &self,
        _txn: &mut dyn KvTxn,
        _group: &GroupId,
        _plain: &[u8],
        _rng: &mut dyn CryptoRng,
    ) -> Result<Vec<u8>, BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn decrypt(
        &self,
        _txn: &mut dyn KvTxn,
        _group: &GroupId,
        _sender: &PeerId,
        _ct: &[u8],
    ) -> Result<(ChainId, Vec<u8>), BackendError> {
        Err(BackendError::Unimplemented)
    }

    fn own_chain(&self, _txn: &dyn KvTxn, _group: &GroupId) -> Option<ChainId> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;
    use crate::storage::kv::MemKv;
    use rand::SeedableRng;

    #[test]
    fn every_method_is_unimplemented_and_stateless() {
        let be = VodozemacBackend::new();
        let mut kv = MemKv::new();
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(1);
        let peer = IdentitySigner::from_seed(&[1; 32]).public();
        let group = GroupId([2; 16]);
        assert_eq!(
            be.init_identity(&mut kv, &mut rng),
            Err(BackendError::Unimplemented)
        );
        assert_eq!(be.local_x25519(&kv), None);
        assert_eq!(
            be.pin_peer_x25519(&mut kv, &peer, &[0; 32]),
            Err(BackendError::Unimplemented)
        );
        assert_eq!(be.remote_session_key(&kv, &peer), None);
        assert_eq!(
            be.make_prekey_bundle(&mut kv, 1, &mut rng),
            Err(BackendError::Unimplemented)
        );
        assert_eq!(
            be.rotate_signed_prekey(&mut kv, 0, &mut rng),
            Err(BackendError::Unimplemented)
        );
        assert_eq!(
            be.process_prekey_bundle(&mut kv, &peer, &[], 0, &mut rng),
            Err(BackendError::Unimplemented)
        );
        assert_eq!(
            PairwiseBackend::encrypt(&be, &mut kv, &peer, b"x", 0, &mut rng),
            Err(BackendError::Unimplemented)
        );
        let ct = PairwiseCt {
            kind: super::super::PairwiseKind::Normal,
            bytes: vec![],
        };
        assert_eq!(
            PairwiseBackend::decrypt(&be, &mut kv, &peer, &ct, &mut rng),
            Err(BackendError::Unimplemented)
        );
        assert!(!be.has_session(&kv, &peer));
        assert_eq!(
            be.new_sender_chain(&mut kv, &group, &mut rng),
            Err(BackendError::Unimplemented)
        );
        assert_eq!(
            be.process_skdm(&mut kv, &group, &peer, &[]),
            Err(BackendError::Unimplemented)
        );
        assert_eq!(
            GroupBackend::encrypt(&be, &mut kv, &group, b"x", &mut rng),
            Err(BackendError::Unimplemented)
        );
        assert_eq!(
            GroupBackend::decrypt(&be, &mut kv, &group, &peer, &[]),
            Err(BackendError::Unimplemented)
        );
        assert_eq!(be.own_chain(&kv, &group), None);
        assert!(kv.is_empty(), "the stub never writes");
        let name = VodozemacBackend::library_account_type();
        assert!(
            name.starts_with("vodozemac::") && name.ends_with("Account"),
            "{name}"
        );
        assert!(!VodozemacBackend::skipped_key_cap_sufficient());
    }
}
