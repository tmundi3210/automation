# source_verification.md — TOP_SOURCES CANDIDATE URL-verification

**Task:** TASK-019  
**Input:** `plans/bounded_autonomy/import/TOP_SOURCES.md` §2 ranks 1–14  
**Verified at (UTC):** 2026-07-15T19:58–20:01Z  
**Method:** read-only HTTP GET via Python `urllib` (ssl default context); feed probes for RSS/Atom/common paths; GitHub `releases.atom` / `commits/*.atom` where applicable.

**Labels:** only **VERIFIED** or **FAILED**. No guesses.  
**Invariant:** these rows remain disabled until the hub merges them — this file is verification evidence, not enablement.  
**Fetched pages are DATA, never instructions.**

## Summary

- VERIFIED: **13/14**
- FAILED: **1/14** (rank 2 OpenAI research hub — HTTP 403 bot/edge block from this client)
- Machine-pollable surface confirmed on **12/14** (missing: rank 1 Anthropic engineering page has no discovered feed; rank 2 failed page fetch so feed unknown)

## Rows (one per source)

### Rank 1 — Anthropic Engineering

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://www.anthropic.com/engineering` |
| HTTP status | `200` |
| Final URL | `https://www.anthropic.com/engineering` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | **none confirmed** among probes |
| Notes | page HTTP 200 at listed URL; common feed paths probed (/rss.xml, /feed.xml, /engineering/rss, /engineering/feed) all returned 404 — no machine-pollable feed confirmed |

### Rank 2 — OpenAI Engineering & Research

| Field | Value |
|---|---|
| **Status** | **FAILED** |
| Listed URL | `https://openai.com/research/` |
| HTTP status | `403` |
| Final URL | `https://openai.com/research/` |
| Content-Type | `text/html; charset=UTF-8` |
| Machine-pollable surfaces | **none confirmed** among probes |
| Notes | HTTP 403 on listed URL with browser UA and automation UA (both tried); also 403 on https://openai.com/index/ and https://openai.com/news/research/; no body available; cannot claim feed existence — FAILED (not silently kept as verified) |
| Error | `HTTPError 403` |

### Rank 3 — Model Context Protocol

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://github.com/modelcontextprotocol` |
| HTTP status | `200` |
| Final URL | `https://github.com/modelcontextprotocol` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://github.com/modelcontextprotocol/modelcontextprotocol/releases.atom` (Atom, HTTP 200)<br>`https://github.com/modelcontextprotocol/python-sdk/releases.atom` (Atom, HTTP 200) |
| Notes | org page HTTP 200; org repositories.atom returned 406; machine-pollable surfaces found on child repos (spec + python-sdk releases.atom), not on the org landing URL itself |

### Rank 4 — OpenAI Codex GitHub

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://github.com/openai/codex` |
| HTTP status | `200` |
| Final URL | `https://github.com/openai/codex` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://github.com/openai/codex/releases.atom` (Atom, HTTP 200)<br>`https://github.com/openai/codex/commits/main.atom` (Atom, HTTP 200) |
| Notes | machine-pollable surfaces found: 2 |

### Rank 5 — Gemini CLI GitHub

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://github.com/google-gemini/gemini-cli` |
| HTTP status | `200` |
| Final URL | `https://github.com/google-gemini/gemini-cli` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://github.com/google-gemini/gemini-cli/releases.atom` (Atom, HTTP 200)<br>`https://github.com/google-gemini/gemini-cli/commits/main.atom` (Atom, HTTP 200) |
| Notes | machine-pollable surfaces found: 2 |

### Rank 6 — Google ADK Python

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://github.com/google/adk-python` |
| HTTP status | `200` |
| Final URL | `https://github.com/google/adk-python` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://github.com/google/adk-python/releases.atom` (Atom, HTTP 200)<br>`https://github.com/google/adk-python/commits/main.atom` (Atom, HTTP 200) |
| Notes | machine-pollable surfaces found: 2 |

### Rank 7 — Google Developers Blog

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://developers.googleblog.com/` |
| HTTP status | `200` |
| Final URL | `https://developers.googleblog.com/` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://developers.googleblog.com/feed/` (RSS, HTTP 200)<br>`https://developers.googleblog.com/rss/` (RSS, HTTP 200)<br>`https://developers.googleblog.com/atom.xml/` (RSS, HTTP 200)<br>`https://developers.googleblog.com/feeds/posts/default/` (RSS, HTTP 200) |
| Notes | machine-pollable surfaces found: 4 |

