//! Namespaced key-value view of one storage transaction.
//!
//! Every state mutation in the core goes through one transaction object so
//! that "decrypt + advance ratchet + store plaintext + delete message key" is
//! committed atomically (BRIEF_v2 §5.3, D6 rule 4). The crypto backends only
//! need this small interface; the SQLite implementation (M1b) wraps a
//! `rusqlite::Transaction` and encrypts secret namespaces at rest.

use std::collections::BTreeMap;

/// Namespaces used by the core. Secret namespaces are encrypted at rest by the
/// SQLite implementation; the in-memory one treats them all alike.
pub mod ns {
    pub const IDENTITY: &str = "identity";
    pub const SESSIONS: &str = "sessions";
    pub const PREKEYS: &str = "prekeys";
    pub const SIGNED_PREKEYS: &str = "signed_prekeys";
    pub const KYBER_PREKEYS: &str = "kyber_prekeys";
    pub const SENDER_KEYS: &str = "sender_keys";
    pub const PEER_IDENTITIES: &str = "peer_identities";
}

/// A transaction-scoped, namespaced byte map.
pub trait KvTxn {
    fn get(&self, ns: &str, key: &[u8]) -> Option<Vec<u8>>;
    fn put(&mut self, ns: &str, key: &[u8], value: &[u8]);
    fn delete(&mut self, ns: &str, key: &[u8]);
    /// All `(key, value)` pairs in `ns` whose key starts with `prefix`, in key order.
    fn scan_prefix(&self, ns: &str, prefix: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)>;
}

/// In-memory store for tests, the simulator and the M0 spike.
#[derive(Debug, Default, Clone)]
pub struct MemKv {
    map: BTreeMap<(String, Vec<u8>), Vec<u8>>,
}

impl MemKv {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl KvTxn for MemKv {
    fn get(&self, ns: &str, key: &[u8]) -> Option<Vec<u8>> {
        self.map.get(&(ns.to_owned(), key.to_vec())).cloned()
    }
    fn put(&mut self, ns: &str, key: &[u8], value: &[u8]) {
        self.map
            .insert((ns.to_owned(), key.to_vec()), value.to_vec());
    }
    fn delete(&mut self, ns: &str, key: &[u8]) {
        self.map.remove(&(ns.to_owned(), key.to_vec()));
    }
    fn scan_prefix(&self, ns: &str, prefix: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
        let lo = (ns.to_owned(), prefix.to_vec());
        self.map
            .range(lo..)
            .take_while(|((n, k), _)| n == ns && k.starts_with(prefix))
            .map(|((_, k), v)| (k.clone(), v.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get_delete_scan() {
        let mut kv = MemKv::new();
        kv.put("a", b"k1", b"v1");
        kv.put("a", b"k2", b"v2");
        kv.put("b", b"k1", b"other");
        assert_eq!(kv.get("a", b"k1").as_deref(), Some(&b"v1"[..]));
        assert_eq!(kv.scan_prefix("a", b"k").len(), 2);
        assert_eq!(kv.scan_prefix("b", b"k").len(), 1);
        kv.delete("a", b"k1");
        assert!(kv.get("a", b"k1").is_none());
        assert_eq!(kv.scan_prefix("a", b"").len(), 1);
    }
}
