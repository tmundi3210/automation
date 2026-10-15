//! Signed-prekey (SPK) lifecycle and bundle issuance over the backend
//! (docs/DESIGN.md §1 row `pairwise/bundles.rs`; BRIEF_v2 §5.3 "Strangers
//! use signed prekey bundles, delivered only at a QR scan, on a direct
//! encounter, or inside group encryption. ... Signed prekeys rotate every
//! 24 h and are deleted by a powered-on-time rule"; DECISIONS D6 rules 2–3;
//! §3 row (4,5,6,7)).
//!
//! * **Rotation** ([`BundleManager::maybe_rotate`]): a new SPK every
//!   [`SPK_ROTATION_MS`] of powered-on time (D6 rule 3 "rotate every 24 h").
//! * **Deletion** ([`BundleManager::retire`], D6 rule 3): a retired SPK is
//!   deleted when `(≥ 7 d of wall-clock time AND ≥ 48 h of powered-on time)
//!   OR ≥ 7 d of powered-on time` have passed since its creation. Wall time
//!   is display time only and may jump; the powered-on-only clause bounds
//!   the lifetime regardless (BRIEF §4 "no timestamps decide anything" is
//!   honoured in that a wall jump can only *delay* deletion within that
//!   bound, never decide protocol state). Deletion goes through
//!   [`SpkRetire::retire_signed_prekeys`], which also drops the Kyber
//!   prekeys issued under the retired SPK (docs/backend-notes.md §5.5).
//!   After deletion a first flight that still references the key is refused
//!   and the sender shows "May have expired · Resend" (D6 rule 7).
//! * **Issuance** ([`BundleManager::issue`]): only in the three allowed
//!   contexts ([`IssueContext`]); anything else is a programming error and
//!   is refused. One-time prekeys (OPKs) go only to a specific peer on a
//!   direct encounter or at the QR scan, never inside group encryption,
//!   because only the first use of a copy succeeds (D6 rule 2, Q12).
//! * **Base-key replay cache** ([`BundleManager::seen_base_key`], D6 rule 3
//!   "keep a base-key replay cache for SPK-only first flights until the SPK
//!   is deleted"): a bounded set of the Curve25519 base keys seen in first
//!   flights, so a captured SPK-only first flight cannot be replayed to
//!   re-run the handshake.
//!
//! The manager's state is small and persistable ([`Encode`]/[`Decode`]);
//! the node stores it with the rest of the identity. Time arrives as a
//! [`Clock`]; nothing here reads the OS clock.

use std::collections::{BTreeSet, VecDeque};

use rand::CryptoRng;

use crate::clock::{Clock, ms};
use crate::codec::{CodecError, Decode, Encode, ListItem, Reader, Writer, unknown_variant};
use crate::crypto::backend::{BackendError, GroupId, PairwiseBackend};
use crate::storage::kv::KvTxn;

/// SPK rotation interval in powered-on milliseconds (BRIEF §5.3, D6 rule 3).
pub const SPK_ROTATION_MS: u64 = ms::DAY;
/// Wall-clock part of the deletion rule: 7 days.
pub const SPK_DELETE_WALL_MS: i64 = 7 * ms::DAY as i64;
/// Powered-on part of the wall-clock clause: 48 h.
pub const SPK_DELETE_POWERED_WITH_WALL_MS: u64 = 2 * ms::DAY;
/// Powered-on-only clause: 7 days.
pub const SPK_DELETE_POWERED_ALONE_MS: u64 = 7 * ms::DAY;
/// Length of a serialized Curve25519 base key (`0x05` ‖ 32 bytes).
pub const BASE_KEY_LEN: usize = 33;
/// Entries the base-key replay cache keeps before evicting the oldest
/// (assumption; one SPK generation sees far fewer first flights).
pub const BASE_KEY_CACHE_MAX_ASM: usize = 1024;
/// Version byte of the persisted manager state.
pub const STATE_VERSION: u8 = 1;

