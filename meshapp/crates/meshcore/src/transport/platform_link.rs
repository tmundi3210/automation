//! The byte-stream link adaptor the platform (uniffi surface) and the
//! simulator drive (docs/DESIGN.md §1 `transport/platform_link.rs`
//! "`feed`/`poll_outbound`"; §7 "the core never blocks: `send` enqueues; the
//! platform drains via `poll_outbound`").
//!
//! A [`PlatformLink`] is one radio connection seen from the core: the
//! platform pushes whatever bytes arrive with [`PlatformLink::feed`] and pulls
//! whatever the core queued with [`PlatformLink::poll_outbound`], in chunks
//! of its own size (ATT MTU−3). The core uses it only through the
//! [`Link`] trait: whole Noise messages in, whole Noise messages out.
//!
//! # Layering (DESIGN §7)
//!
//! ```text
//! inbound:  radio bytes ─► feed ─► StreamFramer ─► recv (one Noise message)
//!                       ─► NoiseLink::decrypt_frame ─► LinkFrame decode
//! outbound: LinkFrame encode ─► NoiseLink::encrypt_frame ─► send
//!                       (adds the u16 LE prefix) ─► poll_outbound ─► radio
//! ```
//!
//! The Noise session is *not* inside the link: the node's link layer owns
//! one [`NoiseLink`](crate::wire::noise::NoiseLink) per [`LinkId`] and calls
//! `encrypt_frame` before `send` and `decrypt_frame` after `recv`. This keeps
//! the adaptor a dumb byte pipe that the simulator can replace and lets the
//! core tear the session down on any fatal Noise error without the platform
//! knowing about keys.
//!
//! # Back-pressure and bounds (BRIEF_v2 §5.5 "rate limits per inbound link")
//!
//! - `send` answers [`LinkError::Busy`] when the outbound queue already
//!   holds frames and adding this one would exceed `max_outbound_bytes`
//!   (default [`DEFAULT_MAX_OUTBOUND_BYTES`]); an empty queue always takes one
//!   frame, so a frame larger than the cap still goes out alone. The queue
//!   therefore never exceeds `max(cap, one frame)`.
//! - `feed` answers [`LinkError::Busy`] without consuming the chunk when
//!   complete frames worth `max_inbound_bytes` (default
//!   [`DEFAULT_MAX_INBOUND_BYTES`]) or `max_inbound_frames` (default
//!   [`DEFAULT_MAX_INBOUND_FRAMES`]) wait for `recv`; the platform must hold
//!   the chunk and retry after the core drained the link (dropping it
//!   corrupts the stream, in which case the platform closes the link
//!   instead). Both caps are checked before the chunk is parsed, so one
//!   chunk can overshoot them by the frames it completes.
//! - A corrupt stream (a frame shorter than a Noise tag, or framer
//!   overflow) closes the link: `feed` returns [`LinkError::Closed`] and the
//!   core sees [`PlatformLink::is_closed`].
//! - After [`PlatformLink::close`] every queue is dropped and `send` /
//!   `feed` answer `Closed`.
//!
//! [`inbound_backlog`](PlatformLink::inbound_backlog) and
//! [`outbound_backlog`](PlatformLink::outbound_backlog) let the scheduler
//! (DESIGN §1 `relay/sched.rs`) see how much is waiting on each side.

use core::fmt;
use std::collections::VecDeque;

use crate::transport::chunk::{MAX_BUFFERED_BYTES, MAX_FRAME_LEN, StreamFramer, frame};
use crate::transport::link::{Link, LinkAuth, LinkError, LinkId, LinkInfo};

