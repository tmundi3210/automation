//! Per-hop link frame carried inside the Noise NN transport message
//! (docs/DESIGN.md §2.3) and the message classes (BRIEF_v2 §5.6).
//!
//! Frame = `frame_type u8 ‖ class u8 ‖ flags u8 ‖ reserved u8 ‖ payload`
//! with no payload length prefix: the Noise message boundary is the frame
//! boundary. `class` is the cell's [`Class`] for a cell frame and 0 for
//! every control frame (BRIEF_v2 §5.5 "the message class inside the per-hop
//! layer": it is visible to the next hop for scheduling and to nobody
//! else). `flags` and `reserved` are 0; a decoder rejects anything else
//! (DESIGN §2.3: bit0 "relayed-not-origin" is never set) so that no hop can
//! smuggle an origin hint. Decoders are strict: a known type, an exact or
//! bounded payload length for that type, and no trailing bytes.
//!
//! Control payloads (DESIGN §2.3): [`Hello`], [`SummaryChunk`], [`Throttle`]
//! and [`ContactProof`]; the typed [`Frame`] enum pairs a [`FrameType`] with
//! its decoded payload. [`Class`] fixes the priority order the DTN store,
//! relay limits and scheduler key on.
//!
//! Sizes: a cell frame is [`CELL_FRAME_LEN`] = 516 bytes and, with the
//! 2-byte length prefix and the 16-byte Poly1305 tag, a
//! [`CELL_NOISE_MESSAGE_LEN`] = 534-byte Noise message (DESIGN §2.2 "frame =
//! 2 + 4 + 512 + 16 = 534 B"). A summary chunk carries up to
//! [`MAX_SUMMARY_BITS_LEN`] bytes, so the largest frame any decoder accepts
//! is [`MAX_FRAME_LEN`].

use crate::codec::{CodecError, Decode, Encode, ListItem, Reader, Writer, decode, unknown_variant};
use crate::wire::cell::{CELL_LEN, Cell};

/// Message class (BRIEF_v2 §5.6). Lower discriminant = higher priority.
/// Strict priority on every queue: C0 > C1 > C2 > C3 > C4; the scheduler
/// additionally splits C2 and C3 by lane (DECISIONS §3 row 4,8,9,10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Class {
    /// SOS, at most 256 B. Reserved at P0 (owner decision 5: not built).
    C0Sos = 0,
    /// Control: roster ops, Sender Key bundles, vote-opens, ballots, task
    /// Take/Release/Done, revocations. Segmented, objects up to 64 kB.
    C1Control = 1,
    /// Text, at most 2 kB.
    C2Text = 2,
    /// News (P2), at most 4 kB.
    C3News = 3,
    /// Media preview, at most 16 kB (B build: 8–16 kB cap, assumption).
    C4Preview = 4,
}

impl Class {
    pub const ALL: [Class; 5] = [
        Class::C0Sos,
        Class::C1Control,
        Class::C2Text,
        Class::C3News,
        Class::C4Preview,
    ];

    pub fn from_u8(v: u8) -> Result<Self, CodecError> {
        Ok(match v {
            0 => Class::C0Sos,
            1 => Class::C1Control,
            2 => Class::C2Text,
            3 => Class::C3News,
            4 => Class::C4Preview,
            other => return Err(unknown_variant("class", other)),
        })
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Largest object this class may carry, in bytes of plaintext before
    /// fragmentation (BRIEF_v2 §5.6; C1's 64 kB is the segmentation ceiling).
    pub const fn max_object_bytes(self) -> usize {
        match self {
            Class::C0Sos => 256,
            Class::C1Control => 64 * 1024,
            Class::C2Text => 2 * 1024,
            Class::C3News => 4 * 1024,
            Class::C4Preview => 16 * 1024,
        }
    }

    /// `true` if `self` has strictly higher priority than `other`.
    pub fn outranks(self, other: Class) -> bool {
        self < other
    }
}

impl Encode for Class {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.as_u8());
        Ok(())
    }
}

impl Decode for Class {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Class::from_u8(r.u8()?)
    }
}

// ---- sizes ----------------------------------------------------------------

