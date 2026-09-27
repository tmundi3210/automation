import { describe, it, expect } from 'vitest';
import content from '../../src/content/content.json';
import type { Content } from '../../src/content/types';
import { emptyState, dayKey } from '../../src/engine/model';
import { review, retrievability, bktEvidence, bktTransition, gradeFor } from '../../src/engine/srs';
import { recordAnswer, diagnose, openRows } from '../../src/engine/learner';
import { fillPct, buildPlan, buildMock, today } from '../../src/engine/plan';
import { binomTail, readiness } from '../../src/engine/readiness';
import { makeResumeCode, readResumeCode } from '../../src/store/resume';

const C = content as unknown as Content;
const T0 = new Date(2026, 8, 28, 10).getTime();
const DAY = 86400_000;

describe('content golden counts', () => {
  it('matches START-HERE totals', () => {
    expect(C.counts.practice).toBe(338);
    expect(C.counts.flashcards).toBe(657);
    expect(C.counts.tyk).toBe(98);
    expect(C.counts.numbers).toBe(486);
    expect(C.counts.traps).toBe(264);
    expect(C.lessons.length).toBe(18);
  });
  it('every pack item has 3 options, a key and a page', () => {
    for (const it of Object.values(C.items).filter((i) => i.origin === 'pack')) {
      expect(it.options.length).toBe(3);
      expect(it.key).toBeGreaterThanOrEqual(0);
      expect(it.pages.length).toBeGreaterThan(0);
      expect(it.concepts.length).toBeGreaterThan(0);
    }
  });
});

describe('FSRS-6', () => {
  it('is deterministic with fuzz off and grows stability on Good', () => {
    const a = review(undefined, 3, T0, 365, false);
    const b = review(undefined, 3, T0, 365, false);
    expect(a).toEqual(b);
    expect(a.stability).toBeCloseTo(2.3065, 3); // w2 default
    let s = a; for (let i = 0; i < 4; i++) s = review(s, 3, s.due, 365, false);
    expect(s.stability).toBeGreaterThan(10);
    expect(retrievability(s, s.due)).toBeGreaterThan(0.85);
  });
  it('caps intervals at days-to-exam', () => {
    let s = review(undefined, 3, T0, 5, false);
    for (let i = 0; i < 6; i++) s = review(s, 3, s.due, 5, false);
    expect(s.scheduled_days).toBeLessThanOrEqual(5);
  });
  it('grade map: wrong/guessed → Again; slow → Hard; MCQ never Easy', () => {
    expect(gradeFor('mcq', false, false, false)).toBe(1);
    expect(gradeFor('mcq', true, true, false)).toBe(1);
    expect(gradeFor('mcq', true, false, true)).toBe(2);
    expect(gradeFor('mcq', true, false, false)).toBe(3);
  });
});

describe('BKT (review fix #3)', () => {
  it('single correct 3-option MCQ evidence update moves P(L) by ≤ +0.07 at 0.2', () => {
    const d = bktEvidence(0.2, true, 'mcq') - 0.2;
    expect(d).toBeGreaterThan(0);
    expect(d).toBeLessThanOrEqual(0.07);
  });
  it('misses move more than hits', () => {
    expect(0.3 - bktEvidence(0.3, false, 'mcq')).toBeGreaterThan(bktEvidence(0.3, true, 'mcq') - 0.3);
  });
  it('transition is small and bounded', () => {
    expect(bktTransition(0.3) - 0.3).toBeLessThan(0.05);
  });
  it('lucky guesser (random answers, 20 items over 20 days) stays below 0.4', () => {
    const s = emptyState(T0);
    const items = Object.values(C.items).filter((i) => i.origin === 'pack' && i.concepts[0] === 'GK-07.c03');
    const pool = items.length ? items : Object.values(C.items).filter((i) => i.lesson === 'GK-07' && i.origin === 'pack');
    let seed = 3; const rnd = () => ((seed = (seed * 16807) % 2147483647) / 2147483647);
    for (let d = 0; d < 20; d++) {
      const it = pool[d % pool.length];
      const chosen = Math.floor(rnd() * 3);
      recordAnswer(s, C, { id: it.id, ev: 'mcq', ok: chosen === it.key, chosen, now: T0 + d * DAY });
    }
    const c = pool[0].concepts[0];
    expect(s.bkt[c].p).toBeLessThan(0.4);
  });
  it('steady learner (all right, 8 days) reaches proficient ≥ 0.6', () => {
    const s = emptyState(T0);
    const pool = Object.values(C.items).filter((i) => i.lesson === 'GK-07' && i.origin === 'pack' && i.concepts[0] === 'GK-07.c02');
    const list = pool.length ? pool : Object.values(C.items).filter((i) => i.lesson === 'GK-07' && i.origin === 'pack').slice(0, 3);
    for (let d = 0; d < 8; d++) for (const it of list.slice(0, 3)) recordAnswer(s, C, { id: it.id, ev: 'mcq', ok: true, chosen: it.key, now: T0 + d * DAY + 1000 });
    expect(s.bkt[list[0].concepts[0]].p).toBeGreaterThanOrEqual(0.6);
  });
});

