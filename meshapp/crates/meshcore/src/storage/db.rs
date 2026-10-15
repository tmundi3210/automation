//! SQLite database handle (docs/DESIGN.md §1 row `storage/db.rs`; §6
//! "Storage"; BRIEF_v2 §5.3 "State is committed atomically before
//! 'Delivered'", §3 "The pin store is encrypted at rest").
//!
//! One [`Db`] owns one `rusqlite` connection, the 32-byte `db_key` the
//! platform unwraps from its keystore (DESIGN §6 "Key handling per platform"),
//! the per-namespace blob keys derived from it, and the nonce generator for
//! the encrypted namespaces. All state mutation goes through [`Db::begin`],
//! which returns a [`Txn`] that either commits as a whole or leaves nothing
//! behind (D6 rule 4 "Ratchet state and plaintext are committed in one atomic
//! transaction before any 'Delivered' ack").
//!
//! Pragmas (DESIGN §6): `journal_mode=WAL` for file databases,
//! `secure_delete=ON` so deleted message keys and plaintext are overwritten
//! rather than left in free pages, `foreign_keys=ON`, `synchronous=NORMAL`
//! (durable at the WAL checkpoint; a power loss can lose the last commits but
//! never corrupts, and a lost commit is exactly the "crash before commit"
//! case the inbound path already replays from `cells`).
//!
//! Nothing here reads the OS clock or the OS entropy pool: the platform
//! passes 32 bytes of entropy at open and all time arrives through
//! `meta` counters the node maintains (`powered_on_ms`, DESIGN §6
//! "Powered-on meter").

use core::fmt;
use std::collections::BTreeMap;
use std::path::Path;

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rusqlite::{Connection, OpenFlags, OptionalExtension, params};
use zeroize::Zeroize;

use crate::crypto::aead::Key32;
use crate::crypto::hash::hkdf32;

use super::migrations;
use super::txn::{SECRET_NAMESPACES, Txn};

/// Errors of the storage layer. Decryption failures carry no detail beyond
/// the namespace (the AEAD reports one opaque error, `crypto::aead`).
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    /// A stored blob in a secret namespace did not authenticate under the
    /// current `db_key`: wrong key, or the row was tampered with.
    #[error("cannot decrypt stored value in namespace {ns:?}")]
    Decrypt { ns: String },
    /// A stored row has an impossible shape (blob shorter than nonce + tag,
    /// counter of the wrong width, negative version).
    #[error("corrupt stored value: {what}")]
    Corrupt { what: String },
    /// The database was written by a newer schema than this build knows.
    #[error("schema version {found} is newer than the supported {supported}")]
    SchemaTooNew { found: u32, supported: u32 },
    /// A pragma did not take effect (for example WAL unavailable on the
    /// filesystem).
    #[error("pragma {pragma} reported {got:?}, expected {expected:?}")]
    Pragma {
        pragma: &'static str,
        got: String,
        expected: &'static str,
    },
    /// `commit` was aborted by the fault-injection hook
    /// (`Txn::set_fail_before_commit`); nothing was written.
    #[error("commit aborted by fault injection")]
    FaultInjected,
}

/// Keys of the `meta` table the core maintains (DESIGN §6
/// `meta(powered_on_ms, last_rotation, skeleton_data_version)`).
pub mod meta_keys {
    /// Accumulated powered-on milliseconds (DESIGN §6 "Powered-on meter";
    /// D6 rule 3 "Persist the powered-on time from elapsedRealtime deltas").
    pub const POWERED_ON_MS: &str = "powered_on_ms";
    /// Powered-on time of the last signed-prekey rotation (D6 rule 3).
    pub const LAST_ROTATION_MONO: &str = "last_rotation_mono";
    /// Version of the bundled skeleton data.
    pub const SKELETON_DATA_VERSION: &str = "skeleton_data_version";
    /// Number of times this database has been opened; mixed into the nonce
    /// generator seed so two opens with the same platform entropy never
    /// replay a nonce stream (see [`super::Db::open`]).
    pub const OPEN_COUNT: &str = "open_count";
}

/// HKDF info prefix of the per-namespace blob keys:
/// `k_ns = HKDF-SHA256(ikm = db_key, salt = "", info = "meshapp.v1.kv." ‖ ns)`.
pub const NS_KEY_INFO_PREFIX: &[u8] = b"meshapp.v1.kv.";

