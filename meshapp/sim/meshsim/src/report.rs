//! The JSON result of one run (docs/scenarios.md: `meshsim run … --out
//! result.json`; `tools/run-scenarios.sh` collects one per seed) and its
//! one-line summary. Field order is fixed by the struct, maps are
//! `BTreeMap`s, so the same run always serialises to the same bytes (the
//! determinism test compares them).
//!
//! Counts are of *objects* (texts: group posts and DMs, by `ObjId`) where
//! the opaque-cell wave counted cells; `cells_created.junk` still counts
//! junk cells. "Delivered" everywhere means shown at the node
//! (`Event::Post` / `Event::Dm`).

use std::collections::BTreeMap;

use serde::Serialize;

use crate::assert::Criterion;

/// One node at the end of the run.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NodeReport {
    pub idx: usize,
    pub name: String,
    pub kind: &'static str,
    /// iPhones: in the foreground at the end.
    pub foreground: bool,
    /// Battery level at the end, in percent (one decimal).
    pub battery_pct: f64,
    pub cells_held: usize,
    pub links_up: usize,
    /// Display-only wall-clock offset, read through the display hook.
    pub wall_offset_ms: i64,
    /// Group posts by others this node showed.
    pub posts_shown: u64,
    /// DMs this node showed.
    pub dms_shown: u64,
    /// Scheduled posts skipped (group not held yet, or no longer a member).
    pub posts_skipped: u64,
    pub decrypted: u64,
    pub decrypt_failures: u64,
    /// Posts rejected for citing their author's removal.
    pub rejected_posts: u64,
    /// Duplicate cells dropped by the dedupe cache.
    pub duplicates: u64,
    /// Objects parked as "Waiting · can't read yet".
    pub parked: u64,
    pub commits: u64,
    pub rollbacks: u64,
    /// Link penalties applied.
    pub penalties: u64,
    /// Rows in the node's key-value store at the end.
    pub kv_rows: u64,
    pub wall_reads: u64,
    /// "Last sync" set at the end (BRIEF_v2 §2: the digest matched a
    /// neighbour's).
    pub last_sync_set: bool,
}

/// Objects created by the generator (`posts`, `dms`) and junk cells.
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct CellsCreated {
    /// `posts + dms`.
    pub total: u64,
    pub posts: u64,
    pub dms: u64,
    /// Junk cells injected by the `flood` adversary.
    pub junk: u64,
    /// Setup hellos sent in the in-person setup phase (not in `total`).
    pub setup: u64,
    /// Objects whose sender's delivery state reached `Delivered` (a receipt
    /// came back) by the end; DESIGN §10 S0 "every text Delivered; receipts".
    pub delivered_at_sender: u64,
}

/// An object that an evaluated criterion found missing somewhere.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Miss {
    /// The `ObjId`.
    pub msg_id: String,
    pub creator: usize,
    pub created_ms: u64,
    pub fragment: usize,
    pub missing_at: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct Delivery {
    /// Fraction of evaluated (object, expected node) pairs shown.
    pub ratio: f64,
    /// Objects the criterion looked at.
    pub evaluated_cells: u64,
    pub miss_count: u64,
    /// The first [`MAX_MISSES_LISTED`](crate::assert::MAX_MISSES_LISTED).
    pub misses: Vec<Miss>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct BytesOnLinks {
    pub max_link_bytes: u64,
    pub p95: u64,
    pub pairs: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct Links {
    pub established: u64,
    pub live_at_end: u64,
    pub dropped: u64,
    pub handshakes: u64,
    pub wake_links: u64,
    pub attempts: u64,
    pub attempts_failed: u64,
    pub dropped_by_reason: BTreeMap<String, u64>,
    /// Links lifted to L2 by a contact proof (`LinkState` events).
    pub l2_links: u64,
}

/// Node counters summed over every node (`Node::stats`), plus the radio
/// batch count.
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct CellTraffic {
    /// Radio batches handed to the air.
    pub frames_sent: u64,
    pub cells_sent: u64,
    pub cells_received: u64,
    pub duplicates: u64,
    pub malformed: u64,
    pub throttled_in: u64,
    pub rejected_in: u64,
    pub no_room: u64,
    pub decrypted: u64,
    pub decrypt_failures: u64,
    pub rejected_posts: u64,
    pub parked: u64,
    pub commits: u64,
    pub rollbacks: u64,
    pub penalties: u64,
    /// Cell offers the `replay_cells` adversary queued.
    pub replay_offers: u64,
}

/// The vote lines read on every node at one `expected_vote_states` time.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct VoteSnapshotReport {
    pub vote: String,
    pub at_ms: u64,
    pub expected: String,
    /// Line → number of nodes showing it.
    pub lines: BTreeMap<String, u64>,
}

/// The whole result.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Report {
    pub scenario: String,
    pub seed: u64,
    pub duration_ms: u64,
    /// The one-line summary (`Report::summary_line`).
    pub summary: String,
    pub nodes: Vec<NodeReport>,
    pub cells_created: CellsCreated,
    pub delivery: Delivery,
    /// Time from the rejoin until every pre-rejoin object was shown at
    /// every expected node; `null` when there was no rejoin or the drain
    /// never finished.
    pub drain_ms: Option<u64>,
    pub rejoin_at_ms: Option<u64>,
    pub split_at_ms: Option<u64>,
    pub bytes_on_links: BytesOnLinks,
    pub links: Links,
    pub cells: CellTraffic,
    pub wall_clock_reads: u64,
    /// Scenario events applied, by kind (retried ops carry their delay).
    pub events_applied: BTreeMap<String, u64>,
    /// Scenario events the node refused, by kind and error.
    pub events_failed: BTreeMap<String, u64>,
    /// Ops still waiting for their task or vote at the end.
    pub events_pending: BTreeMap<String, u64>,
    pub vote_snapshots: Vec<VoteSnapshotReport>,
    /// Mean show latency of honest texts during the flood window.
    pub honest_latency_ms: Option<f64>,
    /// The same without the flooder (same seed).
    pub baseline_latency_ms: Option<f64>,
    pub criteria: Vec<Criterion>,
    /// Host seconds the run took (informational; excluded from the
    /// determinism comparison by [`Report::deterministic_json`]).
    pub host_seconds: f64,
}