/// `frame_type ‖ class ‖ flags ‖ reserved`.
pub const FRAME_HEADER_LEN: usize = 4;
/// A cell frame: header + one cell.
pub const CELL_FRAME_LEN: usize = FRAME_HEADER_LEN + CELL_LEN;
/// The transport's length prefix in front of every Noise message
/// (DESIGN §1 `wire/noise.rs`: "2-B length framing").
pub const NOISE_LENGTH_PREFIX_LEN: usize = 2;
/// Poly1305 tag a Noise transport message appends.
pub const NOISE_TAG_LEN: usize = 16;
/// A cell frame on the stream: `2 + 4 + CELL_LEN + 16` = 534 bytes
/// (DESIGN §2.2 "Why 512 B").
pub const CELL_NOISE_MESSAGE_LEN: usize = NOISE_LENGTH_PREFIX_LEN + CELL_FRAME_LEN + NOISE_TAG_LEN;
/// Largest `bits` field of one [`SummaryChunk`] (summary vectors are
/// chunked into frames, DESIGN §1 `dtn/summary.rs`; 4096 bytes per chunk).
pub const MAX_SUMMARY_BITS_LEN: usize = 4096;
/// Largest payload of any frame type: a full summary chunk.
pub const MAX_PAYLOAD_LEN: usize = if CELL_LEN > SummaryChunk::MAX_LEN {
    CELL_LEN
} else {
    SummaryChunk::MAX_LEN
};
/// Largest frame a decoder accepts (header + [`MAX_PAYLOAD_LEN`]). Longer
/// input is refused before any payload is looked at.
pub const MAX_FRAME_LEN: usize = FRAME_HEADER_LEN + MAX_PAYLOAD_LEN;
/// Largest Noise message the link layer produces or accepts.
pub const MAX_NOISE_MESSAGE_LEN: usize = NOISE_LENGTH_PREFIX_LEN + MAX_FRAME_LEN + NOISE_TAG_LEN;

const _: () = assert!(CELL_NOISE_MESSAGE_LEN == 534);

// ---- frame type -------------------------------------------------------------

/// What a frame carries (DESIGN §2.3 `frame_type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FrameType {
    /// One on-air [`Cell`]; `class` carries its [`Class`].
    Cell = 0,
    /// Link greeting after the Noise handshake ([`Hello`]).
    Hello = 1,
    /// One chunk of a summary vector ([`SummaryChunk`]).
    Summary = 2,
    /// Backpressure for one class ([`Throttle`]).
    Throttle = 3,
    /// Proof of a QR-verified contact relationship, which lifts the link to
    /// L2 ([`ContactProof`]).
    ContactProof = 4,
}

impl FrameType {
    pub const ALL: [FrameType; 5] = [
        FrameType::Cell,
        FrameType::Hello,
        FrameType::Summary,
        FrameType::Throttle,
        FrameType::ContactProof,
    ];

    pub fn from_u8(v: u8) -> Result<Self, CodecError> {
        Ok(match v {
            0 => FrameType::Cell,
            1 => FrameType::Hello,
            2 => FrameType::Summary,
            3 => FrameType::Throttle,
            4 => FrameType::ContactProof,
            other => return Err(unknown_variant("frame type", other)),
        })
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

impl Encode for FrameType {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.as_u8());
        Ok(())
    }
}

impl Decode for FrameType {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        FrameType::from_u8(r.u8()?)
    }
}

// ---- control payloads -------------------------------------------------------

/// Link greeting (DESIGN §2.3 `Hello = {proto u8, caps u16, is_station u8,
/// buffer_free u32}`; §7 link lifecycle). `is_station` marks a keyless relay
/// station (BRIEF_v2 §5.7); `buffer_free` is the sender's free buffer in
/// bytes, for the peer's push budget (D4 BUFFER).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hello {
    pub proto: u8,
    pub caps: u16,
    pub is_station: bool,
    pub buffer_free: u32,
}

impl Hello {
    /// Exact encoded size.
    pub const LEN: usize = 1 + 2 + 1 + 4;
}

impl Encode for Hello {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.proto)
            .u16(self.caps)
            .bool(self.is_station)
            .u32(self.buffer_free);
        Ok(())
    }
}

impl Decode for Hello {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(Hello {
            proto: r.u8()?,
            caps: r.u16()?,
            is_station: r.bool()?,
            buffer_free: r.u32()?,
        })
    }
}

