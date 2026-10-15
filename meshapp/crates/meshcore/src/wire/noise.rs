//! Per-hop link encryption: one Noise NN session per link
//! (docs/DESIGN.md §1 `wire/noise.rs`, §2.3, §7; BRIEF_v2 §5.5 "per-hop link
//! encryption (Noise NN)").
//!
//! Pattern [`PATTERN`] = `Noise_NN_25519_ChaChaPoly_BLAKE2s` through the
//! `snow` crate. NN has no static keys, so nothing on the air identifies
//! either phone (BRIEF_v2 §5.5 "no sender key, signature, group id or epoch
//! appears in the clear"; D4 metadata minimisation). A QR-verified contact
//! proves itself *inside* the session with a
//! [`ContactProof`](crate::wire::frame::ContactProof) keyed on the handshake
//! hash, which binds the proof to this very session (DESIGN §2.3;
//! [`contact_proof`]).
//!
//! # Layering (DESIGN §7)
//!
//! ```text
//! inbound:  radio bytes ─► transport::chunk::StreamFramer ─► NoiseLink::decrypt_frame
//!                       ─► wire::frame::LinkFrame decode
//! outbound: LinkFrame encode ─► NoiseLink::encrypt_frame ─► Link::send
//!                       (PlatformLink adds the u16 LE length prefix) ─► radio
//! ```
//!
//! The Noise session is owned by the node's link layer, not by the
//! [`Link`](crate::transport::Link): a link moves whole Noise messages, the
//! `NoiseLink` seals and opens them.
//!
//! # Handshake
//!
//! `-> e` ([`HANDSHAKE_MSG1_LEN`] = 32 bytes) from the initiator (the BLE
//! central: DESIGN §7 "initiator = central"), then `<- e, ee`
//! ([`HANDSHAKE_MSG2_LEN`] = 32 + 16-byte tag = 48 bytes) from the responder.
//! Neither carries a payload, and a message of any other length is rejected
//! before `snow` sees it: the protocol fixes both lengths, and this layer
//! does not lean on the dependency's own bounds checks. Drive it with
//! [`NoiseLink::write_handshake`] /
//! [`NoiseLink::read_handshake`] until [`NoiseLink::is_handshake_finished`],
//! then [`NoiseLink::into_transport`].
//!
//! # Randomness and determinism
//!
//! `snow` generates the ephemeral key from a CSPRNG it owns (its `Random`
//! trait, resolved once at build time and kept inside the handshake state),
//! so the caller's `&mut dyn CryptoRng` cannot be lent to it. The constructor
//! therefore draws a 32-byte seed from the caller's RNG and hands `snow` a
//! deterministic generator keyed on it (`SeededRandom`: HMAC-SHA256 in
//! counter mode, whose state is zeroised when the handshake state is
//! dropped, so nothing that could reproduce the ephemeral private key
//! outlives the handshake in this layer). A fixed seed gives a fixed
//! ephemeral key and hence byte-identical handshake messages (the
//! simulator's reproducibility rule); the platform passes the OS RNG for
//! real links. Everything after the seed is deterministic: same inputs,
//! same bytes.
//!
//! # Failure
//!
//! A rejected handshake message, a transport message that fails to
//! authenticate (corrupted, replayed, reordered, forged) or nonce exhaustion
//! returns [`NoiseError::Fatal`] and leaves the link dead: every later call
//! fails with `Fatal(Dead)`. The caller tears the link down and a new link
//! runs a new handshake (DESIGN §7 "a lost chunk means the link is torn down
//! and re-handshaked"); nothing is ever retried on the same session.

use core::fmt;

use rand::CryptoRng;
use snow::error::StateProblem;
use snow::params::{CipherChoice, DHChoice, HashChoice, NoiseParams};
use snow::resolvers::{CryptoResolver, DefaultResolver};
use snow::types::{Cipher, Dh, Hash, Random};
use snow::{Builder, HandshakeState, TransportState};
use subtle::ConstantTimeEq;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use crate::crypto::hash::hmac_sha256;
use crate::transport::link::Role;
use crate::wire::frame::{CONTACT_PROOF_LEN, NOISE_TAG_LEN};

// ---- constants --------------------------------------------------------------

/// The Noise protocol name (DESIGN §1 `wire/noise.rs`).
pub const PATTERN: &str = "Noise_NN_25519_ChaChaPoly_BLAKE2s";
/// Largest Noise message, handshake or transport (Noise spec §3: 65535).
pub const MAX_MESSAGE_LEN: usize = 65535;
/// Poly1305 tag a transport message appends (= [`NOISE_TAG_LEN`]).
pub const TAG_LEN: usize = NOISE_TAG_LEN;
/// Largest plaintext [`NoiseLink::encrypt_frame`] accepts: the message limit
/// minus the tag. Every link frame is far below it (DESIGN §2.2: a cell
/// frame is 516 bytes; the largest frame of all is a 4 kB summary chunk).
pub const MAX_PLAINTEXT_LEN: usize = MAX_MESSAGE_LEN - TAG_LEN;
/// BLAKE2s output: the handshake hash length.
pub const HANDSHAKE_HASH_LEN: usize = 32;
/// Initiator's `-> e`: one X25519 public key, no payload.
pub const HANDSHAKE_MSG1_LEN: usize = 32;
/// Responder's `<- e, ee`: one X25519 public key plus the tag over the
/// (empty) payload.
pub const HANDSHAKE_MSG2_LEN: usize = 32 + TAG_LEN;
/// Domain label of the L2 contact proof PRF (DESIGN §2.3 `"l2"`, versioned
/// like every other domain string in the crate).
pub const CONTACT_PROOF_LABEL: &[u8] = b"meshapp.v1.l2";

