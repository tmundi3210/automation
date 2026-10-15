//! Public-API tests for the Noise NN link layer over the platform byte-stream
//! adaptor (docs/DESIGN.md §1 `wire/noise.rs`, `transport/chunk.rs`,
//! `transport/platform_link.rs`; §2.2 "byte stream of length-prefixed Noise
//! frames chunked at MTU−3"; §2.3 ContactProof; §7; §10 "Noise link round
//! trip and `ContactProof` matching" and the stable-proptest version of the
//! `noise_stream` fuzz target).

use meshcore::crypto::hash::hkdf32;
use meshcore::transport::Link;
use meshcore::transport::chunk::{
    FrameError, MAX_BUFFERED_BYTES, MAX_FRAME_LEN, MIN_FRAME_LEN, StreamFramer, frame, framed_len,
};
use meshcore::transport::link::{LinkAuth, LinkError, LinkId, LinkInfo, PeerKind, Role};
use meshcore::transport::platform_link::{DEFAULT_MAX_OUTBOUND_BYTES, PlatformLink};
use meshcore::wire::frame::{CELL_FRAME_LEN, CELL_NOISE_MESSAGE_LEN};
use meshcore::wire::noise::{
    Fatal, HANDSHAKE_MSG1_LEN, HANDSHAKE_MSG2_LEN, MAX_MESSAGE_LEN, MAX_PLAINTEXT_LEN, NoiseError,
    NoiseLink, TAG_LEN, contact_proof, verify_contact_proof,
};
use proptest::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

// ---- helpers ----------------------------------------------------------------

fn info(role: Role, mtu: u16) -> LinkInfo {
    LinkInfo {
        role,
        peer_kind_hint: PeerKind::AppUuid,
        mtu_hint: mtu,
    }
}

/// Moves every queued byte between the two links in chunks of MTU−3, the
/// way the platform does (DESIGN §7 "chunk at MTU−3"). Returns the number
/// of chunks moved.
fn pump(a: &mut PlatformLink, b: &mut PlatformLink, mtu: usize) -> usize {
    let chunk = mtu - 3;
    let mut chunks = 0;
    loop {
        let ab = a.poll_outbound(chunk);
        let ba = b.poll_outbound(chunk);
        if ab.is_empty() && ba.is_empty() {
            return chunks;
        }
        if !ab.is_empty() {
            assert!(ab.len() <= chunk);
            b.feed(&ab).expect("feed a->b");
            chunks += 1;
        }
        if !ba.is_empty() {
            assert!(ba.len() <= chunk);
            a.feed(&ba).expect("feed b->a");
            chunks += 1;
        }
    }
}

struct Pair {
    /// Initiator (central) side.
    ni: NoiseLink,
    li: PlatformLink,
    /// Responder (peripheral) side.
    nr: NoiseLink,
    lr: PlatformLink,
    mtu: usize,
}

/// Runs the full handshake through the two platform links at `mtu`.
fn handshake(mtu: usize, seed_i: u64, seed_r: u64) -> Pair {
    let mut rng_i = ChaCha20Rng::seed_from_u64(seed_i);
    let mut rng_r = ChaCha20Rng::seed_from_u64(seed_r);
    let mut ni = NoiseLink::initiator(&mut rng_i).expect("initiator");
    let mut nr = NoiseLink::responder(&mut rng_r).expect("responder");
    let mtu16 = u16::try_from(mtu).expect("mtu fits");
    let mut li = PlatformLink::new(LinkId(1), info(Role::Central, mtu16));
    let mut lr = PlatformLink::new(LinkId(2), info(Role::Peripheral, mtu16));

    let m1 = ni.write_handshake().expect("msg1");
    assert_eq!(m1.len(), HANDSHAKE_MSG1_LEN);
    li.send(&m1).expect("send msg1");
    assert_eq!(li.outbound_backlog(), framed_len(HANDSHAKE_MSG1_LEN));
    pump(&mut li, &mut lr, mtu);
    assert_eq!(lr.inbound_backlog(), 1);
    let got = lr.recv().expect("msg1 arrives");
    assert_eq!(got, m1);
    nr.read_handshake(&got).expect("responder reads msg1");

    let m2 = nr.write_handshake().expect("msg2");
    assert_eq!(m2.len(), HANDSHAKE_MSG2_LEN);
    assert!(nr.is_handshake_finished());
    lr.send(&m2).expect("send msg2");
    pump(&mut li, &mut lr, mtu);
    let got = li.recv().expect("msg2 arrives");
    assert_eq!(got, m2);
    ni.read_handshake(&got).expect("initiator reads msg2");
    assert!(ni.is_handshake_finished());

    let ni = ni.into_transport().expect("initiator transport");
    let nr = nr.into_transport().expect("responder transport");
    assert!(ni.is_transport() && nr.is_transport());
    assert!(li.recv().is_none() && lr.recv().is_none());
    Pair {
        ni,
        li,
        nr,
        lr,
        mtu,
    }
}

