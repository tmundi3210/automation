//! The modelled BLE radio (DESIGN §7 `SimLink` row; docs/testing.md
//! "Assumptions baked into the simulator"): connectivity from the active
//! partition, link records with their airtime, per-frame loss, the
//! locked-iPhone wake rule and the per-pair byte counters behind "bytes on
//! bottleneck links".
//!
//! Timing (assumption, DESIGN §7): a batch of `n` bytes sent at `t` on a
//! link whose direction is free arrives at `t + latency_ms + n /
//! throughput`; the direction is busy until `t + n / throughput`, so later
//! batches queue behind it. Loss is rolled once per Noise frame the sender
//! handed to the platform; a lost frame dooms the link at its arrival time
//! (DESIGN §7 "a lost chunk means the link is torn down and
//! re-handshaked").
//!
//! Chunking (DESIGN §7 "chunk at MTU−3"): bytes are chunked at
//! [`CHUNK_ANDROID_ANDROID`] between two Android/station nodes and at
//! [`CHUNK_WITH_IPHONE`] when either end is an iPhone, so the stream framer
//! sees both boundaries.
//!
//! The radio owns no clock: `now_ms` comes from the engine.

use std::collections::BTreeMap;

use crate::scenario::{NodeKind, ResolvedPartition};
use crate::simnode::DropReason;

/// ATT MTU−3 between two Android nodes (517-byte MTU; DESIGN §8).
pub const CHUNK_ANDROID_ANDROID: usize = 512;
/// ATT MTU−3 when an iPhone is at either end (iOS 185-byte MTU; assumption).
pub const CHUNK_WITH_IPHONE: usize = 185;

/// The radio's scenario parameters (`[radio]`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadioParams {
    pub latency_ms: u64,
    pub throughput_bytes_per_s: u64,
    pub frame_loss: f64,
    pub iphone_wake_ms: u64,
}

/// One live link between two nodes.
#[derive(Debug, Clone)]
pub struct RadioLink {
    pub id: u32,
    /// The initiator (central).
    pub a: usize,
    /// The responder (peripheral).
    pub b: usize,
    pub up_at: u64,
    pub chunk: usize,
    /// For a link into a locked iPhone: when the wake ends.
    pub wake_deadline: Option<u64>,
    /// Earliest time a lost frame tears the link down.
    pub doom_at: Option<u64>,
    /// Airtime reserved per direction (`[a→b, b→a]`).
    busy_until: [u64; 2],
    /// Bytes carried per direction.
    pub bytes: [u64; 2],
}

impl RadioLink {
    pub fn peer_of(&self, node: usize) -> usize {
        if node == self.a { self.b } else { self.a }
    }

    pub fn pair(&self) -> (usize, usize) {
        (self.a.min(self.b), self.a.max(self.b))
    }

    fn dir_index(&self, from: usize) -> usize {
        usize::from(from != self.a)
    }
}

/// A closed link, kept for the report and the tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkRecord {
    pub id: u32,
    pub a: usize,
    pub b: usize,
    pub up_at: u64,
    pub down_at: u64,
    pub reason: DropReason,
    pub wake: bool,
}

/// The radio. See the module docs.
#[derive(Debug)]
pub struct Radio {
    params: RadioParams,
    kinds: Vec<NodeKind>,
    fragment_of: Vec<usize>,
    fragment_count: usize,
    links: BTreeMap<u32, RadioLink>,
    by_pair: BTreeMap<(usize, usize), u32>,
    next_id: u32,
    /// Bytes carried per unordered node pair over the whole run.
    pub pair_bytes: BTreeMap<(usize, usize), u64>,
    pub history: Vec<LinkRecord>,
}

impl Radio {
    pub fn new(params: RadioParams, kinds: Vec<NodeKind>) -> Self {
        let n = kinds.len();
        Radio {
            params,
            kinds,
            fragment_of: vec![0; n],
            fragment_count: 1,
            links: BTreeMap::new(),
            by_pair: BTreeMap::new(),
            next_id: 1,
            pair_bytes: BTreeMap::new(),
            history: Vec::new(),
        }
    }

    pub fn params(&self) -> RadioParams {
        self.params
    }

    pub fn fragment_of(&self, node: usize) -> usize {
        self.fragment_of.get(node).copied().unwrap_or(0)
    }

