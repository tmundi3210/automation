//! The JSON result of one run (docs/scenarios.md: `meshsim run … --out
//! result.json`; `tools/run-scenarios.sh` collects one per seed) and its
//! one-line summary. Field order is fixed by the struct, maps are
//! `BTreeMap`s, so the same run always serialises to the same bytes (the
//! determinism test compares them).

use std::collections::BTreeMap;

use serde::Serialize;

use crate::assert::Criterion;

/// One node at the end of the run.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NodeReport {
    pub idx: usize,
    pub kind: &'static str,
    /// iPhones: in the foreground at the end.
    pub foreground: bool,
    /// Battery level at the end, in percent (one decimal).
    pub battery_pct: f64,
    pub cells_held: usize,
    pub links_up: usize,
    /// Display-only wall-clock offset, read through the display hook.
    pub wall_offset_ms: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct CellsCreated {
    pub total: u64,
    pub posts: u64,
    pub dms: u64,
    pub junk: u64,
}

/// A cell that an evaluated criterion found missing somewhere.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Miss {
    pub msg_id: String,
    pub creator: usize,
    pub created_ms: u64,
    pub fragment: usize,
    pub missing_at: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct Delivery {
    /// Fraction of evaluated (cell, node) pairs delivered.
    pub ratio: f64,
    /// Cells the criterion looked at.
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
    pub digest_matches: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct CellTraffic {
    pub frames_sent: u64,
    pub cells_sent: u64,
    pub cells_received: u64,
    pub duplicates: u64,
    pub malformed: u64,
    pub throttled_in: u64,
    pub throttled_out: u64,
    pub rejected_in: u64,
    pub rejected_out: u64,
    pub no_room: u64,
    pub throttles_received: u64,
    pub withheld_low_battery: u64,
    pub summary_errors: u64,
}

/// The whole result.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Report {
    pub scenario: String,
    pub seed: u64,
    pub duration_ms: u64,
    pub nodes: Vec<NodeReport>,
    pub cells_created: CellsCreated,
    pub delivery: Delivery,
    /// Time from the rejoin until every pre-rejoin cell reached every
    /// node; `null` when there was no rejoin or the drain never finished.
    pub drain_ms: Option<u64>,
    pub rejoin_at_ms: Option<u64>,
    pub bytes_on_links: BytesOnLinks,
    pub links: Links,
    pub cells: CellTraffic,
    pub wall_clock_reads: u64,
    /// Scenario events this wave cannot act on (roster, tasks, votes,
    /// non-flood adversaries), by kind.
    pub events_ignored: BTreeMap<String, u64>,
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
        format!(
            "{} seed={} nodes={} cells={} delivery={:.4} misses={} drain={} links(est={} drop={} hs={}) max_link_bytes={} wall_reads={} {}",
            self.scenario,
            self.seed,
            self.nodes.len(),
            self.cells_created.total,
            self.delivery.ratio,
            self.delivery.miss_count,
            drain,
            self.links.established,
            self.links.dropped,
            self.links.handshakes,
            self.bytes_on_links.max_link_bytes,
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
            nodes: vec![NodeReport {
                idx: 0,
                kind: "android",
                foreground: true,
                battery_pct: 50.0,
                cells_held: 1,
                links_up: 0,
                wall_offset_ms: 0,
            }],
            cells_created: CellsCreated::default(),
            delivery: Delivery::default(),
            drain_ms: None,
            rejoin_at_ms: None,
            bytes_on_links: BytesOnLinks::default(),
            links: Links::default(),
            cells: CellTraffic::default(),
            wall_clock_reads: 0,
            events_ignored: BTreeMap::new(),
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