// ---- errors -------------------------------------------------------------------

/// Why a fatal error killed the link (see [`NoiseError::Fatal`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum Fatal {
    /// A handshake message was rejected: wrong length, invalid Diffie-Hellman
    /// input or a failed authentication tag.
    #[error("handshake message rejected")]
    Handshake,
    /// A transport message failed to decrypt: corrupted, replayed, reordered
    /// or forged (the tag did not verify), or shorter than a tag.
    #[error("decrypt failed")]
    Decrypt,
    /// 2^64 - 1 messages went one way on this session (Noise spec §5.1).
    #[error("nonce exhausted")]
    Exhausted,
    /// The RNG failed or `snow` reported an error this layer never expects
    /// with the fixed pattern; surfaced rather than panicked.
    #[error("internal noise failure")]
    Internal,
    /// An earlier fatal error already killed the link.
    #[error("link already dead")]
    Dead,
}

/// Errors of the Noise link layer. Only [`NoiseError::Fatal`] changes the
/// link's state; every other variant leaves it exactly as it was.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum NoiseError {
    /// A handshake call out of order: writing when it is the peer's turn,
    /// reading when it is ours, or any handshake call after the handshake
    /// finished (including on a link already in transport mode).
    #[error("noise handshake call out of order")]
    OutOfOrder,
    /// [`NoiseLink::into_transport`], [`NoiseLink::encrypt_frame`] or
    /// [`NoiseLink::decrypt_frame`] before the handshake finished.
    #[error("noise handshake not finished")]
    HandshakeNotFinished,
    /// A plaintext longer than [`MAX_PLAINTEXT_LEN`] or a ciphertext longer
    /// than [`MAX_MESSAGE_LEN`]; the message was not processed.
    #[error("noise message of {0} bytes exceeds the 65535-byte limit")]
    TooLarge(usize),
    /// The link is dead and must be torn down (module doc "Failure").
    #[error("noise fatal: {0}")]
    Fatal(Fatal),
}

impl NoiseError {
    /// `true` for the errors after which the link must be torn down.
    pub fn is_fatal(&self) -> bool {
        matches!(self, NoiseError::Fatal(_))
    }
}

// ---- RNG plumbing -------------------------------------------------------------

/// Domain label of the handshake generator (module doc "Randomness and
/// determinism"), versioned like every other domain string in the crate.
const RNG_LABEL: &[u8] = b"meshapp.v1.noise-rng";
/// One HMAC-SHA256 output: the generator's block size.
const RNG_BLOCK_LEN: usize = 32;

/// `snow`'s RNG for one session, keyed on the seed the caller's RNG
/// supplied (module doc "Randomness and determinism"). Output block `i` is
/// `HMAC-SHA256(seed, RNG_LABEL ‖ 0x00 ‖ LE64(i))`: a PRF under a uniformly
/// random key, i.e. a counter-mode DRBG. `snow` draws exactly one block,
/// the NN ephemeral private key. Seed and counter are zeroised on drop and
/// each output block is wiped once copied out, so dropping the handshake
/// state leaves no copy of the ephemeral's source behind. (A stateful
/// generator such as `ChaCha20Rng` would keep its key *and* its last output
/// block, which is the ephemeral itself, in the heap unwiped.)
#[derive(Zeroize, ZeroizeOnDrop)]
struct SeededRandom {
    seed: [u8; 32],
    ctr: u64,
}

impl Random for SeededRandom {
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), snow::Error> {
        for chunk in dest.chunks_mut(RNG_BLOCK_LEN) {
            let block = Zeroizing::new(hmac_sha256(
                &self.seed,
                &[RNG_LABEL, &[0u8], &self.ctr.to_le_bytes()],
            ));
            // 2^64 blocks are never drawn; refusing beats wrapping onto a
            // block already handed out.
            self.ctr = self.ctr.checked_add(1).ok_or(snow::Error::Rng)?;
            let src = block.get(..chunk.len()).ok_or(snow::Error::Rng)?;
            chunk.copy_from_slice(src);
        }
        Ok(())
    }
}

/// `snow`'s default primitives with the RNG replaced by `SeededRandom`.
/// The seed is zeroised when the resolver is dropped (right after the
/// handshake state is built).
struct SeededResolver {
    seed: Zeroizing<[u8; 32]>,
}

impl CryptoResolver for SeededResolver {
    fn resolve_rng(&self) -> Option<Box<dyn Random>> {
        Some(Box::new(SeededRandom {
            seed: *self.seed,
            ctr: 0,
        }))
    }

    fn resolve_dh(&self, choice: &DHChoice) -> Option<Box<dyn Dh>> {
        DefaultResolver.resolve_dh(choice)
    }

    fn resolve_hash(&self, choice: &HashChoice) -> Option<Box<dyn Hash>> {
        DefaultResolver.resolve_hash(choice)
    }

    fn resolve_cipher(&self, choice: &CipherChoice) -> Option<Box<dyn Cipher>> {
        DefaultResolver.resolve_cipher(choice)
    }
}

// ---- the link -----------------------------------------------------------------