/// Where a bundle is about to be handed out (BRIEF §5.3; DECISIONS §3 row
/// (4,5,6,7) "Bundles and envelope keys travel only: at the QR scan; on a
/// direct encounter, inside the per-hop channel; inside group encryption,
/// to co-members").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IssueContext {
    /// Shown in this node's QR code (`pairwise::pairing`).
    QrScan,
    /// Sent to one peer over a direct link, inside the per-hop channel.
    DirectEncounter,
    /// Carried to co-members inside the group's encryption.
    InsideGroup(GroupId),
    /// Gossiped as DTN traffic. Never allowed: relays would see every
    /// identity key re-broadcast every 24 h (§3 row (4,5,6,7)).
    Gossip,
}

/// Why a bundle operation was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BundleError {
    /// Bundles never travel in this context (a programming error).
    #[error("bundles are not issued in this context")]
    ContextNotAllowed,
    /// One-time prekeys are handed only to a specific peer (D6 rule 2).
    #[error("one-time prekeys are not handed out in this context")]
    OneTimeNotAllowed,
    #[error("backend: {0}")]
    Backend(#[from] BackendError),
}

/// Deletes retired signed prekeys and their Kyber prekeys below a creation
/// stamp. Implemented by the backends whose store layout allows it; the
/// trait exists because [`PairwiseBackend`] (final) does not expose it.
pub trait SpkRetire {
    /// Deletes every stored signed prekey other than the current one whose
    /// creation stamp is below `created_before_mono`; returns the count.
    fn retire_signed_prekeys(&self, txn: &mut dyn KvTxn, created_before_mono: u64) -> usize;
}

#[cfg(feature = "backend-libsignal")]
impl SpkRetire for crate::crypto::backend::libsignal::LibsignalBackend {
    fn retire_signed_prekeys(&self, txn: &mut dyn KvTxn, created_before_mono: u64) -> usize {
        crate::crypto::backend::libsignal::LibsignalBackend::retire_signed_prekeys(
            self,
            txn,
            created_before_mono,
        )
    }
}

/// One SPK generation as the manager remembers it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpkRecord {
    /// The `now_mono` stamp the backend stored with it (0 for the lazily
    /// created first SPK).
    pub created_mono: u64,
    /// Wall clock at creation, display units (`Clock::wall_for_display`).
    pub created_wall_display: i64,
}

impl SpkRecord {
    /// D6 rule 3 applied to this record at `(now_mono, now_wall)`.
    pub fn deletable(&self, now_mono: u64, now_wall_display: i64) -> bool {
        let powered = now_mono.saturating_sub(self.created_mono);
        let wall = now_wall_display.saturating_sub(self.created_wall_display);
        (wall >= SPK_DELETE_WALL_MS && powered >= SPK_DELETE_POWERED_WITH_WALL_MS)
            || powered >= SPK_DELETE_POWERED_ALONE_MS
    }
}

impl Encode for SpkRecord {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u64(self.created_mono).i64(self.created_wall_display);
        Ok(())
    }
}

impl Decode for SpkRecord {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(SpkRecord {
            created_mono: r.u64()?,
            created_wall_display: r.i64()?,
        })
    }
}

impl ListItem for SpkRecord {}

/// SPK rotation, deletion, issuance contexts and the base-key replay cache
/// (module docs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleManager {
    last_spk_rotation_mono: u64,
    /// Oldest first; the last entry is the current SPK.
    spk_created: Vec<SpkRecord>,
    /// Insertion order, for eviction.
    seen_order: VecDeque<[u8; BASE_KEY_LEN]>,
    seen_set: BTreeSet<[u8; BASE_KEY_LEN]>,
}