describe('diagnosis + notebook', () => {
  it('flags NOT/EXCEPT misses as trap wording (T)', () => {
    const s = emptyState(T0);
    const neg = Object.values(C.items).find((i) => i.polarity === 'neg')!;
    const wrong = [0, 1, 2].find((k) => k !== neg.key)!;
    expect(diagnose(s, neg, wrong, T0)).toBe('T');
  });
  it('flags numeric distractor misses as number mix-up (N)', () => {
    const s = emptyState(T0);
    const num = Object.values(C.items).find((i) => i.polarity === 'pos' && i.numeric && i.origin === 'pack')!;
    const wrong = [0, 1, 2].find((k) => k !== num.key)!;
    expect(diagnose(s, num, wrong, T0)).toBe('N');
  });
  it('miss → open row; 2 correct probes on distinct days + mastery → resolved', () => {
    const s = emptyState(T0);
    const it = Object.values(C.items).find((i) => i.origin === 'pack' && i.lesson === 'GK-08' && i.polarity === 'pos')!;
    const wrong = [0, 1, 2].find((k) => k !== it.key)!;
    recordAnswer(s, C, { id: it.id, ev: 'mcq', ok: false, chosen: wrong, now: T0 });
    expect(openRows(s, C, T0).length).toBe(1);
    const same = Object.values(C.items).filter((i) => i.concepts.includes(it.concepts[0]));
    for (let d = 1; d <= 6; d++) for (const x of same) recordAnswer(s, C, { id: x.id, ev: 'typed', ok: true, now: T0 + d * DAY });
    expect(Object.values(s.notebook)[0].status).toBe('resolved');
  });
});

describe('calendar + planner + readiness', () => {
  it('fills in 5% steps, min 5%, tick only at 100', () => {
    expect(fillPct(40, 0)).toBe(0);
    expect(fillPct(40, 0.5)).toBe(5);
    expect(fillPct(40, 21)).toBe(50);
    expect(fillPct(40, 39.9)).toBe(95);
    expect(fillPct(40, 40)).toBe(100);
    expect(fillPct(40, 400)).toBe(100);
    expect(fillPct(0, 0)).toBe(0);
  });
  it('freezes today planned minutes at first open', () => {
    const s = emptyState(T0); s.profile.onboarded = true;
    const q1 = today(s, C, T0);
    const planned = q1.plannedMin;
    const it = Object.values(C.items)[0];
    recordAnswer(s, C, { id: it.id, ev: 'mcq', ok: true, chosen: it.key, now: T0 + 1000 });
    expect(today(s, C, T0 + 2000).plannedMin).toBe(planned);
  });
  it('plans all lessons before the cutoff when feasible', () => {
    const s = emptyState(T0); s.profile.examDates = { GK: dayKey(T0 + 14 * DAY) }; s.profile.minutesPerDay = 120;
    const p = buildPlan(s, C, T0);
    expect(p.feasibility).toBe('go');
    const planned = p.days.flatMap((d) => d.lessons);
    expect(planned.length).toBe(18);
    const cutoffIdx = p.days.findIndex((d) => d.day === p.cutoff);
    expect(p.days.slice(cutoffIdx + 1).every((d) => d.lessons.length === 0)).toBe(true);
    expect(p.days.some((d) => d.mock)).toBe(true);
  });
  it('says no-go when time is short', () => {
    const s = emptyState(T0); s.profile.examDates = { GK: dayKey(T0 + 3 * DAY) }; s.profile.minutesPerDay = 20;
    expect(buildPlan(s, C, T0).feasibility).toBe('no-go');
  });
  it('binomial reference: Bin(50,.9) ≥ 40 = 0.9906', () => {
    expect(binomTail(50, 0.9, 40)).toBeCloseTo(0.9906, 3);
  });
  it('readiness is low for a new learner and deterministic', () => {
    const s = emptyState(T0);
    const r1 = readiness(s, C, 'GK', T0, 800);
    const r2 = readiness(s, C, 'GK', T0, 800);
    expect(r1).toEqual(r2);
    expect(r1.high).toBeLessThan(0.1);
    expect(r1.band).toBe('not-yet');
  });
  it('mock is test-shaped and unique', () => {
    const s = emptyState(T0);
    const gk = buildMock(s, C, 'GK', 1), cv = buildMock(s, C, 'CV', 1);
    expect(gk.length).toBe(50); expect(new Set(gk).size).toBe(50);
    expect(cv.length).toBe(20); expect(cv.every((id) => C.items[id].test === 'CV')).toBe(true);
  });
});

describe('resume code', () => {
  it('round-trips and rejects tampering', () => {
    const s = emptyState(T0); s.profile.cls = 'A'; s.profile.onboarded = true;
    const it = Object.values(C.items)[5];
    recordAnswer(s, C, { id: it.id, ev: 'mcq', ok: true, chosen: it.key, now: T0 });
    const code = makeResumeCode(s);
    const back = readResumeCode(code, T0);
    expect(back.profile.cls).toBe('A');
    expect(Object.keys(back.cards).length).toBe(1);
    expect(() => readResumeCode(code.slice(0, -4), T0)).toThrow();
    expect(() => readResumeCode('hello', T0)).toThrow();
  });
});
