#!/usr/bin/env python3
"""
dedup.py — URL canonicalization + SimHash near-duplicate collapse -> canonical event_id.

Grounded in the `ir` data-layer design (BUILD.md stage 1): "cross-post dedup
(URL-canonicalize -> SimHash/MinHash -> embedding ANN -> canonical event_id)". The
embedding-ANN tier is a HOOK POINT; the URL + SimHash tiers are real and run here.

Stdlib only (hashlib for the 64-bit SimHash). Deterministic.
"""
import hashlib
import re

_TRACKING = re.compile(r"^(utm_|ref$|fbclid$|gclid$|igshid$)")


def canonical_url(url):
    """Lowercase host, drop scheme/www/fragment/trailing slash, strip tracking params."""
    u = (url or "").strip().lower()
    u = re.sub(r"^https?://", "", u)
    u = u.split("#", 1)[0]
    if "?" in u:
        base, query = u.split("?", 1)
        kept = [kv for kv in query.split("&")
                if kv and not _TRACKING.match(kv.split("=", 1)[0])]
        u = base + ("?" + "&".join(sorted(kept)) if kept else "")
    if u.startswith("www."):
        u = u[4:]
    if u.endswith("/"):
        u = u[:-1]
    return u


def _features(text, k=1):
    """Bag-of-token features for SimHash. k=1 (unigrams) is robust to word reordering,
    so paraphrases of the same headline collapse; k>1 catches verbatim copy-paste."""
    toks = re.findall(r"[a-z0-9]+", (text or "").lower())
    if k == 1:
        return toks
    if len(toks) < k:
        return [" ".join(toks)] if toks else []
    return [" ".join(toks[i:i + k]) for i in range(len(toks) - k + 1)]


def simhash64(text):
    """64-bit SimHash over token features. Near-identical text -> near-identical hash."""
    v = [0] * 64
    for sh in _features(text):
        h = int(hashlib.sha1(sh.encode()).hexdigest()[:16], 16)
        for b in range(64):
            v[b] += 1 if (h >> b) & 1 else -1
    out = 0
    for b in range(64):
        if v[b] > 0:
            out |= (1 << b)
    return out


def hamming(a, b):
    return bin(a ^ b).count("1")


def dedup(records, *, sim_threshold=10):
    """Assign a canonical event_id per record and collapse near-duplicates.

    A record duplicates an earlier one if its canonical primary URL matches OR its
    SimHash (over name + what_it_is) is within `sim_threshold` Hamming bits. Returns
    (events, report) where each surviving record gains `event_id` and `_dup_of`/None.
    """
    seen = []  # (event_id, canon_url, simhash)
    out, merged = [], 0
    for rec in records:
        urls = [canonical_url(u) for u in rec.get("source_urls", [])]
        primary = urls[0] if urls else rec.get("_id", "")
        text = f"{rec.get('item', {}).get('canonical_name', '')} {rec.get('item', {}).get('what_it_is', '')}"
        sh = simhash64(text)
        dup_of = None
        for eid, curl, csh in seen:
            if (primary and primary == curl) or hamming(sh, csh) <= sim_threshold:
                dup_of = eid
                break
        if dup_of:
            merged += 1
            rec = dict(rec, event_id=dup_of, _dup_of=dup_of)
        else:
            eid = "EV_" + re.sub(r"[^A-Z0-9]+", "_", rec.get("_id", primary).upper()).strip("_")
            seen.append((eid, primary, sh))
            rec = dict(rec, event_id=eid, _dup_of=None)
        out.append(rec)
    report = {"in": len(records), "events": len(seen), "near_dups_merged": merged}
    return out, report


if __name__ == "__main__":
    # Visible demonstration that the SimHash tier actually merges paraphrases.
    a = "US tightens the H-1B specialty occupation rule in 2025"
    b = "In 2025 the US tightens its H-1B specialty-occupation rule"
    c = "Punjabi singer drops a new bhangra track for the summer"
    print("canon:", canonical_url("HTTPS://www.News.org/x/?utm_source=rss&id=9#top"))
    print(f"hamming(a,b)={hamming(simhash64(a), simhash64(b))}  (paraphrase, expect small)")
    print(f"hamming(a,c)={hamming(simhash64(a), simhash64(c))}  (unrelated, expect large)")
    recs = [{"_id": "a", "source_urls": ["https://n.org/h1b"], "item": {"canonical_name": a, "what_it_is": ""}},
            {"_id": "b", "source_urls": ["https://n.org/h1b-2"], "item": {"canonical_name": b, "what_it_is": ""}},
            {"_id": "c", "source_urls": ["https://n.org/song"], "item": {"canonical_name": c, "what_it_is": ""}}]
    _, rep = dedup(recs)
    print("dedup report:", rep)
