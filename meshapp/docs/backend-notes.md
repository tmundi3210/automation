# Backend notes: libsignal-protocol v0.105.0 behind `crypto::backend`

What the M0 spike (DESIGN.md §11 "M0 Spike") verified against the pinned
revision, what it did not need a fork for, the measured message sizes, and
the gaps between `crates/meshcore/src/crypto/backend/` and DESIGN.md §1.
Source: `~/.cargo/git/checkouts/libsignal-*/4beb029/rust/protocol/src`
(tag `v0.105.0`).

## 1. Verdict on the M0 question

**No fork is needed.** Everything DESIGN §1 and D3 K1–K2 ask of the group
backend is reachable through libsignal's public API and its store traits:

| Need | How it is done without a fork | Where |
|---|---|---|
| Sender-key **rotation** (`new_sender_chain` called again, D3 K2) | `create_sender_key_distribution_message` returns the *existing* chain when the sender's record exists. Rotation = delete this node's own `SenderKeyRecord` for the group (and the own-chain marker) from the transaction, then call it again: a fresh 31-bit random chain id, chain key and per-chain signing key. | `libsignal.rs` `GroupBackend::new_sender_chain` |
| Receivers **keep previous chains** (BRIEF §5.4) | `process_sender_key_distribution_message` → `SenderKeyRecord::add_sender_key_state` pushes the new state to the front and pops the back once `MAX_SENDER_KEY_STATES = 5` is reached (`consts.rs`, `sender_keys.rs`). A re-delivered SKDM for a known (chain id, signing key) keeps the existing state (its iteration is not reset). | test (5) in `tests/backend.rs` |
| `remote_session_key` for the **SKDM atomic check** (D3 K1) | `SessionRecord::remote_identity_key_bytes()` is public; the 33-byte key is stripped to the 32-byte X25519 key the KeyCard carries. | `PairwiseBackend::remote_session_key` |
| **TOFU off** (BRIEF §5.2) | The `IdentityKeyStore` is ours: `is_trusted_identity` = "a pin exists and equals the offered key"; `save_identity` never creates or overwrites a pin and returns `UntrustedIdentity` otherwise. libsignal consults it before any state is written in `process_prekey_bundle`, `process_prekey` (first flight), `message_encrypt` and `message_decrypt_signal`, so a refused peer leaves no session and consumes no prekey (test 3). A re-pin to a different key makes the established session unusable in both directions without consuming a message key, and `has_session` reports `false` for it so the pairwise layer re-runs the bundle (test `repin_makes_the_session_unusable_and_has_session_false`). | `TxnStores`, `has_session` |
| **Forged first flights consume nothing** | `message_decrypt_prekey` only calls `mark_kyber_pre_key_used` / `remove_pre_key` / `store_session` *after* the inner message authenticated, so a stranger holding A's gossiped bundle and B's public KeyCard cannot burn A's one-time or Kyber prekeys with a well-formed but unauthenticated PreKey message (test `unauthenticated_first_flight_consumes_no_prekey`). | — |
| **Message keys deleted on decrypt** (D6 rule 4) | Built in: `try_decrypt_with_state` removes the message key from the receiver chain, and the updated `SessionRecord` is written back through our `SessionStore` in the same transaction. A replay returns `DuplicatedMessage` → `BackendError::Decrypt` (tests 2, 7). | — |
| **One-time prekeys deleted on use** (D6 rule 2) | Built in: `message_decrypt_prekey` calls `PreKeyStore::remove_pre_key` after the session is established, and `KyberPreKeyStore::mark_kyber_pre_key_used`, which this backend implements as delete (every Kyber prekey it issues is one-time, one per bundle). A second first flight on the same session (same base key) does not touch the prekey stores (`promote_matching_session`), so out-of-order first flights still decrypt (test 2). | tests 1, 2 |
| **Atomic commit** (D6 rule 4, DESIGN §6) | All six store traits read and write the caller's `KvTxn`; nothing is cached in the backend object. Dropping the transaction drops the ratchet step: the same ciphertext decrypts again on the pre-transaction copy and is refused after the commit (test 7). | `TxnStores` |
| Skipped keys, forward jumps | `MAX_MESSAGE_KEYS = 2000`, `MAX_FORWARD_JUMPS = 25_000`, `MAX_RECEIVER_CHAINS = 5`, `ARCHIVED_STATES_MAX_LENGTH = 40` (`consts.rs`), as D6 rule 1 states. No time-based expiry anywhere in the ratchet. | — |
| Async API without an executor | Every `async fn` only awaits the store traits, which answer synchronously, so one `poll` with `Waker::noop()` completes them. `block_on` reports a `Pending` as `Internal`; it has never happened. | `libsignal.rs` `block_on` |

