//! Integration tests for `meshcore::storage` (docs/DESIGN.md §6, §10
//! "storage atomicity"; BRIEF_v2 §5.3 "State is committed atomically before
//! 'Delivered'", "Message keys are deleted on decrypt", §3 "pin store
//! encrypted at rest"; D6 rule 4).
//!
//! No OS clock is read anywhere: temporary files are named by process id
//! and a counter.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use meshcore::crypto::aead::{Key32, XNONCE_LEN, open_x};
use meshcore::crypto::hash::hkdf32;
use meshcore::storage::{
    Db, KvTxn, MemKv, SECRET_NAMESPACES, SECRET_OVERHEAD, StorageError, Txn, is_secret_namespace,
    meta_keys, migrations, ns,
};

const DB_KEY: [u8; 32] = [0x42u8; 32];
const ENTROPY: [u8; 32] = [0x11u8; 32];

fn mem_db() -> Db {
    Db::open_in_memory(DB_KEY, ENTROPY).expect("open in-memory db")
}

/// A fresh directory under the system temp dir, removed on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "meshcore-storage-{}-{}-{tag}",
            std::process::id(),
            n
        ));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        TempDir(dir)
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

// ---------------------------------------------------------------------------
// (1) KvTxn conformance, run over MemKv and Txn.
// ---------------------------------------------------------------------------

fn kv_conformance<K: KvTxn>(kv: &mut K) {
    // Empty store.
    assert_eq!(kv.get(ns::SESSIONS, b"a"), None);
    assert!(kv.scan_prefix(ns::SESSIONS, b"").is_empty());

    // put / get, secret and plain namespaces, across namespaces.
    kv.put(ns::SESSIONS, b"a", b"1");
    kv.put(ns::SESSIONS, b"ab", b"2");
    kv.put(ns::SESSIONS, b"b", b"3");
    kv.put("cells", b"a", b"plain");
    assert_eq!(kv.get(ns::SESSIONS, b"a").as_deref(), Some(&b"1"[..]));
    assert_eq!(kv.get(ns::SESSIONS, b"ab").as_deref(), Some(&b"2"[..]));
    assert_eq!(kv.get("cells", b"a").as_deref(), Some(&b"plain"[..]));
    assert_eq!(kv.get("cells", b"ab"), None);

    // scan_prefix: key order, prefix respected, namespace isolated.
    let scanned = kv.scan_prefix(ns::SESSIONS, b"a");
    assert_eq!(
        scanned,
        vec![
            (b"a".to_vec(), b"1".to_vec()),
            (b"ab".to_vec(), b"2".to_vec())
        ]
    );
    let all = kv.scan_prefix(ns::SESSIONS, b"");
    assert_eq!(all.len(), 3);
    assert_eq!(all[2].0, b"b");
    assert_eq!(kv.scan_prefix("cells", b"").len(), 1);
    assert!(kv.scan_prefix("nothing", b"").is_empty());

    // Overwrite.
    kv.put(ns::SESSIONS, b"a", b"one");
    assert_eq!(kv.get(ns::SESSIONS, b"a").as_deref(), Some(&b"one"[..]));
    assert_eq!(kv.scan_prefix(ns::SESSIONS, b"").len(), 3);

    // Delete (idempotent), other keys untouched.
    kv.delete(ns::SESSIONS, b"a");
    kv.delete(ns::SESSIONS, b"a");
    assert_eq!(kv.get(ns::SESSIONS, b"a"), None);
    assert_eq!(kv.get(ns::SESSIONS, b"ab").as_deref(), Some(&b"2"[..]));
    assert_eq!(kv.scan_prefix(ns::SESSIONS, b"a").len(), 1);
    kv.delete("never", b"x");

    // Binary keys and values, including empty ones and NUL bytes.
    let bin_key = [0u8, 255, 0, 7];
    let bin_val = [255u8, 0, 1, 2, 0];
    kv.put(ns::PREKEYS, &bin_key, &bin_val);
    kv.put(ns::PREKEYS, b"", b"");
    assert_eq!(kv.get(ns::PREKEYS, &bin_key).as_deref(), Some(&bin_val[..]));
    assert_eq!(kv.get(ns::PREKEYS, b"").as_deref(), Some(&b""[..]));
    let scanned = kv.scan_prefix(ns::PREKEYS, b"");
    assert_eq!(scanned[0].0, b"");
    assert_eq!(scanned[1].0, bin_key);
}

