//! Scenario files (docs/scenarios.md "TOML schema"): the serde model of
//! every table, the index-set spellings (`"all"`, `"a..b"`, lists) and the
//! validation that turns a file into a [`Resolved`] scenario the engine can
//! run. Every table is `deny_unknown_fields`, so a typo in a scenario is an
//! error rather than a silently ignored knob (DESIGN §10 "each run with
//! ≥20 seeds": a mis-spelled criterion must not pass by omission).
//!
//! Nothing here reads a clock; a scenario is pure data.

use std::collections::BTreeSet;
use std::path::Path;

use serde::Deserialize;

/// Why a scenario file is unusable.
#[derive(Debug, thiserror::Error)]
pub enum ScenarioError {
    #[error("cannot read {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("scenario parse error: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("scenario invalid: {0}")]
    Invalid(String),
}

fn invalid(msg: impl Into<String>) -> ScenarioError {
    ScenarioError::Invalid(msg.into())
}

// ---- raw tables -----------------------------------------------------------------

/// The whole file (docs/scenarios.md).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub scenario: Meta,
    pub nodes: Nodes,
    pub radio: Radio,
    #[serde(default)]
    pub groups: Groups,
    #[serde(default)]
    pub pairs: Pairs,
    pub traffic: Traffic,
    #[serde(default)]
    pub partition: Vec<Partition>,
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(default)]
    pub pass: Pass,
}

/// `[scenario]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    pub name: String,
    #[serde(default)]
    pub description: String,
    /// Total simulated time.
    pub duration_ms: u64,
    /// Node tick period.
    pub tick_ms: u64,
}

/// `[nodes]`: indices `0..android` are Android, then iPhones, then keyless
/// stations (BRIEF_v2 §5.7).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Nodes {
    pub android: usize,
    pub iphone: usize,
    #[serde(default)]
    pub stations: usize,
    /// Fraction of time an iPhone is in the foreground (assumption,
    /// docs/testing.md).
    pub iphone_foreground_duty: f64,
    /// Concurrent GATT links per Android node (DECISIONS D4 Q4/Q9).
    pub android_slots: u32,
    /// Below this fraction a phone stops relaying for others (BRIEF_v2 §5.7).
    pub battery_floor: f64,
}

/// `[radio]`: all assumptions (docs/testing.md "Assumptions baked into the
/// simulator"; DESIGN §7 `SimLink` row).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Radio {
    pub latency_ms: u64,
    pub throughput_bytes_per_s: u64,
    /// Per Noise frame; a loss drops the link, which re-handshakes.
    pub frame_loss: f64,
    /// A locked iPhone stays awake this long per inbound connect
    /// (DECISIONS D10 Q1–Q4).
    pub iphone_wake_ms: u64,
    /// Per-node uniform wall-clock offset range, display only (scenario S4).
    pub wall_clock_offset_ms: [i64; 2],
}

/// `[groups]`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Groups {
    #[serde(default)]
    pub list: Vec<GroupDef>,
}

/// `[[groups.list]]`: created at t=0 by its first admin with everyone
/// pre-verified in person.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupDef {
    pub name: String,
    pub members: NodeSet,
    pub admins: Vec<usize>,
}

/// `[pairs]`: QR-verified 1:1 contacts at t=0.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pairs {
    /// Number of random pairs.
    #[serde(default)]
    pub random: usize,
    /// Explicit pairs.
    #[serde(default)]
    pub explicit: Vec<[usize; 2]>,
}

/// `[traffic]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Traffic {
    /// Each member posts to each of its groups at this period (jittered).
    pub group_post_every_ms: u64,
    /// Each verified pair exchanges a DM at this period (0 = none).
    #[serde(default)]
    pub dm_every_ms: u64,
    /// Uniform text size range in bytes.
    pub text_bytes: [u32; 2],
}

/// `[[partition]]`: the connectivity schedule.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Partition {
    pub at_ms: u64,
    pub fragments: Fragments,
}

/// `fragments = "all"` or a list of fragments, each a list of indices or
/// `"a..b"` ranges (inclusive, as in `"0..17"`, `"18..31"`, `"32..39"`).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Fragments {
    All(String),
    List(Vec<Vec<IndexSpec>>),
}

