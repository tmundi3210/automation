//! The invariant checker (docs/scenarios.md "pass criteria"; DESIGN §10
//! "Invariant checker (`sim/meshsim/assert.rs`) runs over every
//! scenario"). In this wave the nodes carry opaque cells, so only the
//! criteria that are meaningful without crypto, roster or votes are
//! evaluated; every other enabled criterion is reported as
//! `not_evaluated` with the reason, never silently passed.
//!
//! Evaluated here:
//!
//! * `every_post_decrypts_after_rejoin` → every post or DM cell created at
//!   least [`DELIVERY_GRACE_MS`] (or half the run, whichever is shorter)
//!   before the end is held by every node at the end (DESIGN §10 S1 (b)
//!   "after rejoin every honest node holds every post from all fragments";
//!   decrypting is the next wave's job);
//! * `no_duplicate_ui_delivery` → no node recorded the same `msg_id` as
//!   newly seen twice (S1 (e));
//! * `zero_wall_clock_reads` → the instrumented clocks counted no
//!   `wall_for_display` call (S4 (e));
//! * `drain_report` → reported only (S1 (d) "reported, threshold soft"):
//!   the time from the rejoin until every pre-rejoin cell reached every
//!   node.

use serde::Serialize;

use crate::report::{Delivery, Miss};
use crate::scenario::Scenario;
use crate::simnode::{CellKind, CellRegistry, SimNode};

/// A cell must have had this long to spread before the end counts it
/// (docs: "created ≥ 30 min before the end"); short scenarios use half
/// their duration so that S0 still evaluates something.
pub const DELIVERY_GRACE_MS: u64 = 30 * 60_000;
/// Misses listed in the report (the count is always complete).
pub const MAX_MISSES_LISTED: usize = 50;

/// Verdict of one criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Pass,
    Fail,
    NotEvaluated,
}

/// One line of the `criteria` array.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Criterion {
    pub name: String,
    pub status: Status,
    pub detail: String,
}

impl Criterion {
    fn new(name: &str, status: Status, detail: impl Into<String>) -> Self {
        Criterion {
            name: name.to_string(),
            status,
            detail: detail.into(),
        }
    }

    fn not_evaluated(name: &str, why: &str) -> Self {
        Self::new(name, Status::NotEvaluated, why)
    }

    pub fn failed(&self) -> bool {
        self.status == Status::Fail
    }
}

/// Everything the checker computed.
#[derive(Debug, Clone)]
pub struct Evaluation {
    pub criteria: Vec<Criterion>,
    pub delivery: Delivery,
    pub drain_ms: Option<u64>,
}

const NEEDS_REAL_NODE: &str =
    "not evaluated in this wave: needs the real node (roster, keys, votes); cells are opaque";
const NEEDS_FLOOD_MEASURE: &str =
    "not evaluated in this wave: honest-latency and L3-cap measurement needs validated cells";

/// Delivery of post and DM cells created early enough, per node.
fn delivery(
    reg: &CellRegistry,
    nodes: &[SimNode],
    duration_ms: u64,
) -> (Delivery, Vec<(usize, Vec<usize>)>) {
    let grace = DELIVERY_GRACE_MS.min(duration_ms / 2);
    let cutoff = duration_ms.saturating_sub(grace);
    let mut evaluated = 0u64;
    let mut pairs = 0u64;
    let mut delivered = 0u64;
    let mut misses: Vec<(usize, Vec<usize>)> = Vec::new();
    for (i, rec) in reg.records.iter().enumerate() {
        if rec.kind == CellKind::Junk || rec.created_ms > cutoff {
            continue;
        }
        evaluated += 1;
        let mut missing = Vec::new();
        for node in nodes {
            pairs += 1;
            if node.holds(&rec.id) {
                delivered += 1;
            } else {
                missing.push(node.idx());
            }
        }
        if !missing.is_empty() {
            misses.push((i, missing));
        }
    }
    let ratio = if pairs == 0 {
        1.0
    } else {
        delivered as f64 / pairs as f64
    };
    let listed = misses
        .iter()
        .take(MAX_MISSES_LISTED)
        .map(|(i, missing)| {
            let rec = &reg.records[*i];
            Miss {
                msg_id: rec.id.to_string(),
                creator: rec.creator,
                created_ms: rec.created_ms,
                fragment: rec.fragment,
                missing_at: missing.clone(),
            }
        })
        .collect();
    (
        Delivery {
            ratio,
            evaluated_cells: evaluated,
            miss_count: u64::try_from(misses.len()).unwrap_or(u64::MAX),
            misses: listed,
        },
        misses,
    )
}

/// Drain time after the rejoin (module docs). `Err` carries how many
/// pre-rejoin cells never reached every node.
fn drain(reg: &CellRegistry, rejoin_at: u64) -> Result<u64, u64> {
    let mut latest = 0u64;
    let mut incomplete = 0u64;
    for rec in &reg.records {
        if rec.kind == CellKind::Junk || rec.created_ms >= rejoin_at {
            continue;
        }
        match rec.first_seen.iter().max() {
            Some(&u64::MAX) | None => incomplete += 1,
            Some(&t) => latest = latest.max(t),
        }
    }
    if incomplete > 0 {
        Err(incomplete)
    } else {
        Ok(latest.saturating_sub(rejoin_at))
    }
}

