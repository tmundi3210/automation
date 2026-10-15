# DESIGN.md: meshapp P0 design (milestone 1 scope = core + simulator)

_Source: the implementation design produced for plan v5 (2026-10-15) from BRIEF_v2.md and DECISIONS.md. Sections 8–9 (Android, iOS) describe later milestones and are kept for reference; nothing there is built in milestone 1. Where this file and the code disagree, the code's tests and `docs/wire-format.md` win; where this file and BRIEF_v2/DECISIONS.md disagree, the brief and DECISIONS.md (run 2) win. Deviations adopted in plan v5: "Needs someone" after 6 h (not 2 h); no uniffi dependency in milestone 1._

# meshapp P0 implementation plan

Conventions used below: **[B §n]** = BRIEF_v2 section, **[Dn]** = DECISIONS.md decision, **[§3-k]** = DECISIONS.md §3 cross-decision row k (1-based, L918–939). Constants that the brief marks as assumptions are named `*_ASM` in code and live in one file so the spike can retune them.

## 1. Directory layout and module responsibilities

```
meshapp/
  Cargo.toml                      workspace: crates/meshcore, crates/meshcore-fuzz, sim/meshsim
  rust-toolchain.toml             pinned stable; nightly only for cargo-fuzz
  clippy.toml                     disallowed-methods: SystemTime::now, Instant::now (protocol code must use clock::Clock)
  crates/meshcore/                lib + cdylib (uniffi), no_std-free but no OS clock, no threads
  crates/meshcore-fuzz/           cargo-fuzz targets
  sim/meshsim/                    headless multi-node simulator binary + scenarios/*.toml
  bindings/{kotlin,swift}/        uniffi-bindgen output (checked in per release tag)
  android/                        Gradle project (modules in §8)
  ios/                            Xcode project + SwiftPM package MeshCore (XCFramework)
  docs/                           wire-format.md roster-dag.md tasks.md votes.md storage.md
                                  platform-caveats.md testing.md audit-scope.md (= B §5.8 list)
  tools/                          gen-bindings.sh build-android.sh build-ios.sh run-scenarios.sh
  .github/workflows/ci.yml
```

### `crates/meshcore/src` modules (one test file per module under `tests/<module>.rs`)