/// A set of node indices: `"all"`, one `"a..b"` range, or a list of indices
/// and ranges.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum NodeSet {
    Spec(String),
    List(Vec<IndexSpec>),
}

/// One index or one inclusive `"a..b"` range.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum IndexSpec {
    Index(usize),
    Range(String),
}

/// One node index or a list of them (`by` of a ballot event).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany {
    One(usize),
    Many(Vec<usize>),
}

impl OneOrMany {
    pub fn to_vec(&self) -> Vec<usize> {
        match self {
            OneOrMany::One(n) => vec![*n],
            OneOrMany::Many(v) => v.clone(),
        }
    }
}

/// Adversary behaviours (docs/scenarios.md `[[events]]` comment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Behaviour {
    KeepKeysAfterRemoval,
    EquivocateBallot,
    SockpuppetAdds,
    WithholdDms,
    ReplayCells,
    Flood,
}

/// `[[events]]`: timed protocol actions, keyed by `kind`.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Event {
    RemoveMember {
        at_ms: u64,
        by: usize,
        subject: usize,
    },
    AddMember {
        at_ms: u64,
        by: usize,
        subject: usize,
    },
    TaskCreate {
        at_ms: u64,
        by: usize,
        task: String,
    },
    TaskTake {
        at_ms: u64,
        by: usize,
        task: String,
    },
    TaskRelease {
        at_ms: u64,
        by: usize,
        task: String,
    },
    TaskDone {
        at_ms: u64,
        by: usize,
        task: String,
    },
    VoteOpen {
        at_ms: u64,
        by: usize,
        label: String,
        /// Read by the votes wave.
        #[allow(dead_code)]
        question: String,
        options: Vec<String>,
        f: u32,
    },
    Ballot {
        at_ms: u64,
        by: OneOrMany,
        vote: String,
        choice: usize,
        /// The ballots of `by` are spread uniformly over this window (read
        /// by the votes wave).
        #[serde(default)]
        #[allow(dead_code)]
        spread_ms: u64,
    },
    Adversary {
        at_ms: u64,
        node: usize,
        behaviour: Behaviour,
        /// `sockpuppet_adds`.
        count: Option<u32>,
        /// `withhold_dms`.
        hold_ms: Option<u64>,
        /// `flood`.
        cells_per_min: Option<u32>,
        /// `equivocate_ballot`.
        vote: Option<String>,
        choices: Option<Vec<usize>>,
        carry_to_fragment: Option<usize>,
        carry_at_ms: Option<u64>,
    },
}

impl Event {
    pub fn at_ms(&self) -> u64 {
        match self {
            Event::RemoveMember { at_ms, .. }
            | Event::AddMember { at_ms, .. }
            | Event::TaskCreate { at_ms, .. }
            | Event::TaskTake { at_ms, .. }
            | Event::TaskRelease { at_ms, .. }
            | Event::TaskDone { at_ms, .. }
            | Event::VoteOpen { at_ms, .. }
            | Event::Ballot { at_ms, .. }
            | Event::Adversary { at_ms, .. } => *at_ms,
        }
    }

    /// The `kind` string of the file.
    pub fn kind_name(&self) -> &'static str {
        match self {
            Event::RemoveMember { .. } => "remove_member",
            Event::AddMember { .. } => "add_member",
            Event::TaskCreate { .. } => "task_create",
            Event::TaskTake { .. } => "task_take",
            Event::TaskRelease { .. } => "task_release",
            Event::TaskDone { .. } => "task_done",
            Event::VoteOpen { .. } => "vote_open",
            Event::Ballot { .. } => "ballot",
            Event::Adversary { .. } => "adversary",
        }
    }
}

/// One entry of `expected_vote_states` (evaluated by the votes wave).
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedVoteState {
    pub vote: String,
    pub at_ms: u64,
    pub state: String,
    pub tally: String,
    #[serde(default)]
    pub void: u32,
    #[serde(default)]
    pub winner: Option<String>,
}