    pub fn fragment_count(&self) -> usize {
        self.fragment_count
    }

    /// Applies a partition and returns the links it cuts (nodes in
    /// different fragments never link; existing cross-fragment links drop).
    pub fn set_partition(&mut self, p: &ResolvedPartition) -> Vec<u32> {
        self.fragment_of.clone_from(&p.fragment_of);
        self.fragment_count = p.fragment_count;
        self.links
            .values()
            .filter(|l| !self.reachable(l.a, l.b))
            .map(|l| l.id)
            .collect()
    }

    /// Whether two (known, distinct) nodes are in the same fragment.
    pub fn reachable(&self, a: usize, b: usize) -> bool {
        match (self.fragment_of.get(a), self.fragment_of.get(b)) {
            (Some(fa), Some(fb)) => a != b && fa == fb,
            _ => false,
        }
    }

    pub fn is_linked(&self, a: usize, b: usize) -> bool {
        self.by_pair.contains_key(&(a.min(b), a.max(b)))
    }

    pub fn link(&self, id: u32) -> Option<&RadioLink> {
        self.links.get(&id)
    }

    pub fn links(&self) -> impl Iterator<Item = &RadioLink> + '_ {
        self.links.values()
    }

    /// Chunk size for a pair of node kinds (module docs "Chunking").
    pub fn chunk_for(&self, a: usize, b: usize) -> usize {
        let iphone = |n: usize| self.kinds.get(n) == Some(&NodeKind::IPhone);
        if iphone(a) || iphone(b) {
            CHUNK_WITH_IPHONE
        } else {
            CHUNK_ANDROID_ANDROID
        }
    }

    /// Opens a link dialled by `a` into `b`. `wake` is set when `b` is a
    /// locked iPhone: the link lives exactly `iphone_wake_ms`.
    pub fn open(&mut self, a: usize, b: usize, now: u64, wake: bool) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        let chunk = self.chunk_for(a, b);
        self.links.insert(
            id,
            RadioLink {
                id,
                a,
                b,
                up_at: now,
                chunk,
                wake_deadline: wake.then(|| now.saturating_add(self.params.iphone_wake_ms)),
                doom_at: None,
                busy_until: [now, now],
                bytes: [0, 0],
            },
        );
        self.by_pair.insert((a.min(b), a.max(b)), id);
        id
    }

    /// Closes a link and records it.
    pub fn close(&mut self, id: u32, now: u64, reason: DropReason) -> Option<RadioLink> {
        let link = self.links.remove(&id)?;
        self.by_pair.remove(&link.pair());
        self.history.push(LinkRecord {
            id,
            a: link.a,
            b: link.b,
            up_at: link.up_at,
            down_at: now,
            reason,
            wake: link.wake_deadline.is_some(),
        });
        Some(link)
    }

    /// Airtime of `bytes` on this radio, rounded up to whole milliseconds.
    pub fn airtime_ms(&self, bytes: usize) -> u64 {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        bytes
            .saturating_mul(1000)
            .div_ceil(self.params.throughput_bytes_per_s.max(1))
    }

    /// Reserves airtime for `bytes` from `from` on link `id` at `now` and
    /// returns their arrival time (module docs "Timing").
    pub fn transmit(&mut self, id: u32, from: usize, bytes: usize, now: u64) -> Option<u64> {
        let air = self.airtime_ms(bytes);
        let latency = self.params.latency_ms;
        let link = self.links.get_mut(&id)?;
        let d = link.dir_index(from);
        let start = link.busy_until[d].max(now);
        let done = start.saturating_add(air);
        link.busy_until[d] = done;
        link.bytes[d] = link.bytes[d].saturating_add(u64::try_from(bytes).unwrap_or(u64::MAX));
        let pair = link.pair();
        let total = self.pair_bytes.entry(pair).or_insert(0);
        *total = total.saturating_add(u64::try_from(bytes).unwrap_or(u64::MAX));
        Some(done.saturating_add(latency))
    }

    /// A frame from `from` was lost: the link dies when that frame would
    /// have arrived. Returns the doom time if it is new or earlier.
    pub fn doom(&mut self, id: u32, from: usize, now: u64) -> Option<u64> {
        let latency = self.params.latency_ms;
        let link = self.links.get_mut(&id)?;
        let d = link.dir_index(from);
        let at = link.busy_until[d].max(now).saturating_add(latency);
        match link.doom_at {
            Some(existing) if existing <= at => None,
            _ => {
                link.doom_at = Some(at);
                Some(at)
            }
        }
    }

    /// `(max, p95)` of bytes per node pair over the run.
    pub fn bottleneck_bytes(&self) -> (u64, u64) {
        let mut v: Vec<u64> = self.pair_bytes.values().copied().collect();
        if v.is_empty() {
            return (0, 0);
        }
        v.sort_unstable();
        let max = v[v.len() - 1];
        let idx = (v.len() * 95)
            .div_ceil(100)
            .saturating_sub(1)
            .min(v.len() - 1);
        (max, v[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn radio(kinds: Vec<NodeKind>) -> Radio {
        Radio::new(
            RadioParams {
                latency_ms: 40,
                throughput_bytes_per_s: 15_000,
                frame_loss: 0.0,
                iphone_wake_ms: 10_000,
            },
            kinds,
        )
    }

    #[test]
    fn partition_cuts_cross_fragment_links_and_blocks_reachability() {
        let mut r = radio(vec![NodeKind::Android; 4]);
        let l01 = r.open(0, 1, 0, false);
        let l23 = r.open(2, 3, 0, false);
        let l12 = r.open(1, 2, 0, false);
        assert!(r.is_linked(2, 1));
        let p = ResolvedPartition {
            at_ms: 10,
            fragment_of: vec![0, 0, 1, 1],
            fragment_count: 2,
        };
        let cut = r.set_partition(&p);
        assert_eq!(cut, vec![l12]);
        assert!(r.reachable(0, 1) && r.reachable(2, 3));
        assert!(!r.reachable(1, 2) && !r.reachable(0, 0));
        assert!(
            !r.reachable(4, 5) && !r.reachable(0, 4),
            "unknown nodes are in no fragment"
        );
        assert!(r.close(l12, 10, DropReason::Partition).is_some());
        assert!(r.close(l12, 10, DropReason::Partition).is_none());
        assert!(r.link(l01).is_some() && r.link(l23).is_some());
        assert_eq!(r.history.len(), 1);
        assert_eq!(r.history[0].reason, DropReason::Partition);
    }

    #[test]
    fn airtime_queues_per_direction_and_counts_pair_bytes() {
        let mut r = radio(vec![NodeKind::Android, NodeKind::IPhone]);
        let id = r.open(0, 1, 100, true);
        assert_eq!(r.link(id).unwrap().wake_deadline, Some(10_100));
        assert_eq!(r.link(id).unwrap().chunk, CHUNK_WITH_IPHONE);
        // 1500 bytes = 100 ms of air, plus 40 ms latency
        assert_eq!(r.transmit(id, 0, 1500, 100), Some(240));
        // the next batch in the same direction queues behind the first
        assert_eq!(r.transmit(id, 0, 1500, 100), Some(340));
        // the other direction is independent
        assert_eq!(r.transmit(id, 1, 150, 100), Some(150));
        assert_eq!(r.pair_bytes[&(0, 1)], 3150);
        assert_eq!(r.link(id).unwrap().bytes, [3000, 150]);
        assert_eq!(r.doom(id, 0, 100), Some(340));
        assert_eq!(
            r.doom(id, 0, 100),
            None,
            "not earlier than the existing doom"
        );
        assert_eq!(r.bottleneck_bytes(), (3150, 3150));
        assert_eq!(r.transmit(99, 0, 1, 0), None);
    }

    #[test]
    fn chunk_depends_on_the_pair() {
        let r = radio(vec![NodeKind::Android, NodeKind::IPhone, NodeKind::Station]);
        assert_eq!(r.chunk_for(0, 2), CHUNK_ANDROID_ANDROID);
        assert_eq!(r.chunk_for(0, 1), CHUNK_WITH_IPHONE);
        assert_eq!(r.chunk_for(1, 2), CHUNK_WITH_IPHONE);
    }

    #[test]
    fn p95_of_pair_bytes() {
        let mut r = radio(vec![NodeKind::Android; 30]);
        for i in 0..20 {
            let id = r.open(i, i + 1, 0, false);
            r.transmit(id, i, (i + 1) * 100, 0);
        }
        let (max, p95) = r.bottleneck_bytes();
        assert_eq!(max, 2000);
        assert_eq!(p95, 1900);
    }
}