#[test]
fn kv_conformance_memkv() {
    let mut kv = MemKv::new();
    kv_conformance(&mut kv);
}

#[test]
fn kv_conformance_sqlite_txn() {
    let mut db = mem_db();
    let mut txn = db.begin().expect("begin");
    kv_conformance(&mut txn);
    assert!(!txn.is_poisoned());
    txn.commit().expect("commit");
    // The committed state matches what the transaction saw.
    let txn = db.begin().expect("begin");
    assert_eq!(txn.get(ns::SESSIONS, b"ab").as_deref(), Some(&b"2"[..]));
    assert_eq!(txn.get(ns::SESSIONS, b"a"), None);
    assert_eq!(txn.scan_prefix(ns::PREKEYS, b"").len(), 2);
}

#[test]
fn txn_is_usable_through_the_trait_object() {
    let mut db = mem_db();
    let mut txn = db.begin().expect("begin");
    let kv: &mut dyn KvTxn = &mut txn;
    kv.put(ns::IDENTITY, b"me", b"secret");
    assert_eq!(kv.get(ns::IDENTITY, b"me").as_deref(), Some(&b"secret"[..]));
    txn.commit().expect("commit");
}

// ---------------------------------------------------------------------------
// (2) Atomicity: session blob + plaintext blob + message-key deletion.
// ---------------------------------------------------------------------------

const MSG_KEY_NS: &str = ns::SESSIONS;
const MSG_KEY: &[u8] = b"peer1/msgkey/7";

fn seed_pre_state(db: &mut Db) {
    let mut txn = db.begin().expect("begin");
    txn.put(ns::SESSIONS, b"peer1", b"ratchet v0");
    txn.put(MSG_KEY_NS, MSG_KEY, b"message key 7");
    txn.commit().expect("commit");
}

fn apply_inbound(txn: &mut Txn<'_>) {
    // DESIGN §6 "Atomic commit": backend store write + plaintext insert +
    // message-key deletion in one transaction.
    txn.put(ns::SESSIONS, b"peer1", b"ratchet v1");
    txn.put("plaintext", b"obj/1", b"hello");
    txn.delete(MSG_KEY_NS, MSG_KEY);
}

fn assert_pre_state(db: &mut Db) {
    let txn = db.begin().expect("begin");
    assert_eq!(
        txn.get(ns::SESSIONS, b"peer1").as_deref(),
        Some(&b"ratchet v0"[..])
    );
    assert_eq!(txn.get("plaintext", b"obj/1"), None);
    assert_eq!(
        txn.get(MSG_KEY_NS, MSG_KEY).as_deref(),
        Some(&b"message key 7"[..])
    );
}

fn assert_post_state(db: &mut Db) {
    let txn = db.begin().expect("begin");
    assert_eq!(
        txn.get(ns::SESSIONS, b"peer1").as_deref(),
        Some(&b"ratchet v1"[..])
    );
    assert_eq!(
        txn.get("plaintext", b"obj/1").as_deref(),
        Some(&b"hello"[..])
    );
    assert_eq!(txn.get(MSG_KEY_NS, MSG_KEY), None);
}

#[test]
fn atomicity_drop_rolls_back_commit_publishes_all_three() {
    let mut db = mem_db();
    seed_pre_state(&mut db);

    // Dropped without commit: nothing of the three writes is visible.
    {
        let mut txn = db.begin().expect("begin");
        apply_inbound(&mut txn);
        // Inside the transaction the writes are visible to itself.
        assert_eq!(txn.get(MSG_KEY_NS, MSG_KEY), None);
        assert_eq!(
            txn.get("plaintext", b"obj/1").as_deref(),
            Some(&b"hello"[..])
        );
    }
    assert_pre_state(&mut db);

    // Explicit rollback behaves the same.
    {
        let mut txn = db.begin().expect("begin");
        apply_inbound(&mut txn);
        txn.rollback().expect("rollback");
    }
    assert_pre_state(&mut db);

    // Commit: all three visible at once.
    let mut txn = db.begin().expect("begin");
    apply_inbound(&mut txn);
    txn.commit().expect("commit");
    assert_post_state(&mut db);
}

// ---------------------------------------------------------------------------
// (3) Crash replay: fail_before_commit drops the writes; the caller redoes them.
// ---------------------------------------------------------------------------

