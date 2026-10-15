//! Key-bound contacts: the pin store, badges, pin sources and the local
//! re-pin (BRIEF_v2 §3 "Identity & trust UX"; DESIGN.md §1 row
//! `people/contacts.rs`; DECISIONS D5 rules R1, R2, R4, R7, R8).
//!
//! **One contact = one key** (BRIEF §3; D5 R1). A [`Contact`] is addressed
//! by its Ed25519 [`IdentityKey`] and nothing else: the badge, petname,
//! verification date, pins and the "stop trusting" flag all hang off the
//! key. Names are display data that the key *delivered*; two entries may
//! carry the same name (that is what the same-name marker in
//! [`super::samename`] is for) and no operation here ever looks an entry up
//! by name. Binding to a self-asserted name or ID is the Bridgefy TOCTOU
//! pattern the brief rules out (BRIEF §3 [E18]; D5 R1, V2).
//!
//! **Pinning** (BRIEF §3 "Pinning"; D5 R2): a key is pinned only when the
//! user scanned it, exchanged a 1:1 message with it, or shares a group
//! roster with it. **Never pin keys from radio encounters.** [`PinSource`]
//! therefore has exactly those three variants and no "seen on the radio"
//! variant; a relay or handshake encounter cannot be expressed as a pin
//! source, so [`ContactStore::pin`] refuses it by construction (there is no
//! value to pass). The pin store is encrypted at rest by the SQLite [`KvTxn`]
//! implementation (DESIGN §1 `storage/kv.rs`); this module only sees
//! namespaced bytes.
//!
//! **Badges** (BRIEF §3 table; §7 vocabulary row "Identity"): `Unverified`
//! is what a key seen in a shared group or a 1:1 chat gets; `Verified in
//! person` is set by the user's own QR scan; `Vouched` is P1 and `Official`
//! is P2, both kept in the enum so the §3 table is representable, with
//! [`ContactStore::vouch`] returning [`ContactError::NotBuiltAtP0`].
//!
//! **Re-pin** (BRIEF §3 "Re-pin"; D5 R7): only after the user's own in-person
//! scan of the new key. The new key is a separate entry with the Verified
//! badge and inherits nothing (D5 R4); the old entry is marked
//! `stop_trusting` on this phone only. Nothing is signed, sent or gossiped
//! (D5 R8: "zero network side effect"); spreading it is P1 (§9 #8).

use std::fmt;

use crate::codec::{CodecError, Decode, Encode, ListItem, MAX_LIST_U8, Reader, Writer, decode};
use crate::crypto::backend::GroupId;
use crate::crypto::ed25519::{IdentityKey, PUBLIC_KEY_LEN, admit_public_key};
use crate::objects::roster_op::MAX_NAME_BYTES;
use crate::storage::kv::KvTxn;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

/// Storage namespace of the pin store. Keys are the 32 identity-key bytes,
/// values are [`Contact`] encodings. Encrypted at rest by the SQLite
/// implementation (BRIEF §3 "The pin store is encrypted at rest").
pub const NS_CONTACTS: &str = "contacts";

/// Byte ceiling of a petname (DESIGN §1: `petname: String (≤64 B)`).
pub const MAX_PETNAME_BYTES: usize = 64;

/// Byte ceiling of a delivered display name. Display names arrive inside
/// roster entries, whose limit is [`MAX_NAME_BYTES`] (DESIGN §2.4), so the
/// store accepts exactly that.
pub const MAX_DISPLAY_NAME_BYTES: usize = MAX_NAME_BYTES;

/// Version byte at the front of every stored [`Contact`] blob, so a later
/// release can migrate the pin store (DESIGN §2.1: every object carries a
/// `ver` byte and decoders reject unknown versions).
pub const CONTACT_VER: u8 = 1;

// ---- badge ------------------------------------------------------------------

/// The identity level of a contact (BRIEF §3 table; BRIEF §7 row
/// "Identity": Unverified / Vouched / Verified in person / Official, never
/// Trusted / Signed / Key-verified).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Badge {
    /// Level 0: key seen in a shared group or 1:1 chat (P0).
    Unverified,
    /// Level 1 (P1): endorsed by people the user has verified. `via` is the
    /// sorted set of verified keys that vouched. Not produced at P0:
    /// [`ContactStore::vouch`] returns [`ContactError::NotBuiltAtP0`].
    Vouched { via: Vec<IdentityKey> },
    /// Level 2: mutual QR scan (P0). `at_display` is the wall-clock
    /// millisecond of the scan, **display only** ("You scanned this phone's
    /// code on 12 Oct"); no decision reads it (BRIEF §4 "no timestamps decide
    /// anything").
    VerifiedInPerson { at_display: i64 },
    /// Level +: delegated organisational certificate (P2, §2a). Unused at P0;
    /// kept so the §3 table is representable.
    Official,
}

impl Badge {
    /// Discriminant used in the stored encoding.
    const fn tag(&self) -> u8 {
        match self {
            Badge::Unverified => 0,
            Badge::Vouched { .. } => 1,
            Badge::VerifiedInPerson { .. } => 2,
            Badge::Official => 3,
        }
    }

    /// Rank on the §3 ladder: a pin never lowers a badge.
    const fn level(&self) -> u8 {
        match self {
            Badge::Unverified => 0,
            Badge::Vouched { .. } => 1,
            Badge::VerifiedInPerson { .. } => 2,
            Badge::Official => 3,
        }
    }

    /// The badge word (BRIEF §7 "Identity" row, "Use" column; icon + text,
    /// never colour alone, BRIEF §3).
    pub const fn text(&self) -> &'static str {
        match self {
            Badge::Unverified => "Unverified",
            Badge::Vouched { .. } => "Vouched",
            Badge::VerifiedInPerson { .. } => "Verified in person",
            Badge::Official => "Official",
        }
    }
}