/// One chunk of a salted Bloom summary vector (DESIGN §2.3; D10 (4)
/// anti-entropy by summary-vector exchange). `n_bits` and `k` describe the
/// whole vector; `bits` is this chunk's slice of it, at most
/// [`MAX_SUMMARY_BITS_LEN`] bytes; `chunk_idx < chunk_total`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SummaryChunk {
    pub salt: u32,
    pub n_bits: u32,
    pub k: u8,
    pub chunk_idx: u16,
    pub chunk_total: u16,
    pub bits: Vec<u8>,
}

impl SummaryChunk {
    /// Fixed fields plus the `bytes16` length prefix.
    pub const FIXED_LEN: usize = 4 + 4 + 1 + 2 + 2 + 2;
    /// Largest encoded size.
    pub const MAX_LEN: usize = Self::FIXED_LEN + MAX_SUMMARY_BITS_LEN;

    fn check(&self) -> Result<(), CodecError> {
        if self.bits.len() > MAX_SUMMARY_BITS_LEN {
            return Err(CodecError::LengthOverflow {
                what: "summary bits",
                max: MAX_SUMMARY_BITS_LEN,
                got: self.bits.len(),
            });
        }
        if self.chunk_idx >= self.chunk_total {
            return Err(CodecError::Invalid("summary chunk index"));
        }
        Ok(())
    }
}

impl Encode for SummaryChunk {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.check()?;
        w.u32(self.salt)
            .u32(self.n_bits)
            .u8(self.k)
            .u16(self.chunk_idx)
            .u16(self.chunk_total);
        w.bytes16(&self.bits)
    }
}

impl Decode for SummaryChunk {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let salt = r.u32()?;
        let n_bits = r.u32()?;
        let k = r.u8()?;
        let chunk_idx = r.u16()?;
        let chunk_total = r.u16()?;
        // bytes16 is a sub-slice of the input: nothing is allocated before
        // the length is known to be present and within the cap.
        let bits = r.bytes16()?;
        if bits.len() > MAX_SUMMARY_BITS_LEN {
            return Err(CodecError::LengthOverflow {
                what: "summary bits",
                max: MAX_SUMMARY_BITS_LEN,
                got: bits.len(),
            });
        }
        let chunk = SummaryChunk {
            salt,
            n_bits,
            k,
            chunk_idx,
            chunk_total,
            bits: bits.to_vec(),
        };
        chunk.check()?;
        Ok(chunk)
    }
}

/// Backpressure for one class (DESIGN §2.3 `Throttle = {class u8, retry_ms
/// u32}`; D4 "Throttle frames carry backpressure"; BRIEF_v2 §5.9 "Waiting ·
/// network busy"). `class` is carried as the raw byte so that a throttle for
/// a class this version does not know still parses; [`Throttle::class`]
/// resolves it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Throttle {
    pub class: u8,
    pub retry_ms: u32,
}

impl Throttle {
    /// Exact encoded size.
    pub const LEN: usize = 1 + 4;

    pub fn for_class(class: Class, retry_ms: u32) -> Self {
        Throttle {
            class: class.as_u8(),
            retry_ms,
        }
    }

    /// The throttled class, if this version knows it.
    pub fn class(&self) -> Result<Class, CodecError> {
        Class::from_u8(self.class)
    }
}

impl Encode for Throttle {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.class).u32(self.retry_ms);
        Ok(())
    }
}

impl Decode for Throttle {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(Throttle {
            class: r.u8()?,
            retry_ms: r.u32()?,
        })
    }
}

/// Length of one contact proof: `prf16(k_pair(c), "l2", handshake_hash)`.
pub const CONTACT_PROOF_LEN: usize = 16;

/// Proofs of QR-verified contact relationships over this link's Noise
/// handshake hash (DESIGN §2.3 `ContactProof = n u8 ‖ [prf16(...)]×n`; D4
/// L2 "links authenticated as QR-verified contacts by a pairwise tag in the
/// link handshake"). The responder matches each against its own contacts;
/// one match lifts the link to L2. At most 255 proofs (`list_u8`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ContactProof {
    pub proofs: Vec<[u8; CONTACT_PROOF_LEN]>,
}

impl ContactProof {
    /// Largest encoded size: count byte + 255 proofs.
    pub const MAX_LEN: usize = 1 + 255 * CONTACT_PROOF_LEN;
}

impl ListItem for ContactProof {}

impl Encode for ContactProof {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.proofs.encode_to(w)
    }
}

