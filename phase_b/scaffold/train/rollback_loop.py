#!/usr/bin/env python3
"""
rollback_loop.py — eval-in-the-loop training with checkpoint rollback + neutral prune.

Grounded in the 'ftune' specialist (subdomain: monitored training loop) and
IDEA_NEUTRALIZED.md: "watch while fine-tuning; if no progress, step back, delete
neutral last-N" -> eval-in-the-loop training with checkpointing, early stopping, and
rollback when the monitored score stalls/regresses, plus neutral-tail pruning.

Runs offline: the default TrainerBackend is a deterministic SIMULATOR whose eval
trajectory rises, plateaus, then overfits — so the rollback/prune/lr-decay/early-stop
logic is exercised every run without a GPU. The real path is a marked hook point
(HFTrainerBackend) that trains N steps, writes a checkpoint, and returns an eval score
(which in production comes from phase_b/scaffold/eval_harness). Stdlib only.
"""
import argparse
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402


# ----------------------------------------------------------------- trainer backends
class TrainerBackend:
    def train_steps(self, n, lr, total_step):
        """Advance training n steps; return a checkpoint handle (here: a path/string)."""
        raise NotImplementedError

    def evaluate(self, ckpt, total_step, lr):
        """Return a monitored eval score in [0,1]. In production this calls the eval
        harness on held-out tasks."""
        raise NotImplementedError


class SimulatedTrainer(TrainerBackend):
    """Deterministic synthetic dynamics: a saturating gain curve minus an overfit
    penalty that grows after a knee, modulated by learning rate. Lowering lr (on
    rollback) shifts the knee out and buys a smaller further gain — exactly the
    behavior the rollback policy is designed to harvest."""

    def __init__(self, knee=60, noise=0.0):
        self.knee = knee
        self.noise = noise

    def train_steps(self, n, lr, total_step):
        return f"ckpt@{total_step}"

    def evaluate(self, ckpt, total_step, lr):
        # saturating improvement
        gain = 1 - math.exp(-total_step / (self.knee * (0.5 + lr / 0.0002)))
        # overfit penalty past the (lr-shifted) knee
        knee = self.knee * (0.6 + lr / 0.0002)
        overfit = max(0.0, (total_step - knee)) * 0.0009 * (lr / 0.0002)
        score = 0.45 + 0.5 * gain - overfit
        if self.noise:
            # deterministic pseudo-noise from the step (kept off by default)
            score += ((hash((total_step, ckpt)) % 1000) / 1000 - 0.5) * self.noise
        return max(0.0, min(1.0, round(score, 4)))


class HFTrainerBackend(TrainerBackend):
    """HOOK POINT — real fine-tune (transformers/TRL on a rented GPU). Implement
    train_steps() to run a Trainer for n steps and save a checkpoint, and evaluate()
    to score it via phase_b/scaffold/eval_harness. Left unimplemented so the scaffold
    needs no GPU/torch."""

    def train_steps(self, n, lr, total_step):
        raise NotImplementedError("HFTrainerBackend is a GPU hook point; use SimulatedTrainer offline.")

    def evaluate(self, ckpt, total_step, lr):
        raise NotImplementedError


# ----------------------------------------------------------------- the loop
DEFAULT_CONFIG = {
    "max_steps": 400,
    "eval_every": 20,
    "patience": 2,          # evals without min_delta improvement before a rollback
    "min_delta": 0.002,     # what counts as "progress"
    "neutral_prune_n": 2,   # delete the neutral last-N checkpoints on rollback
    "max_rollbacks": 3,     # early-stop after this many rollbacks without new best
    "lr": 0.0002,
    "lr_decay_on_rollback": 0.5,
}


def run(config=None, trainer=None, verbose=True):
    cfg = {**DEFAULT_CONFIG, **(config or {})}
    trainer = trainer or SimulatedTrainer()
    lr = cfg["lr"]
    best = {"score": -1.0, "ckpt": None, "step": 0}
    since_improve = 0
    rollbacks = 0
    step = 0
    lineage = []           # full audit trail (every eval, every action)
    active = []            # the "kept" checkpoints (neutral tail gets pruned off)
    stop_reason = "max_steps"

    while step < cfg["max_steps"]:
        step += cfg["eval_every"]
        ckpt = trainer.train_steps(cfg["eval_every"], lr, step)
        score = trainer.evaluate(ckpt, step, lr)
        rec = {"step": step, "ckpt": ckpt, "score": score, "lr": lr,
               "action": None, "parent": best["ckpt"]}

        if score >= best["score"] + cfg["min_delta"]:
            best = {"score": score, "ckpt": ckpt, "step": step}
            since_improve = 0
            rollbacks = 0  # a genuine new best resets the rollback budget
            rec["action"] = "save_best"
            active.append(rec)
        else:
            since_improve += 1
            active.append(rec)
            if since_improve >= cfg["patience"]:
                # ROLLBACK: rewind to best, prune the neutral last-N, decay lr.
                pruned = []
                for _ in range(cfg["neutral_prune_n"]):
                    if active and active[-1]["ckpt"] != best["ckpt"]:
                        pruned.append(active.pop()["step"])
                rollbacks += 1
                lr *= cfg["lr_decay_on_rollback"]
                rec["action"] = (f"ROLLBACK->{best['ckpt']} (best={best['score']}); "
                                 f"pruned_neutral={pruned}; lr->{lr:g}; rollback#{rollbacks}")
                since_improve = 0
                step = best["step"]  # resume FROM the best checkpoint, not the bad tail
                if rollbacks >= cfg["max_rollbacks"]:
                    stop_reason = "early_stop_max_rollbacks"
                    lineage.append(rec)
                    if verbose:
                        print(f"eval@{rec['step']:4d}  score {score:.4f}  lr {lr:.2e}  "
                              f"best {best['score']:.4f}  {rec['action']}")
                    break

        lineage.append(rec)
        if verbose:
            # rec['step'] is the step the eval ran at (the loop may have just rewound
            # `step` to best on a rollback), so print rec['step'] to avoid confusion.
            print(f"eval@{rec['step']:4d}  score {score:.4f}  lr {lr:.2e}  "
                  f"best {best['score']:.4f}  {rec['action'] or ''}")

    result = {
        "stop_reason": stop_reason,
        "best": best,
        "rollbacks": rollbacks,
        "final_active_checkpoints": [r["step"] for r in active],
        "evals": len(lineage),
        "config": cfg,
        "lineage": lineage,
    }
    return result


def main():
    ap = argparse.ArgumentParser(description="Eval-in-loop training with rollback (simulated).")
    ap.add_argument("--config", help="JSON config overrides")
    ap.add_argument("--out", default=os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                                   "_run_lineage.json"))
    ap.add_argument("--quiet", action="store_true")
    a = ap.parse_args()
    cfg = json.load(open(a.config)) if a.config else None
    res = run(cfg, verbose=not a.quiet)
    json.dump(res, open(a.out, "w"), indent=1)
    b = res["best"]
    print("-" * 60)
    print(f"stop: {res['stop_reason']}  best_score={b['score']:.4f} @ step {b['step']} "
          f"({b['ckpt']})  rollbacks={res['rollbacks']}  evals={res['evals']}")
    print(f"kept checkpoints (neutral tails pruned): {res['final_active_checkpoints']}")
    print(f"lineage -> {os.path.relpath(a.out, common.REPO_ROOT)}")


if __name__ == "__main__":
    main()
