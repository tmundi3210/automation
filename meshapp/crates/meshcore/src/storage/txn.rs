//! Atomic transaction over the SQLite store (docs/DESIGN.md §1 row
//! `storage/txn.rs` "all state mutations go through one `Txn`"; §6 "Atomic
//! commit"; D6 rule 4 "Message keys are deleted on decrypt. Ratchet state and
//! plaintext are committed in one atomic transaction before any 'Delivered'
//! ack"; BRIEF_v2 §5.3).
//!
//! [`Txn`] wraps a `rusqlite::Transaction` and implements
//! [`KvTxn`](super::kv::KvTxn), so the crypto backends, the roster and the
//! node write through the same object. Dropping a `Txn` without
//! [`Txn::commit`] rolls everything back; `commit` makes all of it visible at
//! once.
//!
//! # Encryption at rest (DESIGN §6 "Secret tables are encrypted at the
//! application layer with XChaCha20-Poly1305 under a 32-B `db_key`";
//! BRIEF_v2 §3 "The pin store is encrypted at rest")
//!
//! Values in a namespace listed in [`SECRET_NAMESPACES`] are stored as
//! `nonce(24) ‖ XChaCha20-Poly1305(k_ns, nonce, aad, plaintext)` with
//! `k_ns = HKDF-SHA256(db_key, "", "meshapp.v1.kv." ‖ ns)` and
//! `aad = ns ‖ 0x00 ‖ key`, so a ciphertext cannot be moved to another key
//! or namespace. The nonce is drawn fresh for every write from the
//! `ChaCha20Rng` the [`Db`](super::db::Db) seeded at open. Other namespaces
//! (ciphertext cells, hashes, counters, the dedupe cache) are stored as is.
//!
//! # Errors
//!
//! The `KvTxn` trait has no error channel. A SQLite or decryption failure
//! inside `get`/`put`/`delete`/`scan_prefix` is recorded as the transaction's
//! first error, the call returns `None`/empty/no-op, and [`Txn::commit`]
//! returns that error instead of committing: a transaction that observed a
//! failure can never land partial state. Callers that want the error at the
//! call site use the `try_*` methods.

use core::cell::RefCell;
use core::fmt;
use std::collections::BTreeMap;

use rand::RngCore;
use rand_chacha::ChaCha20Rng;
use rusqlite::{OptionalExtension, Transaction, params};

use crate::crypto::aead::{Key32, TAG_LEN, XNONCE_LEN, open_x, seal_x};

use super::db::{StorageError, meta_delete_in, meta_get_in, meta_get_u64_in, meta_set_in};
use super::kv::{KvTxn, ns};

/// Namespaces whose values are encrypted at rest (DESIGN §6: `identity`,
/// `contacts`, `chains` recog epochs, `backend_kv` = sessions, sender keys,
/// prekeys; `objects.plaintext`; BRIEF_v2 §5.3 "keys, ratchet state and the
/// plaintext outbox"). Everything else is stored in the clear.
pub const SECRET_NAMESPACES: &[&str] = &[
    ns::IDENTITY,
    ns::SESSIONS,
    ns::PREKEYS,
    ns::SIGNED_PREKEYS,
    ns::KYBER_PREKEYS,
    ns::SENDER_KEYS,
    ns::PEER_IDENTITIES,
    "contacts",
    "chains",
    "plaintext",
    "outbox",
];

/// Bytes added to a plaintext by the secret-namespace encoding:
/// 24-byte nonce + 16-byte Poly1305 tag.
pub const SECRET_OVERHEAD: usize = XNONCE_LEN + TAG_LEN;

/// `true` when values in `ns` are encrypted at rest.
pub fn is_secret_namespace(ns: &str) -> bool {
    SECRET_NAMESPACES.contains(&ns)
}

/// A `(key, value)` pair as returned by scans.
pub type KvPair = (Vec<u8>, Vec<u8>);

/// One atomic unit of state change. See the module docs.
pub struct Txn<'a> {
    txn: Transaction<'a>,
    ns_keys: &'a BTreeMap<&'static str, Key32>,
    rng: &'a mut ChaCha20Rng,
    first_error: RefCell<Option<StorageError>>,
    fail_before_commit: bool,
}

impl fmt::Debug for Txn<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Txn")
            .field("poisoned", &self.is_poisoned())
            .field("fail_before_commit", &self.fail_before_commit)
            .finish_non_exhaustive()
    }
}