| Module | Responsibility | Key public types / fns | Implements |
|---|---|---|---|
| `clock.rs` | Injected time. Monotonic powered-on ms persisted across restarts; wall clock exposed only as `for_display()` | `trait Clock { fn mono_ms(&self)->u64; fn wall_for_display(&self)->i64 }`, `PoweredOnMeter` | B §4 "no timestamps decide anything", D6 rule 3 |
| `codec/` | Canonical fixed-field LE codec (see §2). `Writer`, `Reader` with hard length checks; `Encode`/`Decode` traits; `str16` (u16 len + UTF-8) | `fn encode<T:Encode>(&T)->Vec<u8>`, `fn decode<T:Decode>(&[u8])->Result<T,CodecError>` (rejects trailing bytes) | B §5.5 format checks |
| `crypto/hash.rs` | SHA-256, `prf16(key,label,ctr)` = Trunc16(HMAC-SHA256) | `Hash32`, `fn object_hash(domain:&str, bytes)->Hash32` | B §5.5 tags, msg_id |
| `crypto/aead.rs` | ChaCha20-Poly1305 wrapper for cell bodies and storage blobs | `seal(key,nonce,aad,pt)`, `open(...)` | B §5.5 |
| `crypto/ed25519.rs` | One strict verifier for both OSes: `verify_strict` (ed25519-dalek 2.x) + admission check: reject small-order (`is_weak`) and non-canonical encodings (decompress→recompress equality), reject non-canonical `s` | `IdentityKey`, `IdentitySigner`, `fn admit_public_key(&[u8;32])->Result<IdentityKey,KeyReject>`, `fn verify(domain, key, msg, sig)` | B §5.1 last bullet, D1 rule 2, §3-22 |
| `crypto/keycard.rs` | Binds the Ed25519 identity to the backend's X25519 session key | `KeyCard{ed:[u8;32], x25519:[u8;32], sig:[u8;64]}` signed over `"meshapp.v1.keycard"‖x25519` | B §3 "one contact = one key", D3 K1 |
| `crypto/backend/mod.rs` | Minimal backend trait (see below) | `trait PairwiseBackend`, `trait GroupBackend`, `trait BackendStore` | B §5.2 |
| `crypto/backend/libsignal.rs` | Default impl (git dep, pinned rev, feature `backend-libsignal`) | `LibsignalBackend` | D3 K1–K2, D6 rule 1 |
| `crypto/backend/vodozemac.rs` | Feature `backend-vodozemac` (forked crate, MAX_MESSAGE_KEYS=2000, app-side session selection) | `VodozemacBackend` | D6 rule 1 |
| `wire/cell.rs` | 512-B cell build/parse, TTL rules, `msg_id` derivation, padding | `const CELL_LEN:usize=512`, `Cell`, `CellHeader`, `fn parse(&[u8;512])->Result<Cell,CellReject>`, `fn msg_id(&Cell)->Hash32`, `fn initial_ttl(rng)->u8` | B §5.5 on-air, §3-9 |
| `wire/inner.rs` | Recognition-layer header + fragmentation/reassembly of objects into cells | `InnerHeader{kind,obj_seq,frag_idx,frag_total,len}`, `Fragmenter`, `Reassembler` (bounded, per chain) | B §5.5 |
| `wire/recog.rs` | Per-chain recognition epochs: tag windows, per-counter cell keys, lookup table `tag→(chain,ctr)` | `RecogEpoch{k_tag,k_hdr,base_ctr}`, `RecogChain` (≤3 epochs), `TagTable`, `fn precompute(window)` | §3-8 (counter tags), D6-B |
| `wire/seal.rs` | Sealed first flights to a gossiped envelope key, trial match against ≤3 live keys | `EnvelopeKeyPair`, `fn seal(e_r, payload)`, `fn try_unseal(live_keys, cell)` | D6 pick B (2), §3-10 |
| `wire/frame.rs` | Per-hop link frame (inside Noise): `{frame_type, class, flags}` + payload; link control messages Hello/Summary/Throttle/Auth | `LinkFrame`, `FrameType`, `Class` (C0–C4) | B §5.5 "class inside the per-hop layer", §5.6 |
| `wire/noise.rs` | snow `Noise_NN_25519_ChaChaPoly_BLAKE2s` per link, 2-B length framing, handshake-hash export for L2 proofs | `NoiseLink::{initiator,responder}`, `fn encrypt_frame`, `fn decrypt_frame`, `fn handshake_hash` | B §5.5 per-hop Noise NN |
| `objects/sender_chain.rs` | Per-author hash chain for identity-signed objects: `(author_seq, prev_own)`, equivocation detection | `SenderChainIndex`, `fn check(obj)->ChainVerdict{Ok,Equivocation,Gap}` | B §4 votes (fork rule), D2 rule 5 |
| `objects/roster_op.rs` | Roster op encoding/decoding/signing (schema §3) | `RosterOp`, `RosterBody::{Genesis,Add,Remove,AdminSet,Setting}` | B §5.4 |
| `objects/task_op.rs` | Task op (inside group posts) | `TaskOp`, `TaskKind::{Create,Take,Release,Done}` | B §4 tasks |
| `objects/vote_open.rs`, `objects/ballot.rs` | P1 types specified now (§5) | `VoteOpen`, `Ballot`, `fn vote_id` | B §4 votes |
| `objects/chat_post.rs` | Group/1:1 post plaintext: kind, roster heads, body (text, preview ref, receipt, task op, carried ops) | `PostPlain`, `PostKind` | B §5.4 M4 heads-in-ciphertext |
| `objects/skdm_bundle.rs` | SKDM bundle object (padded to 8/16/32/64 recipients) | `SkdmBundle{entries:Vec<(hint8,ct)>}` | D3 K2, §3-11 |
| `roster/dag.rs` | Op storage, parents/children index, `closure(H)`, parking of ops with missing heads, missing-op requests | `RosterDag`, `fn insert(op)->Insert{Applied,Parked(missing),Rejected}`, `fn closure(&[Hash32])->OpSet` | D3 M1 |
| `roster/eval.rs` | Pure `members(H)`, `admins(H)` with remove-wins, authority and equivocation rules (§3) | `fn evaluate(dag,&heads)->RosterView{members,admins,voided:Vec<(Hash32,VoidReason)>}` (memoised by head-set hash) | D3 M2–M3, §3-3 |
| `roster/rotation.rs` | Rotation triggers (removal learned, 24 h powered-on) and the "removed-author post" verdict | `RotationPolicy`, `fn post_verdict(post_heads, author)->{Accept,RejectRemovedInHeads,KeepLabelled}` | D3 K2, §3-4 |
| `roster/labels.rs` | UI label derivation ("[Name] removed · Waiting for people out of range", "Changed after reconnect") | `fn labels(view_before, view_after)` | D3 UI labels |
| `groups/group.rs` | `GroupState`: own chain + recog epochs, members' chains, pending rotation, atomic SKDM acceptance (identity bound), encode/decode posts | `GroupState`, `fn accept_skdm(txn, from:PeerId, skdm)` (checks session x25519 == roster KeyCard.x25519 then stores in the same txn), `fn next_post(txn, plain)->Vec<Cell>` | B §5.2 third bullet, D3 K1 |
| `pairwise/session.rs` | DR session per contact over the backend; crossed initiation; decrypt-failure detection → "Needs their key" | `PairwiseManager`, `fn encrypt_to(peer, plain)`, `fn decrypt_from(peer, ct)` | D6 rules 2,5 |
| `pairwise/bundles.rs` | SPK 24 h rotation, deletion rule (≥7 d wall AND ≥48 h powered-on, or ≥7 d powered-on), OPK reserve handed only at direct encounter, base-key replay cache | `BundleManager` | D6 rule 3 |
| `pairwise/pairing.rs` | QR verify flow: KeyCard exchange, mutual proof over the Noise handshake hash, bundle exchange, first DR round trip on the direct link, recognition seed | `PairingSession`, `fn qr_payload()`, `fn on_scanned(qr)`, `fn step(frame)->PairingStep` | B §3 level 2, §6 step 2, D6 rule 2 |
| `tasks/state.rs` | Fold a task's op DAG into `TaskState`; "2 people on this" | `TaskState::{Open,Taken{holders},Conflict{winner,losers},Done}`, `fn fold(ops)` | B §4 tasks |
| `tasks/tiebreak.rs` | Deterministic, non-grindable winner (§4) | `fn winner(task_id, concurrent_takes:&[TakeOp])->IdentityKey` | B §9 owner decision 6 |
| `tasks/pool.rs` | "Needs someone" evidence-age rule | `fn needs_someone(task, last_heard, now_mono)->Option<EvidenceAge>` | B §4, D10 (6) |
| `votes/open.rs`, `votes/tally.rs`, `votes/state.rs` | P1 logic, specified now; `tally.rs` is pure and property-tested | `fn tally(open,&members,&ballots)->VoteState` | B §4 votes |
| `people/contacts.rs` | Pin store (encrypted blobs), badges, pin sources, local re-pin | `Contact{key,petname,badge,pins:Vec<PinSource>}`, `fn repin_after_scan` | B §3 |
| `people/samename.rs` | Normalisation (NFKC casefold → strip Default_Ignorable/whitespace/punct → UTS#39 skeleton via `unicode-security`, version pinned) and marker state as `f(keys seen, pins)` | `fn normalise(&str)->String`, `SameNameIndex`, `fn marker_for(key)->Option<Marker>` | B §3 same-name, D5 R3–R9 |
| `dtn/store.rs` | Cell store (fixed rows), per-class/per-link accounting | `CellStore`, `fn put(cell, meta)`, `fn candidates_for(link, not_in:&Bloom, budget)` | B §5.7 DTN |
| `dtn/dedupe.rs` | msg_id cache with powered-on expiry = max(retention, key retention)+72 h; rejected-item hashes | `DedupeCache`, `fn seen_or_insert(msg_id)` | B §5.5 relay bullets, §3-13 |
| `dtn/summary.rs` | Bloom summary vectors (salted, chunked into frames), "last sync" digest match | `SummaryVector`, `fn build(cells)`, `fn contains` | D10 (4) |
| `dtn/sync.rs` | Per-link anti-entropy state machine: Hello → summaries both ways → push by priority; member reconciliation on its own budget | `LinkSync`, `fn on_frame`, `fn next_to_send(budget)` | D4 MERGE, D10 (4) |
| `relay/lanes.rs` | Lane assignment L1/L2/L3 per cell per link | `Lane`, `fn classify(cell, link_auth, validation)->Lane` | B §5.5 lanes, §3-4 (iii) |
| `relay/limits.rs` | Token buckets per (link,class): control 20/min, chat 60/min, bulk 256 kB/min; 10-min ramp for unvalidated only; GossipSub-style link score/penalty | `LinkBudget`, `fn admit(cell, lane)->Admit` | D4 PER INBOUND LINK |
| `relay/sched.rs` | Strict class order C0 > C1 > (C3 verified: P2) > C2@L1/L2 > C2@L3 > C3 > C4; fair queue across inbound links; "Waiting · network busy" signal | `Scheduler`, `fn pick(link)->Option<Cell>` | B §5.6, §3-15 |
| `relay/buffers.rs` | 50 MB total, 5 MB/inbound link, 20 % reserve for absent peers, eviction L3-first then oldest/lowest TTL, roster ops never evicted until superseded | `BufferPolicy` | D4 BUFFER, D3 M1 |
| `relay/slots.rs` | Link-slot policy (≥50 % outbound-chosen), non-connectable advertising hint to the platform | `SlotPolicy` → `PlatformHint::{StopAcceptingInbound,ResumeInbound}` | D4 LINK SLOTS |
| `transport/link.rs` | `Link` trait + `LinkId`, `LinkAuth`, `LinkInfo` (see §7) | | B §5.7 |
| `transport/platform_link.rs` | Byte-stream adaptor used by the uniffi surface (`feed`/`poll_outbound`) | `PlatformLink: Link` | |
| `transport/chunk.rs` | Stream framing/reassembly of length-prefixed Noise messages (MTU-agnostic) | `StreamFramer` | |
| `storage/db.rs`, `schema.sql`, `migrations.rs` | rusqlite (bundled), WAL, `PRAGMA secure_delete=ON` | `Db::open(path, db_key)` | B §5.3 storage |
| `storage/txn.rs` | Atomic commit API; all state mutations go through one `Txn` | `Txn`, `fn commit(self)` | D6 rule 4 |
| `storage/kv.rs` | Encrypted blob table (XChaCha20-Poly1305 under `db_key`) for ratchet/sender-key records, pins, plaintext, outbox | `SecretKv` | B §3 "pin store encrypted at rest" |
| `storage/backend_stores.rs` | libsignal store traits implemented over `Txn` (write-through overlay, flushed on commit) | `TxnSessionStore`, `TxnSenderKeyStore`, … | D3 K1 atomic step |
| `node/node.rs` | Wiring; single-threaded `Node` behind the uniffi mutex; tick scheduler | `Node::{new,start,stop,tick}` | |
| `node/inbound.rs` | Pipeline: stream → frame → cell (format/size/TTL) → dedupe → store+relay admit → recognise (tag table / seal trial) → reassemble → decrypt in txn → dispatch | `fn on_cell(link, cell)` | B §5.5 "every relay" |
| `node/outbound.rs` | Object → cells (class, TTL draw, tags), B-build delays/jitter hooks (A: zero) | `fn emit(obj)->Vec<Cell>` | B §0 B defaults |
| `node/status.rs` | Status strip, delivery states (Waiting / Waiting·network busy / Passed along / Delivered / May have expired) | `StatusStrip`, `DeliveryState` | B §2, §5.9 |
| `api/` | uniffi surface (§7, §8): `node.rs` (`MeshNode`), `types.rs`, `events.rs`, `errors.rs` | | |

**Backend trait (kept minimal; everything else is composition in `groups/` and `pairwise/`):**

```rust
pub trait PairwiseBackend {
    fn new_identity(&self, rng:&mut dyn RngCore) -> (IdentitySeed, KeyCard);
    fn make_prekey_bundle(&mut self, txn:&mut Txn) -> Result<Vec<u8>>;          // SPK+Kyber(+OPKs on request)
    fn process_prekey_bundle(&mut self, txn:&mut Txn, peer:&PeerId, bundle:&[u8]) -> Result<()>;
    fn encrypt(&mut self, txn:&mut Txn, peer:&PeerId, plain:&[u8]) -> Result<PairwiseCt>; // {kind: PreKey|Normal, bytes}
    fn decrypt(&mut self, txn:&mut Txn, peer:&PeerId, ct:&PairwiseCt) -> Result<Vec<u8>>;
    fn remote_session_key(&self, txn:&Txn, peer:&PeerId) -> Option<[u8;32]>;      // X25519 used for the SKDM atomic check
    fn has_session(&self, txn:&Txn, peer:&PeerId) -> bool;
}
pub trait GroupBackend {
    fn new_sender_chain(&mut self, txn:&mut Txn, group:&GroupId) -> Result<(ChainId, Vec<u8> /*SKDM*/)>; // called again = rotation
    fn process_skdm(&mut self, txn:&mut Txn, group:&GroupId, sender:&PeerId, skdm:&[u8]) -> Result<ChainId>;
    fn encrypt(&mut self, txn:&mut Txn, group:&GroupId, plain:&[u8]) -> Result<Vec<u8>>;               // SenderKeyMessage incl. per-chain signature
    fn decrypt(&mut self, txn:&mut Txn, group:&GroupId, sender:&PeerId, ct:&[u8]) -> Result<(ChainId, Vec<u8>)>;
}
```
`PeerId` = Ed25519 identity. The backend holds X25519 keys only; the KeyCard binds them. Rotation = `new_sender_chain` again; receivers keep previous chains (libsignal keeps 5 states per sender record).

## 2. Byte layouts

### 2.1 Encoding choice
**Fixed-field little-endian structs with a hand-written codec, no CBOR.** Reasons: (1) canonical by construction, so hashes and signatures need no canonicalisation pass and no second "is this canonical?" validator; (2) the parser surface is ~300 lines, fuzzable and in the §5.8 audit scope; (3) no dependency whose canonical-profile behaviour can drift between versions on two platforms; (4) every object already needs a `ver` byte and a "reject unknown version" rule, which is all the schema evolution P0 needs. Variable-length fields: `str16` = u16 LE length + bytes; lists = u8 count + items; hashes 32 B; keys 32 B; signatures 64 B. Decoders reject trailing bytes, over-long lengths and unknown enum values. All identity signatures use domain separation: `sig = Ed25519(sk, domain_bytes ‖ content)`, domains listed in `docs/wire-format.md`.

### 2.2 On-air cell: `CELL_LEN = 512` (all offsets in bytes)

| off | len | field | value / rule |
|---|---|---|---|
| 0 | 1 | `ver` | 0x01. Relays forward any `ver` in 1..=15 (TTL at a fixed offset) but only interpret 1 |
| 1 | 1 | `ttl` | sender draws uniformly from {6,7,8}; relay forwards iff `ttl ≥ 2`, with `ttl−1`; excluded from `msg_id` |
| 2 | 1 | `token_type` | 0x00 in v1 (reserved for P2 rate tokens); v1 relays reject ≠0 |
| 3 | 1 | `token_len` | 0x00 in v1; v1 relays reject ≠0 |
| 4 | 16 | `tag` | `prf16(k_tag, "tag", ctr)` for chain cells; for sealed first flights `Trunc16(SHA256("seal"‖E_r‖e_pub))` |
| 20 | 492 | `body` | ChaCha20-Poly1305 ciphertext (476 B plaintext + 16 B auth tag); key = `HKDF(k_hdr, "cell", ctr)`, nonce = `LE32(frag_idx)‖0⁸`, AAD = bytes 0, 2..20 |

`msg_id = SHA256(cell[0] ‖ cell[2..512])` — **derived, not transmitted**: a transmitted id adds 32 B/cell (6 %) for no gain, since relays must recompute it to validate it anyway. Dedupe, summary vectors and storage all key on the derived value.

**Inner plaintext (476 B)** under the cell key:

| off | len | field |
|---|---|---|
| 0 | 1 | `kind`: 0x01 pairwise DR message, 0x02 group SenderKey message, 0x03 sealed first flight (then bytes 11..43 = `e_pub` and the AEAD under `k_seal` starts at 43), 0x04 SKDM bundle, 0x05 receipt batch (pairwise), 0x06 SOS |
| 1 | 4 | `obj_seq` u32: per chain object counter |
| 5 | 2 | `frag_idx` u16 |
| 7 | 2 | `frag_total` u16 (≤ 141 → 64 kB C1 cap) |
| 9 | 2 | `len` u16 ≤ 465 |
| 11 | 465 | data, zero-padded |

One cell = one recognition counter (`ctr` increments per cell, not per object), so relays and sniffers cannot group cells of one object except by timing.

**Why 512 B.** ATT MTU is 517 on Android↔Android and Android↔iOS pairs (Android requests 517; iOS accepts up to 527) and 185–247 on older stacks, so no cell size fits "one write" everywhere; the link therefore carries a *byte stream* of length-prefixed Noise frames (frame = 2 + 4 + 512 + 16 = 534 B) chunked at MTU−3 by the platform, and cell size is chosen on padding and overhead, not on MTU. At 512: a typical group text post (~24 B SenderKey header + text + 64 B chain signature + 2×32 B roster heads) ≈ 300 B fits in one cell; SOS (≤256 B), ballots (~165 B), roster ops (~250–450 B), task ops and receipts are single-cell; a PQXDH first flight (~1.6–1.8 kB) is 4 cells; a 2 kB text is 5; a 16 kB preview is 36. Per-cell overhead is 20 + 11 + 16 = 47 B (9 %); padding waste on a 300-B post is ~35 %, versus ~70 % at 1024 B. A locked iPhone's ~10 s wake loses at most one half-written cell. The D9-B "1–2 kB" guess was written for preview cells; the sim reports bytes-on-bottleneck for 512/768/1024 (`CELL_LEN` is a compile-time constant with the inner layout derived from it) and the spike picks the final value before the on-air freeze.

### 2.3 Link frame (inside Noise NN transport message)

`frame_type u8` (0 cell, 1 Hello, 2 Summary chunk, 3 Throttle, 4 ContactProof) ‖ `class u8` (0..4, cells only) ‖ `flags u8` (bit0 = relayed-not-origin hint is **never** set; reserved) ‖ `reserved u8` ‖ payload. Hello = `{proto u8, caps u16, is_station u8, buffer_free u32}`. Summary chunk = `{salt u32, n_bits u32, k u8, chunk_idx u16, chunk_total u16, bits}`. Throttle = `{class u8, retry_ms u32}`. ContactProof = `n u8 ‖ [prf16(k_pair(c), "l2", handshake_hash)]×n` (one per QR-verified contact; the responder matches against its own contacts → L2).

### 2.4 Inner objects

**Common header for identity-signed objects** (roster ops, vote-opens): `ver u8 ‖ kind u8 ‖ group_id [16] ‖ author [32] ‖ author_seq u32 ‖ prev_own [32] ‖ n_heads u8 ‖ heads [32]×n (sorted ascending)` … body … `sig [64]`. `op_hash = SHA256(all bytes incl. sig)`; `prev_own` = hash of the author's previous identity-signed object (0³² for the first).

| Object | kind | Body |
|---|---|---|
| `KeyCard` | — | `ed [32] ‖ x25519 [32] ‖ sig [64]` (domain `meshapp.v1.keycard`) |
| Roster Genesis | 0x10 | `nonce [16] ‖ name str16 ‖ n_admins u8 ‖ KeyCard×n ‖ n_members u8 ‖ (KeyCard ‖ name str16 ‖ in_person u8)×n ‖ settings u8`; `n_heads = 0`; `group_id` field zero; group id = `Trunc16(op_hash)` |
| Roster Add | 0x11 | `subject KeyCard ‖ name str16 ‖ in_person u8 ‖ endorsement{admin [32] ‖ in_person u8 ‖ sig [64]}` where endorsement sig is over `meshapp.v1.endorse ‖ group_id ‖ subject.ed ‖ name ‖ in_person` |
| Roster Remove | 0x12 | `subject [32]` |
| Roster AdminSet | 0x13 | `version u32 ‖ n u8 ‖ admins [32]×n (sorted) ‖ n_cosig u8 ‖ (admin [32] ‖ sig [64])×n_cosig`; cosig domain `meshapp.v1.adminset ‖ group_id ‖ version ‖ admins` |
| Roster Setting | 0x14 | `key u8 ‖ value u8` (straw-mode, media-off; P0 authority, used from P1) |
| VoteOpen | 0x20 | `question str16 ‖ n_opt u8 (2..=8) ‖ options str16×n ‖ f u8 ‖ T u16 ‖ N u16`; `vote_id = SHA256(bytes before sig)` |
| Ballot | 0x21 | `ver u8 ‖ kind u8 ‖ vote_id [32] ‖ key [32] ‖ choice u8 ‖ sig [64]` over `meshapp.v1.ballot ‖ vote_id ‖ key ‖ choice`; **not** in the sender chain; dedupe key = `(vote_id,key,choice)` |
| Post plaintext (inside SenderKeyMessage / DR message) | 0x30 text, 0x31 task op, 0x32 carried VoteOpen, 0x33 carried Ballot, 0x34 carried RosterOp, 0x35 receipt, 0x36 SKDM bundle | `ver u8 ‖ kind u8 ‖ n_heads u8 ‖ roster_heads [32]×n ‖ body`; text body = `text str16 (≤2000 B) ‖ preview_hash [32] (zero if none)`; groups: author = chain owner (bound by the SKDM atomic check); 1:1: `n_heads = 0` |
| TaskOp body | — | `task_id [16] ‖ op u8 {1 Create,2 Take,3 Release,4 Done} ‖ n_heads u8 ‖ task_heads [32]×n ‖ (Create: title str16 ‖ details str16)`; op hash = `SHA256("meshapp.v1.taskop" ‖ 0x00 ‖ author [32] ‖ post plaintext)` (author = the post's chain owner, which the group layer knows for every decrypted post; `objects/task_op.rs::task_op_hash`); `task_id = Trunc16(op hash of the Create)` (`task_id_for_create`). The post plaintext has no author field, so the author is bound into the hash: two members who Create or Take with byte-identical posts get two ids, and a `task_heads` citation names one member's copy |
| SKDM bundle | 0x36 | Bundle object: `chain_id u32 ‖ L u16 ‖ n u8 ‖ (hint [8] ‖ len u16 ‖ ct[len] ‖ zeros[L−len])×n`, `n` padded to 8/16/32/64 with random dummies and **every** entry (real or dummy) padded to `L` = the longest real ciphertext, so a PreKey entry, a Normal entry and a dummy occupy the same bytes (`len > L` or non-zero padding is rejected); `hint = Trunc8(SHA256("meshapp.v1.skdm.hint" ‖ recipient.ed ‖ LE32(chain_id)))`; entries ordered by hint. Each `ct` is the recipient's pairwise encryption (through `PairwiseManager`, so a first message leaves the session Ready) of a kind-0x36 post whose body is `group_id [16] ‖ skdm bytes16 ‖ epoch_seed [32] ‖ next_ctr u32` (the sender's current recognition counter on the new chain: the recipient's 50-tag window starts there, so a member reached after the sender issued more than a window of cells, or a re-sent SKDM, still recognises the chain; `GroupState::skdm_entry_for`). Sent under the sender's **old** chain tags as one C1 object; a sender with no old chain, or one whose bundle exceeds one 64 kB object, sends per-recipient pairwise SKDMs |
| Receipt | 0x05/0x35 | `conv [16] ‖ up_to_obj_seq u32 ‖ n_gaps u8 ‖ (from u32 ‖ to u32)×n` (cumulative; gaps feed "N messages from X not here yet") |
| Sealed first flight | 0x03 | `e_pub [32]` clear ‖ AEAD(`k_seal`, `PreKeySignalMessage bytes ‖ recog_seed [32] ‖ envelope_pub [32]`) |

Admission: every Ed25519 key entering a KeyCard, roster op, ballot or vote-open passes `admit_public_key` (rejects small-order and non-canonical) before any signature check.

## 3. Roster DAG

- **Node** = a roster op (§2.4); edges = `heads`. `closure(H)` = all ops reachable from the head set `H`, computed only from ops the phone holds; ops with missing heads are parked and their parents requested through summary vectors (roster ops are never evicted until superseded, D3 M1).
- **`members(H)`** is a pure function of the op *set* `closure(H)` (D2/§3-3): later ops never change it. Reference implementation `roster::eval::evaluate` recomputes from scratch (≤50 members, ops ≤ a few thousand: milliseconds); the UI cache is keyed by the sorted head-set hash.

**Validity, evaluated against each op's own causal past `P(op) = closure(op.heads)`:**

| Op | Valid iff |
|---|---|
| Genesis | well-formed; `n_admins ≥ 1`; exactly one genesis per group (the group id *is* its hash) |
| Remove(m) | author ∈ `admins(P)`; m ∉ `admins(P)` (admins are demoted via AdminSet first, so there are no concurrent power changes, D3 Q5); **monotone**: stands even if the author is removed/demoted concurrently |
| Add(m) / re-add | author ∈ `admins(P)`; endorsement.admin ∈ `admins(P)`; endorsement.admin ≠ author; `in_person` set on at least one of the two; endorsement signature valid; m ∉ `admins(P)`; every valid `Remove(m)` in the *evaluated set* `S = closure(H)` is in `P` (remove-wins / re-add only causally after the removal); **voided** if `S` holds a valid Remove or demotion of the author or of the endorsing admin that is not in `P` ("Changed after reconnect", D3 M3) |
| AdminSet(v) | `v = 1 + (number of applied AdminSet ops in P)`; cosigners ⊆ `admins(P)`, author ∈ cosigners; `|cosigners| > |admins(P)| / 2` (strict majority); **equivocation**: an admin cosigning two different AdminSet ops with the same `v` anywhere in `S` voids both (D3 M2); two concurrent valid ops at the same `v` imply overlapping cosigners, so this rule also resolves them |
| Setting | author ∈ `admins(P)`; last-writer by op-hash order among concurrent settings (display-only switches) |
| any | sender-chain check: two objects with the same `(author, author_seq)` and different hashes → both ignored, event `Equivocation(author)`; signature over all bytes with the domain prefix |

`admins(P)` = genesis admins, then each applied AdminSet in `P` in version order. `members(S)` = genesis members ∪ {m : ∃ valid Add(m) ∈ S such that every valid Remove(m) ∈ S is in P(Add)} minus nothing else (admins are members). Memoise `admins(P)` per op hash.

**Posts by removed members (§3-4, deterministic, no timers):** a decrypted post by X carries `roster_heads`; if `closure(roster_heads)` contains a valid `Remove(X)` → reject everywhere (hidden; its msg_id is added to the "rejected" set so summary vectors stop re-offering it). Otherwise keep; if X ∉ `members(current heads)`, collapse and label "Sent by [X], who was removed". If the post's heads are unresolved, display with the current label and re-evaluate when the ops arrive (final state is identical on all phones). Relay penalties (L1 drop + link score) apply only to bad signatures, never-members, or posts whose heads include the author's removal; they key on the envelope sender's link, so re-senders of M's ballots are never penalised.

**Sender Key rotation (`roster/rotation.rs`):** `pending_rotation` is set when (a) any valid Remove enters the DAG (of anyone), (b) `powered_on − last_rotation ≥ 24 h` (B: 12 h + 0–10 min). Before the next post, `GroupState::rotate(txn)`: `new_sender_chain` → new `RecogEpoch` seed (k_tag, k_hdr) → SKDM bundle to `members(current)` \ {self} under the old chain's tags as a C1 object (or pairwise if no old chain), then the post under the new chain. Coalesced to one rotation per post. Receivers keep previous chains (libsignal: 5 states per sender; our `RecogChain` keeps the matching epochs) until the gap closes (receipt says no gaps) or the cap evicts; A build has no time purge. "Waiting · can't read yet" is shown for cells whose tag is unrecognised but whose later SKDM makes them readable: unrecognised cells stay in the DTN store and are re-tried whenever a chain/epoch is registered.

**Vote-open fork rule hook:** `SenderChainIndex` records, per author, the heads cited by each identity-signed object; a VoteOpen with heads `H_v` is invalid everywhere if some earlier object by the same author cites a head not in `closure(H_v)`.

## 4. Tasks

- **Ops**: Create, Take, Release, Done (§2.4), inside group posts, author bound by chain. Each task is its own small DAG via `task_heads`. An op's identity is its author-bound hash (§2.4 `task_op_hash(author, post plaintext)`): the node computes it from the chain owner of the decrypted post, so byte-identical ops from two members are two ops, a repeated hash is one op, and every head cited in `task_heads` resolves to exactly one op.
- **Fold** (`tasks/state.rs`): walk ops in topological order (ties by op hash). Claims = Takes without a later-or-concurrent Release by the same author. A Take with another live Take in its causal past is a *second claim* ("2 people on this"), not a conflict, whatever else it is concurrent with (two helpers who each saw the holder but not each other are "3 people on this", not rivals). The remaining live Takes saw no live Take, so they are pairwise concurrent (neither in the other's past); two or more of them form the conflict set. Done by any current claimant → Done (idempotent; concurrent Dones fine). Create may be issued by any member (OWNER DECISION default; a `Setting` can restrict to admins later).
- **UI states** (`DeliveryState`-style enum, hollow unless Done): `Open`; `TakenByYou·Provisional` while `reach_set(t_take)` (members synced in the last 10 min of powered-on time) is the only evidence; `TakenByYou·NoConflictSeen` once a member outside `reach_set(t_take)` reports holding our Take (its summary vector contains the Take's msg_id); `Conflict{other}` after a merge reveals a concurrent Take; `Done`.
- **Tie-break** (`tasks/tiebreak.rs`): for the conflict set `C` of concurrent Take ops, `R = SHA256(sorted author-bound op hashes of C)` (one hash per rival even when their Take posts are byte-identical), `score(k) = SHA256("meshapp.v1.tiebreak" ‖ task_id ‖ R ‖ k)`, lowest score wins. Not grindable by key choice (keys are fixed in the roster before any task exists, and each task re-randomises the ranking); not grindable by the task creator (R is unknown at creation); a taker who hides a seen rival Take to fake concurrency has only the few bits of freedom in its own heads choice, and that lie is also the cheaper "just take it anyway" path, which the UI handles as "2 people on this". Loser is offered "Keep helping" (emits nothing; state shows 2 people) or "Release".
- **Needs someone** (`tasks/pool.rs`): a task with ≥1 holder where `last_heard[holder]` (any cell recognised from that member's chains or any identity-signed object) is older than `NEEDS_SOMEONE_AFTER_ASM = 6 h` of powered-on time since the later of the Take and that holder's last evidence (docs/decisions-assumed.md row 7: assumption, tune in field tests); no op is emitted; shows "not heard from since HH:MM" (wall clock of `last_heard`, display only). B build: label "No update from [name] since 14:05 (phone may be closed)". Anyone may Take from the pool (second claim).

## 5. Votes (P1 types, specified now)

- **VoteOpen** (§2.4): opener ∈ `members(H)`, else invalid; `N` must equal `|members(H)|` and `T` must equal `⌊(N+f)/2⌋+1` recomputed locally, else invalid; `f ≥ F_MIN` (A: 1, B: 2). Same `(opener, author_seq)` with different content → vote void with the alert icon (sender-chain equivocation). Same opener + same question hash while another is open → "Duplicate vote" flag. Fork rule from §3 → invalid. `H` unresolved → "Waiting · needs this vote's member list", never Final.
- **Ballot**: standalone Ed25519 object; accepted iff `key ∈ members(H)` and strict signature; identical payloads collapse; two distinct payloads from one key → that key is **void** (keep ≤2 payloads as proof, drop further ones); holders re-send ballots after rejoin (they are ordinary DTN cells; anti-entropy does it).
- **Tally** (`votes/tally.rs`, pure): `s_X` = distinct roster keys holding ≥1 valid ballot for X (equivocators included); `void` = keys with ≥2 payloads; `heard` = keys with any ballot; displayed tally excludes equivocators.
  - `Final(X)` iff `s_X ≥ T` AND `void ≤ 2T−N−1` AND (`|options| > 2` ⇒ `s_X > N/2`, already implied by T but checked).
  - `Disputed{members}` iff `void > 2T−N−1`.
  - else `heard < ⌊N/2⌋+1` ⇒ `LeadingProvisional{k_not_heard}`; else `ResultProvisional{needs T}`.
  - `Void` if the opening is void/forked; `ChangedAfterReconnect` flag when a provisional lead changes.
  - Annotations only (never change counts): "N members as of [date]", "Not on this vote's list (joined after it opened)", "Includes k ballot(s) from people removed after this vote opened" with "Propose a new vote" when k ≥ gap of the top two.
- Nothing reads a clock; "Target 18:00 · late votes still count" is a label field in the opening's UI metadata, not in the signed body (keeps the protocol clock-free; if the owner wants it signed it goes in a `str16` display field that the tally ignores).

## 6. Storage

- SQLite via `rusqlite` (`bundled`), WAL, `secure_delete=ON`, `foreign_keys=ON`. Secret tables are encrypted at the application layer with XChaCha20-Poly1305 under a 32-B `db_key` supplied at `MeshNode::new`; non-secret tables hold only ciphertext cells, hashes and counters. (SQLCipher is the alternative if the audit prefers whole-file encryption; the `Db` API hides the choice.)
- **Key handling per platform**: Android: `db_key` generated once, wrapped by an AES-GCM Keystore key (`setUnlockedDeviceRequired(false)` in the A build; `KeyGenParameterSpec` without user-auth), stored in `noBackupFilesDir`; DB file also under `noBackupFilesDir`; manifest `android:allowBackup="false"` + `dataExtractionRules` excluding everything. iOS: `db_key` in Keychain with `kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly` (A build); DB and cell store files with `NSFileProtectionCompleteUntilFirstUserAuthentication` and `isExcludedFromBackup = true`. B build later swaps to `WhenUnlocked`/`setUnlockedDeviceRequired(true)` (Android 15+).
- **Tables** (`schema.sql`): `identity`, `contacts` (secret), `pins`, `groups`, `roster_ops(op_hash PK, group, author, seq, bytes, parsed_ok, void_reason)`, `roster_heads`, `sender_chain(author, seq, hash, heads)`, `chains(chain_id, group, owner, recog_epochs secret)`, `backend_kv` (secret: sessions, sender-key records, prekeys), `cells(msg_id PK, bytes[512], class, ttl, lane, first_seen_mono, size_class, rejected)`, `dedupe(msg_id PK, expires_mono)`, `objects(obj_id, conv, author, kind, plaintext secret, delivery_state, obj_seq)`, `reassembly(chain, obj_seq, frag_idx, …)`, `tasks`, `task_ops`, `votes`, `ballots`, `receipts`, `link_scores`, `meta(powered_on_ms, last_rotation, skeleton_data_version)`.
- **Atomic commit**: `node::inbound` decrypts inside `Txn`: backend store writes (overlay) + `objects` insert + message-key deletion (libsignal consumes the key in the session record that is saved in the same txn) → `commit` → only then the receipt is scheduled and `Delivered` can appear at the sender. Crash between decrypt and commit replays the cell from `cells` (the ratchet state was not advanced on disk).
- **Dedupe** retention = `max(MSG_RETENTION, KEY_RETENTION_ASM=7 d) + 72 h` of powered-on time (A: chats unbounded → the cache is bounded by key retention + 72 h = 10 d; B: 72 h + 7 d). Deleting a message for retention also deletes its `objects` row and, for undelivered counters, the skipped keys (app-side list of consumed counters; libsignal's own cap of 2000 bounds the rest).
- **Powered-on meter**: `meta.powered_on_ms` += `mono_ms` delta on every tick; all expiries (dedupe, SPK deletion, rotation interval, Needs-someone) use it.

## 7. Transport

```rust
pub struct LinkId(pub u32);
pub enum LinkAuth { Anonymous, VerifiedContact(IdentityKey) }   // set after Noise + optional ContactProof
pub struct LinkInfo { pub role: Role /*Central|Peripheral*/, pub peer_kind_hint: PeerKind /*Unknown|AppleOverflow|AppUuid|Station*/, pub mtu_hint: u16 }
pub trait Link {
    fn id(&self) -> LinkId;
    fn info(&self) -> &LinkInfo;
    fn auth(&self) -> LinkAuth;
    fn send(&mut self, frame_bytes: &[u8]) -> Result<(), LinkError>;   // whole Noise frame; Err(Busy) = backpressure
    fn recv(&mut self) -> Option<Vec<u8>>;                             // next complete Noise frame
    fn lane_for(&self, validated: Validation) -> Lane;                  // L1 if validated, L2 if auth is VerifiedContact, else L3
}
```
The core never blocks: `send` enqueues; the platform drains via `poll_outbound`. Order per link is guaranteed by GATT; a lost chunk means the link is torn down and re-handshaked (new Noise session).

| Impl | Where | Behaviour |
|---|---|---|
| `SimLink` | `sim/meshsim/radio.rs` | Discrete-event radio: per-pair latency (default 40 ms + size/throughput at 15 kB/s), loss (0.5 %/frame → link drop), partition schedule (`t → connectivity graph`), link-slot caps (7 per Android-14 node), locked-iPhone wake model: node asleep → woken only by an inbound connect from an Android/station/foreground-iPhone, awake 10 s, link dropped at wake end, never links to another locked iPhone; foreground windows from a usage model. |
| `AndroidLink` | `android/ble` (`LinkChannel.kt`) → core `PlatformLink` | One GATT service `MESH_SVC_UUID` with TX char (WRITE_NO_RESPONSE; central→peripheral) and RX char (NOTIFY, CCCD; peripheral→central). `requestMtu(517)` by the central; chunk at MTU−3; one write in flight (wait `onCharacteristicWrite`); notifications paced by `onNotificationSent`. Both roles concurrently; scan filters: `MESH_SVC_UUID` + manufacturer 0x004C with type-0x01 overflow mask (locked iPhones) → connect + GATT probe. Extra `AdvertisingSet` for B-build markers (128-bit UUID = nonce64 ‖ MAC64 under per-chain tag keys, rotated per period, from `MeshNode.current_markers()`). ≤5 `startScan` per 30 s; non-connectable advertising when `SlotPolicy` says inbound slots are full. When the peer's TX char is write-with-response (iOS), use `WRITE_TYPE_DEFAULT`. |
| `IosLink` | `ios/MeshApp/BleManager.swift` | `CBPeripheralManager` service with the same UUIDs; TX char is `.write` (with response) so over-quota writes answer `CBATTError.insufficientResources` (D4); RX via `updateValue` notifications with `peripheralManagerIsReady(toUpdateSubscribers:)` pacing. `CBCentralManager` scanning for `MESH_SVC_UUID` only (background scans are UUID-filtered); pending connects refreshed each wake, cancelled after 15 min (Android RPA rotation). Restoration keys `CBCentralManagerOptionRestoreIdentifierKey`/`CBPeripheralManagerOptionRestoreIdentifierKey`; managers recreated in `application(_:didFinishLaunchingWithOptions:)` when launched with `bluetoothCentrals`/`bluetoothPeripherals` keys. Caveat recorded in code and docs: in the background the service UUID moves to the Apple overflow area; Android sees only 0x004C and must probe; local name is not advertised; B markers alternate with the fixed UUID only while foregrounded (unmeasured). |

Link lifecycle in core: `link_up` → Noise NN handshake (initiator = central) → `Hello` → optional `ContactProof` → summary exchange (`dtn/sync`) → scheduled push; `Throttle` frames carry backpressure; L1 validation happens per cell when the tag is recognised and the inner signature/roster check passes.

## 8. Android app

**Gradle modules** (`android/settings.gradle.kts`): `:app` (Compose UI), `:core-bindings` (uniffi Kotlin + `jniLibs/` from `cargo ndk -t arm64-v8a -t x86_64`), `:ble`, `:service`, `:platform`, `:logic` (pure JVM: chunker, delivery-state mapping, marker aggregation; tested without the SDK). minSdk 31 (A build = managed devices; drops the legacy `ACCESS_FINE_LOCATION` scan branch), targetSdk 35.

**Key Kotlin files**
- `app/src/main/java/.../MeshApplication.kt` (creates `MeshNode` with `KeystoreDbKey`), `ui/MainScaffold.kt` (3 tabs + `StatusStrip.kt`), `ui/chats/{ChatsScreen,ThreadScreen,DeliveryBadge}.kt`, `ui/tasks/{TasksScreen,TaskCard,NeedsSomeoneSection}.kt`, `ui/people/{PeopleScreen,PersonDetail,SameNameMarker,VerifyScreen}.kt` (CameraX + ML Kit/ZXing QR; runs `PairingSession` over a direct link to the scanned device), `ui/onboarding/{NameStep,VerifyOneStep,PermissionsStep}.kt`, `ui/tips/Tips.kt` (Passed along, Provisional, Conflict, same-name), `vm/NodeViewModel.kt` (event → StateFlow bridge).
- `ble/BleManager.kt`, `GattServerHost.kt`, `CentralConnector.kt`, `Scanner.kt` (filters, 5/30 s cap), `Advertiser.kt` (app-UUID set + marker set), `LinkChannel.kt` (byte stream ↔ `feed`/`poll_outbound`), `Uuids.kt`.
- `service/MeshForegroundService.kt` (`foregroundServiceType="connectedDevice"`, notification "Passing messages along · N nearby", relay loop on the service thread, `ApplicationExitInfo` check → "Passing along stopped at HH:MM"), `BootReceiver.kt` (`BOOT_COMPLETED` → start only if the user had enabled it), `BatteryMonitor.kt` (`ACTION_BATTERY_CHANGED` → `node.set_power(level, charging)`; below 30 % and not charging the core stops forwarding others' cells), `ServiceController.kt`.
- `platform/KeystoreDbKey.kt`, `FileProtection.kt`, `MonotonicClock.kt` (`SystemClock.elapsedRealtime`).

**Permissions** (manifest + `PermissionsStep`): API 31+: `BLUETOOTH_SCAN` (`usesPermissionFlags="neverForLocation"`), `BLUETOOTH_ADVERTISE`, `BLUETOOTH_CONNECT`; API 33+: `POST_NOTIFICATIONS`; API 34+: `FOREGROUND_SERVICE` + `FOREGROUND_SERVICE_CONNECTED_DEVICE`; `RECEIVE_BOOT_COMPLETED`; `CAMERA` (requested just-in-time on the verify step). One plain sentence each (B §6).

**FGS lifecycle**: user-started from the onboarding step or People → Settings; survives app swipe; restarted at boot if enabled; stops relaying for others under the battery floor but keeps own traffic; link to battery-settings intent (no `REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` unless Play allows).

**Station mode**: same APK, `station=true` in managed config → `NodeConfig.station`, no identity, L3 cap = whole budget, control before text, never media, TTL ≤ 12 h buffer, health ping signed by a device key.

## 9. iOS app

- `ios/MeshCore` SwiftPM package: `meshcore.xcframework` (`aarch64-apple-ios`, `aarch64-apple-ios-sim`) + generated `meshcore.swift`.
- `ios/MeshApp/`: `MeshAppApp.swift` (creates managers in the app delegate for background launch), `NodeController.swift` (owns `MeshNode`, event bridge to `@Observable` state), `BleManager.swift` (central + peripheral, restoration, wake-refresh of pending connects), `LinkChannel.swift`, `KeychainDbKey.swift`, `Views/{MainTabView,StatusStripView,ChatsView,ThreadView,TasksView,PeopleView,VerifyView (AVCaptureSession QR),OnboardingView,BadgeView,SameNameMarkerView,TipSheet}.swift`.
- **Info.plist**: `UIBackgroundModes = [bluetooth-central, bluetooth-peripheral]`, `NSBluetoothAlwaysUsageDescription`, `NSCameraUsageDescription`, `UIApplicationSceneManifest` (restore IDs persisted for scene-based apps), `ITSAppUsesNonExemptEncryption = YES` with export compliance notes.
- **Onboarding copy (verbatim, B §6)**: permissions step: "Sends and receives fully while open. In your pocket it works slowly when an Android phone, a relay station or an iPhone with the app open is nearby; two pocketed iPhones rarely reach each other. Force-quitting the app, turning off Bluetooth or turning on Airplane Mode stops it until you reopen it." Scope line once (A build): "Not designed for places where authorities may monitor or seize phones." Residual-risk line: "People nearby can tell this app is running and roughly where a message started; what you send, and to whom, is hidden from phones that pass it along."
- Vocabulary table (B §7) shipped as one `Strings.swift`/`strings.xml` pair generated from `docs/vocabulary.csv` so both platforms use identical terms.

## 10. Test plan

**Unit tests (one file per module, `cargo test`)**: codec round-trip + rejection of trailing/overlong input; cell build/parse incl. TTL draw ∈ {6,7,8}, `msg_id` excludes TTL, token slot rejection; recog tag windows and epoch rollover; seal/unseal with ≤3 keys and wrong-key rejection; Noise link round trip and `ContactProof` matching; ed25519 strict (vectors: small-order keys, non-canonical R/s/A are rejected; identical verdict table checked into `tests/vectors/ed25519_strict.json` and re-run through the Kotlin JVM smoke test); keycard binding; sender-chain equivocation; roster rules (every row of the §3 table has a positive and a negative test, plus the S3 walkthrough as a fixture); rotation triggers; SKDM atomic check (mismatched x25519 → no store write); pairwise crossed initiation; SPK deletion rule on powered-on time; task fold/tie-break/pool; same-name normalisation (confusables, casefold, Default_Ignorable); dedupe expiry; summary vectors; limits/ramp/lanes/scheduler order; buffer eviction order; storage atomicity (kill between decrypt and commit via a fault-injection hook → replay yields exactly one delivery).

**Property tests (proptest)**
- `roster_convergence`: random op DAGs from honest and malicious admins (omitted heads, concurrent adds/removes, equivocating AdminSets, re-adds); for 20 random insertion orders with parking, `members(H)` and`admins(H)` are identical across orders and equal the batch `evaluate`; invariants: removal monotone, re-add only causally after the removal, concurrent add by a removed/demoted admin void, admin set never changes without a strict majority.
- `final_rule`: N ∈ 3..=60, f ∈ 1..=3, keys assigned none / one option / two options; with equivocators ≤ 2T−N−1, any two subsets of the ballot set that both yield Final agree on X; adding ballots to a Final(X) set yields Final(X) or Disputed, never Final(Y) or Provisional; with equivocators > 2T−N−1 the full set is Disputed; the brief's thresholds (7/12, 8/13, 12/21, 21/40; f=2: 8/12, 8/13, 12/21, 22/40) as fixed cases.
- `task_tiebreak`: same winner for any permutation of the conflict set; changing any op hash re-randomises; no key wins more than 1/|C| + ε over many tasks.
- `cell_roundtrip`: arbitrary objects ≤ 64 kB fragment/reassemble through random cell orderings and duplicates.

**Fuzz targets (`crates/meshcore-fuzz`, cargo-fuzz, nightly)**: `cell_parse`, `frame_parse`, `noise_stream` (chunked framer), `roster_op_decode`, `inner_decode`, `summary_decode`, `qr_payload`. Each also runs on stable as a proptest with the same harness so CI without nightly still exercises them.

**Simulator scenarios (`sim/meshsim/scenarios/*.toml`, `cargo run -p meshsim -- run s1.toml --seed N`)**, each run with ≥20 seeds; pass criteria map to the brief's criteria 1 (correct across split/rejoin), 3 (honest provisional vs final), 5 (threat model):

| Scenario | Setup | Pass criteria |
|---|---|---|
| S0 smoke | 3 nodes, no partition | every text Delivered; receipts; last-sync set |
| S1 | 40 users (20 Android, 20 iPhone with 30 % foreground duty), one group of 40 + 10 pairwise verified pairs, 3 fragments 18/14/8 for 6 h, then rejoin; posts every 2 min per node; one removal in F18 at T+1 h | (a) within each fragment every member decrypts every post from its fragment; (b) after rejoin every honest node holds every post from all fragments and decrypts it (no "can't read yet" left after the drain); (c) `members(H)` identical on all 40 for the common head; (d) C1 backlog drains within `DRAIN_ASM = 15 min` after the bridge (D4 figure ≈ 6 min; reported, threshold soft); (e) no object delivered twice to the UI; (f) no solid state (Delivered/Done) ever reverts in the trace; (g) message keys for delivered objects are absent from the store |
| S3 | S1 topology; insider Mo in F14 runs the `adversary` hooks (keeps keys after removal, posts after removal with and without the removal in heads, signs concurrent Adds of 3 sock-puppet keys, withholds 20 DMs for 38 h, replays old cells) | (a) no post made by an F18 member after it learned of the removal is decryptable by Mo (adversary node records decrypt attempts); (b) after the merge Mo's sock-puppet Adds are void everywhere ("Changed after reconnect" once, then stable); (c) Mo's posts whose heads include his removal are hidden everywhere, his other split-time posts shown with "Sent by Mo, who was removed" everywhere; (d) replays are dropped by dedupe/AEAD, never redelivered; (e) honest split-time posts, Takes and ballots all decrypt after the merge; (f) the withheld DMs decrypt at h40 with an age label |
| S4 | 21 members, vote opened at T, split 11/10 at T+10 min, per-node wall clocks offset uniformly in ±20 min, courier Z carries a bundle to F1 with a Yes ballot after sending No in F2; variants: honest courier (11–10), lopsided (14–7) | (a) no node shows Final while split in the 11/10 and 11–10 variants; (b) after merge all 21 show "Y10 N10 · 1 void · Provisional" (Z void) in the courier variant and "11–10 Provisional" in the honest variant; (c) lopsided variant: Final on every node after merge and never changes after; (d) a late ballot from a node whose clock is +25 min counts; (e) the protocol trace contains zero wall-clock reads (enforced by the `Clock` trait: the sim's protocol clock is a counter and `wall_for_display` is instrumented) |
| S-flood | S1 topology plus one modified node injecting 600 junk cells/min at a hub | honest C0/C1 latency stays under 2× baseline; junk never exceeds the L3 cap; penalised link is throttled; no honest cell evicted by junk |

Invariant checker (`sim/meshsim/assert.rs`) runs over every scenario: hollow/solid monotonicity, identical `members(H)` per head set across nodes, no duplicate UI delivery, no Final disagreement, no decrypt of post-removal traffic by a removed node, bounded buffers.

**What runs in this Linux container with no phones**
- Runs: `cargo build/test/clippy` for `meshcore` + `meshsim` on x86_64 (libsignal git dependency fetched through the proxy; if github.com is blocked the `backend-vodozemac` feature from crates.io is the CI fallback, and the plan says so in `docs/testing.md`); all unit, property and simulator checks above; `cargo fuzz` if `rustup toolchain install nightly` succeeds, otherwise the stable proptest harness; `uniffi-bindgen` generation for Kotlin and Swift; a pure-JVM Gradle module (`android/logic` plus a `jvm-smoke` test module) that compiles the generated Kotlin against JNA and drives two in-process `MeshNode`s over a byte pipe using the Linux `.so` (a real end-to-end check of the uniffi surface and callback threading); `gradle :logic:test`.
- Attempted, honestly reported: Android `assembleDebug` and `testDebugUnitTest` only if `sdkmanager` can download the platform and build-tools through the proxy (one probe command; if it fails the Android modules are CI-only and the plan records that no Android compile happened here).
- Cannot run here: any BLE behaviour (scan, advertise, MTU, slots, wakes, RPA rotation), FGS lifecycle and OEM killers, battery, iOS compilation (needs Xcode/macOS), App Store/Play policy. These live in `docs/testing.md` as a device checklist tied to gate 2 (the measured spike) and each milestone's definition of done separates "container-verified" from "device-verified"; no milestone claims device behaviour without a device log attached.

## 11. Milestones

| M | Scope | Definition of done |
|---|---|---|
| M0 Spike (1 wk) | Workspace, `codec`, `crypto/ed25519`, backend trait, `LibsignalBackend` against the pinned rev: PQXDH session both ways, sender-key create/rotate/decrypt-old-chain, store traits over `Txn` | `cargo test` green; a written note in `docs/` on whether libsignal's sender-key API supports rotation without forking (if not: decision between a small fork and vodozemac before M1) |
| M1a Wire + DTN + sim radio (2 wk) | `wire/*`, `dtn/*`, `relay/*`, `transport/*`, `meshsim` radio/partition/wake model, S0 | S0 passes; S1 topology floods opaque cells with the drain metric reported; fuzz targets exist and run 10 min each clean |
| M1b Identity + groups + roster (3 wk) | `crypto/keycard`, `pairwise/*`, `groups/*`, `objects/*`, `roster/*`, `people/*`, storage schema, atomic commit | S1 and S3 pass all criteria for 20 seeds; `roster_convergence` proptest 10k cases; §3 rule table fully covered |
| M1c Tasks + vote types (1 wk) | `tasks/*`, `votes/*` (types + tally, no UI) | S4 passes all variants; `final_rule` proptest 10k cases; tie-break proptest |
| M1d Hardening (1 wk) | S-flood, limits tuning, invariant checker, `docs/wire-format.md`, `roster-dag.md`, `votes.md`, `audit-scope.md` | All scenarios pass under `--seeds 50`; docs match code (a doc test extracts the layouts from constants) |
| **M1 gate** | | Core + simulator independently valuable: the audit package (docs + sim traces) can go to the §5.8 reviewers before any UI exists |
| M2 Bindings (1 wk) | `api/*`, uniffi Kotlin/Swift, JVM smoke test, XCFramework build script | JVM two-node smoke passes in CI; Swift bindings generate; `tools/build-ios.sh` runs on a macOS runner |
| M3 Android (4 wk) | `:ble`, `:service`, `:platform`, `:app` screens, onboarding, QR verify, FGS, boot restart, battery floor | 2-phone and then 5-phone bench test logs: verify flow completes, text Delivered, group Take conflict shown after a staged split, FGS survives swipe and reboot; `assembleDebug` in CI |
| M4 iOS (4 wk) | `MeshCore` package, `BleManager`, screens, onboarding copy, restoration | Bench logs: iPhone↔Android exchange foreground; locked iPhone woken by an Android write and drains a backlog; restoration after a background relaunch |
| M5 Spike + audit prep (2 wk) | Gate-2 measurements, cell size decision, B-marker advertising set, station mode | Spike report with the brief's measures; `CELL_LEN` frozen; on-air format v1 tagged |

## 12. Risks and the top 5 ways this plan could be wrong

1. **libsignal's sender-key API cannot rotate or expose what the atomic check needs** (`remote_session_key`, rotation of a `SenderKeyRecord` without a fork). Cheap check: M0, one day against the pinned rev; fallback is a ≤50-line fork or vodozemac behind the same trait.
2. **512-B cells make the locked-iPhone bottleneck too slow** (padding plus per-cell tags; a 50-member rotation bundle is ~17 cells and a first-join bundle ~170). Cheap check: `meshsim` bytes-on-bottleneck for 512/768/1024 in M1a, then the gate-2 measurement; `CELL_LEN` is a single constant until the v1 freeze.
3. **The roster rules have a hole the tests do not reach** (Matrix-style state reset, or an admin-set race the strict-majority rule does not close). Cheap check: the `roster_convergence` proptest with adversarial generators, plus a 200-line Alloy/TLA+ model of `members(H)` in `docs/` before the audit.
4. **Bloom summary vectors miss or over-cost on 10-s wakes** (1 % false negatives per encounter accumulate for rarely-met peers; 8 kB filters eat the wake). Cheap check: S1 drain time and "never offered" counter in the sim; salt rotation per encounter and a smaller recent-window filter are the knobs.
5. **Licence and policy** (AGPL on the App Store; Play's view of `connectedDevice` for phone-to-phone). Cheap check: legal review in parallel with M0; CI builds both backends so the vodozemac path never rots; station-via-ADB stays as the Play fallback.

Also tracked: uniffi callback cost on the BLE hot path (mitigation: bytes in/out only, no per-cell callbacks); the Ed25519→X25519 KeyCard binding being misread by the audit as a second identity (documented in `docs/wire-format.md`); the "Taken by you · no conflict seen" heuristic overclaiming (it stays hollow and is comprehension-tested at gate 5); Android OEM killers (device checklist, not fixable in code).

### Critical Files for Implementation
- `meshapp/crates/meshcore/src/roster/eval.rs` — `members(H)`/`admins(H)` with the full validity table; the correctness core of S1 and S3
- `meshapp/crates/meshcore/src/wire/cell.rs` — the frozen 512-B on-air layout, TTL and `msg_id` rules
- `meshapp/crates/meshcore/src/groups/group.rs` — rotation, SKDM atomic identity check, post encode/decode with roster heads
- `meshapp/crates/meshcore/src/node/inbound.rs` — the relay/recipient pipeline where dedupe, lanes, recognition and atomic commit meet
- `meshapp/sim/meshsim/src/scenario.rs` — scenario loader and invariant assertions that make milestone 1 independently valuable