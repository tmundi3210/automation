#!/usr/bin/env python3
"""
pdf_to_chunks.py — source document -> token-bounded JSONL chunks.

First stage of the distillation pipeline (IDEA_NEUTRALIZED.md: "find academic
books/PDFs, break them into chunks, agent per chunk -> dense KB"). Grounded in the
'distill' specialist.

Input : files under phase_b/sources/ (or a path arg). .txt / .md read natively;
        .pdf via an OPTIONAL extractor (pypdf) — if absent, prints a clear hook
        message instead of failing the whole run.
Output: <out>.jsonl, one record per chunk:
        {source, chunk_id, ordinal, est_tokens, text}

Chunking respects paragraph boundaries, targets a token budget, and overlaps
consecutive chunks so a claim split across a boundary survives. Stdlib only.
"""
import argparse
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import common  # noqa: E402


def extract_text(path):
    ext = os.path.splitext(path)[1].lower()
    if ext in (".txt", ".md", ".json", ".jsonl", ".text"):
        with open(path, encoding="utf-8", errors="replace") as f:
            return f.read()
    if ext == ".pdf":
        try:
            import pypdf  # optional dependency
        except ImportError:
            print(f"  [skip] {os.path.basename(path)}: PDF support needs `pip install pypdf`. "
                  f"Convert to .txt/.md or install the extractor (HOOK).", file=sys.stderr)
            return None
        reader = pypdf.PdfReader(path)
        return "\n\n".join((p.extract_text() or "") for p in reader.pages)
    print(f"  [skip] {os.path.basename(path)}: unsupported extension {ext}", file=sys.stderr)
    return None


def chunk_text(text, target_tokens=800, overlap_tokens=100):
    """Greedy paragraph packer with token-budgeted chunks + overlap tail."""
    paras = [p.strip() for p in text.replace("\r\n", "\n").split("\n\n") if p.strip()]
    chunks, cur, cur_tok = [], [], 0
    for p in paras:
        pt = common.est_tokens(p)
        if cur and cur_tok + pt > target_tokens:
            chunks.append("\n\n".join(cur))
            # carry an overlap tail (last paragraphs up to overlap budget)
            tail, ttok = [], 0
            for q in reversed(cur):
                qt = common.est_tokens(q)
                if ttok + qt > overlap_tokens:
                    break
                tail.insert(0, q)
                ttok += qt
            cur, cur_tok = list(tail), ttok
        cur.append(p)
        cur_tok += pt
    if cur:
        chunks.append("\n\n".join(cur))
    return chunks


def iter_sources(path):
    if os.path.isfile(path):
        yield path
    elif os.path.isdir(path):
        for root, _, files in os.walk(path):
            for fn in sorted(files):
                if not fn.startswith("."):
                    yield os.path.join(root, fn)


def run(src_path, out_path, target_tokens=800, overlap_tokens=100):
    records, n_src = [], 0
    for src in iter_sources(src_path):
        text = extract_text(src)
        if not text or not text.strip():
            continue
        n_src += 1
        rel = os.path.relpath(src, common.REPO_ROOT)
        for i, ch in enumerate(chunk_text(text, target_tokens, overlap_tokens)):
            records.append({"source": rel, "chunk_id": f"{os.path.basename(src)}#{i:04d}",
                            "ordinal": i, "est_tokens": common.est_tokens(ch), "text": ch})
    os.makedirs(os.path.dirname(os.path.abspath(out_path)), exist_ok=True)
    with open(out_path, "w") as f:
        for r in records:
            f.write(json.dumps(r, ensure_ascii=False) + "\n")
    return n_src, records


DEMO_SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_demo_source.txt")


if __name__ == "__main__":
    ap = argparse.ArgumentParser(description="Break source docs into JSONL chunks.")
    ap.add_argument("src", nargs="?", default=common.SOURCES_DIR,
                    help="file or dir of sources (default: phase_b/sources/)")
    ap.add_argument("--out", default=os.path.join(common.SOURCES_DIR, "_chunks.jsonl"))
    ap.add_argument("--target-tokens", type=int, default=800)
    ap.add_argument("--overlap-tokens", type=int, default=100)
    a = ap.parse_args()

    has_sources = os.path.isdir(a.src) and any(
        not f.startswith(".") for f in (os.listdir(a.src) if os.path.isdir(a.src) else []))
    if not has_sources and a.src == common.SOURCES_DIR:
        # offline self-demo against a committed fixture; leaves phase_b/sources/ pristine
        a.src = DEMO_SOURCE
        print(f"no user sources in phase_b/sources/; using demo fixture "
              f"{os.path.relpath(DEMO_SOURCE, common.REPO_ROOT)}")

    n_src, records = run(a.src, a.out, a.target_tokens, a.overlap_tokens)
    print(f"sources: {n_src} | chunks: {len(records)} | out: {os.path.relpath(a.out, common.REPO_ROOT)}")
    for r in records[:3]:
        print(f"  {r['chunk_id']}  {r['est_tokens']} tok  {r['text'][:60].replace(chr(10),' ')}...")
