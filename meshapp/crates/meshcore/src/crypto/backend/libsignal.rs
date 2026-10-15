//! Default backend over the pinned `libsignal-protocol` crate (DESIGN §1 row
//! `crypto/backend/libsignal.rs`; docs/decisions-assumed.md row 2; D3 K1–K2;
//! D6 rule 1 "libsignal protocol crate, version pinned"). What was verified
//! against the pinned revision, and the observed message sizes, are written
//! up in `docs/backend-notes.md`.
//!
//! Shape: [`LibsignalBackend`] is a zero-sized handle. Every call opens the
//! caller's [`KvTxn`] as the six libsignal store traits through
//! [`TxnStores`], drives the library's `async fn` to completion with a no-op
//! waker ([`block_on`]: the futures never pend because the stores are
//! synchronous) and leaves every byte of state in the transaction (D6 rule 4,
//! DESIGN §6 "backend_kv (secret: sessions, sender-key records, prekeys)").
//!
//! Namespaces and keys (values are libsignal's own `serialize()` records,
//! so the library's record evolution is not re-implemented here):
//!
//! | namespace | key | value |
//! |---|---|---|
//! | `IDENTITY` | `x25519_keypair` | `IdentityKeyPair::serialize()` (33 B public ‖ 32 B private) |
//! | `IDENTITY` | `registration_id`, `current_signed_prekey_id`, `next_*_id` | u32 LE |
//! | `IDENTITY` | `own_chain/` ‖ group id | u32 LE chain id of this node's sender chain |
//! | `PEER_IDENTITIES` | address | the peer's pinned 32-byte X25519 key |
//! | `SESSIONS` | address | `SessionRecord::serialize()` |
//! | `PREKEYS`, `SIGNED_PREKEYS`, `KYBER_PREKEYS` | id u32 BE | the prekey record |
//! | `SENDER_KEYS` | address ‖ `/` ‖ group id | `SenderKeyRecord::serialize()` |
//!
//! An address is `<name>/<device>`; a peer's name is the lowercase hex of its
//! Ed25519 identity and the device id is always 1 (DESIGN §1). This node's
//! own address is `x25519:<hex of its X25519 identity>/1`: libsignal only
//! binds addresses into message MACs when they parse as Signal service ids,
//! which these never do, so the name is a storage key and nothing more; the
//! cryptographic binding of a peer to its keys is the roster KeyCard (D3 K1).
//!
//! **Time.** libsignal wants a `SystemTime` for session creation stamps. It
//! gets `UNIX_EPOCH + now_mono` ([`mono_to_system_time`]), i.e. the powered-on
//! counter, never a wall clock (BRIEF §4; D6 rule 6). The library compares
//! the stamp only with later values of the same counter (its 30-day
//! `MAX_UNACKNOWLEDGED_SESSION_AGE`), so the choice of epoch is irrelevant.

use std::cell::{RefCell, RefMut};
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, SystemTime};

use async_trait::async_trait;
use libsignal_protocol::{
    CiphertextMessage, CiphertextMessageType, DeviceId, Direction, GenericSignedPreKey,
    IdentityChange, IdentityKey, IdentityKeyPair, IdentityKeyStore, KeyPair, KyberPreKeyId,
    KyberPreKeyRecord, KyberPreKeyStore, PreKeyBundle, PreKeyId, PreKeyRecord, PreKeySignalMessage,
    PreKeyStore, ProtocolAddress, PublicKey, SenderKeyDistributionMessage, SenderKeyMessage,
    SenderKeyRecord, SenderKeyStore, SessionRecord, SessionStore, SessionUsabilityRequirements,
    SignalMessage, SignalProtocolError, SignedPreKeyId, SignedPreKeyRecord, SignedPreKeyStore,
    Timestamp, kem,
};
use rand::CryptoRng;
use uuid::Uuid;

use super::{
    BackendError, ChainId, EC_PUBLIC_KEY_LEN, GroupBackend, GroupId, OneTimePreKey,
    PairwiseBackend, PairwiseCt, PairwiseKind, PeerId, PreKeyBundleWire,
};
use crate::codec::{self, Encode};
use crate::storage::kv::{KvTxn, ns};

type SignalResult<T> = Result<T, SignalProtocolError>;

/// libsignal device id of every mesh node: one phone = one device (DESIGN §1
/// "device id 1").
const DEVICE_ID: DeviceId = match DeviceId::new(1) {
    Ok(d) => d,
    Err(_) => panic!("device id 1 is in range"),
};

/// Registration ids are drawn from Signal's 14-bit range `1..=16380`; the
/// value only labels this node's prekey bundles and sessions.
const REGISTRATION_ID_RANGE: u32 = 16380;

// Keys in `ns::IDENTITY`.
const K_X25519_KEYPAIR: &[u8] = b"x25519_keypair";
const K_REGISTRATION_ID: &[u8] = b"registration_id";
const K_CURRENT_SPK_ID: &[u8] = b"current_signed_prekey_id";
const K_NEXT_PREKEY_ID: &[u8] = b"next_prekey_id";
const K_NEXT_SPK_ID: &[u8] = b"next_signed_prekey_id";
const K_NEXT_KYBER_ID: &[u8] = b"next_kyber_prekey_id";
const K_OWN_CHAIN_PREFIX: &[u8] = b"own_chain/";

/// The default backend (docs/decisions-assumed.md row 2). Stateless: all
/// state is in the transaction passed to each call.
#[derive(Debug, Default, Clone, Copy)]
pub struct LibsignalBackend;

impl LibsignalBackend {
    pub const fn new() -> Self {
        Self
    }