#[test]
fn crash_before_commit_leaves_pre_state_and_replay_succeeds() {
    let mut db = mem_db();
    seed_pre_state(&mut db);

    let mut txn = db.begin().expect("begin");
    apply_inbound(&mut txn);
    txn.set_fail_before_commit(true);
    assert!(matches!(txn.commit(), Err(StorageError::FaultInjected)));
    // The ratchet was not advanced on disk and the message key is still
    // there, so the cell can be replayed (DESIGN §6 "Crash between decrypt
    // and commit replays the cell from `cells`").
    assert_pre_state(&mut db);

    // The Db-level flag has the same effect for transactions the node opens
    // itself (simulator hook).
    db.set_fail_before_commit(true);
    let mut txn = db.begin().expect("begin");
    apply_inbound(&mut txn);
    assert!(matches!(txn.commit(), Err(StorageError::FaultInjected)));
    assert_pre_state(&mut db);
    db.set_fail_before_commit(false);

    // Replay: the same three writes commit exactly once.
    let mut txn = db.begin().expect("begin");
    apply_inbound(&mut txn);
    txn.commit().expect("replayed commit");
    assert_post_state(&mut db);

    // A second replay finds the message key gone: exactly one delivery.
    let txn = db.begin().expect("begin");
    assert_eq!(txn.get(MSG_KEY_NS, MSG_KEY), None);
}

// ---------------------------------------------------------------------------
// (4) Reopen a file DB and read back secret and non-secret values.
// ---------------------------------------------------------------------------

#[test]
fn file_db_reopen_reads_back_secret_and_plain_values() {
    let dir = TempDir::new("reopen");
    let path = dir.path("mesh.db");

    {
        let mut db = Db::open(&path, DB_KEY, ENTROPY).expect("open");
        assert_eq!(db.schema_version(), migrations::LATEST_VERSION);
        let mut txn = db.begin().expect("begin");
        txn.put(ns::SESSIONS, b"peer1", b"ratchet state bytes");
        txn.put("contacts", b"peer1", b"pinned key");
        txn.put("cells", b"msg1", &[0xAB; 512]);
        txn.put("dedupe", b"msg1", &7u64.to_le_bytes());
        txn.meta_set_u64(meta_keys::POWERED_ON_MS, 90_000).unwrap();
        txn.commit().expect("commit");
        db.meta_set(meta_keys::SKELETON_DATA_VERSION, b"2026-10")
            .expect("meta_set");
    }

    let mut db = Db::open(&path, DB_KEY, [0x99; 32]).expect("reopen");
    assert_eq!(db.schema_version(), migrations::LATEST_VERSION);
    let txn = db.begin().expect("begin");
    assert_eq!(
        txn.get(ns::SESSIONS, b"peer1").as_deref(),
        Some(&b"ratchet state bytes"[..])
    );
    assert_eq!(
        txn.get("contacts", b"peer1").as_deref(),
        Some(&b"pinned key"[..])
    );
    assert_eq!(txn.get("cells", b"msg1").as_deref(), Some(&[0xAB; 512][..]));
    assert_eq!(
        txn.get("dedupe", b"msg1").as_deref(),
        Some(&7u64.to_le_bytes()[..])
    );
    assert_eq!(
        txn.meta_get_u64(meta_keys::POWERED_ON_MS).unwrap(),
        Some(90_000)
    );
    assert!(!txn.is_poisoned());
    drop(txn);
    assert_eq!(
        db.meta_get(meta_keys::SKELETON_DATA_VERSION)
            .unwrap()
            .as_deref(),
        Some(&b"2026-10"[..])
    );
    // Each open bumps the counter that diversifies the nonce stream.
    assert_eq!(db.meta_get_u64(meta_keys::OPEN_COUNT).unwrap(), Some(2));

    // WAL is in effect on the file database.
    let raw = rusqlite::Connection::open(&path).expect("raw open");
    let mode: String = raw
        .query_row("PRAGMA journal_mode", [], |r| r.get(0))
        .unwrap();
    assert_eq!(mode.to_ascii_lowercase(), "wal");
}

// ---------------------------------------------------------------------------
// (5) Raw rows of secret namespaces are ciphertext; wrong key and corruption
//     are errors, never panics.
// ---------------------------------------------------------------------------