impl Report {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
    }

    /// The JSON without the host timing, for byte-for-byte comparison.
    #[cfg(test)]
    pub fn deterministic_json(&self) -> String {
        let mut copy = self.clone();
        copy.host_seconds = 0.0;
        copy.to_json()
    }

    /// `true` if no evaluated criterion failed.
    pub fn passed(&self) -> bool {
        self.criteria.iter().all(|c| !c.failed())
    }

    pub fn summary_line(&self) -> String {
        let drain = match self.drain_ms {
            Some(ms) => format!("{} s", ms / 1000),
            None => "n/a".to_string(),
        };
        let failed: Vec<&str> = self
            .criteria
            .iter()
            .filter(|c| c.failed())
            .map(|c| c.name.as_str())
            .collect();
        let verdict = if failed.is_empty() {
            "PASS".to_string()
        } else {
            format!("FAIL [{}]", failed.join(", "))
        };
        let shown: u64 = self.nodes.iter().map(|n| n.posts_shown + n.dms_shown).sum();
        format!(
            "{} seed={} nodes={} objects={} (posts={} dms={} junk={}) shown={} delivery={:.4} misses={} drain={} links(est={} drop={} hs={}) decrypted={} dups={} wall_reads={} {}",
            self.scenario,
            self.seed,
            self.nodes.len(),
            self.cells_created.total,
            self.cells_created.posts,
            self.cells_created.dms,
            self.cells_created.junk,
            shown,
            self.delivery.ratio,
            self.delivery.miss_count,
            drain,
            self.links.established,
            self.links.dropped,
            self.links.handshakes,
            self.cells.decrypted,
            self.cells.duplicates,
            self.wall_clock_reads,
            verdict
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert::Status;

    fn report() -> Report {
        Report {
            scenario: "t".into(),
            seed: 3,
            duration_ms: 10,
            summary: String::new(),
            nodes: vec![NodeReport {
                idx: 0,
                name: "Node 0".into(),
                kind: "android",
                foreground: true,
                battery_pct: 50.0,
                cells_held: 1,
                links_up: 0,
                wall_offset_ms: 0,
                posts_shown: 0,
                dms_shown: 0,
                posts_skipped: 0,
                decrypted: 0,
                decrypt_failures: 0,
                rejected_posts: 0,
                duplicates: 0,
                parked: 0,
                commits: 0,
                rollbacks: 0,
                penalties: 0,
                kv_rows: 0,
                wall_reads: 0,
                last_sync_set: false,
            }],
            cells_created: CellsCreated::default(),
            delivery: Delivery::default(),
            drain_ms: None,
            rejoin_at_ms: None,
            split_at_ms: None,
            bytes_on_links: BytesOnLinks::default(),
            links: Links::default(),
            cells: CellTraffic::default(),
            wall_clock_reads: 0,
            events_applied: BTreeMap::new(),
            events_failed: BTreeMap::new(),
            events_pending: BTreeMap::new(),
            vote_snapshots: Vec::new(),
            honest_latency_ms: None,
            baseline_latency_ms: None,
            criteria: vec![Criterion {
                name: "x".into(),
                status: Status::Fail,
                detail: "d".into(),
            }],
            host_seconds: 1.5,
        }
    }

    #[test]
    fn json_is_stable_and_drops_host_time_for_comparison() {
        let r = report();
        let j = r.deterministic_json();
        assert!(j.contains("\"host_seconds\": 0.0"));
        assert!(j.contains("\"status\": \"fail\""));
        assert!(!r.passed());
        assert!(r.summary_line().contains("FAIL [x]"));
        assert_eq!(r.deterministic_json(), r.clone().deterministic_json());
    }
}
