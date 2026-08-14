#!/usr/bin/env python3
"""
graph.py — any-to-any weighted DAG orchestration with summarizer + token-budget cap.

Grounded in the 'orch' specialist (phase_b/specialists/orch.specialist.json) and the
neutralized brief (IDEA_NEUTRALIZED.md): "models not in series, not parallel, but
3D/4D — any-to-any, weighted, generate by token budget" + "a summarizer model when
many outputs exceed an input".

This is a generic weighted-DAG engine, not a fixed pipeline:
  * Nodes of several kinds (source / specialist / summarizer / aggregator).
  * Edges carry a WEIGHT; a node aggregates its weighted upstream context.
  * A SUMMARIZER node compresses combined upstream context when it exceeds an input
    token budget (bounds inter-agent context blow-up).
  * A global TOKEN-BUDGET CAP stops adding specialist contribution at the
    value-saturation point (diminishing-returns rule), and caps final output length.

Runs offline on backends.MockBackend by default; swap in a real ModelBackend to make
each specialist node a served fine-tuned checkpoint. Cycles are rejected (it's a DAG).
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402
import backends  # noqa: E402
from orch import router as _router  # noqa: E402


# --------------------------------------------------------------------------- nodes
class Node:
    kind = "node"

    def __init__(self, node_id):
        self.id = node_id

    def run(self, need, inbox, ctx):
        """inbox: list of (weight, from_id, text). Returns this node's output text."""
        raise NotImplementedError


class SourceNode(Node):
    kind = "source"

    def __init__(self, node_id, text):
        super().__init__(node_id)
        self.text = text

    def run(self, need, inbox, ctx):
        return self.text


class SpecialistNode(Node):
    kind = "specialist"

    def __init__(self, node_id, specialist_id, backend):
        super().__init__(node_id)
        self.specialist_id = specialist_id
        self.backend = backend

    def run(self, need, inbox, ctx):
        spec = common.load_specialist(self.specialist_id)
        # System prompt = the specialist's grounded role + first decision steps.
        steps = spec.get("decision_procedure", [])[:4]
        system = (f"specialist={self.specialist_id} ({spec.get('domain_label','')}). "
                  f"role={spec.get('role','')[:400]} decision={'; '.join(map(str, steps))[:400]}")
        context = _weighted_context(inbox)
        prompt = f"NEED: {need}\nUPSTREAM(weighted):\n{context}\nProduce a dense, persona-free answer."
        return self.backend.generate(prompt, system=system, max_tokens=ctx["node_cap"])


class SummarizerNode(Node):
    """Compresses combined upstream context to a token budget when it overflows.
    Grounded in distill/orch: bound inter-agent context so a fan-in doesn't exceed
    a downstream node's input window."""
    kind = "summarizer"

    def __init__(self, node_id, backend, input_budget_tokens):
        super().__init__(node_id)
        self.backend = backend
        self.input_budget = input_budget_tokens

    def run(self, need, inbox, ctx):
        context = _weighted_context(inbox)
        if common.est_tokens(context) <= self.input_budget:
            ctx["trace"].append(f"{self.id}: passthrough ({common.est_tokens(context)} <= {self.input_budget} tok)")
            return context
        ctx["trace"].append(f"{self.id}: COMPRESS {common.est_tokens(context)} -> ~{self.input_budget} tok")
        prompt = (f"Compress the following multi-agent context to <= {self.input_budget} tokens, "
                  f"preserving claims relevant to: {need}\n{context}")
        return self.backend.generate(prompt, max_tokens=self.input_budget)


class AggregatorNode(Node):
    """Weighted synthesis of upstream outputs into the final answer, under the
    output token cap (value-saturation point)."""
    kind = "aggregator"

    def __init__(self, node_id, backend):
        super().__init__(node_id)
        self.backend = backend

    def run(self, need, inbox, ctx):
        # Saturation rule: include contributors by descending weight until marginal
        # token cost stops buying value (cap reached). Documents what was dropped.
        ranked = sorted(inbox, key=lambda m: -m[0])
        kept, used, dropped = [], 0, []
        cap = ctx["output_cap"]
        for w, src, text in ranked:
            cost = common.est_tokens(text)
            if used + cost > cap and kept:  # saturated
                dropped.append(src)
                continue
            kept.append((w, src, text))
            used += cost
        if dropped:
            ctx["trace"].append(f"{self.id}: saturated at {used} tok; dropped {dropped}")
        context = _weighted_context(kept)
        prompt = (f"Synthesize a single weighted answer to: {need}\n"
                  f"Weighted specialist outputs:\n{context}\nReturn the dense synthesis only.")
        return self.backend.generate(prompt, max_tokens=cap)


