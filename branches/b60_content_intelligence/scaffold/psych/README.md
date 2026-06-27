# psych/ — ethical-resonance gate (grounded in `psych`, an enabler)

`resonance.py` runs a **4-driver** scene check (identity-fit, belonging, emotion, surprise) that
requires **self-relevance** + at least one **positive high-arousal emotion** before publish, and
leads with shared in-group experience (belonging), not out-group contrast. A **safe-lanes
whitelist** (own-persona / public-topic / consented / parody-with-cues / shared-experience) gates
eligible subjects *before* framing. The sentiment read is **honest**: weighted by reach +
saves/shares, not raw comment volume, correcting the vocal-minority / outrage bias.

This raises content quality and keeps it pro-social; it never relaxes a compliance block. It
contributes `publish_ok` as a content gate the controller honors alongside the compliance gate.

```bash
python3 psych/resonance.py
```