impl<'a> Txn<'a> {
    pub(crate) fn new(
        txn: Transaction<'a>,
        ns_keys: &'a BTreeMap<&'static str, Key32>,
        rng: &'a mut ChaCha20Rng,
        fail_before_commit: bool,
    ) -> Self {
        Txn {
            txn,
            ns_keys,
            rng,
            first_error: RefCell::new(None),
            fail_before_commit,
        }
    }

    /// Commits every write of this transaction at once (D6 rule 4).
    ///
    /// Returns `Err` and writes nothing durable when a `KvTxn` call recorded
    /// an error ([`Txn::is_poisoned`]), when the fault-injection flag is set
    /// ([`StorageError::FaultInjected`]), or when SQLite refuses the commit.
    pub fn commit(self) -> Result<(), StorageError> {
        if let Some(err) = self.first_error.borrow_mut().take() {
            // `self.txn` is dropped at the end of this function: rollback.
            return Err(err);
        }
        if self.fail_before_commit {
            return Err(StorageError::FaultInjected);
        }
        self.txn.commit()?;
        Ok(())
    }

    /// Explicit rollback (the same as dropping the transaction).
    pub fn rollback(self) -> Result<(), StorageError> {
        self.txn.rollback()?;
        Ok(())
    }

    /// Fault injection for crash tests (DESIGN §10 "kill between decrypt and
    /// commit via a fault-injection hook"): when set, [`Txn::commit`] returns
    /// [`StorageError::FaultInjected`] and rolls back, exactly as a process
    /// kill before the SQLite commit would. The simulator uses this to check
    /// that a replayed cell yields exactly one delivery.
    pub fn set_fail_before_commit(&mut self, fail: bool) {
        self.fail_before_commit = fail;
    }

    /// `true` once a `KvTxn` call has failed; `commit` will then return the
    /// recorded error.
    pub fn is_poisoned(&self) -> bool {
        self.first_error.borrow().is_some()
    }

    fn record(&self, err: StorageError) {
        let mut slot = self.first_error.borrow_mut();
        if slot.is_none() {
            *slot = Some(err);
        }
    }

    /// Reads a value, decrypting it when `ns` is secret.
    pub fn try_get(&self, ns: &str, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
        let mut stmt = self
            .txn
            .prepare_cached("SELECT value FROM kv WHERE ns = ?1 AND key = ?2")?;
        let stored: Option<Vec<u8>> = stmt
            .query_row(params![ns, key], |row| row.get(0))
            .optional()?;
        match stored {
            None => Ok(None),
            Some(v) => self.decode_value(ns, key, &v).map(Some),
        }
    }

    /// Writes (or overwrites) a value, encrypting it when `ns` is secret.
    pub fn try_put(&mut self, ns: &str, key: &[u8], value: &[u8]) -> Result<(), StorageError> {
        let stored = self.encode_value(ns, key, value);
        let mut stmt = self
            .txn
            .prepare_cached("INSERT OR REPLACE INTO kv (ns, key, value) VALUES (?1, ?2, ?3)")?;
        stmt.execute(params![ns, key, stored])?;
        Ok(())
    }

    /// Deletes a value; a missing key is not an error. With
    /// `secure_delete=ON` the freed page bytes are overwritten.
    pub fn try_delete(&mut self, ns: &str, key: &[u8]) -> Result<(), StorageError> {
        let mut stmt = self
            .txn
            .prepare_cached("DELETE FROM kv WHERE ns = ?1 AND key = ?2")?;
        stmt.execute(params![ns, key])?;
        Ok(())
    }

