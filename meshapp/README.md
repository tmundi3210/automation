# meshapp

Protocol core and simulator for an offline Bluetooth-LE mesh coordination app: verified identities, end-to-end-authenticated messaging and group tasking that keep working when the network splits and rejoins (disaster response, shelters). This directory implements **milestone 1** of the product brief: the `meshcore` Rust crate and the `meshsim` multi-node simulator. No phone apps yet.

- Product brief (normative): `BRIEF_v2.md` and the decision record `DECISIONS.md` (kept outside this repo with the design workspace; copies can be added under `docs/` on request).
- Design: `docs/DESIGN.md`. Decisions and defaults assumed for this milestone: `docs/decisions-assumed.md`.
- What is verified here and what is not: `docs/testing.md`.

## Layout

```
crates/meshcore   protocol core: identity, codec, roster DAG, groups, DTN relay, tasks, votes, storage
sim/meshsim       discrete-event simulator and the brief's scenarios (S0, S1, S3, S4, S-flood)
docs/             design, wire format, roster rules, tasks, votes, storage, testing, audit scope
tools/            get-protoc.sh (build prerequisite), run-scenarios.sh
```

## Build

Prerequisites: Rust 1.97 (pinned in `rust-toolchain.toml`), a C compiler (bundled SQLite), and `protoc` for the libsignal backend:

```
tools/get-protoc.sh          # downloads protoc 29.3 into tools/protoc (git-ignored)
cargo build --workspace
cargo test --workspace
tools/run-scenarios.sh       # all simulator scenarios × seeds → sim/out/*.json
```

Feature flags on `meshcore`: `backend-libsignal` (default) and `backend-vodozemac` (stub; compiles, no behaviour claimed).

## Licence

The default crypto backend is [libsignal](https://github.com/signalapp/libsignal) (`libsignal-protocol`, **AGPL-3.0-only**), so this code is published under AGPL-3.0-only as long as that backend is the default. The alternative backend, [vodozemac](https://github.com/matrix-org/vodozemac) (Apache-2.0), is stubbed behind a feature flag; switching to it is an owner decision (brief §5.2 gate 1) and would change the licence options. App Store compatibility of AGPL software is unverified.

## Status

Milestone 1 in progress. See `docs/testing.md` for the scenario results table once it exists; nothing in this README claims a result that the test output does not show.