/// HKDF info of the nonce-generator seed.
const RNG_SEED_INFO: &[u8] = b"meshapp.v1.storage.nonce-rng";

/// An open database.
pub struct Db {
    conn: Connection,
    db_key: Key32,
    /// One derived key per secret namespace, in namespace order.
    ns_keys: BTreeMap<&'static str, Key32>,
    /// Nonce source for the encrypted namespaces, seeded at open.
    rng: ChaCha20Rng,
    /// Every `Txn` started from this handle begins with this flag
    /// (fault injection for the simulator; see [`Db::set_fail_before_commit`]).
    fail_before_commit: bool,
    schema_version: u32,
}

impl fmt::Debug for Db {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Db")
            .field("db_key", &self.db_key)
            .field("secret_namespaces", &self.ns_keys.len())
            .field("schema_version", &self.schema_version)
            .field("fail_before_commit", &self.fail_before_commit)
            .finish_non_exhaustive()
    }
}

impl Db {
    /// Opens (creating if needed) the database file at `path`.
    ///
    /// * `db_key`: the 32-byte application key the platform keeps in its
    ///   keystore (DESIGN §6). The copy is wiped before this returns; the
    ///   handle keeps a zeroizing [`Key32`].
    /// * `entropy`: 32 bytes from the OS entropy pool, supplied by the
    ///   platform (the core never reads the OS itself). It seeds the nonce
    ///   generator for the encrypted namespaces, mixed with `db_key` and an
    ///   `open_count` counter kept in `meta`, so even a platform that passes
    ///   the same bytes twice gets a fresh nonce stream per open. Tests pass
    ///   a fixed value.
    pub fn open(path: &Path, db_key: [u8; 32], entropy: [u8; 32]) -> Result<Db, StorageError> {
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        let conn = Connection::open_with_flags(path, flags)?;
        Self::init(conn, db_key, entropy, true)
    }

    /// Opens a private in-memory database (tests, simulator). Same semantics
    /// as [`Db::open`] except that `journal_mode` stays `memory`.
    pub fn open_in_memory(db_key: [u8; 32], entropy: [u8; 32]) -> Result<Db, StorageError> {
        let conn = Connection::open_in_memory()?;
        Self::init(conn, db_key, entropy, false)
    }