    /// All `(key, value)` pairs in `ns` whose key starts with `prefix`, in
    /// byte-lexicographic key order (SQLite compares BLOBs with `memcmp`),
    /// values decrypted.
    pub fn try_scan_prefix(&self, ns: &str, prefix: &[u8]) -> Result<Vec<KvPair>, StorageError> {
        let mut out = Vec::new();
        let rows: Vec<KvPair> = match prefix_upper_bound(prefix) {
            Some(hi) => {
                let mut stmt = self.txn.prepare_cached(
                    "SELECT key, value FROM kv WHERE ns = ?1 AND key >= ?2 AND key < ?3 ORDER BY key",
                )?;
                let mapped = stmt.query_map(params![ns, prefix, hi], |row| {
                    Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, Vec<u8>>(1)?))
                })?;
                mapped.collect::<Result<_, _>>()?
            }
            None => {
                let mut stmt = self.txn.prepare_cached(
                    "SELECT key, value FROM kv WHERE ns = ?1 AND key >= ?2 ORDER BY key",
                )?;
                let mapped = stmt.query_map(params![ns, prefix], |row| {
                    Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, Vec<u8>>(1)?))
                })?;
                mapped.collect::<Result<_, _>>()?
            }
        };
        for (key, stored) in rows {
            // The range bound already guarantees the prefix; keep the check
            // so a bound bug can never leak a foreign key.
            if !key.starts_with(prefix) {
                continue;
            }
            let value = self.decode_value(ns, &key, &stored)?;
            out.push((key, value));
        }
        Ok(out)
    }

    /// Reads a `meta` value inside this transaction.
    pub fn meta_get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        meta_get_in(&self.txn, key)
    }

    /// Writes a `meta` value as part of this transaction (for example
    /// `powered_on_ms` alongside the state it timestamps).
    pub fn meta_set(&mut self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        meta_set_in(&self.txn, key, value)
    }

    /// Removes a `meta` entry as part of this transaction.
    pub fn meta_delete(&mut self, key: &str) -> Result<(), StorageError> {
        meta_delete_in(&self.txn, key)
    }

    /// Reads a `u64` counter from `meta`.
    pub fn meta_get_u64(&self, key: &str) -> Result<Option<u64>, StorageError> {
        meta_get_u64_in(&self.txn, key)
    }

    /// Writes a `u64` counter to `meta`.
    pub fn meta_set_u64(&mut self, key: &str, value: u64) -> Result<(), StorageError> {
        meta_set_in(&self.txn, key, &value.to_le_bytes())
    }

    fn ns_key(&self, ns: &str) -> Option<&Key32> {
        self.ns_keys.get(ns)
    }

    /// Plaintext for public namespaces; `nonce ‖ seal_x(...)` for secret ones.
    fn encode_value(&mut self, ns: &str, key: &[u8], value: &[u8]) -> Vec<u8> {
        let Some(k) = self.ns_keys.get(ns) else {
            return value.to_vec();
        };
        let mut nonce = [0u8; XNONCE_LEN];
        self.rng.fill_bytes(&mut nonce);
        let aad = secret_aad(ns, key);
        let ct = seal_x(k, &nonce, &aad, value);
        let mut out = Vec::with_capacity(XNONCE_LEN + ct.len());
        out.extend_from_slice(&nonce);
        out.extend_from_slice(&ct);
        out
    }

    fn decode_value(&self, ns: &str, key: &[u8], stored: &[u8]) -> Result<Vec<u8>, StorageError> {
        match self.ns_key(ns) {
            None => Ok(stored.to_vec()),
            Some(k) => decode_secret_value(k, ns, key, stored),
        }
    }
}

/// `aad = ns ‖ 0x00 ‖ key`: binds a blob to its row.
pub(crate) fn secret_aad(ns: &str, key: &[u8]) -> Vec<u8> {
    let mut aad = Vec::with_capacity(ns.len() + 1 + key.len());
    aad.extend_from_slice(ns.as_bytes());
    aad.push(0);
    aad.extend_from_slice(key);
    aad
}

/// Decodes `nonce ‖ ciphertext` under `k_ns`. Too short is
/// [`StorageError::Corrupt`]; an authentication failure (wrong key, tampered
/// row) is [`StorageError::Decrypt`]. Never panics on any input.
pub(crate) fn decode_secret_value(
    k_ns: &Key32,
    ns: &str,
    key: &[u8],
    stored: &[u8],
) -> Result<Vec<u8>, StorageError> {
    if stored.len() < SECRET_OVERHEAD {
        return Err(StorageError::Corrupt {
            what: format!(
                "secret blob of {} bytes in namespace {ns:?} (minimum {SECRET_OVERHEAD})",
                stored.len()
            ),
        });
    }
    let (nonce_bytes, ct) = stored.split_at(XNONCE_LEN);
    let nonce: [u8; XNONCE_LEN] = nonce_bytes.try_into().map_err(|_| StorageError::Corrupt {
        what: "nonce width".to_owned(),
    })?;
    let aad = secret_aad(ns, key);
    open_x(k_ns, &nonce, &aad, ct).map_err(|_| StorageError::Decrypt { ns: ns.to_owned() })
}

