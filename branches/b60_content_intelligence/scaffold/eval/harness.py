#!/usr/bin/env python3
"""
harness.py — leakage-controlled, frozen-metric, terminating backtest -> one honest verdict.

Grounded in `eval_heavy` (BUILD.md stage 8): a sealed PRE-REGISTRATION (one construct,
one OEC, two baselines, a beat-the-human threshold); a strictly POST-CUTOFF + BLIND
holdout scored against a frozen realized-outcome ledger with both leakage channels
controlled; bootstrap CIs with base-rate correction; a medium-tier claim that counts
only if its out-of-sample CI EXCLUDES ZERO; and an improve-loop that TERMINATES (threshold
OR max_cycles) with a termination certificate.

This is the MVP's second deliverable: it does not optimize content, it returns ONE
leakage-controlled GO/NO-GO with its evidence. Everything is deterministic (hash-seeded
bootstrap, no RNG state). HOOK POINT: swap the synthetic frozen holdout for a real
realized-outcome ledger + a confirmed cutoff before trusting the verdict. Stdlib only.
"""
import argparse
import hashlib
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402

HOLDOUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_fixtures", "holdout.frozen.jsonl")

# --- the FROZEN pre-registration (hash this; never edit post-hoc) ----------
PREREG = {
    "version": "prereg-v1",
    "construct": "diaspora-anxiety resonance",
    "oec": "medium_tier_bet = mean(realized|MEDIUM_OPPORTUNITY) - mean(realized|OTHER)",
    "baselines": ["naive_base_rate", "human_curated"],
    "selection_k_frac": 0.3,
    "delta_min": 0.0,          # GO requires the OEC CI lower bound to exceed this
    "ci": 0.95,
    "bootstrap_B": 2000,
    "max_cycles": 5,
    "go_requires": "OEC CI excludes delta_min AND system beats BOTH baselines on top-k realized",
}


def prereg_hash():
    return hashlib.sha256(json.dumps(PREREG, sort_keys=True).encode()).hexdigest()[:16]


def _u(i, salt):
    return int(hashlib.sha256(f"{salt}:{i}".encode()).hexdigest()[:8], 16) / 0xFFFFFFFF


def _mean(xs):
    return sum(xs) / len(xs) if xs else 0.0


def load_holdout(path=HOLDOUT):
    rows = common.read_jsonl(path)
    meta = rows[0]["_meta"] if rows and "_meta" in rows[0] else {}
    items = [r for r in rows if "_meta" not in r]
    return meta, items


def control_leakage(items, cutoff):
    """Two channels: (1) TEMPORAL — keep only strictly post-cutoff items; (2) BLIND —
    the holdout carries realized outcomes the system never saw (asserted by construction).
    Returns (kept, dropped)."""
    kept = [it for it in items if it["item_date"] > cutoff]
    dropped = [it["id"] for it in items if it["item_date"] <= cutoff]
    return kept, dropped


def _topk_mean(items, key, k):
    ranked = sorted(items, key=lambda it: -it[key])[:k]
    return _mean([it["realized"] for it in ranked])


def _bootstrap_ci(values_fn, items, B, conf):
    """Percentile bootstrap CI of a statistic over resampled items (hash-seeded)."""
    n = len(items)
    stats = []
    for b in range(B):
        sample = [items[int(_u(b * n + j, "boot") * n)] for j in range(n)]
        stats.append(values_fn(sample))
    stats.sort()
    lo = stats[int((1 - conf) / 2 * B)]
    hi = stats[int((1 + conf) / 2 * B)]
    return round(lo, 4), round(hi, 4), round(_mean(stats), 4)


def _medium_bet(items):
    med = [it["realized"] for it in items if it["tier"] == "MEDIUM_OPPORTUNITY"]
    oth = [it["realized"] for it in items if it["tier"] != "MEDIUM_OPPORTUNITY"]
    if not med or not oth:
        return 0.0
    return _mean(med) - _mean(oth)