/// Sends `plain` from `from` to `to` through the platform links and checks
/// that it arrives intact; returns the stream length it occupied.
fn send_through(
    from: &mut NoiseLink,
    from_link: &mut PlatformLink,
    to: &mut NoiseLink,
    to_link: &mut PlatformLink,
    mtu: usize,
    plain: &[u8],
) -> usize {
    let ct = from.encrypt_frame(plain).expect("encrypt");
    assert_eq!(ct.len(), plain.len() + TAG_LEN);
    from_link.send(&ct).expect("send");
    let stream_len = from_link.outbound_backlog();
    assert_eq!(stream_len, framed_len(ct.len()));
    let chunks = pump(from_link, to_link, mtu);
    assert_eq!(chunks, stream_len.div_ceil(mtu - 3));
    let got = to_link.recv().expect("frame arrives");
    assert_eq!(got, ct);
    assert!(to_link.recv().is_none());
    assert_eq!(to.decrypt_frame(&got).expect("decrypt"), plain);
    stream_len
}

fn k_pair(label: &str) -> [u8; 32] {
    hkdf32(b"noise_link test pairwise secret", b"", label.as_bytes())
}

// ---- handshake and frames at each MTU -------------------------------------------

#[test]
fn full_handshake_and_frames_at_every_mtu() {
    for mtu in [20usize, 185, 517] {
        let mut p = handshake(mtu, 11, 22);
        assert_eq!(p.ni.handshake_hash(), p.nr.handshake_hash(), "mtu {mtu}");
        assert_ne!(p.ni.handshake_hash(), [0u8; 32]);

        // sizes 1, a cell frame (534 B on the stream) and the largest frame
        let one = send_through(&mut p.ni, &mut p.li, &mut p.nr, &mut p.lr, p.mtu, &[0x42]);
        assert_eq!(one, framed_len(1 + TAG_LEN));

        let cell_frame = vec![0xC5u8; CELL_FRAME_LEN];
        let cell = send_through(
            &mut p.ni,
            &mut p.li,
            &mut p.nr,
            &mut p.lr,
            p.mtu,
            &cell_frame,
        );
        assert_eq!(
            cell, CELL_NOISE_MESSAGE_LEN,
            "DESIGN §2.2: 2 + 4 + 512 + 16"
        );
        assert_eq!(cell, 534);

        let max: Vec<u8> = (0..MAX_PLAINTEXT_LEN).map(|i| (i % 253) as u8).collect();
        let largest = send_through(&mut p.ni, &mut p.li, &mut p.nr, &mut p.lr, p.mtu, &max);
        assert_eq!(largest, framed_len(MAX_MESSAGE_LEN));
        assert_eq!(largest, framed_len(MAX_FRAME_LEN));

        // and the other direction, interleaved
        for n in 0..3u8 {
            send_through(&mut p.nr, &mut p.lr, &mut p.ni, &mut p.li, p.mtu, &[n; 300]);
            send_through(&mut p.ni, &mut p.li, &mut p.nr, &mut p.lr, p.mtu, &[n; 7]);
        }
        assert_eq!(p.ni.sending_nonce(), Some(6));
        assert_eq!(p.nr.sending_nonce(), Some(3));
    }
}

#[test]
fn several_frames_in_flight_arrive_in_order_through_one_pump() {
    let mut p = handshake(185, 1, 2);
    let plains: Vec<Vec<u8>> = (0..20u8)
        .map(|n| vec![n; 50 + usize::from(n) * 37])
        .collect();
    for plain in &plains {
        let ct = p.ni.encrypt_frame(plain).unwrap();
        p.li.send(&ct).unwrap();
    }
    pump(&mut p.li, &mut p.lr, 185);
    assert_eq!(p.lr.inbound_backlog(), plains.len());
    for plain in &plains {
        let got = p.lr.recv().unwrap();
        assert_eq!(&p.nr.decrypt_frame(&got).unwrap(), plain);
    }
    assert_eq!(p.lr.inbound_backlog(), 0);
    assert_eq!(p.lr.inbound_backlog_bytes(), 0);
}