impl Encode for Badge {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.tag());
        match self {
            Badge::Unverified | Badge::Official => Ok(()),
            Badge::Vouched { via } => {
                if !strictly_ascending(via) {
                    return Err(CodecError::Invalid("vouched via (unsorted)"));
                }
                w.list_u8(via, |w, k| {
                    w.bytes(k.as_bytes());
                    Ok(())
                })
            }
            Badge::VerifiedInPerson { at_display } => {
                w.i64(*at_display);
                Ok(())
            }
        }
    }
}

impl Decode for Badge {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        match r.u8()? {
            0 => Ok(Badge::Unverified),
            1 => {
                let via = r.list_u8(|r| read_key(r, "vouched via key"))?;
                if !strictly_ascending(&via) {
                    return Err(CodecError::Invalid("vouched via (unsorted)"));
                }
                Ok(Badge::Vouched { via })
            }
            2 => Ok(Badge::VerifiedInPerson {
                at_display: r.i64()?,
            }),
            3 => Ok(Badge::Official),
            other => Err(crate::codec::unknown_variant("badge", other)),
        }
    }
}

// ---- pin source -------------------------------------------------------------

/// Why a key is in the pin store (BRIEF §3 "Pinning"; D5 R2). These are the
/// only three reasons; **there is deliberately no radio-encounter variant**,
/// so "pin everything we hear" (K28) cannot be written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PinSource {
    /// The user scanned this key's QR code on this phone. `at_mono` is the
    /// powered-on millisecond of the scan (ordering only, never displayed).
    QrScan { at_mono: u64 },
    /// The user exchanged a 1:1 message with this key.
    DirectMessage,
    /// This key and the user share the roster of this group.
    SharedGroup(GroupId),
}

impl PinSource {
    const fn tag(&self) -> u8 {
        match self {
            PinSource::QrScan { .. } => 0,
            PinSource::DirectMessage => 1,
            PinSource::SharedGroup(_) => 2,
        }
    }

    /// True for the user's own in-person scan, the only source that
    /// verifies (BRIEF §3 level 2) and the only one that permits a re-pin
    /// (D5 R7).
    pub const fn is_scan(&self) -> bool {
        matches!(self, PinSource::QrScan { .. })
    }
}

impl ListItem for PinSource {}

impl Encode for PinSource {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.tag());
        match self {
            PinSource::QrScan { at_mono } => {
                w.u64(*at_mono);
            }
            PinSource::DirectMessage => {}
            PinSource::SharedGroup(g) => {
                w.bytes(g.as_bytes());
            }
        }
        Ok(())
    }
}

impl Decode for PinSource {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        match r.u8()? {
            0 => Ok(PinSource::QrScan { at_mono: r.u64()? }),
            1 => Ok(PinSource::DirectMessage),
            2 => Ok(PinSource::SharedGroup(GroupId::decode_from(r)?)),
            other => Err(crate::codec::unknown_variant("pin source", other)),
        }
    }
}

// ---- contact ----------------------------------------------------------------

/// One pinned key and everything bound to it (BRIEF §3 "One contact = one
/// key"; D5 R1). Stored under [`NS_CONTACTS`] keyed by `key`.
///
/// `link_key` is `k_pair` (DESIGN §2.3), a symmetric secret, so the value
/// is wiped on drop and `Debug` never prints it (same convention as
/// `pairwise::Paired`). The public keys and names are shown.
#[derive(Clone, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct Contact {
    /// The identity. Everything else is bound to this.
    #[zeroize(skip)]
    pub key: IdentityKey,
    /// The X25519 session key the key's keycard binds (DESIGN §1
    /// `crypto/keycard.rs`).
    pub x25519: [u8; 32],
    /// Local name the user chose for this key (≤ [`MAX_PETNAME_BYTES`]).
    /// Empty means "none"; the UI then shows `display_name`.
    pub petname: String,
    /// The name this key delivered inside an encrypted payload (≤
    /// [`MAX_DISPLAY_NAME_BYTES`]). Never used to look the entry up.
    pub display_name: String,
    #[zeroize(skip)]
    pub badge: Badge,
    /// Every reason this key is pinned, in the order they were added, each
    /// at most once ([`ContactError::DuplicatePin`] otherwise).
    #[zeroize(skip)]
    pub pins: Vec<PinSource>,
    /// Pairwise link key, when a session exists (DESIGN §1 `pairwise/*`).
    pub link_key: Option<[u8; 32]>,
    /// Envelope public key, when known (DESIGN §1 `wire/seal.rs`).
    pub envelope_pub: Option<[u8; 32]>,
    /// Set by [`ContactStore::repin_after_scan`] on the *old* entry: "Stop
    /// trusting Ravi's other phone". Local only (D5 R7); never propagates.
    pub stop_trusting: bool,
}

impl fmt::Debug for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Contact")
            .field("key", &self.key)
            .field("x25519", &hex::encode(self.x25519))
            .field("petname", &self.petname)
            .field("display_name", &self.display_name)
            .field("badge", &self.badge)
            .field("pins", &self.pins)
            .field("link_key", &self.link_key.as_ref().map(|_| "<redacted>"))
            .field("envelope_pub", &self.envelope_pub.map(hex::encode))
            .field("stop_trusting", &self.stop_trusting)
            .finish()
    }
}

impl Contact {
    /// A fresh entry for a key met in a shared group or a 1:1 chat: badge
    /// `Unverified`, no pins yet, nothing inherited from any other entry
    /// (BRIEF §3 level 0; D5 R4).
    pub fn new(
        key: IdentityKey,
        x25519: [u8; 32],
        display_name: &str,
    ) -> Result<Self, ContactError> {
        check_display_name(display_name)?;
        Ok(Contact {
            key,
            x25519,
            petname: String::new(),
            display_name: display_name.to_owned(),
            badge: Badge::Unverified,
            pins: Vec::new(),
            link_key: None,
            envelope_pub: None,
            stop_trusting: false,
        })
    }