impl BundleManager {
    /// A manager for a node whose first SPK the backend creates lazily with
    /// stamp 0 at the first bundle (docs/backend-notes.md §5.5): that
    /// generation is recorded as created at powered-on time 0 and the given
    /// wall time, and the first rotation is due at [`SPK_ROTATION_MS`].
    pub fn new(now_wall_display: i64) -> Self {
        BundleManager {
            last_spk_rotation_mono: 0,
            spk_created: vec![SpkRecord {
                created_mono: 0,
                created_wall_display: now_wall_display,
            }],
            seen_order: VecDeque::new(),
            seen_set: BTreeSet::new(),
        }
    }

    /// Powered-on time of the last rotation (0 for the lazy first SPK).
    pub fn last_spk_rotation_mono(&self) -> u64 {
        self.last_spk_rotation_mono
    }

    /// The SPK generations still stored, oldest first (last = current).
    pub fn records(&self) -> &[SpkRecord] {
        &self.spk_created
    }

    /// Whether a rotation is due at `now_mono`.
    pub fn rotation_due(&self, now_mono: u64) -> bool {
        now_mono.saturating_sub(self.last_spk_rotation_mono) >= SPK_ROTATION_MS
    }

    /// Rotates the SPK when [`SPK_ROTATION_MS`] of powered-on time passed
    /// since the last rotation. Returns whether it rotated.
    pub fn maybe_rotate(
        &mut self,
        txn: &mut dyn KvTxn,
        backend: &dyn PairwiseBackend,
        clock: &dyn Clock,
        rng: &mut dyn CryptoRng,
    ) -> Result<bool, BundleError> {
        let now = clock.mono_ms();
        if !self.rotation_due(now) {
            return Ok(false);
        }
        backend.rotate_signed_prekey(txn, now, rng)?;
        self.last_spk_rotation_mono = now;
        self.spk_created.push(SpkRecord {
            created_mono: now,
            created_wall_display: clock.wall_for_display(),
        });
        Ok(true)
    }

    /// The creation-stamp horizon below which retired SPKs may be deleted
    /// now: records are deletable in creation order (an older record that
    /// is not yet deletable, e.g. because the wall clock jumped back, holds
    /// every later one), and the current SPK is never deleted.
    pub fn deletion_horizon(&self, now_mono: u64, now_wall_display: i64) -> u64 {
        let mut horizon = 0;
        let n = self.spk_created.len();
        for (i, r) in self.spk_created.iter().enumerate() {
            if i + 1 == n || !r.deletable(now_mono, now_wall_display) {
                break;
            }
            horizon = r.created_mono.saturating_add(1);
        }
        horizon
    }

    /// Applies D6 rule 3: deletes every retired SPK (and the Kyber prekeys
    /// under it) whose deletion time has come. Returns how many SPKs the
    /// backend deleted.
    pub fn retire(
        &mut self,
        txn: &mut dyn KvTxn,
        backend: &dyn SpkRetire,
        clock: &dyn Clock,
    ) -> usize {
        let horizon = self.deletion_horizon(clock.mono_ms(), clock.wall_for_display());
        if horizon == 0 {
            return 0;
        }
        let deleted = backend.retire_signed_prekeys(txn, horizon);
        let keep_from = self
            .spk_created
            .iter()
            .position(|r| r.created_mono >= horizon)
            .unwrap_or(self.spk_created.len().saturating_sub(1));
        self.spk_created.drain(..keep_from);
        deleted
    }

    /// Issues a prekey bundle for `context` with `one_time` one-time
    /// prekeys (BRIEF §5.3; D6 rule 2). Refused for [`IssueContext::Gossip`]
    /// and for one-time prekeys inside group encryption.
    pub fn issue(
        &self,
        txn: &mut dyn KvTxn,
        backend: &dyn PairwiseBackend,
        context: IssueContext,
        one_time: u8,
        rng: &mut dyn CryptoRng,
    ) -> Result<Vec<u8>, BundleError> {
        match context {
            IssueContext::QrScan | IssueContext::DirectEncounter => {}
            IssueContext::InsideGroup(_) => {
                if one_time > 0 {
                    return Err(BundleError::OneTimeNotAllowed);
                }
            }
            IssueContext::Gossip => return Err(BundleError::ContextNotAllowed),
        }
        Ok(backend.make_prekey_bundle(txn, one_time, rng)?)
    }

