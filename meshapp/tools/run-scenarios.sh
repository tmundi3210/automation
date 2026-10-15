#!/usr/bin/env bash
# Runs every simulator scenario over a range of seeds and writes one JSON per run to sim/out/.
# Usage: tools/run-scenarios.sh [SEEDS=20] [scenario.toml ...]
set -euo pipefail
cd "$(dirname "$0")/.."
SEEDS="${1:-20}"; shift || true
if [ "$#" -gt 0 ]; then SCEN=("$@"); else mapfile -t SCEN < <(ls sim/meshsim/scenarios/*.toml); fi
mkdir -p sim/out
cargo build --release -p meshsim >/dev/null
BIN=target/release/meshsim
fail=0
printf '%-14s %-6s %s\n' scenario seeds result
for s in "${SCEN[@]}"; do
  name="$(basename "$s" .toml)"
  ok=0
  for seed in $(seq 1 "$SEEDS"); do
    out="sim/out/${name}-${seed}.json"
    if "$BIN" run "$s" --seed "$seed" --out "$out" >/dev/null 2>"sim/out/${name}-${seed}.log"; then ok=$((ok+1)); else fail=1; fi
  done
  printf '%-14s %-6s %s\n' "$name" "$SEEDS" "$ok/$SEEDS pass"
done
exit $fail
