// Build-time lesson compiler: source/lessons/*.md → src/content/content.json
// Grammar: research/PARSER_SPEC.md + research/REVIEW_FINDINGS.md (observed forms).
// Fails loudly (exit 1) if any golden count drifts.
import { readFileSync, writeFileSync, readdirSync, mkdirSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { render, inline, plain, isTableSep, parseTable } from './md.ts';
import type { Content, Lesson, Concept, NumberFact, Trap, Tyk, Flashcard, Item, GlossaryEntry, TestId, DistractorTag } from '../src/content/types.ts';

const HERE = dirname(fileURLToPath(import.meta.url));
const SRC = join(HERE, '../../source/lessons');
const OUT = join(HERE, '../src/content/content.json');
const ENRICH = join(HERE, '../../enrich');

// ---------- golden fixture (verified by independent review, see research/REVIEW_FINDINGS.md)
const GOLDEN: Record<string, [number, number, number, number, number]> = {
  // lesson: [practice, flashcards, tyk, numbers, traps]
  'GK-01': [18, 35, 0, 28, 14], 'GK-02': [15, 30, 0, 21, 12], 'GK-03': [25, 40, 0, 49, 22], 'GK-04': [19, 41, 11, 21, 13],
  'GK-05': [18, 34, 8, 25, 11], 'GK-06': [20, 36, 5, 27, 13], 'GK-07': [22, 43, 5, 30, 15], 'GK-08': [20, 30, 4, 23, 13],
  'GK-09': [16, 38, 8, 22, 13], 'GK-10': [18, 42, 5, 32, 16], 'GK-11': [17, 33, 5, 25, 14], 'GK-12': [20, 38, 7, 29, 20],
  'GK-13': [24, 42, 9, 40, 20], 'GK-14': [18, 43, 9, 34, 19], 'CV-01': [16, 35, 6, 18, 12], 'CV-02': [19, 35, 7, 19, 14],
  'CV-03': [18, 35, 4, 22, 13], 'CV-04': [15, 27, 5, 21, 10],
};

// ---------- helpers
function fnv(s: string): string {
  let h = 0x811c9dc5;
  for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 0x01000193) >>> 0; }
  return h.toString(36).padStart(7, '0').slice(-6);
}
const norm = (s: string) => plain(s).toLowerCase().replace(/[^a-z0-9]+/g, ' ').trim();
const lid = (id: string) => id.replace('-', '');

/** Page tokens from a citation string like "(pp. 2-15 – 2-16)" or a Numbers "Page" cell. Ranges expand. */
function pageTokens(s: string): string[] {
  const out: string[] = [];
  const re = /(\d{1,2})-(\d{1,2})(?:\s*(?:–|-|to)\s*(\d{1,2})-(\d{1,2}))?/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(s))) {
    const s1 = +m[1], p1 = +m[2];
    if (m[3] && +m[3] === s1 && +m[4] >= p1 && +m[4] - p1 < 20) { for (let p = p1; p <= +m[4]; p++) out.push(`${s1}-${p}`); }
    else { out.push(`${s1}-${p1}`); if (m[3]) out.push(`${m[3]}-${m[4]}`); }
  }
  return [...new Set(out)];
}
/** Citations inside "(p. …)" / "(pp. …)" / "(Figure …, p. …)" parentheticals only (never bare "20-45 psi"). */
function citePages(s: string): string[] {
  const out: string[] = [];
  const re = /\(([^()]*\bpp?\.\s[^()]*)\)/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(s))) out.push(...pageTokens(m[1].replace(/Fig(?:ure|\.)\s*\d+\.\d+/g, '')));
  return [...new Set(out)];
}
const stripTrailingCite = (s: string) => s.replace(/\s*\((?:[^()]*\bpp?\.\s[^()]*)\)\s*\.?\s*$/, '').trim();

