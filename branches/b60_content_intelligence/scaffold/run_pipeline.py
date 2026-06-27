#!/usr/bin/env python3
"""
run_pipeline.py — the v1 MVP end-to-end run: the two honest deliverables in one command.

BUILD.md's MVP target is NOT virality. It is:
  (a) one end-to-end run that ships a CLEARANCE-PASSING brief about a SAFE-LANE subject, and
  (b) one LEAKAGE-CONTROLLED eval verdict.

This driver runs the live emit path (controller -> outbox) for (a), runs the offline
backtest (eval harness) for (b), then shows the downstream-session view by PULLING the
outbox. It also demonstrates the gate failing closed on a real-person + political + voice
subject (held to counsel, never emitted). Offline + deterministic on the mock backend.

Run: python3 branches/b60_content_intelligence/scaffold/run_pipeline.py
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402
from orch import controller, blackboard  # noqa: E402
from eval import harness  # noqa: E402

LINE = "=" * 78


def main():
    print(LINE)
    print("B60 v1 MVP — prove the loop closes (ship a clearance-passing brief + an honest verdict)")
    print(LINE)

    # (a) live emit path -----------------------------------------------------
    print("\n[a] LIVE EMIT PATH — ingest -> salience -> link -> signal -> dense -> creative -> GATE -> emit")
    res = controller.run()           # auto-picks the medium-tier safe-lane anchor
    for t in res["trace"]:
        print("    -", t)
    assert res["status"] == "EMITTED", f"expected EMITTED, got {res['status']}"
    b = res["brief"]
    print(f"    => EMITTED scene {b['scene_id']}  (disposition={res['clearance']['scene_disposition']}, "
          f"derivative_risk={b['contract']['derivative_risk_flag']})")
    print(f"       disclosure: {b['contract']['disclosure_slot']}")
    print(f"       signed token: {res['clearance']['disclosure_token']['token']}")

    # fail-closed demonstration ---------------------------------------------
    print("\n[gate] FAIL-CLOSED CHECK — anchor on a real-person + political + voice subject")
    blocked = controller.run(anchor_id="IN_REAL_ARTIST_POLITICAL", emit=False)
    print(f"    => status={blocked['status']} (NOT emitted) — routed to counsel: "
          f"{[m['node_id'] for m in blocked.get('counsel_queue', [])]}")
    assert blocked["status"] != "EMITTED", "fail-closed subject must not emit"

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
