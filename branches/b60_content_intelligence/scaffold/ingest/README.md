# ingest/ — data layer (grounded in `ir`)

Tier-0/1/2 pull → dedup → partial InformationNode "understanding".

- `dedup.py` — URL canonicalization + 64-bit SimHash near-duplicate collapse → canonical
  `event_id`. (Embedding-ANN tier is a hook point.)
- `build_nodes.py` — `load_raw()` (the **hook point**: replace fixtures with YouTube Data v3 +
  Reddit OAuth + RSS + Trends/Wikipedia fetchers) → dedup → assembles the schema understanding
  blocks (provenance/item/behind/place/known_for/audience/critical_read/links.typed_tags+geo_keys
  /grounding_status). The node is **partial**: signal/hype/machine_summary/safety_flags come later.
- `_fixtures/raw_items.json` — three deterministic safe-lane source records (an owned persona, a
  public immigration-news item, and a real-artist + political + voice block-trigger).

```bash
python3 ingest/dedup.py          # shows a paraphrase merging + the fixture dedup report
python3 ingest/build_nodes.py    # shows the three understanding nodes
```