enum Stage {
    /// Boxed: snow's handshake state is ~850 bytes and lives only until
    /// `into_transport`, while the transport state is small.
    Handshake(Box<HandshakeState>),
    Transport(TransportState),
    Dead,
}

/// One Noise NN session: the handshake and, after
/// [`into_transport`](NoiseLink::into_transport), the transport cipher
/// states for one link (DESIGN §1 `NoiseLink::{initiator,responder}`).
pub struct NoiseLink {
    stage: Stage,
    initiator: bool,
    /// The transcript hash `h` so far; final once the handshake finished.
    hash: [u8; HANDSHAKE_HASH_LEN],
}

impl fmt::Debug for NoiseLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self.stage {
            Stage::Handshake(_) => "handshake",
            Stage::Transport(_) => "transport",
            Stage::Dead => "dead",
        };
        f.debug_struct("NoiseLink")
            .field("stage", &stage)
            .field("initiator", &self.initiator)
            .finish_non_exhaustive()
    }
}

/// Copies `snow`'s handshake hash into a fixed array. BLAKE2s is 32 bytes,
/// so the length always matches; any other length would be a `snow` bug and
/// yields zeros rather than a panic.
fn hash32(h: &[u8]) -> [u8; HANDSHAKE_HASH_LEN] {
    let mut out = [0u8; HANDSHAKE_HASH_LEN];
    if let Some(src) = h.get(..HANDSHAKE_HASH_LEN) {
        out.copy_from_slice(src);
    }
    out
}

impl NoiseLink {
    /// The initiator side (the BLE central, DESIGN §7): it writes the first
    /// handshake message.
    pub fn initiator(rng: &mut dyn CryptoRng) -> Result<Self, NoiseError> {
        Self::build(rng, true)
    }

    /// The responder side (the BLE peripheral): it reads the first handshake
    /// message.
    pub fn responder(rng: &mut dyn CryptoRng) -> Result<Self, NoiseError> {
        Self::build(rng, false)
    }

    fn build(rng: &mut dyn CryptoRng, initiator: bool) -> Result<Self, NoiseError> {
        let params: NoiseParams = PATTERN
            .parse()
            .map_err(|_| NoiseError::Fatal(Fatal::Internal))?;
        let mut seed = Zeroizing::new([0u8; 32]);
        rng.fill_bytes(&mut *seed);
        let builder = Builder::with_resolver(params, Box::new(SeededResolver { seed }));
        let hs = if initiator {
            builder.build_initiator()
        } else {
            builder.build_responder()
        }
        .map_err(|_| NoiseError::Fatal(Fatal::Internal))?;
        let hash = hash32(hs.get_handshake_hash());
        Ok(NoiseLink {
            stage: Stage::Handshake(Box::new(hs)),
            initiator,
            hash,
        })
    }

    /// `true` on the side that wrote the first handshake message.
    pub fn is_initiator(&self) -> bool {
        self.initiator
    }

    /// `true` once both handshake messages were processed (also in transport
    /// mode); `false` on a dead link.
    pub fn is_handshake_finished(&self) -> bool {
        match &self.stage {
            Stage::Handshake(hs) => hs.is_handshake_finished(),
            Stage::Transport(_) => true,
            Stage::Dead => false,
        }
    }

    /// `true` after [`into_transport`](Self::into_transport) succeeded.
    pub fn is_transport(&self) -> bool {
        matches!(self.stage, Stage::Transport(_))
    }

    /// `true` after a [`NoiseError::Fatal`]: the link must be torn down.
    pub fn is_dead(&self) -> bool {
        matches!(self.stage, Stage::Dead)
    }

    /// `true` while it is this side's turn to call
    /// [`write_handshake`](Self::write_handshake); `false` once the handshake
    /// finished or the link died.
    pub fn is_my_turn(&self) -> bool {
        match &self.stage {
            Stage::Handshake(hs) => hs.is_my_turn() && !hs.is_handshake_finished(),
            Stage::Transport(_) | Stage::Dead => false,
        }
    }

    /// Kills the link and returns the error to hand back.
    fn kill(&mut self, why: Fatal) -> NoiseError {
        self.stage = Stage::Dead;
        NoiseError::Fatal(why)
    }

    /// Produces the next handshake message to send to the peer: 32 bytes
    /// for the initiator's first message, 48 for the responder's reply.
    pub fn write_handshake(&mut self) -> Result<Vec<u8>, NoiseError> {
        let hs = match &mut self.stage {
            Stage::Handshake(hs) => hs,
            Stage::Transport(_) => return Err(NoiseError::OutOfOrder),
            Stage::Dead => return Err(NoiseError::Fatal(Fatal::Dead)),
        };
        if hs.is_handshake_finished() || !hs.is_my_turn() {
            return Err(NoiseError::OutOfOrder);
        }
        // Both NN messages fit in 48 bytes; a larger buffer costs nothing and
        // keeps `snow` from reporting `Input` if a payload were ever added.
        let mut buf = [0u8; 2 * HANDSHAKE_MSG2_LEN];
        let written = hs.write_message(&[], &mut buf);
        match written {
            Ok(n) => {
                let hash = hash32(hs.get_handshake_hash());
                match buf.get(..n) {
                    Some(msg) => {
                        self.hash = hash;
                        Ok(msg.to_vec())
                    }
                    None => Err(self.kill(Fatal::Internal)),
                }
            }
            Err(snow::Error::State(
                StateProblem::NotTurnToWrite | StateProblem::HandshakeAlreadyFinished,
            )) => Err(NoiseError::OutOfOrder),
            Err(_) => Err(self.kill(Fatal::Internal)),
        }
    }

