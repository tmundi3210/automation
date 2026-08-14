#!/usr/bin/env python3
"""
run_pipeline.py — the v1 MVP end-to-end run: the two honest deliverables in one command.

BUILD.md's MVP target is NOT virality. It is:
  (a) one end-to-end run that ships a CLEARANCE-PASSING brief about a SAFE-LANE subject, and
  (b) one LEAKAGE-CONTROLLED eval verdict.

This driver runs the two SEPARATED steps — STEP 1 the brain (draft + flag), STEP 2 the
gate (the stop/yellow safety check + emit) — for (a), runs the offline backtest (eval
harness) for (b), then shows the downstream-session view by PULLING the outbox. It also
demonstrates the gate failing closed on a real-person + political + voice subject (held to
counsel, never emitted). Offline + deterministic on the mock backend.

Run: python3 branches/b60_content_intelligence/scaffold/run_pipeline.py
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402
from orch import brain, gate, blackboard  # noqa: E402
from eval import harness  # noqa: E402

LINE = "=" * 78


def main():
    print(LINE)
    print("B60 v1 MVP — prove the loop closes (ship a clearance-passing brief + an honest verdict)")
    print(LINE)

    # (a) the two separated steps -------------------------------------------
    print("\n[STEP 1] BRAIN — collect -> connect -> draft + flag safety facts (NO safety decision)")
    proposal = brain.run_brain()         # auto-picks the medium-tier safe-lane anchor
    assert proposal.get("status") == "DRAFTED", f"brain failed: {proposal.get('status')}"
    print(f"    drafted {proposal['scene_id']} (anchor={proposal['anchor']}, members={proposal['members']})")
    for sf in proposal["safety_facts"]:
        print(f"    fact: {sf['node_id']:26s} real={sf['is_real_identifiable_person']!s:5s} "
              f"political={sf['political_or_election']!s:5s} voice/likeness={sf['uses_voice_or_likeness']!s:5s}")

    print("\n[STEP 2] GATE — the separated stop/yellow check, run last (the only safety decision)")
    g = gate.run_gate(proposal)
    print(f"    VERDICT: {g['verdict']}   CLEAR TO RENDER: {'yes' if g['clear_to_render'] else 'no'}")
    print(f"    REQUIRED EDITS: {g['required_edits'] or 'none'}")
    print(f"    DISCLOSURE: {g['disclosure_line']}")
    assert g["clear_to_render"], f"safe-lane scene should clear, got {g['verdict']}"
    print(f"    => EMITTED to outbox (token {g['disclosure_token']['token']})")

    # fail-closed demonstration: brain drafts, gate STOPs ------------------
    print("\n[fail-closed] same two steps on a real-person + political + voice subject")
    p2 = brain.run_brain(anchor_id="IN_REAL_ARTIST_POLITICAL", save=False)
    g2 = gate.run_gate(p2, emit=False)
    print(f"    brain drafted it (no decision); GATE verdict={g2['verdict']} clear_to_render={g2['clear_to_render']}")
    print(f"    => held for counsel: {[m['node_id'] for m in g2['counsel_queue']]}")
    assert g2["verdict"] == "STOP", "fail-closed subject must STOP at the gate"

    # (b) offline leakage-controlled verdict --------------------------------
    print("\n[b] OFFLINE BACKTEST — frozen pre-registration, post-cutoff blind holdout, terminating loop")
    verdict, cert = harness.improve_loop()
    mb = verdict["medium_tier_bet"]
    print(f"    prereg={verdict['prereg_hash']}  post-cutoff n={verdict['n_post_cutoff']} "
          f"(leakage-dropped {len(verdict['leakage_dropped'])})")
    print(f"    medium-tier bet={mb['point']} CI{mb['ci']} excludes_zero={mb['excludes_zero']}; "
          f"beats_human={verdict['beats_human']}")
    print(f"    => VERDICT: {verdict['verdict']}  ({'; '.join(verdict['reasons'])})")
    print(f"       termination: {cert['reason']} after {cert['cycles_run']}/{cert['max_cycles']} cycles")

    # downstream-session view (outbox pull) ---------------------------------
    print("\n[outbox] DOWNSTREAM-SESSION VIEW — pull the durable packet (no in-process coupling)")
    packets = blackboard.pull()
    for p in packets:
        ok = bool(p.get("disclosure_token"))
        print(f"    {p['idempotency_key']}: {len(p['nodes'])} nodes, token_bound={ok}, "
              f"resonance_ok={p['resonance']['publish_ok']}")

    print("\n" + LINE)
    print("MVP RESULT: (a) shipped a clearance-passing safe-lane brief to the outbox; "
          f"(b) returned a leakage-controlled {verdict['verdict']}.")
    print("The loop CLOSES. Optimize content only after this holds on real data (see the hook points).")
    print(LINE)
    return 0


if __name__ == "__main__":
    sys.exit(main())