def _weighted_context(inbox):
    # Higher weight => listed first and labeled with its weight (prominence).
    parts = []
    for w, src, text in sorted(inbox, key=lambda m: -m[0]):
        parts.append(f"[w={w:.2f} <- {src}] {text}")
    return "\n".join(parts)


# --------------------------------------------------------------------------- graph
class Graph:
    def __init__(self):
        self.nodes = {}
        self.edges = []          # (from_id, to_id, weight)

    def add(self, node):
        self.nodes[node.id] = node
        return node.id

    def link(self, src, dst, weight=1.0):
        if src not in self.nodes or dst not in self.nodes:
            raise KeyError(f"link {src}->{dst}: unknown node")
        self.edges.append((src, dst, float(weight)))

    def _toposort(self):
        indeg = {n: 0 for n in self.nodes}
        adj = {n: [] for n in self.nodes}
        for s, d, _ in self.edges:
            indeg[d] += 1
            adj[s].append(d)
        ready = [n for n, k in indeg.items() if k == 0]
        order = []
        while ready:
            n = ready.pop()
            order.append(n)
            for m in adj[n]:
                indeg[m] -= 1
                if indeg[m] == 0:
                    ready.append(m)
        if len(order) != len(self.nodes):
            raise ValueError("orchestration graph has a cycle (must be a DAG)")
        return order

    def run(self, need, output_cap=400, node_cap=200):
        ctx = {"output_cap": output_cap, "node_cap": node_cap, "trace": []}
        incoming = {n: [] for n in self.nodes}
        for s, d, w in self.edges:
            incoming[d].append((s, w))
        out = {}
        for n in self._toposort():
            inbox = [(w, s, out[s]) for (s, w) in incoming[n]]
            out[n] = self.nodes[n].run(need, inbox, ctx)
            ctx["trace"].append(f"{n} [{self.nodes[n].kind}] -> {common.est_tokens(out[n])} tok")
        return out, ctx


# --------------------------------------------------------- default graph from routing
def build_default_graph(need, backend=None, input_budget=300, weights=None):
    """Construct a runnable any-to-any graph from a routing decision:
      source(need) -> each selected specialist -> summarizer -> aggregator.
    Edge weights default to each specialist's routing score (relevance-weighted
    aggregation). Pass `weights={specialist_id: w}` to override."""
    backend = backend or backends.get_backend("mock")
    decision = _router.route(need)
    g = Graph()
    src = g.add(SourceNode("need", need))
    summ = SummarizerNode("summarizer", backend, input_budget)
    agg = AggregatorNode("aggregate", backend)

    score_by_id = {sid: sc for sid, sc, _, _ in decision["scores"]}
    selected = decision["selected"] or [decision["scores"][0][0]]  # fallback: top scorer
    for sid in selected:
        nid = g.add(SpecialistNode(f"spec::{sid}", sid, backend))
        g.link(src, nid, weight=1.0)
        w = (weights or {}).get(sid, score_by_id.get(sid, 1))
        g.add(summ) if summ.id not in g.nodes else None
        g.link(nid, summ.id, weight=float(w))
    g.add(agg)
    g.link(summ.id, agg.id, weight=1.0)
    # any-to-any demonstration: also feed the raw need to the aggregator (skip edge),
    # so the final synthesis can ground against the original ask, not only the summary.
    g.link(src, agg.id, weight=0.5)
    return g, decision


if __name__ == "__main__":
    need = " ".join(sys.argv[1:]) or \
        "fine-tune a small model with LoRA and roll back if the eval score stalls"
    g, decision = build_default_graph(need, input_budget=120)
    out, ctx = g.run(need, output_cap=300, node_cap=150)
    print(f"NEED: {need}")
    print(f"ROUTE: {decision['mode']} -> {decision['selected']}")
    print("TRACE:")
    for t in ctx["trace"]:
        print("  " + t)
    print("\nFINAL (aggregate):")
    print("  " + out["aggregate"][:500])