/// Evaluates the `[pass]` table of `scenario` over the finished run.
pub fn evaluate(
    scenario: &Scenario,
    nodes: &[SimNode],
    reg: &CellRegistry,
    rejoin_at: Option<u64>,
    duration_ms: u64,
) -> Evaluation {
    let pass = &scenario.pass;
    let mut criteria = Vec::new();
    let (delivery, _) = delivery(reg, nodes, duration_ms);

    if pass.every_post_decrypts_after_rejoin {
        let status = if delivery.miss_count == 0 {
            Status::Pass
        } else {
            Status::Fail
        };
        criteria.push(Criterion::new(
            "every_post_decrypts_after_rejoin",
            status,
            format!(
                "opaque form: {} of {} early cells held by every node (ratio {:.4}); decrypt check needs the real node",
                delivery.evaluated_cells - delivery.miss_count,
                delivery.evaluated_cells,
                delivery.ratio
            ),
        ));
    }
    if pass.members_h_identical {
        criteria.push(Criterion::not_evaluated(
            "members_h_identical",
            NEEDS_REAL_NODE,
        ));
    }
    if pass.no_solid_state_reverts {
        criteria.push(Criterion::not_evaluated(
            "no_solid_state_reverts",
            NEEDS_REAL_NODE,
        ));
    }
    if pass.no_duplicate_ui_delivery {
        let dups = reg.duplicate_first_seen;
        let status = if dups == 0 {
            Status::Pass
        } else {
            Status::Fail
        };
        criteria.push(Criterion::new(
            "no_duplicate_ui_delivery",
            status,
            format!("{dups} msg_ids recorded as newly seen twice at one node"),
        ));
    }
    if pass.message_keys_deleted_after_delivery {
        criteria.push(Criterion::not_evaluated(
            "message_keys_deleted_after_delivery",
            NEEDS_REAL_NODE,
        ));
    }
    if pass.removed_cannot_decrypt_post_removal {
        criteria.push(Criterion::not_evaluated(
            "removed_cannot_decrypt_post_removal",
            NEEDS_REAL_NODE,
        ));
    }
    if pass.sockpuppet_adds_void_everywhere {
        criteria.push(Criterion::not_evaluated(
            "sockpuppet_adds_void_everywhere",
            NEEDS_REAL_NODE,
        ));
    }
    if pass.removed_posts_labelled_or_rejected {
        criteria.push(Criterion::not_evaluated(
            "removed_posts_labelled_or_rejected",
            NEEDS_REAL_NODE,
        ));
    }
    if pass.replays_never_redelivered {
        criteria.push(Criterion::not_evaluated(
            "replays_never_redelivered",
            NEEDS_REAL_NODE,
        ));
    }
    if pass.no_final_while_split {
        criteria.push(Criterion::not_evaluated(
            "no_final_while_split",
            NEEDS_REAL_NODE,
        ));
    }
    if !pass.expected_vote_states.is_empty() {
        criteria.push(Criterion::not_evaluated(
            "expected_vote_states",
            NEEDS_REAL_NODE,
        ));
    }
    if pass.final_never_changes {
        criteria.push(Criterion::not_evaluated(
            "final_never_changes",
            NEEDS_REAL_NODE,
        ));
    }
    if pass.late_ballot_counts {
        criteria.push(Criterion::not_evaluated(
            "late_ballot_counts",
            NEEDS_REAL_NODE,
        ));
    }
    if pass.zero_wall_clock_reads {
        let reads: u64 = nodes.iter().map(SimNode::wall_reads).sum();
        let status = if reads == 0 {
            Status::Pass
        } else {
            Status::Fail
        };
        criteria.push(Criterion::new(
            "zero_wall_clock_reads",
            status,
            format!("{reads} wall_for_display calls through the Clock trait"),
        ));
    }
    if pass.flood_honest_latency_factor_max > 0.0 {
        criteria.push(Criterion::not_evaluated(
            "flood_honest_latency_factor_max",
            NEEDS_FLOOD_MEASURE,
        ));
    }
    if pass.flood_junk_within_l3_cap {
        criteria.push(Criterion::not_evaluated(
            "flood_junk_within_l3_cap",
            NEEDS_FLOOD_MEASURE,
        ));
    }
    let drain_ms = rejoin_at.and_then(|r| drain(reg, r).ok());
    if pass.drain_report {
        let detail = match (rejoin_at, rejoin_at.map(|r| drain(reg, r))) {
            (None, _) => "report only: no rejoin in this scenario".to_string(),
            (Some(r), Some(Ok(ms))) => format!(
                "report only: rejoin at {r} ms, every pre-rejoin cell everywhere {ms} ms later"
            ),
            (Some(r), Some(Err(n))) => {
                format!(
                    "report only: rejoin at {r} ms, {n} pre-rejoin cells never reached every node"
                )
            }
            (Some(_), None) => "report only".to_string(),
        };
        criteria.push(Criterion::new("drain_report", Status::NotEvaluated, detail));
    }

    Evaluation {
        criteria,
        delivery,
        drain_ms,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::{NodeKind, parse};
    use crate::simnode::{Audience, NodeConfig};
    use std::collections::BTreeSet;

    fn node(idx: usize) -> SimNode {
        SimNode::new(
            NodeConfig {
                idx,
                kind: NodeKind::Android,
                slots: 7,
                battery_floor_ppm: 300_000,
                battery_start_ppm: 1_000_000,
                foreground_duty_ppm: 1_000_000,
                wall_offset_ms: 0,
                dedupe_retention_ms: 1_000_000,
                contacts: BTreeSet::new(),
            },
            idx as u64,
        )
    }

    fn scenario(pass: &str) -> Scenario {
        parse(&format!(
            r#"
[scenario]
name = "t"
duration_ms = 100000
tick_ms = 100
[nodes]
android = 2
iphone = 0
iphone_foreground_duty = 1.0
android_slots = 7
battery_floor = 0.3
[radio]
latency_ms = 40
throughput_bytes_per_s = 15000
frame_loss = 0.0
iphone_wake_ms = 10000
wall_clock_offset_ms = [0, 0]
[traffic]
group_post_every_ms = 100
text_bytes = [40, 300]
[pass]
{pass}
"#
        ))
        .unwrap()
    }

    #[test]
    fn delivery_fails_on_a_miss_and_lists_it() {
        let scn = scenario(
            "every_post_decrypts_after_rejoin = true\nno_duplicate_ui_delivery = true\nzero_wall_clock_reads = true\ndrain_report = true\nmembers_h_identical = true",
        );
        let mut reg = CellRegistry::new(2, vec![BTreeSet::from([0, 1])]);
        let mut nodes = vec![node(0), node(1)];
        let early = nodes[0].create_cell(CellKind::Post, Audience::Group(0), 0, 1_000, &mut reg);
        // a late cell is outside the grace window and not evaluated
        nodes[1].create_cell(CellKind::Post, Audience::Group(0), 0, 90_000, &mut reg);
        // junk is never evaluated
        nodes[1].create_cell(CellKind::Junk, Audience::Nobody, 0, 1_000, &mut reg);
        let ev = evaluate(&scn, &nodes, &reg, Some(50_000), 100_000);
        assert_eq!(ev.delivery.evaluated_cells, 1);
        assert_eq!(ev.delivery.miss_count, 1);
        assert_eq!(ev.delivery.ratio, 0.5);
        assert_eq!(ev.delivery.misses[0].msg_id, early.to_string());
        assert_eq!(ev.delivery.misses[0].missing_at, vec![1]);
        assert!(ev.criteria.iter().any(Criterion::failed));
        let by_name = |n: &str| ev.criteria.iter().find(|c| c.name == n).unwrap().status;
        assert_eq!(by_name("every_post_decrypts_after_rejoin"), Status::Fail);
        assert_eq!(by_name("no_duplicate_ui_delivery"), Status::Pass);
        assert_eq!(by_name("zero_wall_clock_reads"), Status::Pass);
        assert_eq!(by_name("members_h_identical"), Status::NotEvaluated);
        assert_eq!(by_name("drain_report"), Status::NotEvaluated);
        assert_eq!(ev.drain_ms, None, "the early cell never reached node 1");
    }

    #[test]
    fn drain_is_measured_from_the_rejoin_and_wall_reads_fail_the_check() {
        let scn = scenario("zero_wall_clock_reads = true\ndrain_report = true");
        let mut reg = CellRegistry::new(2, vec![BTreeSet::from([0, 1])]);
        let mut nodes = vec![node(0), node(1)];
        let id = nodes[0].create_cell(CellKind::Post, Audience::Group(0), 0, 1_000, &mut reg);
        reg.record_first_seen(&id, 1, 70_000);
        let _ = meshcore::Clock::wall_for_display(&nodes[1].clock);
        let ev = evaluate(&scn, &nodes, &reg, Some(50_000), 100_000);
        assert_eq!(ev.drain_ms, Some(20_000));
        assert!(ev.criteria.iter().any(Criterion::failed));
        assert_eq!(ev.criteria[0].name, "zero_wall_clock_reads");
        assert_eq!(ev.criteria[0].status, Status::Fail);
        // duplicate first-seen is a violation
        reg.record_first_seen(&id, 1, 71_000);
        let scn = scenario("no_duplicate_ui_delivery = true");
        let ev = evaluate(&scn, &nodes, &reg, None, 100_000);
        assert_eq!(ev.criteria[0].status, Status::Fail);
        assert_eq!(ev.drain_ms, None);
    }

    #[test]
    fn nothing_enabled_means_nothing_fails() {
        let scn = scenario("");
        let reg = CellRegistry::new(2, vec![BTreeSet::from([0, 1])]);
        let nodes = vec![node(0), node(1)];
        let ev = evaluate(&scn, &nodes, &reg, None, 100_000);
        assert!(ev.criteria.is_empty());
        assert_eq!(ev.delivery.ratio, 1.0);
    }
}