    /// Records a first flight's base key. Returns `true` if it was already
    /// seen (a replay: refuse the flight), `false` the first time. Bounded:
    /// above [`BASE_KEY_CACHE_MAX_ASM`] entries the oldest is forgotten.
    pub fn seen_base_key(&mut self, base_key: &[u8; BASE_KEY_LEN]) -> bool {
        if self.seen_set.contains(base_key) {
            return true;
        }
        while self.seen_order.len() >= BASE_KEY_CACHE_MAX_ASM {
            if let Some(old) = self.seen_order.pop_front() {
                self.seen_set.remove(&old);
            }
        }
        self.seen_order.push_back(*base_key);
        self.seen_set.insert(*base_key);
        false
    }

    /// Entries in the replay cache.
    pub fn base_key_cache_len(&self) -> usize {
        self.seen_order.len()
    }

    /// Clears the replay cache (when every SPK it protected is gone).
    pub fn clear_base_keys(&mut self) {
        self.seen_order.clear();
        self.seen_set.clear();
    }
}

/// Persisted layout: `ver u8 ‖ last_rotation u64 ‖ records list_u8 ‖
/// base_keys bytes16 (33-byte keys concatenated, oldest first)`.
impl Encode for BundleManager {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(STATE_VERSION).u64(self.last_spk_rotation_mono);
        self.spk_created.encode_to(w)?;
        let mut keys = Vec::with_capacity(self.seen_order.len() * BASE_KEY_LEN);
        for k in &self.seen_order {
            keys.extend_from_slice(k);
        }
        w.bytes16(&keys)
    }
}

impl Decode for BundleManager {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let ver = r.u8()?;
        if ver != STATE_VERSION {
            return Err(unknown_variant("bundle manager state version", ver));
        }
        let last_spk_rotation_mono = r.u64()?;
        let spk_created: Vec<SpkRecord> = Vec::decode_from(r)?;
        if spk_created
            .windows(2)
            .any(|w| w[0].created_mono > w[1].created_mono)
        {
            return Err(CodecError::Invalid("spk records order"));
        }
        let keys = r.bytes16()?;
        if keys.len() % BASE_KEY_LEN != 0 {
            return Err(CodecError::Invalid("base key cache length"));
        }
        let mut m = BundleManager {
            last_spk_rotation_mono,
            spk_created,
            seen_order: VecDeque::new(),
            seen_set: BTreeSet::new(),
        };
        for chunk in keys.chunks_exact(BASE_KEY_LEN) {
            let mut k = [0u8; BASE_KEY_LEN];
            k.copy_from_slice(chunk);
            if m.seen_set.contains(&k) {
                return Err(CodecError::Invalid("duplicate base key"));
            }
            m.seen_base_key(&k);
        }
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::ManualClock;
    use crate::codec::{decode, encode};
    use proptest::prelude::*;

    /// A backend stand-in that records retire calls.
    struct FakeRetire(std::cell::Cell<Option<u64>>);

    impl SpkRetire for FakeRetire {
        fn retire_signed_prekeys(&self, _txn: &mut dyn KvTxn, before: u64) -> usize {
            self.0.set(Some(before));
            2
        }
    }