impl Decode for ContactProof {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        Ok(ContactProof {
            proofs: Vec::decode_from(r)?,
        })
    }
}

// ---- the frame --------------------------------------------------------------

/// The raw per-hop frame (DESIGN §2.3): header fields plus the payload
/// bytes. Encoders always write `flags = 0` and `reserved = 0`; the `flags`
/// field is kept so that a decoded frame can be inspected, but its value is
/// never transmitted (a non-zero flag would be rejected by every peer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkFrame {
    pub frame_type: FrameType,
    /// [`Class`] discriminant for a cell frame, 0 otherwise.
    pub class: u8,
    /// Reserved; always 0.
    pub flags: u8,
    pub payload: Vec<u8>,
}

impl LinkFrame {
    /// A cell frame carrying `cell` in `class`.
    pub fn cell(class: Class, cell: &Cell) -> Self {
        LinkFrame {
            frame_type: FrameType::Cell,
            class: class.as_u8(),
            flags: 0,
            payload: cell.as_bytes().to_vec(),
        }
    }

    /// A control frame of `frame_type` with an already-encoded payload.
    fn control(frame_type: FrameType, payload: Vec<u8>) -> Self {
        LinkFrame {
            frame_type,
            class: 0,
            flags: 0,
            payload,
        }
    }

    pub fn hello(hello: &Hello) -> Self {
        Self::control(FrameType::Hello, hello.encode())
    }

    /// Fails only if `chunk` violates its limits (bits over
    /// [`MAX_SUMMARY_BITS_LEN`], `chunk_idx ≥ chunk_total`).
    pub fn summary(chunk: &SummaryChunk) -> Result<Self, CodecError> {
        Ok(Self::control(FrameType::Summary, chunk.try_encode()?))
    }

    pub fn throttle(throttle: &Throttle) -> Self {
        Self::control(FrameType::Throttle, throttle.encode())
    }

    /// Fails only with more than 255 proofs.
    pub fn contact_proof(proof: &ContactProof) -> Result<Self, CodecError> {
        Ok(Self::control(FrameType::ContactProof, proof.try_encode()?))
    }

    /// Encoded size: header + payload.
    pub fn encoded_len(&self) -> usize {
        FRAME_HEADER_LEN + self.payload.len()
    }

    /// The strict per-type rules a decoder applies to `(frame_type, class,
    /// payload)`: a cell frame has a known class and exactly [`CELL_LEN`]
    /// payload bytes; a control frame has class 0 and a payload that decodes
    /// exactly (no trailing bytes) as its struct.
    pub fn validate(&self) -> Result<(), CodecError> {
        if self.flags != 0 {
            return Err(CodecError::Invalid("frame flags"));
        }
        match self.frame_type {
            FrameType::Cell => {
                Class::from_u8(self.class)?;
                if self.payload.len() != CELL_LEN {
                    return Err(CodecError::Invalid("cell frame length"));
                }
                Ok(())
            }
            control => {
                if self.class != 0 {
                    return Err(CodecError::Invalid("frame class"));
                }
                match control {
                    FrameType::Cell => Ok(()),
                    FrameType::Hello => decode::<Hello>(&self.payload).map(drop),
                    FrameType::Summary => decode::<SummaryChunk>(&self.payload).map(drop),
                    FrameType::Throttle => decode::<Throttle>(&self.payload).map(drop),
                    FrameType::ContactProof => decode::<ContactProof>(&self.payload).map(drop),
                }
            }
        }
    }
}

/// Reads the 4-byte header and the payload slice, checking the fixed rules
/// (known type, flags and reserved zero, total length within
/// [`MAX_FRAME_LEN`]) before anything is allocated. Per-type payload rules
/// are applied by the caller.
fn read_header<'a>(r: &mut Reader<'a>) -> Result<(FrameType, u8, &'a [u8]), CodecError> {
    if r.remaining() > MAX_FRAME_LEN {
        return Err(CodecError::LengthOverflow {
            what: "link frame",
            max: MAX_FRAME_LEN,
            got: r.remaining(),
        });
    }
    let frame_type = FrameType::decode_from(r)?;
    let class = r.u8()?;
    let flags = r.u8()?;
    let reserved = r.u8()?;
    if flags != 0 {
        return Err(CodecError::Invalid("frame flags"));
    }
    if reserved != 0 {
        return Err(CodecError::Invalid("frame reserved"));
    }
    Ok((frame_type, class, r.rest()))
}

