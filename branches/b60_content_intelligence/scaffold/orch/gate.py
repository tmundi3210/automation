#!/usr/bin/env python3
"""
gate.py — STEP 2 of 2: the GATE. The separated stop/yellow safety check (run last).

This is the safety half, decoupled from the brain (`brain.py`). It reads a Proposal from
`_proposals/`, applies the clearance rules + the resonance check, and returns ONE explicit
verdict — GREEN / YELLOW(+required edits) / STOP(+reason) — plus the disclosure line and a
single CLEAR_TO_RENDER flag. Only when it clears does it complete safety_flags, validate
every node against the schema, and write the EmitPacket to the outbox.

Separating the gate is the whole point: the brain never refuses mid-thought; every stop is
an explicit, reviewable decision here, with the exact edits that would turn a YELLOW green.
Hard blocks (minor, real-person+political+voice, etc.) are never overridden. Stdlib only.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402
from compliance import clearance  # noqa: E402
from psych import resonance  # noqa: E402
from orch import blackboard as bb  # noqa: E402

PROPOSALS = common.PROPOSALS_DIR

# disposition -> human-facing verdict (the stop / yellow / green sign)
_VERDICT = {"ALLOW": "GREEN", "ALLOW_WITH_CONSTRAINTS": "YELLOW",
            "HUMAN_REVIEW": "STOP", "BLOCK": "STOP"}

# constraint code -> plain-language required edit (what makes a YELLOW shippable)
_EDIT_TEXT = {
    "SR_SYN_DISCLOSE": "show the AI-generated disclosure on/under the content",
    "SR_FACTUAL_ONLY": "keep the political topic factual; state no opinion as fact",
    "SR_NO_CANDIDATE_DEPICTION": "do not depict any candidate or office-holder",
    "SR_PARODY_CUES": "add visible parody cues so it can't be mistaken for the real person",
    "SR_LIKENESS": "do not reproduce the real person's face/likeness",
    "SR_VOICE": "do not clone or imitate the real person's voice",
    "SR_NO_FACT_IMPLICATION": "do not imply the real person said/did/endorsed this",
    "SR_ELECTION": "no election/candidate context involving a real person",
    "SR_SOURCE_LICENSE": "confirm a license/consent for the source before use",
    "SR_MINOR_PROTECTED": "subject is a minor/protected person — not permitted",
}


def latest_proposal():
    if not os.path.isdir(PROPOSALS):
        return None
    files = sorted(f for f in os.listdir(PROPOSALS) if f.endswith(".json"))
    return common.read_json(os.path.join(PROPOSALS, files[-1])) if files else None


def _records_from(proposal):
    return [{"node": proposal["nodes"][mid], "raw": proposal["raw"][mid], "meta": {}}
            for mid in proposal["members"]]


def run_gate(proposal, emit=True, backend=None):
    """Run step 2 over a Proposal. Returns the verdict + (on clear) the emitted packet."""
    scene = {"anchor": proposal["anchor"], "members": proposal["members"]}
    brief = proposal["brief"]
    records = _records_from(proposal)

    # the safety checks (this is the only place safety is decided)
    cr = clearance.clear_scene(scene, brief, records)          # writes safety_flags onto nodes
    res = resonance.check(scene, cr, records)

    verdict = _VERDICT[cr["scene_disposition"]]
    required_edits = sorted({c for m in cr["per_member"] if m["disposition"] != "ALLOW"
                             for c in m["constraints"]})
    edit_text = [_EDIT_TEXT.get(c, c) for c in required_edits]
    reasons = [m["review_reason"] for m in cr["per_member"] if m["review_reason"]]

    # schema-parse check (nodes now carry safety_flags)
    bad = {mid: errs for mid in proposal["members"]
           if (errs := common.validate_node(proposal["nodes"][mid]))}

    clear_to_render = bool(cr["passed"] and res["publish_ok"] and not bad)
    token_ok, token_why = (False, "scene did not pass")
    packet, outbox_path = None, None
    if clear_to_render:
        token_ok, token_why = clearance.verify_token(cr, brief["contract"]["disclosure_slot"])
        clear_to_render = clear_to_render and token_ok
    if clear_to_render and emit:
        packet = bb.make_packet(scene, brief, cr, res, records)
        outbox_path, _new = bb.enqueue(packet)

    if not res["publish_ok"] and verdict != "STOP":
        verdict, reasons = "STOP", reasons + res["blockers"]   # resonance can hold a pass

    return {
        "step": "gate",
        "scene_id": brief["scene_id"],
        "verdict": verdict,                                     # GREEN / YELLOW / STOP
        "reason": "; ".join(reasons) or ("clears the green lane" if verdict == "GREEN"
                                         else "allowed with the edits below"),
        "required_edits": edit_text,
        "disclosure_line": brief["contract"]["disclosure_slot"],
        "clear_to_render": clear_to_render,
        "schema_errors": bad,
        "disposition": cr["scene_disposition"],
        "counsel_queue": cr["counsel_queue"],
        "disclosure_token": cr.get("disclosure_token"),
        "token_check": token_why,
        "packet": packet,
        "outbox_path": outbox_path,
        "clearance": cr,
        "resonance": res,
    }


if __name__ == "__main__":
    p = latest_proposal()
    if not p:
        print("no proposal found — run brain.py first"); sys.exit(0)
    g = run_gate(p, emit=True)
    print(f"GATE on {g['scene_id']}")
    print(f"  VERDICT: {g['verdict']}")
    print(f"  REASON: {g['reason']}")
    print(f"  REQUIRED EDITS: {g['required_edits'] or 'none'}")
    print(f"  DISCLOSURE LINE: {g['disclosure_line']}")
    print(f"  CLEAR TO RENDER: {'yes' if g['clear_to_render'] else 'no'}")
    if g["clear_to_render"]:
        print(f"  emitted -> _outbox/{os.path.basename(g['outbox_path'])} (token {g['disclosure_token']['token']})")
    elif g["counsel_queue"]:
        print(f"  held for counsel: {[m['node_id'] for m in g['counsel_queue']]}")
