#!/usr/bin/env python3
"""
make_holdout.py — deterministically generate the FROZEN post-cutoff blind holdout.

The holdout is a committed, frozen artifact (leakage control requires it be fixed and
strictly post-knowledge-cutoff). This generator documents exactly how it was built and
lets anyone reproduce it bit-for-bit: all randomness is a hash of the item index (no
Date/random), so re-running yields the identical file.

Each item carries a realized outcome (what actually happened, post-cutoff), a system
prediction, a human-curated baseline prediction, a tier label (to test the medium-tier
bet), an item_date strictly after the cutoff, and a guardrail-pass flag.

Run: python3 eval/_fixtures/make_holdout.py   # writes holdout.frozen.jsonl
"""
import hashlib
import json
import os

CUTOFF = "2026-01-31"          # confirmed model/designer knowledge cutoff
N = 40
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "holdout.frozen.jsonl")


def u(i, salt):
    """Deterministic uniform [0,1) from a hash of (salt, index) — reproducible, no RNG state."""
    return int(hashlib.sha256(f"{salt}:{i}".encode()).hexdigest()[:8], 16) / 0xFFFFFFFF


def _clip(x):
    return round(max(0.0, min(1.0, x)), 4)


def make():
    rows = []
    for i in range(N):
        system_score = round(u(i, "sys"), 4)
        is_medium = u(i, "med") < 0.5
        noise = (u(i, "noise") - 0.5) * 0.40
        realized = _clip(0.15 + 0.50 * system_score + 0.18 * (1 if is_medium else 0) + noise)
        human_score = _clip(realized + (u(i, "human") - 0.5) * 0.50)   # noisy human proxy
        month = 2 + (i % 5)                                            # 2026-02 .. 2026-06 (post-cutoff)
        rows.append({
            "id": f"HO_{i:04d}",
            "item_date": f"2026-{month:02d}-15",
            "tier": "MEDIUM_OPPORTUNITY" if is_medium else "OTHER",
            "system_score": system_score,        # the system's predicted opportunity
            "human_score": human_score,          # human-curated baseline prediction
            "realized": realized,                # realized outcome (blind; post-cutoff)
            "guardrail_pass": u(i, "safe") > 0.05,
        })
    return rows


if __name__ == "__main__":
    rows = make()
    with open(OUT, "w") as f:
        f.write(f'{json.dumps({"_meta": {"cutoff": CUTOFF, "n": N, "construct": "diaspora-anxiety resonance", "frozen": True}})}\n')
        for r in rows:
            f.write(json.dumps(r) + "\n")
    print(f"wrote {OUT} ({N} post-cutoff items, cutoff={CUTOFF})")