    /// Deletes retired signed prekeys: every stored signed prekey other than
    /// the current one whose creation stamp (the `now_mono` given to
    /// [`PairwiseBackend::rotate_signed_prekey`]; 0 for the first, lazily
    /// created one) is below `created_before_mono`. Returns how many signed
    /// prekeys were deleted. The horizon is the caller's powered-on-time rule
    /// (BRIEF §5.3 "deleted by a powered-on-time rule"; D6 rule 3); after
    /// deletion a first flight that still references the key is refused and
    /// the sender shows "May have expired · Resend" (D6 rule 7).
    ///
    /// The identity-signed Kyber prekeys issued under a retired signed prekey
    /// go with it: [`PairwiseBackend::make_prekey_bundle`] stamps each one
    /// with its signed prekey's generation, a first flight needs both keys,
    /// and keeping them would accumulate ~3 KB of post-quantum secret per
    /// bundle ever handed out past the deletion horizon. One-time prekeys are
    /// not time-bound (D6 rule 2 reserve OPKs) and stay.
    pub fn retire_signed_prekeys(&self, txn: &mut dyn KvTxn, created_before_mono: u64) -> usize {
        let current = get_u32(txn, ns::IDENTITY, K_CURRENT_SPK_ID);
        let mut deleted = 0;
        let mut kept_generations = Vec::new();
        for (key, value) in txn.scan_prefix(ns::SIGNED_PREKEYS, &[]) {
            let Ok(record) = SignedPreKeyRecord::deserialize(&value) else {
                continue;
            };
            let (Ok(id), Ok(stamp)) = (record.id(), record.timestamp()) else {
                continue;
            };
            if Some(u32::from(id)) == current || stamp.epoch_millis() >= created_before_mono {
                kept_generations.push(stamp.epoch_millis());
                continue;
            }
            txn.delete(ns::SIGNED_PREKEYS, &key);
            deleted += 1;
        }
        for (key, value) in txn.scan_prefix(ns::KYBER_PREKEYS, &[]) {
            let Ok(record) = KyberPreKeyRecord::deserialize(&value) else {
                continue;
            };
            let Ok(stamp) = record.timestamp() else {
                continue;
            };
            if !kept_generations.contains(&stamp.epoch_millis()) {
                txn.delete(ns::KYBER_PREKEYS, &key);
            }
        }
        deleted
    }
}

// ---- small helpers ---------------------------------------------------------

/// Powered-on milliseconds as the `SystemTime` libsignal expects. This is a
/// monotonic counter re-based at the Unix epoch, not a wall clock (BRIEF §4;
/// module docs "Time").
fn mono_to_system_time(now_mono: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_millis(now_mono))
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

/// Runs one libsignal future to completion. The library's `async fn`s only
/// await the store traits, which [`TxnStores`] answers synchronously, so a
/// single poll with a no-op waker always completes; a `Pending` would mean a
/// store that actually blocks, which is an internal error here.
fn block_on<F: Future>(fut: F) -> Result<F::Output, BackendError> {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => Ok(v),
        Poll::Pending => Err(BackendError::Internal(
            "libsignal future pended on a synchronous store".into(),
        )),
    }
}

/// Maps the library's error to the opaque [`BackendError`] classes. Decrypt
/// failures stay opaque (D6 rule 7); encoding errors are "the bytes are not
/// an object"; everything that cannot be caused by peer input is `Internal`.
fn map_err(e: SignalProtocolError) -> BackendError {
    use SignalProtocolError as E;
    match e {
        E::SessionNotFound(_) => BackendError::NoSession,
        E::UntrustedIdentity(_) => BackendError::UntrustedIdentity,
        E::NoSenderKeyState { .. } => BackendError::NoSenderKey,
        E::InvalidProtobufEncoding
        | E::CiphertextMessageTooShort(_)
        | E::LegacyCiphertextVersion(_)
        | E::UnrecognizedCiphertextVersion(_)
        | E::UnrecognizedMessageVersion(_)
        | E::NoKeyTypeIdentifier
        | E::BadKeyType(_)
        | E::BadKeyLength(..)
        | E::BadKEMKeyType(_)
        | E::WrongKEMKeyType(..)
        | E::BadKEMKeyLength(..)
        | E::BadKEMCiphertextLength(..)
        | E::InvalidArgument(_)
        | E::InvalidProtocolAddress { .. } => BackendError::Encoding,
        E::SignatureValidationFailed
        | E::InvalidMessage(..)
        | E::DuplicatedMessage(..)
        | E::InvalidKeyAgreement
        | E::InvalidMacKeyLength(_)
        | E::InvalidPreKeyId
        | E::InvalidSignedPreKeyId
        | E::InvalidKyberPreKeyId
        | E::InvalidRegistrationId(..) => BackendError::Decrypt,
        other => BackendError::Internal(other.to_string()),
    }
}

/// Any parse error of a key or bundle field is an encoding error.
fn encoding<E>(_: E) -> BackendError {
    BackendError::Encoding
}

