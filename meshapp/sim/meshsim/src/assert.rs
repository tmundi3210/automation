//! The invariant checker (docs/scenarios.md "pass criteria"; DESIGN §10
//! "Invariant checker (`sim/meshsim/assert.rs`) runs over every
//! scenario"). Every criterion reads the [`Trace`] the engine folded from
//! the nodes' events and the nodes' queries at the end of the run; a
//! criterion that cannot be evaluated in a run (no removal, no vote, no
//! flooder) is reported as `not_evaluated` with the reason, never silently
//! passed. "Delivered" always means *shown* at the node (`Event::Post` /
//! `Event::Dm`), never store presence.
//!
//! The `[pass]` table selects which criteria are enforced; the invariants
//! listed under "always on" below run on every scenario regardless
//! (`invariant:*` lines in the report) and fail the run like any other:
//! hollow/solid monotonicity, identical `members(H)` per head set across
//! nodes, no duplicate UI delivery, no `Final` disagreement, no decrypt of
//! post-removal traffic by a removed node, bounded buffers.
//!
//! Criteria (DESIGN §10 table):
//!
//! * `every_post_decrypts_after_rejoin` (S1 (a)–(b)): every object created
//!   at least [`DELIVERY_GRACE_MS`] (or half the run) before the end was
//!   shown at every expected node by the end, and every object created
//!   during a split at least the grace before the rejoin was shown at the
//!   expected nodes of its own fragment before the rejoin;
//! * `members_h_identical` (S1 (c)): nodes with the same roster head set
//!   hold the same `members(H)`;
//! * `no_solid_state_reverts` (S1 (f)): no `Delivered` or `Done` reverted;
//! * `no_duplicate_ui_delivery` (S1 (e)): no object shown twice at a node;
//! * `message_keys_deleted_after_delivery` (S1 (g)): see the criterion;
//! * `removed_cannot_decrypt_post_removal` (S3 (a)), `sockpuppet_adds_void_everywhere`
//!   (S3 (b)), `removed_posts_labelled_or_rejected` (S3 (c)),
//!   `replays_never_redelivered` (S3 (d));
//! * `no_final_while_split`, `expected_vote_states`, `final_never_changes`
//!   (the decision, Final and its winner, never changes; the displayed
//!   counts may still grow as late ballots arrive),
//!   `late_ballot_counts` (S4 (a)–(d)); `zero_wall_clock_reads` (S4 (e));
//! * `flood_honest_latency_factor_max`, `flood_junk_within_l3_cap` (S-flood);
//! * `drain_report` (S1 (d), reported only).

use std::collections::{BTreeMap, BTreeSet};

use meshcore::crypto::backend::GroupId;
use meshcore::crypto::ed25519::IdentityKey;
use meshcore::crypto::hash::Hash32;
use meshcore::node::VoteId;
use meshcore::relay::Lane;
use meshcore::relay::buffers::BufferPolicy;
use meshcore::relay::limits::{L2_QUOTA_MULT_ASM, TEXT_CELLS_PER_MIN_ASM};
use meshcore::votes::tally::VoteState;
use meshcore::wire::cell::CELL_LEN;
use serde::Serialize;

use crate::report::{Delivery, Miss};
use crate::scenario::{ExpectedVoteState, NodeKind, Resolved, Scenario};
use crate::simnode::{ObjKind, ObjRecord, ObjRegistry, SimNode, Trace};

/// An object must have had this long to spread before the end counts it
/// (docs: "created ≥ 30 min before the end"); short scenarios use half
/// their duration so that S0 still evaluates something.
pub const DELIVERY_GRACE_MS: u64 = 30 * 60_000;
/// Misses listed in the report (the count is always complete).
pub const MAX_MISSES_LISTED: usize = 50;
/// The removed-member label the UI shows (`roster::labels::sent_by_removed`).
const REMOVED_LABEL: &str = "who was removed";

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

    fn check(name: &str, ok: bool, detail: impl Into<String>) -> Self {
        Self::new(name, if ok { Status::Pass } else { Status::Fail }, detail)
    }

    fn not_evaluated(name: &str, why: impl Into<String>) -> Self {
        Self::new(name, Status::NotEvaluated, why)
    }

    pub fn failed(&self) -> bool {
        self.status == Status::Fail
    }
}

/// The vote lines read on every node at one `expected_vote_states` time.
#[derive(Debug, Clone)]
pub struct VoteSnapshot {
    pub index: usize,
    pub at_ms: u64,
    pub lines: Vec<String>,
}

/// What the S-flood criteria need.
#[derive(Debug, Clone)]
pub struct FloodInfo<'a> {
    pub flooder: usize,
    pub start_ms: u64,
    pub junk_ids: &'a BTreeSet<Hash32>,
    pub baseline_latency_ms: Option<f64>,
    /// Scores honest nodes give their live links to the flooder at the end.
    pub link_scores: Vec<f32>,
}

/// Everything the checker reads.
pub struct EvalInput<'a> {
    pub scenario: &'a Scenario,
    pub resolved: &'a Resolved,
    pub nodes: &'a [SimNode],
    pub trace: &'a Trace,
    pub rejoin_at: Option<u64>,
    pub split_at: Option<u64>,
    pub duration_ms: u64,
    pub group_ids: &'a [Option<GroupId>],
    pub vote_ids: &'a BTreeMap<String, VoteId>,
    pub vote_snapshots: &'a [VoteSnapshot],
    pub ids: &'a [Option<IdentityKey>],
    /// Nodes running `keep_keys_after_removal`.
    pub adversaries: &'a BTreeSet<usize>,
    /// Subjects of scenario `remove_member` events.
    pub removal_subjects: &'a BTreeSet<usize>,
    pub puppets: &'a [IdentityKey],
    pub replay_offers: u64,
    pub flood: Option<FloodInfo<'a>>,
    /// Nodes the courier event moved out of their scheduled fragment.
    pub moved: &'a BTreeSet<usize>,
    /// Ballots cast through the generator: label → (node, at).
    pub ballots_cast: &'a BTreeMap<String, Vec<(usize, u64)>>,
    pub equivocators: &'a BTreeSet<usize>,
}

/// Everything the checker computed.
#[derive(Debug, Clone)]
pub struct Evaluation {
    pub criteria: Vec<Criterion>,
    pub delivery: Delivery,
    pub drain_ms: Option<u64>,
    pub honest_latency_ms: Option<f64>,
}

fn grace(duration_ms: u64) -> u64 {
    DELIVERY_GRACE_MS.min(duration_ms / 2)
}