#[test]
fn secret_rows_are_ciphertext_wrong_key_and_corruption_are_errors() {
    let dir = TempDir::new("raw");
    let path = dir.path("mesh.db");
    let plaintext = b"ratchet secret: the quick brown fox";

    {
        let mut db = Db::open(&path, DB_KEY, ENTROPY).expect("open");
        let mut txn = db.begin().expect("begin");
        for secret_ns in SECRET_NAMESPACES {
            txn.put(secret_ns, b"row", plaintext);
        }
        txn.put("cells", b"row", plaintext);
        txn.commit().expect("commit");
    }

    // Inspect the raw rows with a plain connection.
    let raw = rusqlite::Connection::open(&path).expect("raw open");
    let get_raw = |namespace: &str| -> Vec<u8> {
        raw.query_row(
            "SELECT value FROM kv WHERE ns = ?1 AND key = ?2",
            rusqlite::params![namespace, &b"row"[..]],
            |r| r.get(0),
        )
        .expect("row exists")
    };
    let mut ciphertexts = Vec::new();
    for secret_ns in SECRET_NAMESPACES {
        assert!(is_secret_namespace(secret_ns));
        let stored = get_raw(secret_ns);
        assert_eq!(
            stored.len(),
            plaintext.len() + SECRET_OVERHEAD,
            "{secret_ns}"
        );
        assert_eq!(SECRET_OVERHEAD, 24 + 16);
        for window in plaintext.windows(8) {
            assert!(
                !stored.windows(8).any(|w| w == window),
                "{secret_ns}: plaintext fragment found in raw row"
            );
        }
        ciphertexts.push(stored);
    }
    // Same plaintext, different namespace => different ciphertext (per-ns
    // key and nonce) and no shared nonce.
    for (i, a) in ciphertexts.iter().enumerate() {
        for b in ciphertexts.iter().skip(i + 1) {
            assert_ne!(a, b);
            assert_ne!(a[..24], b[..24]);
        }
    }
    assert!(!is_secret_namespace("cells"));
    assert_eq!(get_raw("cells"), plaintext);

    // Wrong db_key: every secret read is Err, plain reads still work, the
    // trait view returns None and poisons the transaction, commit fails.
    {
        let mut wrong = Db::open(&path, [0x43u8; 32], ENTROPY).expect("open with wrong key");
        let mut txn = wrong.begin().expect("begin");
        assert!(matches!(
            txn.try_get(ns::SESSIONS, b"row"),
            Err(StorageError::Decrypt { .. })
        ));
        assert!(matches!(
            txn.try_scan_prefix("plaintext", b""),
            Err(StorageError::Decrypt { .. })
        ));
        assert_eq!(
            txn.try_get("cells", b"row").unwrap().as_deref(),
            Some(&plaintext[..])
        );
        assert!(!txn.is_poisoned());
        assert_eq!(txn.get(ns::SESSIONS, b"row"), None);
        assert!(txn.is_poisoned());
        txn.put("cells", b"new", b"x");
        assert!(matches!(txn.commit(), Err(StorageError::Decrypt { .. })));
        let txn = wrong.begin().expect("begin");
        assert_eq!(txn.get("cells", b"new"), None, "poisoned txn wrote nothing");
        drop(txn);
        assert_ne!(wrong.db_key(), &Key32::from_bytes(DB_KEY));
    }

    // Corruption: flip a ciphertext byte, flip a nonce byte, truncate below
    // nonce + tag, replace with garbage, move a blob to another key.
    let set_raw = |namespace: &str, key: &[u8], value: &[u8]| {
        raw.execute(
            "INSERT OR REPLACE INTO kv (ns, key, value) VALUES (?1, ?2, ?3)",
            rusqlite::params![namespace, key, value],
        )
        .expect("raw write");
    };
    let good = get_raw(ns::SESSIONS);
    let mut flipped_ct = good.clone();
    flipped_ct[30] ^= 0x01;
    set_raw(ns::SESSIONS, b"flipped_ct", &flipped_ct);
    let mut flipped_nonce = good.clone();
    flipped_nonce[0] ^= 0x80;
    set_raw(ns::SESSIONS, b"flipped_nonce", &flipped_nonce);
    set_raw(ns::SESSIONS, b"short", &good[..SECRET_OVERHEAD - 1]);
    set_raw(ns::SESSIONS, b"empty", b"");
    set_raw(ns::SESSIONS, b"garbage", &[0x5A; 64]);
    set_raw(ns::SESSIONS, b"moved", &good);
    drop(raw);

    let mut db = Db::open(&path, DB_KEY, ENTROPY).expect("open");
    let txn = db.begin().expect("begin");
    assert_eq!(
        txn.try_get(ns::SESSIONS, b"row").unwrap().as_deref(),
        Some(&plaintext[..])
    );
    for key in [&b"flipped_ct"[..], b"flipped_nonce", b"garbage", b"moved"] {
        assert!(
            matches!(
                txn.try_get(ns::SESSIONS, key),
                Err(StorageError::Decrypt { .. })
            ),
            "{}",
            String::from_utf8_lossy(key)
        );
    }
    for key in [&b"short"[..], b"empty"] {
        assert!(
            matches!(
                txn.try_get(ns::SESSIONS, key),
                Err(StorageError::Corrupt { .. })
            ),
            "{}",
            String::from_utf8_lossy(key)
        );
    }
    // A scan over a namespace with a bad row reports the error rather than
    // silently skipping or panicking.
    assert!(txn.try_scan_prefix(ns::SESSIONS, b"").is_err());
    assert!(txn.scan_prefix(ns::SESSIONS, b"").is_empty());
    assert!(txn.is_poisoned());
    assert!(txn.commit().is_err());
}