/// Default cap on queued outbound bytes (64 kB: about two seconds of a
/// 30 kB/s BLE link, and the whole of a C1 control object).
pub const DEFAULT_MAX_OUTBOUND_BYTES: usize = 64 * 1024;
/// Default cap on complete inbound frames waiting for `recv`, in bytes
/// (the same four maximal frames the framer itself holds).
pub const DEFAULT_MAX_INBOUND_BYTES: usize = MAX_BUFFERED_BYTES;
/// Default cap on complete inbound frames waiting for `recv`, in frames
/// (a burst of a few hundred cells; bounds the per-frame queue overhead
/// independently of the byte cap).
pub const DEFAULT_MAX_INBOUND_FRAMES: usize = 512;

/// In-memory link adaptor (module doc).
pub struct PlatformLink {
    id: LinkId,
    info: LinkInfo,
    auth: LinkAuth,
    framer: StreamFramer,
    /// Complete inbound frames, oldest first.
    inbound: VecDeque<Vec<u8>>,
    inbound_bytes: usize,
    /// Length-prefixed outbound frames as one byte stream.
    outbound: VecDeque<u8>,
    max_outbound_bytes: usize,
    max_inbound_bytes: usize,
    max_inbound_frames: usize,
    closed: bool,
}

impl fmt::Debug for PlatformLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlatformLink")
            .field("id", &self.id)
            .field("info", &self.info)
            .field("auth", &self.auth)
            .field("inbound_frames", &self.inbound.len())
            .field("inbound_bytes", &self.inbound_bytes)
            .field("outbound_bytes", &self.outbound.len())
            .field("framer", &self.framer)
            .field("closed", &self.closed)
            .finish()
    }
}

impl PlatformLink {
    /// A fresh, open, anonymous link with the default caps.
    pub fn new(id: LinkId, info: LinkInfo) -> Self {
        PlatformLink {
            id,
            info,
            auth: LinkAuth::Anonymous,
            framer: StreamFramer::new(),
            inbound: VecDeque::new(),
            inbound_bytes: 0,
            outbound: VecDeque::new(),
            max_outbound_bytes: DEFAULT_MAX_OUTBOUND_BYTES,
            max_inbound_bytes: DEFAULT_MAX_INBOUND_BYTES,
            max_inbound_frames: DEFAULT_MAX_INBOUND_FRAMES,
            closed: false,
        }
    }

    /// Changes the outbound cap (module doc "Back-pressure").
    pub fn set_max_outbound_bytes(&mut self, bytes: usize) {
        self.max_outbound_bytes = bytes;
    }

    /// Changes the inbound byte cap (module doc "Back-pressure").
    pub fn set_max_inbound_bytes(&mut self, bytes: usize) {
        self.max_inbound_bytes = bytes;
    }

    /// Changes the inbound frame-count cap (module doc "Back-pressure").
    pub fn set_max_inbound_frames(&mut self, frames: usize) {
        self.max_inbound_frames = frames;
    }

    pub fn max_outbound_bytes(&self) -> usize {
        self.max_outbound_bytes
    }

    pub fn max_inbound_bytes(&self) -> usize {
        self.max_inbound_bytes
    }

    pub fn max_inbound_frames(&self) -> usize {
        self.max_inbound_frames
    }

    /// Records what the handshake and the optional ContactProof established
    /// (DESIGN §7 "set after Noise + optional ContactProof").
    pub fn set_auth(&mut self, auth: LinkAuth) {
        self.auth = auth;
    }

    /// Inbound bytes from the radio, at any chunk boundary. `Closed` on a
    /// closed link or when the chunk corrupts the stream (the link is then
    /// closed); `Busy` without consuming anything when an inbound cap is
    /// reached (module doc "Back-pressure").
    pub fn feed(&mut self, raw: &[u8]) -> Result<(), LinkError> {
        if self.closed {
            return Err(LinkError::Closed);
        }
        if self.inbound_bytes >= self.max_inbound_bytes
            || self.inbound.len() >= self.max_inbound_frames
        {
            return Err(LinkError::Busy);
        }
        if self.framer.push(raw).is_err() {
            self.close();
            return Err(LinkError::Closed);
        }
        while let Some(frame) = self.framer.next_frame() {
            self.inbound_bytes = self.inbound_bytes.saturating_add(frame.len());
            self.inbound.push_back(frame);
        }
        if self.framer.is_poisoned() {
            self.close();
            return Err(LinkError::Closed);
        }
        Ok(())
    }