/// Error raised from inside a store callback (surfaces as `Internal`).
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct StoreError(&'static str);

fn store_err(what: &'static str) -> SignalProtocolError {
    SignalProtocolError::ApplicationCallbackError("TxnStores", Box::new(StoreError(what)))
}

/// The protocol address of a peer: lowercase hex of its Ed25519 identity,
/// device 1 (DESIGN §1).
pub(crate) fn peer_address(peer: &PeerId) -> ProtocolAddress {
    ProtocolAddress::new(peer.to_string(), DEVICE_ID)
}

/// This node's own address, named after its X25519 identity (module docs).
fn local_address(local: &IdentityKeyPair) -> ProtocolAddress {
    let name = format!(
        "x25519:{}",
        hex::encode(local.public_key().public_key_bytes())
    );
    ProtocolAddress::new(name, DEVICE_ID)
}

fn address_key(address: &ProtocolAddress) -> Vec<u8> {
    format!("{}/{}", address.name(), u32::from(address.device_id())).into_bytes()
}

fn sender_key_key(sender: &ProtocolAddress, distribution_id: Uuid) -> Vec<u8> {
    let mut key = address_key(sender);
    key.push(b'/');
    key.extend_from_slice(distribution_id.as_bytes());
    key
}

fn own_chain_key(group: &GroupId) -> Vec<u8> {
    [K_OWN_CHAIN_PREFIX, group.as_bytes()].concat()
}

/// Sender-key "distribution id" = the group id bytes as a UUID (DESIGN §1).
fn distribution_id(group: &GroupId) -> Uuid {
    Uuid::from_bytes(*group.as_bytes())
}

fn id_key(id: u32) -> [u8; 4] {
    id.to_be_bytes()
}

fn get_u32(txn: &dyn KvTxn, namespace: &str, key: &[u8]) -> Option<u32> {
    let value = txn.get(namespace, key)?;
    let bytes: [u8; 4] = value.as_slice().try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}

fn put_u32(txn: &mut dyn KvTxn, namespace: &str, key: &[u8], value: u32) {
    txn.put(namespace, key, &value.to_le_bytes());
}

/// Allocates the next prekey id from a counter in `ns::IDENTITY` (ids start
/// at 1 and never repeat within a node's lifetime short of wrapping u32).
fn next_id(txn: &mut dyn KvTxn, counter: &[u8]) -> u32 {
    let id = get_u32(txn, ns::IDENTITY, counter).unwrap_or(1);
    put_u32(txn, ns::IDENTITY, counter, id.wrapping_add(1).max(1));
    id
}

fn key33(pk: &PublicKey) -> Result<[u8; EC_PUBLIC_KEY_LEN], BackendError> {
    pk.serialize()
        .as_ref()
        .try_into()
        .map_err(|_| BackendError::Internal("curve public key is not 33 bytes".into()))
}

fn key32(pk: &PublicKey) -> Result<[u8; 32], BackendError> {
    pk.public_key_bytes()
        .try_into()
        .map_err(|_| BackendError::Internal("curve public key is not 32 bytes".into()))
}

/// This node's X25519 identity key pair and registration id, stored by
/// [`PairwiseBackend::init_identity`].
fn load_local(txn: &dyn KvTxn) -> Result<(IdentityKeyPair, u32), BackendError> {
    let bytes = txn.get(ns::IDENTITY, K_X25519_KEYPAIR).ok_or_else(|| {
        BackendError::Internal("identity not initialised: call init_identity first".into())
    })?;
    let pair = IdentityKeyPair::try_from(bytes.as_slice()).map_err(map_err)?;
    let registration_id = get_u32(txn, ns::IDENTITY, K_REGISTRATION_ID)
        .ok_or_else(|| BackendError::Internal("registration id missing".into()))?;
    Ok((pair, registration_id))
}

fn load_session(txn: &dyn KvTxn, address: &ProtocolAddress) -> Option<SessionRecord> {
    let bytes = txn.get(ns::SESSIONS, &address_key(address))?;
    SessionRecord::deserialize(&bytes).ok()
}

/// The peer's pinned X25519 key, if [`PairwiseBackend::pin_peer_x25519`] ran.
fn pinned_key(txn: &dyn KvTxn, address: &ProtocolAddress) -> Option<[u8; 32]> {
    txn.get(ns::PEER_IDENTITIES, &address_key(address))?
        .as_slice()
        .try_into()
        .ok()
}

/// The 32-byte X25519 identity key the record's current session is bound to.
fn session_remote_x25519(record: &SessionRecord) -> Option<[u8; 32]> {
    let bytes = record.remote_identity_key_bytes().ok()??;
    let key = PublicKey::deserialize(&bytes).ok()?;
    key32(&key).ok()
}

/// Creates a signed prekey stamped `now_mono`, stores it and makes it current.
fn create_signed_prekey(
    txn: &mut dyn KvTxn,
    local: &IdentityKeyPair,
    now_mono: u64,
    mut rng: &mut dyn CryptoRng,
) -> Result<SignedPreKeyRecord, BackendError> {
    let id = next_id(txn, K_NEXT_SPK_ID);
    let pair = KeyPair::generate(&mut rng);
    let signature = local
        .private_key()
        .calculate_signature(&pair.public_key.serialize(), &mut rng)
        .map_err(|e| map_err(e.into()))?;
    let record = SignedPreKeyRecord::new(
        SignedPreKeyId::from(id),
        Timestamp::from_epoch_millis(now_mono),
        &pair,
        &signature,
    );
    txn.put(
        ns::SIGNED_PREKEYS,
        &id_key(id),
        &record.serialize().map_err(map_err)?,
    );
    put_u32(txn, ns::IDENTITY, K_CURRENT_SPK_ID, id);
    Ok(record)
}

/// The current signed prekey; the first one is created lazily with stamp 0
/// (powered-on time starts at 0 on first launch) when no rotation ran yet.
fn current_signed_prekey(
    txn: &mut dyn KvTxn,
    local: &IdentityKeyPair,
    rng: &mut dyn CryptoRng,
) -> Result<SignedPreKeyRecord, BackendError> {
    if let Some(id) = get_u32(txn, ns::IDENTITY, K_CURRENT_SPK_ID) {
        let bytes = txn
            .get(ns::SIGNED_PREKEYS, &id_key(id))
            .ok_or_else(|| BackendError::Internal("current signed prekey missing".into()))?;
        return SignedPreKeyRecord::deserialize(&bytes).map_err(map_err);
    }
    create_signed_prekey(txn, local, 0, rng)
}

/// Converts a decoded wire bundle into the library's bundle; any key that does
/// not parse is an encoding error. Only the first one-time prekey is used
/// ([`PreKeyBundleWire`] docs).
fn to_library_bundle(wire: &PreKeyBundleWire) -> Result<PreKeyBundle, BackendError> {
    let identity = IdentityKey::decode(&wire.identity_key).map_err(encoding)?;
    let signed_prekey = PublicKey::deserialize(&wire.signed_prekey).map_err(encoding)?;
    let kyber = kem::PublicKey::deserialize(&wire.kyber_prekey).map_err(encoding)?;
    let one_time = match wire.one_time.first() {
        Some(k) => Some((
            PreKeyId::from(k.id),
            PublicKey::deserialize(&k.key).map_err(encoding)?,
        )),
        None => None,
    };
    PreKeyBundle::new(
        wire.registration_id,
        DEVICE_ID,
        one_time,
        SignedPreKeyId::from(wire.signed_prekey_id),
        signed_prekey,
        wire.signed_prekey_sig.clone(),
        KyberPreKeyId::from(wire.kyber_prekey_id),
        kyber,
        wire.kyber_prekey_sig.clone(),
        identity,
    )
    .map_err(encoding)
}

// ---- store traits over the transaction -------------------------------------

/// The six libsignal store traits over one [`KvTxn`] (DESIGN §1 row
/// `storage/backend_stores.rs`, realised here as one struct). The traits are
/// implemented for `&TxnStores` so that one transaction can be handed to a
/// library call as several `&mut dyn …Store` arguments at once; the inner
/// `RefCell` is only ever borrowed for the duration of one synchronous store
/// method.
///
/// Identity semantics (TOFU off, BRIEF §5.2, D3 K1): `is_trusted_identity` is
/// "the stored pin exists and equals the offered key"; `save_identity` never
/// creates or overwrites a pin and fails with `UntrustedIdentity` when the key
/// is not the pinned one. Pins are written only by
/// [`PairwiseBackend::pin_peer_x25519`].
pub(crate) struct TxnStores<'a> {
    kv: RefCell<&'a mut dyn KvTxn>,
    local: IdentityKeyPair,
    registration_id: u32,
}

impl<'a> TxnStores<'a> {
    pub(crate) fn open(txn: &'a mut dyn KvTxn) -> Result<Self, BackendError> {
        let (local, registration_id) = load_local(txn)?;
        Ok(Self {
            kv: RefCell::new(txn),
            local,
            registration_id,
        })
    }

    fn local_address(&self) -> ProtocolAddress {
        local_address(&self.local)
    }

    fn kv(&self) -> SignalResult<RefMut<'_, &'a mut dyn KvTxn>> {
        self.kv
            .try_borrow_mut()
            .map_err(|_| store_err("transaction already borrowed by another store call"))
    }

    fn get(&self, namespace: &str, key: &[u8]) -> SignalResult<Option<Vec<u8>>> {
        Ok(self.kv()?.get(namespace, key))
    }

