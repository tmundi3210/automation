#!/usr/bin/env python3
"""
backends.py — pluggable model backend for the Phase B scaffold.

Every subsystem that "calls a model" (orchestration nodes, the LLM-as-judge in the
eval harness, the chunk->KB distiller, the dense<->human renderer) goes through the
ModelBackend interface. This keeps the scaffold runnable in a CPU-only container:
the default MockBackend is deterministic and offline. The real backends are thin,
clearly-marked HOOK POINTS — fill in an API key or a local model path and the rest
of the scaffold is unchanged.

Boundary note (see phase_b/IDEA_NEUTRALIZED.md §5): the "steer"/format-control path
is for terse, persona-free, token-dense machine-to-machine I/O and neutral rendering.
It is NOT a guardrail-removal mechanism. The MockBackend does plain text transforms;
real backends must be used with their providers' safety policies intact.

Stdlib only. Choose a backend with get_backend(name): "mock" (default), "api", "local".
"""
import hashlib
import json
import os
import textwrap


class ModelBackend:
    """Minimal text-in/text-out contract shared by all scaffold subsystems."""

    name = "base"

    def generate(self, prompt, *, system=None, max_tokens=512, temperature=0.0, stop=None):
        raise NotImplementedError

    def score(self, prompt, *, system=None):
        """Return a float in [0,1] (used by the LLM-as-judge). Default: parse a
        leading number out of a short generation."""
        out = self.generate(prompt, system=system, max_tokens=16, temperature=0.0)
        for tok in out.replace(",", " ").split():
            try:
                v = float(tok)
                return max(0.0, min(1.0, v if v <= 1.0 else v / 100.0))
            except ValueError:
                continue
        return 0.0


class MockBackend(ModelBackend):
    """Deterministic, offline. Good enough to exercise every code path end-to-end
    and to make demos reproducible. Generations are derived from the prompt so the
    same input always yields the same output (important for testable pipelines)."""

    name = "mock"

    def __init__(self, persona_free=True):
        self.persona_free = persona_free  # emit terse, machine-facing text (steer boundary)

    def _seed(self, *parts):
        h = hashlib.sha256("||".join(str(p) for p in parts).encode()).hexdigest()
        return int(h[:8], 16)

    def generate(self, prompt, *, system=None, max_tokens=512, temperature=0.0, stop=None):
        # Deterministic "summary-ish" transform: pick salient lines, prefix with a tag.
        words = [w for w in prompt.split() if w]
        budget_words = max(8, max_tokens // 2)
        head = " ".join(words[:budget_words])
        tag = "MOCK" if self.persona_free else "Here is a response"
        text = f"[{tag}] {head}"
        if stop:
            for s in stop:
                if s in text:
                    text = text.split(s)[0]
        return text.strip()

    def score(self, prompt, *, system=None):
        # Deterministic pseudo-score in [0,1] keyed on the prompt; stable across runs.
        return round((self._seed(system or "", prompt) % 1000) / 1000.0, 3)


class ApiBackend(ModelBackend):
    """HOOK POINT — real hosted model (e.g. Anthropic Messages API).

    Intentionally not wired to a network call in the scaffold. To enable:
      1. pip install the provider SDK,
      2. set the API key env var,
      3. implement generate() per the provider docs (see the `claude-api` skill for
         the current Claude model ids, params, streaming, and tool-use shapes).
    Until then it raises, so nothing silently makes paid calls."""

    name = "api"

    def __init__(self, model=None, api_key_env="ANTHROPIC_API_KEY"):
        self.model = model
        self.api_key = os.environ.get(api_key_env)

    def generate(self, prompt, *, system=None, max_tokens=512, temperature=0.0, stop=None):
        raise NotImplementedError(
            "ApiBackend is a hook point. Provide an SDK + key and implement generate(). "
            "Run the scaffold with backend='mock' for an offline demo.")


class LocalBackend(ModelBackend):
    """HOOK POINT — local/served fine-tuned specialist (vLLM/TGI/transformers).

    This is where a per-specialist fine-tuned checkpoint produced by the
    train/rollback loop would be loaded and served. Left unimplemented so the
    scaffold has zero heavy dependencies; the orchestrator treats it like any
    other ModelBackend."""

    name = "local"

    def __init__(self, model_path=None, endpoint=None):
        self.model_path = model_path
        self.endpoint = endpoint

    def generate(self, prompt, *, system=None, max_tokens=512, temperature=0.0, stop=None):
        raise NotImplementedError(
            "LocalBackend is a hook point for a served fine-tuned checkpoint "
            "(vLLM/TGI/transformers). Run with backend='mock' for an offline demo.")


_REGISTRY = {"mock": MockBackend, "api": ApiBackend, "local": LocalBackend}


def get_backend(name="mock", **kwargs):
    name = (name or "mock").lower()
    if name not in _REGISTRY:
        raise ValueError(f"unknown backend {name!r}; choose from {sorted(_REGISTRY)}")
    return _REGISTRY[name](**kwargs)


if __name__ == "__main__":
    b = get_backend("mock")
    print("backend:", b.name)
    print(textwrap.shorten(b.generate("summarize the fine-tuning rollback policy in dense form",
                                      max_tokens=40), width=100))
    print("score:", b.score("is this answer correct? expected=4 got=4"))