def evaluate(path=HOLDOUT):
    meta, items = load_holdout(path)
    cutoff = meta.get("cutoff", PREREG["version"])
    kept, dropped = control_leakage(items, cutoff)
    n = len(kept)
    k = max(1, int(round(PREREG["selection_k_frac"] * n)))
    base_rate = _mean([it["realized"] for it in kept])

    # selection lift over baselines (top-k realized mean minus base rate)
    sys_mean = _topk_mean(kept, "system_score", k)
    human_mean = _topk_mean(kept, "human_score", k)
    sys_lift = round(sys_mean - base_rate, 4)
    human_lift = round(human_mean - base_rate, 4)
    beats_naive = sys_mean > base_rate
    beats_human = sys_mean > human_mean

    # primary OEC: the medium-tier bet, with a bootstrap CI (counts iff CI excludes delta_min)
    lo, hi, mean = _bootstrap_ci(_medium_bet, kept, PREREG["bootstrap_B"], PREREG["ci"])
    bet_excludes_zero = lo > PREREG["delta_min"]

    # guardrail penalty: fraction of system-selected items failing a guardrail
    sel = sorted(kept, key=lambda it: -it["system_score"])[:k]
    guardrail_penalty = round(_mean([0.0 if it["guardrail_pass"] else 1.0 for it in sel]), 4)

    go = bool(bet_excludes_zero and beats_naive and beats_human and guardrail_penalty == 0.0)
    reasons = []
    if not bet_excludes_zero:
        reasons.append(f"medium-tier OEC CI [{lo},{hi}] includes delta_min={PREREG['delta_min']}")
    if not beats_naive:
        reasons.append("system does not beat naive base rate")
    if not beats_human:
        reasons.append(f"system top-k realized {sys_mean:.3f} does not beat human-curated {human_mean:.3f}")
    if guardrail_penalty > 0:
        reasons.append(f"guardrail penalty {guardrail_penalty} on selected set")

    return {
        "prereg_hash": prereg_hash(), "cutoff": cutoff, "n_post_cutoff": n,
        "leakage_dropped": dropped, "base_rate": round(base_rate, 4),
        "system_topk_realized": round(sys_mean, 4), "human_topk_realized": round(human_mean, 4),
        "system_lift_over_naive": sys_lift, "human_lift_over_naive": human_lift,
        "beats_naive": beats_naive, "beats_human": beats_human,
        "medium_tier_bet": {"point": round(mean, 4), "ci": [lo, hi],
                            "excludes_zero": bet_excludes_zero},
        "guardrail_penalty": guardrail_penalty,
        "verdict": "GO" if go else "NO-GO",
        "reasons": reasons or ["all pre-registered conditions met"],
        "note": "measured on the frozen SYNTHETIC holdout; real realized-outcome ledger is a HOOK POINT",
    }


def improve_loop(path=HOLDOUT):
    """The offline improve-loop: evaluate up to max_cycles; terminate on GO or max_cycles
    with a termination certificate (the loop ALWAYS terminates). The scaffold does not
    retrain between cycles, so a NO-GO converges to a max_cycles certificate."""
    result, cycles, reason = None, 0, "max_cycles"
    for c in range(1, PREREG["max_cycles"] + 1):
        cycles = c
        result = evaluate(path)
        if result["verdict"] == "GO":
            reason = "threshold_met"
            break
    cert = {"terminated": True, "cycles_run": cycles, "max_cycles": PREREG["max_cycles"],
            "reason": reason, "final_verdict": result["verdict"], "prereg_hash": prereg_hash()}
    return result, cert


if __name__ == "__main__":
    ap = argparse.ArgumentParser(description="Leakage-controlled frozen-metric backtest.")
    ap.add_argument("--holdout", default=HOLDOUT)
    a = ap.parse_args()
    res, cert = improve_loop(a.holdout)
    print(f"prereg={res['prereg_hash']} construct='{PREREG['construct']}' cutoff={res['cutoff']}")
    print(f"post-cutoff n={res['n_post_cutoff']} (leakage-dropped {len(res['leakage_dropped'])}); "
          f"base_rate={res['base_rate']}")
    print(f"selection: system top-k realized={res['system_topk_realized']} vs human={res['human_topk_realized']} "
          f"vs base={res['base_rate']}  beats_naive={res['beats_naive']} beats_human={res['beats_human']}")
    mb = res["medium_tier_bet"]
    print(f"medium-tier bet: {mb['point']} CI{mb['ci']} excludes_zero={mb['excludes_zero']}")
    print(f"guardrail_penalty={res['guardrail_penalty']}")
    print(f"VERDICT: {res['verdict']}  reasons: {res['reasons']}")
    print(f"termination certificate: {cert}")
