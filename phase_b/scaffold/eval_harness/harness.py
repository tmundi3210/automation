#!/usr/bin/env python3
"""
harness.py — evaluation harness: test sets + scorers + LLM-as-judge + score tracking.

Grounded in the 'eval' specialist and IDEA_NEUTRALIZED.md ("scoring system; pull real
benchmark questions; test yourself" -> harness + curated/real test sets + LLM-as-judge
+ score tracking). This is also the evaluate() that train/rollback_loop.py calls in
production.

Scorers (chosen per item.task_type), reflecting the eval specialist:
  * exact/numeric  -> tool-grounded comparison (number parse + tolerance), NOT a model
                      (mirrors the math boundary: exact work is computed, not guessed).
  * choice         -> normalized string match against the reference.
  * open           -> LLM-as-judge via backend.score(rubric), with a contamination flag
                      and a note on judge bias (position/self-preference).
  * pass@k         -> any-of-k samples correct (metric-at-k).

The "system under test" is any callable question->answer; plug in MockBackend, a served
fine-tuned checkpoint (LocalBackend), or the orchestration graph. Score history is
appended to a JSONL so trends are trackable across runs. Stdlib only.
"""
import argparse
import json
import os
import re
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402
import backends  # noqa: E402

EXAMPLE_SET = os.path.join(os.path.dirname(os.path.abspath(__file__)), "testset.example.jsonl")
HISTORY = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_scores_history.jsonl")


# ------------------------------------------------------------------ scorers
def _numbers(s):
    return re.findall(r"-?\d+(?:\.\d+)?", s or "")


def score_numeric(answer, reference, tol=1e-6):
    a, r = _numbers(answer), _numbers(reference)
    if not a or not r:
        return 0.0
    try:
        return 1.0 if abs(float(a[-1]) - float(r[-1])) <= tol else 0.0
    except ValueError:
        return 0.0


def score_choice(answer, reference):
    norm = lambda s: re.sub(r"[^a-z0-9]+", "", (s or "").lower())
    return 1.0 if norm(reference) and norm(reference) in norm(answer) else 0.0


def score_open(answer, item, judge):
    rubric = (f"Rate 0..1 how well the answer meets the reference for the question. "
              f"Q: {item['question']}\nReference: {item.get('reference','')}\nAnswer: {answer}")
    return judge.score(rubric, system="impartial grader; ignore answer order/verbosity")


def contamination_flag(item):
    """Toy guard (eval specialist: contamination detection): reference text leaking
    verbatim into the question makes the item trivially gameable. Choice items are
    exempt — their options legitimately contain the answer."""
    if item.get("task_type") == "choice":
        return False
    ref = (item.get("reference") or "").strip()
    return bool(ref) and len(ref) > 3 and ref.lower() in item["question"].lower()


# ------------------------------------------------------------------ systems under test
def mock_system(backend):
    return lambda q: backend.generate(q, max_tokens=48)


def oracle_system():
    """Echoes the reference — used to VALIDATE the scorers (should score ~1.0)."""
    return lambda item_ref: item_ref


# ------------------------------------------------------------------ harness
def evaluate(testset, system_fn, judge=None, k=1, oracle=False):
    judge = judge or backends.get_backend("mock")
    per_item, by_type = [], {}
    for item in testset:
        tt = item.get("task_type", "open")
        # system under test: oracle gets the reference (scorer validation), else the question
        if oracle:
            samples = [item.get("reference", "")]
        else:
            samples = [system_fn(item["question"]) for _ in range(k)]

        best = 0.0
        for ans in samples:
            if tt == "numeric":
                s = score_numeric(ans, item.get("reference", ""))
            elif tt == "choice":
                s = score_choice(ans, item.get("reference", ""))
            else:
                s = score_open(ans, item, judge)
            best = max(best, s)

        row = {"id": item["id"], "task_type": tt, "score": round(best, 3),
               "contaminated": contamination_flag(item),
               "answer": samples[0][:80]}
        per_item.append(row)
        by_type.setdefault(tt, []).append(best)

    by_type_avg = {t: round(sum(v) / len(v), 3) for t, v in by_type.items()}
    overall = round(sum(r["score"] for r in per_item) / max(1, len(per_item)), 3)
    contaminated = [r["id"] for r in per_item if r["contaminated"]]
    return {"overall": overall, "by_type": by_type_avg, "n": len(per_item),
            "contaminated_items": contaminated, "per_item": per_item, "k": k}


def track(result, run_id):
    rec = {"ts": time.strftime("%Y-%m-%dT%H:%M:%S"), "run_id": run_id,
           "overall": result["overall"], "by_type": result["by_type"], "n": result["n"]}
    with open(HISTORY, "a") as f:
        f.write(json.dumps(rec) + "\n")
    return rec


def load_testset(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def main():
    ap = argparse.ArgumentParser(description="Run the eval harness over a test set.")
    ap.add_argument("testset", nargs="?", default=EXAMPLE_SET)
    ap.add_argument("--system", default="mock", choices=["mock", "oracle"],
                    help="mock=MockBackend under test; oracle=echo reference (validates scorers)")
    ap.add_argument("--k", type=int, default=1, help="samples per item (pass@k / metric-at-k)")
    ap.add_argument("--run-id", default=None)
    a = ap.parse_args()

    testset = load_testset(a.testset)
    backend = backends.get_backend("mock")
    if a.system == "oracle":
        res = evaluate(testset, None, judge=backend, k=a.k, oracle=True)
    else:
        res = evaluate(testset, mock_system(backend), judge=backend, k=a.k)
    rec = track(res, a.run_id or f"{a.system}-k{a.k}")

    print(f"testset: {os.path.relpath(a.testset, common.REPO_ROOT)}  n={res['n']}  "
          f"system={a.system}  k={a.k}")
    print(f"overall: {res['overall']}   by_type: {res['by_type']}")
    if res["contaminated_items"]:
        print(f"CONTAMINATION FLAGGED: {res['contaminated_items']} (excluded from trust, not score)")
    for r in res["per_item"]:
        flag = " [contam]" if r["contaminated"] else ""
        print(f"  {r['id']:14s} {r['task_type']:8s} {r['score']:.3f}{flag}  ans={r['answer']!r}")
    print(f"tracked -> {os.path.relpath(HISTORY, common.REPO_ROOT)} ({rec['ts']})")


if __name__ == "__main__":
    main()