#[test]
fn handshake_bytes_are_deterministic_in_the_seeds() {
    let a = handshake(517, 5, 6);
    let b = handshake(517, 5, 6);
    let c = handshake(517, 5, 7);
    assert_eq!(a.ni.handshake_hash(), b.ni.handshake_hash());
    assert_ne!(a.ni.handshake_hash(), c.ni.handshake_hash());
    let mut i1 = NoiseLink::initiator(&mut ChaCha20Rng::seed_from_u64(9)).unwrap();
    let mut i2 = NoiseLink::initiator(&mut ChaCha20Rng::seed_from_u64(9)).unwrap();
    assert_eq!(i1.write_handshake().unwrap(), i2.write_handshake().unwrap());
}

// ---- failure -----------------------------------------------------------------------

#[test]
fn a_flipped_bit_fails_decrypt_with_fatal_and_kills_the_link() {
    let mut p = handshake(517, 1, 2);
    let ct = p.ni.encrypt_frame(b"the quick brown fox").unwrap();
    for bit in [0usize, 7, 8 * 10 + 3, 8 * (ct.len() - 1) + 7] {
        let mut flipped = ct.clone();
        flipped[bit / 8] ^= 1 << (bit % 8);
        let mut fresh = handshake(517, 1, 2);
        let _ = fresh.ni.encrypt_frame(b"the quick brown fox").unwrap();
        let err = fresh.nr.decrypt_frame(&flipped).unwrap_err();
        assert_eq!(err, NoiseError::Fatal(Fatal::Decrypt), "bit {bit}");
        assert!(err.is_fatal());
        assert!(fresh.nr.is_dead());
        assert_eq!(
            fresh.nr.decrypt_frame(&ct),
            Err(NoiseError::Fatal(Fatal::Dead))
        );
        assert_eq!(
            fresh.nr.encrypt_frame(b"x"),
            Err(NoiseError::Fatal(Fatal::Dead))
        );
    }
    // the good copy still opens on an untouched session
    assert_eq!(p.nr.decrypt_frame(&ct).unwrap(), b"the quick brown fox");
    // a tampered handshake is fatal too
    let mut i = NoiseLink::initiator(&mut ChaCha20Rng::seed_from_u64(1)).unwrap();
    let mut r = NoiseLink::responder(&mut ChaCha20Rng::seed_from_u64(2)).unwrap();
    let m1 = i.write_handshake().unwrap();
    r.read_handshake(&m1).unwrap();
    let mut m2 = r.write_handshake().unwrap();
    m2[40] ^= 0x80;
    assert_eq!(
        i.read_handshake(&m2),
        Err(NoiseError::Fatal(Fatal::Handshake))
    );
    assert!(i.is_dead());
}

#[test]
fn sessions_do_not_cross() {
    let mut a = handshake(517, 1, 2);
    let mut b = handshake(517, 3, 4);
    let ct = a.ni.encrypt_frame(b"for a").unwrap();
    assert_eq!(
        b.nr.decrypt_frame(&ct),
        Err(NoiseError::Fatal(Fatal::Decrypt))
    );
    assert_eq!(a.nr.decrypt_frame(&ct).unwrap(), b"for a");
}

// ---- contact proof -------------------------------------------------------------------

#[test]
fn contact_proof_matches_on_both_sides_and_only_for_this_session() {
    let a = handshake(185, 1, 2);
    let b = handshake(185, 1, 3);
    let k = k_pair("alice-bob");
    let from_i = contact_proof(&k, &a.ni.handshake_hash());
    let from_r = contact_proof(&k, &a.nr.handshake_hash());
    assert_eq!(
        from_i, from_r,
        "DESIGN §2.3: the responder matches the proof"
    );
    assert!(verify_contact_proof(&k, &a.nr.handshake_hash(), &from_i));
    assert!(verify_contact_proof(&k, &a.ni.handshake_hash(), &from_r));
    // another session, another proof: a replay proves nothing
    assert_ne!(from_i, contact_proof(&k, &b.ni.handshake_hash()));
    assert!(!verify_contact_proof(&k, &b.nr.handshake_hash(), &from_i));
    // another pair secret, another proof
    assert_ne!(
        from_i,
        contact_proof(&k_pair("alice-carol"), &a.ni.handshake_hash())
    );
    assert!(!verify_contact_proof(
        &k_pair("alice-carol"),
        &a.nr.handshake_hash(),
        &from_i
    ));
    // the proof lifts the link to L2 (DESIGN §7 LinkAuth)
    let mut lr = a.lr;
    assert_eq!(lr.auth(), LinkAuth::Anonymous);
    let key = meshcore::crypto::ed25519::IdentitySigner::from_seed(&[1u8; 32]).public();
    lr.set_auth(LinkAuth::VerifiedContact(key));
    assert_eq!(lr.auth(), LinkAuth::VerifiedContact(key));
}