/// `[pass]`: which criteria the invariant checker enforces. Absent keys are
/// off, so a scenario only ever opts *in* to a check.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Pass {
    pub every_post_decrypts_after_rejoin: bool,
    pub members_h_identical: bool,
    pub no_solid_state_reverts: bool,
    pub no_duplicate_ui_delivery: bool,
    pub message_keys_deleted_after_delivery: bool,
    pub removed_cannot_decrypt_post_removal: bool,
    pub sockpuppet_adds_void_everywhere: bool,
    pub removed_posts_labelled_or_rejected: bool,
    pub replays_never_redelivered: bool,
    pub no_final_while_split: bool,
    pub expected_vote_states: Vec<ExpectedVoteState>,
    pub final_never_changes: bool,
    pub late_ballot_counts: bool,
    pub zero_wall_clock_reads: bool,
    /// 0 = not checked.
    pub flood_honest_latency_factor_max: f64,
    pub flood_junk_within_l3_cap: bool,
    pub drain_report: bool,
}

// ---- resolved form -----------------------------------------------------------------

/// What kind of phone a node is (`[nodes]` order: Android, iPhone, station).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeKind {
    Android,
    IPhone,
    /// Always-on Android without identity (BRIEF_v2 §5.7 keyless station).
    Station,
}