    fn put(&self, namespace: &str, key: &[u8], value: &[u8]) -> SignalResult<()> {
        self.kv()?.put(namespace, key, value);
        Ok(())
    }

    fn delete(&self, namespace: &str, key: &[u8]) -> SignalResult<()> {
        self.kv()?.delete(namespace, key);
        Ok(())
    }

    fn pinned(&self, address: &ProtocolAddress) -> SignalResult<Option<[u8; 32]>> {
        match self.get(ns::PEER_IDENTITIES, &address_key(address))? {
            None => Ok(None),
            Some(v) => v
                .as_slice()
                .try_into()
                .map(Some)
                .map_err(|_| store_err("pinned peer key is not 32 bytes")),
        }
    }

    fn matches_pin(&self, address: &ProtocolAddress, identity: &IdentityKey) -> SignalResult<bool> {
        Ok(self
            .pinned(address)?
            .is_some_and(|pin| pin[..] == *identity.public_key().public_key_bytes()))
    }
}

#[async_trait(?Send)]
impl IdentityKeyStore for &TxnStores<'_> {
    async fn get_identity_key_pair(&self) -> SignalResult<IdentityKeyPair> {
        Ok(self.local)
    }

    async fn get_local_registration_id(&self) -> SignalResult<u32> {
        Ok(self.registration_id)
    }

    async fn save_identity(
        &mut self,
        address: &ProtocolAddress,
        identity: &IdentityKey,
    ) -> SignalResult<IdentityChange> {
        if self.matches_pin(address, identity)? {
            Ok(IdentityChange::NewOrUnchanged)
        } else {
            Err(SignalProtocolError::UntrustedIdentity(address.clone()))
        }
    }

    async fn is_trusted_identity(
        &self,
        address: &ProtocolAddress,
        identity: &IdentityKey,
        _direction: Direction,
    ) -> SignalResult<bool> {
        self.matches_pin(address, identity)
    }

    async fn get_identity(&self, address: &ProtocolAddress) -> SignalResult<Option<IdentityKey>> {
        match self.pinned(address)? {
            None => Ok(None),
            Some(pin) => Ok(Some(IdentityKey::new(
                PublicKey::from_djb_public_key_bytes(&pin)?,
            ))),
        }
    }
}

#[async_trait(?Send)]
impl PreKeyStore for &TxnStores<'_> {
    async fn get_pre_key(&self, prekey_id: PreKeyId) -> SignalResult<PreKeyRecord> {
        let bytes = self
            .get(ns::PREKEYS, &id_key(prekey_id.into()))?
            .ok_or(SignalProtocolError::InvalidPreKeyId)?;
        PreKeyRecord::deserialize(&bytes)
    }

    async fn save_pre_key(
        &mut self,
        prekey_id: PreKeyId,
        record: &PreKeyRecord,
    ) -> SignalResult<()> {
        self.put(ns::PREKEYS, &id_key(prekey_id.into()), &record.serialize()?)
    }

    /// Called by the library once the one-time prekey was consumed by a first
    /// flight (D6 rule 2 "only the first use of a copy succeeds").
    async fn remove_pre_key(&mut self, prekey_id: PreKeyId) -> SignalResult<()> {
        self.delete(ns::PREKEYS, &id_key(prekey_id.into()))
    }
}

#[async_trait(?Send)]
impl SignedPreKeyStore for &TxnStores<'_> {
    async fn get_signed_pre_key(&self, id: SignedPreKeyId) -> SignalResult<SignedPreKeyRecord> {
        let bytes = self
            .get(ns::SIGNED_PREKEYS, &id_key(id.into()))?
            .ok_or(SignalProtocolError::InvalidSignedPreKeyId)?;
        SignedPreKeyRecord::deserialize(&bytes)
    }

    async fn save_signed_pre_key(
        &mut self,
        id: SignedPreKeyId,
        record: &SignedPreKeyRecord,
    ) -> SignalResult<()> {
        self.put(ns::SIGNED_PREKEYS, &id_key(id.into()), &record.serialize()?)
    }
}

#[async_trait(?Send)]
impl KyberPreKeyStore for &TxnStores<'_> {
    async fn get_kyber_pre_key(&self, id: KyberPreKeyId) -> SignalResult<KyberPreKeyRecord> {
        let bytes = self
            .get(ns::KYBER_PREKEYS, &id_key(id.into()))?
            .ok_or(SignalProtocolError::InvalidKyberPreKeyId)?;
        KyberPreKeyRecord::deserialize(&bytes)
    }

    async fn save_kyber_pre_key(
        &mut self,
        id: KyberPreKeyId,
        record: &KyberPreKeyRecord,
    ) -> SignalResult<()> {
        self.put(ns::KYBER_PREKEYS, &id_key(id.into()), &record.serialize()?)
    }

    /// Every Kyber prekey this backend issues is one-time (a fresh one per
    /// bundle), so "used" means deleted: a second first flight against the
    /// same bundle is refused instead of reusing post-quantum key material.
    async fn mark_kyber_pre_key_used(
        &mut self,
        id: KyberPreKeyId,
        _ec_prekey_id: SignedPreKeyId,
        _base_key: &PublicKey,
    ) -> SignalResult<()> {
        self.delete(ns::KYBER_PREKEYS, &id_key(id.into()))
    }
}

#[async_trait(?Send)]
impl SessionStore for &TxnStores<'_> {
    async fn load_session(&self, address: &ProtocolAddress) -> SignalResult<Option<SessionRecord>> {
        match self.get(ns::SESSIONS, &address_key(address))? {
            None => Ok(None),
            Some(bytes) => Ok(Some(SessionRecord::deserialize(&bytes)?)),
        }
    }

    async fn store_session(
        &mut self,
        address: &ProtocolAddress,
        record: &SessionRecord,
    ) -> SignalResult<()> {
        self.put(ns::SESSIONS, &address_key(address), &record.serialize()?)
    }
}

#[async_trait(?Send)]
impl SenderKeyStore for &TxnStores<'_> {
    async fn store_sender_key(
        &mut self,
        sender: &ProtocolAddress,
        distribution_id: Uuid,
        record: &SenderKeyRecord,
    ) -> SignalResult<()> {
        self.put(
            ns::SENDER_KEYS,
            &sender_key_key(sender, distribution_id),
            &record.serialize()?,
        )
    }

    async fn load_sender_key(
        &mut self,
        sender: &ProtocolAddress,
        distribution_id: Uuid,
    ) -> SignalResult<Option<SenderKeyRecord>> {
        match self.get(ns::SENDER_KEYS, &sender_key_key(sender, distribution_id))? {
            None => Ok(None),
            Some(bytes) => Ok(Some(SenderKeyRecord::deserialize(&bytes)?)),
        }
    }
}

