//! The `Link` trait and its identifiers (docs/DESIGN.md §7).
//!
//! A link is one BLE connection (or one simulated radio pairing) after the
//! Noise NN handshake. The core never blocks on it: `send` enqueues a whole
//! frame and the platform drains it; `recv` yields the next complete frame.
//! Order per link is guaranteed by the transport; a lost chunk tears the link
//! down and a new link (new Noise session) is made.

use crate::crypto::ed25519::IdentityKey;
use crate::relay::Lane;

/// Node-local link identifier (never on the air).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkId(pub u32);

/// Who initiated the connection. The central runs the Noise initiator side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Central,
    Peripheral,
}

/// What the platform could tell about the peer before any handshake
/// (advert type), used only as a scheduling hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PeerKind {
    #[default]
    Unknown,
    /// Advertised the app's fixed service UUID (A build, or foreground).
    AppUuid,
    /// Only Apple manufacturer data 0x004C was visible (a locked iPhone).
    AppleOverflow,
    /// A keyless relay station.
    Station,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkInfo {
    pub role: Role,
    pub peer_kind_hint: PeerKind,
    /// ATT MTU the platform negotiated (chunking happens below this trait).
    pub mtu_hint: u16,
}

/// What this phone knows about the peer after the handshake and the optional
/// ContactProof exchange (DESIGN §2.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkAuth {
    /// No proof: the peer is any phone running the app.
    Anonymous,
    /// The peer proved it is one of this phone's QR-verified contacts.
    VerifiedContact(IdentityKey),
}

/// Whether this phone could validate a cell it received on a link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Validation {
    /// Tag recognised and the inner checks passed.
    Validated,
    /// Nothing recognised.
    Opaque,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum LinkError {
    /// The link's send queue is full; the caller retries later.
    #[error("link busy")]
    Busy,
    /// The link is gone; the caller drops it.
    #[error("link closed")]
    Closed,
    /// Frame larger than the link accepts.
    #[error("frame too large: {0} bytes")]
    TooLarge(usize),
}

/// One connection to one peer, after the Noise handshake.
pub trait Link {
    fn id(&self) -> LinkId;
    fn info(&self) -> &LinkInfo;
    fn auth(&self) -> LinkAuth;
    /// Enqueues one whole frame (already Noise-encrypted by the caller's
    /// link layer). `Err(Busy)` means back-pressure, not failure.
    fn send(&mut self, frame_bytes: &[u8]) -> Result<(), LinkError>;
    /// The next complete inbound frame, if any.
    fn recv(&mut self) -> Option<Vec<u8>>;

    /// Lane for a cell received on this link (BRIEF_v2 §5.5): L1 if this
    /// phone validated it, L2 if the link is to a verified contact, else L3.
    fn lane_for(&self, validated: Validation) -> Lane {
        match (validated, self.auth()) {
            (Validation::Validated, _) => Lane::L1,
            (Validation::Opaque, LinkAuth::VerifiedContact(_)) => Lane::L2,
            (Validation::Opaque, LinkAuth::Anonymous) => Lane::L3,
        }
    }
}