// ---- Busy / Closed ---------------------------------------------------------------------

#[test]
fn busy_and_closed_behaviour() {
    let mut p = handshake(517, 1, 2);
    assert_eq!(p.li.max_outbound_bytes(), DEFAULT_MAX_OUTBOUND_BYTES);
    assert_eq!(DEFAULT_MAX_OUTBOUND_BYTES, 64 * 1024);

    // outbound: fill the default cap with cell-sized messages
    let ct = p.ni.encrypt_frame(&[0u8; CELL_FRAME_LEN]).unwrap();
    let mut sent = 0;
    loop {
        match p.li.send(&ct) {
            Ok(()) => sent += 1,
            Err(LinkError::Busy) => break,
            Err(e) => panic!("unexpected {e}"),
        }
        assert!(sent < 1000, "never Busy");
    }
    assert!(p.li.outbound_backlog() <= DEFAULT_MAX_OUTBOUND_BYTES);
    assert_eq!(p.li.outbound_backlog(), sent * CELL_NOISE_MESSAGE_LEN);
    assert_eq!(sent, DEFAULT_MAX_OUTBOUND_BYTES / CELL_NOISE_MESSAGE_LEN);
    // draining one byte short of a whole frame's room is not enough...
    let room = DEFAULT_MAX_OUTBOUND_BYTES - p.li.outbound_backlog();
    assert!(room < CELL_NOISE_MESSAGE_LEN);
    let drained = p.li.poll_outbound(CELL_NOISE_MESSAGE_LEN - room - 1);
    assert_eq!(drained.len(), CELL_NOISE_MESSAGE_LEN - room - 1);
    p.lr.feed(&drained).unwrap();
    assert_eq!(p.li.send(&ct), Err(LinkError::Busy));
    // ...one more byte is exactly enough
    let one = p.li.poll_outbound(1);
    assert_eq!(one.len(), 1);
    p.lr.feed(&one).unwrap();
    p.li.send(&ct).unwrap();
    assert_eq!(p.li.outbound_backlog(), DEFAULT_MAX_OUTBOUND_BYTES);
    assert_eq!(p.li.send(&ct), Err(LinkError::Busy));
    // the peer still decodes everything in order after the back-pressure
    pump(&mut p.li, &mut p.lr, 517);
    assert_eq!(p.lr.inbound_backlog(), sent + 1);
    assert_eq!(
        p.nr.decrypt_frame(&p.lr.recv().unwrap()).unwrap(),
        vec![0u8; CELL_FRAME_LEN]
    );

    // oversize
    assert_eq!(
        p.li.send(&[0u8; MAX_FRAME_LEN + 1]),
        Err(LinkError::TooLarge(MAX_FRAME_LEN + 1))
    );

    // inbound back-pressure: feed is refused, nothing consumed, until recv
    let mut lr = PlatformLink::new(LinkId(9), info(Role::Peripheral, 517));
    // the cap counts frame bytes (no prefix): three cell messages fill it
    lr.set_max_inbound_bytes(3 * ct.len());
    let framed = frame(&ct).unwrap();
    for _ in 0..3 {
        lr.feed(&framed).unwrap();
    }
    assert_eq!(lr.feed(&framed), Err(LinkError::Busy));
    assert_eq!(lr.inbound_backlog(), 3);
    assert!(lr.recv().is_some());
    lr.feed(&framed).unwrap();
    assert_eq!(lr.inbound_backlog(), 3);

    // closed
    p.li.close();
    assert!(p.li.is_closed());
    assert_eq!(p.li.send(&ct), Err(LinkError::Closed));
    assert_eq!(p.li.feed(&framed), Err(LinkError::Closed));
    assert!(p.li.poll_outbound(1000).is_empty());
    assert!(p.li.recv().is_none());
    assert_eq!(p.li.outbound_backlog(), 0);
    assert_eq!(p.li.inbound_backlog(), 0);

    // a corrupt stream closes the link from the inside: a frame shorter
    // than a Noise tag (no Noise message is), or a framer overflow
    let mut lr = PlatformLink::new(LinkId(10), info(Role::Peripheral, 517));
    lr.feed(&framed).unwrap();
    assert_eq!(lr.feed(&[15, 0, 1, 2, 3]), Err(LinkError::Closed));
    assert!(lr.is_closed());
    assert!(lr.recv().is_none());
    let mut lr = PlatformLink::new(LinkId(11), info(Role::Peripheral, 517));
    lr.feed(&[0xFF, 0xFF]).unwrap();
    assert_eq!(
        lr.feed(&vec![0u8; MAX_BUFFERED_BYTES - 1]),
        Err(LinkError::Closed)
    );
    assert!(lr.is_closed());
    // frame-count cap: tiny frames cannot inflate the queue past it
    let mut lr = PlatformLink::new(LinkId(12), info(Role::Peripheral, 517));
    lr.set_max_inbound_frames(8);
    let tiny = frame(&[0u8; MIN_FRAME_LEN]).unwrap();
    let burst: Vec<u8> = tiny.iter().cycle().take(8 * tiny.len()).copied().collect();
    lr.feed(&burst).unwrap();
    assert_eq!(lr.inbound_backlog(), 8);
    assert_eq!(lr.feed(&tiny), Err(LinkError::Busy));
    assert!(!lr.is_closed());
}