    /// Consumes the peer's handshake message. The responder expects exactly
    /// [`HANDSHAKE_MSG1_LEN`] bytes, the initiator exactly
    /// [`HANDSHAKE_MSG2_LEN`]; anything else, or a message `snow` rejects,
    /// is `Fatal(Handshake)` and kills the link.
    pub fn read_handshake(&mut self, msg: &[u8]) -> Result<(), NoiseError> {
        let expected = if self.initiator {
            HANDSHAKE_MSG2_LEN
        } else {
            HANDSHAKE_MSG1_LEN
        };
        let hs = match &mut self.stage {
            Stage::Handshake(hs) => hs,
            Stage::Transport(_) => return Err(NoiseError::OutOfOrder),
            Stage::Dead => return Err(NoiseError::Fatal(Fatal::Dead)),
        };
        if hs.is_handshake_finished() || hs.is_my_turn() {
            return Err(NoiseError::OutOfOrder);
        }
        if msg.len() != expected {
            return Err(self.kill(Fatal::Handshake));
        }
        // NN carries no handshake payload: with the exact length checked
        // above, the payload is empty (message 1) or just a tag (message 2).
        let mut payload = [0u8; 0];
        let read = hs.read_message(msg, &mut payload);
        match read {
            Ok(_) => {
                self.hash = hash32(hs.get_handshake_hash());
                Ok(())
            }
            Err(snow::Error::State(
                StateProblem::NotTurnToRead | StateProblem::HandshakeAlreadyFinished,
            )) => Err(NoiseError::OutOfOrder),
            Err(_) => Err(self.kill(Fatal::Handshake)),
        }
    }

    /// Switches a finished handshake to transport mode. Idempotent on a link
    /// already in transport mode; `HandshakeNotFinished` (and the link is
    /// consumed, which is what a caller would do anyway) if called early.
    pub fn into_transport(self) -> Result<Self, NoiseError> {
        let NoiseLink {
            stage,
            initiator,
            hash,
        } = self;
        match stage {
            Stage::Handshake(hs) => {
                if !hs.is_handshake_finished() {
                    return Err(NoiseError::HandshakeNotFinished);
                }
                let hash = hash32(hs.get_handshake_hash());
                let state = (*hs)
                    .into_transport_mode()
                    .map_err(|_| NoiseError::Fatal(Fatal::Internal))?;
                Ok(NoiseLink {
                    stage: Stage::Transport(state),
                    initiator,
                    hash,
                })
            }
            Stage::Transport(state) => Ok(NoiseLink {
                stage: Stage::Transport(state),
                initiator,
                hash,
            }),
            Stage::Dead => Err(NoiseError::Fatal(Fatal::Dead)),
        }
    }

    /// The Noise handshake hash `h` (DESIGN §1 "handshake-hash export for L2
    /// proofs"): the transcript hash so far, final and identical on both
    /// sides once [`is_handshake_finished`](Self::is_handshake_finished).
    /// Feed it to [`contact_proof`]. A dead link keeps its last value.
    pub fn handshake_hash(&self) -> [u8; HANDSHAKE_HASH_LEN] {
        self.hash
    }

    /// Seals one link frame into a Noise transport message (`plain` plus a
    /// 16-byte tag). `TooLarge` above [`MAX_PLAINTEXT_LEN`] (nothing
    /// changed), `HandshakeNotFinished` before transport mode,
    /// `Fatal(Exhausted)` when the sending nonce ran out.
    pub fn encrypt_frame(&mut self, plain: &[u8]) -> Result<Vec<u8>, NoiseError> {
        if plain.len() > MAX_PLAINTEXT_LEN {
            return Err(NoiseError::TooLarge(plain.len()));
        }
        let state = match &mut self.stage {
            Stage::Transport(state) => state,
            Stage::Handshake(_) => return Err(NoiseError::HandshakeNotFinished),
            Stage::Dead => return Err(NoiseError::Fatal(Fatal::Dead)),
        };
        let mut out = vec![0u8; plain.len() + TAG_LEN];
        let written = state.write_message(plain, &mut out);
        match written {
            Ok(n) => {
                out.truncate(n);
                Ok(out)
            }
            Err(snow::Error::State(StateProblem::Exhausted)) => Err(self.kill(Fatal::Exhausted)),
            Err(snow::Error::Input) => Err(NoiseError::TooLarge(plain.len())),
            Err(_) => Err(self.kill(Fatal::Internal)),
        }
    }

