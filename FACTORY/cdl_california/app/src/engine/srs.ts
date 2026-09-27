// FSRS-6 scheduling (ts-fsrs 5.4.2, 21 weights) + grade mapping + BKT mastery per concept.
import { fsrs, createEmptyCard, generatorParameters, Rating, type Card, type Grade } from 'ts-fsrs';
import type { CardState, EvidenceClass } from './model';
import { EVIDENCE_WEIGHT, GUESS_RATE } from './model';

export const DESIRED_RETENTION = 0.9;

function scheduler(maxIntervalDays: number, fuzz = true) {
  return fsrs(generatorParameters({ request_retention: DESIRED_RETENTION, maximum_interval: Math.max(1, Math.round(maxIntervalDays)), enable_fuzz: fuzz }));
}

function toCard(s: CardState): Card {
  return { ...s, due: new Date(s.due), last_review: s.last_review ? new Date(s.last_review) : undefined } as Card;
}
function fromCard(c: Card): CardState {
  return {
    due: c.due.getTime(), stability: c.stability, difficulty: c.difficulty, elapsed_days: c.elapsed_days, scheduled_days: c.scheduled_days,
    learning_steps: c.learning_steps, reps: c.reps, lapses: c.lapses, state: c.state, last_review: c.last_review ? c.last_review.getTime() : undefined,
  };
}

export function newCard(now: number): CardState { return fromCard(createEmptyCard(new Date(now))); }

/** 1 Again · 2 Hard · 3 Good · 4 Easy (MCQ never Easy). */
export function review(s: CardState | undefined, grade: 1 | 2 | 3 | 4, now: number, maxIntervalDays = 365, fuzz = true): CardState {
  const card = s ? toCard(s) : createEmptyCard(new Date(now));
  const r = scheduler(maxIntervalDays, fuzz).next(card, new Date(now), grade as Grade);
  const out = fromCard(r.card);
  // hard cap: never schedule past the exam (ts-fsrs may round one day over its maximum_interval)
  const cap = Math.max(1, Math.floor(maxIntervalDays));
  if (out.scheduled_days > cap) { out.scheduled_days = cap; out.due = now + cap * 86400_000; }
  return out;
}

export function retrievability(s: CardState | undefined, at: number): number {
  if (!s || !s.last_review) return 0;
  return scheduler(36500).get_retrievability(toCard(s), new Date(at), false) as number;
}

/**
 * Grade mapping for recognition items (ENGINE_SPEC §1, review fixes):
 * wrong / "don't know" → Again; correct but "guessed" (toggle set before feedback) → Again;
 * correct but slow (response time per character > 2× the learner's rolling median, only after 20 answers) → Hard; else Good.
 */
export function gradeFor(ev: EvidenceClass, ok: boolean, guessed: boolean, slow: boolean): 1 | 2 | 3 | 4 {
  if (!ok || guessed) return 1;
  if (ev === 'self') return 3;
  if (slow) return 2;
  return 3;
}

export function isSlow(rtMs: number | undefined, chars: number, history: number[]): boolean {
  if (!rtMs || history.length < 20 || chars <= 0) return false;
  const per = rtMs / chars;
  const sorted = [...history].sort((a, b) => a - b);
  const med = sorted[Math.floor(sorted.length / 2)];
  return per > 2 * med;
}

// ---------- BKT (tempered odds update; FIX_SPEC_BKT_EVIDENCE + review fix #3)
export const BKT = { L0: 0.3, S: 0.1, T: 0.15 };

/** Evidence-only update. Correct answers damped on easy items (pHat = predicted chance correct). */
export function bktEvidence(p: number, ok: boolean, ev: EvidenceClass, pHat = 0.5): number {
  const G = GUESS_RATE[ev];
  const w = EVIDENCE_WEIGHT[ev];
  const S = BKT.S;
  const lr = ok ? (1 - S) / G : S / (1 - G);
  const e = ok ? Math.min(1, Math.max(0.25, (1 - pHat) / 0.15)) : 1;
  const odds = p / (1 - p);
  const o2 = odds * Math.pow(lr, w * e);
  return clampP(o2 / (1 + o2));
}
/** Learning transition, applied at most once per concept per day, only if that day had a correct answer. */
export function bktTransition(p: number, w = 0.3): number { return clampP(p + (1 - p) * BKT.T * w); }
function clampP(p: number) { return Math.min(0.995, Math.max(0.005, p)); }

export type MasteryBand = 'new' | 'familiar' | 'proficient' | 'mastered';
export function band(p: number | undefined, n = 0): MasteryBand {
  if (p === undefined || n === 0) return 'new';
  if (p >= 0.95) return 'mastered';
  if (p >= 0.6) return 'proficient';
  return 'familiar';
}