// ---- the framer ---------------------------------------------------------------------------

#[test]
fn framer_limits() {
    assert_eq!(MAX_FRAME_LEN, 65535);
    assert_eq!(MAX_BUFFERED_BYTES, 4 * (65535 + 2));
    assert_eq!(
        frame(&[0u8; MAX_FRAME_LEN + 1]),
        Err(FrameError::TooLarge(MAX_FRAME_LEN + 1))
    );
    let max = frame(&[7u8; MAX_FRAME_LEN]).unwrap();
    let mut fr = StreamFramer::new();
    for chunk in max.chunks(17) {
        fr.push(chunk).unwrap();
    }
    assert_eq!(fr.next_frame().unwrap(), vec![7u8; MAX_FRAME_LEN]);
    assert!(fr.next_frame().is_none());
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]

    /// Random splits of a frame sequence reassemble identically, through
    /// the framer directly and through a PlatformLink.
    #[test]
    fn random_splits_reassemble_identically(
        frames in prop::collection::vec(
            prop::collection::vec(any::<u8>(), MIN_FRAME_LEN..1200), 0..8),
        seed in any::<u64>(),
        max_chunk in 1usize..600,
    ) {
        let stream: Vec<u8> = frames.iter().flat_map(|f| frame(f).unwrap()).collect();
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let mut fr = StreamFramer::new();
        let mut link = PlatformLink::new(LinkId(3), info(Role::Central, 247));
        let mut got = Vec::new();
        let mut via_link = Vec::new();
        let mut pos = 0;
        while pos < stream.len() {
            let n = rng.random_range(1..=max_chunk).min(stream.len() - pos);
            fr.push(&stream[pos..pos + n]).unwrap();
            link.feed(&stream[pos..pos + n]).unwrap();
            pos += n;
            while let Some(f) = fr.next_frame() {
                got.push(f);
            }
            while let Some(f) = link.recv() {
                via_link.push(f);
            }
        }
        prop_assert_eq!(&got, &frames);
        prop_assert_eq!(&via_link, &frames);
        prop_assert_eq!(fr.pending_bytes(), 0);
    }

    /// DESIGN §10 `noise_stream`: random garbage never panics the framer,
    /// the link adaptor or the Noise reader.
    #[test]
    fn random_garbage_never_panics(
        chunks in prop::collection::vec(prop::collection::vec(any::<u8>(), 0..2000), 0..20),
        seed in any::<u64>(),
    ) {
        let mut fr = StreamFramer::new();
        let mut link = PlatformLink::new(LinkId(4), info(Role::Peripheral, 20));
        let mut p = handshake(20, seed, seed ^ 1);
        for c in &chunks {
            let _ = fr.push(c);
            let _ = link.feed(c);
            while let Some(f) = fr.next_frame() {
                prop_assert!(f.len() <= MAX_FRAME_LEN);
            }
            while let Some(f) = link.recv() {
                prop_assert!(f.len() <= MAX_FRAME_LEN);
                let res = p.nr.decrypt_frame(&f);
                prop_assert!(res.is_err());
            }
            let mut r = NoiseLink::responder(&mut ChaCha20Rng::seed_from_u64(seed)).unwrap();
            let _ = r.read_handshake(c);
        }
    }
}