    #[test]
    fn deletion_rule_follows_d6_rule_3() {
        let r = SpkRecord {
            created_mono: 1000,
            created_wall_display: 5_000,
        };
        let d = ms::DAY;
        // 7 d wall but < 48 h powered-on: keep
        assert!(!r.deletable(1000 + 47 * ms::HOUR, 5_000 + 7 * d as i64));
        // 7 d wall and 48 h powered-on: delete
        assert!(r.deletable(1000 + 48 * ms::HOUR, 5_000 + 7 * d as i64));
        // < 7 d wall, 48 h powered-on: keep
        assert!(!r.deletable(1000 + 48 * ms::HOUR, 5_000 + 7 * d as i64 - 1));
        // 7 d powered-on alone: delete whatever the wall clock says
        assert!(r.deletable(1000 + 7 * d, 0));
        assert!(r.deletable(1000 + 7 * d, i64::MIN));
        assert!(!r.deletable(1000 + 7 * d - 1, 5_000));
        // no overflow on extreme inputs
        assert!(r.deletable(u64::MAX, i64::MAX));
        assert!(
            !SpkRecord {
                created_mono: u64::MAX,
                created_wall_display: i64::MAX
            }
            .deletable(0, i64::MIN)
        );
    }

    #[test]
    fn horizon_is_the_deletable_prefix_and_never_the_current_spk() {
        let mut m = BundleManager::new(0);
        assert_eq!(m.records().len(), 1);
        assert_eq!(m.deletion_horizon(100 * ms::DAY, 100 * ms::DAY as i64), 0);
        m.spk_created.push(SpkRecord {
            created_mono: ms::DAY,
            created_wall_display: ms::DAY as i64,
        });
        m.spk_created.push(SpkRecord {
            created_mono: 2 * ms::DAY,
            created_wall_display: 2 * ms::DAY as i64,
        });
        // at 7 d only the first (created at 0) is 7 d old in powered-on time
        assert_eq!(m.deletion_horizon(7 * ms::DAY, 7 * ms::DAY as i64), 1);
        // at 8 d the second too; the third is current and stays
        assert_eq!(
            m.deletion_horizon(8 * ms::DAY, 8 * ms::DAY as i64),
            ms::DAY + 1
        );
        assert_eq!(
            m.deletion_horizon(100 * ms::DAY, 100 * ms::DAY as i64),
            ms::DAY + 1
        );
        // a wall clock that jumped back holds the wall clause; powered-on
        // alone still deletes after 7 d
        assert_eq!(m.deletion_horizon(3 * ms::DAY, i64::MIN / 2), 0);
        let clock = ManualClock::new(8 * ms::DAY);
        clock.set_wall_offset_ms(0);
        let fake = FakeRetire(std::cell::Cell::new(None));
        let mut kv = crate::storage::kv::MemKv::new();
        assert_eq!(m.retire(&mut kv, &fake, &clock), 2);
        assert_eq!(fake.0.get(), Some(ms::DAY + 1));
        assert_eq!(m.records().len(), 1);
        assert_eq!(m.records()[0].created_mono, 2 * ms::DAY);
        // nothing to do: the backend is not called
        fake.0.set(None);
        assert_eq!(m.retire(&mut kv, &fake, &clock), 0);
        assert_eq!(fake.0.get(), None);
    }

    #[test]
    #[cfg(feature = "backend-libsignal")] // needs a real identity in the backend
    fn issue_contexts() {
        let backend = crate::crypto::backend::default_backend();
        let mut kv = crate::storage::kv::MemKv::new();
        let mut rng = <rand_chacha::ChaCha20Rng as rand::SeedableRng>::seed_from_u64(1);
        backend.init_identity(&mut kv, &mut rng).unwrap();
        let m = BundleManager::new(0);
        let before = kv.len();
        assert_eq!(
            m.issue(&mut kv, &*backend, IssueContext::Gossip, 0, &mut rng),
            Err(BundleError::ContextNotAllowed)
        );
        assert_eq!(
            m.issue(
                &mut kv,
                &*backend,
                IssueContext::InsideGroup(GroupId([1; 16])),
                1,
                &mut rng
            ),
            Err(BundleError::OneTimeNotAllowed)
        );
        assert_eq!(kv.len(), before, "refusals write nothing");
        assert!(
            m.issue(
                &mut kv,
                &*backend,
                IssueContext::InsideGroup(GroupId([1; 16])),
                0,
                &mut rng
            )
            .is_ok()
        );
        assert!(
            m.issue(&mut kv, &*backend, IssueContext::QrScan, 2, &mut rng)
                .is_ok()
        );
        assert!(
            m.issue(
                &mut kv,
                &*backend,
                IssueContext::DirectEncounter,
                10,
                &mut rng
            )
            .is_ok()
        );
    }