impl Encode for LinkFrame {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        w.u8(self.frame_type.as_u8()).u8(self.class).u8(0).u8(0);
        w.raw(&self.payload);
        Ok(())
    }
}

impl Decode for LinkFrame {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let (frame_type, class, payload) = read_header(r)?;
        let frame = LinkFrame {
            frame_type,
            class,
            flags: 0,
            payload: payload.to_vec(),
        };
        frame.validate()?;
        Ok(frame)
    }
}

/// A frame with its payload decoded. [`Frame::decode`] is the inbound
/// pipeline's entry point (DESIGN §1 `node/inbound.rs`: stream → frame →
/// cell); the cell inside a `Frame::Cell` has passed only the length check,
/// so the pipeline applies [`Cell::validate`] next.
///
/// The `Cell` variant is as large as a cell; that is the common case on a
/// link and a frame is handled once, not stored, so it is not boxed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum Frame {
    Cell { class: Class, cell: Cell },
    Hello(Hello),
    Summary(SummaryChunk),
    Throttle(Throttle),
    ContactProof(ContactProof),
}

impl Frame {
    pub fn frame_type(&self) -> FrameType {
        match self {
            Frame::Cell { .. } => FrameType::Cell,
            Frame::Hello(_) => FrameType::Hello,
            Frame::Summary(_) => FrameType::Summary,
            Frame::Throttle(_) => FrameType::Throttle,
            Frame::ContactProof(_) => FrameType::ContactProof,
        }
    }

    /// The raw frame this encodes to. Fails only for a payload over its
    /// limits (see [`LinkFrame::summary`], [`LinkFrame::contact_proof`]).
    pub fn to_link_frame(&self) -> Result<LinkFrame, CodecError> {
        Ok(match self {
            Frame::Cell { class, cell } => LinkFrame::cell(*class, cell),
            Frame::Hello(h) => LinkFrame::hello(h),
            Frame::Summary(s) => LinkFrame::summary(s)?,
            Frame::Throttle(t) => LinkFrame::throttle(t),
            Frame::ContactProof(p) => LinkFrame::contact_proof(p)?,
        })
    }

    /// Decodes a raw frame's payload for its type, under the same strict
    /// rules as [`LinkFrame::validate`].
    pub fn from_link_frame(frame: &LinkFrame) -> Result<Self, CodecError> {
        if frame.flags != 0 {
            return Err(CodecError::Invalid("frame flags"));
        }
        Self::from_parts(frame.frame_type, frame.class, &frame.payload)
    }

    fn from_parts(frame_type: FrameType, class: u8, payload: &[u8]) -> Result<Self, CodecError> {
        if frame_type != FrameType::Cell && class != 0 {
            return Err(CodecError::Invalid("frame class"));
        }
        Ok(match frame_type {
            FrameType::Cell => {
                let class = Class::from_u8(class)?;
                let cell = Cell::from_bytes(payload)
                    .map_err(|_| CodecError::Invalid("cell frame length"))?;
                Frame::Cell { class, cell }
            }
            FrameType::Hello => Frame::Hello(decode(payload)?),
            FrameType::Summary => Frame::Summary(decode(payload)?),
            FrameType::Throttle => Frame::Throttle(decode(payload)?),
            FrameType::ContactProof => Frame::ContactProof(decode(payload)?),
        })
    }
}

impl Encode for Frame {
    fn encode_to(&self, w: &mut Writer) -> Result<(), CodecError> {
        self.to_link_frame()?.encode_to(w)
    }
}