    /// The name the UI shows for this entry: the petname when the user set
    /// one, else the delivered display name.
    pub fn shown_name(&self) -> &str {
        if self.petname.is_empty() {
            &self.display_name
        } else {
            &self.petname
        }
    }

    /// True when the user's own scan is among the pins (D5 R7's
    /// precondition).
    pub fn has_scan(&self) -> bool {
        self.pins.iter().any(PinSource::is_scan)
    }

    fn check(&self) -> Result<(), ContactError> {
        check_petname(&self.petname)?;
        check_display_name(&self.display_name)?;
        if self.pins.len() > MAX_LIST_U8 {
            return Err(ContactError::TooManyPins);
        }
        if has_duplicate_pin(&self.pins) {
            return Err(ContactError::DuplicatePin);
        }
        Ok(())
    }

    /// Adds `source` once, applying the badge rules (BRIEF §3 table): a scan
    /// sets `Verified in person` dated `at_display` (a later scan refreshes
    /// the date; an `Official` badge is never lowered); a group or 1:1
    /// source leaves the badge alone (a new entry is already `Unverified`).
    fn add_pin(&mut self, source: PinSource, at_display: i64) -> Result<(), ContactError> {
        if !self.pins.contains(&source) {
            if self.pins.len() >= MAX_LIST_U8 {
                return Err(ContactError::TooManyPins);
            }
            self.pins.push(source);
        }
        let verified = Badge::VerifiedInPerson { at_display };
        if source.is_scan() && self.badge.level() <= verified.level() {
            self.badge = verified;
        }
        Ok(())
    }
}

fn check_petname(petname: &str) -> Result<(), ContactError> {
    if petname.len() > MAX_PETNAME_BYTES {
        return Err(ContactError::PetnameTooLong(petname.len()));
    }
    Ok(())
}

fn check_display_name(name: &str) -> Result<(), ContactError> {
    if name.len() > MAX_DISPLAY_NAME_BYTES {
        return Err(ContactError::DisplayNameTooLong(name.len()));
    }
    Ok(())
}

fn strictly_ascending(keys: &[IdentityKey]) -> bool {
    keys.windows(2).all(|w| w[0] < w[1])
}

/// True when some pin source appears twice (exact equality: two scans at
/// different powered-on times are distinct).
fn has_duplicate_pin(pins: &[PinSource]) -> bool {
    pins.iter()
        .enumerate()
        .any(|(i, p)| pins.iter().skip(i.saturating_add(1)).any(|q| q == p))
}

fn read_key(r: &mut Reader<'_>, what: &'static str) -> Result<IdentityKey, CodecError> {
    let bytes: [u8; PUBLIC_KEY_LEN] = r.bytes()?;
    admit_public_key(&bytes).map_err(|_| CodecError::Invalid(what))
}

fn write_opt32(w: &mut Writer, v: &Option<[u8; 32]>) {
    match v {
        None => {
            w.u8(0);
        }
        Some(bytes) => {
            w.u8(1);
            w.bytes(bytes);
        }
    }
}

fn read_opt32(r: &mut Reader<'_>, what: &'static str) -> Result<Option<[u8; 32]>, CodecError> {
    match r.u8()? {
        0 => Ok(None),
        1 => Ok(Some(r.bytes()?)),
        _ => Err(CodecError::Invalid(what)),
    }
}

/// Layout (stored blob, not a wire object; DESIGN §2.1 style, one encoding
/// per value): `ver u8 ‖ key [32] ‖ x25519 [32] ‖ petname str16 ‖
/// display_name str16 ‖ badge ‖ pins list_u8 ‖ link_key (0 | 1‖[32]) ‖
/// envelope_pub (0 | 1‖[32]) ‖ stop_trusting bool`. The two optional keys
/// use a presence byte that must be exactly 0 or 1 so every value still has
/// one encoding.
impl Encode for Contact {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        if self.petname.len() > MAX_PETNAME_BYTES {
            return Err(CodecError::Invalid("petname"));
        }
        if self.display_name.len() > MAX_DISPLAY_NAME_BYTES {
            return Err(CodecError::Invalid("display name"));
        }
        if has_duplicate_pin(&self.pins) {
            return Err(CodecError::Invalid("pins (duplicate)"));
        }
        w.u8(CONTACT_VER);
        w.bytes(self.key.as_bytes());
        w.bytes(&self.x25519);
        w.str16(&self.petname)?;
        w.str16(&self.display_name)?;
        self.badge.encode_to(w)?;
        self.pins.encode_to(w)?;
        write_opt32(w, &self.link_key);
        write_opt32(w, &self.envelope_pub);
        w.bool(self.stop_trusting);
        Ok(())
    }
}

impl Decode for Contact {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let ver = r.u8()?;
        if ver != CONTACT_VER {
            return Err(crate::codec::unknown_variant("contact ver", ver));
        }
        let key = read_key(r, "contact key")?;
        let x25519 = r.bytes()?;
        let petname = r.str16()?;
        if petname.len() > MAX_PETNAME_BYTES {
            return Err(CodecError::Invalid("petname"));
        }
        let display_name = r.str16()?;
        if display_name.len() > MAX_DISPLAY_NAME_BYTES {
            return Err(CodecError::Invalid("display name"));
        }
        let badge = Badge::decode_from(r)?;
        let pins = Vec::<PinSource>::decode_from(r)?;
        if has_duplicate_pin(&pins) {
            return Err(CodecError::Invalid("pins (duplicate)"));
        }
        let link_key = read_opt32(r, "link key presence")?;
        let envelope_pub = read_opt32(r, "envelope pub presence")?;
        let stop_trusting = r.bool()?;
        Ok(Contact {
            key,
            x25519,
            petname: petname.to_owned(),
            display_name: display_name.to_owned(),
            badge,
            pins,
            link_key,
            envelope_pub,
            stop_trusting,
        })
    }
}

