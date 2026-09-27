// The learner "brain": surfaces, answer recording, diagnosis, mistake notebook, review queue.
// Pure functions over (AppState, Content) so they are unit- and simulation-testable.
import type { Content, Item, TestId } from '../content/types';
import type { AppState, Attempt, Cause, EvidenceClass, NotebookRow } from './model';
import { ATTEMPT_CAP, dayKey, daysBetween } from './model';
import { bktEvidence, bktTransition, gradeFor, isSlow, retrievability, review, BKT } from './srs';

export type SurfaceKind = 'item' | 'number' | 'flash' | 'tyk';
export interface Surface { id: string; kind: SurfaceKind; ku: string; lesson: string; test: TestId; concepts: string[]; stakes: number }

export function surface(c: Content, id: string): Surface | null {
  const L = (lid: string) => c.lessons.find((l) => l.id === lid)!;
  const st = (lid: string) => (L(lid).weight === 'High' ? 2 : 1);
  const it = c.items[id];
  if (it) return { id, kind: 'item', ku: it.ku, lesson: it.lesson, test: it.test, concepts: it.concepts, stakes: st(it.lesson) };
  const n = c.numbers[id];
  if (n) return { id, kind: 'number', ku: n.ku, lesson: n.lesson, test: L(n.lesson).test, concepts: n.concepts, stakes: st(n.lesson) };
  const f = c.flash[id];
  if (f) return { id, kind: 'flash', ku: f.ku, lesson: f.lesson, test: L(f.lesson).test, concepts: f.concepts, stakes: st(f.lesson) };
  const t = c.tyk[id];
  if (t) return { id, kind: 'tyk', ku: t.ku, lesson: t.lesson, test: L(t.lesson).test, concepts: t.concepts, stakes: st(t.lesson) };
  return null;
}

/** All surfaces that belong to a knowledge unit (lets a review rotate the surface shown). */
export function kuIndex(c: Content): Map<string, string[]> {
  const m = new Map<string, string[]>();
  const add = (ku: string, id: string) => { const a = m.get(ku); if (a) a.push(id); else m.set(ku, [id]); };
  for (const x of Object.values(c.items)) add(x.ku, x.id);
  for (const x of Object.values(c.numbers)) add(x.ku, x.id);
  for (const x of Object.values(c.flash)) add(x.ku, x.id);
  for (const x of Object.values(c.tyk)) add(x.ku, x.id);
  return m;
}

// ---------- diagnosis (deterministic MVP causes: T, W, N, E4, else U)
export function diagnose(state: AppState, item: Item, chosen: number | undefined, now: number): Cause {
  const tag = chosen !== undefined ? item.tags[chosen] : null;
  const chosenText = chosen !== undefined ? item.optionsText[chosen] ?? '' : '';
  const hasDigit = /\d/.test(chosenText);
  // never opened the lesson: nothing more specific can be said yet
  if (!state.lessons[item.lesson]?.opened) return 'U';
  // 0) a trap duel (true/false on the trap statement): fell for the trap itself
  if (item.origin === 'derived-trap') return 'M';
  // 1) the stem itself flips the wording (NOT / EXCEPT / true-false)
  if (item.polarity === 'neg' || item.polarity === 'tf') return 'T';
  // 2) picked the federal / car-handbook / website value
  if (tag === 'alt_source') return 'W';
  // 3) picked the famous wrong idea for this rule
  if (tag === 'trap_named') return 'M';
  // 4) picked a different number (only when the chosen option really is a number)
  if (hasDigit && (tag === 'neighbor_number' || tag === 'sibling_rule_number' || tag === 'round_number' || tag === 'unit_match' || !tag)) return 'N';
  // 5) knew it before, but it faded
  const card = state.cards[item.ku];
  const hadSuccess = state.attempts.some((a) => a.ku === item.ku && a.ok);
  if (card && hadSuccess && retrievability(card, now) < 0.8) return 'E4';
  return 'U';
}

// ---------- recording an answer
export interface AnswerInput { id: string; ev: EvidenceClass; ok: boolean; chosen?: number; guessed?: boolean; rt?: number; now: number; examCapDays?: number; charCount?: number }

