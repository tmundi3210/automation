//! Roster operations: the identity-signed objects that make up the roster
//! DAG (docs/DESIGN.md §2.4 "Roster Genesis / Add / Remove / AdminSet /
//! Setting" rows, §3; BRIEF_v2 §5.4 "Roster: a causal, hash-linked log of
//! admin-signed operations"; DECISIONS.md D3 M1 "The roster is a hash-linked
//! DAG of ops signed with identity keys. Each op cites the roster heads its
//! author had seen").
//!
//! Layout (DESIGN §2.4): `header ‖ body ‖ sig [64]` where `header` is the
//! common [`SignedHeader`] and the body is chosen by `header.kind`:
//!
//! | kind | body |
//! |---|---|
//! | `ROSTER_GENESIS` 0x10 | `nonce [16] ‖ name str16 ‖ n_admins u8 ‖ KeyCard×n ‖ n_members u8 ‖ (KeyCard ‖ name str16 ‖ in_person u8)×n ‖ settings u8` |
//! | `ROSTER_ADD` 0x11 | `subject KeyCard ‖ name str16 ‖ in_person u8 ‖ admin [32] ‖ in_person u8 ‖ sig [64]` |
//! | `ROSTER_REMOVE` 0x12 | `subject [32]` |
//! | `ROSTER_ADMIN_SET` 0x13 | `version u32 ‖ n u8 ‖ admins [32]×n (sorted) ‖ n_cosig u8 ‖ (admin [32] ‖ sig [64])×n_cosig (sorted by admin)` |
//! | `ROSTER_SETTING` 0x14 | `key u8 ‖ value u8` |
//!
//! Signatures (all strict Ed25519 with the `domain ‖ 0x00 ‖ bytes`
//! convention of `crypto::ed25519`):
//! * the author's signature is over `header ‖ body` under [`domain::ROSTER_OP`];
//! * an Add's endorsement is over `group_id ‖ subject.ed ‖ name str16 ‖
//!   in_person u8` under [`domain::ENDORSE`], where `in_person` is the
//!   endorsing admin's own flag ([`Endorsement::in_person`]);
//! * each AdminSet cosignature is over `group_id ‖ LE32(version) ‖ admins
//!   [32]×n` (the sorted keys, no count byte) under [`domain::ADMIN_SET`].
//!
//! `op_hash = SHA256(all bytes including sig)` ([`RosterOp::op_hash`]). The
//! Genesis cites no heads and carries a zero `group_id`: the group id *is*
//! `Trunc16(op_hash)` of the genesis (DESIGN §2.4), so a group has exactly one
//! genesis by construction.
//!
//! Decoding checks format only (caps, sorting, the genesis header rules);
//! [`RosterOp::verify`] checks every signature and KeyCard. Authority rules
//! (who may do what) live in `roster::eval`.

use crate::codec::{CodecError, Decode, Encode, Reader, Writer, decode, unknown_variant};
use crate::crypto::backend::GroupId;
use crate::crypto::ed25519::{IdentityKey, IdentitySigner, SIGNATURE_LEN, admit_public_key};
use crate::crypto::hash::Hash32;
use crate::crypto::keycard::{KEYCARD_LEN, KeyCard, KeyCardError};
use crate::objects::{SignedHeader, domain, kind, object_id, sign_bytes, verify_bytes};

/// Largest number of distinct members a group may have (BRIEF_v2 §5.4
/// "Caps: 50 members in both builds"; D3 K3). Enforced on the Genesis lists
/// and on AdminSet lists; the running cap for Adds is enforced by the author
/// (`groups::group` refuses to issue an Add at the cap), because a cap on
/// concurrent Adds from different fragments cannot be order-independent.
pub const MAX_MEMBERS: usize = 50;

/// Largest display name, in UTF-8 bytes (group name, member names).
pub const MAX_NAME_BYTES: usize = 64;

/// Encoded size of an [`Endorsement`]: `admin [32] ‖ in_person u8 ‖ sig [64]`.
pub const ENDORSEMENT_LEN: usize = 32 + 1 + SIGNATURE_LEN;

/// Roster setting keys (DESIGN §2.4 "Roster Setting: straw-mode, media-off;
/// P0 authority, used from P1"; docs/decisions-assumed.md #11). Key
/// [`setting::FLAGS`] is the bitfield the Genesis `settings` byte initialises.
pub mod setting {
    /// Bitfield of boolean switches; its initial value is the Genesis
    /// `settings` byte.
    pub const FLAGS: u8 = 0;
    /// `FLAGS` bit: votes are non-binding straw polls.
    pub const FLAG_STRAW_MODE: u8 = 0x01;
    /// `FLAGS` bit: previews are off for this group.
    pub const FLAG_MEDIA_OFF: u8 = 0x02;
    /// `FLAGS` bit: public (identity-signed, visible) ballots in the B build
    /// (docs/decisions-assumed.md #11: off by default, admin opt-in).
    pub const FLAG_PUBLIC_VOTES: u8 = 0x04;
}

/// One member as listed by a Genesis or added by an Add: the KeyCard that
/// binds the identity to its session key (D3 K1), the display name and
/// whether the lister verified the person in person (BRIEF_v2 §5.4
/// "at least one of whom verified the newcomer in person").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberEntry {
    pub keycard: KeyCard,
    pub name: String,
    pub in_person: bool,
}