// ---- errors -----------------------------------------------------------------

/// Why a pin-store operation refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ContactError {
    /// `Vouched` (BRIEF §3 level 1) ships at P1.
    #[error("vouching is not built at P0 (BRIEF §3 level 1 ships at P1)")]
    NotBuiltAtP0,
    /// No entry for this key; pins attach to existing entries only.
    #[error("no contact pinned for key {0}")]
    UnknownContact(IdentityKey),
    /// `repin_after_scan` was given a `new_contact` whose key is not `new_key`.
    #[error("new contact's key does not match new_key")]
    KeyMismatch,
    /// `first_contact` for a pinned key with a different X25519 session key
    /// than the one pinned; nothing is written (DESIGN §1 `groups/group.rs`
    /// atomic SKDM rule applied to the pin store).
    #[error("pinned x25519 differs from the KeyCard offered")]
    X25519Mismatch,
    /// `repin_after_scan` with `old_key == new_key`: a re-pin is a different
    /// key by definition.
    #[error("old and new key are the same key")]
    SameKey,
    /// `repin_after_scan` without the user's own scan among the new entry's
    /// pins (D5 R7: "only after the user's own in-person QR scan").
    #[error("re-pin requires the user's own scan of the new key")]
    RequiresScan,
    #[error("petname is {0} bytes, max {MAX_PETNAME_BYTES}")]
    PetnameTooLong(usize),
    #[error("display name is {0} bytes, max {MAX_DISPLAY_NAME_BYTES}")]
    DisplayNameTooLong(usize),
    #[error("too many pin sources (max {MAX_LIST_U8})")]
    TooManyPins,
    /// The same pin source listed twice (each reason is recorded once).
    #[error("duplicate pin source")]
    DuplicatePin,
    /// A stored blob did not decode. The store is local and encrypted, so
    /// this is corruption, not an attack surface; it is still never a panic.
    #[error("stored contact is corrupt: {0}")]
    Corrupt(CodecError),
}

// ---- store ------------------------------------------------------------------

/// What [`ContactStore::repin_after_scan`] did (BRIEF §3 "Re-pin").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepinOutcome {
    /// The separate new entry, badge `Verified in person`, nothing inherited.
    pub new_entry: Contact,
    /// The old entry, now `stop_trusting = true` on this phone only.
    pub old_entry: Contact,
}

impl RepinOutcome {
    /// "This is Ravi's phone now (you scanned it today)" (BRIEF §3 "Re-pin";
    /// D5 R7). `name` is the old entry's shown name.
    pub fn text(&self) -> String {
        format!(
            "This is {}'s phone now (you scanned it today)",
            self.old_entry.shown_name()
        )
    }

    /// The local-only offer that accompanies it: "Stop trusting Ravi's other
    /// phone" (BRIEF §3 "Re-pin"; D5 R7).
    pub fn stop_trusting_offer_text(&self) -> String {
        format!(
            "Stop trusting {}'s other phone",
            self.old_entry.shown_name()
        )
    }
}

/// The pin store over one storage transaction (DESIGN §1 `people/contacts.rs`:
/// "Pin store (encrypted blobs), badges, pin sources, local re-pin").
///
/// Every method is a pure function of the transaction's contents plus its
/// arguments, so two phones applying the same calls hold the same bytes.
pub struct ContactStore<'a> {
    txn: &'a mut dyn KvTxn,
}

impl fmt::Debug for ContactStore<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ContactStore")
    }
}

impl<'a> ContactStore<'a> {
    pub fn new(txn: &'a mut dyn KvTxn) -> Self {
        Self { txn }
    }

    /// The entry for `key`, if pinned.
    pub fn get(&self, key: &IdentityKey) -> Result<Option<Contact>, ContactError> {
        match self.txn.get(NS_CONTACTS, key.as_bytes()) {
            None => Ok(None),
            Some(bytes) => {
                // the blob carries `link_key`; wipe the copy after decoding
                let bytes = Zeroizing::new(bytes);
                decode::<Contact>(&bytes)
                    .map(Some)
                    .map_err(ContactError::Corrupt)
            }
        }
    }

    /// Writes `contact` under its own key, replacing any previous entry.
    pub fn upsert(&mut self, contact: &Contact) -> Result<(), ContactError> {
        contact.check()?;
        let bytes = Zeroizing::new(contact.try_encode().map_err(ContactError::Corrupt)?);
        self.txn.put(NS_CONTACTS, contact.key.as_bytes(), &bytes);
        Ok(())
    }

    /// Deletes the entry for `key` (a dropped contact). Nothing else
    /// referencing the key is touched: roster slots, claims and ballots are
    /// bound to the key, not to this entry (D5 R1).
    pub fn remove(&mut self, key: &IdentityKey) {
        self.txn.delete(NS_CONTACTS, key.as_bytes());
    }

    /// Every pinned contact in key order (deterministic: the namespace scan
    /// is byte-ordered).
    pub fn list(&self) -> Result<Vec<Contact>, ContactError> {
        self.txn
            .scan_prefix(NS_CONTACTS, &[])
            .into_iter()
            .map(|(_, v)| {
                let v = Zeroizing::new(v);
                decode::<Contact>(&v).map_err(ContactError::Corrupt)
            })
            .collect()
    }