/// Delivery of every object created early enough, at its expected nodes.
fn delivery(reg: &ObjRegistry, duration_ms: u64) -> (Delivery, Vec<(usize, Vec<usize>)>) {
    let cutoff = duration_ms.saturating_sub(grace(duration_ms));
    let mut evaluated = 0u64;
    let mut pairs = 0u64;
    let mut delivered = 0u64;
    let mut misses: Vec<(usize, Vec<usize>)> = Vec::new();
    for (i, rec) in reg.records.iter().enumerate() {
        if rec.created_ms > cutoff || rec.setup {
            continue;
        }
        evaluated += 1;
        let mut missing = Vec::new();
        for &node in &rec.expected {
            pairs += 1;
            if rec.shown.get(node).is_some_and(|&t| t != u64::MAX) {
                delivered += 1;
            } else {
                missing.push(node);
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
                msg_id: rec.obj.to_string(),
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

/// S1 (a): objects created during a split (at least the grace before the
/// rejoin) must be shown at the expected nodes of the creator's fragment
/// before the rejoin. Returns `(evaluated, misses)`.
fn fragment_delivery(input: &EvalInput<'_>) -> (u64, u64) {
    let Some(rejoin) = input.rejoin_at else {
        return (0, 0);
    };
    let cutoff = rejoin.saturating_sub(grace(input.duration_ms));
    let mut evaluated = 0u64;
    let mut misses = 0u64;
    for rec in &input.trace.registry.records {
        if rec.setup
            || rec.created_ms > cutoff
            || rec.created_ms >= rejoin
            || input.moved.contains(&rec.creator)
        {
            continue;
        }
        let Some(p) = input.resolved.partitions.get(rec.partition_idx) else {
            continue;
        };
        if p.fragment_count <= 1 {
            continue;
        }
        for &x in &rec.expected {
            if input.moved.contains(&x) || p.fragment_of.get(x) != Some(&rec.fragment) {
                continue;
            }
            evaluated += 1;
            if !rec.shown.get(x).is_some_and(|&t| t < rejoin) {
                misses += 1;
            }
        }
    }
    (evaluated, misses)
}

/// Drain time after the rejoin (module docs). `Err` carries how many
/// pre-rejoin objects never reached every expected node.
fn drain(reg: &ObjRegistry, rejoin_at: u64) -> Result<u64, u64> {
    let mut latest = 0u64;
    let mut incomplete = 0u64;
    for rec in &reg.records {
        if rec.created_ms >= rejoin_at || rec.setup {
            continue;
        }
        let mut done = true;
        for &x in &rec.expected {
            match rec.shown.get(x) {
                Some(&u64::MAX) | None => done = false,
                Some(&t) => latest = latest.max(t),
            }
        }
        if !done {
            incomplete += 1;
        }
    }
    if incomplete > 0 {
        Err(incomplete)
    } else {
        Ok(latest.saturating_sub(rejoin_at))
    }
}

/// Mean show latency (ms) of objects created in `[start, cutoff]` by nodes
/// other than `exclude`, over every expected node that showed them.
pub fn honest_latency_ms(
    reg: &ObjRegistry,
    start_ms: u64,
    duration_ms: u64,
    exclude: Option<usize>,
) -> Option<f64> {
    let cutoff = duration_ms.saturating_sub(grace(duration_ms));
    let mut total: u128 = 0;
    let mut count: u64 = 0;
    for rec in &reg.records {
        if rec.setup
            || rec.created_ms < start_ms
            || rec.created_ms > cutoff
            || Some(rec.creator) == exclude
        {
            continue;
        }
        for &x in &rec.expected {
            if let Some(&t) = rec.shown.get(x)
                && t != u64::MAX
            {
                total += u128::from(t.saturating_sub(rec.created_ms));
                count += 1;
            }
        }
    }
    if count == 0 {
        None
    } else {
        Some(total as f64 / count as f64)
    }
}

/// The line an `expected_vote_states` entry describes: "Y10 N10 · 1 void ·
/// Provisional" (`votes::state::tally_line`).
pub fn expected_vote_line(v: &ExpectedVoteState) -> String {
    let mut parts = vec![v.tally.clone()];
    if v.void > 0 {
        parts.push(format!("{} void", v.void));
    }
    parts.push(v.state.clone());
    parts.join(" · ")
}

/// Members of scenario group `g` that are sim nodes holding a roster.
fn member_nodes<'a>(input: &'a EvalInput<'_>, g: usize) -> impl Iterator<Item = usize> + 'a {
    input.resolved.groups[g]
        .members
        .iter()
        .copied()
        .filter(move |&m| input.resolved.kinds[m] != NodeKind::Station)
}

/// One member node's roster at the end: `(head_set_hash, members(H))`, or
/// `None` when it does not hold the group.
type RosterSummary = Option<(Hash32, Vec<IdentityKey>)>;

/// One group's verdict over `views` = `(node, RosterSummary)` per member
/// node: `(invariant_ok, criterion_ok, detail)`. The always-on invariant asks that nodes with the same head set
/// agree on `members(H)`; the `[pass]` criterion (S1 (c) "`members(H)`
/// identical on all 40 for the common head") further asks that every member
/// node holds the roster and, when `require_converged` (the run has been
/// unpartitioned for at least the grace), that all of them share one head
/// set: a run in which every node sits on its own head set has no common
/// head to agree on and must not pass by vacuity.
fn members_verdict(
    name: &str,
    n_members: usize,
    views: &[(usize, RosterSummary)],
    require_converged: bool,
) -> (bool, bool, String) {
    let mut by_heads: BTreeMap<Hash32, BTreeMap<Vec<IdentityKey>, Vec<usize>>> = BTreeMap::new();
    let mut missing = Vec::new();
    for (m, view) in views {
        match view {
            Some((heads, members)) => by_heads
                .entry(*heads)
                .or_default()
                .entry(members.clone())
                .or_default()
                .push(*m),
            None => missing.push(*m),
        }
    }
    let disagreements = by_heads.values().filter(|sets| sets.len() > 1).count();
    let invariant_ok = disagreements == 0;
    let converged = by_heads.len() <= 1;
    let criterion_ok = invariant_ok && missing.is_empty() && (converged || !require_converged);
    let detail = format!(
        "{}: {} head set(s) among {} member nodes{}, {} with disagreeing members(H), {} nodes without the roster{}",
        name,
        by_heads.len(),
        n_members,
        if require_converged {
            " (one required: unpartitioned for the grace)"
        } else {
            ""
        },
        disagreements,
        missing.len(),
        if missing.is_empty() {
            String::new()
        } else {
            format!(" {missing:?}")
        }
    );
    (invariant_ok, criterion_ok, detail)
}

/// `members_h_identical` and the always-on invariant over every group
/// ([`members_verdict`]): `(invariant_ok, criterion_ok, detail)`.
fn members_identical(input: &EvalInput<'_>) -> (bool, bool, String) {
    let last = input.resolved.partitions.last();
    let require_converged = last.is_some_and(|p| {
        p.fragment_count == 1
            && p.at_ms.saturating_add(grace(input.duration_ms)) <= input.duration_ms
    });
    let mut invariant_ok = true;
    let mut criterion_ok = true;
    let mut details = Vec::new();
    for (g, rg) in input.resolved.groups.iter().enumerate() {
        let Some(gid) = input.group_ids[g] else {
            details.push(format!("{}: never created", rg.name));
            invariant_ok = false;
            criterion_ok = false;
            continue;
        };
        let views: Vec<(usize, RosterSummary)> = member_nodes(input, g)
            .map(|m| {
                let view = input.nodes[m]
                    .node()
                    .roster(&gid)
                    .ok()
                    .map(|v| (v.head_set_hash(), v.members.keys().copied().collect()));
                (m, view)
            })
            .collect();
        let (inv, crit, detail) =
            members_verdict(&rg.name, rg.members.len(), &views, require_converged);
        invariant_ok &= inv;
        criterion_ok &= crit;
        details.push(detail);
    }
    (invariant_ok, criterion_ok, details.join("; "))
}

/// Whether `rec` was created after `creator` applied the removal of `key`.
fn created_after_applied(trace: &Trace, rec: &ObjRecord, node: usize, key: &IdentityKey) -> bool {
    trace
        .removal_applied
        .get(&(node, *key))
        .is_some_and(|&t| t <= rec.created_ms)
}

/// S3 (a) and the always-on invariant: for every removal subject, no post
/// by a member who had applied the removal was shown at the subject.
/// Whether any node ever dropped `key` from its `members(H)`.
fn removal_took_effect(trace: &Trace, key: &IdentityKey) -> bool {
    trace.removal_applied.keys().any(|(_, k)| k == key)
}

fn removed_cannot_decrypt(input: &EvalInput<'_>) -> Option<(bool, String)> {
    let mut subjects: Vec<usize> = input.removal_subjects.iter().copied().collect();
    if subjects.is_empty() {
        return None;
    }
    if !subjects
        .iter()
        .any(|&s| input.ids[s].is_some_and(|k| removal_took_effect(input.trace, &k)))
    {
        return None;
    }
    subjects.sort_unstable();
    let mut ok = true;
    let mut details = Vec::new();
    for s in subjects {
        let Some(key) = input.ids[s] else {
            continue;
        };
        let mut post_removal = 0u64;
        let mut shown_at_subject = 0u64;
        for rec in &input.trace.registry.records {
            if rec.creator == s || !matches!(rec.kind, ObjKind::Post { .. }) {
                continue;
            }
            if created_after_applied(input.trace, rec, rec.creator, &key) {
                post_removal += 1;
                if rec.shown.get(s).is_some_and(|&t| t != u64::MAX) {
                    shown_at_subject += 1;
                }
            }
        }
        if shown_at_subject > 0 {
            ok = false;
        }
        let stats = input.nodes[s].stats();
        details.push(format!(
            "node {s}{}: {post_removal} posts made after their author applied the removal, {shown_at_subject} shown at it (its decrypt failures {}, decrypted {})",
            if input.adversaries.contains(&s) {
                " (keeps keys)"
            } else {
                ""
            },
            stats.decrypt_failures,
            stats.decrypted
        ));
    }
    Some((ok, details.join("; ")))
}

/// S3 (c): the removed node's own posts: those citing its removal hidden
/// everywhere; the others labelled at every node that already knew.
fn removed_posts_labelled(input: &EvalInput<'_>) -> Option<(bool, String)> {
    let mut subjects: Vec<usize> = input.removal_subjects.iter().copied().collect();
    if subjects.is_empty() {
        return None;
    }
    if !subjects
        .iter()
        .any(|&s| input.ids[s].is_some_and(|k| removal_took_effect(input.trace, &k)))
    {
        return None;
    }
    subjects.sort_unstable();
    let mut ok = true;
    let mut any = false;
    let mut details = Vec::new();
    for s in subjects {
        let Some(key) = input.ids[s] else {
            continue;
        };
        let mut citing = 0u64;
        let mut citing_shown = 0u64;
        let mut earlier = 0u64;
        let mut labelled = 0u64;
        let mut unlabelled = 0u64;
        for rec in &input.trace.registry.records {
            if rec.creator != s || !matches!(rec.kind, ObjKind::Post { .. }) {
                continue;
            }
            if created_after_applied(input.trace, rec, s, &key) {
                citing += 1;
                citing_shown += rec
                    .shown
                    .iter()
                    .enumerate()
                    .filter(|(x, t)| *x != s && **t != u64::MAX)
                    .count() as u64;
            } else {
                // Every other post by the subject: the label follows the
                // viewer's knowledge at show time (BRIEF_v2 §5.4 "any other
                // is kept, collapsed and labelled"; `rotation::post_verdict`),
                // whenever the post was made.
                let mut shown_knowing = false;
                for (x, &t) in rec.shown.iter().enumerate() {
                    if x == s || t == u64::MAX {
                        continue;
                    }
                    let knew = input
                        .trace
                        .removal_applied
                        .get(&(x, key))
                        .is_some_and(|&a| a < t);
                    if !knew {
                        continue;
                    }
                    shown_knowing = true;
                    let has_label = rec
                        .labels
                        .get(&x)
                        .is_some_and(|l| l.iter().any(|s| s.contains(REMOVED_LABEL)));
                    if has_label {
                        labelled += 1;
                    } else {
                        unlabelled += 1;
                    }
                }
                if shown_knowing {
                    earlier += 1;
                }
            }
        }
        if citing + earlier > 0 {
            any = true;
        }
        if citing_shown > 0 || unlabelled > 0 {
            ok = false;
        }
        let rejected: u64 = input
            .nodes
            .iter()
            .enumerate()
            .filter(|(x, _)| *x != s)
            .map(|(_, n)| n.stats().rejected_posts)
            .sum();
        details.push(format!(
            "node {s}: {citing} posts citing its removal shown {citing_shown} times (honest nodes rejected {rejected} posts); {earlier} posts without it in heads shown at nodes that knew: {labelled} labelled, {unlabelled} unlabelled"
        ));
    }
    if !any {
        return Some((
            true,
            format!(
                "no post by a removed node after its removal; {}",
                details.join("; ")
            ),
        ));
    }
    Some((ok, details.join("; ")))
}

/// S4 vote lines at the end, per vote: `node → (state, line)`.
fn vote_lines_at_end(
    input: &EvalInput<'_>,
    vote_id: &VoteId,
) -> BTreeMap<usize, (Option<VoteState>, String)> {
    let mut out = BTreeMap::new();
    for (i, n) in input.nodes.iter().enumerate() {
        if let Ok((tally, line)) = n.node().vote(vote_id) {
            out.insert(i, (tally.map(|t| t.state), line));
        }
    }
    out
}

/// Distinct winning options among the nodes showing `Final` (D1: the
/// decision is the option, the displayed counts may still differ while
/// late ballots arrive).
fn distinct_final_winners(lines: &BTreeMap<usize, (Option<VoteState>, String)>) -> usize {
    lines
        .values()
        .filter_map(|(state, _)| match state {
            Some(VoteState::Final { option }) => Some(*option),
            _ => None,
        })
        .collect::<BTreeSet<u8>>()
        .len()
}

/// Evaluates the `[pass]` table of `scenario` over the finished run plus
/// the always-on invariants.
pub fn evaluate(input: &EvalInput<'_>) -> Evaluation {
    let pass = &input.scenario.pass;
    let reg = &input.trace.registry;
    let trace = input.trace;
    let mut criteria = Vec::new();
    let (delivery, _) = delivery(reg, input.duration_ms);
    let (frag_evaluated, frag_misses) = fragment_delivery(input);

    if pass.every_post_decrypts_after_rejoin {
        let ok = delivery.miss_count == 0 && frag_misses == 0;
        let withheld: Vec<&ObjRecord> = reg.records.iter().filter(|r| r.withheld).collect();
        let withheld_shown = withheld
            .iter()
            .filter(|r| {
                r.expected
                    .iter()
                    .all(|&x| r.shown.get(x).is_some_and(|&t| t != u64::MAX))
            })
            .count();
        criteria.push(Criterion::check(
            "every_post_decrypts_after_rejoin",
            ok,
            format!(
                "{} of {} early objects shown at every expected node (ratio {:.4}); split-time in-fragment pairs {} of {} shown before the rejoin; {} skipped posts (group not held yet / not a member); withheld DMs released {} of which {withheld_shown} shown (S3 (f))",
                delivery.evaluated_cells - delivery.miss_count,
                delivery.evaluated_cells,
                delivery.ratio,
                frag_evaluated - frag_misses,
                frag_evaluated,
                input.nodes.iter().map(|n| n.posts_skipped).sum::<u64>(),
                withheld.len()
            ),
        ));
    }
    let (members_inv_ok, members_ok, members_detail) = members_identical(input);
    if pass.members_h_identical {
        criteria.push(Criterion::check(
            "members_h_identical",
            members_ok,
            members_detail.clone(),
        ));
    }
    let solid_ok = trace.solid_reverts.is_empty() && trace.done_reverts.is_empty();
    let solid_detail = format!(
        "{} delivery events, {} Delivered reverts, {} Done reverts{}{}",
        trace.delivery_events,
        trace.solid_reverts.len(),
        trace.done_reverts.len(),
        trace
            .solid_reverts
            .first()
            .map(|s| format!("; first: {s}"))
            .unwrap_or_default(),
        trace
            .done_reverts
            .first()
            .map(|s| format!("; first: {s}"))
            .unwrap_or_default()
    );
    if pass.no_solid_state_reverts {
        criteria.push(Criterion::check(
            "no_solid_state_reverts",
            solid_ok,
            solid_detail.clone(),
        ));
    }
    let dup_detail = format!(
        "{} objects shown twice at one node, {} shown objects the sim never created",
        reg.duplicate_shown, reg.unknown_shown
    );
    if pass.no_duplicate_ui_delivery {
        criteria.push(Criterion::check(
            "no_duplicate_ui_delivery",
            reg.duplicate_shown == 0 && reg.unknown_shown == 0,
            dup_detail.clone(),
        ));
    }
    if pass.message_keys_deleted_after_delivery {
        // D6 rule 4: the backend consumes message keys inside its session
        // and sender-key records (one opaque row per peer / per chain),
        // so no per-message row can exist; the measurable proxy is the kv
        // growth per object, dominated by the persisted dedupe marks (one
        // per cell seen), reported for the record.
        let rows: u64 = input
            .nodes
            .iter()
            .map(|n| {
                n.node()
                    .kv_len()
                    .unwrap_or(0)
                    .saturating_sub(n.kv_rows_at_start)
            })
            .sum();
        let objects: u64 = reg
            .records
            .iter()
            .map(|r| r.shown.iter().filter(|&&t| t != u64::MAX).count() as u64)
            .sum();
        criteria.push(Criterion::not_evaluated(
            "message_keys_deleted_after_delivery",
            format!(
                "no per-message key rows exist by construction (libsignal consumes them inside the session/sender-key records on decrypt, D6 rule 4); kv rows grew by {rows} over {objects} shown objects and {} cells received (dedupe marks included)",
                input.nodes.iter().map(|n| n.stats().cells_received).sum::<u64>()
            ),
        ));
    }
    let removed_eval = removed_cannot_decrypt(input);
    if pass.removed_cannot_decrypt_post_removal {
        criteria.push(match &removed_eval {
            Some((ok, d)) => {
                Criterion::check("removed_cannot_decrypt_post_removal", *ok, d.clone())
            }
            None => Criterion::not_evaluated(
                "removed_cannot_decrypt_post_removal",
                "no remove_member event in this scenario, or no node ever applied the removal (a void op: removing an admin needs a demotion first, D3)",
            ),
        });
    }
    if pass.sockpuppet_adds_void_everywhere {
        criteria.push(sockpuppets(input));
    }
    if pass.removed_posts_labelled_or_rejected {
        criteria.push(match removed_posts_labelled(input) {
            Some((ok, d)) => Criterion::check("removed_posts_labelled_or_rejected", ok, d),
            None => Criterion::not_evaluated(
                "removed_posts_labelled_or_rejected",
                "no remove_member event in this scenario, or no node ever applied the removal (a void op: removing an admin needs a demotion first, D3)",
            ),
        });
    }
    if pass.replays_never_redelivered {
        let dups: u64 = input.nodes.iter().map(|n| n.stats().duplicates).sum();
        criteria.push(if input.replay_offers == 0 {
            Criterion::not_evaluated(
                "replays_never_redelivered",
                "no replay_cells event fired (or the replaying node had no pushing link)",
            )
        } else {
            Criterion::check(
                "replays_never_redelivered",
                reg.duplicate_shown == 0,
                format!(
                    "{} replayed cell offers; the nodes dropped {dups} duplicate cells; {} objects shown twice",
                    input.replay_offers, reg.duplicate_shown
                ),
            )
        });
    }
    // Votes.
    let finals_while_split: Vec<String> = match (input.split_at, input.rejoin_at) {
        (Some(s), Some(r)) => trace
            .votes
            .iter()
            .filter(|v| {
                v.at_ms > s && v.at_ms < r && matches!(v.state, Some(VoteState::Final { .. }))
            })
            .map(|v| {
                format!(
                    "node {} vote {} \"{}\" at {} ms",
                    v.node,
                    hex::encode(&v.vote_id.as_bytes()[..4]),
                    v.line,
                    v.at_ms
                )
            })
            .collect(),
        _ => Vec::new(),
    };
    if pass.no_final_while_split {
        criteria.push(if input.vote_ids.is_empty() {
            Criterion::not_evaluated("no_final_while_split", "no vote in this run")
        } else if input.split_at.is_none() || input.rejoin_at.is_none() {
            Criterion::not_evaluated("no_final_while_split", "no split and rejoin in this run")
        } else {
            Criterion::check(
                "no_final_while_split",
                finals_while_split.is_empty(),
                format!(
                    "{} Final lines between the split and the rejoin{}",
                    finals_while_split.len(),
                    finals_while_split
                        .first()
                        .map(|s| format!("; first: {s}"))
                        .unwrap_or_default()
                ),
            )
        });
    }
    if !pass.expected_vote_states.is_empty() {
        let mut ok = true;
        let mut details = Vec::new();
        for (i, v) in pass.expected_vote_states.iter().enumerate() {
            let expected = expected_vote_line(v);
            let Some(snap) = input.vote_snapshots.iter().find(|s| s.index == i) else {
                ok = false;
                details.push(format!("{} at {} ms: never sampled", v.vote, v.at_ms));
                continue;
            };
            let voters: BTreeSet<usize> = input
                .resolved
                .groups
                .iter()
                .flat_map(|g| g.members.iter().copied())
                .collect();
            let mut counts: BTreeMap<&str, u64> = BTreeMap::new();
            let mut mismatches = 0u64;
            for (n, line) in snap.lines.iter().enumerate() {
                if !voters.contains(&n) {
                    continue;
                }
                *counts.entry(line.as_str()).or_insert(0) += 1;
                if *line != expected {
                    mismatches += 1;
                }
            }
            if mismatches > 0 {
                ok = false;
            }
            details.push(format!(
                "{} at {} ms: expected \"{expected}\"{}, seen {:?}",
                v.vote,
                v.at_ms,
                v.winner
                    .as_ref()
                    .map(|w| format!(" (winner {w})"))
                    .unwrap_or_default(),
                counts
            ));
        }
        criteria.push(Criterion::check(
            "expected_vote_states",
            ok,
            details.join("; "),
        ));
    }
    if pass.final_never_changes {
        let finals = trace
            .votes
            .iter()
            .filter(|v| matches!(v.state, Some(VoteState::Final { .. })))
            .count();
        criteria.push(if finals == 0 {
            Criterion::not_evaluated("final_never_changes", "no node reached Final")
        } else {
            Criterion::check(
                "final_never_changes",
                trace.final_changes.is_empty(),
                format!(
                    "{finals} Final lines, {} decisions (state or winner) changed afterwards{}",
                    trace.final_changes.len(),
                    trace
                        .final_changes
                        .first()
                        .map(|s| format!("; first: {s}"))
                        .unwrap_or_default()
                ),
            )
        });
    }
    if pass.late_ballot_counts {
        criteria.push(late_ballots(input));
    }
    let reads: u64 = input.nodes.iter().map(SimNode::wall_reads).sum();
    if pass.zero_wall_clock_reads {
        criteria.push(Criterion::check(
            "zero_wall_clock_reads",
            reads == 0,
            format!("{reads} wall-clock reads (display hook + Clock trait) on every node"),
        ));
    }
    // Flood.
    let honest_latency_ms = input
        .flood
        .as_ref()
        .and_then(|f| honest_latency_ms(reg, f.start_ms, input.duration_ms, Some(f.flooder)));
    if pass.flood_honest_latency_factor_max > 0.0 {
        criteria.push(match &input.flood {
            None => Criterion::not_evaluated("flood_honest_latency_factor_max", "no flood event"),
            Some(f) => match (f.baseline_latency_ms, honest_latency_ms) {
                (Some(base), Some(now)) if base > 0.0 => {
                    let factor = now / base;
                    Criterion::check(
                        "flood_honest_latency_factor_max",
                        factor <= pass.flood_honest_latency_factor_max,
                        format!(
                            "mean show latency of honest texts during the flood {now:.0} ms vs {base:.0} ms without the flooder (same seed): factor {factor:.2} (max {})",
                            pass.flood_honest_latency_factor_max
                        ),
                    )
                }
                _ => Criterion::not_evaluated(
                    "flood_honest_latency_factor_max",
                    "no baseline or no honest text shown during the flood window",
                ),
            },
        });
    }
    if pass.flood_junk_within_l3_cap {
        criteria.push(match &input.flood {
            None => Criterion::not_evaluated("flood_junk_within_l3_cap", "no flood event"),
            Some(f) => junk_within_cap(input, f),
        });
    }
    if let Some(f) = &input.flood {
        criteria.push(flood_penalty(input, f));
    }
    let drain_ms = input.rejoin_at.and_then(|r| drain(reg, r).ok());
    if pass.drain_report {
        let detail = match (input.rejoin_at, input.rejoin_at.map(|r| drain(reg, r))) {
            (None, _) => "report only: no rejoin in this scenario".to_string(),
            (Some(r), Some(Ok(ms))) => format!(
                "report only: rejoin at {r} ms, every pre-rejoin object shown everywhere {ms} ms later"
            ),
            (Some(r), Some(Err(n))) => {
                format!(
                    "report only: rejoin at {r} ms, {n} pre-rejoin objects never shown at every expected node"
                )
            }
            (Some(_), None) => "report only".to_string(),
        };
        criteria.push(Criterion::new("drain_report", Status::NotEvaluated, detail));
    }

    // Always-on invariants (module docs).
    criteria.push(Criterion::check(
        "invariant:solid_states_monotone",
        solid_ok,
        solid_detail,
    ));
    criteria.push(Criterion::check(
        "invariant:members_h_identical_per_head_set",
        members_inv_ok,
        members_detail,
    ));
    criteria.push(Criterion::check(
        "invariant:no_duplicate_ui_delivery",
        reg.duplicate_shown == 0,
        dup_detail,
    ));
    criteria.push(final_agreement(input));
    if let Some((ok, d)) = removed_eval {
        criteria.push(Criterion::check(
            "invariant:removed_cannot_decrypt_post_removal",
            ok,
            d,
        ));
    }
    criteria.push(bounded_buffers(input));

    Evaluation {
        criteria,
        delivery,
        drain_ms,
        honest_latency_ms,
    }
}

/// S3 (b): the sock-puppet keys are members nowhere at the end.
fn sockpuppets(input: &EvalInput<'_>) -> Criterion {
    if input.puppets.is_empty() {
        return Criterion::not_evaluated(
            "sockpuppet_adds_void_everywhere",
            "no sockpuppet_adds event applied",
        );
    }
    let mut members_somewhere = 0u64;
    let mut listed_changed = 0u64;
    let mut ever_member = 0u64;
    let mut nodes_checked = 0u64;
    for (g, _) in input.resolved.groups.iter().enumerate() {
        let Some(gid) = input.group_ids[g] else {
            continue;
        };
        for m in member_nodes(input, g) {
            let Ok(view) = input.nodes[m].node().roster(&gid) else {
                continue;
            };
            nodes_checked += 1;
            for p in input.puppets {
                if view.is_member(p) {
                    members_somewhere += 1;
                }
                if view.changed_after_reconnect.contains(p) {
                    listed_changed += 1;
                }
                if input.trace.removal_applied.contains_key(&(m, *p)) {
                    ever_member += 1;
                }
            }
        }
    }
    if ever_member == 0 && members_somewhere == 0 {
        return Criterion::not_evaluated(
            "sockpuppet_adds_void_everywhere",
            format!(
                "the {} sock-puppet Adds never appeared in any node's members(H) (not applied or never delivered)",
                input.puppets.len()
            ),
        );
    }
    Criterion::check(
        "sockpuppet_adds_void_everywhere",
        members_somewhere == 0,
        format!(
            "{} puppets × {nodes_checked} member nodes: {members_somewhere} still members at the end, {listed_changed} listed as \"Changed after reconnect\", {ever_member} (node, puppet) pairs applied then voided",
            input.puppets.len()
        ),
    )
}

/// Member nodes of the group the vote `label` was opened in (the opener's
/// first group, as the engine picks it).
fn vote_member_nodes(input: &EvalInput<'_>, label: &str) -> Vec<usize> {
    let opener = input.scenario.events.iter().find_map(|e| match e {
        crate::scenario::Event::VoteOpen { by, label: l, .. } if l == label => Some(*by),
        _ => None,
    });
    let Some(by) = opener else {
        return Vec::new();
    };
    input
        .resolved
        .groups
        .iter()
        .position(|g| g.members.contains(&by))
        .map(|g| member_nodes(input, g).collect())
        .unwrap_or_default()
}

/// S4 (d): every honest ballot cast is heard (not void, not "not heard")
/// at every member node by the end; the latest cast is named.
fn late_ballots(input: &EvalInput<'_>) -> Criterion {
    if input.ballots_cast.is_empty() {
        return Criterion::not_evaluated("late_ballot_counts", "no ballot cast in this run");
    }
    let mut ok = true;
    let mut details = Vec::new();
    for (label, casts) in input.ballots_cast {
        let Some(vid) = input.vote_ids.get(label) else {
            ok = false;
            details.push(format!("{label}: vote never opened"));
            continue;
        };
        let latest = casts.iter().max_by_key(|(_, at)| *at).copied();
        let mut missing = 0u64;
        let mut checked = 0u64;
        let honest: Vec<usize> = casts
            .iter()
            .map(|(c, _)| *c)
            .filter(|c| !input.equivocators.contains(c) && input.ids[*c].is_some())
            .collect();
        // The vote lives in the opener's group: a member node that never
        // held it, or holds it without a resolvable tally ("Waiting · needs
        // a sync"), has counted nothing (not a vacuous pass).
        let mut members = vote_member_nodes(input, label);
        if members.is_empty() {
            members = (0..input.nodes.len()).collect();
        }
        for n in members {
            let tally = match input.nodes[n].node().vote(vid) {
                Ok((Some(t), _)) => t,
                Ok((None, line)) => {
                    checked += u64::try_from(honest.len()).unwrap_or(u64::MAX);
                    missing += u64::try_from(honest.len()).unwrap_or(u64::MAX);
                    details.push(format!("node {n} has no tally: \"{line}\""));
                    continue;
                }
                Err(_) => {
                    checked += u64::try_from(honest.len()).unwrap_or(u64::MAX);
                    missing += u64::try_from(honest.len()).unwrap_or(u64::MAX);
                    details.push(format!("node {n} never held the vote"));
                    continue;
                }
            };
            for &caster in &honest {
                let Some(key) = input.ids[caster] else {
                    continue;
                };
                checked += 1;
                let not_heard = tally.not_heard.contains(&key);
                if not_heard || tally.void_keys.contains(&key) {
                    missing += 1;
                    if missing <= 3 {
                        details.push(format!(
                            "node {n} has not counted node {caster} ({}; its tally: heard {}, rejected {}, void {})",
                            if not_heard { "not heard" } else { "void" },
                            tally.heard,
                            tally.rejected,
                            tally.void_keys.len()
                        ));
                    }
                }
            }
        }
        if missing > 0 {
            ok = false;
        }
        details.push(format!(
            "{label}: {} honest ballots, {checked} (node, ballot) pairs checked, {missing} not counted; latest cast {}",
            casts.len(),
            latest
                .map(|(n, at)| format!("by node {n} at {at} ms"))
                .unwrap_or_default()
        ));
    }
    Criterion::check("late_ballot_counts", ok, details.join("; "))
}

/// Always on: nodes showing `Final` for the same vote agree on the winner.
fn final_agreement(input: &EvalInput<'_>) -> Criterion {
    if input.vote_ids.is_empty() {
        return Criterion::not_evaluated("invariant:no_final_disagreement", "no vote in this run");
    }
    let mut ok = true;
    let mut details = Vec::new();
    for (label, vid) in input.vote_ids {
        let lines = vote_lines_at_end(input, vid);
        let winners = distinct_final_winners(&lines);
        if winners > 1 {
            ok = false;
        }
        let mut counts: BTreeMap<&str, u64> = BTreeMap::new();
        for (_, l) in lines.values() {
            *counts.entry(l.as_str()).or_insert(0) += 1;
        }
        details.push(format!(
            "{label}: {winners} distinct Final winners; lines at the end {counts:?}"
        ));
    }
    Criterion::check("invariant:no_final_disagreement", ok, details.join("; "))
}

/// Always on: every store is within the buffer caps (DECISIONS D4 BUFFER).
fn bounded_buffers(input: &EvalInput<'_>) -> Criterion {
    let policy = BufferPolicy::default();
    let cap_cells = policy.total_cap_bytes / CELL_LEN;
    let mut worst_bytes = 0usize;
    let mut worst_cells = 0usize;
    let mut over = 0usize;
    for n in input.nodes {
        let b = n.store_bytes();
        let c = n.store_len();
        worst_bytes = worst_bytes.max(b);
        worst_cells = worst_cells.max(c);
        if b > policy.total_cap_bytes || c > cap_cells {
            over += 1;
        }
    }
    Criterion::check(
        "invariant:bounded_buffers",
        over == 0,
        format!(
            "largest store {worst_cells} cells / {worst_bytes} bytes (cap {} cells / {} bytes); {over} nodes over the cap",
            cap_cells, policy.total_cap_bytes
        ),
    )
}

/// S-flood: junk held at any honest node stays within the aggregate L3
/// admission of its links over the flood window (BRIEF_v2 §5.5: 25 % of
/// the text rate per link, doubled for L2 links to verified contacts),
/// and no honest node refused a cell for lack of room.
fn junk_within_cap(input: &EvalInput<'_>, f: &FloodInfo<'_>) -> Criterion {
    let minutes = input
        .duration_ms
        .saturating_sub(f.start_ms)
        .div_ceil(60_000)
        .max(1);
    let mut worst = 0usize;
    let mut worst_node = 0usize;
    let mut over = 0usize;
    let mut no_room = 0u64;
    for (i, n) in input.nodes.iter().enumerate() {
        if i == f.flooder {
            continue;
        }
        let slots = u64::from(match n.kind() {
            NodeKind::IPhone => crate::simnode::IPHONE_SLOTS,
            _ => input.scenario.nodes.android_slots,
        });
        let bound = slots
            .saturating_mul(minutes)
            .saturating_mul(TEXT_CELLS_PER_MIN_ASM)
            .saturating_mul(Lane::L3_SHARE_PERMILLE_ASM)
            .saturating_mul(L2_QUOTA_MULT_ASM)
            / 1000;
        let held = f.junk_ids.iter().filter(|id| n.holds(id)).count();
        if held > worst {
            worst = held;
            worst_node = i;
        }
        if u64::try_from(held).unwrap_or(u64::MAX) > bound {
            over += 1;
        }
        no_room += n.stats().no_room;
    }
    let injected = f.junk_ids.len();
    Criterion::check(
        "flood_junk_within_l3_cap",
        over == 0 && no_room == 0,
        format!(
            "{injected} junk cells injected; most held by an honest node: {worst} (node {worst_node}); {over} nodes above their aggregate L3 admission over {minutes} min; honest no-room refusals {no_room}"
        ),
    )
}

/// S-flood "penalised link is throttled": the honest side throttles the
/// flooder (BRIEF_v2 §5.5); a score penalty for a peer that ignores
/// `Throttle` has no path in `relay::limits` (penalties exist for bad
/// signatures, never-members and removed-in-heads only), so the scores
/// are reported and the criterion is not evaluated.
fn flood_penalty(input: &EvalInput<'_>, f: &FloodInfo<'_>) -> Criterion {
    let throttled: u64 = input
        .nodes
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != f.flooder)
        .map(|(_, n)| n.stats().throttled_in)
        .sum();
    let min_score = f.link_scores.iter().copied().fold(f32::INFINITY, f32::min);
    Criterion::not_evaluated(
        "flood_link_penalised",
        format!(
            "honest nodes sent {throttled} Throttle frames; {} live links to the flooder at the end with lowest score {}; relay::limits has no penalty path for a peer that ignores Throttle (Penalty::Overflow is never applied by the node), so the score cannot drop for flooding",
            f.link_scores.len(),
            if min_score.is_finite() {
                format!("{min_score:.2}")
            } else {
                "n/a".to_owned()
            }
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::parse;
    use crate::simnode::{NodeConfig, ObjKind};

    fn node(idx: usize) -> SimNode {
        SimNode::new(
            NodeConfig {
                idx,
                kind: NodeKind::Android,
                name: format!("Node {idx}"),
                slots: 7,
                battery_floor_ppm: 300_000,
                battery_start_ppm: 1_000_000,
                foreground_duty_ppm: 1_000_000,
                wall_offset_ms: 0,
                dedupe_retention_ms: 1_000_000,
            },
            idx as u64,
            0,
        )
        .unwrap()
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
[groups]
[[groups.list]]
name = "main"
members = "all"
admins = [0]
[traffic]
group_post_every_ms = 100
text_bytes = [40, 300]
[[partition]]
at_ms = 10000
fragments = [[0], [1]]
[[partition]]
at_ms = 50000
fragments = "all"
[pass]
{pass}
"#
        ))
        .unwrap()
    }

    /// The empty adversary/removal/vote/ballot sets of a plain run.
    type EmptySets = (
        BTreeSet<usize>,
        BTreeSet<usize>,
        BTreeMap<String, VoteId>,
        BTreeMap<String, Vec<(usize, u64)>>,
    );

    fn input<'a>(
        scn: &'a Scenario,
        resolved: &'a Resolved,
        nodes: &'a [SimNode],
        trace: &'a Trace,
        ids: &'a [Option<IdentityKey>],
        empty_sets: &'a EmptySets,
        group_ids: &'a [Option<GroupId>],
    ) -> EvalInput<'a> {
        EvalInput {
            scenario: scn,
            resolved,
            nodes,
            trace,
            rejoin_at: Some(50_000),
            split_at: Some(10_000),
            duration_ms: 100_000,
            group_ids,
            vote_ids: &empty_sets.2,
            vote_snapshots: &[],
            ids,
            adversaries: &empty_sets.0,
            removal_subjects: &empty_sets.1,
            puppets: &[],
            replay_offers: 0,
            flood: None,
            moved: &empty_sets.0,
            ballots_cast: &empty_sets.3,
            equivocators: &empty_sets.0,
        }
    }

    #[test]
    fn delivery_fails_on_a_miss_and_lists_it() {
        let scn = scenario(
            "every_post_decrypts_after_rejoin = true\nno_duplicate_ui_delivery = true\nzero_wall_clock_reads = true\ndrain_report = true\nmembers_h_identical = true",
        );
        let resolved = scn.resolve().unwrap();
        let nodes = vec![node(0), node(1)];
        let ids: Vec<Option<IdentityKey>> = nodes.iter().map(SimNode::identity).collect();
        let mut trace = Trace::new(2);
        let early = Hash32::ZERO;
        trace.registry.register(
            early,
            0,
            1_000,
            0,
            0,
            ObjKind::Post { group: 0 },
            BTreeSet::from([1]),
            false,
            false,
        );
        // a late object is outside the grace window and not evaluated
        let late = meshcore::crypto::hash::sha256(b"late");
        trace.registry.register(
            late,
            1,
            90_000,
            2,
            0,
            ObjKind::Post { group: 0 },
            BTreeSet::from([0]),
            false,
            false,
        );
        let empty = Default::default();
        let group_ids = vec![None];
        let ev = evaluate(&input(
            &scn, &resolved, &nodes, &trace, &ids, &empty, &group_ids,
        ));
        assert_eq!(ev.delivery.evaluated_cells, 1);
        assert_eq!(ev.delivery.miss_count, 1);
        assert_eq!(ev.delivery.ratio, 0.0);
        assert_eq!(ev.delivery.misses[0].msg_id, early.to_string());
        assert_eq!(ev.delivery.misses[0].missing_at, vec![1]);
        let by_name = |n: &str| ev.criteria.iter().find(|c| c.name == n).unwrap().status;
        assert_eq!(by_name("every_post_decrypts_after_rejoin"), Status::Fail);
        assert_eq!(by_name("no_duplicate_ui_delivery"), Status::Pass);
        assert_eq!(by_name("zero_wall_clock_reads"), Status::Pass);
        assert_eq!(
            by_name("members_h_identical"),
            Status::Fail,
            "the group was never created"
        );
        assert_eq!(by_name("drain_report"), Status::NotEvaluated);
        assert_eq!(by_name("invariant:bounded_buffers"), Status::Pass);
        assert_eq!(
            by_name("invariant:no_final_disagreement"),
            Status::NotEvaluated
        );
        assert_eq!(ev.drain_ms, None, "the early object never reached node 1");
    }

    #[test]
    fn drain_is_measured_from_the_rejoin_and_duplicates_fail() {
        let scn = scenario(
            "zero_wall_clock_reads = true\ndrain_report = true\nno_duplicate_ui_delivery = true",
        );
        let resolved = scn.resolve().unwrap();
        let nodes = vec![node(0), node(1)];
        let ids: Vec<Option<IdentityKey>> = nodes.iter().map(SimNode::identity).collect();
        let mut trace = Trace::new(2);
        let id = Hash32::ZERO;
        trace.registry.register(
            id,
            0,
            1_000,
            0,
            0,
            ObjKind::Post { group: 0 },
            BTreeSet::from([1]),
            false,
            false,
        );
        trace.registry.record_shown(&id, 1, 70_000, &[]);
        let _ = meshcore::clock::Clock::wall_for_display(&nodes[1].clock);
        let empty = Default::default();
        let group_ids = vec![None];
        let ev = evaluate(&input(
            &scn, &resolved, &nodes, &trace, &ids, &empty, &group_ids,
        ));
        assert_eq!(ev.drain_ms, Some(20_000));
        let by_name = |n: &str| ev.criteria.iter().find(|c| c.name == n).unwrap().status;
        assert_eq!(by_name("zero_wall_clock_reads"), Status::Fail);
        assert_eq!(by_name("no_duplicate_ui_delivery"), Status::Pass);
        // a second show is a violation
        trace.registry.record_shown(&id, 1, 71_000, &[]);
        let ev = evaluate(&input(
            &scn, &resolved, &nodes, &trace, &ids, &empty, &group_ids,
        ));
        let by_name = |n: &str| ev.criteria.iter().find(|c| c.name == n).unwrap().status;
        assert_eq!(by_name("no_duplicate_ui_delivery"), Status::Fail);
        assert_eq!(by_name("invariant:no_duplicate_ui_delivery"), Status::Fail);
    }

    #[test]
    fn expected_vote_line_matches_the_tally_line_format() {
        let v = ExpectedVoteState {
            vote: "v".into(),
            at_ms: 1,
            state: "Provisional".into(),
            tally: "Y10 N10".into(),
            void: 1,
            winner: None,
        };
        assert_eq!(expected_vote_line(&v), "Y10 N10 · 1 void · Provisional");
        let v = ExpectedVoteState {
            void: 0,
            state: "Final".into(),
            ..v
        };
        assert_eq!(expected_vote_line(&v), "Y10 N10 · Final");
    }

    #[test]
    fn latency_is_the_mean_over_shown_pairs() {
        let mut reg = ObjRegistry::new(3);
        let a = Hash32::ZERO;
        reg.register(
            a,
            0,
            1_000,
            0,
            0,
            ObjKind::Post { group: 0 },
            BTreeSet::from([1, 2]),
            false,
            false,
        );
        reg.record_shown(&a, 1, 2_000, &[]);
        reg.record_shown(&a, 2, 4_000, &[]);
        assert_eq!(honest_latency_ms(&reg, 0, 100_000, None), Some(2_000.0));
        assert_eq!(honest_latency_ms(&reg, 0, 100_000, Some(0)), None);
        assert_eq!(honest_latency_ms(&reg, 5_000, 100_000, None), None);
    }

    #[test]
    fn late_ballots_fail_when_a_member_node_never_held_the_vote() {
        // S4 (d): a ballot "counts" only where the vote exists; a node that
        // never received the opening must not pass by being skipped.
        let scn = parse(
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
[groups]
[[groups.list]]
name = "main"
members = "all"
admins = [0]
[traffic]
group_post_every_ms = 100
text_bytes = [40, 300]
[[events]]
at_ms = 1
kind = "vote_open"
by = 0
label = "v"
question = "q"
options = ["a", "b"]
f = 1
[pass]
late_ballot_counts = true
"#,
        )
        .unwrap();
        let resolved = scn.resolve().unwrap();
        let nodes = vec![node(0), node(1)];
        let ids: Vec<Option<IdentityKey>> = nodes.iter().map(SimNode::identity).collect();
        let trace = Trace::new(2);
        let vote_ids = BTreeMap::from([("v".to_owned(), Hash32::ZERO)]);
        let ballots_cast = BTreeMap::from([("v".to_owned(), vec![(1usize, 5_000u64)])]);
        let empty: EmptySets = Default::default();
        let group_ids = vec![None];
        let input = EvalInput {
            scenario: &scn,
            resolved: &resolved,
            nodes: &nodes,
            trace: &trace,
            rejoin_at: None,
            split_at: None,
            duration_ms: 100_000,
            group_ids: &group_ids,
            vote_ids: &vote_ids,
            vote_snapshots: &[],
            ids: &ids,
            adversaries: &empty.0,
            removal_subjects: &empty.1,
            puppets: &[],
            replay_offers: 0,
            flood: None,
            moved: &empty.0,
            ballots_cast: &ballots_cast,
            equivocators: &empty.0,
        };
        let ev = evaluate(&input);
        let c = ev
            .criteria
            .iter()
            .find(|c| c.name == "late_ballot_counts")
            .unwrap();
        assert_eq!(c.status, Status::Fail, "{}", c.detail);
        assert!(c.detail.contains("never held the vote"), "{}", c.detail);
    }

    #[test]
    fn final_agreement_is_about_the_winner_not_the_displayed_counts() {
        // `final_never_changes` doc: "the displayed counts may still grow as
        // late ballots arrive"; two nodes Final for the same option with
        // different counts agree, two Final for different options do not.
        let same = BTreeMap::from([
            (
                0usize,
                (
                    Some(VoteState::Final { option: 0 }),
                    "Y14 N7 · Final".to_owned(),
                ),
            ),
            (
                1usize,
                (
                    Some(VoteState::Final { option: 0 }),
                    "Y13 N7 · Final".to_owned(),
                ),
            ),
            (2usize, (None, "Y13 N7 · Provisional".to_owned())),
        ]);
        assert_eq!(distinct_final_winners(&same), 1);
        let split = BTreeMap::from([
            (
                0usize,
                (
                    Some(VoteState::Final { option: 0 }),
                    "Y14 N7 · Final".to_owned(),
                ),
            ),
            (
                1usize,
                (
                    Some(VoteState::Final { option: 1 }),
                    "Y7 N14 · Final".to_owned(),
                ),
            ),
        ]);
        assert_eq!(distinct_final_winners(&split), 2);
        assert_eq!(distinct_final_winners(&BTreeMap::new()), 0);
    }

    #[test]
    fn removed_posts_shown_after_the_node_knew_must_be_labelled_whenever_made() {
        // BRIEF_v2 §5.4 rotation: "any other is kept, collapsed and
        // labelled" — the label follows the viewer's knowledge at show time,
        // not the post's creation time, so a post made before the removal
        // and shown after the node applied it is labelled too.
        let scn = scenario("removed_posts_labelled_or_rejected = true");
        let resolved = scn.resolve().unwrap();
        let nodes = vec![node(0), node(1)];
        let ids: Vec<Option<IdentityKey>> = nodes.iter().map(SimNode::identity).collect();
        let key1 = ids[1].unwrap();
        let mut trace = Trace::new(2);
        let post = Hash32::ZERO;
        trace.registry.register(
            post,
            1,
            500,
            0,
            0,
            ObjKind::Post { group: 0 },
            BTreeSet::from([0]),
            false,
            false,
        );
        // node 0 applied the removal of node 1 at 2000 ms, then showed the
        // post at 5000 ms without a label
        trace.removal_applied.insert((0, key1), 2_000);
        trace.registry.record_shown(&post, 0, 5_000, &[]);
        let subjects = BTreeSet::from([1usize]);
        let empty: EmptySets = Default::default();
        let group_ids = vec![None];
        let mut inp = input(&scn, &resolved, &nodes, &trace, &ids, &empty, &group_ids);
        inp.removal_subjects = &subjects;
        let ev = evaluate(&inp);
        let c = ev
            .criteria
            .iter()
            .find(|c| c.name == "removed_posts_labelled_or_rejected")
            .unwrap();
        assert_eq!(c.status, Status::Fail, "{}", c.detail);
        assert!(c.detail.contains("1 unlabelled"), "{}", c.detail);
        // labelled: passes
        let mut trace2 = Trace::new(2);
        trace2.registry.register(
            post,
            1,
            500,
            0,
            0,
            ObjKind::Post { group: 0 },
            BTreeSet::from([0]),
            false,
            false,
        );
        trace2.removal_applied.insert((0, key1), 2_000);
        trace2.registry.record_shown(
            &post,
            0,
            5_000,
            &["Sent by Node 1, who was removed".to_owned()],
        );
        let mut inp = input(&scn, &resolved, &nodes, &trace2, &ids, &empty, &group_ids);
        inp.removal_subjects = &subjects;
        let ev = evaluate(&inp);
        let c = ev
            .criteria
            .iter()
            .find(|c| c.name == "removed_posts_labelled_or_rejected")
            .unwrap();
        assert_eq!(c.status, Status::Pass, "{}", c.detail);
        assert!(
            c.detail.contains("1 labelled, 0 unlabelled"),
            "{}",
            c.detail
        );
    }

    #[test]
    fn members_identical_needs_every_member_node_to_hold_one_converged_roster() {
        // S1 (c) "members(H) identical on all 40 for the common head": a
        // member node without the roster, or two head sets at the end of a
        // run that has been unpartitioned long enough, is a failure of the
        // criterion (the per-head-set invariant still holds).
        let scn = scenario("members_h_identical = true");
        let resolved = scn.resolve().unwrap();
        let mut nodes = vec![node(0), node(1)];
        let card1 = nodes[1].node().my_keycard().unwrap();
        let gid = nodes[0]
            .node_mut()
            .create_group(
                "main",
                &[],
                &[meshcore::objects::roster_op::MemberEntry {
                    keycard: card1,
                    name: "Node 1".into(),
                    in_person: true,
                }],
                0,
                0,
            )
            .unwrap();
        let ids: Vec<Option<IdentityKey>> = nodes.iter().map(SimNode::identity).collect();
        let trace = Trace::new(2);
        let empty: EmptySets = Default::default();
        let group_ids = vec![Some(gid)];
        let ev = evaluate(&input(
            &scn, &resolved, &nodes, &trace, &ids, &empty, &group_ids,
        ));
        let by_name = |n: &str| ev.criteria.iter().find(|c| c.name == n).unwrap().clone();
        let c = by_name("members_h_identical");
        assert_eq!(c.status, Status::Fail, "{}", c.detail);
        assert!(
            c.detail.contains("1 nodes without the roster"),
            "{}",
            c.detail
        );
        assert_eq!(
            by_name("invariant:members_h_identical_per_head_set").status,
            Status::Pass
        );
        // the pure verdict: two head sets after a long-enough rejoin fail
        // the criterion but not the invariant; before the grace they are fine
        let a = Hash32::ZERO;
        let b = meshcore::crypto::hash::sha256(b"b");
        let views = vec![(0usize, Some((a, vec![]))), (1usize, Some((b, vec![])))];
        let (inv, crit, detail) = members_verdict("main", 2, &views, true);
        assert!(inv && !crit, "{detail}");
        assert!(detail.contains("2 head set(s)"), "{detail}");
        let (inv, crit, _) = members_verdict("main", 2, &views, false);
        assert!(inv && crit);
        // the same head set with different members(H) breaks both
        let views = vec![
            (0usize, Some((a, vec![]))),
            (1usize, Some((a, vec![ids[0].unwrap()]))),
        ];
        let (inv, crit, detail) = members_verdict("main", 2, &views, false);
        assert!(!inv && !crit, "{detail}");
    }
}