    /// Opens one Noise transport message. `TooLarge` above
    /// [`MAX_MESSAGE_LEN`] (nothing changed; the stream framer never yields
    /// such a message), `HandshakeNotFinished` before transport mode,
    /// `Fatal(Decrypt)` for a message shorter than a tag or whose tag does
    /// not verify, `Fatal(Exhausted)` when the receiving nonce ran out.
    pub fn decrypt_frame(&mut self, ct: &[u8]) -> Result<Vec<u8>, NoiseError> {
        if ct.len() > MAX_MESSAGE_LEN {
            return Err(NoiseError::TooLarge(ct.len()));
        }
        let state = match &mut self.stage {
            Stage::Transport(state) => state,
            Stage::Handshake(_) => return Err(NoiseError::HandshakeNotFinished),
            Stage::Dead => return Err(NoiseError::Fatal(Fatal::Dead)),
        };
        // `snow`'s cipher backends compute `ct.len() - TAG_LEN` unchecked
        // (its `CipherState` screens that first); guard here so this layer
        // never leans on the dependency's check.
        let Some(plain_len) = ct.len().checked_sub(TAG_LEN) else {
            return Err(self.kill(Fatal::Decrypt));
        };
        let mut out = vec![0u8; plain_len];
        let read = state.read_message(ct, &mut out);
        match read {
            Ok(n) => {
                out.truncate(n);
                Ok(out)
            }
            Err(snow::Error::Decrypt) => Err(self.kill(Fatal::Decrypt)),
            Err(snow::Error::State(StateProblem::Exhausted)) => Err(self.kill(Fatal::Exhausted)),
            Err(_) => Err(self.kill(Fatal::Internal)),
        }
    }

    /// Nonce of the next message this side sends (`None` outside transport
    /// mode); for metrics and tests.
    pub fn sending_nonce(&self) -> Option<u64> {
        match &self.stage {
            Stage::Transport(state) => Some(state.sending_nonce()),
            Stage::Handshake(_) | Stage::Dead => None,
        }
    }

    /// Nonce the next received message must carry (`None` outside transport
    /// mode); for metrics and tests.
    pub fn receiving_nonce(&self) -> Option<u64> {
        match &self.stage {
            Stage::Transport(state) => Some(state.receiving_nonce()),
            Stage::Handshake(_) | Stage::Dead => None,
        }
    }
}

// ---- L2 contact proof -----------------------------------------------------------

/// Role byte mixed into a contact proof: `0x01` for the Noise initiator (the
/// BLE central), `0x02` for the responder (the peripheral).
pub const fn role_byte(role: Role) -> u8 {
    match role {
        Role::Central => 0x01,
        Role::Peripheral => 0x02,
    }
}

/// The pairwise L2 proof for one QR-verified contact (DESIGN §2.3
/// `ContactProof = n u8 ‖ [prf16(k_pair(c), "l2", handshake_hash)]×n`; D4 L2
/// "links authenticated as QR-verified contacts by a pairwise tag in the link
/// handshake"):
///
/// `Trunc16(HMAC-SHA256(k_pair, "meshapp.v1.l2" ‖ 0x00 ‖ role_byte ‖ handshake_hash))`
///
/// with `role_byte` = [`role_byte`]`(role)` of the *prover*. The role is
/// mixed in so that a proof is never reflected: a peer that received a
/// proof on this session cannot send it back as its own, because the
/// verifier recomputes it with the peer's role, which differs from its own
/// (both pairing sides prove knowledge of the same `link_key`,
/// `pairwise::pairing`, and the same key proves the contact on later links).
///
/// This settles DESIGN §2.3's `prf16` wording: `prf16` takes a `u64`
/// counter, so the proof uses [`hmac_sha256`] directly with the handshake
/// hash in the counter's place and the versioned label
/// [`CONTACT_PROOF_LABEL`]. Keying on the handshake hash binds the proof to
/// this session, so a proof replayed on another link proves nothing.
pub fn contact_proof(
    k_pair: &[u8; 32],
    role: Role,
    handshake_hash: &[u8; HANDSHAKE_HASH_LEN],
) -> [u8; CONTACT_PROOF_LEN] {
    let full = hmac_sha256(
        k_pair,
        &[
            CONTACT_PROOF_LABEL,
            &[0u8],
            &[role_byte(role)],
            handshake_hash,
        ],
    );
    let mut out = [0u8; CONTACT_PROOF_LEN];
    out.copy_from_slice(&full[..CONTACT_PROOF_LEN]);
    out
}

/// Constant-time check of a received proof against the one expected from
/// a peer in role `peer_role` for `k_pair` on this session, so that a peer
/// probing proofs byte by byte learns nothing from timing (the responder
/// runs this once per contact, DESIGN §2.3 "the responder matches against
/// its own contacts → L2"). The verifier passes the *peer's* role, never
/// its own, which is what makes a reflected proof fail.
pub fn verify_contact_proof(
    k_pair: &[u8; 32],
    peer_role: Role,
    handshake_hash: &[u8; HANDSHAKE_HASH_LEN],
    proof: &[u8; CONTACT_PROOF_LEN],
) -> bool {
    contact_proof(k_pair, peer_role, handshake_hash)
        .ct_eq(proof)
        .into()
}