    /// Records a reason to pin `key` (BRIEF §3 "Pinning"; D5 R2) on an
    /// existing entry and applies the badge rules (see [`Contact`]). The
    /// only sources that exist are the user's scan, a 1:1 message and a
    /// shared group; **a radio encounter is not a [`PinSource`] and can never
    /// be pinned** (K28). `at_display` is the wall-clock millisecond of the
    /// call, used only to date a `Verified in person` badge for display.
    ///
    /// Returns [`ContactError::UnknownContact`] when `key` has no entry:
    /// create one first with [`ContactStore::first_contact`], which needs the
    /// key's X25519 key and delivered name.
    pub fn pin(
        &mut self,
        key: &IdentityKey,
        source: PinSource,
        at_display: i64,
    ) -> Result<Contact, ContactError> {
        let mut contact = self.get(key)?.ok_or(ContactError::UnknownContact(*key))?;
        contact.add_pin(source, at_display)?;
        self.upsert(&contact)?;
        Ok(contact)
    }

    /// Pins a key met for the first time in a shared group or a 1:1 chat:
    /// creates the entry as `Unverified` (BRIEF §3 level 0) with `source`
    /// as its first pin, or adds `source` to the existing entry (a scan
    /// source verifies, as in [`ContactStore::pin`]). The delivered
    /// `display_name` is recorded on a new entry and updated on an existing
    /// one; nothing is keyed on it.
    ///
    /// `x25519` is the session key the key's KeyCard binds (DESIGN §2.4).
    /// An existing entry keeps the x25519 it was pinned with: if `x25519`
    /// differs, nothing is written and [`ContactError::X25519Mismatch`] is
    /// returned, mirroring the atomic SKDM check (DESIGN §1 `groups/group.rs`:
    /// "mismatched x25519 → no store write"). A re-issued KeyCard for the
    /// same identity is the node's decision to apply with
    /// [`ContactStore::upsert`] once it has verified the card.
    pub fn first_contact(
        &mut self,
        key: &IdentityKey,
        x25519: [u8; 32],
        display_name: &str,
        source: PinSource,
        at_display: i64,
    ) -> Result<Contact, ContactError> {
        check_display_name(display_name)?;
        let mut contact = match self.get(key)? {
            Some(mut c) => {
                if c.x25519 != x25519 {
                    return Err(ContactError::X25519Mismatch);
                }
                c.display_name = display_name.to_owned();
                c
            }
            None => Contact::new(*key, x25519, display_name)?,
        };
        contact.add_pin(source, at_display)?;
        self.upsert(&contact)?;
        Ok(contact)
    }

    /// Sets the local petname of `key` (≤ [`MAX_PETNAME_BYTES`]; empty
    /// clears it). The caller also tells the same-name index
    /// ([`super::samename::SameNameIndex::set_petname`]), since petnames
    /// take part in the match (D5 R3).
    pub fn set_petname(
        &mut self,
        key: &IdentityKey,
        petname: &str,
    ) -> Result<Contact, ContactError> {
        check_petname(petname)?;
        let mut contact = self.get(key)?.ok_or(ContactError::UnknownContact(*key))?;
        contact.petname = petname.to_owned();
        self.upsert(&contact)?;
        Ok(contact)
    }

    /// Flips the local-only "Stop trusting Ravi's other phone" flag (D5 R7),
    /// so the user can take the offer later or undo it.
    pub fn set_stop_trusting(
        &mut self,
        key: &IdentityKey,
        stop: bool,
    ) -> Result<Contact, ContactError> {
        let mut contact = self.get(key)?.ok_or(ContactError::UnknownContact(*key))?;
        contact.stop_trusting = stop;
        self.upsert(&contact)?;
        Ok(contact)
    }

    /// Re-pin after the user's own in-person scan of `new_key` (BRIEF §3
    /// "Re-pin"; D5 R7):
    ///
    /// * `new_contact` (whose `key` must be `new_key`, with the user's
    ///   [`PinSource::QrScan`] among its pins) is stored as a **separate**
    ///   entry with badge `Verified in person` dated `at_display`. It
    ///   inherits nothing from `old_key`'s entry: not the petname, thread,
    ///   roster slot, roles, claims or ballots (D5 R4). Any entry already
    ///   stored for `new_key` is replaced by `new_contact`.
    /// * `old_key`'s entry gets `stop_trusting = true`, **on this phone
    ///   only** (BRIEF §3 "local only"). Nothing is signed or sent (D5 R8);
    ///   removing the old key from groups stays an admin roster operation
    ///   (D5 R7, decision 3).
    ///
    /// Fails with [`ContactError::SameKey`], [`ContactError::KeyMismatch`],
    /// [`ContactError::RequiresScan`] or [`ContactError::UnknownContact`]
    /// (for `old_key`) without writing anything.
    pub fn repin_after_scan(
        &mut self,
        old_key: &IdentityKey,
        new_key: &IdentityKey,
        new_contact: Contact,
        at_display: i64,
    ) -> Result<RepinOutcome, ContactError> {
        if old_key == new_key {
            return Err(ContactError::SameKey);
        }
        if new_contact.key != *new_key {
            return Err(ContactError::KeyMismatch);
        }
        if !new_contact.has_scan() {
            return Err(ContactError::RequiresScan);
        }
        let mut old_entry = self
            .get(old_key)?
            .ok_or(ContactError::UnknownContact(*old_key))?;
        let mut new_entry = new_contact;
        new_entry.badge = Badge::VerifiedInPerson { at_display };
        new_entry.stop_trusting = false;
        new_entry.check()?;
        old_entry.stop_trusting = true;
        self.upsert(&new_entry)?;
        self.upsert(&old_entry)?;
        Ok(RepinOutcome {
            new_entry,
            old_entry,
        })
    }