/// Smallest byte string greater than every string with `prefix`, or `None`
/// when no such bound exists (empty prefix or all `0xFF`).
pub(crate) fn prefix_upper_bound(prefix: &[u8]) -> Option<Vec<u8>> {
    let mut hi = prefix.to_vec();
    while let Some(last) = hi.pop() {
        if last < 0xFF {
            hi.push(last.wrapping_add(1));
            return Some(hi);
        }
    }
    None
}

impl KvTxn for Txn<'_> {
    fn get(&self, ns: &str, key: &[u8]) -> Option<Vec<u8>> {
        match self.try_get(ns, key) {
            Ok(v) => v,
            Err(e) => {
                self.record(e);
                None
            }
        }
    }

    fn put(&mut self, ns: &str, key: &[u8], value: &[u8]) {
        if let Err(e) = self.try_put(ns, key, value) {
            self.record(e);
        }
    }

    fn delete(&mut self, ns: &str, key: &[u8]) {
        if let Err(e) = self.try_delete(ns, key) {
            self.record(e);
        }
    }

    fn scan_prefix(&self, ns: &str, prefix: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
        match self.try_scan_prefix(ns, prefix) {
            Ok(v) => v,
            Err(e) => {
                self.record(e);
                Vec::new()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::db::{Db, derive_ns_key};
    use proptest::prelude::*;

    fn db() -> Db {
        Db::open_in_memory([3u8; 32], [9u8; 32]).unwrap()
    }

    #[test]
    fn secret_namespace_list() {
        for n in [
            ns::IDENTITY,
            ns::SESSIONS,
            ns::PREKEYS,
            ns::SIGNED_PREKEYS,
            ns::KYBER_PREKEYS,
            ns::SENDER_KEYS,
            ns::PEER_IDENTITIES,
            "contacts",
            "chains",
            "plaintext",
            "outbox",
        ] {
            assert!(is_secret_namespace(n), "{n}");
        }
        assert!(!is_secret_namespace("cells"));
        assert!(!is_secret_namespace("dedupe"));
        assert!(!is_secret_namespace(""));
        assert_eq!(SECRET_NAMESPACES.len(), 11);
        assert_eq!(SECRET_OVERHEAD, 40);
    }

    #[test]
    fn prefix_bounds() {
        assert_eq!(prefix_upper_bound(b""), None);
        assert_eq!(prefix_upper_bound(&[0xFF]), None);
        assert_eq!(prefix_upper_bound(&[0xFF, 0xFF]), None);
        assert_eq!(prefix_upper_bound(b"ab"), Some(b"ac".to_vec()));
        assert_eq!(prefix_upper_bound(&[0x61, 0xFF]), Some(vec![0x62]));
        assert_eq!(prefix_upper_bound(&[0x00]), Some(vec![0x01]));
    }

    #[test]
    fn secret_values_are_encrypted_and_bound_to_row() {
        let mut db = db();
        let mut t = db.begin().unwrap();
        t.put(ns::SESSIONS, b"peer1", b"ratchet state");
        t.put("cells", b"peer1", b"ratchet state");
        t.commit().unwrap();

        let t = db.begin().unwrap();
        let raw_secret: Vec<u8> = t
            .txn
            .query_row(
                "SELECT value FROM kv WHERE ns = ?1 AND key = ?2",
                params![ns::SESSIONS, &b"peer1"[..]],
                |r| r.get(0),
            )
            .unwrap();
        let raw_plain: Vec<u8> = t
            .txn
            .query_row(
                "SELECT value FROM kv WHERE ns = ?1 AND key = ?2",
                params!["cells", &b"peer1"[..]],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(raw_plain, b"ratchet state");
        assert_eq!(raw_secret.len(), b"ratchet state".len() + SECRET_OVERHEAD);
        assert!(
            !raw_secret
                .windows(b"ratchet".len())
                .any(|w| w == b"ratchet")
        );
        drop(t);
        // Decodes under the right (ns, key); not under another key or ns.
        let k = derive_ns_key(db.db_key(), ns::SESSIONS);
        assert_eq!(
            decode_secret_value(&k, ns::SESSIONS, b"peer1", &raw_secret).unwrap(),
            b"ratchet state"
        );
        assert!(matches!(
            decode_secret_value(&k, ns::SESSIONS, b"peer2", &raw_secret),
            Err(StorageError::Decrypt { .. })
        ));
        assert!(matches!(
            decode_secret_value(&k, ns::PREKEYS, b"peer1", &raw_secret),
            Err(StorageError::Decrypt { .. })
        ));
        let other = derive_ns_key(&Key32::from_bytes([4u8; 32]), ns::SESSIONS);
        assert!(matches!(
            decode_secret_value(&other, ns::SESSIONS, b"peer1", &raw_secret),
            Err(StorageError::Decrypt { .. })
        ));
        assert!(matches!(
            decode_secret_value(&k, ns::SESSIONS, b"peer1", &raw_secret[..39]),
            Err(StorageError::Corrupt { .. })
        ));
    }

    #[test]
    fn each_write_draws_a_fresh_nonce() {
        let mut db = db();
        let mut t = db.begin().unwrap();
        t.put(ns::SESSIONS, b"k", b"same");
        let first: Vec<u8> = t
            .txn
            .query_row(
                "SELECT value FROM kv WHERE ns = ?1 AND key = ?2",
                params![ns::SESSIONS, &b"k"[..]],
                |r| r.get(0),
            )
            .unwrap();
        t.put(ns::SESSIONS, b"k", b"same");
        let second: Vec<u8> = t
            .txn
            .query_row(
                "SELECT value FROM kv WHERE ns = ?1 AND key = ?2",
                params![ns::SESSIONS, &b"k"[..]],
                |r| r.get(0),
            )
            .unwrap();
        assert_ne!(first, second);
        assert_ne!(first[..XNONCE_LEN], second[..XNONCE_LEN]);
        assert_eq!(t.get(ns::SESSIONS, b"k").as_deref(), Some(&b"same"[..]));
        t.commit().unwrap();
    }

    #[test]
    fn corrupt_row_poisons_the_transaction() {
        let mut db = db();
        let mut t = db.begin().unwrap();
        t.put(ns::SESSIONS, b"k", b"v");
        t.commit().unwrap();
        {
            let t = db.begin().unwrap();
            t.txn
                .execute(
                    "UPDATE kv SET value = X'00' WHERE ns = ?1",
                    params![ns::SESSIONS],
                )
                .unwrap();
            t.commit().unwrap();
        }
        let mut t = db.begin().unwrap();
        assert!(!t.is_poisoned());
        assert!(matches!(
            t.try_get(ns::SESSIONS, b"k"),
            Err(StorageError::Corrupt { .. })
        ));
        assert!(!t.is_poisoned(), "try_* does not poison");
        assert_eq!(t.get(ns::SESSIONS, b"k"), None);
        assert!(t.is_poisoned());
        assert!(format!("{t:?}").contains("poisoned: true"));
        t.put("cells", b"x", b"y");
        assert!(matches!(t.commit(), Err(StorageError::Corrupt { .. })));
        // The poisoned transaction wrote nothing.
        let t = db.begin().unwrap();
        assert_eq!(t.get("cells", b"x"), None);
        assert!(t.scan_prefix(ns::SESSIONS, b"").is_empty());
        assert!(t.is_poisoned());
    }

    #[test]
    fn scan_prefix_orders_keys_and_respects_prefix_edges() {
        let mut db = db();
        let mut t = db.begin().unwrap();
        for (k, v) in [
            (&[0x61u8, 0xFF][..], &b"aff"[..]),
            (b"a", b"a"),
            (b"ab", b"ab"),
            (b"b", b"b"),
            (&[0xFF, 0x01], b"ff01"),
            (&[0xFF], b"ff"),
        ] {
            t.put(ns::PREKEYS, k, v);
            t.put("other", k, v);
        }
        t.commit().unwrap();
        let t = db.begin().unwrap();
        let all = t.scan_prefix(ns::PREKEYS, b"");
        let keys: Vec<&[u8]> = all.iter().map(|(k, _)| k.as_slice()).collect();
        assert_eq!(
            keys,
            vec![
                &b"a"[..],
                b"ab",
                &[0x61, 0xFF],
                b"b",
                &[0xFF],
                &[0xFF, 0x01]
            ]
        );
        let a = t.scan_prefix(ns::PREKEYS, b"a");
        assert_eq!(a.len(), 3);
        assert_eq!(a[2].1, b"aff");
        let ff = t.scan_prefix(ns::PREKEYS, &[0xFF]);
        assert_eq!(ff.len(), 2);
        assert_eq!(t.scan_prefix(ns::PREKEYS, b"zzz").len(), 0);
        assert_eq!(t.scan_prefix("missing", b"").len(), 0);
        assert_eq!(t.scan_prefix("other", b"a").len(), 3);
        assert!(!t.is_poisoned());
    }

    #[test]
    fn rollback_discards_and_meta_in_txn_is_atomic() {
        let mut db = db();
        let mut t = db.begin().unwrap();
        t.put("cells", b"c", b"1");
        t.meta_set_u64("powered_on_ms", 42).unwrap();
        t.rollback().unwrap();
        assert_eq!(db.meta_get_u64("powered_on_ms").unwrap(), None);
        let mut t = db.begin().unwrap();
        assert_eq!(t.get("cells", b"c"), None);
        t.meta_set("s", b"v").unwrap();
        assert_eq!(t.meta_get("s").unwrap().as_deref(), Some(&b"v"[..]));
        t.meta_set_u64("powered_on_ms", 42).unwrap();
        assert_eq!(t.meta_get_u64("powered_on_ms").unwrap(), Some(42));
        t.meta_delete("s").unwrap();
        t.commit().unwrap();
        assert_eq!(db.meta_get_u64("powered_on_ms").unwrap(), Some(42));
        assert_eq!(db.meta_get("s").unwrap(), None);
    }

    #[test]
    fn empty_value_and_empty_key_round_trip() {
        let mut db = db();
        let mut t = db.begin().unwrap();
        t.put(ns::IDENTITY, b"", b"");
        t.put("cells", b"", b"");
        t.commit().unwrap();
        let t = db.begin().unwrap();
        assert_eq!(t.get(ns::IDENTITY, b"").as_deref(), Some(&b""[..]));
        assert_eq!(t.get("cells", b"").as_deref(), Some(&b""[..]));
    }

    proptest! {
        #[test]
        fn decode_secret_value_never_panics(
            stored in proptest::collection::vec(any::<u8>(), 0..96),
            key in proptest::collection::vec(any::<u8>(), 0..8),
        ) {
            let k = Key32::from_bytes([5u8; 32]);
            match decode_secret_value(&k, ns::SESSIONS, &key, &stored) {
                Err(StorageError::Corrupt { .. }) => prop_assert!(stored.len() < SECRET_OVERHEAD),
                Err(StorageError::Decrypt { .. }) => prop_assert!(stored.len() >= SECRET_OVERHEAD),
                Ok(_) => prop_assert!(false, "random bytes authenticated"),
                Err(e) => prop_assert!(false, "unexpected {e:?}"),
            }
        }

        #[test]
        fn secret_round_trip_any_bytes(
            key in proptest::collection::vec(any::<u8>(), 0..16),
            value in proptest::collection::vec(any::<u8>(), 0..200),
        ) {
            let mut db = Db::open_in_memory([3u8; 32], [9u8; 32]).unwrap();
            let mut t = db.begin().unwrap();
            t.put(ns::SENDER_KEYS, &key, &value);
            prop_assert_eq!(t.get(ns::SENDER_KEYS, &key), Some(value.clone()));
            t.commit().unwrap();
            let t = db.begin().unwrap();
            prop_assert_eq!(t.get(ns::SENDER_KEYS, &key), Some(value.clone()));
            let scan = t.scan_prefix(ns::SENDER_KEYS, &key);
            prop_assert_eq!(scan, vec![(key.clone(), value)]);
        }

        #[test]
        fn prefix_upper_bound_is_tight(prefix in proptest::collection::vec(any::<u8>(), 0..6)) {
            match prefix_upper_bound(&prefix) {
                None => prop_assert!(prefix.iter().all(|b| *b == 0xFF)),
                Some(hi) => {
                    prop_assert!(hi.as_slice() > prefix.as_slice());
                    // Every extension of the prefix sorts below `hi`.
                    let mut ext = prefix.clone();
                    ext.extend_from_slice(&[0xFF; 4]);
                    prop_assert!(ext.as_slice() < hi.as_slice());
                    // Nothing with a different prefix lands inside [prefix, hi).
                    prop_assert!(!hi.starts_with(&prefix));
                }
            }
        }
    }
}