## 2. How `now` is derived

libsignal takes a `SystemTime` in `process_prekey_bundle` and
`message_encrypt`. The backend passes `UNIX_EPOCH + Duration::from_millis(now_mono)`
where `now_mono` is the injected powered-on counter (`clock::Clock::mono_ms`).
It is a monotonic counter re-based at the epoch, never a wall clock; the
library only ever compares it with later values of the same counter, so the
epoch is irrelevant. `clippy.toml` forbids `SystemTime::now` in the crate and
the backend does not call it. The only library rule that reads it:
an **unacknowledged** session (bundle processed, no reply decrypted yet) older
than `MAX_UNACKNOWLEDGED_SESSION_AGE = 30 days` of *powered-on* time is
refused by `message_encrypt` (`SessionNotFound` → `BackendError::NoSession`),
which the pairwise layer should surface as "Needs their key" and re-run the
bundle. Established sessions have no expiry (D6 rule 6).
`has_session` asks the library with `SessionUsabilityRequirements::empty()`,
so its `now` argument is unused and that call reads no clock at all.

Two library constructors read the OS clock or OS RNG internally
(`KyberPreKeyRecord::generate`, `SignedPreKeyRecord` helpers in the FFI
layer); the backend does not use them and builds those records by hand from
the injected RNG and `now_mono` (`GenericSignedPreKey::new`).

## 3. Observed sizes (test 9, `cargo test -p meshcore --test backend -- --nocapture`)