function sections(md: string): Record<string, string> {
  const out: Record<string, string> = {};
  const parts = md.split(/^## /m);
  out['_head'] = parts[0];
  for (const p of parts.slice(1)) {
    const nl = p.indexOf('\n');
    out[p.slice(0, nl).trim()] = p.slice(nl + 1);
  }
  return out;
}
function bullets(block: string): string[] {
  return block.split('\n').filter((l) => /^- /.test(l)).map((l) => inline(l.slice(2).trim()));
}

// ---------- parse one lesson
const all: Omit<Content, 'version' | 'handbook' | 'counts' | 'handbookWay' | 'mostMissed'> = {
  lessons: [], concepts: {}, numbers: {}, traps: {}, tyk: {}, flash: {}, items: {}, glossary: [],
};
const problems: string[] = [];
const dropped: string[] = []; // enrichment items rejected by the gate (logged, excluded)

function parseLesson(file: string, md: string) {
  md = md.replace(/\r/g, '').replace(/−/g, '-');
  const h1 = md.match(/^# ((GK|CV)-(\d\d)) · (.+)$/m);
  if (!h1) throw new Error(`${file}: no H1`);
  const id = h1[1];
  const test = h1[2] as TestId;
  const meta = md.match(/^> \*\*Handbook:\*\* (.+?), pages (\d+-\d+)–(\d+-\d+) · \*\*Test:\*\* (.+?) · \*\*Exam weight:\*\* (High|Medium) · \*\*Study time:\*\* ~(\d+) min$/m);
  if (!meta) throw new Error(`${file}: meta line`);
  const S = sections(md);
  const names = Object.keys(S).filter((k) => k !== '_head');
  const want = ["What you'll be able to answer", 'Learn it', 'Numbers & terms to memorize', 'Exam traps', 'Handbook review questions — answered', 'Flashcards', 'Practice test (real-exam style)', 'One-minute recap'];
  if (names.join('|') !== want.join('|')) problems.push(`${id}: H2 order ${names.join('|')}`);

  const L: Lesson = {
    id, num: +h1[3], test, title: h1[4].trim(), handbook: meta[1], pages: [meta[2], meta[3]], weight: meta[5] as 'High' | 'Medium', minutes: +meta[6],
    objectives: bullets(S["What you'll be able to answer"]), recap: bullets(S['One-minute recap']),
    conceptIds: [], numberIds: [], trapIds: [], tykIds: [], flashIds: [], itemIds: [],
  };

  // ---- Learn it → concepts (H3) ; preamble before first H3 becomes an "intro" concept
  const learn = S['Learn it'];
  const chunks = learn.split(/^### /m);
  const pre = chunks[0].trim();
  const conceptBlocks: { heading: string; body: string }[] = [];
  if (pre) conceptBlocks.push({ heading: 'Start here', body: pre });
  for (const c of chunks.slice(1)) { const nl = c.indexOf('\n'); const body = c.slice(nl + 1).trim(); if (body) conceptBlocks.push({ heading: c.slice(0, nl).trim(), body }); } // empty section headers are skipped
  conceptBlocks.forEach((b, i) => {
    const ca = /\[CA\]/.test(b.heading);
    let h = b.heading.replace(/\s*\[CA\]\s*/g, ' ').trim();
    const pages = citePages(h);
    h = stripTrailingCite(h);
    const secM = h.match(/^(\d+(?:\.\d+)*(?:–\d+(?:\.\d+)*)?)\s+(.*)$/);
    const section = secM ? secM[1] : null;
    const title = (secM ? secM[2] : h).replace(/^—\s*/, '').replace(/\s+—\s*$/, '').trim();
    const bodyPages = pages.length ? pages : citePages(b.body).slice(0, 3);
    // core = key points. If the body has bold pseudo-headings ("**Tires** (p. 2-2)."), take the first bold fact under
    // each so every sub-topic is represented; otherwise the first bullets carrying bold facts. Never beyond-handbook notes.
    const core: string[] = [];
    const lines = b.body.split('\n');
    const isPseudo = (l: string) => /^\*\*[^*]{2,60}\*\*\s*(\([^)]*\))?\s*[.:]?\s*$/.test(l.trim()) || /^\*\*[^*]{2,60}\*\*\s*\(p{1,2}\.[^)]*\)[.:]/.test(l.trim());
    const pseudoIdx = lines.map((l, k) => (isPseudo(l) ? k : -1)).filter((k) => k >= 0);
    const factLine = (l: string) => { const m = l.match(/^\s{0,2}- (.*\*\*.+\*\*.*)$/); return m && !/beyond the handbook/i.test(l) ? m[1] : null; };
    if (pseudoIdx.length >= 2) {
      for (let q = 0; q < pseudoIdx.length && core.length < 9; q++) {
        const from = pseudoIdx[q], to = pseudoIdx[q + 1] ?? lines.length;
        const head = lines[from].trim().match(/^\*\*([^*]+)\*\*/)![1].replace(/[.:]$/, '');
        const inline_ = lines[from].trim().replace(/^\*\*[^*]+\*\*\s*(\([^)]*\))?\s*[.:]?\s*/, '');
        const fact = lines.slice(from + 1, to).map(factLine).find(Boolean) || (/\*\*/.test(inline_) ? inline_ : null) || lines.slice(from + 1, to).find((l) => /^- /.test(l))?.slice(2) || inline_;
        if (fact) core.push(`<strong>${inline(stripTrailingCite(head).replace(/\s*\(pp?\.[^)]*\)\s*$/, ''))}:</strong> ${inline(stripTrailingCite(fact.replace(/^\*\*[^*]+\*\*\s*[—:-]\s*/, '')))}`);
      }
    }
    if (core.length < 2) {
      lines.forEach((line, k) => {
        if (core.length >= 6) return;
        const f = factLine(line) ?? (/^- [a-z]/.test(line) ? line.slice(2) : null);
        if (!f || line.startsWith('  ')) return;
        // a bullet that continues a sentence ("between two places…") needs its lead-in line ("Interstate means:")
        const firstOfGroup = !/^\s*- /.test(lines[k - 1] ?? '');
        if (/^[a-z]/.test(f) || firstOfGroup) {
          let j = k - 1; while (j >= 0 && (/^\s*- /.test(lines[j]) || !lines[j].trim())) j--;
          const leadLine = j >= 0 ? lines[j].trim() : '';
          const li = inline(stripTrailingCite(leadLine));
          if (leadLine.endsWith(':') && !core.includes(li)) core.push(li);
        }
        core.push(inline(stripTrailingCite(f)));
      });
    }
    if (!core.length) {
      const firstPara = lines.find((l) => l.trim() && !/^(\||>|```|\*\*[^*]+\*\*\s*$)/.test(l));
      if (firstPara) core.push(inline(stripTrailingCite(firstPara.replace(/^- /, ''))));
    }
    // lead-in sentence (e.g. "A road test is required to:") so the bullets never start mid-thought
    const lead = lines.find((l) => l.trim());
    if (lead && pseudoIdx.length < 2 && !/^(\s*[-*]|\s*\d+[.)]|\||>|```|#|\*\*[^*]+\*\*\s*$|Why:|\*Why)/.test(lead) && !/beyond the handbook/i.test(lead)) {
      const li = inline(stripTrailingCite(lead.length > 260 ? lead.slice(0, lead.lastIndexOf(' ', 250)) + '…' : lead));
      if (!core.includes(li)) core.unshift(li);
    }
    // key table / numbered step list, when bullets alone would miss the substance
    let coreExtra: string | undefined;
    const tables = lines.filter((l, k) => /^\s*\|/.test(l) && isTableSep(lines[k + 1] ?? '')).length;
    const tRows = lines.filter((l) => /^\s*\|/.test(l)).length;
    if (pseudoIdx.length < 2 && (core.length <= 3 || (tables === 1 && tRows <= 10))) {
      const tStart = lines.findIndex((l, k) => /^\s*\|/.test(l) && isTableSep(lines[k + 1] ?? ''));
      const olStart = lines.findIndex((l) => /^\d+\. /.test(l));
      if (tStart >= 0) { let e = tStart; while (e < lines.length && /^\s*\|/.test(lines[e])) e++; coreExtra = render(lines.slice(tStart, e).join('\n')); }
      else if (olStart >= 0) {
        const steps: string[] = []; for (let k = olStart; k < lines.length && steps.length < 20; k++) { const m = lines[k].match(/^\d+\. (.*)$/); if (m) steps.push(m[1]); else if (lines[k].trim() && !/^\s/.test(lines[k])) break; }
        if (steps.length >= 3) coreExtra = '<ol class="steps">' + steps.map((x) => `<li>${inline(stripTrailingCite(x.split(/(?<=[.!?])\s+(?=[A-Z(])/)[0]))}</li>`).join('') + '</ol>';
      }
      if (coreExtra) { for (let k = core.length - 1; k >= 0; k--) if (/^\d+\. /.test(core[k].replace(/<[^>]+>/g, ''))) core.splice(k, 1); }
    }
    if (!core.length && !coreExtra) {
      const para = lines.find((l) => l.trim() && !/^(\||>|```|#)/.test(l) && !/beyond the handbook/i.test(l));
      if (para) core.push(inline(stripTrailingCite(para.replace(/^\s*[-*] /, ''))));
    }
    const cid = `${id}.c${String(i + 1).padStart(2, '0')}`;
    const C: Concept = {
      id: cid, lesson: id, title, section, pages: bodyPages, ca: ca || /\[CA\]/.test(b.body.split('\n')[0] || ''), core, coreExtra,
      html: render(b.body), hasBeyond: /beyond the handbook/i.test(b.body), hasConflict: /handbook vs other|handbook conflict/i.test(b.body),
    };
    all.concepts[cid] = C; L.conceptIds.push(cid);
  });

  // ---- glossary (key-word blocks + any "**Term** = def" line)
  let inKey = false;
  for (const line of learn.split('\n')) {
    if (/^(#{3}\s+Key words|\*\*Key words\.?\*\*|Key terms:|Words used a lot:|\*Terms used below:\*)/i.test(line.trim())) inKey = true;
    else if (/^### /.test(line)) inKey = false;
    const re = /\*\*([^*]{1,48})\*\*(\s*\([^)]{1,80}\))?\s*(=|:|—)\s*(.+?)(?=\s\*\*[^*]{1,48}\*\*(?:\s*\/\s*\*\*[^*]{1,48}\*\*)?(?:\s*\([^)]*\))?\s*(?:=|—)|$)/g;
    let m: RegExpExecArray | null;
    const src = line.replace(/^- /, '').replace(/^\*\*Key words\.\*\*\s*/, '');
    while ((m = re.exec(src))) {
      if ((m[3] === ':' || m[3] === '—') && !inKey) continue;
      const term = m[1].replace(/:$/, '').trim();
      if (term.length < 2 || /^\d/.test(term) || /^(why|note|trap|correct|step|rule|the |a )/i.test(term)) continue;
      // "GVWR = … allows. A **converter dolly** is …" → two entries, so each definition covers only its own term
      let def = m[4].trim();
      const second = def.match(/^(.*?\.)\s+(?:An?|The)\s+\*\*([^*]{2,40})\*\*\s+(?:is|are|means)\s+(.+)$/);
      if (second) { def = second[1]; all.glossary.push({ term: second[2], defHtml: inline(stripTrailingCite(second[3].trim())), lesson: id }); }
      all.glossary.push({ term: term + (m[2] ? m[2] : ''), defHtml: inline(stripTrailingCite(def)), lesson: id });
    }
  }

  // ---- numbers table
  const numLines = S['Numbers & terms to memorize'].split('\n').filter((l) => /^\|/.test(l));
  const { rows } = parseTable(numLines);
  if (!isTableSep(numLines[1])) problems.push(`${id}: numbers sep`);
  rows.forEach((r, i) => {
    if (r.length !== 3) { problems.push(`${id}: numbers row ${i + 1} has ${r.length} cells`); return; }
    const nid = `${lid(id)}-n${String(i + 1).padStart(2, '0')}-${fnv(r[0] + r[1])}`;
    const item = r[0].replace(/\*\*\[CA\]\*\*|\[CA\]/g, '').replace(/\*\*/g, '').trim();
    all.numbers[nid] = { id: nid, lesson: id, item, valueHtml: inline(r[1]), value: plain(r[1]), pages: pageTokens(r[2]), ca: /\[CA\]/.test(r[0] + r[1]), ku: nid, concepts: [] };
    L.numberIds.push(nid);
  });

  // ---- traps
  S['Exam traps'].split('\n').filter((l) => /^- \*\*Trap/.test(l)).forEach((l, i) => {
    const m = l.match(/^- \*\*Trap(?: \[CA\])?:\*\*\s*(.+?)\s*\.?\s*→\s*\*\*Correct(?: for [^:]+)?:\*\*\s*(.+)$/);
    if (!m) { problems.push(`${id}: trap unparsed: ${l.slice(0, 60)}`); return; }
    const tid = `${lid(id)}-t${String(i + 1).padStart(2, '0')}-${fnv(m[1])}`;
    const corr = m[2].trim();
    all.traps[tid] = { id: tid, lesson: id, trap: plain(m[1]).replace(/^"|"$/g, ''), correct: plain(stripTrailingCite(corr)), correctHtml: inline(stripTrailingCite(corr)), pages: citePages(corr), ca: /\[CA\]/.test(l), ku: tid, concepts: [] };
    L.trapIds.push(tid);
  });

  // ---- TYK
  const tl = S['Handbook review questions — answered'].split('\n');
  for (let i = 0; i < tl.length; i++) {
    const m = tl[i].match(/^\*\*TYK (.+?) #(\d+)\*\*\s*(.+)$/);
    if (!m) continue;
    let j = i + 1; while (j < tl.length && !tl[j].trim()) j++;
    const a = (tl[j] || '').replace(/^→\s*/, '');
    if (!/^→/.test(tl[j] || '')) problems.push(`${id}: TYK #${m[2]} missing → answer`);
    const box = m[1].replace(/^Subsections?\s+/i, '').replace(/["“”]/g, '').replace(/\s*(&|AND|and)\s*/g, '–').replace(/,\s*/g, '–').replace(/\s+/g, '').toUpperCase();
    const kid = `TYK-${box}-${m[2]}`;
    if (all.tyk[kid]) problems.push(`${id}: duplicate TYK ${kid}`);
    all.tyk[kid] = { id: kid, lesson: id, box, n: +m[2], q: inline(m[3]), aHtml: inline(stripTrailingCite(a)), a: plain(stripTrailingCite(a)), pages: citePages(a), ku: kid, concepts: [] };
    L.tykIds.push(kid);
  }

  // ---- flashcards
  const fl = S['Flashcards'].split('\n').filter((l) => /^\|\s*\d+\s*\|/.test(l));
  fl.forEach((l) => {
    const cells = l.trim().replace(/^\|/, '').replace(/\|$/, '').split('|').map((c) => c.trim());
    if (cells.length !== 3) { problems.push(`${id}: flashcard cells ${cells.length}: ${l.slice(0, 50)}`); return; }
    const fid = `${lid(id)}-f${cells[0].padStart(2, '0')}-${fnv(cells[1])}`;
    all.flash[fid] = { id: fid, lesson: id, n: +cells[0], q: inline(cells[1]), a: inline(cells[2]), ca: /\[CA\]/.test(cells[1]), ku: fid, concepts: [] };
    L.flashIds.push(fid);
  });

  // ---- practice test + answer key
  const pt = S['Practice test (real-exam style)'];
  const [qPart, kPart = ''] = pt.split(/^### Answer key\s*$/m);
  const qs: { n: number; stem: string; opts: string[] }[] = [];
  for (const line of qPart.split('\n')) {
    const sm = line.match(/^(\d+)\. (.+)$/);
    const om = line.match(/^\s{3,4}([abc])\) (.+)$/);
    if (sm) qs.push({ n: +sm[1], stem: sm[2].trim(), opts: [] });
    else if (om && qs.length) qs[qs.length - 1].opts.push(om[2].trim());
    else if (line.trim() && !/^\s*$/.test(line)) problems.push(`${id}: practice unparsed line: ${line.slice(0, 60)}`);
  }
  const keys = new Map<number, { k: number; expl: string; pages: string[] }>();
  for (const line of kPart.split('\n')) {
    const km = line.match(/^(\d+)\. ([abc]) — (.+)$/);
    if (km) keys.set(+km[1], { k: 'abc'.indexOf(km[2]), expl: km[3].trim(), pages: citePages(km[3]) });
    else if (line.trim()) problems.push(`${id}: key unparsed: ${line.slice(0, 60)}`);
  }
  for (const q of qs) {
    const key = keys.get(q.n);
    if (!key) { problems.push(`${id}: no key for Q${q.n}`); continue; }
    if (q.opts.length !== 3) problems.push(`${id}: Q${q.n} has ${q.opts.length} options`);
    if (!key.pages.length) problems.push(`${id}: Q${q.n} key has no page`);
    const stemText = plain(q.stem);
    const polarity = /\b(NOT|EXCEPT)\b/.test(q.stem) ? 'neg' : /^true or false/i.test(stemText) ? 'tf' : 'pos';
    const qid = `${lid(id)}-q${String(q.n).padStart(2, '0')}-${fnv(q.stem)}`;
    // make explanations shuffle-safe: "(b)" → the option's own words
    key.expl = key.expl.replace(/\(([abc])\)/g, (_m, l: string) => `“${plain(q.opts['abc'.indexOf(l)] ?? l)}”`);
    const it: Item = {
      id: qid, lesson: id, test, origin: 'pack', stem: inline(q.stem), stemText, options: q.opts.map(inline), optionsText: q.opts.map(plain), key: key.k,
      explanation: inline(stripTrailingCite(key.expl)), pages: key.pages, polarity, numeric: q.opts.every((o) => /\d/.test(o)),
      tags: q.opts.map((o, i) => (i === key.k ? null : /\d/.test(o) ? 'neighbor_number' : 'plausible_generic')) as (DistractorTag | null)[],
      concepts: [], ku: qid,
    };
    all.items[qid] = it; L.itemIds.push(qid);
  }

  // ---- golden check
  const g = GOLDEN[id];
  const got: [number, number, number, number, number] = [L.itemIds.length, L.flashIds.length, L.tykIds.length, L.numberIds.length, L.trapIds.length];
  if (!g || g.join() !== got.join()) problems.push(`${id}: golden ${g} != parsed ${got} [practice,flash,tyk,numbers,traps]`);
  all.lessons.push(L);
}

// ---------- concept mapping by page overlap (enrichment may refine); flashcards by keyword overlap
const STOP = new Set('the a an of to in on at for and or is are be by with what when which how your you it its as that this from do does must can may if than more less not no into after before each every one two three'.split(' '));
const words = (s: string) => new Set(norm(s).split(' ').filter((w) => w.length > 2 && !STOP.has(w)));
const conceptWords = new Map<string, { title: Set<string>; body: Set<string> }>();
function cw(cid: string) {
  let w = conceptWords.get(cid);
  if (!w) { const c = all.concepts[cid]; w = { title: words(c.title), body: words(c.core.join(' ') + ' ' + c.html.replace(/<[^>]+>/g, ' ')) }; conceptWords.set(cid, w); }
  return w;
}
/** Best-matching concept(s) in a lesson: page overlap + keyword overlap with the concept's title and body. */
function bestConcepts(lesson: string, pages: string[], text: string, n = 1): string[] {
  const L = all.lessons.find((l) => l.id === lesson)!;
  const tw = words(text);
  const scored = L.conceptIds.map((cid) => {
    const c = all.concepts[cid];
    const w = cw(cid);
    let sc = c.pages.some((p) => pages.includes(p)) ? 3 : 0;
    for (const x of tw) { if (w.title.has(x)) sc += 2; else if (w.body.has(x)) sc += 1; }
    if (/^(start here|key words)/i.test(c.title)) sc -= 3;
    return { cid, sc, span: c.pages.length || 9 };
  }).sort((a, b) => b.sc - a.sc || a.span - b.span);
  return scored.slice(0, n).filter((x) => x.sc > 0).map((x) => x.cid);
}
function byPages(lesson: string, pages: string[], text = ''): string[] {
  const r = bestConcepts(lesson, pages, text, 1);
  return r.length ? r : [all.lessons.find((l) => l.id === lesson)!.conceptIds[0]];
}
function mapFacts() {
  for (const n of Object.values(all.numbers)) n.concepts = byPages(n.lesson, n.pages, n.item + ' ' + n.value);
  for (const t of Object.values(all.traps)) t.concepts = byPages(t.lesson, t.pages, t.trap + ' ' + t.correct);
  for (const t of Object.values(all.tyk)) t.concepts = byPages(t.lesson, t.pages, plain(t.q) + ' ' + t.a);
  for (const f of Object.values(all.flash)) {
    const L = all.lessons.find((l) => l.id === f.lesson)!;
    const fw = words(f.q + ' ' + f.a);
    let best: string | null = null, bestScore = 0;
    for (const cid of L.conceptIds) {
      const c = all.concepts[cid];
      const cw = words(c.title + ' ' + c.html.replace(/<[^>]+>/g, ' '));
      let sc = 0; for (const w of fw) if (cw.has(w)) sc++;
      if (sc > bestScore) { bestScore = sc; best = cid; }
    }
    (f as Flashcard & { concepts?: string[] }).concepts = best && bestScore >= 2 ? [best] : [];
  }
}

// ---------- item → concept mapping by page overlap (enrichment may refine)
function mapConcepts() {
  for (const it of Object.values(all.items)) it.concepts = byPages(it.lesson, it.pages, it.stemText + ' ' + it.optionsText[it.key] + ' ' + plain(it.explanation));
}

// ---------- hand-written key points (enrich/summaries/<LESSON>.json: conceptId → 2–5 bullets, checked against the lesson)
function mergeSummaries() {
  let files: string[] = [];
  try { files = readdirSync(join(ENRICH, 'summaries')).filter((f) => f.endsWith('.json')); } catch { return; }
  let n = 0;
  for (const f of files) {
    const S = JSON.parse(readFileSync(join(ENRICH, 'summaries', f), 'utf8')) as Record<string, string[]>;
    for (const [cid, bullets] of Object.entries(S)) {
      const c = all.concepts[cid];
      if (!c) throw new Error(`summaries/${f}: unknown concept ${cid}`);
      if (!Array.isArray(bullets) || bullets.length < 2 || bullets.length > 5) throw new Error(`summaries/${f}: ${cid} needs 2–5 bullets`);
      c.core = bullets.map((b) => inline(b.replace(/^\[CA\]\s*/, '**[CA]** ')));
      c.coreExtra = undefined;
      n++;
    }
  }
  console.log(`summaries: ${n} concepts use hand-written key points`);
}

// ---------- deterministic derived items
// Number MCQs are NOT generated by heuristics (tested 2026-09-27: unit-matched distractors were often
// semantically wrong, e.g. "8% / 39% / .04 percent"). They come from verified enrichment
// (enrich/<LESSON>.json → numberItems) merged below. Trap duels are safe: the trap statement is false by construction.
function deriveItems() {
  // trap duels (True/False) — the trap statement is false by construction
  for (const t of Object.values(all.traps)) {
    const id = `${lid(t.lesson)}-dt-${fnv(t.id)}`;
    const L = all.lessons.find((l) => l.id === t.lesson)!;
    const cands = L.conceptIds.map((c) => all.concepts[c]).filter((c) => c.pages.some((p) => t.pages.includes(p)));
    // Trap text is often a fragment ("6 triangles"); present it as a quoted claim so the stem is a full sentence.
    let stmt = t.trap.replace(/^[a-z]/, (c) => c.toUpperCase()).replace(/[.\s]+$/, '');
    if ((stmt.match(/"/g) ?? []).length % 2) stmt = stmt.replace(/"/g, '');
    all.items[id] = {
      id, lesson: t.lesson, test: L.test, origin: 'derived-trap', stem: `True or false? The handbook says: “${inline(stmt)}.”`, stemText: `True or false? The handbook says: “${stmt}.”`,
      options: ['True', 'False'], optionsText: ['True', 'False'], key: 1, explanation: `<strong>False.</strong> ${t.correctHtml}`, pages: t.pages,
      polarity: 'tf', numeric: false, tags: ['trap_named', null], concepts: (cands.length ? cands.slice(0, 1) : [all.concepts[L.conceptIds[0]]]).map((c) => c.id), ku: t.id,
    };
  }
}


// ---------- verified enrichment (enrich/<LESSON>.json): number MCQs + per-option distractor tags
interface EnrichFile {
  numberItems?: { numberId: string; stem: string; options: string[]; key: number; tags: (DistractorTag | null)[]; explanation: string; verified?: boolean }[];
  distractorTags?: Record<string, (DistractorTag | null)[]>;
}
function mergeEnrichment() {
  let rejected = new Set<string>();
  try { rejected = new Set((JSON.parse(readFileSync(join(ENRICH, 'rejected.json'), 'utf8')) as { id: string }[]).map((r) => r.id)); } catch { /* none yet */ }
  let files: string[] = [];
  try { files = readdirSync(ENRICH).filter((f: string) => /^(GK|CV)-\d\d\.json$/.test(f)); } catch { return; }
  let seed = 11;
  const rnd = () => ((seed = (seed * 1103515245 + 12345) & 0x7fffffff) / 0x7fffffff);
  for (const f of files.sort()) {
    const lesson = f.replace('.json', '');
    const E = JSON.parse(readFileSync(join(ENRICH, f), 'utf8')) as EnrichFile;
    const L = all.lessons.find((l) => l.id === lesson)!;
    for (const [iid, tags] of Object.entries(E.distractorTags ?? {})) {
      const it = all.items[iid];
      if (!it) { dropped.push(`enrich ${f}: unknown item ${iid}`); continue; }
      if (tags.length !== it.options.length || tags[it.key] !== null || tags.some((t, k) => k !== it.key && !t)) { dropped.push(`enrich ${f}: bad tags for ${iid}`); continue; }
      it.tags = tags;
    }
    for (const n of E.numberItems ?? []) {
      if (n.verified === false) continue;
      const nf = all.numbers[n.numberId];
      if (!nf) { dropped.push(`enrich ${f}: unknown number ${n.numberId}`); continue; }
      const bad = n.options.length !== 3 || n.key < 0 || n.key > 2 || new Set(n.options.map(norm)).size !== 3 || n.tags.length !== 3 || n.tags[n.key] !== null;
      if (bad) { dropped.push(`enrich ${f}: malformed number item ${n.numberId}`); continue; }
      const keyNums = (n.options[n.key].match(/\d[\d,./]*/g) || []).map((x) => x.replace(/,/g, ''));
      const factNums = (nf.value.match(/\d[\d,./]*/g) || []).map((x) => x.replace(/,/g, ''));
      if (keyNums.length && !keyNums.every((x) => factNums.includes(x))) { dropped.push(`enrich ${f}: key of ${n.numberId} not in fact value (${n.options[n.key]} vs ${nf.value})`); continue; }
      const id = `${lid(lesson)}-dn-${fnv(n.numberId + n.stem)}`;
      if (rejected.has(id)) { dropped.push(`enrich ${f}: ${id} rejected by independent verifier`); continue; }
      all.items[id] = {
        id, lesson, test: L.test, origin: 'derived-number', stem: inline(n.stem), stemText: plain(n.stem), options: n.options.map(inline), optionsText: n.options.map(plain),
        key: n.key, explanation: inline(n.explanation), pages: nf.pages, polarity: /\b(NOT|EXCEPT)\b/.test(n.stem) ? 'neg' : 'pos', numeric: n.options.every((o) => /\d/.test(o)), tags: n.tags,
        concepts: nf.concepts.length ? nf.concepts : [L.conceptIds[0]], ku: nf.id, heldOut: rnd() < 0.5,
      };
    }
  }
}

// ---------- explanations must not name options by position (options are shuffled in the app)
const ORD: Record<string, number> = { first: 0, second: 1, third: 2 };
function positionSafe(it: Item) {
  const txt = (k: number) => `“${it.optionsText[k] ?? ''}”`;
  it.explanation = it.explanation
    .replace(/\b[Tt]he (first|second|third) (?:option|choice|answer)\b/g, (_m, w: string) => txt(ORD[w]))
    .replace(/\b(?:choice|option|answer) \(?([abc])\)?(?![a-z])/gi, (_m, l: string) => txt('abc'.indexOf(l.toLowerCase())))
    .replace(/\(([abc])\)/g, (_m, l: string) => txt('abc'.indexOf(l)));
}

// ---------- per-option "why this is wrong" notes (enrich/notes/<LESSON>.json)
function mergeNotes() {
  let files: string[] = [];
  try { files = readdirSync(join(ENRICH, 'notes')).filter((f: string) => /^(GK|CV)-\d\d\.json$/.test(f)); } catch { return; }
  for (const f of files) {
    const N = JSON.parse(readFileSync(join(ENRICH, 'notes', f), 'utf8')) as Record<string, (string | null)[]>;
    for (const [iid, notes] of Object.entries(N)) {
      const it = all.items[iid];
      if (!it) continue; // item was dropped/rejected
      if (notes.length !== it.options.length || notes[it.key] !== null || notes.some((n, k) => k !== it.key && (!n || /\b(option|answer)\s+[abc]\b|\([abc]\)/i.test(n)))) { dropped.push(`notes ${f}: bad notes for ${iid}`); continue; }
      it.notes = notes.map((n) => (n ? inline(n) : null));
    }
  }
}

// ---------- START-HERE: handbook-way table + most-missed list
function parseStartHere(md: string) {
  const S = sections(md);
  const hw = S['Answer the HANDBOOK way'].split('\n').filter((l) => /^\|/.test(l));
  const { rows } = parseTable(hw);
  const handbookWay = rows.map((r) => ({ topic: inline(r[0]), answer: inline(r[1]), elsewhere: inline(r[2]) }));
  const mm = S['The most-missed questions'];
  const mostMissed: Content['mostMissed'] = [];
  let test: TestId = 'GK';
  for (const line of mm.split('\n')) {
    if (/\*\*Combination Vehicles\*\*/.test(line)) test = 'CV';
    const m = line.match(/^\d+\. (.+?)\s*\[((?:GK|CV)-\d\d)(?:,\s*(?:GK|CV)-\d\d)*\]\s*$/);
    if (m) mostMissed.push({ test, text: inline(m[1]), lesson: m[2] });
  }
  return { handbookWay, mostMissed };
}

// ---------- main
const files = readdirSync(SRC).filter((f: string) => /^(GK|CV)-\d\d.*\.md$/.test(f)).sort((a: string, b: string) => (a.startsWith('GK') === b.startsWith('GK') ? a.localeCompare(b) : a.startsWith('GK') ? -1 : 1));
// Source lessons are written for one learner who passed Air Brakes; neutralize that for everyone (lessons stay verbatim on disk).
const TEXT_FIX: [string, string][] = [['Tip: you passed Air Brakes.', 'Tip, if you have passed Air Brakes:']];
for (const f of files) { let md = readFileSync(join(SRC, f), 'utf8'); for (const [a, b] of TEXT_FIX) md = md.split(a).join(b); parseLesson(f, md); }
mergeSummaries();
mapConcepts();
mapFacts();
deriveItems();
mergeEnrichment();
mergeNotes();
// Stems that name a handbook section number a beginner cannot know; source lessons stay verbatim.
const STEM_FIX: [RegExp, string][] = [[/is NOT a rule in Section 1\.4\?/, 'is NOT one of the CDL reporting rules (license, convictions, employer notices)?']];
for (const it of Object.values(all.items)) for (const [re, to] of STEM_FIX) if (re.test(it.stemText)) { it.stem = it.stem.replace(re, to); it.stemText = it.stemText.replace(re, to); }
for (const it of Object.values(all.items)) positionSafe(it);
mapConcepts(); // re-map every item (incl. derived) by page + keyword overlap
const derivedNumbers = Object.values(all.items).filter((i) => i.origin === 'derived-number').length;
const sh = parseStartHere(readFileSync(join(SRC, '00-START-HERE.md'), 'utf8'));

// terms the lessons define in running text or recaps rather than key-word lists (wording from CV-02 lines on p. 6-6)
all.glossary.push(
  { term: 'Dummy couplers', defHtml: inline('places behind the cab to hook the air hoses when no trailer is connected. They keep **water and dirt** out of the couplers and air lines.'), lesson: 'CV-02' },
  { term: 'Service line', defHtml: inline('the **blue** air line (control or signal line). Its pressure, set by the brake pedal or trailer hand valve, tells the trailer relay valves how hard to brake.'), lesson: 'CV-02' },
  { term: 'Emergency line', defHtml: inline('the **red** air line (supply line). It fills the trailer air tanks and controls the trailer emergency brakes.'), lesson: 'CV-02' },
);
// dedupe glossary by term (first wins)
// dedupe by the bare term (ignoring a parenthetical expansion); keep the most informative entry; sort ignoring punctuation
const gkey = (t: string) => norm(t.replace(/\([^)]*\)/g, ''));
// not glossary terms: control labels and headings the key-word scan picks up
const NOT_TERMS = new Set(['in', 'out', 'push in', 'pull out', 'defect', 'knowing when to shift down', 'before']);
all.glossary = all.glossary.filter((g) => !NOT_TERMS.has(gkey(g.term)));
const best = new Map<string, GlossaryEntry>();
for (const g of all.glossary) { const k = gkey(g.term); const cur = best.get(k); if (!cur || g.term.length + g.defHtml.length > cur.term.length + cur.defHtml.length) best.set(k, g); }
const sortKey = (t: string) => t.replace(/^[^A-Za-z0-9]+/, '').toLowerCase();
all.glossary = [...best.values()].sort((a, b) => sortKey(a.term).localeCompare(sortKey(b.term)));

const pack = Object.values(all.items).filter((i) => i.origin === 'pack');
const counts = {
  lessons: all.lessons.length, concepts: Object.keys(all.concepts).length, practice: pack.length, options: pack.reduce((s, i) => s + i.options.length, 0),
  flashcards: Object.keys(all.flash).length, tyk: Object.keys(all.tyk).length, numbers: Object.keys(all.numbers).length, traps: Object.keys(all.traps).length,
  derivedNumber: derivedNumbers, derivedTrap: Object.keys(all.traps).length, heldOut: Object.values(all.items).filter((i) => i.heldOut).length,
  glossary: all.glossary.length, handbookWay: sh.handbookWay.length, mostMissed: sh.mostMissed.length,
};
const expect: Record<string, number> = { lessons: 18, practice: 338, options: 1014, flashcards: 657, tyk: 98, numbers: 486, traps: 264 };
for (const [k, v] of Object.entries(expect)) if ((counts as Record<string, number>)[k] !== v) problems.push(`TOTAL ${k}: expected ${v}, got ${(counts as Record<string, number>)[k]}`);
if (sh.handbookWay.length !== 9) problems.push(`handbookWay rows ${sh.handbookWay.length} != 9`);
if (sh.mostMissed.length !== 50) problems.push(`mostMissed ${sh.mostMissed.length} != 50`);

// ---------- tap-to-define: first use of each glossary term in a concept becomes a button (text nodes only)
function linkGlossary() {
  // too common to link everywhere (still listed in the glossary page)
  const SKIP = new Set(['cdl', 'cmv', 'clp', 'emergency', 'hazard', 'conflict', 'tractor', 'texting', 'traction', 'conviction', 'restriction', 'endorsement']);
  const terms = all.glossary.map((g, i) => ({ i, bare: g.term.replace(/\s*\([^)]*\)\s*/g, '').replace(/["“”]/g, '').trim() }))
    .filter((t) => t.bare.length >= 3 && !SKIP.has(t.bare.toLowerCase()))
    .sort((a, b) => b.bare.length - a.bare.length);
  for (const c of Object.values(all.concepts)) {
    let used = new Set<number>();
    const wrap = (html: string) => html.split(/(<[^>]+>)/).map((seg, k, arr) => {
      if (seg.startsWith('<')) return seg;
      const inButton = arr.slice(0, k).reverse().find((x) => /^<\/?button/.test(x));
      if (inButton && !inButton.startsWith('</')) return seg;
      // placeholders keep one link from ever landing inside another
      const tokens: string[] = [];
      for (const t of terms) {
        if (used.has(t.i)) continue;
        const re = new RegExp(`(^|[^A-Za-z0-9\u0000])(${t.bare.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})(?=[^A-Za-z0-9\u0000]|$)`, /^[A-Z0-9]+$/.test(t.bare) ? '' : 'i');
        const m = seg.match(re);
        if (m && m.index !== undefined) {
          used.add(t.i);
          const at = m.index + m[1].length;
          tokens.push(`<button type="button" class="gl" data-g="${t.i}">${m[2]}</button>`);
          seg = seg.slice(0, at) + `\u0000${tokens.length - 1}\u0000` + seg.slice(at + m[2].length);
        }
      }
      return seg.replace(/\u0000(\d+)\u0000/g, (_m, n: string) => tokens[+n]);
    }).join('');
    used = new Set<number>(); c.core = c.core.map(wrap);   // quick view: first use of each term
    used = new Set<number>(); c.html = wrap(c.html);       // dive deeper: its own first uses
  }
}
linkGlossary();

const content: Content = { version: fnv(JSON.stringify(counts) + files.join()), handbook: 'DL 650 California Commercial Driver Handbook (R12-2019)', ...all, ...sh, counts };
mkdirSync(dirname(OUT), { recursive: true });
writeFileSync(OUT, JSON.stringify(content));
console.log(JSON.stringify(counts));
if (dropped.length) console.warn(`ENRICHMENT DROPPED (${dropped.length}):\n` + dropped.join('\n'));
if (problems.length) { console.error('PARSE PROBLEMS:\n' + problems.join('\n')); process.exit(1); }
console.log(`OK → ${OUT} (${(JSON.stringify(content).length / 1024).toFixed(0)} KB)`);