    fn init(
        mut conn: Connection,
        mut db_key: [u8; 32],
        mut entropy: [u8; 32],
        file_backed: bool,
    ) -> Result<Db, StorageError> {
        let key = Key32::from_bytes(db_key);
        db_key.zeroize();

        if file_backed {
            // journal_mode returns the mode actually in effect.
            let mode: String = conn.query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))?;
            if !mode.eq_ignore_ascii_case("wal") {
                return Err(StorageError::Pragma {
                    pragma: "journal_mode",
                    got: mode,
                    expected: "wal",
                });
            }
        }
        conn.execute_batch(
            "PRAGMA secure_delete = ON;\n\
             PRAGMA foreign_keys = ON;\n\
             PRAGMA synchronous = NORMAL;",
        )?;
        Self::check_pragma_on(&conn, "secure_delete")?;
        Self::check_pragma_on(&conn, "foreign_keys")?;

        let schema_version = migrations::run(&mut conn)?;

        // Bump the open counter (its own small transaction) and derive the
        // nonce-generator seed from entropy ‖ open_count under db_key.
        let open_count = {
            let txn = conn.transaction()?;
            let prev = meta_get_u64_in(&txn, meta_keys::OPEN_COUNT)?.unwrap_or(0);
            let next = prev.checked_add(1).ok_or_else(|| StorageError::Corrupt {
                what: "open_count overflow".to_owned(),
            })?;
            meta_set_in(&txn, meta_keys::OPEN_COUNT, &next.to_le_bytes())?;
            txn.commit()?;
            next
        };
        let mut info = Vec::with_capacity(RNG_SEED_INFO.len() + 8);
        info.extend_from_slice(RNG_SEED_INFO);
        info.extend_from_slice(&open_count.to_le_bytes());
        let mut seed = hkdf32(&entropy, key.as_bytes(), &info);
        entropy.zeroize();
        let rng = ChaCha20Rng::from_seed(seed);
        seed.zeroize();

        let ns_keys = SECRET_NAMESPACES
            .iter()
            .map(|ns| (*ns, derive_ns_key(&key, ns)))
            .collect();

        Ok(Db {
            conn,
            db_key: key,
            ns_keys,
            rng,
            fail_before_commit: false,
            schema_version,
        })
    }

    fn check_pragma_on(conn: &Connection, pragma: &'static str) -> Result<(), StorageError> {
        let v: i64 = conn.query_row(&format!("PRAGMA {pragma}"), [], |row| row.get(0))?;
        if v == 1 {
            Ok(())
        } else {
            Err(StorageError::Pragma {
                pragma,
                got: v.to_string(),
                expected: "1",
            })
        }
    }

    /// The application key this handle encrypts secret namespaces under.
    pub fn db_key(&self) -> &Key32 {
        &self.db_key
    }

    /// Schema version after migrations (`migrations::LATEST_VERSION`).
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Starts a transaction. Every state mutation goes through the returned
    /// [`Txn`]; dropping it without [`Txn::commit`] rolls back (D6 rule 4).
    pub fn begin(&mut self) -> Result<Txn<'_>, StorageError> {
        let txn = self.conn.transaction()?;
        Ok(Txn::new(
            txn,
            &self.ns_keys,
            &mut self.rng,
            self.fail_before_commit,
        ))
    }

    /// Fault injection (DESIGN §10 "storage atomicity (kill between decrypt
    /// and commit via a fault-injection hook → replay yields exactly one
    /// delivery)"): while set, every transaction started by [`Db::begin`]
    /// fails its `commit` with [`StorageError::FaultInjected`] after writing
    /// nothing durable. Individual transactions can override it with
    /// [`Txn::set_fail_before_commit`].
    pub fn set_fail_before_commit(&mut self, fail: bool) {
        self.fail_before_commit = fail;
    }

    /// Current fault-injection setting.
    pub fn fail_before_commit(&self) -> bool {
        self.fail_before_commit
    }

    /// Reads a small value from `meta` (autocommit read).
    pub fn meta_get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        meta_get_in(&self.conn, key)
    }

    /// Writes a small value to `meta` in its own transaction. Use
    /// [`Txn::meta_set`] when the value must land together with other writes.
    pub fn meta_set(&mut self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        meta_set_in(&self.conn, key, value)
    }

    /// Removes a `meta` entry.
    pub fn meta_delete(&mut self, key: &str) -> Result<(), StorageError> {
        meta_delete_in(&self.conn, key)
    }

    /// Reads a `u64` counter stored by [`Db::meta_set_u64`] (little-endian,
    /// 8 bytes; any other width is [`StorageError::Corrupt`]).
    pub fn meta_get_u64(&self, key: &str) -> Result<Option<u64>, StorageError> {
        meta_get_u64_in(&self.conn, key)
    }

    /// Stores a `u64` counter (for example `powered_on_ms`).
    pub fn meta_set_u64(&mut self, key: &str, value: u64) -> Result<(), StorageError> {
        meta_set_in(&self.conn, key, &value.to_le_bytes())
    }

    /// Number of rows in `kv` across all namespaces (diagnostics, tests).
    pub fn kv_len(&self) -> Result<u64, StorageError> {
        let n: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM kv", [], |row| row.get(0))?;
        u64::try_from(n).map_err(|_| StorageError::Corrupt {
            what: "negative row count".to_owned(),
        })
    }
}

/// `k_ns = HKDF-SHA256(db_key, salt = "", info = "meshapp.v1.kv." ‖ ns)`.
pub(crate) fn derive_ns_key(db_key: &Key32, ns: &str) -> Key32 {
    let mut info = Vec::with_capacity(NS_KEY_INFO_PREFIX.len() + ns.len());
    info.extend_from_slice(NS_KEY_INFO_PREFIX);
    info.extend_from_slice(ns.as_bytes());
    let mut okm = hkdf32(db_key.as_bytes(), b"", &info);
    let k = Key32::from_bytes(okm);
    okm.zeroize();
    k
}