    /// Drains up to `max_bytes` of the outbound byte stream for the radio
    /// (the platform calls this with MTU−3). Empty when nothing is queued or
    /// the link is closed.
    pub fn poll_outbound(&mut self, max_bytes: usize) -> Vec<u8> {
        let n = max_bytes.min(self.outbound.len());
        self.outbound.drain(..n).collect()
    }

    /// Closes the link: drops every queue; `send` and `feed` answer `Closed`
    /// from now on.
    pub fn close(&mut self) {
        self.closed = true;
        self.inbound.clear();
        self.inbound_bytes = 0;
        self.outbound.clear();
        self.framer.reset();
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Complete inbound frames waiting for [`Link::recv`].
    pub fn inbound_backlog(&self) -> usize {
        self.inbound.len()
    }

    /// Bytes of complete inbound frames waiting for [`Link::recv`].
    pub fn inbound_backlog_bytes(&self) -> usize {
        self.inbound_bytes
    }

    /// Bytes (prefixed frames) waiting for [`poll_outbound`](Self::poll_outbound).
    pub fn outbound_backlog(&self) -> usize {
        self.outbound.len()
    }
}

impl Link for PlatformLink {
    fn id(&self) -> LinkId {
        self.id
    }

    fn info(&self) -> &LinkInfo {
        &self.info
    }

    fn auth(&self) -> LinkAuth {
        self.auth
    }

    /// Queues one whole Noise message with its length prefix. `Closed`
    /// after [`close`](PlatformLink::close), `TooLarge` above
    /// [`MAX_FRAME_LEN`], `Busy` per the module doc; a failed send queues
    /// nothing.
    fn send(&mut self, frame_bytes: &[u8]) -> Result<(), LinkError> {
        if self.closed {
            return Err(LinkError::Closed);
        }
        if frame_bytes.len() > MAX_FRAME_LEN {
            return Err(LinkError::TooLarge(frame_bytes.len()));
        }
        let framed = frame(frame_bytes).map_err(|_| LinkError::TooLarge(frame_bytes.len()))?;
        if !self.outbound.is_empty()
            && self.outbound.len().saturating_add(framed.len()) > self.max_outbound_bytes
        {
            return Err(LinkError::Busy);
        }
        self.outbound.extend(framed);
        Ok(())
    }

    fn recv(&mut self) -> Option<Vec<u8>> {
        let frame = self.inbound.pop_front()?;
        self.inbound_bytes = self.inbound_bytes.saturating_sub(frame.len());
        Some(frame)
    }
}

// ---- tests ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relay::Lane;
    use crate::transport::chunk::{MIN_FRAME_LEN, framed_len};
    use crate::transport::link::{PeerKind, Role, Validation};

    fn info(role: Role, mtu: u16) -> LinkInfo {
        LinkInfo {
            role,
            peer_kind_hint: PeerKind::AppUuid,
            mtu_hint: mtu,
        }
    }

    fn link() -> PlatformLink {
        PlatformLink::new(LinkId(7), info(Role::Central, 517))
    }

    #[test]
    fn send_prefixes_and_poll_drains_in_arbitrary_chunks() {
        let mut l = link();
        assert_eq!(l.id(), LinkId(7));
        assert_eq!(l.info().role, Role::Central);
        assert_eq!(l.auth(), LinkAuth::Anonymous);
        l.send(b"abc").unwrap();
        l.send(&[9u8; 516]).unwrap();
        assert_eq!(l.outbound_backlog(), framed_len(3) + framed_len(516));
        let first = l.poll_outbound(4);
        assert_eq!(first, vec![3, 0, b'a', b'b']);
        let rest = l.poll_outbound(usize::MAX);
        assert_eq!(rest.len(), 1 + framed_len(516));
        assert_eq!(&rest[..3], &[b'c', 0x04, 0x02]);
        assert!(l.poll_outbound(100).is_empty());
        assert_eq!(l.outbound_backlog(), 0);
    }

