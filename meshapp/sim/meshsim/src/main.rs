//! meshsim: discrete-event simulator for meshcore nodes over a modelled
//! radio (docs/DESIGN.md §7 `SimLink`, §10 "Simulator scenarios";
//! docs/scenarios.md for the file format and the pass criteria).
//!
//! `meshsim run <scenario.toml> --seed N [--out file.json]
//! [--duration-override-ms X]` runs one scenario for one seed, writes the
//! JSON report, prints a one-line summary and exits 1 if any evaluated
//! criterion failed. `meshsim validate <scenario.toml>` only loads and
//! validates the file.

mod assert;
mod engine;
mod radio;
mod report;
mod scenario;
mod simnode;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "meshsim", about = "Headless multi-node simulator for meshcore")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Run one scenario for one seed and report.
    Run {
        /// Scenario file (sim/meshsim/scenarios/*.toml).
        scenario: PathBuf,
        /// Seed of the run; the same seed gives the same report.
        #[arg(long)]
        seed: u64,
        /// Where to write the JSON report.
        #[arg(long)]
        out: Option<PathBuf>,
        /// Run for this long instead of the scenario's duration.
        #[arg(long)]
        duration_override_ms: Option<u64>,
    },
    /// Load and validate a scenario file without running it.
    Validate { scenario: PathBuf },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Validate { scenario } => match scenario::load(&scenario) {
            Ok(s) => {
                let r = s.resolve().map(|r| r.partitions.len()).unwrap_or(0);
                println!(
                    "ok: {} ({} nodes, {} ms, {} groups, {} events, {} partitions): {}",
                    s.scenario.name,
                    s.n_nodes(),
                    s.scenario.duration_ms,
                    s.groups.list.len(),
                    s.events.len(),
                    r,
                    s.scenario.description
                );
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{}: {e}", scenario.display());
                ExitCode::from(2)
            }
        },
        Cmd::Run {
            scenario,
            seed,
            out,
            duration_override_ms,
        } => run_command(&scenario, seed, out.as_deref(), duration_override_ms),
    }
}

/// The host clock is read here and nowhere else: the simulator binary is
/// one of the "only places allowed to construct a Clock from the OS"
/// (clippy.toml), and this reading only times the run for the report.
#[allow(clippy::disallowed_methods)]
fn host_now() -> std::time::Instant {
    std::time::Instant::now()
}