export function recordAnswer(state: AppState, c: Content, a: AnswerInput): { cause?: Cause } {
  const s = surface(c, a.id);
  if (!s) return {};
  const it = c.items[a.id];
  // FSRS on the knowledge unit
  const rtHist = state.attempts.filter((x) => x.rt && x.ev !== 'self').slice(-200).map((x) => x.rt! / 120);
  const slow = isSlow(a.rt, a.charCount ?? 120, rtHist);
  const g = gradeFor(a.ev, a.ok, !!a.guessed, slow);
  if (a.ev !== 'check' || !state.cards[s.ku]) {
    state.cards[s.ku] = review(state.cards[s.ku], g, a.now, a.examCapDays ?? 365);
  }
  // BKT per concept (evidence update; one learning transition per concept per day if a correct answer occurred)
  const today = dayKey(a.now);
  for (const cid of s.concepts) {
    const b = state.bkt[cid] ?? { p: BKT.L0, n: 0 };
    b.p = bktEvidence(b.p, a.ok && !a.guessed, a.ev);
    if (a.ok && !a.guessed && b.lastDayGain !== today) { b.p = bktTransition(b.p, 0.3); b.lastDayGain = today; }
    b.n += 1; b.lastT = a.now;
    state.bkt[cid] = b;
  }
  // diagnosis + notebook
  let cause: Cause | undefined;
  if (it) {
    if ((!a.ok || a.guessed) && a.ev !== 'typed' && a.ev !== 'self') {
      cause = a.ok ? 'U' : diagnose(state, it, a.chosen, a.now);
      for (const cid of it.concepts.slice(0, 1)) upsertRow(state, cid, it.lesson, cause, it.id, a.now);
    }
    probeRows(state, it.concepts, a.ok && !a.guessed, a.now, a.ev === 'typed');
  } else {
    probeRows(state, s.concepts, a.ok, a.now, a.ev === 'typed');
  }
  const att: Attempt = { t: a.now, id: a.id, ku: s.ku, ev: a.ev, ok: a.ok, chosen: a.chosen, guessed: a.guessed || undefined, rt: a.rt, cause, concepts: s.concepts };
  state.attempts.push(att);
  if (state.attempts.length > ATTEMPT_CAP) state.attempts.splice(0, state.attempts.length - ATTEMPT_CAP);
  // study time credit for the calendar (5–60 s per answer)
  creditTime(state, a.now, Math.min(60_000, Math.max(5_000, a.rt ?? 15_000)));
  state.updatedAt = a.now;
  return { cause };
}

export function creditTime(state: AppState, now: number, ms: number) {
  const k = dayKey(now);
  const d = state.days[k] ?? { planned: 0, done: 0 };
  d.done = Math.round((d.done + ms / 60_000) * 100) / 100;
  state.days[k] = d;
}

// ---------- notebook
export function rowKey(concept: string, cause: Cause) { return `${concept}|${cause}`; }

function upsertRow(state: AppState, concept: string, lesson: string, cause: Cause, itemId: string, now: number) {
  const key = rowKey(concept, cause);
  const r: NotebookRow = state.notebook[key] ?? { key, concept, lesson, cause, items: [], status: 'open', created: now, updated: now, misses: 0, probes: [] };
  if (!r.items.includes(itemId)) r.items.push(itemId);
  r.misses += 1; r.updated = now; r.status = 'open'; r.snoozeUntil = undefined;
  r.probes = r.probes.filter((p) => p.t > now); // a new miss restarts the proof
  state.notebook[key] = r;
}

/** A correct probe counts once per day; resolution = ≥2 correct probes on distinct days and concept mastery ≥ 0.6. */
function probeRows(state: AppState, concepts: string[], ok: boolean, now: number, typed: boolean) {
  for (const r of Object.values(state.notebook)) {
    if (r.status === 'resolved' || !concepts.includes(r.concept)) continue;
    if (!ok) continue; // misses are handled by upsertRow
    const today = dayKey(now);
    if (!r.probes.some((p) => dayKey(p.t) === today && p.ok)) r.probes.push({ t: now, ok: true, typed });
    r.status = 'probing'; r.updated = now;
    const days = new Set(r.probes.filter((p) => p.ok).map((p) => dayKey(p.t)));
    const p = state.bkt[r.concept]?.p ?? 0;
    if (days.size >= 2 && p >= 0.6) r.status = 'resolved';
  }
}