    #[test]
    fn feed_reassembles_and_recv_yields_in_order() {
        let mut l = link();
        let stream: Vec<u8> = frame(&[1u8; 16])
            .unwrap()
            .into_iter()
            .chain(frame(&[0u8; 400]).unwrap())
            .collect();
        for chunk in stream.chunks(17) {
            l.feed(chunk).unwrap();
        }
        assert_eq!(l.inbound_backlog(), 2);
        assert_eq!(l.inbound_backlog_bytes(), 416);
        assert_eq!(l.recv().unwrap(), vec![1u8; 16]);
        assert_eq!(l.recv().unwrap(), vec![0u8; 400]);
        assert_eq!(l.inbound_backlog(), 0);
        assert_eq!(l.inbound_backlog_bytes(), 0);
        assert!(l.recv().is_none());
        // a zero-length feed is harmless
        l.feed(&[]).unwrap();
    }

    #[test]
    fn busy_when_the_outbound_cap_is_reached() {
        let mut l = link();
        l.set_max_outbound_bytes(100);
        assert_eq!(l.max_outbound_bytes(), 100);
        l.send(&[1u8; 60]).unwrap(); // 62 queued
        assert_eq!(l.send(&[2u8; 60]), Err(LinkError::Busy));
        l.send(&[3u8; 36]).unwrap(); // 62 + 38 = 100: exactly the cap fits
        assert_eq!(l.outbound_backlog(), 100);
        assert_eq!(l.send(&[4u8; 1]), Err(LinkError::Busy));
        let _ = l.poll_outbound(usize::MAX);
        // an empty queue takes a frame larger than the cap, alone
        l.send(&[5u8; 200]).unwrap();
        assert_eq!(l.outbound_backlog(), 202);
        assert_eq!(l.send(&[6u8; 1]), Err(LinkError::Busy));
        let _ = l.poll_outbound(usize::MAX);
        assert_eq!(
            l.send(&[0u8; MAX_FRAME_LEN + 1]),
            Err(LinkError::TooLarge(MAX_FRAME_LEN + 1))
        );
        l.send(&[0u8; MAX_FRAME_LEN]).unwrap();
    }

    #[test]
    fn busy_when_an_inbound_cap_is_reached_and_nothing_is_consumed() {
        let mut l = link();
        l.set_max_inbound_bytes(40);
        assert_eq!(l.max_inbound_bytes(), 40);
        l.feed(&frame(&[1u8; 20]).unwrap()).unwrap(); // 20 < 40: still open
        l.feed(&frame(&[2u8; 20]).unwrap()).unwrap(); // 40 >= 40 now
        assert_eq!(l.feed(&[0, 0]), Err(LinkError::Busy));
        assert!(!l.is_closed(), "the refused chunk was not parsed");
        assert_eq!(l.inbound_backlog(), 2);
        assert_eq!(l.recv().unwrap(), vec![1u8; 20]);
        assert_eq!(l.recv().unwrap(), vec![2u8; 20]);
        l.feed(&frame(&[3u8; 16]).unwrap()).unwrap();
        assert_eq!(l.recv().unwrap(), vec![3u8; 16]);

        // the frame-count cap, independent of bytes
        let mut l = link();
        l.set_max_inbound_frames(3);
        assert_eq!(l.max_inbound_frames(), 3);
        assert_eq!(l.max_inbound_frames(), 3);
        let one = frame(&[4u8; MIN_FRAME_LEN]).unwrap();
        let three: Vec<u8> = one.iter().cycle().take(3 * one.len()).copied().collect();
        l.feed(&three).unwrap();
        assert_eq!(l.inbound_backlog(), 3);
        assert_eq!(l.feed(&one), Err(LinkError::Busy));
        assert!(l.recv().is_some());
        l.feed(&one).unwrap();
        assert_eq!(l.inbound_backlog(), 3);
        assert_eq!(DEFAULT_MAX_INBOUND_FRAMES, 512);
    }

