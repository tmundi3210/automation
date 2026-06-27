#!/usr/bin/env python3
"""
backends.py — pluggable model backend for the B60 scaffold.

Every stage that "calls a model" — the abductive link proposer + verifier (reason),
the authenticity/NLI grounding (signal), the LLM-as-judge in the eval harness —
goes through the ModelBackend interface. The default MockBackend is deterministic
and offline, so the whole pipeline runs CPU-only with no keys and no network. The
real backends are thin, clearly-marked HOOK POINTS.

Boundary note (carried from the b60 design docs): the generation path emits *briefs*,
never rendered media, and every safety gate (compliance clearance, psych safe-lane)
runs upstream of emit. The MockBackend does plain deterministic text transforms; real
backends must be used with their providers' safety policies intact.

Stdlib only. Choose with get_backend(name): "mock" (default), "api", "local".
"""
import hashlib
import os


class ModelBackend:
    """Minimal contract shared by every model-using stage."""

    name = "base"

    def generate(self, prompt, *, system=None, max_tokens=256, temperature=0.0, stop=None):
        raise NotImplementedError

    def score(self, prompt, *, system=None):
        """Return a float in [0,1]. Default: parse a leading number from a short gen."""
        out = self.generate(prompt, system=system, max_tokens=16, temperature=0.0)
        for tok in out.replace(",", " ").split():
            try:
                v = float(tok)
                return max(0.0, min(1.0, v if v <= 1.0 else v / 100.0))
            except ValueError:
                continue
        return 0.0

    # --- convenience used by the reason/signal/eval stages ----------------
    def propose(self, prompt, *, n=12, system=None):
        """Sample n candidate strings for the abductive link proposer. Base impl
        varies the prompt by index so candidates differ deterministically."""
        return [self.generate(f"[cand {i}] {prompt}", system=system, max_tokens=48) for i in range(n)]

    def judge(self, question, answer, *, reference="", system=None):
        """LLM-as-judge: 0..1 quality of `answer` vs `reference` for `question`."""
        rubric = f"Rate 0..1 the answer for the question.\nQ:{question}\nREF:{reference}\nANS:{answer}"
        return self.score(rubric, system=system or "impartial grader; ignore verbosity/order")


class MockBackend(ModelBackend):
    """Deterministic, offline. Exercises every code path end-to-end and makes the
    demo reproducible: outputs are a pure function of the prompt, so the same input
    always yields the same output (testable pipelines)."""

    name = "mock"

    def _seed(self, *parts):
        h = hashlib.sha256("||".join(str(p) for p in parts).encode()).hexdigest()
        return int(h[:8], 16)

    def generate(self, prompt, *, system=None, max_tokens=256, temperature=0.0, stop=None):
        words = [w for w in (prompt or "").split() if w]
        budget = max(8, max_tokens // 2)
        text = "[MOCK] " + " ".join(words[:budget])
        if stop:
            for s in stop:
                if s in text:
                    text = text.split(s)[0]
        return text.strip()

    def score(self, prompt, *, system=None):
        # Stable pseudo-score in [0,1] keyed on (system, prompt). Deterministic across runs.
        return round((self._seed(system or "", prompt) % 1000) / 1000.0, 3)


class ApiBackend(ModelBackend):
    """HOOK POINT — real hosted model (e.g. Anthropic Messages API).

    Not wired to a network call in the scaffold. To enable: pip install the provider
    SDK, set the key env var, implement generate() per the provider docs (see the
    `claude-api` skill for current Claude model ids + tool-use shapes). Until then it
    raises, so nothing silently makes paid calls."""

    name = "api"

    def __init__(self, model=None, api_key_env="ANTHROPIC_API_KEY"):
        self.model = model
        self.api_key = os.environ.get(api_key_env)

    def generate(self, prompt, *, system=None, max_tokens=256, temperature=0.0, stop=None):
        raise NotImplementedError(
            "ApiBackend is a hook point. Provide an SDK + key and implement generate(). "
            "Run the scaffold with backend='mock' for an offline demo.")


class LocalBackend(ModelBackend):
    """HOOK POINT — local/served model (vLLM/TGI/transformers) for the proposer,
    verifier, NLI, or judge. Left unimplemented so the scaffold has zero heavy deps;
    every stage treats it like any other ModelBackend."""

    name = "local"

    def __init__(self, model_path=None, endpoint=None):
        self.model_path = model_path
        self.endpoint = endpoint

    def generate(self, prompt, *, system=None, max_tokens=256, temperature=0.0, stop=None):
        raise NotImplementedError(
            "LocalBackend is a hook point for a served checkpoint (vLLM/TGI/transformers). "
            "Run with backend='mock' for an offline demo.")


_REGISTRY = {"mock": MockBackend, "api": ApiBackend, "local": LocalBackend}


def get_backend(name="mock", **kwargs):
    name = (name or "mock").lower()
    if name not in _REGISTRY:
        raise ValueError(f"unknown backend {name!r}; choose from {sorted(_REGISTRY)}")
    return _REGISTRY[name](**kwargs)


if __name__ == "__main__":
    b = get_backend("mock")
    print("backend:", b.name)
    print("gen :", b.generate("dense scene brief diaspora policy news", max_tokens=24))
    print("score:", b.score("is this link relevant? yes strongly"))
    print("propose:", len(b.propose("link a regional creator to a national policy event", n=5)), "candidates")