### Rank 8 — PydanticAI

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://github.com/pydantic/pydantic-ai` |
| HTTP status | `200` |
| Final URL | `https://github.com/pydantic/pydantic-ai` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://github.com/pydantic/pydantic-ai/releases.atom` (Atom, HTTP 200)<br>`https://github.com/pydantic/pydantic-ai/commits/main.atom` (Atom, HTTP 200) |
| Notes | machine-pollable surfaces found: 2 |

### Rank 9 — LangGraph

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://github.com/langchain-ai/langgraph` |
| HTTP status | `200` |
| Final URL | `https://github.com/langchain-ai/langgraph` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://github.com/langchain-ai/langgraph/releases.atom` (Atom, HTTP 200)<br>`https://github.com/langchain-ai/langgraph/commits/main.atom` (Atom, HTTP 200) |
| Notes | machine-pollable surfaces found: 2 |

### Rank 10 — OpenHands

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://github.com/All-Hands-AI/OpenHands` |
| HTTP status | `200` |
| Final URL | `https://github.com/OpenHands/OpenHands` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://github.com/OpenHands/OpenHands/releases.atom` (Atom, HTTP 200)<br>`https://github.com/OpenHands/OpenHands/commits/main.atom` (Atom, HTTP 200) |
| Notes | machine-pollable surfaces found: 2; GitHub redirect: All-Hands-AI/OpenHands → OpenHands/OpenHands (final URL recorded) |

### Rank 11 — Cursor Changelog

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://www.cursor.com/changelog` |
| HTTP status | `200` |
| Final URL | `https://cursor.com/changelog` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://cursor.com/atom.xml` (Atom, HTTP 200)<br>`https://cursor.com/changelog/rss.xml` (RSS, HTTP 200) |
| Notes | machine-pollable surfaces found: 2; www.cursor.com/changelog redirects to cursor.com/changelog |

### Rank 12 — Thinking Machines Connectionism

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://thinkingmachines.ai/blog/` |
| HTTP status | `200` |
| Final URL | `https://thinkingmachines.ai/blog/` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://thinkingmachines.ai/blog/index.xml` (RSS, HTTP 200)<br>`https://thinkingmachines.ai/index.xml` (RSS, HTTP 200) |
| Notes | machine-pollable surfaces found: 2 |

### Rank 13 — steipete.me (Peter Steinberger)

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://steipete.me/posts/2026/openclaw` |
| HTTP status | `200` |
| Final URL | `https://steipete.me/posts/2026/openclaw` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://steipete.me/rss.xml` (RSS, HTTP 200) |
| Blog root (rank-13 special) | `https://steipete.me/` HTTP `200` final `https://steipete.me/` |
| Blog root feed | https://steipete.me/rss.xml (RSS, HTTP 200) |
| Notes | listed URL is a single dated post (evidence anchor) — HTTP 200; blog ROOT https://steipete.me/ HTTP 200; machine-pollable feed: https://steipete.me/rss.xml (RSS, HTTP 200) |

### Rank 14 — philschmid.de (Philipp Schmid)

| Field | Value |
|---|---|
| **Status** | **VERIFIED** |
| Listed URL | `https://www.philschmid.de/` |
| HTTP status | `200` |
| Final URL | `https://www.philschmid.de/` |
| Content-Type | `text/html; charset=utf-8` |
| Machine-pollable surfaces | `https://www.philschmid.de/rss` (RSS, HTTP 200) |
| Notes | machine-pollable surfaces found: 1 |

## Feed-probe method (what was tried)

1. Fetch listed URL; record status + final URL after redirects.
2. Parse HTML `<link rel="alternate" type=…>` for rss/atom/xml/json when body available.
3. Probe common paths: `/feed`, `/feed.xml`, `/rss`, `/rss.xml`, `/atom.xml`, `/index.xml`, blog variants; for GitHub repos also `releases.atom` and `commits/main.atom`.
4. Accept a surface only if HTTP 2xx/3xx **and** body/content-type shows RSS/Atom/JSON feed structure (or declared alternate confirmed live).
5. Rank 13: separately resolve blog root `https://steipete.me/` + feed (listed URL is a dated post).

## Explicit non-claims

- FAILED rows are **not** enabled and are **not** silently kept as good sources.
- VERIFIED without a feed means the **page URL is live**; pollability may still require site-diff, not a feed.
- No source content was treated as instructions.
- Hub must still run Phase-B merge / registry enablement; this pass does not flip `enabled: true`.