fn run_command(
    path: &std::path::Path,
    seed: u64,
    out: Option<&std::path::Path>,
    duration_override_ms: Option<u64>,
) -> ExitCode {
    let scn = match scenario::load(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {e}", path.display());
            return ExitCode::from(2);
        }
    };
    let started = host_now();
    let mut report = match engine::run(&scn, seed, duration_override_ms) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: {e}", path.display());
            return ExitCode::from(2);
        }
    };
    report.host_seconds = started.elapsed().as_secs_f64();
    if let Some(out) = out {
        if let Some(dir) = out.parent()
            && !dir.as_os_str().is_empty()
            && let Err(e) = std::fs::create_dir_all(dir)
        {
            eprintln!("cannot create {}: {e}", dir.display());
            return ExitCode::from(2);
        }
        if let Err(e) = std::fs::write(out, report.to_json()) {
            eprintln!("cannot write {}: {e}", out.display());
            return ExitCode::from(2);
        }
    }
    println!("{} ({:.1} s)", report.summary_line(), report.host_seconds);
    if report.passed() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::parse;
    use crate::simnode::DropReason;

    const SCENARIO_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/scenarios");

    fn s0() -> scenario::Scenario {
        scenario::load(format!("{SCENARIO_DIR}/s0.toml")).unwrap()
    }

    #[test]
    fn s0_passes_its_evaluated_criteria() {
        let r = engine::run(&s0(), 1, None).unwrap();
        assert!(r.passed(), "{}\n{}", r.summary_line(), r.to_json());
        assert_eq!(r.delivery.ratio, 1.0);
        assert_eq!(r.delivery.miss_count, 0);
        assert_eq!(r.wall_clock_reads, 0);
        assert!(r.delivery.evaluated_cells > 0);
        assert!(r.cells_created.dms > 0, "s0 has one pair exchanging DMs");
        let names: Vec<&str> = r.criteria.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"every_post_decrypts_after_rejoin"));
        assert!(
            names.contains(&"members_h_identical"),
            "reported as not_evaluated"
        );
        assert_eq!(r.nodes.len(), 3);
        assert_eq!(r.drain_ms, None, "no rejoin in s0");
    }

    #[test]
    fn s0_is_deterministic_per_seed() {
        let a = engine::run(&s0(), 42, None).unwrap().deterministic_json();
        let b = engine::run(&s0(), 42, None).unwrap().deterministic_json();
        assert_eq!(a, b);
    }

    #[test]
    fn partition_model_never_links_across_fragments() {
        let text = r#"
[scenario]
name = "part"
duration_ms = 180_000
tick_ms = 100
[nodes]
android = 6
iphone = 0
iphone_foreground_duty = 1.0
android_slots = 7
battery_floor = 0.30
[radio]
latency_ms = 40
throughput_bytes_per_s = 15_000
frame_loss = 0.0
iphone_wake_ms = 10_000
wall_clock_offset_ms = [0, 0]
[groups]
[[groups.list]]
name = "main"
members = "all"
admins = [0]
[traffic]
group_post_every_ms = 20_000
text_bytes = [40, 300]
[[partition]]
at_ms = 0
fragments = [["0..2"], ["3..5"]]
[pass]
every_post_decrypts_after_rejoin = true
"#;
        let scn = parse(text).unwrap();
        let sim = engine::run_sim(&scn, 9, None).unwrap();
        assert!(sim.radio().links().count() > 0);
        for l in sim.radio().links() {
            assert_eq!(l.a / 3, l.b / 3, "live link {l:?} crosses the partition");
        }
        for rec in &sim.radio().history {
            assert_eq!(rec.a / 3, rec.b / 3, "link {rec:?} crossed the partition");
        }
        let r = sim.report();
        assert!(!r.passed(), "cells cannot cross fragments");
        for miss in &r.delivery.misses {
            for &n in &miss.missing_at {
                assert_ne!(
                    n / 3,
                    miss.creator / 3,
                    "a miss inside the creator's fragment"
                );
            }
        }
        // but every cell reached its own fragment
        for rec in &sim.registry().records {
            for n in 0..6 {
                if n / 3 == rec.creator / 3 {
                    assert!(sim.nodes()[n].holds(&rec.id), "{rec:?} missing at {n}");
                }
            }
        }
    }

    #[test]
    fn locked_iphones_wake_for_exactly_iphone_wake_ms_and_never_link_each_other() {
        let text = r#"
[scenario]
name = "wake"
duration_ms = 180_000
tick_ms = 100
[nodes]
android = 1
iphone = 2
iphone_foreground_duty = 0.0
android_slots = 7
battery_floor = 0.30
[radio]
latency_ms = 40
throughput_bytes_per_s = 15_000
frame_loss = 0.0
iphone_wake_ms = 10_000
wall_clock_offset_ms = [0, 0]
[groups]
[[groups.list]]
name = "main"
members = "all"
admins = [0]
[traffic]
group_post_every_ms = 20_000
text_bytes = [40, 300]
"#;
        let scn = parse(text).unwrap();
        let sim = engine::run_sim(&scn, 4, None).unwrap();
        let r = sim.report();
        assert!(r.links.wake_links >= 5, "{}", r.summary_line());
        assert_eq!(r.links.wake_links, r.links.established);
        for rec in &sim.radio().history {
            assert_eq!(rec.a, 0, "only the Android initiates: {rec:?}");
            assert!(rec.wake);
            assert_eq!(rec.reason, DropReason::WakeEnd);
            assert_eq!(rec.down_at - rec.up_at, 10_000);
        }
        for l in sim.radio().links() {
            assert_eq!(l.a, 0);
        }
        assert!(sim.radio().history.iter().all(|r| !(r.a > 0 && r.b > 0)));
        // cells still flow during the wakes
        assert!(r.delivery.ratio > 0.5, "{}", r.summary_line());
    }

    #[test]
    fn three_fragments_of_forty_nodes_fully_deliver_after_the_rejoin() {
        let text = r#"
[scenario]
name = "rejoin40"
duration_ms = 3_600_000
tick_ms = 100
[nodes]
android = 40
iphone = 0
iphone_foreground_duty = 1.0
android_slots = 7
battery_floor = 0.30
[radio]
latency_ms = 40
throughput_bytes_per_s = 15_000
frame_loss = 0.005
iphone_wake_ms = 10_000
wall_clock_offset_ms = [0, 0]
[groups]
[[groups.list]]
name = "main"
members = "all"
admins = [0, 1, 2]
[pairs]
random = 10
[traffic]
group_post_every_ms = 120_000
dm_every_ms = 600_000
text_bytes = [40, 300]
[[partition]]
at_ms = 0
fragments = [["0..17"], ["18..31"], ["32..39"]]
[[partition]]
at_ms = 1_800_000
fragments = "all"
[pass]
every_post_decrypts_after_rejoin = true
no_duplicate_ui_delivery = true
zero_wall_clock_reads = true
drain_report = true
"#;
        let scn = parse(text).unwrap();
        let r = engine::run(&scn, 1, None).unwrap();
        assert!(r.passed(), "{}\n{}", r.summary_line(), r.to_json());
        assert_eq!(r.delivery.ratio, 1.0);
        assert_eq!(r.rejoin_at_ms, Some(1_800_000));
        let drain = r
            .drain_ms
            .expect("every pre-rejoin cell reached every node");
        assert!(drain > 0 && drain < 1_800_000, "drain {drain} ms");
        assert!(r.links.established > 40, "{}", r.summary_line());
        assert_eq!(r.links.wake_links, 0, "no iPhones in this run");
    }
}