// ---- PairwiseBackend ---------------------------------------------------------

impl PairwiseBackend for LibsignalBackend {
    fn init_identity(
        &self,
        txn: &mut dyn KvTxn,
        mut rng: &mut dyn CryptoRng,
    ) -> Result<[u8; 32], BackendError> {
        if let Some(bytes) = txn.get(ns::IDENTITY, K_X25519_KEYPAIR) {
            let pair = IdentityKeyPair::try_from(bytes.as_slice()).map_err(map_err)?;
            return key32(pair.public_key());
        }
        let pair = IdentityKeyPair::generate(&mut rng);
        let registration_id = rng.next_u32() % REGISTRATION_ID_RANGE + 1;
        txn.put(ns::IDENTITY, K_X25519_KEYPAIR, &pair.serialize());
        put_u32(txn, ns::IDENTITY, K_REGISTRATION_ID, registration_id);
        key32(pair.public_key())
    }

    fn local_x25519(&self, txn: &dyn KvTxn) -> Option<[u8; 32]> {
        let bytes = txn.get(ns::IDENTITY, K_X25519_KEYPAIR)?;
        let pair = IdentityKeyPair::try_from(bytes.as_slice()).ok()?;
        key32(pair.public_key()).ok()
    }

    fn pin_peer_x25519(
        &self,
        txn: &mut dyn KvTxn,
        peer: &PeerId,
        x25519: &[u8; 32],
    ) -> Result<(), BackendError> {
        txn.put(
            ns::PEER_IDENTITIES,
            &address_key(&peer_address(peer)),
            x25519,
        );
        Ok(())
    }

    fn remote_session_key(&self, txn: &dyn KvTxn, peer: &PeerId) -> Option<[u8; 32]> {
        session_remote_x25519(&load_session(txn, &peer_address(peer))?)
    }

    fn make_prekey_bundle(
        &self,
        txn: &mut dyn KvTxn,
        one_time: u8,
        mut rng: &mut dyn CryptoRng,
    ) -> Result<Vec<u8>, BackendError> {
        let (local, registration_id) = load_local(txn)?;
        let spk = current_signed_prekey(txn, &local, rng)?;
        let generation = spk.timestamp().map_err(map_err)?;

        // A fresh one-time Kyber prekey per bundle (deleted on first use).
        let kyber_id = next_id(txn, K_NEXT_KYBER_ID);
        let kyber_pair = kem::KeyPair::generate(kem::KeyType::Kyber1024, &mut rng);
        let kyber_public = kyber_pair.public_key.serialize();
        let kyber_sig = local
            .private_key()
            .calculate_signature(&kyber_public, &mut rng)
            .map_err(|e| map_err(e.into()))?;
        let kyber_record = KyberPreKeyRecord::new(
            KyberPreKeyId::from(kyber_id),
            generation,
            &kyber_pair,
            &kyber_sig,
        );
        txn.put(
            ns::KYBER_PREKEYS,
            &id_key(kyber_id),
            &kyber_record.serialize().map_err(map_err)?,
        );

        let mut one_time_keys = Vec::with_capacity(usize::from(one_time));
        for _ in 0..one_time {
            let id = next_id(txn, K_NEXT_PREKEY_ID);
            let pair = KeyPair::generate(&mut rng);
            let record = PreKeyRecord::new(PreKeyId::from(id), &pair);
            txn.put(
                ns::PREKEYS,
                &id_key(id),
                &record.serialize().map_err(map_err)?,
            );
            one_time_keys.push(OneTimePreKey {
                id,
                key: key33(&pair.public_key)?,
            });
        }

        let wire = PreKeyBundleWire {
            registration_id,
            one_time: one_time_keys,
            signed_prekey_id: spk.id().map_err(map_err)?.into(),
            signed_prekey: key33(&spk.public_key().map_err(map_err)?)?,
            signed_prekey_sig: spk.signature().map_err(map_err)?,
            kyber_prekey_id: kyber_id,
            kyber_prekey: kyber_public.into_vec(),
            kyber_prekey_sig: kyber_sig.into_vec(),
            identity_key: key33(local.public_key())?,
        };
        wire.try_encode().map_err(BackendError::from)
    }

    fn rotate_signed_prekey(
        &self,
        txn: &mut dyn KvTxn,
        now_mono: u64,
        rng: &mut dyn CryptoRng,
    ) -> Result<(), BackendError> {
        let (local, _) = load_local(txn)?;
        create_signed_prekey(txn, &local, now_mono, rng).map(drop)
    }

    fn process_prekey_bundle(
        &self,
        txn: &mut dyn KvTxn,
        peer: &PeerId,
        bundle: &[u8],
        now_mono: u64,
        mut rng: &mut dyn CryptoRng,
    ) -> Result<(), BackendError> {
        let wire: PreKeyBundleWire = codec::decode(bundle)?;
        let bundle = to_library_bundle(&wire)?;
        let remote = peer_address(peer);
        let stores = TxnStores::open(txn)?;
        let local = stores.local_address();
        let (mut sessions, mut identities) = (&stores, &stores);
        block_on(libsignal_protocol::process_prekey_bundle(
            &remote,
            &local,
            &mut sessions,
            &mut identities,
            &bundle,
            mono_to_system_time(now_mono),
            &mut rng,
        ))?
        .map_err(|e| match e {
            // the identity key's signature over the SPK/Kyber key does not verify:
            // a forged or corrupt bundle, not a decrypt failure
            SignalProtocolError::SignatureValidationFailed => BackendError::Encoding,
            other => map_err(other),
        })
    }

    fn encrypt(
        &self,
        txn: &mut dyn KvTxn,
        peer: &PeerId,
        plain: &[u8],
        now_mono: u64,
        mut rng: &mut dyn CryptoRng,
    ) -> Result<PairwiseCt, BackendError> {
        let remote = peer_address(peer);
        let stores = TxnStores::open(txn)?;
        let local = stores.local_address();
        let (mut sessions, mut identities) = (&stores, &stores);
        let message = block_on(libsignal_protocol::message_encrypt(
            plain,
            &remote,
            &local,
            &mut sessions,
            &mut identities,
            mono_to_system_time(now_mono),
            &mut rng,
        ))?
        .map_err(map_err)?;
        let kind = match message.message_type() {
            CiphertextMessageType::PreKey => PairwiseKind::PreKey,
            CiphertextMessageType::Whisper => PairwiseKind::Normal,
            other => {
                return Err(BackendError::Internal(format!(
                    "unexpected pairwise message type {other:?}"
                )));
            }
        };
        Ok(PairwiseCt {
            kind,
            bytes: message.serialize().to_vec(),
        })
    }