// ---------------------------------------------------------------------------
// (5b) Blob layout, checked independently of the code that produces it:
//      value = nonce(24) ‖ XChaCha20-Poly1305(k_ns, nonce, aad, plaintext),
//      k_ns = HKDF-SHA256(db_key, salt = "", info = "meshapp.v1.kv." ‖ ns),
//      aad = ns ‖ 0x00 ‖ key (DESIGN §6; storage/txn.rs module docs).
// ---------------------------------------------------------------------------

#[test]
fn secret_blob_layout_matches_the_documented_formula() {
    let dir = TempDir::new("layout");
    let path = dir.path("mesh.db");
    let plaintext = b"ratchet state under test";
    let key = b"peer/7";
    {
        let mut db = Db::open(&path, DB_KEY, ENTROPY).expect("open");
        let mut txn = db.begin().expect("begin");
        txn.put(ns::SESSIONS, key, plaintext);
        txn.commit().expect("commit");
    }
    let raw = rusqlite::Connection::open(&path).expect("raw open");
    let stored: Vec<u8> = raw
        .query_row(
            "SELECT value FROM kv WHERE ns = ?1 AND key = ?2",
            rusqlite::params![ns::SESSIONS, &key[..]],
            |r| r.get(0),
        )
        .expect("row exists");
    assert_eq!(stored.len(), plaintext.len() + SECRET_OVERHEAD);
    let (nonce, ct) = stored.split_at(XNONCE_LEN);
    let nonce: [u8; XNONCE_LEN] = nonce.try_into().unwrap();

    // Recompute every input from the documented formula, not from the crate.
    let mut info = b"meshapp.v1.kv.".to_vec();
    info.extend_from_slice(ns::SESSIONS.as_bytes());
    let k_ns = Key32::from_bytes(hkdf32(&DB_KEY, b"", &info));
    let mut aad = ns::SESSIONS.as_bytes().to_vec();
    aad.push(0x00);
    aad.extend_from_slice(key);
    assert_eq!(
        open_x(&k_ns, &nonce, &aad, ct).expect("documented layout decrypts"),
        plaintext
    );

    // Each deviation from the formula must fail to authenticate.
    let mut aad_no_separator = ns::SESSIONS.as_bytes().to_vec();
    aad_no_separator.extend_from_slice(key);
    assert!(open_x(&k_ns, &nonce, &aad_no_separator, ct).is_err());
    let mut aad_key_only = key.to_vec();
    aad_key_only.push(0x00);
    assert!(open_x(&k_ns, &nonce, &aad_key_only, ct).is_err());
    assert!(open_x(&k_ns, &nonce, b"", ct).is_err());
    let k_bare = Key32::from_bytes(DB_KEY);
    assert!(open_x(&k_bare, &nonce, &aad, ct).is_err());
    let k_other_info = Key32::from_bytes(hkdf32(&DB_KEY, b"", ns::SESSIONS.as_bytes()));
    assert!(open_x(&k_other_info, &nonce, &aad, ct).is_err());
    let k_salted = Key32::from_bytes(hkdf32(&DB_KEY, &info, b""));
    assert!(open_x(&k_salted, &nonce, &aad, ct).is_err());
}