export function rowPriority(state: AppState, c: Content, r: NotebookRow, now: number): number {
  const L = c.lessons.find((l) => l.id === r.lesson);
  const stakes = L?.weight === 'High' ? 2 : 1;
  const p = state.bkt[r.concept]?.p ?? BKT.L0;
  const lowR = r.items.some((id) => retrievability(state.cards[c.items[id]?.ku ?? id], now) < 0.9) ? 1 : 0;
  // a miss in the last 3 days keeps a topic near the top (right after a miss R is high, so lowR alone would drop it)
  const recent = now - r.updated < 3 * 86400_000 ? 1 : 0;
  return stakes * (1 - p) * (1 + Math.max(lowR, recent)) * (1 + Math.min(5, r.misses) * 0.25);
}

export function openRows(state: AppState, c: Content, now: number): NotebookRow[] {
  return Object.values(state.notebook)
    .filter((r) => r.status !== 'resolved' && !(r.status === 'snoozed' && (r.snoozeUntil ?? 0) > now))
    .sort((a, b) => rowPriority(state, c, b, now) - rowPriority(state, c, a, now));
}

// ---------- due reviews
export function dueSurfaces(state: AppState, c: Content, now: number, tests: TestId[], ku2: Map<string, string[]>): Surface[] {
  const endOfDay = now + 3600_000; // due within the next hour counts
  const out: { s: Surface; over: number }[] = [];
  for (const [ku, card] of Object.entries(state.cards)) {
    if (card.due > endOfDay) continue;
    const ids = ku2.get(ku); if (!ids) continue;
    // rotate: prefer the surface least recently shown
    const last = new Map<string, number>();
    for (const a of state.attempts) if (a.ku === ku) last.set(a.id, a.t);
    const pick = [...ids].sort((x, y) => (last.get(x) ?? 0) - (last.get(y) ?? 0))[0];
    const s = surface(c, pick);
    if (!s || !tests.includes(s.test)) continue;
    const over = (now - card.due) / 86400_000 / Math.max(1, card.scheduled_days || 1);
    out.push({ s, over: over * s.stakes });
  }
  return out.sort((a, b) => b.over - a.over).map((x) => x.s);
}

export function lessonDone(state: AppState, lessonId: string): boolean {
  const lp = state.lessons[lessonId];
  return !!lp?.completed;
}

export function nextLesson(state: AppState, c: Content, tests: TestId[]): string | null {
  for (const l of c.lessons) if (tests.includes(l.test) && !lessonDone(state, l.id)) return l.id;
  return null;
}

/** Surfaces of a lesson that were never answered (new cards). */
export function newSurfaces(state: AppState, c: Content, lessonId: string): string[] {
  const L = c.lessons.find((l) => l.id === lessonId)!;
  const ids = [...L.numberIds, ...L.flashIds, ...L.tykIds];
  return ids.filter((id) => !state.cards[surface(c, id)!.ku]);
}

export function examCapDays(state: AppState, test: TestId, now: number): number {
  const d = state.profile.examDates[test];
  if (!d) return 365;
  return Math.max(1, daysBetween(dayKey(now), d));
}

/**
 * Evidence from an untimed widget CHALLENGE answer (practice evidence, not a game reward): weak BKT update
 * (weight 0.2, guess 1/3 — about ±0.04 per answer), no learning transition, never FSRS.
 * Explore and timed interactions must not call this (tested in tests/e2e/widgets.spec.ts).
 */
export const CHECK_MAX_PER_DAY = 3;   // one challenge run is correlated evidence: count the first few answers per concept per day
export const CHECK_CEILING = 0.55;    // widget answers alone never make a topic "proficient" (0.6); questions must
export function recordCheck(state: AppState, concepts: string[], ok: boolean, now: number) {
  const today = dayKey(now);
  for (const cid of concepts) {
    const b = state.bkt[cid] ?? { p: BKT.L0, n: 0 };
    if (b.checkDay !== today) { b.checkDay = today; b.checkN = 0; }
    if ((b.checkN ?? 0) >= CHECK_MAX_PER_DAY) continue;
    b.checkN = (b.checkN ?? 0) + 1;
    const next = bktEvidence(b.p, ok, 'check');
    b.p = ok ? Math.max(b.p, Math.min(next, CHECK_CEILING)) : next;
    b.n += 1; b.lastT = now;
    state.bkt[cid] = b;
  }
  creditTime(state, now, 20_000);
}