impl NodeKind {
    pub fn name(self) -> &'static str {
        match self {
            NodeKind::Android => "android",
            NodeKind::IPhone => "iphone",
            NodeKind::Station => "station",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedGroup {
    pub name: String,
    pub members: BTreeSet<usize>,
    pub admins: BTreeSet<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPartition {
    pub at_ms: u64,
    /// Fragment index per node.
    pub fragment_of: Vec<usize>,
    pub fragment_count: usize,
}

/// The validated, index-resolved scenario.
#[derive(Debug, Clone)]
pub struct Resolved {
    pub n_nodes: usize,
    pub kinds: Vec<NodeKind>,
    pub groups: Vec<ResolvedGroup>,
    /// Explicit pairs; random ones are drawn by the engine from the seed.
    pub explicit_pairs: Vec<(usize, usize)>,
    pub random_pairs: usize,
    /// Sorted by `at_ms`; the first is at 0 ("all" when the file starts
    /// later).
    pub partitions: Vec<ResolvedPartition>,
}

impl Resolved {
    /// The partition in force at `t_ms`.
    pub fn partition_at(&self, t_ms: u64) -> &ResolvedPartition {
        let mut cur = &self.partitions[0];
        for p in &self.partitions {
            if p.at_ms <= t_ms {
                cur = p;
            }
        }
        cur
    }
}

// ---- parsing helpers ----------------------------------------------------------------

/// Parses `"a..b"` (inclusive) into its bounds.
fn parse_range(s: &str) -> Result<(usize, usize), ScenarioError> {
    let (a, b) = s
        .split_once("..")
        .ok_or_else(|| invalid(format!("bad index range {s:?} (want \"a..b\" or \"all\")")))?;
    let a: usize = a
        .trim()
        .parse()
        .map_err(|_| invalid(format!("bad range start in {s:?}")))?;
    let b: usize = b
        .trim()
        .parse()
        .map_err(|_| invalid(format!("bad range end in {s:?}")))?;
    if a > b {
        return Err(invalid(format!("empty index range {s:?}")));
    }
    Ok((a, b))
}

impl IndexSpec {
    fn resolve(&self, n_nodes: usize, out: &mut BTreeSet<usize>) -> Result<(), ScenarioError> {
        match self {
            IndexSpec::Index(i) => {
                check_index(*i, n_nodes)?;
                out.insert(*i);
            }
            IndexSpec::Range(s) => {
                let (a, b) = parse_range(s)?;
                check_index(b, n_nodes)?;
                out.extend(a..=b);
            }
        }
        Ok(())
    }
}

impl NodeSet {
    /// The node indices this set names.
    pub fn resolve(&self, n_nodes: usize) -> Result<BTreeSet<usize>, ScenarioError> {
        let mut out = BTreeSet::new();
        match self {
            NodeSet::Spec(s) if s == "all" => out.extend(0..n_nodes),
            NodeSet::Spec(s) => IndexSpec::Range(s.clone()).resolve(n_nodes, &mut out)?,
            NodeSet::List(items) => {
                for item in items {
                    item.resolve(n_nodes, &mut out)?;
                }
            }
        }
        Ok(out)
    }
}

fn check_index(i: usize, n_nodes: usize) -> Result<(), ScenarioError> {
    if i >= n_nodes {
        return Err(invalid(format!(
            "node index {i} out of range (the scenario has {n_nodes} nodes)"
        )));
    }
    Ok(())
}

fn check_fraction(name: &str, v: f64) -> Result<(), ScenarioError> {
    if !(v.is_finite() && (0.0..=1.0).contains(&v)) {
        return Err(invalid(format!("{name} must be in [0, 1], got {v}")));
    }
    Ok(())
}

impl Partition {
    fn resolve(&self, n_nodes: usize) -> Result<ResolvedPartition, ScenarioError> {
        let mut fragment_of = vec![usize::MAX; n_nodes];
        let fragment_count = match &self.fragments {
            Fragments::All(s) if s == "all" => {
                fragment_of.iter_mut().for_each(|f| *f = 0);
                1
            }
            Fragments::All(s) => {
                return Err(invalid(format!(
                    "fragments must be \"all\" or a list of fragments, got {s:?}"
                )));
            }
            Fragments::List(frags) => {
                if frags.is_empty() {
                    return Err(invalid("a partition needs at least one fragment"));
                }
                for (fi, frag) in frags.iter().enumerate() {
                    let mut set = BTreeSet::new();
                    for item in frag {
                        item.resolve(n_nodes, &mut set)?;
                    }
                    for n in set {
                        if fragment_of[n] != usize::MAX {
                            return Err(invalid(format!(
                                "node {n} appears in two fragments of the partition at {} ms",
                                self.at_ms
                            )));
                        }
                        fragment_of[n] = fi;
                    }
                }
                frags.len()
            }
        };
        if let Some(n) = fragment_of.iter().position(|f| *f == usize::MAX) {
            return Err(invalid(format!(
                "node {n} is in no fragment of the partition at {} ms",
                self.at_ms
            )));
        }
        Ok(ResolvedPartition {
            at_ms: self.at_ms,
            fragment_of,
            fragment_count,
        })
    }
}

// ---- loading and validation ---------------------------------------------------------

/// Reads, parses and validates a scenario file.
pub fn load(path: impl AsRef<Path>) -> Result<Scenario, ScenarioError> {
    let path = path.as_ref();
    let text = std::fs::read_to_string(path).map_err(|source| ScenarioError::Io {
        path: path.display().to_string(),
        source,
    })?;
    parse(&text)
}

/// Parses and validates scenario TOML text.
pub fn parse(text: &str) -> Result<Scenario, ScenarioError> {
    let scenario: Scenario = toml::from_str(text)?;
    scenario.resolve()?;
    Ok(scenario)
}

impl Scenario {
    pub fn n_nodes(&self) -> usize {
        self.nodes
            .android
            .saturating_add(self.nodes.iphone)
            .saturating_add(self.nodes.stations)
    }

    /// The kind of node `i` (`[nodes]` ordering).
    pub fn kind_of(&self, i: usize) -> NodeKind {
        if i < self.nodes.android {
            NodeKind::Android
        } else if i < self.nodes.android + self.nodes.iphone {
            NodeKind::IPhone
        } else {
            NodeKind::Station
        }
    }

    /// Validates every table and resolves index sets. Errors name the
    /// first problem found.
    pub fn resolve(&self) -> Result<Resolved, ScenarioError> {
        let n_nodes = self.n_nodes();
        if n_nodes == 0 {
            return Err(invalid("the scenario has no nodes"));
        }
        if n_nodes > 100_000 {
            return Err(invalid("too many nodes"));
        }
        if self.scenario.name.is_empty() {
            return Err(invalid("scenario.name is empty"));
        }
        if self.scenario.duration_ms == 0 {
            return Err(invalid("scenario.duration_ms must be positive"));
        }
        if self.scenario.tick_ms == 0 {
            return Err(invalid("scenario.tick_ms must be positive"));
        }
        check_fraction(
            "nodes.iphone_foreground_duty",
            self.nodes.iphone_foreground_duty,
        )?;
        check_fraction("nodes.battery_floor", self.nodes.battery_floor)?;
        if self.nodes.android_slots == 0 {
            return Err(invalid("nodes.android_slots must be positive"));
        }
        check_fraction("radio.frame_loss", self.radio.frame_loss)?;
        if self.radio.throughput_bytes_per_s == 0 {
            return Err(invalid("radio.throughput_bytes_per_s must be positive"));
        }
        if self.radio.iphone_wake_ms == 0 {
            return Err(invalid("radio.iphone_wake_ms must be positive"));
        }
        if self.radio.wall_clock_offset_ms[0] > self.radio.wall_clock_offset_ms[1] {
            return Err(invalid(
                "radio.wall_clock_offset_ms must be [lo, hi] with lo <= hi",
            ));
        }
        if self.traffic.group_post_every_ms == 0 && !self.groups.list.is_empty() {
            return Err(invalid("traffic.group_post_every_ms must be positive"));
        }
        if self.traffic.text_bytes[0] > self.traffic.text_bytes[1] {
            return Err(invalid("traffic.text_bytes must be [lo, hi] with lo <= hi"));
        }
        if self.traffic.text_bytes[1] > 2000 {
            return Err(invalid("traffic.text_bytes above the 2000-byte text cap"));
        }

        let kinds: Vec<NodeKind> = (0..n_nodes).map(|i| self.kind_of(i)).collect();

        let mut groups = Vec::new();
        let mut names = BTreeSet::new();
        for g in &self.groups.list {
            if !names.insert(g.name.clone()) {
                return Err(invalid(format!("duplicate group name {:?}", g.name)));
            }
            let members = g.members.resolve(n_nodes)?;
            if members.is_empty() {
                return Err(invalid(format!("group {:?} has no members", g.name)));
            }
            let mut admins = BTreeSet::new();
            for a in &g.admins {
                check_index(*a, n_nodes)?;
                if !members.contains(a) {
                    return Err(invalid(format!(
                        "admin {a} of group {:?} is not a member",
                        g.name
                    )));
                }
                admins.insert(*a);
            }
            if admins.is_empty() {
                return Err(invalid(format!("group {:?} has no admins", g.name)));
            }
            for m in &members {
                if kinds[*m] == NodeKind::Station {
                    return Err(invalid(format!(
                        "station {m} cannot be a member of group {:?}",
                        g.name
                    )));
                }
            }
            groups.push(ResolvedGroup {
                name: g.name.clone(),
                members,
                admins,
            });
        }

        let mut explicit_pairs = Vec::new();
        let mut seen_pairs = BTreeSet::new();
        for [a, b] in &self.pairs.explicit {
            check_index(*a, n_nodes)?;
            check_index(*b, n_nodes)?;
            if a == b {
                return Err(invalid(format!("pair [{a}, {b}] pairs a node with itself")));
            }
            if kinds[*a] == NodeKind::Station || kinds[*b] == NodeKind::Station {
                return Err(invalid(format!("pair [{a}, {b}] includes a station")));
            }
            let key = (*a.min(b), *a.max(b));
            if !seen_pairs.insert(key) {
                return Err(invalid(format!("duplicate pair [{a}, {b}]")));
            }
            explicit_pairs.push(key);
        }
        let phones = self.nodes.android + self.nodes.iphone;
        let max_pairs = phones * phones.saturating_sub(1) / 2;
        if self.pairs.random + explicit_pairs.len() > max_pairs {
            return Err(invalid(format!(
                "{} random + {} explicit pairs exceed the {} possible pairs",
                self.pairs.random,
                explicit_pairs.len(),
                max_pairs
            )));
        }

        let mut partitions = Vec::new();
        for p in &self.partition {
            if p.at_ms > self.scenario.duration_ms {
                return Err(invalid(format!(
                    "partition at {} ms is after the end of the scenario",
                    p.at_ms
                )));
            }
            partitions.push(p.resolve(n_nodes)?);
        }
        partitions.sort_by_key(|p| p.at_ms);
        for w in partitions.windows(2) {
            if w[0].at_ms == w[1].at_ms {
                return Err(invalid(format!(
                    "two partitions at the same time {} ms",
                    w[0].at_ms
                )));
            }
        }
        if partitions.first().is_none_or(|p| p.at_ms != 0) {
            partitions.insert(
                0,
                ResolvedPartition {
                    at_ms: 0,
                    fragment_of: vec![0; n_nodes],
                    fragment_count: 1,
                },
            );
        }
        let max_fragments = partitions
            .iter()
            .map(|p| p.fragment_count)
            .max()
            .unwrap_or(1);

        self.validate_events(n_nodes, &kinds, max_fragments)?;

        for v in &self.pass.expected_vote_states {
            if !self
                .events
                .iter()
                .any(|e| matches!(e, Event::VoteOpen { label, .. } if *label == v.vote))
            {
                return Err(invalid(format!(
                    "pass.expected_vote_states names unknown vote {:?}",
                    v.vote
                )));
            }
        }
        if !(self.pass.flood_honest_latency_factor_max.is_finite()
            && self.pass.flood_honest_latency_factor_max >= 0.0)
        {
            return Err(invalid("pass.flood_honest_latency_factor_max must be >= 0"));
        }

        Ok(Resolved {
            n_nodes,
            kinds,
            groups,
            explicit_pairs,
            random_pairs: self.pairs.random,
            partitions,
        })
    }

    fn validate_events(
        &self,
        n_nodes: usize,
        kinds: &[NodeKind],
        max_fragments: usize,
    ) -> Result<(), ScenarioError> {
        let check_phone = |i: usize, what: &str| -> Result<(), ScenarioError> {
            check_index(i, n_nodes)?;
            if kinds[i] == NodeKind::Station {
                return Err(invalid(format!("{what} {i} is a station")));
            }
            Ok(())
        };
        let mut votes: BTreeSet<String> = BTreeSet::new();
        let mut tasks: BTreeSet<String> = BTreeSet::new();
        // Labels are known once opened, in file order of `at_ms`.
        let mut ordered: Vec<&Event> = self.events.iter().collect();
        ordered.sort_by_key(|e| e.at_ms());
        for e in ordered {
            if e.at_ms() > self.scenario.duration_ms {
                return Err(invalid(format!(
                    "event {} at {} ms is after the end of the scenario",
                    e.kind_name(),
                    e.at_ms()
                )));
            }
            match e {
                Event::RemoveMember { by, subject, .. } | Event::AddMember { by, subject, .. } => {
                    check_phone(*by, "event actor")?;
                    check_phone(*subject, "event subject")?;
                }
                Event::TaskCreate { by, task, .. } => {
                    check_phone(*by, "event actor")?;
                    if !tasks.insert(task.clone()) {
                        return Err(invalid(format!("task {task:?} created twice")));
                    }
                }
                Event::TaskTake { by, task, .. }
                | Event::TaskRelease { by, task, .. }
                | Event::TaskDone { by, task, .. } => {
                    check_phone(*by, "event actor")?;
                    if !tasks.contains(task) {
                        return Err(invalid(format!(
                            "{} references unknown task {task:?}",
                            e.kind_name()
                        )));
                    }
                }
                Event::VoteOpen {
                    by,
                    label,
                    options,
                    f,
                    ..
                } => {
                    check_phone(*by, "event actor")?;
                    if !(2..=8).contains(&options.len()) {
                        return Err(invalid(format!(
                            "vote {label:?} needs 2..=8 options (DESIGN §2.4)"
                        )));
                    }
                    if *f == 0 {
                        return Err(invalid(format!("vote {label:?} needs f >= 1")));
                    }
                    if !votes.insert(label.clone()) {
                        return Err(invalid(format!("vote {label:?} opened twice")));
                    }
                }
                Event::Ballot {
                    by, vote, choice, ..
                } => {
                    for b in by.to_vec() {
                        check_phone(b, "ballot caster")?;
                    }
                    let opened = self.events.iter().find_map(|o| match o {
                        Event::VoteOpen { label, options, .. } if label == vote => {
                            Some(options.len())
                        }
                        _ => None,
                    });
                    let Some(n_options) = opened else {
                        return Err(invalid(format!("ballot references unknown vote {vote:?}")));
                    };
                    if !votes.contains(vote) {
                        return Err(invalid(format!(
                            "ballot for {vote:?} is cast before the vote opens"
                        )));
                    }
                    if *choice >= n_options {
                        return Err(invalid(format!(
                            "ballot choice {choice} out of range for vote {vote:?}"
                        )));
                    }
                }
                Event::Adversary {
                    node,
                    behaviour,
                    count,
                    hold_ms,
                    cells_per_min,
                    vote,
                    choices,
                    carry_to_fragment,
                    carry_at_ms,
                    ..
                } => {
                    check_index(*node, n_nodes)?;
                    let need = |ok: bool, what: &str| -> Result<(), ScenarioError> {
                        if ok {
                            Ok(())
                        } else {
                            Err(invalid(format!(
                                "adversary {behaviour:?} on node {node} needs {what}"
                            )))
                        }
                    };
                    match behaviour {
                        Behaviour::KeepKeysAfterRemoval | Behaviour::ReplayCells => {}
                        Behaviour::SockpuppetAdds => need(count.is_some_and(|c| c > 0), "count")?,
                        Behaviour::WithholdDms => need(hold_ms.is_some_and(|h| h > 0), "hold_ms")?,
                        Behaviour::Flood => {
                            need(cells_per_min.is_some_and(|c| c > 0), "cells_per_min")?;
                        }
                        Behaviour::EquivocateBallot => {
                            let label = vote.as_ref();
                            need(label.is_some(), "vote")?;
                            if let Some(label) = label
                                && !votes.contains(label)
                            {
                                return Err(invalid(format!(
                                    "equivocate_ballot references unknown vote {label:?}"
                                )));
                            }
                            need(
                                choices.as_ref().is_some_and(|c| c.len() == 2),
                                "choices=[a,b]",
                            )?;
                            need(carry_at_ms.is_some(), "carry_at_ms")?;
                            let frag = carry_to_fragment;
                            need(frag.is_some(), "carry_to_fragment")?;
                            if let Some(frag) = frag
                                && *frag >= max_fragments
                            {
                                return Err(invalid(format!(
                                    "carry_to_fragment {frag} is beyond the {max_fragments} fragments"
                                )));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIO_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/scenarios");

    fn all_files() -> Vec<std::path::PathBuf> {
        let mut v: Vec<_> = std::fs::read_dir(SCENARIO_DIR)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "toml"))
            .collect();
        v.sort();
        v
    }

    #[test]
    fn all_seven_scenario_files_load() {
        let files = all_files();
        assert_eq!(files.len(), 7, "{files:?}");
        for f in files {
            let s = load(&f).unwrap_or_else(|e| panic!("{}: {e}", f.display()));
            let r = s.resolve().unwrap();
            assert_eq!(r.n_nodes, s.n_nodes());
            assert_eq!(r.partitions[0].at_ms, 0);
        }
    }

    #[test]
    fn s1_resolves_fragments_and_group() {
        let s = load(format!("{SCENARIO_DIR}/s1.toml")).unwrap();
        let r = s.resolve().unwrap();
        assert_eq!(r.n_nodes, 40);
        assert_eq!(r.kinds[0], NodeKind::Android);
        assert_eq!(r.kinds[20], NodeKind::IPhone);
        assert_eq!(r.partitions.len(), 3);
        let split = &r.partitions[1];
        assert_eq!(split.fragment_count, 3);
        assert_eq!(split.fragment_of[17], 0);
        assert_eq!(split.fragment_of[18], 1);
        assert_eq!(split.fragment_of[31], 1);
        assert_eq!(split.fragment_of[32], 2);
        assert_eq!(r.partition_at(599_999).fragment_count, 1);
        assert_eq!(r.partition_at(600_000).fragment_count, 3);
        assert_eq!(r.partition_at(22_200_000).fragment_count, 1);
        assert_eq!(r.groups[0].members.len(), 40);
        assert_eq!(r.random_pairs, 10);
    }

    #[test]
    fn s_flood_range_members_and_flood_event() {
        let s = load(format!("{SCENARIO_DIR}/s-flood.toml")).unwrap();
        let r = s.resolve().unwrap();
        assert_eq!(r.groups[0].members.len(), 39);
        assert!(!r.groups[0].members.contains(&39));
        assert!(matches!(
            s.events[0],
            Event::Adversary {
                behaviour: Behaviour::Flood,
                cells_per_min: Some(600),
                ..
            }
        ));
    }

    fn minimal(extra: &str) -> String {
        format!(
            r#"
[scenario]
name = "t"
duration_ms = 1000
tick_ms = 100
[nodes]
android = 3
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
{extra}
"#
        )
    }

    #[test]
    fn unknown_fields_are_errors() {
        let err = parse(&minimal("[pass]\nevery_post_decrypt = true")).unwrap_err();
        assert!(matches!(err, ScenarioError::Parse(_)), "{err}");
        let err = parse(&minimal(
            "[[events]]\nat_ms = 1\nkind = \"task_create\"\nby = 0\ntask = \"x\"\nbogus = 1",
        ))
        .unwrap_err();
        assert!(matches!(err, ScenarioError::Parse(_)), "{err}");
    }

    #[test]
    fn indices_must_be_in_range() {
        let err = parse(&minimal(
            "[groups]\n[[groups.list]]\nname = \"g\"\nmembers = \"0..5\"\nadmins = [0]",
        ))
        .unwrap_err();
        assert!(matches!(err, ScenarioError::Invalid(_)), "{err}");
        let err = parse(&minimal("[pairs]\nexplicit = [[0, 3]]")).unwrap_err();
        assert!(matches!(err, ScenarioError::Invalid(_)), "{err}");
        let err = parse(&minimal(
            "[[partition]]\nat_ms = 0\nfragments = [[\"0..1\"]]",
        ))
        .unwrap_err();
        assert!(format!("{err}").contains("in no fragment"), "{err}");
        let err = parse(&minimal(
            "[[partition]]\nat_ms = 0\nfragments = [[\"0..1\"], [1, 2]]",
        ))
        .unwrap_err();
        assert!(format!("{err}").contains("two fragments"), "{err}");
    }

    #[test]
    fn events_must_reference_known_labels() {
        let err = parse(&minimal(
            "[[events]]\nat_ms = 1\nkind = \"ballot\"\nby = 0\nvote = \"v\"\nchoice = 0",
        ))
        .unwrap_err();
        assert!(format!("{err}").contains("unknown vote"), "{err}");
        let err = parse(&minimal(
            "[[events]]\nat_ms = 1\nkind = \"task_take\"\nby = 0\ntask = \"w\"",
        ))
        .unwrap_err();
        assert!(format!("{err}").contains("unknown task"), "{err}");
        let ok = parse(&minimal(
            "[[events]]\nat_ms = 1\nkind = \"vote_open\"\nby = 0\nlabel = \"v\"\nquestion = \"q\"\noptions = [\"a\",\"b\"]\nf = 1\n[[events]]\nat_ms = 2\nkind = \"ballot\"\nby = [0, 1]\nvote = \"v\"\nchoice = 1",
        ))
        .unwrap();
        assert_eq!(ok.events.len(), 2);
    }

    #[test]
    fn missing_partition_defaults_to_all_at_zero() {
        let s = parse(&minimal("")).unwrap();
        let r = s.resolve().unwrap();
        assert_eq!(r.partitions.len(), 1);
        assert_eq!(r.partitions[0].fragment_of, vec![0, 0, 0]);
        let s = parse(&minimal(
            "[[partition]]\nat_ms = 500\nfragments = [[0], [\"1..2\"]]",
        ))
        .unwrap();
        let r = s.resolve().unwrap();
        assert_eq!(r.partitions.len(), 2);
        assert_eq!(r.partitions[0].at_ms, 0);
        assert_eq!(r.partitions[1].fragment_of, vec![0, 1, 1]);
    }

    #[test]
    fn range_parsing_is_strict() {
        assert_eq!(parse_range("3..7").unwrap(), (3, 7));
        assert!(parse_range("7..3").is_err());
        assert!(parse_range("a..b").is_err());
        assert!(parse_range("3").is_err());
        let set = NodeSet::List(vec![IndexSpec::Index(1), IndexSpec::Range("3..4".into())])
            .resolve(5)
            .unwrap();
        assert_eq!(set.into_iter().collect::<Vec<_>>(), vec![1, 3, 4]);
    }
}