// ---------------------------------------------------------------------------
// (5c) A second open of the same file with the same platform entropy must
//      not replay the nonce stream (db.rs `Db::open`: seed mixes
//      `meta.open_count`), or two writes of different plaintexts under the
//      same (ns, key) would share a (key, nonce) pair.
// ---------------------------------------------------------------------------

#[test]
fn reopen_with_same_entropy_draws_a_different_nonce_stream() {
    let dir = TempDir::new("reopen-nonce");
    let path = dir.path("mesh.db");
    let first_nonce = |path: &std::path::Path| -> Vec<u8> {
        let raw = rusqlite::Connection::open(path).expect("raw open");
        let stored: Vec<u8> = raw
            .query_row(
                "SELECT value FROM kv WHERE ns = ?1 AND key = ?2",
                rusqlite::params![ns::SESSIONS, &b"k"[..]],
                |r| r.get(0),
            )
            .expect("row exists");
        stored[..XNONCE_LEN].to_vec()
    };
    let write_first = |path: &std::path::Path| {
        let mut db = Db::open(path, DB_KEY, ENTROPY).expect("open");
        let mut txn = db.begin().expect("begin");
        // The first secret write of this open: draws the first nonce.
        txn.put(ns::SESSIONS, b"k", b"v");
        txn.commit().expect("commit");
    };

    write_first(&path);
    let nonce_open_1 = first_nonce(&path);
    write_first(&path);
    let nonce_open_2 = first_nonce(&path);
    assert_ne!(
        nonce_open_1, nonce_open_2,
        "second open with identical entropy replayed the nonce stream"
    );

    // Same key, same entropy, same open count on a fresh file: the stream
    // is deterministic (tests rely on it), so this is the only variable.
    let other = dir.path("other.db");
    write_first(&other);
    assert_eq!(first_nonce(&other), nonce_open_1);
}

// ---------------------------------------------------------------------------
// (5d) A file that is not a database, or a truncated one, is refused with
//      Err (never a panic), and nothing about it is "migrated".
// ---------------------------------------------------------------------------

#[test]
fn garbage_or_truncated_file_is_an_error_not_a_panic() {
    let dir = TempDir::new("garbage");
    let garbage = dir.path("garbage.db");
    std::fs::write(&garbage, [0x5Au8; 4096]).unwrap();
    assert!(matches!(
        Db::open(&garbage, DB_KEY, ENTROPY),
        Err(StorageError::Sqlite(_))
    ));
    // A real database truncated inside its header.
    let real = dir.path("real.db");
    {
        let mut db = Db::open(&real, DB_KEY, ENTROPY).expect("open");
        let mut txn = db.begin().expect("begin");
        txn.put(ns::SESSIONS, b"k", b"v");
        txn.commit().expect("commit");
    }
    let bytes = std::fs::read(&real).unwrap();
    assert!(bytes.len() > 64);
    std::fs::write(&real, &bytes[..64]).unwrap();
    for suffix in ["-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{suffix}", real.display()));
    }
    assert!(Db::open(&real, DB_KEY, ENTROPY).is_err());
}

// ---------------------------------------------------------------------------
// (6) Migrations are idempotent and schema_version is recorded.
// ---------------------------------------------------------------------------

#[test]
fn migrations_are_idempotent_and_recorded() {
    let dir = TempDir::new("migrate");
    let path = dir.path("mesh.db");

    for _ in 0..3 {
        let db = Db::open(&path, DB_KEY, ENTROPY).expect("open");
        assert_eq!(db.schema_version(), migrations::LATEST_VERSION);
        assert_eq!(migrations::LATEST_VERSION, 1);
    }

    let raw = rusqlite::Connection::open(&path).expect("raw open");
    let versions: Vec<i64> = raw
        .prepare("SELECT version FROM schema_version ORDER BY version")
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(versions, vec![1]);
    assert_eq!(
        migrations::current_version(&raw).unwrap(),
        migrations::LATEST_VERSION
    );
    for table in ["kv", "meta", "schema_version"] {
        let n: i64 = raw
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                [table],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(n, 1, "{table}");
    }
    // Running the step list directly on the migrated file changes nothing.
    let mut conn = raw;
    assert_eq!(
        migrations::run(&mut conn).unwrap(),
        migrations::LATEST_VERSION
    );
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM schema_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);

    // A database stamped with a future version is refused on open.
    conn.execute("INSERT INTO schema_version (version) VALUES (99)", [])
        .unwrap();
    drop(conn);
    assert!(matches!(
        Db::open(&path, DB_KEY, ENTROPY),
        Err(StorageError::SchemaTooNew {
            found: 99,
            supported: 1
        })
    ));
}

