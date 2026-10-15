# Testing: what is verified where

This file separates what the Linux build host can verify from what only a phone can. Every milestone's definition of done is split the same way (docs/DESIGN.md §10–11). Nothing in the repo claims device behaviour without a device log attached.

## Verified on the build host (this repo's CI)

| Check | Command | Covers |
|---|---|---|
| Unit tests per module | `cargo test --workspace` | codec, hashes, AEAD, strict Ed25519 (conformance table `crates/meshcore/tests/vectors/ed25519_strict.json`), key cards, backends, wire cells, recognition tags, Noise link, roster rules, groups, tasks, votes, storage atomicity |
| Property tests | same (`proptest`) | `roster_convergence`, `final_rule`, `task_tiebreak`, `cell_roundtrip` (see DESIGN §10) |
| Parse fuzzing on stable | same (proptest harness over the parsers) | cell, frame, inner, roster op, summary, QR payload decoders never panic |
| Simulator scenarios | `tools/run-scenarios.sh` | S0, S1, S3, S4 (+variants), S-flood with the pass criteria in DESIGN §10; results in `sim/out/*.json` |
| Lints | `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check` | clock discipline (`clippy.toml` forbids `SystemTime::now`/`Instant::now`) |
| Backend stub | `cargo build -p meshcore --no-default-features --features backend-vodozemac` | the vodozemac backend compiles; no behaviour is claimed |

## Assumptions baked into the simulator (retune at gate 2, the measured spike)

Named `*_ASM` in code. Current values and their source:

| Constant | Value | Source |
|---|---|---|
| BLE per-pair latency | 40 ms + size / 15 kB/s | DESIGN §7 (assumption) |
| Frame loss | 0.5 % → link drop + re-handshake | DESIGN §7 (assumption) |
| Android link slots | 7 per node | DECISIONS D4 Q4/Q9 (Android 14; PARTIAL) |
| Locked-iPhone wake | 10 s per inbound connect from an Android/station/foreground iPhone; never locked↔locked | DECISIONS D10 Q1–Q4 (DTS statements; throughput per wake UNVERIFIED) |
| iPhone foreground duty | 30 % | simulation assumption |
| Cell size | 512 B | DESIGN §2.2; sim reports bytes-on-bottleneck for 512/768/1024 |
| Rate limits | control 20/min, chat 60/min, bulk 256 kB/min per link | DECISIONS D4 (assumption) |
| Buffers | 50 MB total, 5 MB per link | DECISIONS D4 (assumption) |
| Dedupe retention | max(message retention, 7 d key retention) + 72 h | BRIEF §5.5, DECISIONS §3 row (4,6,7) |
| "Needs someone" | 6 h powered-on without hearing from the holder | owner default (docs/decisions-assumed.md) |

## Not verifiable here (device checklist, gate 2)

- Any Bluetooth behaviour: discovery latency, MTU, GATT link limits, advertising overflow on locked iPhones, background wake duration and throughput, RPA rotation effects, station → locked-iPhone reachability.
- Battery per hour, OEM battery killers, the Android foreground-service type's acceptance by Play policy (gate 3).
- iOS compilation and App Store review (needs macOS/Xcode; AGPL compatibility unverified).
- Real-user comprehension of Provisional/Final, the same-name marker and "Pass messages along" (gate 5).

## Results

Filled in by `tools/run-scenarios.sh` (scenario × seeds × pass/fail) once M1a lands.
