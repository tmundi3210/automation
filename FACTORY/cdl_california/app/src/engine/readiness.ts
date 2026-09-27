// Readiness estimate (ENGINE_SPEC §5 + review fix #9): on-device Monte Carlo over the real test format.
// Each draw samples the test's question count from the pack items under a weighting scheme, adds a latent
// ability offset, and counts passes. Output is a RANGE across schemes; the band uses the lower bound.
import type { Content, TestId } from '../content/types';
import type { AppState } from './model';
import { retrievability, BKT } from './srs';

export const TEST_FORMAT: Record<TestId, { n: number; pass: number; name: string }> = {
  GK: { n: 50, pass: 40, name: 'General Knowledge' },
  CV: { n: 20, pass: 16, name: 'Combination Vehicles' },
};

export function mulberry32(seed: number) {
  return () => { seed |= 0; seed = (seed + 0x6d2b79f5) | 0; let t = Math.imul(seed ^ (seed >>> 15), 1 | seed); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
}
const logit = (p: number) => Math.log(p / (1 - p));
const sigm = (x: number) => 1 / (1 + Math.exp(-x));

/** Probability the learner answers this item right on exam day. */
export function itemP(state: AppState, c: Content, itemId: string, at: number, calib = { a: 1, b: -0.2 }): number {
  const it = c.items[itemId];
  const card = state.cards[it.ku];
  const conceptP = it.concepts.length ? it.concepts.map((x) => state.bkt[x]?.p ?? BKT.L0).reduce((s, v) => s + v, 0) / it.concepts.length : BKT.L0;
  let know: number;
  if (card?.last_review) {
    const R = retrievability(card, at);
    const recent = state.attempts.filter((a) => a.ku === it.ku).slice(-2);
    const lastOk = recent.length ? recent[recent.length - 1].ok : true;
    know = 0.6 * R * (lastOk ? 1 : 0.5) + 0.4 * conceptP;
  } else {
    know = conceptP * 0.85;
  }
  know = Math.min(0.98, Math.max(0.02, know));
  const pRight = know + (1 - know) / 3; // 3 options: a non-knower still guesses
  return sigm(calib.a * logit(Math.min(0.99, Math.max(0.01, pRight))) + calib.b);
}

export interface Readiness { test: TestId; low: number; high: number; band: 'not-yet' | 'borderline' | 'likely'; studiedShare: number; strongReady: boolean; draws: number }

export function readiness(state: AppState, c: Content, test: TestId, at: number, draws = 4000, seed = 42): Readiness {
  const items = Object.values(c.items).filter((i) => i.test === test && i.origin === 'pack');
  const ps = items.map((i) => itemP(state, c, i.id, at));
  const weightsHigh = items.map((i) => (c.lessons.find((l) => l.id === i.lesson)!.weight === 'High' ? 2 : 1));
  const schemes = [weightsHigh, items.map(() => 1)];
  const { n, pass } = TEST_FORMAT[test];
  const rnd = mulberry32(seed);
  const gauss = () => { const u = Math.max(1e-9, rnd()), v = rnd(); return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v); };
  const results: number[] = [];
  for (const w of schemes) {
    const cum: number[] = []; let s = 0; for (const x of w) { s += x; cum.push(s); }
    let passes = 0;
    for (let d = 0; d < draws; d++) {
      const theta = gauss() * 0.35;
      let right = 0;
      for (let k = 0; k < n; k++) {
        const r = rnd() * s; let lo = 0, hi = cum.length - 1;
        while (lo < hi) { const mid = (lo + hi) >> 1; if (cum[mid] < r) lo = mid + 1; else hi = mid; }
        const p = sigm(logit(ps[lo]) + theta);
        if (rnd() < p) right++;
      }
      if (right >= pass) passes++;
    }
    results.push(passes / draws);
  }
  const low = Math.min(...results), high = Math.max(...results);
  const studied = items.filter((i) => state.cards[i.ku]?.last_review).length / items.length;
  const mocks = state.mocks.filter((m) => m.test === test).slice(-2);
  const strongReady = mocks.length === 2 && mocks.every((m) => m.score / m.total >= 0.9);
  const band = low >= 0.9 ? 'likely' : low >= 0.5 ? 'borderline' : 'not-yet';
  return { test, low, high, band, studiedShare: studied, strongReady, draws };
}

/** Exact binomial tail — reference used by tests (Bin(50,.9) ≥ 40 = 0.9906). */
export function binomTail(n: number, p: number, k: number): number {
  let s = 0;
  for (let i = k; i <= n; i++) s += comb(n, i) * Math.pow(p, i) * Math.pow(1 - p, n - i);
  return s;
}
function comb(n: number, k: number): number { let r = 1; for (let i = 1; i <= k; i++) r = (r * (n - k + i)) / i; return r; }