    #[test]
    fn base_key_cache_is_bounded() {
        let mut m = BundleManager::new(0);
        let mut k = [0u8; BASE_KEY_LEN];
        k[0] = 5;
        assert!(!m.seen_base_key(&k));
        assert!(m.seen_base_key(&k), "replay");
        for i in 0..BASE_KEY_CACHE_MAX_ASM as u32 {
            let mut key = [0u8; BASE_KEY_LEN];
            key[1..5].copy_from_slice(&i.to_le_bytes());
            key[5] = 1;
            assert!(!m.seen_base_key(&key));
        }
        assert_eq!(m.base_key_cache_len(), BASE_KEY_CACHE_MAX_ASM);
        assert!(!m.seen_base_key(&k), "the oldest entry was evicted");
        m.clear_base_keys();
        assert_eq!(m.base_key_cache_len(), 0);
    }

    #[test]
    fn state_roundtrip_and_rejections() {
        let mut m = BundleManager::new(42);
        m.last_spk_rotation_mono = 7;
        m.spk_created.push(SpkRecord {
            created_mono: 7,
            created_wall_display: 49,
        });
        m.seen_base_key(&[1; BASE_KEY_LEN]);
        m.seen_base_key(&[2; BASE_KEY_LEN]);
        let bytes = encode(&m);
        assert_eq!(bytes[0], STATE_VERSION);
        assert_eq!(decode::<BundleManager>(&bytes), Ok(m.clone()));
        let mut bad = bytes.clone();
        bad[0] = 9;
        assert!(decode::<BundleManager>(&bad).is_err());
        // out-of-order records
        let mut w = Writer::new();
        w.u8(1).u64(0);
        vec![
            SpkRecord {
                created_mono: 5,
                created_wall_display: 0,
            },
            SpkRecord {
                created_mono: 4,
                created_wall_display: 0,
            },
        ]
        .encode_to(&mut w)
        .unwrap();
        w.bytes16(&[]).unwrap();
        assert_eq!(
            decode::<BundleManager>(w.as_bytes()),
            Err(CodecError::Invalid("spk records order"))
        );
        // key blob not a multiple of 33
        let mut w = Writer::new();
        w.u8(1).u64(0);
        Vec::<SpkRecord>::new().encode_to(&mut w).unwrap();
        w.bytes16(&[0; 34]).unwrap();
        assert_eq!(
            decode::<BundleManager>(w.as_bytes()),
            Err(CodecError::Invalid("base key cache length"))
        );
        // duplicate keys
        let mut w = Writer::new();
        w.u8(1).u64(0);
        Vec::<SpkRecord>::new().encode_to(&mut w).unwrap();
        w.bytes16(&[7; 66]).unwrap();
        assert_eq!(
            decode::<BundleManager>(w.as_bytes()),
            Err(CodecError::Invalid("duplicate base key"))
        );
    }

    proptest! {
        #[test]
        fn state_decoder_survives_random_bytes(bytes in prop::collection::vec(any::<u8>(), 0..400)) {
            if let Ok(m) = decode::<BundleManager>(&bytes) {
                prop_assert_eq!(encode(&m), bytes);
            }
        }

        #[test]
        fn deletable_is_monotone_in_time(created in 0u64..u64::MAX / 2, wall in -(1i64 << 40)..(1i64 << 40), dm in 0u64..(1 << 40), dw in 0i64..(1 << 40)) {
            let r = SpkRecord { created_mono: created, created_wall_display: wall };
            if r.deletable(created + dm, wall + dw) {
                prop_assert!(r.deletable(created + dm + 1, wall + dw + 1));
            }
        }
    }
}