    fn decrypt(
        &self,
        txn: &mut dyn KvTxn,
        peer: &PeerId,
        ct: &PairwiseCt,
        mut rng: &mut dyn CryptoRng,
    ) -> Result<Vec<u8>, BackendError> {
        let message = match ct.kind {
            PairwiseKind::PreKey => CiphertextMessage::PreKeySignalMessage(
                PreKeySignalMessage::try_from(ct.bytes.as_slice()).map_err(map_err)?,
            ),
            PairwiseKind::Normal => CiphertextMessage::SignalMessage(
                SignalMessage::try_from(ct.bytes.as_slice()).map_err(map_err)?,
            ),
        };
        let remote = peer_address(peer);
        let stores = TxnStores::open(txn)?;
        let local = stores.local_address();
        let (mut sessions, mut identities, mut prekeys, signed, mut kyber) =
            (&stores, &stores, &stores, &stores, &stores);
        block_on(libsignal_protocol::message_decrypt(
            &message,
            &remote,
            &local,
            &mut sessions,
            &mut identities,
            &mut prekeys,
            &signed,
            &mut kyber,
            &mut rng,
        ))?
        .map_err(map_err)
    }

    /// True when a session record with a sending chain exists *and* its remote
    /// identity key is still the pinned one: TOFU is off, so after a re-pin
    /// `message_encrypt` refuses the session (`UntrustedIdentity`) and the
    /// pairwise layer must re-run the bundle ("Needs their key", D6 rule 7).
    /// The library's staleness check is not requested, so its `now` argument
    /// is unused and no clock enters (D6 rule 6 "established sessions never
    /// expire on a clock").
    fn has_session(&self, txn: &dyn KvTxn, peer: &PeerId) -> bool {
        let address = peer_address(peer);
        let Some(record) = load_session(txn, &address) else {
            return false;
        };
        let usable = record
            .has_usable_sender_chain(
                SystemTime::UNIX_EPOCH,
                SessionUsabilityRequirements::empty(),
            )
            .unwrap_or(false);
        if !usable {
            return false;
        }
        match (pinned_key(txn, &address), session_remote_x25519(&record)) {
            (Some(pin), Some(key)) => pin == key,
            _ => false,
        }
    }
}

// ---- GroupBackend ------------------------------------------------------------

impl GroupBackend for LibsignalBackend {
    /// Rotation without a fork: the library returns the *existing* chain when
    /// a record exists, so this deletes this node's own record (and chain
    /// marker) first and then creates a fresh one (D3 K2; backend notes).
    fn new_sender_chain(
        &self,
        txn: &mut dyn KvTxn,
        group: &GroupId,
        mut rng: &mut dyn CryptoRng,
    ) -> Result<(ChainId, Vec<u8>), BackendError> {
        let dist = distribution_id(group);
        let (local, _) = load_local(txn)?;
        let me = local_address(&local);
        txn.delete(ns::SENDER_KEYS, &sender_key_key(&me, dist));
        txn.delete(ns::IDENTITY, &own_chain_key(group));
        let skdm = {
            let stores = TxnStores::open(txn)?;
            let mut sender_keys = &stores;
            block_on(libsignal_protocol::create_sender_key_distribution_message(
                &me,
                dist,
                &mut sender_keys,
                &mut rng,
            ))?
            .map_err(map_err)?
        };
        let chain = skdm.chain_id().map_err(map_err)?;
        put_u32(txn, ns::IDENTITY, &own_chain_key(group), chain);
        Ok((ChainId(chain), skdm.serialized().to_vec()))
    }

    /// Stores `sender`'s chain. An SKDM whose distribution id is not `group`
    /// is rejected as malformed so it cannot plant a chain under another
    /// group's record. The library keeps the sender's previous chains
    /// (`MAX_SENDER_KEY_STATES = 5`) and leaves an already-known chain's
    /// position untouched when its SKDM is delivered again.
    fn process_skdm(
        &self,
        txn: &mut dyn KvTxn,
        group: &GroupId,
        sender: &PeerId,
        skdm: &[u8],
    ) -> Result<ChainId, BackendError> {
        let message = SenderKeyDistributionMessage::try_from(skdm).map_err(map_err)?;
        if message.distribution_id().map_err(map_err)? != distribution_id(group) {
            return Err(BackendError::Encoding);
        }
        let chain = message.chain_id().map_err(map_err)?;
        let from = peer_address(sender);
        let stores = TxnStores::open(txn)?;
        let mut sender_keys = &stores;
        block_on(libsignal_protocol::process_sender_key_distribution_message(
            &from,
            &message,
            &mut sender_keys,
        ))?
        .map_err(map_err)?;
        Ok(ChainId(chain))
    }

    fn encrypt(
        &self,
        txn: &mut dyn KvTxn,
        group: &GroupId,
        plain: &[u8],
        mut rng: &mut dyn CryptoRng,
    ) -> Result<Vec<u8>, BackendError> {
        let dist = distribution_id(group);
        let stores = TxnStores::open(txn)?;
        let me = stores.local_address();
        let mut sender_keys = &stores;
        let message = block_on(libsignal_protocol::group_encrypt(
            &mut sender_keys,
            &me,
            dist,
            plain,
            &mut rng,
        ))?
        .map_err(map_err)?;
        Ok(message.serialized().to_vec())
    }

    fn decrypt(
        &self,
        txn: &mut dyn KvTxn,
        group: &GroupId,
        sender: &PeerId,
        ct: &[u8],
    ) -> Result<(ChainId, Vec<u8>), BackendError> {
        let message = SenderKeyMessage::try_from(ct).map_err(map_err)?;
        if message.distribution_id() != distribution_id(group) {
            return Err(BackendError::Encoding);
        }
        let chain = ChainId(message.chain_id());
        let from = peer_address(sender);
        let stores = TxnStores::open(txn)?;
        let mut sender_keys = &stores;
        let plain = block_on(libsignal_protocol::group_decrypt(
            ct,
            &mut sender_keys,
            &from,
        ))?
        .map_err(map_err)?;
        Ok((chain, plain))
    }

    fn own_chain(&self, txn: &dyn KvTxn, group: &GroupId) -> Option<ChainId> {
        get_u32(txn, ns::IDENTITY, &own_chain_key(group)).map(ChainId)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;
    use crate::storage::kv::MemKv;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    fn rng(seed: u64) -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(seed)
    }

    fn peer(seed: u8) -> PeerId {
        IdentitySigner::from_seed(&[seed; 32]).public()
    }

    #[test]
    fn block_on_completes_ready_futures_and_reports_pending() {
        assert_eq!(block_on(async { 7 }), Ok(7));
        struct Never;
        impl Future for Never {
            type Output = ();
            fn poll(self: std::pin::Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
                Poll::Pending
            }
        }
        assert!(matches!(block_on(Never), Err(BackendError::Internal(_))));
    }