impl MemberEntry {
    /// The member's identity key.
    pub fn key(&self) -> IdentityKey {
        self.keycard.ed
    }
}

/// The second admin's signature on an Add (BRIEF_v2 §5.4 "Add or re-add: 2
/// distinct admins"; docs/decisions-assumed.md #8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endorsement {
    pub admin: IdentityKey,
    pub in_person: bool,
    pub sig: [u8; SIGNATURE_LEN],
}

/// The body of a roster op, one variant per `kind::ROSTER_*`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RosterBody {
    /// Creates the group: the creator signs the initial admin set (D3 M2).
    Genesis {
        nonce: [u8; 16],
        name: String,
        /// Initial admins; they are members too. A key that also appears
        /// in `members` takes its name from there.
        admins: Vec<KeyCard>,
        members: Vec<MemberEntry>,
        /// Initial value of the [`setting::FLAGS`] setting.
        settings: u8,
    },
    /// Adds (or re-adds) a member; the author is the first admin, the
    /// endorsement the second.
    Add {
        subject: MemberEntry,
        endorsement: Endorsement,
    },
    /// Removes a member (any one admin; D3 M2).
    Remove { subject: IdentityKey },
    /// Replaces the admin set with `admins` (strict majority of the current
    /// admins cosigning; D3 M2).
    AdminSet {
        version: u32,
        /// Sorted ascending, no duplicates, non-empty.
        admins: Vec<IdentityKey>,
        /// Sorted by admin key ascending, no duplicates, non-empty.
        cosigs: Vec<(IdentityKey, [u8; SIGNATURE_LEN])>,
    },
    /// Display-only switch (DESIGN §3 "Setting").
    Setting { key: u8, value: u8 },
}

impl RosterBody {
    /// The `kind` byte this body encodes under.
    pub fn kind(&self) -> u8 {
        match self {
            RosterBody::Genesis { .. } => kind::ROSTER_GENESIS,
            RosterBody::Add { .. } => kind::ROSTER_ADD,
            RosterBody::Remove { .. } => kind::ROSTER_REMOVE,
            RosterBody::AdminSet { .. } => kind::ROSTER_ADMIN_SET,
            RosterBody::Setting { .. } => kind::ROSTER_SETTING,
        }
    }

    /// Is `k` one of the roster op kinds?
    pub fn is_roster_kind(k: u8) -> bool {
        matches!(
            k,
            kind::ROSTER_GENESIS
                | kind::ROSTER_ADD
                | kind::ROSTER_REMOVE
                | kind::ROSTER_ADMIN_SET
                | kind::ROSTER_SETTING
        )
    }
}

/// Why a roster op was not accepted.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RosterOpError {
    #[error("roster op codec: {0}")]
    Codec(#[from] CodecError),
    /// `header.kind` is not a roster kind.
    #[error("unknown roster op kind {0:#04x}")]
    UnknownKind(u8),
    /// `header.kind` names a different body than the one present.
    #[error("header kind {header:#04x} does not match body kind {body:#04x}")]
    KindMismatch { header: u8, body: u8 },
    /// `sign` was handed a header whose author is not the signer.
    #[error("header author is not the signing key")]
    AuthorMismatch,
    /// A format rule (cap, ordering, genesis header rule) is violated; the
    /// string names the field.
    #[error("malformed roster op: {0}")]
    Malformed(&'static str),
    /// The author's signature over `header ‖ body` does not verify.
    #[error("roster op signature does not verify")]
    BadSignature,
    /// A KeyCard in the op does not verify.
    #[error("keycard {key} rejected: {err}")]
    BadKeyCard { key: IdentityKey, err: KeyCardError },
    /// The Add's endorsement signature does not verify.
    #[error("endorsement signature does not verify")]
    BadEndorsement,
    /// An AdminSet cosignature does not verify.
    #[error("cosignature by {0} does not verify")]
    BadCosignature(IdentityKey),
}

/// A roster operation: common header, body and the author's signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RosterOp {
    pub header: SignedHeader,
    pub body: RosterBody,
    pub sig: [u8; SIGNATURE_LEN],
}

// ---- encoding helpers ------------------------------------------------------

fn write_keycard(w: &mut Writer, kc: &KeyCard) {
    w.bytes(&kc.to_bytes());
}

/// Reads a KeyCard's bytes and admits its identity key. The KeyCard
/// signature is checked by [`RosterOp::verify`], not here (decode = format).
fn read_keycard(r: &mut Reader<'_>) -> Result<KeyCard, CodecError> {
    let bytes: [u8; KEYCARD_LEN] = r.bytes()?;
    let mut ed = [0u8; 32];
    ed.copy_from_slice(&bytes[..32]);
    let mut x25519 = [0u8; 32];
    x25519.copy_from_slice(&bytes[32..64]);
    let mut sig = [0u8; SIGNATURE_LEN];
    sig.copy_from_slice(&bytes[64..]);
    let ed = admit_public_key(&ed).map_err(|_| CodecError::Invalid("keycard identity key"))?;
    Ok(KeyCard { ed, x25519, sig })
}