    #[test]
    fn corrupt_stream_closes_the_link() {
        // a frame shorter than a Noise tag
        for short in [[0u8, 0], [1, 0], [15, 0]] {
            let mut l = link();
            l.feed(&frame(&[1u8; 16]).unwrap()).unwrap();
            assert_eq!(l.feed(&short), Err(LinkError::Closed));
            assert!(l.is_closed());
            assert!(l.recv().is_none(), "close drops what was queued");
            assert_eq!(l.feed(b"x"), Err(LinkError::Closed));
            assert_eq!(l.send(b"x"), Err(LinkError::Closed));
        }
        // the short prefix split across chunks
        let mut l = link();
        l.feed(&[0u8]).unwrap();
        assert_eq!(l.feed(&[0u8]), Err(LinkError::Closed));
        // framer overflow: a giant chunk that never completes its frame
        let mut l = link();
        l.feed(&[0xFF, 0xFF]).unwrap();
        assert_eq!(
            l.feed(&vec![0u8; MAX_BUFFERED_BYTES - 1]),
            Err(LinkError::Closed)
        );
        assert!(l.is_closed());
        // exactly the cap, completing the frame, is fine
        let mut l = link();
        l.feed(&[0xFF, 0xFF]).unwrap();
        l.feed(&vec![0u8; MAX_FRAME_LEN]).unwrap();
        assert_eq!(l.recv().unwrap().len(), MAX_FRAME_LEN);
        assert!(!l.is_closed());
    }

    /// The framer poisons silently while a frame is being drained; `feed`
    /// must notice before it reports success (module doc "A corrupt stream
    /// closes the link"), or the link stays open with a dead framer.
    #[test]
    fn a_short_prefix_behind_a_complete_frame_closes_the_link_in_the_same_feed() {
        let mut chunk = frame(&[1u8; 16]).unwrap();
        chunk.extend_from_slice(&[0, 0]);
        let mut l = link();
        assert_eq!(l.feed(&chunk), Err(LinkError::Closed));
        assert!(l.is_closed());
        assert!(l.recv().is_none(), "nothing from a torn-down link");
        assert_eq!(l.inbound_backlog_bytes(), 0);
        assert_eq!(l.feed(&frame(&[2u8; 16]).unwrap()), Err(LinkError::Closed));
    }

    #[test]
    fn close_drops_everything_and_refuses_further_io() {
        let mut l = link();
        l.send(b"out").unwrap();
        l.feed(&frame(&[1u8; 16]).unwrap()).unwrap();
        l.close();
        assert!(l.is_closed());
        assert_eq!(l.send(b"x"), Err(LinkError::Closed));
        assert_eq!(l.feed(b"x"), Err(LinkError::Closed));
        assert!(l.recv().is_none());
        assert!(l.poll_outbound(100).is_empty());
        assert_eq!(l.inbound_backlog(), 0);
        assert_eq!(l.inbound_backlog_bytes(), 0);
        assert_eq!(l.outbound_backlog(), 0);
    }

    #[test]
    fn auth_drives_the_lane() {
        let mut l = link();
        assert_eq!(l.lane_for(Validation::Opaque), Lane::L3);
        assert_eq!(l.lane_for(Validation::Validated), Lane::L1);
        let key = crate::crypto::ed25519::IdentitySigner::from_seed(&[3u8; 32]).public();
        l.set_auth(LinkAuth::VerifiedContact(key));
        assert_eq!(l.auth(), LinkAuth::VerifiedContact(key));
        assert_eq!(l.lane_for(Validation::Opaque), Lane::L2);
        assert_eq!(l.lane_for(Validation::Validated), Lane::L1);
        let dbg = format!("{l:?}");
        assert!(dbg.contains("PlatformLink") && dbg.contains("closed: false"));
    }
}