    #[test]
    fn mono_time_is_epoch_plus_counter() {
        assert_eq!(mono_to_system_time(0), SystemTime::UNIX_EPOCH);
        assert_eq!(
            mono_to_system_time(1_500),
            SystemTime::UNIX_EPOCH + Duration::from_millis(1_500)
        );
        // never panics on absurd counters
        let _ = mono_to_system_time(u64::MAX);
    }

    #[test]
    fn error_mapping_classes() {
        use SignalProtocolError as E;
        let addr = peer_address(&peer(1));
        assert_eq!(
            map_err(E::SessionNotFound(
                libsignal_protocol::SessionNotFound::new(addr.clone(), "t")
            )),
            BackendError::NoSession
        );
        assert_eq!(
            map_err(E::UntrustedIdentity(addr)),
            BackendError::UntrustedIdentity
        );
        assert_eq!(
            map_err(E::NoSenderKeyState {
                distribution_id: Uuid::nil()
            }),
            BackendError::NoSenderKey
        );
        assert_eq!(map_err(E::InvalidProtobufEncoding), BackendError::Encoding);
        assert_eq!(map_err(E::DuplicatedMessage(3, 1)), BackendError::Decrypt);
        assert_eq!(map_err(E::InvalidKyberPreKeyId), BackendError::Decrypt);
        assert!(matches!(
            map_err(E::InvalidSessionStructure("x")),
            BackendError::Internal(_)
        ));
        assert!(
            matches!(map_err(store_err("busy")), BackendError::Internal(m) if m.contains("busy"))
        );
    }

    #[test]
    fn addresses_and_keys_are_deterministic() {
        let p = peer(2);
        let a = peer_address(&p);
        assert_eq!(a.name(), p.to_string());
        assert_eq!(a.name().len(), 64);
        assert!(
            a.name()
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        );
        assert_eq!(u32::from(a.device_id()), 1);
        assert_eq!(address_key(&a), format!("{}/1", p).into_bytes());
        let g = GroupId([9; 16]);
        let d = distribution_id(&g);
        assert_eq!(d.as_bytes(), &[9; 16]);
        assert_eq!(
            sender_key_key(&a, d),
            [address_key(&a), b"/".to_vec(), vec![9; 16]].concat()
        );
        assert_eq!(
            own_chain_key(&g),
            [b"own_chain/".to_vec(), vec![9; 16]].concat()
        );
        assert_eq!(id_key(0x0102_0304), [1, 2, 3, 4]);
    }

    #[test]
    fn id_counters_start_at_one_and_advance() {
        let mut kv = MemKv::new();
        assert_eq!(next_id(&mut kv, K_NEXT_PREKEY_ID), 1);
        assert_eq!(next_id(&mut kv, K_NEXT_PREKEY_ID), 2);
        assert_eq!(next_id(&mut kv, K_NEXT_SPK_ID), 1);
        assert_eq!(get_u32(&kv, ns::IDENTITY, K_NEXT_PREKEY_ID), Some(3));
        kv.put(ns::IDENTITY, b"bad", b"xyz");
        assert_eq!(get_u32(&kv, ns::IDENTITY, b"bad"), None);
    }

    #[test]
    fn init_identity_is_idempotent_and_deterministic() {
        let be = LibsignalBackend::new();
        let mut kv = MemKv::new();
        assert_eq!(be.local_x25519(&kv), None);
        let k1 = be.init_identity(&mut kv, &mut rng(1)).unwrap();
        let k2 = be.init_identity(&mut kv, &mut rng(99)).unwrap();
        assert_eq!(k1, k2, "second call returns the stored key");
        assert_eq!(be.local_x25519(&kv), Some(k1));
        let reg = get_u32(&kv, ns::IDENTITY, K_REGISTRATION_ID).unwrap();
        assert!((1..=REGISTRATION_ID_RANGE).contains(&reg));
        let mut other = MemKv::new();
        let k3 = be.init_identity(&mut other, &mut rng(1)).unwrap();
        assert_eq!(k1, k3, "same rng stream, same key");
        let (pair, _) = load_local(&kv).unwrap();
        assert_eq!(key32(pair.public_key()).unwrap(), k1);
        assert_eq!(key33(pair.public_key()).unwrap()[0], 0x05);
    }

    #[test]
    fn identity_store_has_tofu_off() {
        let be = LibsignalBackend::new();
        let mut kv = MemKv::new();
        be.init_identity(&mut kv, &mut rng(1)).unwrap();
        let p = peer(3);
        let addr = peer_address(&p);
        let offered_pair = IdentityKeyPair::generate(&mut rng(5));
        let offered = *offered_pair.identity_key();
        let other = *IdentityKeyPair::generate(&mut rng(6)).identity_key();

        {
            let stores = TxnStores::open(&mut kv).unwrap();
            let mut s = &stores;
            assert!(
                !block_on(s.is_trusted_identity(&addr, &offered, Direction::Sending))
                    .unwrap()
                    .unwrap()
            );
            assert!(
                matches!(
                    block_on(s.save_identity(&addr, &offered)).unwrap(),
                    Err(SignalProtocolError::UntrustedIdentity(_))
                ),
                "save never creates a pin"
            );
            assert!(block_on(s.get_identity(&addr)).unwrap().unwrap().is_none());
        }
        assert!(kv.get(ns::PEER_IDENTITIES, &address_key(&addr)).is_none());

        be.pin_peer_x25519(&mut kv, &p, &key32(offered.public_key()).unwrap())
            .unwrap();
        let stores = TxnStores::open(&mut kv).unwrap();
        let mut s = &stores;
        assert!(
            block_on(s.is_trusted_identity(&addr, &offered, Direction::Receiving))
                .unwrap()
                .unwrap()
        );
        assert!(
            !block_on(s.is_trusted_identity(&addr, &other, Direction::Receiving))
                .unwrap()
                .unwrap()
        );
        assert_eq!(
            block_on(s.save_identity(&addr, &offered)).unwrap().unwrap(),
            IdentityChange::NewOrUnchanged
        );
        assert!(
            matches!(
                block_on(s.save_identity(&addr, &other)).unwrap(),
                Err(SignalProtocolError::UntrustedIdentity(_))
            ),
            "save never overwrites a different pin"
        );
        assert_eq!(
            block_on(s.get_identity(&addr)).unwrap().unwrap(),
            Some(offered)
        );
        assert_eq!(
            block_on(s.get_identity_key_pair())
                .unwrap()
                .unwrap()
                .public_key(),
            stores.local.public_key()
        );
        assert_eq!(
            block_on(s.get_local_registration_id()).unwrap().unwrap(),
            stores.registration_id
        );
    }

