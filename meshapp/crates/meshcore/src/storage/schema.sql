-- meshcore storage schema, version 1 (docs/DESIGN.md §6; BRIEF_v2 §5.3).
--
-- Milestone 1 keeps every record (ratchet sessions, prekeys, sender keys,
-- pins, cells, objects, the dedupe cache, plaintext, outbox) in the single
-- namespaced `kv` table; the per-kind tables and indexes of DESIGN §6 are
-- later milestones. Secret namespaces (storage/txn.rs `SECRET_NAMESPACES`)
-- hold `nonce ‖ XChaCha20-Poly1305 ciphertext` under a per-namespace key
-- derived from `db_key`; the other namespaces hold plaintext.
--
-- Every statement is idempotent (`IF NOT EXISTS`) so a step that was
-- interrupted before `schema_version` was written can be re-run.

CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS kv (
    ns    TEXT NOT NULL,
    key   BLOB NOT NULL,
    value BLOB NOT NULL,
    PRIMARY KEY (ns, key)
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS meta (
    key   TEXT PRIMARY KEY,
    value BLOB
);