| Object | Bytes | Notes |
|---|---|---|
| Prekey bundle, 0 one-time prekeys | 1783 | `PreKeyBundleWire`: 1 ver + 4 reg id + 1 list count + (4+33+2+64) SPK + (4+2+1569+2+64) Kyber1024 + 33 identity |
| Prekey bundle, 1 one-time prekey | 1820 | +37 per one-time prekey (id u32 + 33-byte key) |
| PreKey message, 100 B plaintext | 1856 | PQXDH first flight: carries the 1568-byte Kyber1024 ciphertext; matches D6's "~1.57 KB per pre-reply message" |
| Normal message, 100 B plaintext | 168 (responder's first) / 201 (initiator, after acknowledgement) | the SPQR post-quantum ratchet adds a few dozen bytes on some messages |
| SenderKeyDistributionMessage | 96 | matches D3 Q6 "SKDM about 95 B" |
| SenderKeyMessage, 100 B plaintext | 205 | AES-CBC padding to 112 + 64-byte per-chain signature + header; a post ≤ 2 kB fits in 5 cells of `BODY_PLAINTEXT_LEN = 476` |

Overheads: Normal ≈ plaintext + 68 … 101 B; SenderKeyMessage ≈ plaintext +
~105 B (plus CBC padding to 16); PreKey ≈ Normal + ~1690 B. The 50-member
SKDM bundle estimate in D3 K2 (n−1 pairwise ciphertexts of a 96-byte SKDM ≈
(96+~70)×49 ≈ 8 KB) holds; the first-contact cost is the bundle + PreKey
message (≈ 3.7 KB per pair) and is paid once per pair, as DECISIONS §3 row
"3, 4, 6, 9, 10" assumes.

## 4. Storage layout (all libsignal-native record encodings)

| namespace | key | value |
|---|---|---|
| `identity` | `x25519_keypair` | `IdentityKeyPair::serialize()` |
| `identity` | `registration_id`, `current_signed_prekey_id`, `next_prekey_id`, `next_signed_prekey_id`, `next_kyber_prekey_id` | u32 LE |
| `identity` | `own_chain/` ‖ group id | u32 LE chain id (the record's chain id is not readable through the public API) |
| `peer_identities` | `<ed25519 hex>/1` | pinned 32-byte X25519 key |
| `sessions` | `<ed25519 hex>/1` | `SessionRecord::serialize()` |
| `prekeys`, `signed_prekeys`, `kyber_prekeys` | id u32 BE | prekey record `serialize()` |
| `sender_keys` | `<address>/` ‖ group id (own chains under `x25519:<hex>/1`) | `SenderKeyRecord::serialize()` |

The own address `x25519:<hex>/1` is only a storage key: libsignal binds
addresses into message MACs only when they parse as Signal service ids
(`SignalMessage::serialize_addresses`), which ours never do. Peer binding is
the roster KeyCard, checked through the identity store.

## 5. Gaps and decisions left open by DESIGN §1

1. **Trait shape.** DESIGN §1 sketches `&mut self` methods and a
   `new_identity(rng) -> (IdentitySeed, KeyCard)`; the implemented traits are
   `&self` over a `&mut dyn KvTxn` (the backend is stateless) and identity
   creation is split: `init_identity` returns the X25519 public key and the
   caller issues the KeyCard with its Ed25519 signer (`keycard::KeyCard::issue`).
   DESIGN names a third `trait BackendStore` and a `storage/backend_stores.rs`
   module; both collapsed into the crate-private `TxnStores` in
   `libsignal.rs` because libsignal's store traits are the only consumer.
2. **Both traits have `encrypt`/`decrypt`.** On a `dyn Backend` they must be
   called as `PairwiseBackend::encrypt(&*be, …)` / `GroupBackend::decrypt(…)`.
   Renaming (`encrypt_to` / `post`) is a one-line change if the composition
   layer finds this annoying.
3. **One-time prekeys per bundle.** `make_prekey_bundle(one_time)` puts
   `one_time` keys in the bundle but `process_prekey_bundle` uses only the
   first. The "10 reserve OPKs for re-establishment" of D6 rule 2 needs the
   pairwise layer to remember the unused entries and pass a bundle with the
   next one when re-establishing; not built.
4. **Gossiped (SPK-only) bundles are single-use.** Every Kyber prekey is
   one-time and deleted on first use, so a bundle with `one_time = 0` still
   serves exactly one first flight. D6 rule 2's gossiped stranger bundles need
   a *last-resort* Kyber prekey (kept, with the base-key replay cache of D6
   rule 3) — libsignal's `mark_kyber_pre_key_used` contract allows it; the
   store would keep a per-(kyber id, SPK id) set of seen base keys. Decide
   before `pairwise/session.rs`.
5. **SPK deletion horizon.** `LibsignalBackend::retire_signed_prekeys(txn,
   created_before_mono)` deletes retired SPKs *and the Kyber prekeys issued
   under them* (each Kyber record is stamped with its SPK's generation; a
   first flight needs both, so a Kyber key without its SPK is dead weight of
   ~3 KB of PQ secret per bundle). Computing the horizon ("≥ 7 d wall AND
   ≥ 48 h powered-on, OR ≥ 7 d powered-on", D6 rule 3) is the caller's, since
   the backend sees only `now_mono`. The first SPK is created lazily with
   stamp 0 by the first `make_prekey_bundle`. Unused one-time prekeys are
   never retired by the backend (they are the D6 rule 2 reserve keys; the
   pairwise layer owns their lifetime).
6. **Chain retention is exactly 5 states per sender** (test 5: after the 5th
   rotation the first chain is gone; after 6 rotations chains 0 and 1 are
   gone). The window is ordered by the *last SKDM delivery*, not by chain
   creation: `add_sender_key_state` moves a re-delivered chain to the front,
   so the next eviction then takes the next-oldest chain (test
   `redelivered_skdm_moves_its_chain_to_the_front_of_the_retention_window`).
   The groups layer should not re-send old SKDMs casually. D3 K2 says to
   measure late-message loss before changing the cap; changing it is a
   one-constant fork of `consts::MAX_SENDER_KEY_STATES` and then needs the
   §5.8 audit.
7. **Sender keeps only its current chain.** Rotation deletes the own record,
   so a node cannot decrypt its own earlier posts through the backend; the
   plaintext outbox (DESIGN §6 `objects`) is the source for one's own posts.
8. **Header encryption / tags** (D6 pick B) are not here, as expected; the
   Double Ratchet header and the SKDM/SenderKeyMessage headers
   (distribution id, chain id, iteration) are cleartext inside the backend's
   bytes and must sit under the §5.5 envelope.
9. **vodozemac** stays a stub: `MAX_MESSAGE_KEYS = 40` is `pub(crate)` in
   `olm/session/receiver_chain.rs`, so D6 rule 1's 2000 needs a fork, plus an
   app-side multi-session store for crossed initiation and per-(sender, chain)
   Megolm inbound sessions for retention. `cargo build -p meshcore
   --no-default-features --features backend-vodozemac` compiles.
10. **Licence.** The default backend is AGPL-3.0-only (crate licence field);
    the owner's decision (docs/decisions-assumed.md row 2) is still open.
