//! End-to-end check of the `meshsim` binary (docs/scenarios.md: `meshsim run
//! <scenario.toml> --seed N [--out result.json]`, `meshsim validate`): every
//! shipped scenario validates, S0 runs, writes JSON, exits 0 and is
//! deterministic per seed.

use std::path::PathBuf;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_meshsim"))
}

fn scenarios() -> Vec<PathBuf> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios");
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "toml"))
        .collect();
    v.sort();
    v
}

#[test]
fn validate_accepts_every_shipped_scenario_and_rejects_garbage() {
    let files = scenarios();
    assert_eq!(files.len(), 7);
    for f in &files {
        let out = bin().arg("validate").arg(f).output().unwrap();
        assert!(
            out.status.success(),
            "{}: {}",
            f.display(),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(String::from_utf8_lossy(&out.stdout).starts_with("ok: "));
    }
    let tmp = std::env::temp_dir().join(format!("meshsim-bad-{}.toml", std::process::id()));
    std::fs::write(&tmp, "[scenario]\nname = \"x\"\nbogus = 1\n").unwrap();
    let out = bin().arg("validate").arg(&tmp).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let _ = std::fs::remove_file(&tmp);
}

#[test]
fn run_s0_writes_a_deterministic_report_and_exits_zero() {
    let s0 = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios/s0.toml");
    let dir = std::env::temp_dir().join(format!("meshsim-cli-{}", std::process::id()));
    let a = dir.join("a.json");
    let b = dir.join("b.json");
    for out in [&a, &b] {
        let res = bin()
            .args(["run", s0.to_str().unwrap(), "--seed", "3", "--out"])
            .arg(out)
            .output()
            .unwrap();
        assert!(
            res.status.success(),
            "{}",
            String::from_utf8_lossy(&res.stderr)
        );
        let line = String::from_utf8_lossy(&res.stdout);
        assert!(
            line.contains("s0 seed=3") && line.contains("PASS"),
            "{line}"
        );
    }
    let ja: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&a).unwrap()).unwrap();
    let mut jb: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&b).unwrap()).unwrap();
    assert_eq!(ja["scenario"], "s0");
    assert_eq!(ja["seed"], 3);
    assert_eq!(ja["delivery"]["ratio"], 1.0);
    assert_eq!(ja["wall_clock_reads"], 0);
    assert!(
        ja["criteria"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["status"] == "not_evaluated")
    );
    let mut ja = ja;
    ja["host_seconds"] = 0.into();
    jb["host_seconds"] = 0.into();
    assert_eq!(ja, jb, "same seed, same report");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn duration_override_shortens_the_run() {
    let s1 = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scenarios/s1.toml");
    let res = bin()
        .args([
            "run",
            s1.to_str().unwrap(),
            "--seed",
            "1",
            "--duration-override-ms",
            "30000",
        ])
        .output()
        .unwrap();
    let line = String::from_utf8_lossy(&res.stdout);
    assert!(
        line.contains("s1 seed=1 nodes=40"),
        "{line}\n{}",
        String::from_utf8_lossy(&res.stderr)
    );
}