/// Decodes a `meta` counter: exactly 8 little-endian bytes.
pub(crate) fn decode_u64(bytes: &[u8]) -> Result<u64, StorageError> {
    <[u8; 8]>::try_from(bytes)
        .map(u64::from_le_bytes)
        .map_err(|_| StorageError::Corrupt {
            what: format!("meta counter of {} bytes", bytes.len()),
        })
}

pub(crate) fn meta_get_in(conn: &Connection, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
    let mut stmt = conn.prepare_cached("SELECT value FROM meta WHERE key = ?1")?;
    let v: Option<Option<Vec<u8>>> = stmt.query_row(params![key], |row| row.get(0)).optional()?;
    // A NULL value is reported as an empty blob rather than absence.
    Ok(v.map(Option::unwrap_or_default))
}

pub(crate) fn meta_set_in(conn: &Connection, key: &str, value: &[u8]) -> Result<(), StorageError> {
    let mut stmt =
        conn.prepare_cached("INSERT OR REPLACE INTO meta (key, value) VALUES (?1, ?2)")?;
    stmt.execute(params![key, value])?;
    Ok(())
}

pub(crate) fn meta_delete_in(conn: &Connection, key: &str) -> Result<(), StorageError> {
    let mut stmt = conn.prepare_cached("DELETE FROM meta WHERE key = ?1")?;
    stmt.execute(params![key])?;
    Ok(())
}