// ---- tests ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    fn rng(seed: u64) -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(seed)
    }

    /// Runs a full NN handshake and returns both sides in transport mode.
    fn session(seed_i: u64, seed_r: u64) -> (NoiseLink, NoiseLink) {
        let mut i = NoiseLink::initiator(&mut rng(seed_i)).unwrap();
        let mut r = NoiseLink::responder(&mut rng(seed_r)).unwrap();
        assert!(i.is_my_turn());
        assert!(!r.is_my_turn());
        let m1 = i.write_handshake().unwrap();
        assert_eq!(m1.len(), HANDSHAKE_MSG1_LEN);
        r.read_handshake(&m1).unwrap();
        assert!(r.is_my_turn());
        let m2 = r.write_handshake().unwrap();
        assert_eq!(m2.len(), HANDSHAKE_MSG2_LEN);
        assert!(r.is_handshake_finished());
        assert!(!i.is_handshake_finished());
        i.read_handshake(&m2).unwrap();
        assert!(i.is_handshake_finished());
        let i = i.into_transport().unwrap();
        let r = r.into_transport().unwrap();
        assert!(i.is_transport() && r.is_transport());
        (i, r)
    }

    #[test]
    fn handshake_round_trip_hashes_match_and_frames_flow_both_ways() {
        let (mut i, mut r) = session(1, 2);
        assert_eq!(i.handshake_hash(), r.handshake_hash());
        assert_ne!(i.handshake_hash(), [0u8; 32]);
        assert!(i.is_initiator() && !r.is_initiator());
        for n in 0..5u8 {
            let plain = vec![n; 516];
            let ct = i.encrypt_frame(&plain).unwrap();
            assert_eq!(ct.len(), 516 + TAG_LEN);
            assert_eq!(r.decrypt_frame(&ct).unwrap(), plain);
            let back = r.encrypt_frame(&[n, 1, 2]).unwrap();
            assert_eq!(i.decrypt_frame(&back).unwrap(), [n, 1, 2]);
        }
        assert_eq!(i.sending_nonce(), Some(5));
        assert_eq!(r.receiving_nonce(), Some(5));
        // an empty plaintext is just a tag and still round-trips
        let ct = i.encrypt_frame(&[]).unwrap();
        assert_eq!(ct.len(), TAG_LEN);
        assert_eq!(r.decrypt_frame(&ct).unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn handshake_is_deterministic_in_the_seed() {
        let mut a = NoiseLink::initiator(&mut rng(7)).unwrap();
        let mut b = NoiseLink::initiator(&mut rng(7)).unwrap();
        let mut c = NoiseLink::initiator(&mut rng(8)).unwrap();
        let ma = a.write_handshake().unwrap();
        assert_eq!(ma, b.write_handshake().unwrap());
        assert_ne!(ma, c.write_handshake().unwrap());
        assert_eq!(a.handshake_hash(), b.handshake_hash());
        // and so is the whole session, including the transport bytes
        let (mut i1, _) = session(3, 4);
        let (mut i2, _) = session(3, 4);
        let (i3, _) = session(3, 5);
        assert_eq!(i1.handshake_hash(), i2.handshake_hash());
        assert_ne!(i1.handshake_hash(), i3.handshake_hash());
        assert_eq!(
            i1.encrypt_frame(b"same").unwrap(),
            i2.encrypt_frame(b"same").unwrap()
        );
    }

    #[test]
    fn out_of_order_calls_do_not_change_state() {
        let mut i = NoiseLink::initiator(&mut rng(1)).unwrap();
        let mut r = NoiseLink::responder(&mut rng(2)).unwrap();
        assert_eq!(r.write_handshake(), Err(NoiseError::OutOfOrder));
        assert_eq!(i.read_handshake(&[0u8; 48]), Err(NoiseError::OutOfOrder));
        assert_eq!(i.encrypt_frame(b"x"), Err(NoiseError::HandshakeNotFinished));
        assert_eq!(
            i.decrypt_frame(&[0u8; 32]),
            Err(NoiseError::HandshakeNotFinished)
        );
        let m1 = i.write_handshake().unwrap();
        assert_eq!(i.write_handshake(), Err(NoiseError::OutOfOrder));
        assert!(!i.is_dead());
        r.read_handshake(&m1).unwrap();
        assert_eq!(r.read_handshake(&m1), Err(NoiseError::OutOfOrder));
        let m2 = r.write_handshake().unwrap();
        assert_eq!(r.write_handshake(), Err(NoiseError::OutOfOrder));
        assert_eq!(r.read_handshake(&m2), Err(NoiseError::OutOfOrder));
        i.read_handshake(&m2).unwrap();
        assert_eq!(i.write_handshake(), Err(NoiseError::OutOfOrder));
        let mut i = i.into_transport().unwrap();
        assert_eq!(i.write_handshake(), Err(NoiseError::OutOfOrder));
        assert_eq!(i.read_handshake(&m2), Err(NoiseError::OutOfOrder));
        // idempotent
        let i = i.into_transport().unwrap();
        assert!(i.is_transport());
    }

    #[test]
    fn into_transport_before_finished_is_refused() {
        let i = NoiseLink::initiator(&mut rng(1)).unwrap();
        assert_eq!(
            i.into_transport().err(),
            Some(NoiseError::HandshakeNotFinished)
        );
    }

    #[test]
    fn wrong_length_handshake_messages_are_fatal_not_panics() {
        // a tag-less message 2 would underflow inside snow
        let mut i = NoiseLink::initiator(&mut rng(1)).unwrap();
        let _ = i.write_handshake().unwrap();
        assert_eq!(
            i.read_handshake(&[0u8; 32]),
            Err(NoiseError::Fatal(Fatal::Handshake))
        );
        assert!(i.is_dead());
        assert_eq!(i.write_handshake(), Err(NoiseError::Fatal(Fatal::Dead)));
        assert_eq!(
            i.read_handshake(&[0u8; 48]),
            Err(NoiseError::Fatal(Fatal::Dead))
        );
        assert_eq!(i.encrypt_frame(b"x"), Err(NoiseError::Fatal(Fatal::Dead)));
        assert!(i.into_transport().is_err());

        for len in [0usize, 31, 33, 48, 100] {
            let mut r = NoiseLink::responder(&mut rng(2)).unwrap();
            assert_eq!(
                r.read_handshake(&vec![1u8; len]),
                Err(NoiseError::Fatal(Fatal::Handshake))
            );
            assert!(r.is_dead());
        }
        // the right length but a forged tag
        let mut i = NoiseLink::initiator(&mut rng(1)).unwrap();
        let _ = i.write_handshake().unwrap();
        assert_eq!(
            i.read_handshake(&[7u8; 48]),
            Err(NoiseError::Fatal(Fatal::Handshake))
        );
    }

    #[test]
    fn corrupted_short_replayed_and_reordered_messages_are_fatal() {
        let (mut i, mut r) = session(1, 2);
        let mut ct = i.encrypt_frame(b"hello").unwrap();
        ct[3] ^= 0x01;
        assert_eq!(r.decrypt_frame(&ct), Err(NoiseError::Fatal(Fatal::Decrypt)));
        assert!(r.is_dead());
        assert_eq!(
            r.decrypt_frame(&ct),
            Err(NoiseError::Fatal(Fatal::Dead)),
            "a dead link stays dead"
        );
        assert_eq!(r.encrypt_frame(b"x"), Err(NoiseError::Fatal(Fatal::Dead)));

        // shorter than a tag
        let (mut i, mut r) = session(1, 2);
        let _ = i.encrypt_frame(b"hello").unwrap();
        assert_eq!(
            r.decrypt_frame(&[0u8; 15]),
            Err(NoiseError::Fatal(Fatal::Decrypt))
        );
        let (_, mut r) = session(1, 2);
        assert_eq!(r.decrypt_frame(&[]), Err(NoiseError::Fatal(Fatal::Decrypt)));

        // replay
        let (mut i, mut r) = session(1, 2);
        let ct = i.encrypt_frame(b"once").unwrap();
        assert_eq!(r.decrypt_frame(&ct).unwrap(), b"once");
        assert_eq!(r.decrypt_frame(&ct), Err(NoiseError::Fatal(Fatal::Decrypt)));

        // reorder
        let (mut i, mut r) = session(1, 2);
        let first = i.encrypt_frame(b"first").unwrap();
        let second = i.encrypt_frame(b"second").unwrap();
        assert_eq!(
            r.decrypt_frame(&second),
            Err(NoiseError::Fatal(Fatal::Decrypt))
        );
        assert!(r.decrypt_frame(&first).is_err());
    }

    #[test]
    fn size_limits_are_enforced_without_touching_state() {
        let (mut i, mut r) = session(1, 2);
        let big = vec![0u8; MAX_PLAINTEXT_LEN + 1];
        assert_eq!(
            i.encrypt_frame(&big),
            Err(NoiseError::TooLarge(MAX_PLAINTEXT_LEN + 1))
        );
        assert!(!i.is_dead());
        assert_eq!(i.sending_nonce(), Some(0));
        let huge = vec![0u8; MAX_MESSAGE_LEN + 1];
        assert_eq!(
            r.decrypt_frame(&huge),
            Err(NoiseError::TooLarge(MAX_MESSAGE_LEN + 1))
        );
        assert!(!r.is_dead());
        // the maximum itself round-trips and is exactly one Noise message
        let max = vec![0xA5u8; MAX_PLAINTEXT_LEN];
        let ct = i.encrypt_frame(&max).unwrap();
        assert_eq!(ct.len(), MAX_MESSAGE_LEN);
        assert_eq!(r.decrypt_frame(&ct).unwrap(), max);
        assert!(NoiseError::Fatal(Fatal::Decrypt).is_fatal());
        assert!(!NoiseError::TooLarge(1).is_fatal());
    }

    #[test]
    fn receiving_nonce_exhaustion_is_fatal() {
        let (mut i, mut r) = session(1, 2);
        let ct = i.encrypt_frame(b"late").unwrap();
        if let Stage::Transport(state) = &mut r.stage {
            state.set_receiving_nonce(u64::MAX);
        }
        assert_eq!(r.receiving_nonce(), Some(u64::MAX));
        assert_eq!(
            r.decrypt_frame(&ct),
            Err(NoiseError::Fatal(Fatal::Exhausted))
        );
        assert!(r.is_dead());
    }

    #[test]
    fn contact_proof_is_the_documented_hmac_and_binds_to_the_session() {
        let k = [0x11u8; 32];
        let (i, r) = session(1, 2);
        let (i2, _) = session(1, 3);
        let hh = i.handshake_hash();
        let proof = contact_proof(&k, Role::Central, &hh);
        let full = hmac_sha256(&k, &[b"meshapp.v1.l2", &[0u8], &[0x01u8], &hh]);
        assert_eq!(&proof[..], &full[..16]);
        assert_eq!(proof, contact_proof(&k, Role::Central, &r.handshake_hash()));
        assert_ne!(
            proof,
            contact_proof(&k, Role::Central, &i2.handshake_hash())
        );
        assert_ne!(proof, contact_proof(&[0x12u8; 32], Role::Central, &hh));
        // the responder verifies the central's proof with the peer's role
        assert!(verify_contact_proof(&k, Role::Central, &hh, &proof));
        assert!(!verify_contact_proof(
            &[0x12u8; 32],
            Role::Central,
            &hh,
            &proof
        ));
        let mut bad = proof;
        bad[15] ^= 1;
        assert!(!verify_contact_proof(&k, Role::Central, &hh, &bad));
    }

    #[test]
    fn reflected_contact_proof_never_verifies() {
        let k = [0x33u8; 32];
        let (i, r) = session(4, 5);
        let hh = r.handshake_hash();
        assert_eq!(hh, i.handshake_hash());
        let from_central = contact_proof(&k, Role::Central, &hh);
        let from_peripheral = contact_proof(&k, Role::Peripheral, &hh);
        assert_ne!(from_central, from_peripheral);
        let full = hmac_sha256(&k, &[b"meshapp.v1.l2", &[0u8], &[0x02u8], &hh]);
        assert_eq!(&from_peripheral[..], &full[..16]);
        // the central expects a peripheral proof: its own echoed back fails
        assert!(!verify_contact_proof(
            &k,
            Role::Peripheral,
            &hh,
            &from_central
        ));
        assert!(verify_contact_proof(
            &k,
            Role::Peripheral,
            &hh,
            &from_peripheral
        ));
        assert!(!verify_contact_proof(
            &k,
            Role::Central,
            &hh,
            &from_peripheral
        ));
        assert_eq!(role_byte(Role::Central), 0x01);
        assert_eq!(role_byte(Role::Peripheral), 0x02);
    }

    #[test]
    fn debug_does_not_leak_state() {
        let i = NoiseLink::initiator(&mut rng(1)).unwrap();
        let s = format!("{i:?}");
        assert!(s.contains("handshake") && s.contains("initiator: true"));
    }

    /// The handshake generator is a PRF of the seed, never repeats a block
    /// and wipes itself on drop: nothing that can reproduce the ephemeral
    /// private key may outlive the handshake (module doc "Randomness and
    /// determinism").
    #[test]
    fn seeded_rng_is_a_deterministic_prf_that_zeroises_on_drop() {
        fn wipes_on_drop<T: ZeroizeOnDrop>() {}
        wipes_on_drop::<SeededRandom>();

        let fresh = |seed: u8| SeededRandom {
            seed: [seed; 32],
            ctr: 0,
        };
        let (mut a, mut b, mut c) = (fresh(1), fresh(1), fresh(2));
        let (mut x, mut y, mut z) = ([0u8; 70], [0u8; 70], [0u8; 70]);
        a.try_fill_bytes(&mut x).unwrap();
        b.try_fill_bytes(&mut y).unwrap();
        c.try_fill_bytes(&mut z).unwrap();
        assert_eq!(x, y, "same seed, same stream");
        assert_ne!(x, z, "another seed, another stream");
        let block0 = hmac_sha256(&[1u8; 32], &[RNG_LABEL, &[0u8], &0u64.to_le_bytes()]);
        assert_eq!(&x[..32], &block0[..], "block 0 is the documented HMAC");
        assert_ne!(&x[..32], &x[32..64], "the counter moves between blocks");
        assert_eq!(a.ctr, 3, "70 bytes = three blocks");
        // a later draw continues the counter instead of replaying block 0
        let mut again = [0u8; 32];
        a.try_fill_bytes(&mut again).unwrap();
        assert_ne!(again, block0);
        assert_eq!(a.ctr, 4);
        // the counter never wraps onto a block already handed out
        let mut spent = SeededRandom {
            seed: [3u8; 32],
            ctr: u64::MAX,
        };
        assert!(spent.try_fill_bytes(&mut again).is_err());
        // what snow draws is block 0: the ephemeral is a function of the seed
        let mut i1 = NoiseLink::initiator(&mut rng(5)).unwrap();
        let mut i2 = NoiseLink::initiator(&mut rng(5)).unwrap();
        assert_eq!(i1.write_handshake().unwrap(), i2.write_handshake().unwrap());
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// DESIGN §10 `noise_stream`: arbitrary bytes never panic the
        /// handshake reader on either side.
        #[test]
        fn random_handshake_bytes_never_panic(
            msg in prop::collection::vec(any::<u8>(), 0..200),
            seed in any::<u64>(),
        ) {
            let mut r = NoiseLink::responder(&mut rng(seed)).unwrap();
            let res = r.read_handshake(&msg);
            prop_assert!(res.is_ok() || res == Err(NoiseError::Fatal(Fatal::Handshake)));
            let mut i = NoiseLink::initiator(&mut rng(seed)).unwrap();
            let _ = i.write_handshake().unwrap();
            let res = i.read_handshake(&msg);
            prop_assert!(res.is_ok() || res == Err(NoiseError::Fatal(Fatal::Handshake)));
        }

        /// Arbitrary bytes never panic the transport reader and never
        /// decrypt (an unkeyed forgery passing would be a cipher failure).
        #[test]
        fn random_transport_bytes_never_panic_or_decrypt(
            ct in prop::collection::vec(any::<u8>(), 0..300),
            seed in any::<u64>(),
        ) {
            let (_, mut r) = session(seed, seed.wrapping_add(1));
            let res = r.decrypt_frame(&ct);
            prop_assert_eq!(res, Err(NoiseError::Fatal(Fatal::Decrypt)));
            prop_assert!(r.is_dead());
        }

        /// Any plaintext up to the limit round-trips.
        #[test]
        fn any_plaintext_round_trips(
            plain in prop::collection::vec(any::<u8>(), 0..2000),
            seed in any::<u64>(),
        ) {
            let (mut i, mut r) = session(seed, seed ^ 0xBEEF);
            let ct = i.encrypt_frame(&plain).unwrap();
            prop_assert_eq!(ct.len(), plain.len() + TAG_LEN);
            prop_assert_eq!(r.decrypt_frame(&ct).unwrap(), plain);
        }
    }
}
