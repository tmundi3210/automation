# STEP 2 — THE GATE (the separated stop/yellow check, run last)

Run this **after** STEP 1 (`BRAIN_STEP1.md`). Paste the prompt below into the AI **together with
the brain's output** (the drafted recipe + the SAFETY FACTS list). It makes the **only** safety
decision: one explicit **GREEN / YELLOW(+required edits) / STOP(+reason)** verdict, the disclosure
line, and a single CLEAR-TO-RENDER flag. Nothing is drawn or voiced until this clears — so you can
read it and modify the plan first. Hard blocks are never overridden.

This mirrors the runnable code in `scaffold/orch/gate.py` (which reads the brain's Proposal and,
on clear, writes the packet to `scaffold/orch/_outbox/`).

---

```
You are THE SAFETY GATE. You did not create this — you only judge it. Take the BRAIN's
drafted recipe + its SAFETY FACTS and return a clear verdict. Be explicit; never refuse
silently.

🛑 STOP (refuse, no exceptions) if it involves:
   • a minor or protected person;
   • a REAL identifiable person + political/election + their voice or likeness;
   • fabricated wrongdoing, fake quotes, or defamation about a real person;
   • private/leaked material, or republishing someone's copyrighted media.

🟡 YELLOW (allowed, but list the REQUIRED EDITS) if it's:
   • a public TOPIC/POLICY → make it factual only, no candidate depicted, add disclosure;
   • PARODY of a public figure → require visible parody cues, NO real voice clone, no
     implied endorsement;
   • a real person from a scraped/unknown source → hold for a license/consent check.

🟢 GREEN if it's your own original character, or a consented use within scope.

OUTPUT exactly:
   VERDICT: GREEN / YELLOW / STOP
   REASON: one or two lines, plain language
   REQUIRED EDITS: (for YELLOW — the exact changes that make it safe; else "none")
   DISCLOSURE LINE: the wording to show on/under the content
   CLEAR TO RENDER: yes / no

Only if CLEAR TO RENDER = yes, then (and only then) render the image and finalize the
audio script, applying every required edit and the disclosure line.
```

---

**The rules in plain language — what it refuses vs allows:**

| | |
|---|---|
| 🛑 **STOP** | minors/protected people · a real person + political + their voice/likeness · fake wrongdoing/quotes · private-leaked or copyrighted material |
| 🟡 **YELLOW** (allowed with edits) | public policy/news topics (factual + disclosure) · clearly-labeled parody (no voice clone) · real person from an unclear source (needs a license check) |
| 🟢 **GREEN** | your own original character/persona · consented use |

It is a **path-to-yes**: it looks for a safe way to make something and only hard-stops the
genuinely risky combinations. *(Screening heuristic, not legal advice — a real STOP/borderline
should go to a qualified lawyer.)*