fn read_key(r: &mut Reader<'_>, what: &'static str) -> Result<IdentityKey, CodecError> {
    let bytes: [u8; 32] = r.bytes()?;
    admit_public_key(&bytes).map_err(|_| CodecError::Invalid(what))
}

fn check_name(name: &str, what: &'static str) -> Result<(), CodecError> {
    if name.len() > MAX_NAME_BYTES {
        return Err(CodecError::Invalid(what));
    }
    Ok(())
}

fn write_member(w: &mut Writer, m: &MemberEntry) -> Result<(), CodecError> {
    check_name(&m.name, "member name")?;
    write_keycard(w, &m.keycard);
    w.str16(&m.name)?;
    w.bool(m.in_person);
    Ok(())
}

fn read_member(r: &mut Reader<'_>) -> Result<MemberEntry, CodecError> {
    let keycard = read_keycard(r)?;
    let name = r.str16()?;
    check_name(name, "member name")?;
    let in_person = r.bool()?;
    Ok(MemberEntry {
        keycard,
        name: name.to_owned(),
        in_person,
    })
}

fn strictly_ascending(keys: &[IdentityKey]) -> bool {
    keys.windows(2).all(|w| w[0] < w[1])
}

impl Encode for RosterBody {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        match self {
            RosterBody::Genesis {
                nonce,
                name,
                admins,
                members,
                settings,
            } => {
                check_name(name, "group name")?;
                w.bytes(nonce);
                w.str16(name)?;
                w.list_u8(admins, |w, kc| {
                    write_keycard(w, kc);
                    Ok(())
                })?;
                w.list_u8(members, write_member)?;
                w.u8(*settings);
                Ok(())
            }
            RosterBody::Add {
                subject,
                endorsement,
            } => {
                write_member(w, subject)?;
                w.bytes(endorsement.admin.as_bytes())
                    .bool(endorsement.in_person)
                    .bytes(&endorsement.sig);
                Ok(())
            }
            RosterBody::Remove { subject } => {
                w.bytes(subject.as_bytes());
                Ok(())
            }
            RosterBody::AdminSet {
                version,
                admins,
                cosigs,
            } => {
                w.u32(*version);
                w.list_u8(admins, |w, k| {
                    w.bytes(k.as_bytes());
                    Ok(())
                })?;
                w.list_u8(cosigs, |w, (k, sig)| {
                    w.bytes(k.as_bytes()).bytes(sig);
                    Ok(())
                })
            }
            RosterBody::Setting { key, value } => {
                w.u8(*key).u8(*value);
                Ok(())
            }
        }
    }
}

impl RosterBody {
    /// Decodes the body that `kind` names.
    pub fn decode_kind(kind_byte: u8, r: &mut Reader<'_>) -> Result<Self, CodecError> {
        match kind_byte {
            kind::ROSTER_GENESIS => {
                let nonce: [u8; 16] = r.bytes()?;
                let name = r.str16()?;
                check_name(name, "group name")?;
                let admins = r.list_u8(read_keycard)?;
                let members = r.list_u8(read_member)?;
                let settings = r.u8()?;
                Ok(RosterBody::Genesis {
                    nonce,
                    name: name.to_owned(),
                    admins,
                    members,
                    settings,
                })
            }
            kind::ROSTER_ADD => {
                let subject = read_member(r)?;
                let admin = read_key(r, "endorsing admin key")?;
                let in_person = r.bool()?;
                let sig: [u8; SIGNATURE_LEN] = r.bytes()?;
                Ok(RosterBody::Add {
                    subject,
                    endorsement: Endorsement {
                        admin,
                        in_person,
                        sig,
                    },
                })
            }
            kind::ROSTER_REMOVE => Ok(RosterBody::Remove {
                subject: read_key(r, "remove subject key")?,
            }),
            kind::ROSTER_ADMIN_SET => {
                let version = r.u32()?;
                let admins = r.list_u8(|r| read_key(r, "admin key"))?;
                let cosigs = r.list_u8(|r| {
                    let k = read_key(r, "cosigner key")?;
                    let sig: [u8; SIGNATURE_LEN] = r.bytes()?;
                    Ok((k, sig))
                })?;
                Ok(RosterBody::AdminSet {
                    version,
                    admins,
                    cosigs,
                })
            }
            kind::ROSTER_SETTING => Ok(RosterBody::Setting {
                key: r.u8()?,
                value: r.u8()?,
            }),
            other => Err(unknown_variant("roster op kind", other)),
        }
    }
}

impl Encode for RosterOp {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.header.encode_to(w)?;
        self.body.encode_to(w)?;
        w.bytes(&self.sig);
        Ok(())
    }
}

impl Decode for RosterOp {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let header = SignedHeader::decode_from(r)?;
        let body = RosterBody::decode_kind(header.kind, r)?;
        let sig: [u8; SIGNATURE_LEN] = r.bytes()?;
        let op = RosterOp { header, body, sig };
        op.check_wellformed().map_err(|e| match e {
            RosterOpError::Codec(c) => c,
            RosterOpError::UnknownKind(k) => unknown_variant("roster op kind", k),
            RosterOpError::Malformed(what) => CodecError::Invalid(what),
            _ => CodecError::Invalid("roster op"),
        })?;
        Ok(op)
    }
}