// ---------------------------------------------------------------------------
// (7) meta round trip.
// ---------------------------------------------------------------------------

#[test]
fn meta_round_trip() {
    let mut db = mem_db();
    assert_eq!(db.meta_get(meta_keys::POWERED_ON_MS).unwrap(), None);
    assert_eq!(db.meta_get_u64(meta_keys::POWERED_ON_MS).unwrap(), None);

    db.meta_set_u64(meta_keys::POWERED_ON_MS, 1_234_567)
        .unwrap();
    db.meta_set_u64(meta_keys::LAST_ROTATION_MONO, u64::MAX)
        .unwrap();
    db.meta_set(meta_keys::SKELETON_DATA_VERSION, b"v3")
        .unwrap();
    assert_eq!(
        db.meta_get_u64(meta_keys::POWERED_ON_MS).unwrap(),
        Some(1_234_567)
    );
    assert_eq!(
        db.meta_get_u64(meta_keys::LAST_ROTATION_MONO).unwrap(),
        Some(u64::MAX)
    );
    assert_eq!(
        db.meta_get(meta_keys::POWERED_ON_MS).unwrap().as_deref(),
        Some(&1_234_567u64.to_le_bytes()[..])
    );
    assert_eq!(
        db.meta_get(meta_keys::SKELETON_DATA_VERSION)
            .unwrap()
            .as_deref(),
        Some(&b"v3"[..])
    );

    // Overwrite, and a counter that is not 8 bytes is Corrupt, not a panic.
    db.meta_set_u64(meta_keys::POWERED_ON_MS, 2).unwrap();
    assert_eq!(db.meta_get_u64(meta_keys::POWERED_ON_MS).unwrap(), Some(2));
    assert!(matches!(
        db.meta_get_u64(meta_keys::SKELETON_DATA_VERSION),
        Err(StorageError::Corrupt { .. })
    ));

    // Through a transaction: rolled back with the rest, committed with the rest.
    {
        let mut txn = db.begin().unwrap();
        txn.meta_set_u64(meta_keys::POWERED_ON_MS, 3).unwrap();
        assert_eq!(txn.meta_get_u64(meta_keys::POWERED_ON_MS).unwrap(), Some(3));
    }
    assert_eq!(db.meta_get_u64(meta_keys::POWERED_ON_MS).unwrap(), Some(2));
    let mut txn = db.begin().unwrap();
    txn.meta_set_u64(meta_keys::POWERED_ON_MS, 4).unwrap();
    txn.meta_delete(meta_keys::SKELETON_DATA_VERSION).unwrap();
    txn.commit().unwrap();
    assert_eq!(db.meta_get_u64(meta_keys::POWERED_ON_MS).unwrap(), Some(4));
    assert_eq!(db.meta_get(meta_keys::SKELETON_DATA_VERSION).unwrap(), None);
    db.meta_delete(meta_keys::POWERED_ON_MS).unwrap();
    assert_eq!(db.meta_get(meta_keys::POWERED_ON_MS).unwrap(), None);
}

// ---------------------------------------------------------------------------
// Determinism: same key, entropy and writes produce the same bytes.
// ---------------------------------------------------------------------------

#[test]
fn same_inputs_produce_same_raw_rows() {
    let dir = TempDir::new("det");
    let mut raws = Vec::new();
    for i in 0..2 {
        let path = dir.path(&format!("mesh{i}.db"));
        {
            let mut db = Db::open(&path, DB_KEY, ENTROPY).expect("open");
            let mut txn = db.begin().unwrap();
            txn.put(ns::SESSIONS, b"k", b"v");
            txn.put("outbox", b"o", b"draft");
            txn.commit().unwrap();
        }
        let raw = rusqlite::Connection::open(&path).unwrap();
        let rows: Vec<(String, Vec<u8>, Vec<u8>)> = raw
            .prepare("SELECT ns, key, value FROM kv ORDER BY ns, key")
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        raws.push(rows);
    }
    assert_eq!(raws[0], raws[1]);
    assert_eq!(raws[0].len(), 2);
}