pub(crate) fn meta_get_u64_in(conn: &Connection, key: &str) -> Result<Option<u64>, StorageError> {
    meta_get_in(conn, key)?.map(|v| decode_u64(&v)).transpose()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::kv::KvTxn;
    use proptest::prelude::*;

    fn db() -> Db {
        Db::open_in_memory([7u8; 32], [1u8; 32]).unwrap()
    }

    #[test]
    fn open_runs_migrations_and_sets_pragmas() {
        let db = db();
        assert_eq!(db.schema_version(), migrations::LATEST_VERSION);
        assert_eq!(migrations::current_version(&db.conn).unwrap(), 1);
        let sd: i64 = db
            .conn
            .query_row("PRAGMA secure_delete", [], |r| r.get(0))
            .unwrap();
        assert_eq!(sd, 1);
        let fk: i64 = db
            .conn
            .query_row("PRAGMA foreign_keys", [], |r| r.get(0))
            .unwrap();
        assert_eq!(fk, 1);
        let sync: i64 = db
            .conn
            .query_row("PRAGMA synchronous", [], |r| r.get(0))
            .unwrap();
        assert_eq!(sync, 1, "NORMAL");
        assert_eq!(db.meta_get_u64(meta_keys::OPEN_COUNT).unwrap(), Some(1));
        assert_eq!(db.kv_len().unwrap(), 0);
        assert!(!db.fail_before_commit());
    }

    #[test]
    fn db_key_is_kept_and_redacted() {
        let db = db();
        assert_eq!(db.db_key(), &Key32::from_bytes([7u8; 32]));
        let dbg = format!("{db:?}");
        assert!(dbg.contains("<redacted>"));
        assert!(!dbg.contains("07"));
    }

    #[test]
    fn ns_keys_are_distinct_per_namespace_and_key() {
        let a = Key32::from_bytes([1u8; 32]);
        let b = Key32::from_bytes([2u8; 32]);
        assert_eq!(derive_ns_key(&a, "sessions"), derive_ns_key(&a, "sessions"));
        assert_ne!(derive_ns_key(&a, "sessions"), derive_ns_key(&a, "prekeys"));
        assert_ne!(derive_ns_key(&a, "sessions"), derive_ns_key(&b, "sessions"));
        assert_ne!(derive_ns_key(&a, "sessions"), a);
        // Layout check against the documented formula.
        let expected = hkdf32(a.as_bytes(), b"", b"meshapp.v1.kv.sessions");
        assert_eq!(derive_ns_key(&a, "sessions").as_bytes(), &expected);
    }

    #[test]
    fn meta_round_trip_and_counters() {
        let mut db = db();
        assert_eq!(db.meta_get("x").unwrap(), None);
        db.meta_set("x", b"hello").unwrap();
        assert_eq!(db.meta_get("x").unwrap().as_deref(), Some(&b"hello"[..]));
        db.meta_set("x", b"").unwrap();
        assert_eq!(db.meta_get("x").unwrap().as_deref(), Some(&b""[..]));
        db.meta_delete("x").unwrap();
        assert_eq!(db.meta_get("x").unwrap(), None);

        db.meta_set_u64(meta_keys::POWERED_ON_MS, 123_456).unwrap();
        assert_eq!(
            db.meta_get_u64(meta_keys::POWERED_ON_MS).unwrap(),
            Some(123_456)
        );
        db.meta_set("short", b"abc").unwrap();
        assert!(matches!(
            db.meta_get_u64("short"),
            Err(StorageError::Corrupt { .. })
        ));
        // NULL values read back as empty.
        db.conn
            .execute("INSERT INTO meta (key, value) VALUES ('n', NULL)", [])
            .unwrap();
        assert_eq!(db.meta_get("n").unwrap().as_deref(), Some(&b""[..]));
    }

    #[test]
    fn nonce_streams_differ_per_entropy_and_per_open() {
        use rand::RngCore;
        let mut a = Db::open_in_memory([7u8; 32], [1u8; 32]).unwrap();
        let mut b = Db::open_in_memory([7u8; 32], [2u8; 32]).unwrap();
        let mut c = Db::open_in_memory([7u8; 32], [1u8; 32]).unwrap();
        let (mut na, mut nb, mut nc) = ([0u8; 24], [0u8; 24], [0u8; 24]);
        a.rng.fill_bytes(&mut na);
        b.rng.fill_bytes(&mut nb);
        c.rng.fill_bytes(&mut nc);
        assert_ne!(na, nb);
        assert_eq!(na, nc, "same key, entropy and open count => same stream");

        // A real second open of the same file with the same entropy must
        // start a different stream (open_count is part of the seed). No
        // clock: the file is named by process id.
        let path = std::env::temp_dir().join(format!(
            "meshcore-db-unit-{}-nonce-per-open.db",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let mut first = [0u8; 24];
        Db::open(&path, [7u8; 32], [1u8; 32])
            .unwrap()
            .rng
            .fill_bytes(&mut first);
        assert_eq!(
            first, na,
            "first open of a file matches a fresh in-memory db"
        );
        let mut second = [0u8; 24];
        let reopened = Db::open(&path, [7u8; 32], [1u8; 32]).unwrap();
        assert_eq!(
            reopened.meta_get_u64(meta_keys::OPEN_COUNT).unwrap(),
            Some(2)
        );
        let mut reopened = reopened;
        reopened.rng.fill_bytes(&mut second);
        drop(reopened);
        for suffix in ["", "-wal", "-shm"] {
            let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
        }
        assert_ne!(second, first, "second open replayed the nonce stream");
    }

    #[test]
    fn fail_flag_propagates_to_txn() {
        let mut db = db();
        db.set_fail_before_commit(true);
        assert!(db.fail_before_commit());
        let mut t = db.begin().unwrap();
        t.put("plain", b"k", b"v");
        assert!(matches!(t.commit(), Err(StorageError::FaultInjected)));
        assert_eq!(db.kv_len().unwrap(), 0);
        db.set_fail_before_commit(false);
        let mut t = db.begin().unwrap();
        t.put("plain", b"k", b"v");
        t.commit().unwrap();
        assert_eq!(db.kv_len().unwrap(), 1);
    }

    #[test]
    fn error_messages() {
        assert_eq!(
            StorageError::FaultInjected.to_string(),
            "commit aborted by fault injection"
        );
        assert_eq!(
            StorageError::SchemaTooNew {
                found: 3,
                supported: 1
            }
            .to_string(),
            "schema version 3 is newer than the supported 1"
        );
        assert!(
            StorageError::Decrypt {
                ns: "sessions".into()
            }
            .to_string()
            .contains("sessions")
        );
    }

    proptest! {
        #[test]
        fn decode_u64_never_panics(bytes in proptest::collection::vec(any::<u8>(), 0..20)) {
            match decode_u64(&bytes) {
                Ok(v) => {
                    prop_assert_eq!(bytes.len(), 8);
                    prop_assert_eq!(v.to_le_bytes().to_vec(), bytes);
                }
                Err(StorageError::Corrupt { .. }) => prop_assert_ne!(bytes.len(), 8),
                Err(e) => prop_assert!(false, "unexpected {e:?}"),
            }
        }
    }
}