    #[test]
    fn prekey_stores_round_trip_and_kyber_used_means_deleted() {
        let be = LibsignalBackend::new();
        let mut kv = MemKv::new();
        be.init_identity(&mut kv, &mut rng(1)).unwrap();
        let (local, _) = load_local(&kv).unwrap();
        let mut r = rng(2);
        let pair = KeyPair::generate(&mut r);
        let kyber_pair = kem::KeyPair::generate(kem::KeyType::Kyber1024, &mut r);
        let kyber = KyberPreKeyRecord::new(
            KyberPreKeyId::from(4),
            Timestamp::from_epoch_millis(0),
            &kyber_pair,
            &[0; 64],
        );
        let spk = create_signed_prekey(&mut kv, &local, 42, &mut r).unwrap();
        assert_eq!(spk.timestamp().unwrap().epoch_millis(), 42);

        let stores = TxnStores::open(&mut kv).unwrap();
        let mut s = &stores;
        assert!(matches!(
            block_on(s.get_pre_key(PreKeyId::from(1))).unwrap(),
            Err(SignalProtocolError::InvalidPreKeyId)
        ));
        block_on(s.save_pre_key(
            PreKeyId::from(1),
            &PreKeyRecord::new(PreKeyId::from(1), &pair),
        ))
        .unwrap()
        .unwrap();
        assert_eq!(
            block_on(s.get_pre_key(PreKeyId::from(1)))
                .unwrap()
                .unwrap()
                .public_key()
                .unwrap(),
            pair.public_key
        );
        block_on(s.remove_pre_key(PreKeyId::from(1)))
            .unwrap()
            .unwrap();
        assert!(block_on(s.get_pre_key(PreKeyId::from(1))).unwrap().is_err());

        assert_eq!(
            block_on(s.get_signed_pre_key(spk.id().unwrap()))
                .unwrap()
                .unwrap()
                .public_key()
                .unwrap(),
            spk.public_key().unwrap()
        );
        assert!(matches!(
            block_on(s.get_signed_pre_key(SignedPreKeyId::from(77))).unwrap(),
            Err(SignalProtocolError::InvalidSignedPreKeyId)
        ));

        block_on(s.save_kyber_pre_key(KyberPreKeyId::from(4), &kyber))
            .unwrap()
            .unwrap();
        assert!(
            block_on(s.get_kyber_pre_key(KyberPreKeyId::from(4)))
                .unwrap()
                .is_ok()
        );
        block_on(s.mark_kyber_pre_key_used(
            KyberPreKeyId::from(4),
            SignedPreKeyId::from(1),
            &pair.public_key,
        ))
        .unwrap()
        .unwrap();
        assert!(matches!(
            block_on(s.get_kyber_pre_key(KyberPreKeyId::from(4))).unwrap(),
            Err(SignalProtocolError::InvalidKyberPreKeyId)
        ));
        assert!(
            block_on(s.load_session(&peer_address(&peer(1))))
                .unwrap()
                .unwrap()
                .is_none()
        );
        assert!(
            block_on(s.load_sender_key(&peer_address(&peer(1)), Uuid::nil()))
                .unwrap()
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn retire_signed_prekeys_keeps_current_and_newer() {
        let be = LibsignalBackend::new();
        let mut kv = MemKv::new();
        be.init_identity(&mut kv, &mut rng(1)).unwrap();
        let mut r = rng(3);
        let bundle = be.make_prekey_bundle(&mut kv, 0, &mut r).unwrap();
        let wire: PreKeyBundleWire = codec::decode(&bundle).unwrap();
        assert_eq!(
            wire.signed_prekey_id, 1,
            "first signed prekey is created lazily"
        );
        be.rotate_signed_prekey(&mut kv, 10, &mut r).unwrap();
        let _ = be.make_prekey_bundle(&mut kv, 0, &mut r).unwrap();
        be.rotate_signed_prekey(&mut kv, 20, &mut r).unwrap();
        assert_eq!(kv.scan_prefix(ns::SIGNED_PREKEYS, &[]).len(), 3);
        assert_eq!(
            kv.scan_prefix(ns::KYBER_PREKEYS, &[]).len(),
            2,
            "one Kyber prekey under stamp 0, one under stamp 10"
        );
        assert_eq!(
            be.retire_signed_prekeys(&mut kv, 10),
            1,
            "only the stamp-0 key is older"
        );
        assert_eq!(
            kv.scan_prefix(ns::KYBER_PREKEYS, &[]).len(),
            1,
            "the stamp-0 generation's Kyber prekey went with its SPK"
        );
        assert_eq!(
            be.retire_signed_prekeys(&mut kv, u64::MAX),
            1,
            "the current key (id 3) survives"
        );
        assert_eq!(kv.scan_prefix(ns::SIGNED_PREKEYS, &[]).len(), 1);
        assert_eq!(kv.scan_prefix(ns::KYBER_PREKEYS, &[]).len(), 0);
        assert_eq!(get_u32(&kv, ns::IDENTITY, K_CURRENT_SPK_ID), Some(3));
        let later: PreKeyBundleWire =
            codec::decode(&be.make_prekey_bundle(&mut kv, 0, &mut r).unwrap()).unwrap();
        assert_eq!(later.signed_prekey_id, 3);
        assert_eq!(kv.scan_prefix(ns::KYBER_PREKEYS, &[]).len(), 1);
        assert_eq!(
            be.retire_signed_prekeys(&mut kv, u64::MAX),
            0,
            "nothing older than the current generation"
        );
        assert_eq!(
            kv.scan_prefix(ns::KYBER_PREKEYS, &[]).len(),
            1,
            "the current generation's Kyber prekey is kept"
        );
    }

    #[test]
    fn operations_need_an_initialised_identity() {
        let be = LibsignalBackend::new();
        let mut kv = MemKv::new();
        let p = peer(4);
        let g = GroupId([1; 16]);
        assert!(matches!(
            be.make_prekey_bundle(&mut kv, 1, &mut rng(1)),
            Err(BackendError::Internal(_))
        ));
        assert!(matches!(
            PairwiseBackend::encrypt(&be, &mut kv, &p, b"x", 0, &mut rng(1)),
            Err(BackendError::Internal(_))
        ));
        assert!(matches!(
            be.new_sender_chain(&mut kv, &g, &mut rng(1)),
            Err(BackendError::Internal(_))
        ));
        assert!(!be.has_session(&kv, &p));
        assert_eq!(be.remote_session_key(&kv, &p), None);
        assert_eq!(be.own_chain(&kv, &g), None);
    }
}