    /// Level 1 "Vouched" (BRIEF §3) ships at P1; at P0 this always returns
    /// [`ContactError::NotBuiltAtP0`] and writes nothing.
    pub fn vouch(
        &mut self,
        _key: &IdentityKey,
        _via: &[IdentityKey],
    ) -> Result<Contact, ContactError> {
        Err(ContactError::NotBuiltAtP0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::ed25519::IdentitySigner;
    use crate::storage::kv::MemKv;
    use proptest::prelude::*;

    fn key(i: u8) -> IdentityKey {
        let mut seed = [0u8; 32];
        seed[0] = i;
        seed[1] = 0xC3;
        IdentitySigner::from_seed(&seed).public()
    }

    fn contact(i: u8, name: &str) -> Contact {
        Contact::new(key(i), [i; 32], name).expect("short name")
    }

    #[test]
    fn pin_source_has_no_radio_variant() {
        // BRIEF §3 "Never pin keys from radio encounters": the enum is
        // exhaustively the three user-driven reasons. If a variant is ever
        // added this match stops compiling and the rule must be re-argued.
        let all = [
            PinSource::QrScan { at_mono: 1 },
            PinSource::DirectMessage,
            PinSource::SharedGroup(GroupId([1; 16])),
        ];
        for s in all {
            match s {
                PinSource::QrScan { .. } | PinSource::DirectMessage | PinSource::SharedGroup(_) => {
                }
            }
        }
        assert!(all[0].is_scan());
        assert!(!all[1].is_scan());
        assert!(!all[2].is_scan());
    }

    #[test]
    fn badge_text_uses_brief_vocabulary() {
        assert_eq!(Badge::Unverified.text(), "Unverified");
        assert_eq!(Badge::Vouched { via: vec![] }.text(), "Vouched");
        assert_eq!(
            Badge::VerifiedInPerson { at_display: 0 }.text(),
            "Verified in person"
        );
        assert_eq!(Badge::Official.text(), "Official");
    }

    #[test]
    fn first_contact_is_unverified_and_scan_verifies() {
        let mut kv = MemKv::new();
        let mut store = ContactStore::new(&mut kv);
        let g = GroupId([9; 16]);
        let c = store
            .first_contact(&key(1), [1; 32], "Ravi", PinSource::SharedGroup(g), 1_000)
            .unwrap();
        assert_eq!(c.badge, Badge::Unverified);
        assert_eq!(c.pins, vec![PinSource::SharedGroup(g)]);
        // same source again: no duplicate pin
        let c = store
            .pin(&key(1), PinSource::SharedGroup(g), 1_001)
            .unwrap();
        assert_eq!(c.pins.len(), 1);
        // 1:1 message: still Unverified
        let c = store.pin(&key(1), PinSource::DirectMessage, 1_002).unwrap();
        assert_eq!(c.badge, Badge::Unverified);
        assert_eq!(c.pins.len(), 2);
        // scan: Verified in person, dated for display
        let c = store
            .pin(
                &key(1),
                PinSource::QrScan { at_mono: 50 },
                1_700_000_000_000,
            )
            .unwrap();
        assert_eq!(
            c.badge,
            Badge::VerifiedInPerson {
                at_display: 1_700_000_000_000
            }
        );
        // a later group pin never lowers the badge
        let c = store
            .pin(&key(1), PinSource::SharedGroup(GroupId([2; 16])), 5)
            .unwrap();
        assert_eq!(
            c.badge,
            Badge::VerifiedInPerson {
                at_display: 1_700_000_000_000
            }
        );
        assert_eq!(store.get(&key(1)).unwrap(), Some(c));
    }

    #[test]
    fn pin_needs_an_entry_and_vouch_is_p1() {
        let mut kv = MemKv::new();
        let mut store = ContactStore::new(&mut kv);
        assert_eq!(
            store.pin(&key(1), PinSource::DirectMessage, 0),
            Err(ContactError::UnknownContact(key(1)))
        );
        store.upsert(&contact(1, "Ravi")).unwrap();
        assert_eq!(
            store.vouch(&key(1), &[key(2)]),
            Err(ContactError::NotBuiltAtP0)
        );
        assert_eq!(
            store.get(&key(1)).unwrap().unwrap().badge,
            Badge::Unverified
        );
    }

    #[test]
    fn list_is_key_ordered_and_remove_drops() {
        let mut kv = MemKv::new();
        let mut store = ContactStore::new(&mut kv);
        for i in [5u8, 1, 3] {
            store.upsert(&contact(i, "x")).unwrap();
        }
        let listed: Vec<IdentityKey> = store.list().unwrap().iter().map(|c| c.key).collect();
        let mut sorted = listed.clone();
        sorted.sort();
        assert_eq!(listed, sorted);
        assert_eq!(listed.len(), 3);
        store.remove(&key(3));
        assert_eq!(store.list().unwrap().len(), 2);
        assert_eq!(store.get(&key(3)).unwrap(), None);
    }

    #[test]
    fn petname_limits() {
        let mut kv = MemKv::new();
        let mut store = ContactStore::new(&mut kv);
        store.upsert(&contact(1, "Ravi")).unwrap();
        let long = "p".repeat(MAX_PETNAME_BYTES + 1);
        assert_eq!(
            store.set_petname(&key(1), &long),
            Err(ContactError::PetnameTooLong(MAX_PETNAME_BYTES + 1))
        );
        let ok = "p".repeat(MAX_PETNAME_BYTES);
        assert_eq!(store.set_petname(&key(1), &ok).unwrap().petname, ok);
        let mut c = contact(2, "x");
        c.petname = long;
        assert!(matches!(
            store.upsert(&c),
            Err(ContactError::PetnameTooLong(_))
        ));
        assert!(matches!(
            Contact::new(key(3), [0; 32], &"n".repeat(MAX_DISPLAY_NAME_BYTES + 1)),
            Err(ContactError::DisplayNameTooLong(_))
        ));
    }

    #[test]
    fn repin_sets_verified_on_new_and_stop_trusting_on_old() {
        let mut kv = MemKv::new();
        let mut store = ContactStore::new(&mut kv);
        let mut old = contact(1, "Ravi");
        old.petname = "Ravi K".into();
        old.badge = Badge::VerifiedInPerson { at_display: 10 };
        store.upsert(&old).unwrap();
        let mut new = contact(2, "Ravi");
        new.pins.push(PinSource::QrScan { at_mono: 77 });
        let out = store
            .repin_after_scan(&key(1), &key(2), new.clone(), 99)
            .unwrap();
        assert_eq!(
            out.new_entry.badge,
            Badge::VerifiedInPerson { at_display: 99 }
        );
        assert_eq!(out.new_entry.petname, "", "inherits nothing");
        assert!(!out.new_entry.stop_trusting);
        assert!(out.old_entry.stop_trusting);
        assert_eq!(out.old_entry.badge, old.badge, "old badge untouched");
        assert_eq!(store.get(&key(1)).unwrap().unwrap(), out.old_entry);
        assert_eq!(store.get(&key(2)).unwrap().unwrap(), out.new_entry);
        assert_eq!(
            out.text(),
            "This is Ravi K's phone now (you scanned it today)"
        );
        assert_eq!(
            out.stop_trusting_offer_text(),
            "Stop trusting Ravi K's other phone"
        );
        // undo is local too
        assert!(
            !store
                .set_stop_trusting(&key(1), false)
                .unwrap()
                .stop_trusting
        );
    }

    #[test]
    fn repin_refusals_write_nothing() {
        let mut kv = MemKv::new();
        let mut store = ContactStore::new(&mut kv);
        store.upsert(&contact(1, "Ravi")).unwrap();
        let mut new = contact(2, "Ravi");
        assert_eq!(
            store.repin_after_scan(&key(1), &key(2), new.clone(), 0),
            Err(ContactError::RequiresScan)
        );
        new.pins.push(PinSource::QrScan { at_mono: 1 });
        assert_eq!(
            store.repin_after_scan(&key(1), &key(1), new.clone(), 0),
            Err(ContactError::SameKey)
        );
        assert_eq!(
            store.repin_after_scan(&key(1), &key(3), new.clone(), 0),
            Err(ContactError::KeyMismatch)
        );
        assert_eq!(
            store.repin_after_scan(&key(4), &key(2), new, 0),
            Err(ContactError::UnknownContact(key(4)))
        );
        assert_eq!(store.list().unwrap().len(), 1);
        assert!(!store.get(&key(1)).unwrap().unwrap().stop_trusting);
    }

    #[test]
    fn contact_round_trips_and_is_canonical() {
        let mut c = contact(1, "Ravi");
        c.petname = "R".into();
        c.badge = Badge::Vouched {
            via: {
                let mut v = vec![key(2), key(3)];
                v.sort();
                v
            },
        };
        c.pins = vec![
            PinSource::QrScan { at_mono: 5 },
            PinSource::DirectMessage,
            PinSource::SharedGroup(GroupId([3; 16])),
        ];
        c.link_key = Some([7; 32]);
        c.envelope_pub = None;
        c.stop_trusting = true;
        let bytes = c.encode();
        assert_eq!(bytes[0], CONTACT_VER);
        let back: Contact = decode(&bytes).unwrap();
        assert_eq!(back, c);
        assert_eq!(back.encode(), bytes);
        // trailing byte and bad presence byte are rejected
        let mut t = bytes.clone();
        t.push(0);
        assert_eq!(decode::<Contact>(&t), Err(CodecError::TrailingBytes(1)));
        let mut bad_ver = bytes.clone();
        bad_ver[0] = 9;
        assert_eq!(
            decode::<Contact>(&bad_ver),
            Err(crate::codec::unknown_variant("contact ver", 9u8))
        );
        // unsorted vouch list does not encode
        let mut unsorted = c.clone();
        let mut v = vec![key(2), key(3)];
        v.sort();
        v.reverse();
        unsorted.badge = Badge::Vouched { via: v };
        assert_eq!(
            unsorted.try_encode(),
            Err(CodecError::Invalid("vouched via (unsorted)"))
        );
    }

    #[test]
    fn corrupt_blob_is_an_error_not_a_panic() {
        let mut kv = MemKv::new();
        kv.put(NS_CONTACTS, key(1).as_bytes(), &[1, 2, 3]);
        let store = ContactStore::new(&mut kv);
        assert!(matches!(store.get(&key(1)), Err(ContactError::Corrupt(_))));
        assert!(matches!(store.list(), Err(ContactError::Corrupt(_))));
    }

    #[test]
    fn debug_redacts_link_key_and_contact_is_wiped_on_drop() {
        fn assert_zeroize_on_drop<T: zeroize::ZeroizeOnDrop>() {}
        assert_zeroize_on_drop::<Contact>();
        let mut c = contact(1, "Ravi");
        c.link_key = Some([0xAB; 32]);
        let dbg = format!("{c:?}");
        assert!(!dbg.contains("171"), "{dbg}");
        assert!(!dbg.to_lowercase().contains("abab"), "{dbg}");
        assert!(dbg.contains("Ravi"), "{dbg}");
        let out = RepinOutcome {
            new_entry: c.clone(),
            old_entry: c,
        };
        assert!(!format!("{out:?}").to_lowercase().contains("abab"));
    }

    #[test]
    fn duplicate_pins_are_rejected_everywhere() {
        let mut c = contact(1, "Ravi");
        c.pins = vec![
            PinSource::SharedGroup(GroupId([1; 16])),
            PinSource::DirectMessage,
            PinSource::SharedGroup(GroupId([1; 16])),
        ];
        assert_eq!(c.check(), Err(ContactError::DuplicatePin));
        assert_eq!(c.try_encode(), Err(CodecError::Invalid("pins (duplicate)")));
        let mut kv = MemKv::new();
        let mut store = ContactStore::new(&mut kv);
        assert_eq!(store.upsert(&c), Err(ContactError::DuplicatePin));
        assert_eq!(store.list().unwrap().len(), 0);
        // the same blob hand-built with the duplicate is refused on decode
        let mut w = Writer::new();
        w.u8(CONTACT_VER);
        w.bytes(c.key.as_bytes());
        w.bytes(&c.x25519);
        w.str16("").unwrap();
        w.str16("Ravi").unwrap();
        w.u8(0); // Unverified
        w.u8(2); // two pins
        w.u8(1); // DirectMessage
        w.u8(1); // DirectMessage again
        w.u8(0);
        w.u8(0);
        w.bool(false);
        assert_eq!(
            decode::<Contact>(w.as_bytes()),
            Err(CodecError::Invalid("pins (duplicate)"))
        );
        // two scans at different powered-on times are distinct pins
        let mut ok = contact(2, "Ravi");
        ok.pins = vec![
            PinSource::QrScan { at_mono: 1 },
            PinSource::QrScan { at_mono: 2 },
        ];
        assert_eq!(ok.check(), Ok(()));
    }

    #[test]
    fn decoder_rejects_bad_presence_bytes_and_overlong_names() {
        let c = contact(1, "Ravi");
        let bytes = c.encode();
        // layout tail: ... ‖ pins count (0) ‖ link presence ‖ envelope presence ‖ bool
        let n = bytes.len();
        let mut bad_link = bytes.clone();
        bad_link[n - 3] = 2;
        assert_eq!(
            decode::<Contact>(&bad_link),
            Err(CodecError::Invalid("link key presence"))
        );
        let mut bad_env = bytes.clone();
        bad_env[n - 2] = 0xFF;
        assert_eq!(
            decode::<Contact>(&bad_env),
            Err(CodecError::Invalid("envelope pub presence"))
        );
        let build = |petname: &str, display: &str| {
            let mut w = Writer::new();
            w.u8(CONTACT_VER);
            w.bytes(c.key.as_bytes());
            w.bytes(&c.x25519);
            w.str16(petname).unwrap();
            w.str16(display).unwrap();
            w.u8(0);
            w.u8(0);
            w.u8(0);
            w.u8(0);
            w.bool(false);
            w.into_inner()
        };
        assert_eq!(
            decode::<Contact>(&build(&"p".repeat(MAX_PETNAME_BYTES + 1), "Ravi")),
            Err(CodecError::Invalid("petname"))
        );
        assert_eq!(
            decode::<Contact>(&build("", &"d".repeat(MAX_DISPLAY_NAME_BYTES + 1))),
            Err(CodecError::Invalid("display name"))
        );
        assert!(
            decode::<Contact>(&build(
                &"p".repeat(MAX_PETNAME_BYTES),
                &"d".repeat(MAX_DISPLAY_NAME_BYTES)
            ))
            .is_ok()
        );
    }

    #[test]
    fn second_scan_refreshes_the_verified_date() {
        let mut kv = MemKv::new();
        let mut store = ContactStore::new(&mut kv);
        store.upsert(&contact(1, "Ravi")).unwrap();
        store
            .pin(&key(1), PinSource::QrScan { at_mono: 1 }, 1_000)
            .unwrap();
        let c = store
            .pin(&key(1), PinSource::QrScan { at_mono: 2 }, 2_000)
            .unwrap();
        assert_eq!(c.badge, Badge::VerifiedInPerson { at_display: 2_000 });
        assert_eq!(c.pins.len(), 2);
    }

    #[test]
    fn first_contact_updates_the_delivered_name_of_an_existing_entry() {
        let mut kv = MemKv::new();
        let mut store = ContactStore::new(&mut kv);
        let g = GroupId([1; 16]);
        store
            .first_contact(&key(1), [1; 32], "Ravi", PinSource::SharedGroup(g), 0)
            .unwrap();
        // a different session key for the same identity: refused, nothing written
        assert_eq!(
            store.first_contact(&key(1), [9; 32], "Ravi K", PinSource::DirectMessage, 0),
            Err(ContactError::X25519Mismatch)
        );
        let unchanged = store.get(&key(1)).unwrap().unwrap();
        assert_eq!(unchanged.display_name, "Ravi");
        assert_eq!(unchanged.pins.len(), 1);
        let c = store
            .first_contact(&key(1), [1; 32], "Ravi K", PinSource::DirectMessage, 0)
            .unwrap();
        assert_eq!(c.display_name, "Ravi K");
        assert_eq!(c.x25519, [1; 32]);
        assert_eq!(c.pins.len(), 2);
        assert_eq!(store.list().unwrap().len(), 1);
    }

    #[test]
    fn repin_never_carries_stop_trusting_onto_the_new_entry() {
        let mut kv = MemKv::new();
        let mut store = ContactStore::new(&mut kv);
        store.upsert(&contact(1, "Ravi")).unwrap();
        let mut new = contact(2, "Ravi");
        new.pins.push(PinSource::QrScan { at_mono: 1 });
        new.stop_trusting = true;
        new.badge = Badge::Unverified;
        let out = store.repin_after_scan(&key(1), &key(2), new, 5).unwrap();
        assert!(!out.new_entry.stop_trusting);
        assert_eq!(
            out.new_entry.badge,
            Badge::VerifiedInPerson { at_display: 5 }
        );
        assert!(!store.get(&key(2)).unwrap().unwrap().stop_trusting);
    }

    proptest! {
        #[test]
        fn decoder_never_panics_on_random_bytes(bytes in proptest::collection::vec(any::<u8>(), 0..400)) {
            let _ = decode::<Contact>(&bytes);
            let _ = decode::<Badge>(&bytes);
            let _ = decode::<PinSource>(&bytes);
        }
    }
}
