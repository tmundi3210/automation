#!/usr/bin/env bash
set -euo pipefail

home_dir="${HOME:-}"
state_dir="${home_dir}/.exchange-gate"
codex_dir="${home_dir}/.exchange-gate-codex"
stamp="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

echo "GATE-AUDIT-START ${stamp}"
echo "HOME=${home_dir}"
echo "STATE_DIR=${state_dir}"
echo "CODEX_DIR=${codex_dir}"

probe_dir() {
  local label="$1"
  local path="$2"
  if [ -e "$path" ]; then
    echo "${label}: exists"
    ls -ld "$path"
    if [ -f "$path/last_seen_tip" ]; then
      echo "${label}: last_seen_tip=$(cat "$path/last_seen_tip")"
    else
      echo "${label}: last_seen_tip=missing"
    fi
    if [ -f "$path/last_seen_msg" ]; then
      echo "${label}: last_seen_msg=$(cat "$path/last_seen_msg")"
    else
      echo "${label}: last_seen_msg=missing"
    fi
  else
    echo "${label}: missing"
  fi
}

probe_dir "shared" "$state_dir"
probe_dir "codex" "$codex_dir"

if [ -d "$state_dir" ] && [ -d "$codex_dir" ]; then
  echo "ACTION: copy any codex-owned state into ${codex_dir} and re-point the codex wrapper there"
elif [ -d "$state_dir" ] && [ ! -d "$codex_dir" ]; then
  echo "ACTION: create ${codex_dir} and migrate codex-owned gate files out of ${state_dir}"
else
  echo "ACTION: no shared gate directory detected; keep codex state isolated in ${codex_dir}"
fi

cat <<'EOF'
EVIDENCE:
- checked_home_access: operator-run only
- target_state: codex-owned poller state should live under $HOME/.exchange-gate-codex/
- conflict_check: shared $HOME/.exchange-gate/ usage is the risk to eliminate
EOF

echo "GATE-AUDIT-END"