impl Decode for Frame {
    fn decode_from(r: &mut Reader<'_>) -> Result<Self, CodecError> {
        let (frame_type, class, payload) = read_header(r)?;
        Self::from_parts(frame_type, class, payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::encode;

    #[test]
    fn class_order_is_strict_priority() {
        assert!(Class::C0Sos.outranks(Class::C1Control));
        assert!(Class::C1Control.outranks(Class::C2Text));
        assert!(Class::C2Text.outranks(Class::C3News));
        assert!(Class::C3News.outranks(Class::C4Preview));
        assert!(!Class::C4Preview.outranks(Class::C4Preview));
    }

    #[test]
    fn class_round_trips_and_rejects_unknown() {
        for c in Class::ALL {
            assert_eq!(Class::from_u8(c.as_u8()).unwrap(), c);
        }
        assert!(Class::from_u8(5).is_err());
        assert!(Class::from_u8(255).is_err());
    }

    #[test]
    fn object_caps_match_the_brief() {
        assert_eq!(Class::C0Sos.max_object_bytes(), 256);
        assert_eq!(Class::C1Control.max_object_bytes(), 65_536);
        assert_eq!(Class::C2Text.max_object_bytes(), 2_048);
        assert_eq!(Class::C3News.max_object_bytes(), 4_096);
        assert_eq!(Class::C4Preview.max_object_bytes(), 16_384);
    }

    // ---- wave 2: frames and control payloads ---------------------------

    fn cell() -> Cell {
        let mut raw = [0x5Au8; CELL_LEN];
        raw[0] = 1;
        raw[1] = 7;
        raw[2] = 0;
        raw[3] = 0;
        Cell::from_array(raw)
    }

    fn roundtrip<T: Encode + Decode + PartialEq + core::fmt::Debug>(v: &T) -> Vec<u8> {
        let bytes = encode(v);
        let back: T = decode(&bytes).expect("decode");
        assert_eq!(&back, v);
        assert_eq!(encode(&back), bytes);
        bytes
    }

    #[test]
    fn sizes_match_the_design() {
        assert_eq!(FRAME_HEADER_LEN, 4);
        assert_eq!(CELL_FRAME_LEN, 516);
        assert_eq!(CELL_NOISE_MESSAGE_LEN, 534);
        assert_eq!(Hello::LEN, 8);
        assert_eq!(Throttle::LEN, 5);
        assert_eq!(SummaryChunk::FIXED_LEN, 15);
        assert_eq!(MAX_FRAME_LEN, 4 + 15 + 4096);
        assert_eq!(MAX_NOISE_MESSAGE_LEN, MAX_FRAME_LEN + 18);
        assert_eq!(ContactProof::MAX_LEN, 4081);
    }

    #[test]
    fn frame_type_round_trips_and_rejects_unknown() {
        for t in FrameType::ALL {
            assert_eq!(FrameType::from_u8(t.as_u8()), Ok(t));
            roundtrip(&t);
        }
        assert_eq!(
            FrameType::from_u8(5),
            Err(unknown_variant("frame type", 5u8))
        );
        for c in Class::ALL {
            roundtrip(&c);
        }
    }

    #[test]
    fn cell_frame_layout() {
        let c = cell();
        let f = LinkFrame::cell(Class::C2Text, &c);
        let bytes = roundtrip(&f);
        assert_eq!(bytes.len(), CELL_FRAME_LEN);
        assert_eq!(&bytes[..4], &[0, 2, 0, 0]);
        assert_eq!(&bytes[4..], c.as_bytes());
        let typed: Frame = decode(&bytes).expect("typed decode");
        assert_eq!(
            typed,
            Frame::Cell {
                class: Class::C2Text,
                cell: c.clone()
            }
        );
        assert_eq!(encode(&typed), bytes);
        assert_eq!(Frame::from_link_frame(&f), Ok(typed.clone()));
        assert_eq!(typed.to_link_frame(), Ok(f.clone()));
        assert_eq!(typed.frame_type(), FrameType::Cell);
        assert_eq!(f.encoded_len(), CELL_FRAME_LEN);
    }

    #[test]
    fn cell_frame_rejections() {
        let good = encode(&LinkFrame::cell(Class::C1Control, &cell()));
        // unknown class
        let mut b = good.clone();
        b[1] = 5;
        assert_eq!(decode::<LinkFrame>(&b), Err(unknown_variant("class", 5u8)));
        assert!(decode::<Frame>(&b).is_err());
        // flags / reserved
        let mut b = good.clone();
        b[2] = 1;
        assert_eq!(
            decode::<LinkFrame>(&b),
            Err(CodecError::Invalid("frame flags"))
        );
        assert_eq!(decode::<Frame>(&b), Err(CodecError::Invalid("frame flags")));
        let mut b = good.clone();
        b[3] = 0x80;
        assert_eq!(
            decode::<LinkFrame>(&b),
            Err(CodecError::Invalid("frame reserved"))
        );
        // wrong payload length
        assert_eq!(
            decode::<LinkFrame>(&good[..good.len() - 1]),
            Err(CodecError::Invalid("cell frame length"))
        );
        let mut longer = good.clone();
        longer.push(0);
        assert_eq!(
            decode::<LinkFrame>(&longer),
            Err(CodecError::Invalid("cell frame length"))
        );
        assert_eq!(
            decode::<Frame>(&longer),
            Err(CodecError::Invalid("cell frame length"))
        );
        // unknown type
        let mut b = good.clone();
        b[0] = 9;
        assert_eq!(
            decode::<LinkFrame>(&b),
            Err(unknown_variant("frame type", 9u8))
        );
        // header only / empty
        assert!(matches!(
            decode::<LinkFrame>(&[]),
            Err(CodecError::UnexpectedEof { .. })
        ));
        assert!(matches!(
            decode::<LinkFrame>(&[0, 0]),
            Err(CodecError::UnexpectedEof { .. })
        ));
        // a frame with nonzero flags set by hand does not validate
        let mut f = LinkFrame::cell(Class::C1Control, &cell());
        f.flags = 1;
        assert_eq!(f.validate(), Err(CodecError::Invalid("frame flags")));
        assert_eq!(
            Frame::from_link_frame(&f),
            Err(CodecError::Invalid("frame flags"))
        );
        // but the encoder still writes 0
        assert_eq!(encode(&f)[2], 0);
    }

    #[test]
    fn hello_layout_and_strictness() {
        let h = Hello {
            proto: 1,
            caps: 0x0102,
            is_station: true,
            buffer_free: 0x0A0B_0C0D,
        };
        let bytes = roundtrip(&h);
        assert_eq!(bytes, [1, 0x02, 0x01, 1, 0x0D, 0x0C, 0x0B, 0x0A]);
        assert_eq!(bytes.len(), Hello::LEN);
        let mut bad_bool = bytes.clone();
        bad_bool[3] = 2;
        assert_eq!(decode::<Hello>(&bad_bool), Err(CodecError::Invalid("bool")));
        let f = LinkFrame::hello(&h);
        let fb = roundtrip(&f);
        assert_eq!(&fb[..4], &[1, 0, 0, 0]);
        assert_eq!(&fb[4..], &bytes[..]);
        assert_eq!(decode::<Frame>(&fb), Ok(Frame::Hello(h)));
        // class must be 0 on a control frame
        let mut c = fb.clone();
        c[1] = 1;
        assert_eq!(
            decode::<LinkFrame>(&c),
            Err(CodecError::Invalid("frame class"))
        );
        assert_eq!(decode::<Frame>(&c), Err(CodecError::Invalid("frame class")));
        // trailing byte in the payload
        let mut t = fb.clone();
        t.push(0);
        assert_eq!(decode::<LinkFrame>(&t), Err(CodecError::TrailingBytes(1)));
        assert_eq!(decode::<Frame>(&t), Err(CodecError::TrailingBytes(1)));
    }

    #[test]
    fn summary_chunk_layout_and_limits() {
        let s = SummaryChunk {
            salt: 0x0102_0304,
            n_bits: 0x1000,
            k: 7,
            chunk_idx: 1,
            chunk_total: 2,
            bits: vec![0xAA, 0xBB, 0xCC],
        };
        let bytes = roundtrip(&s);
        let mut expect = vec![
            0x04, 0x03, 0x02, 0x01, 0x00, 0x10, 0, 0, 7, 1, 0, 2, 0, 3, 0,
        ];
        expect.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
        assert_eq!(bytes, expect);
        let f = LinkFrame::summary(&s).expect("within limits");
        roundtrip(&f);
        assert_eq!(decode::<Frame>(&encode(&f)), Ok(Frame::Summary(s.clone())));
        // the largest chunk fits, one more byte does not
        let max = SummaryChunk {
            bits: vec![1; MAX_SUMMARY_BITS_LEN],
            chunk_idx: 0,
            chunk_total: 1,
            ..s.clone()
        };
        let fb = encode(&LinkFrame::summary(&max).expect("at the cap"));
        assert_eq!(fb.len(), MAX_FRAME_LEN);
        assert!(decode::<LinkFrame>(&fb).is_ok());
        let over = SummaryChunk {
            bits: vec![1; MAX_SUMMARY_BITS_LEN + 1],
            ..max.clone()
        };
        assert_eq!(
            over.try_encode(),
            Err(CodecError::LengthOverflow {
                what: "summary bits",
                max: MAX_SUMMARY_BITS_LEN,
                got: MAX_SUMMARY_BITS_LEN + 1
            })
        );
        assert!(LinkFrame::summary(&over).is_err());
        // an over-cap chunk written by hand is refused by the decoder
        let mut w = Writer::new();
        w.u32(0).u32(0).u8(1).u16(0).u16(1);
        w.bytes16(&vec![0u8; MAX_SUMMARY_BITS_LEN + 1]).unwrap();
        let raw = w.into_inner();
        assert!(matches!(
            decode::<SummaryChunk>(&raw),
            Err(CodecError::LengthOverflow {
                what: "summary bits",
                ..
            })
        ));
        let mut framed = vec![2, 0, 0, 0];
        framed.extend_from_slice(&raw);
        assert!(matches!(
            decode::<LinkFrame>(&framed),
            Err(CodecError::LengthOverflow {
                what: "link frame",
                ..
            })
        ));
        // chunk_idx must be below chunk_total
        let bad_idx = SummaryChunk {
            chunk_idx: 2,
            chunk_total: 2,
            ..s.clone()
        };
        assert_eq!(
            bad_idx.try_encode(),
            Err(CodecError::Invalid("summary chunk index"))
        );
        let mut raw = bytes.clone();
        raw[9] = 2;
        assert_eq!(
            decode::<SummaryChunk>(&raw),
            Err(CodecError::Invalid("summary chunk index"))
        );
        // a claimed length past the input is EOF, not an allocation
        let mut short = bytes.clone();
        short[13] = 0xFF;
        short[14] = 0x0F;
        assert!(matches!(
            decode::<SummaryChunk>(&short),
            Err(CodecError::UnexpectedEof { .. })
        ));
    }

    #[test]
    fn throttle_layout() {
        let t = Throttle::for_class(Class::C2Text, 0x0102_0304);
        let bytes = roundtrip(&t);
        assert_eq!(bytes, [2, 0x04, 0x03, 0x02, 0x01]);
        assert_eq!(t.class(), Ok(Class::C2Text));
        let unknown = Throttle {
            class: 9,
            retry_ms: 1,
        };
        roundtrip(&unknown);
        assert!(unknown.class().is_err());
        let f = LinkFrame::throttle(&t);
        assert_eq!(decode::<Frame>(&encode(&f)), Ok(Frame::Throttle(t)));
        let mut longer = encode(&f);
        longer.push(0);
        assert_eq!(decode::<Frame>(&longer), Err(CodecError::TrailingBytes(1)));
    }

    #[test]
    fn contact_proof_layout_and_limits() {
        let p = ContactProof {
            proofs: vec![[1u8; 16], [2u8; 16]],
        };
        let bytes = roundtrip(&p);
        assert_eq!(bytes.len(), 1 + 32);
        assert_eq!(bytes[0], 2);
        assert_eq!(&bytes[1..17], &[1u8; 16]);
        let f = LinkFrame::contact_proof(&p).expect("two proofs");
        assert_eq!(
            decode::<Frame>(&encode(&f)),
            Ok(Frame::ContactProof(p.clone()))
        );
        assert_eq!(roundtrip(&ContactProof::default()), [0]);
        let many = ContactProof {
            proofs: vec![[0u8; 16]; 255],
        };
        assert_eq!(encode(&many).len(), ContactProof::MAX_LEN);
        let too_many = ContactProof {
            proofs: vec![[0u8; 16]; 256],
        };
        assert!(LinkFrame::contact_proof(&too_many).is_err());
        // count claims more than present
        assert!(matches!(
            decode::<ContactProof>(&[3, 0, 0]),
            Err(CodecError::UnexpectedEof { .. })
        ));
        // partial proof is trailing/EOF, never a panic
        let mut cut = bytes.clone();
        cut.truncate(20);
        assert!(decode::<ContactProof>(&cut).is_err());
    }

    #[test]
    fn frame_too_long_is_refused_before_parsing() {
        let big = vec![0u8; MAX_FRAME_LEN + 1];
        assert_eq!(
            decode::<LinkFrame>(&big),
            Err(CodecError::LengthOverflow {
                what: "link frame",
                max: MAX_FRAME_LEN,
                got: MAX_FRAME_LEN + 1
            })
        );
        assert!(decode::<Frame>(&big).is_err());
    }
}