impl RosterOp {
    /// Bytes the author signs: `header ‖ body` (DESIGN §2.4 "signature over
    /// all bytes with the domain prefix").
    pub fn signature_input(
        header: &SignedHeader,
        body: &RosterBody,
    ) -> Result<Vec<u8>, CodecError> {
        let mut w = Writer::new();
        header.encode_to(&mut w)?;
        body.encode_to(&mut w)?;
        Ok(w.into_inner())
    }

    /// Message an Add endorsement signs under [`domain::ENDORSE`]:
    /// `group_id ‖ subject.ed ‖ name str16 ‖ in_person u8` (DESIGN §2.4
    /// "endorsement sig is over `meshapp.v1.endorse ‖ group_id ‖ subject.ed ‖
    /// name ‖ in_person`"), with `in_person` the endorser's own flag.
    pub fn endorsement_message(
        group_id: &GroupId,
        subject: &IdentityKey,
        name: &str,
        in_person: bool,
    ) -> Result<Vec<u8>, CodecError> {
        check_name(name, "member name")?;
        let mut w = Writer::with_capacity(16 + 32 + 2 + name.len() + 1);
        w.bytes(group_id.as_bytes()).bytes(subject.as_bytes());
        w.str16(name)?;
        w.bool(in_person);
        Ok(w.into_inner())
    }

    /// Signs an Add endorsement (the second admin's part of an Add).
    pub fn endorse(
        endorser: &IdentitySigner,
        group_id: &GroupId,
        subject: &IdentityKey,
        name: &str,
        in_person: bool,
    ) -> Result<Endorsement, CodecError> {
        let msg = Self::endorsement_message(group_id, subject, name, in_person)?;
        Ok(Endorsement {
            admin: endorser.public(),
            in_person,
            sig: sign_bytes(endorser, domain::ENDORSE, &msg),
        })
    }

    /// Message an AdminSet cosignature signs under [`domain::ADMIN_SET`]:
    /// `group_id ‖ LE32(version) ‖ admins [32]×n` (DESIGN §2.4 "cosig domain
    /// `meshapp.v1.adminset ‖ group_id ‖ version ‖ admins`").
    pub fn admin_set_message(group_id: &GroupId, version: u32, admins: &[IdentityKey]) -> Vec<u8> {
        let mut w = Writer::with_capacity(16 + 4 + 32 * admins.len());
        w.bytes(group_id.as_bytes()).u32(version);
        for k in admins {
            w.bytes(k.as_bytes());
        }
        w.into_inner()
    }

    /// Produces one admin's cosignature on an AdminSet (`admins` must already
    /// be sorted; the author collects these and puts them in the op).
    pub fn cosign(
        admin: &IdentitySigner,
        group_id: &GroupId,
        version: u32,
        admins: &[IdentityKey],
    ) -> (IdentityKey, [u8; SIGNATURE_LEN]) {
        let msg = Self::admin_set_message(group_id, version, admins);
        (admin.public(), sign_bytes(admin, domain::ADMIN_SET, &msg))
    }

    /// Signs `header ‖ body` as `signer`. Fails if the header's author is not
    /// the signer, if the header kind does not match the body, or if the op
    /// is malformed (the op would never verify, so it is not produced).
    pub fn sign(
        signer: &IdentitySigner,
        header: SignedHeader,
        body: RosterBody,
    ) -> Result<RosterOp, RosterOpError> {
        if header.author != signer.public() {
            return Err(RosterOpError::AuthorMismatch);
        }
        let bytes = Self::signature_input(&header, &body)?;
        let sig = sign_bytes(signer, domain::ROSTER_OP, &bytes);
        let op = RosterOp { header, body, sig };
        op.check_wellformed()?;
        Ok(op)
    }

    /// The author's identity key.
    pub fn author(&self) -> &IdentityKey {
        &self.header.author
    }

    /// Is this the group's Genesis?
    pub fn is_genesis(&self) -> bool {
        matches!(self.body, RosterBody::Genesis { .. })
    }

    /// `op_hash = SHA256(all bytes including sig)` (DESIGN §2.4).
    ///
    /// # Panics
    ///
    /// Only if the op violates a codec length limit, which [`RosterOp::sign`]
    /// and decoding both rule out; use [`RosterOp::try_op_hash`] for a
    /// hand-assembled op.
    pub fn op_hash(&self) -> Hash32 {
        object_id(&self.encode())
    }

    /// [`RosterOp::op_hash`] without the panic path.
    pub fn try_op_hash(&self) -> Result<Hash32, CodecError> {
        Ok(object_id(&self.try_encode()?))
    }

    /// The group this op belongs to: `Trunc16(op_hash)` for the Genesis
    /// (DESIGN §2.4 "group id = Trunc16(op_hash)"), the header field
    /// otherwise.
    pub fn group_id(&self) -> GroupId {
        if self.is_genesis() {
            GroupId(self.op_hash().truncate16())
        } else {
            self.header.group_id
        }
    }

