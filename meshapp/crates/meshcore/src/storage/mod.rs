//! Storage. Milestone-1 wave 1 provides only the key-value transaction
//! abstraction the crypto backends write through; the SQLite-backed `Txn`,
//! schema and encrypted blob tables land in M1b (docs/DESIGN.md §6).

pub mod kv;

pub use kv::{KvTxn, MemKv};
