//! Storage (docs/DESIGN.md §6; BRIEF_v2 §5.3).
//!
//! * [`kv`]: the namespaced key-value transaction abstraction the crypto
//!   backends, roster and node write through, plus the in-memory `MemKv`.
//! * [`db`]: the SQLite handle (`Db::open`, pragmas, migrations, `meta`).
//! * [`txn`]: the SQLite-backed `Txn` implementing `KvTxn` with secret
//!   namespaces encrypted at rest under `db_key`, and atomic `commit`.
//! * [`migrations`]: ordered schema steps (`schema.sql` is v1).

pub mod db;
pub mod kv;
pub mod migrations;
pub mod txn;

pub use db::{Db, StorageError, meta_keys};
pub use kv::{KvTxn, MemKv, ns};
pub use txn::{SECRET_NAMESPACES, SECRET_OVERHEAD, Txn, is_secret_namespace};