    /// Format rules that do not need any cryptography. Decoding enforces the
    /// same rules, so a decoded op always passes; [`RosterOp::verify`] runs
    /// them again for hand-assembled ops.
    pub fn check_wellformed(&self) -> Result<(), RosterOpError> {
        let k = self.header.kind;
        if !RosterBody::is_roster_kind(k) {
            return Err(RosterOpError::UnknownKind(k));
        }
        if k != self.body.kind() {
            return Err(RosterOpError::KindMismatch {
                header: k,
                body: self.body.kind(),
            });
        }
        match &self.body {
            RosterBody::Genesis {
                name,
                admins,
                members,
                ..
            } => {
                // DESIGN §2.4: `n_heads = 0`; `group_id` field zero.
                if !self.header.heads.is_empty() {
                    return Err(RosterOpError::Malformed("genesis cites heads"));
                }
                if self.header.group_id != GroupId([0u8; GroupId::LEN]) {
                    return Err(RosterOpError::Malformed("genesis group id must be zero"));
                }
                if name.len() > MAX_NAME_BYTES {
                    return Err(RosterOpError::Malformed("group name"));
                }
                // DESIGN §3: `n_admins ≥ 1`; D3 M2 "the creator signs the
                // initial admin set" → the creator is one of them.
                if admins.is_empty() {
                    return Err(RosterOpError::Malformed("genesis needs at least one admin"));
                }
                if !admins.iter().any(|kc| kc.ed == self.header.author) {
                    return Err(RosterOpError::Malformed("genesis author is not an admin"));
                }
                let mut keys: Vec<IdentityKey> = admins.iter().map(|kc| kc.ed).collect();
                keys.sort_unstable();
                if keys.windows(2).any(|w| w[0] == w[1]) {
                    return Err(RosterOpError::Malformed("duplicate genesis admin"));
                }
                let mut member_keys: Vec<IdentityKey> = members.iter().map(|m| m.key()).collect();
                member_keys.sort_unstable();
                if member_keys.windows(2).any(|w| w[0] == w[1]) {
                    return Err(RosterOpError::Malformed("duplicate genesis member"));
                }
                if members.iter().any(|m| m.name.len() > MAX_NAME_BYTES) {
                    return Err(RosterOpError::Malformed("member name"));
                }
                keys.extend(member_keys);
                keys.sort_unstable();
                keys.dedup();
                if keys.len() > MAX_MEMBERS {
                    return Err(RosterOpError::Malformed("too many members"));
                }
            }
            RosterBody::Add { subject, .. } => {
                self.check_non_genesis_header()?;
                if subject.name.len() > MAX_NAME_BYTES {
                    return Err(RosterOpError::Malformed("member name"));
                }
            }
            RosterBody::Remove { .. } | RosterBody::Setting { .. } => {
                self.check_non_genesis_header()?;
            }
            RosterBody::AdminSet { admins, cosigs, .. } => {
                self.check_non_genesis_header()?;
                if admins.is_empty() || admins.len() > MAX_MEMBERS {
                    return Err(RosterOpError::Malformed("admin set size"));
                }
                if !strictly_ascending(admins) {
                    return Err(RosterOpError::Malformed("admin set not sorted"));
                }
                if cosigs.is_empty() || cosigs.len() > MAX_MEMBERS {
                    return Err(RosterOpError::Malformed("cosignature count"));
                }
                if cosigs.windows(2).any(|w| w[0].0 >= w[1].0) {
                    return Err(RosterOpError::Malformed("cosignatures not sorted"));
                }
            }
        }
        Ok(())
    }

    fn check_non_genesis_header(&self) -> Result<(), RosterOpError> {
        // Every non-genesis op cites at least one head: its causal past must
        // reach the genesis for `admins(P)` to be defined (DESIGN §3).
        if self.header.heads.is_empty() {
            return Err(RosterOpError::Malformed("op cites no heads"));
        }
        if self.header.group_id == GroupId([0u8; GroupId::LEN]) {
            return Err(RosterOpError::Malformed("zero group id"));
        }
        Ok(())
    }

    /// Full check: format, the author's signature, every KeyCard, the
    /// endorsement and every cosignature (DESIGN §2.4 "Admission: every
    /// Ed25519 key ... passes `admit_public_key` before any signature
    /// check"; the keys were admitted when the op was decoded or built).
    pub fn verify(&self) -> Result<(), RosterOpError> {
        self.check_wellformed()?;
        let bytes = Self::signature_input(&self.header, &self.body)?;
        verify_bytes(domain::ROSTER_OP, &self.header.author, &bytes, &self.sig)
            .map_err(|_| RosterOpError::BadSignature)?;
        let group_id = self.group_id();
        match &self.body {
            RosterBody::Genesis {
                admins, members, ..
            } => {
                for kc in admins.iter().chain(members.iter().map(|m| &m.keycard)) {
                    kc.verify()
                        .map_err(|err| RosterOpError::BadKeyCard { key: kc.ed, err })?;
                }
            }
            RosterBody::Add {
                subject,
                endorsement,
            } => {
                subject
                    .keycard
                    .verify()
                    .map_err(|err| RosterOpError::BadKeyCard {
                        key: subject.keycard.ed,
                        err,
                    })?;
                let msg = Self::endorsement_message(
                    &group_id,
                    &subject.keycard.ed,
                    &subject.name,
                    endorsement.in_person,
                )?;
                verify_bytes(domain::ENDORSE, &endorsement.admin, &msg, &endorsement.sig)
                    .map_err(|_| RosterOpError::BadEndorsement)?;
            }
            RosterBody::Remove { .. } | RosterBody::Setting { .. } => {}
            RosterBody::AdminSet {
                version,
                admins,
                cosigs,
            } => {
                let msg = Self::admin_set_message(&group_id, *version, admins);
                for (k, sig) in cosigs {
                    verify_bytes(domain::ADMIN_SET, k, &msg, sig)
                        .map_err(|_| RosterOpError::BadCosignature(*k))?;
                }
            }
        }
        Ok(())
    }

    /// Decodes and fully verifies an op from its bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<RosterOp, RosterOpError> {
        let op: RosterOp = decode(bytes)?;
        op.verify()?;
        Ok(op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::keycard::KeyCard;
    use crate::objects::MAX_HEADS;
    use proptest::prelude::*;

    fn signer(i: u8) -> IdentitySigner {
        IdentitySigner::from_seed(&[i; 32])
    }

    fn card(i: u8) -> KeyCard {
        KeyCard::issue(&signer(i), [0x40 + i; 32])
    }

    fn member(i: u8, in_person: bool) -> MemberEntry {
        MemberEntry {
            keycard: card(i),
            name: format!("member {i}"),
            in_person,
        }
    }

    fn genesis_header(author: u8) -> SignedHeader {
        SignedHeader::new(
            kind::ROSTER_GENESIS,
            GroupId([0u8; 16]),
            signer(author).public(),
            0,
            Hash32::ZERO,
            vec![],
        )
        .unwrap()
    }

    fn genesis() -> RosterOp {
        RosterOp::sign(
            &signer(1),
            genesis_header(1),
            RosterBody::Genesis {
                nonce: [7u8; 16],
                name: "Shelter A".into(),
                admins: vec![card(1), card(2)],
                members: vec![member(1, true), member(3, true), member(4, false)],
                settings: setting::FLAG_MEDIA_OFF,
            },
        )
        .unwrap()
    }

    fn header(author: u8, k: u8, group: GroupId, heads: Vec<Hash32>) -> SignedHeader {
        SignedHeader::new(k, group, signer(author).public(), 1, Hash32::ZERO, heads).unwrap()
    }

    #[test]
    fn genesis_round_trips_and_group_id_is_trunc16_of_hash() {
        let g = genesis();
        assert!(g.verify().is_ok());
        let bytes = g.encode();
        let back: RosterOp = decode(&bytes).unwrap();
        assert_eq!(back, g);
        assert_eq!(back.op_hash(), g.op_hash());
        assert_eq!(g.group_id(), GroupId(g.op_hash().truncate16()));
        assert_eq!(RosterOp::from_bytes(&bytes).unwrap(), g);
        // header (1+1+16+32+4+32+1) + nonce 16 + name (2+9) + admins (1+2*128)
        // + members (1 + 3*(128 + 2 + len + 1)) + settings 1 + sig 64
        let names: usize = (1..=3).map(|_| "member x".len()).sum();
        let expect =
            87 + 16 + 11 + 1 + 2 * KEYCARD_LEN + 1 + 3 * (KEYCARD_LEN + 3) + names + 1 + 64;
        assert_eq!(bytes.len(), expect);
    }

    #[test]
    fn every_body_round_trips() {
        let g = genesis();
        let gid = g.group_id();
        let heads = vec![g.op_hash()];
        let sub = member(9, false);
        let end = RosterOp::endorse(&signer(2), &gid, &sub.key(), &sub.name, true).unwrap();
        let add = RosterOp::sign(
            &signer(1),
            header(1, kind::ROSTER_ADD, gid, heads.clone()),
            RosterBody::Add {
                subject: sub,
                endorsement: end,
            },
        )
        .unwrap();
        let rem = RosterOp::sign(
            &signer(1),
            header(1, kind::ROSTER_REMOVE, gid, heads.clone()),
            RosterBody::Remove {
                subject: signer(3).public(),
            },
        )
        .unwrap();
        let mut admins = vec![signer(1).public(), signer(2).public(), signer(3).public()];
        admins.sort_unstable();
        let mut cosigs = vec![
            RosterOp::cosign(&signer(1), &gid, 1, &admins),
            RosterOp::cosign(&signer(2), &gid, 1, &admins),
        ];
        cosigs.sort_by_key(|a| a.0);
        let aset = RosterOp::sign(
            &signer(1),
            header(1, kind::ROSTER_ADMIN_SET, gid, heads.clone()),
            RosterBody::AdminSet {
                version: 1,
                admins,
                cosigs,
            },
        )
        .unwrap();
        let set = RosterOp::sign(
            &signer(2),
            header(2, kind::ROSTER_SETTING, gid, heads),
            RosterBody::Setting {
                key: setting::FLAGS,
                value: setting::FLAG_STRAW_MODE,
            },
        )
        .unwrap();
        for op in [&add, &rem, &aset, &set] {
            assert!(op.verify().is_ok(), "{op:?}");
            let bytes = op.encode();
            let back: RosterOp = decode(&bytes).unwrap();
            assert_eq!(&back, op);
            assert_eq!(back.group_id(), gid);
        }
        assert_eq!(rem.encode().len(), 87 + 32 + 32 + 64);
        assert_eq!(set.encode().len(), 87 + 32 + 2 + 64);
    }

    #[test]
    fn verify_catches_each_tampered_signature() {
        let g = genesis();
        let gid = g.group_id();
        let heads = vec![g.op_hash()];
        let sub = member(9, false);
        let mut end = RosterOp::endorse(&signer(2), &gid, &sub.key(), &sub.name, true).unwrap();
        end.sig[0] ^= 1;
        let add = RosterOp::sign(
            &signer(1),
            header(1, kind::ROSTER_ADD, gid, heads.clone()),
            RosterBody::Add {
                subject: sub.clone(),
                endorsement: end,
            },
        )
        .unwrap();
        assert_eq!(add.verify(), Err(RosterOpError::BadEndorsement));

        // endorsement signed for a different in_person flag
        let end = RosterOp::endorse(&signer(2), &gid, &sub.key(), &sub.name, true).unwrap();
        let flipped = Endorsement {
            in_person: false,
            ..end
        };
        let add = RosterOp::sign(
            &signer(1),
            header(1, kind::ROSTER_ADD, gid, heads.clone()),
            RosterBody::Add {
                subject: sub.clone(),
                endorsement: flipped,
            },
        )
        .unwrap();
        assert_eq!(add.verify(), Err(RosterOpError::BadEndorsement));

        let mut tampered = g.clone();
        tampered.sig[5] ^= 0x80;
        assert_eq!(tampered.verify(), Err(RosterOpError::BadSignature));

        let mut bad_card = g.clone();
        if let RosterBody::Genesis { members, .. } = &mut bad_card.body {
            members[1].keycard.sig[0] ^= 1;
        }
        let bytes = RosterOp::signature_input(&bad_card.header, &bad_card.body).unwrap();
        bad_card.sig = sign_bytes(&signer(1), domain::ROSTER_OP, &bytes);
        assert!(matches!(
            bad_card.verify(),
            Err(RosterOpError::BadKeyCard { .. })
        ));

        let admins = {
            let mut a = vec![signer(1).public(), signer(2).public()];
            a.sort_unstable();
            a
        };
        let mut cosigs = vec![
            RosterOp::cosign(&signer(1), &gid, 1, &admins),
            RosterOp::cosign(&signer(2), &gid, 2, &admins), // wrong version
        ];
        cosigs.sort_by_key(|a| a.0);
        let aset = RosterOp::sign(
            &signer(1),
            header(1, kind::ROSTER_ADMIN_SET, gid, heads),
            RosterBody::AdminSet {
                version: 1,
                admins,
                cosigs,
            },
        )
        .unwrap();
        assert_eq!(
            aset.verify(),
            Err(RosterOpError::BadCosignature(signer(2).public()))
        );
    }

    #[test]
    fn sign_rejects_author_and_kind_mismatch_and_malformed() {
        let g = genesis();
        let gid = g.group_id();
        let heads = vec![g.op_hash()];
        assert_eq!(
            RosterOp::sign(
                &signer(2),
                header(1, kind::ROSTER_REMOVE, gid, heads.clone()),
                RosterBody::Remove {
                    subject: signer(3).public()
                }
            )
            .err(),
            Some(RosterOpError::AuthorMismatch)
        );
        assert!(matches!(
            RosterOp::sign(
                &signer(1),
                header(1, kind::ROSTER_SETTING, gid, heads.clone()),
                RosterBody::Remove {
                    subject: signer(3).public()
                }
            ),
            Err(RosterOpError::KindMismatch { .. })
        ));
        assert!(matches!(
            RosterOp::sign(
                &signer(1),
                header(1, 0x77, gid, heads.clone()),
                RosterBody::Remove {
                    subject: signer(3).public()
                }
            ),
            Err(RosterOpError::UnknownKind(0x77))
        ));
        // genesis with heads / non-zero group id / no admins / author not admin
        let mut h = genesis_header(1);
        h.heads = heads.clone();
        let body = RosterBody::Genesis {
            nonce: [0; 16],
            name: "x".into(),
            admins: vec![card(1)],
            members: vec![],
            settings: 0,
        };
        assert!(matches!(
            RosterOp::sign(&signer(1), h, body.clone()),
            Err(RosterOpError::Malformed(_))
        ));
        let mut h = genesis_header(1);
        h.group_id = gid;
        assert!(matches!(
            RosterOp::sign(&signer(1), h, body.clone()),
            Err(RosterOpError::Malformed(_))
        ));
        let no_admins = RosterBody::Genesis {
            nonce: [0; 16],
            name: "x".into(),
            admins: vec![],
            members: vec![member(1, true)],
            settings: 0,
        };
        assert!(matches!(
            RosterOp::sign(&signer(1), genesis_header(1), no_admins),
            Err(RosterOpError::Malformed(_))
        ));
        let not_admin = RosterBody::Genesis {
            nonce: [0; 16],
            name: "x".into(),
            admins: vec![card(2)],
            members: vec![],
            settings: 0,
        };
        assert!(matches!(
            RosterOp::sign(&signer(1), genesis_header(1), not_admin),
            Err(RosterOpError::Malformed(_))
        ));
        // non-genesis with no heads
        assert!(matches!(
            RosterOp::sign(
                &signer(1),
                header(1, kind::ROSTER_REMOVE, gid, vec![]),
                RosterBody::Remove {
                    subject: signer(3).public()
                }
            ),
            Err(RosterOpError::Malformed(_))
        ));
        // name too long
        let long = "x".repeat(MAX_NAME_BYTES + 1);
        let end = RosterOp::endorse(&signer(2), &gid, &signer(9).public(), &long, true);
        assert!(end.is_err());
        // unsorted admin set
        let mut admins = vec![signer(1).public(), signer(2).public()];
        admins.sort_unstable();
        admins.reverse();
        let cosigs = vec![RosterOp::cosign(&signer(1), &gid, 1, &admins)];
        assert!(matches!(
            RosterOp::sign(
                &signer(1),
                header(1, kind::ROSTER_ADMIN_SET, gid, heads),
                RosterBody::AdminSet {
                    version: 1,
                    admins,
                    cosigs
                }
            ),
            Err(RosterOpError::Malformed(_))
        ));
    }

    #[test]
    fn genesis_caps_members_at_fifty() {
        let members: Vec<MemberEntry> = (2..=51u8).map(|i| member(i, true)).collect();
        let body = RosterBody::Genesis {
            nonce: [0; 16],
            name: "big".into(),
            admins: vec![card(1)],
            members,
            settings: 0,
        };
        assert!(matches!(
            RosterOp::sign(&signer(1), genesis_header(1), body),
            Err(RosterOpError::Malformed("too many members"))
        ));
        let members: Vec<MemberEntry> = (1..=50u8).map(|i| member(i, true)).collect();
        let body = RosterBody::Genesis {
            nonce: [0; 16],
            name: "big".into(),
            admins: vec![card(1)],
            members,
            settings: 0,
        };
        assert!(RosterOp::sign(&signer(1), genesis_header(1), body).is_ok());
    }

    #[test]
    fn decoder_rejects_unknown_kind_trailing_bytes_and_bad_keys() {
        let g = genesis();
        let mut bytes = g.encode();
        bytes[1] = 0x55;
        assert!(matches!(
            decode::<RosterOp>(&bytes),
            Err(CodecError::UnknownVariant { .. })
        ));
        let mut bytes = g.encode();
        bytes.push(0);
        assert!(matches!(
            decode::<RosterOp>(&bytes),
            Err(CodecError::TrailingBytes(1))
        ));
        // small-order identity key inside the first admin keycard
        let mut bytes = g.encode();
        let off = 87 + 16 + 2 + "Shelter A".len() + 1;
        bytes[off..off + 32].copy_from_slice(&[0u8; 32]);
        assert_eq!(
            decode::<RosterOp>(&bytes),
            Err(CodecError::Invalid("keycard identity key"))
        );
        assert!(decode::<RosterOp>(&g.encode()[..40]).is_err());
    }

    proptest! {
        #[test]
        fn decoder_never_panics_on_random_bytes(bytes in proptest::collection::vec(any::<u8>(), 0..600)) {
            let _ = decode::<RosterOp>(&bytes);
            let _ = RosterOp::from_bytes(&bytes);
        }

        #[test]
        fn decoder_never_panics_on_mutated_ops(
            which in 0usize..3,
            flips in proptest::collection::vec((0usize..400, 0u8..=255), 1..6),
        ) {
            let g = genesis();
            let gid = g.group_id();
            let heads = vec![g.op_hash()];
            let op = match which {
                0 => g,
                1 => {
                    let sub = member(9, false);
                    let end = RosterOp::endorse(&signer(2), &gid, &sub.key(), &sub.name, true).unwrap();
                    RosterOp::sign(
                        &signer(1),
                        header(1, kind::ROSTER_ADD, gid, heads),
                        RosterBody::Add { subject: sub, endorsement: end },
                    )
                    .unwrap()
                }
                _ => {
                    let mut admins = vec![signer(1).public(), signer(2).public()];
                    admins.sort_unstable();
                    let cosigs = vec![RosterOp::cosign(&signer(1), &gid, 1, &admins)];
                    RosterOp::sign(
                        &signer(1),
                        header(1, kind::ROSTER_ADMIN_SET, gid, heads),
                        RosterBody::AdminSet { version: 1, admins, cosigs },
                    )
                    .unwrap()
                }
            };
            let mut bytes = op.encode();
            for (pos, val) in flips {
                let i = pos % bytes.len();
                bytes[i] = val;
            }
            if let Ok(back) = decode::<RosterOp>(&bytes) {
                let _ = back.verify();
                prop_assert_eq!(back.encode(), bytes);
            }
        }

        #[test]
        fn heads_cap_is_enforced_by_header(n in 0usize..=MAX_HEADS) {
            let g = genesis();
            let gid = g.group_id();
            let heads: Vec<Hash32> = (0..n).map(|i| {
                let mut b = [0u8; 32];
                b[0] = i as u8;
                b[1] = 1;
                Hash32::from_bytes(b)
            }).collect();
            let h = header(1, kind::ROSTER_REMOVE, gid, heads);
            let r = RosterOp::sign(&signer(1), h, RosterBody::Remove { subject: signer(3).public() });
            prop_assert_eq!(r.is_ok(), n >= 1);
        }
    }
}
