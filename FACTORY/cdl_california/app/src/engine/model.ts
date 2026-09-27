// Learner state model. Persisted (see store/). Keep JSON-plain.
import type { TestId } from '../content/types';

export type Cls = 'A' | 'B' | 'C';
export type Cause = 'T' | 'W' | 'N' | 'E4' | 'U';
export const CAUSE_LABEL: Record<Cause, string> = {
  T: 'Trap wording',
  W: 'Wrong source',
  N: 'Number mix-up',
  E4: 'Faded memory',
  U: 'Not learned yet',
};
export const CAUSE_HELP: Record<Cause, string> = {
  T: 'You knew the rule but the question flipped it (NOT, EXCEPT, true/false). Read every stem twice and find the flip word.',
  W: 'You picked an answer from federal rules, the car handbook or a website. The DMV test follows the CA commercial handbook.',
  N: 'You mixed up a number with a neighbouring number. Drill the number family side by side.',
  E4: 'You had this right before, but it faded. A quick review brings it back.',
  U: 'This one has not stuck yet. Re-read the short lesson, then try again.',
};

export type EvidenceClass = 'mcq' | 'tf' | 'typed' | 'self' | 'check' | 'mock';
export const EVIDENCE_WEIGHT: Record<EvidenceClass, number> = { mcq: 0.3, tf: 0.2, typed: 1, self: 0.25, check: 0.5, mock: 0.3 };
export const GUESS_RATE: Record<EvidenceClass, number> = { mcq: 1 / 3, tf: 0.5, typed: 0.05, self: 0.3, check: 0.2, mock: 1 / 3 };

export interface Profile {
  cls: Cls | null;
  airBrakesPassed: boolean;
  tests: TestId[];
  examDates: Partial<Record<TestId, string>>; // yyyy-mm-dd
  minutesPerDay: number;
  offDays: number[]; // 0=Sun..6=Sat
  onboarded: boolean;
  createdAt: number;
  name?: string;
}

export interface CardState {
  due: number; stability: number; difficulty: number; elapsed_days: number; scheduled_days: number;
  learning_steps: number; reps: number; lapses: number; state: number; last_review?: number;
}

export interface Attempt {
  t: number; id: string; ku: string; ev: EvidenceClass; ok: boolean; chosen?: number; guessed?: boolean; rt?: number; cause?: Cause; concepts?: string[];
}

export interface NotebookRow {
  key: string; concept: string; lesson: string; cause: Cause; items: string[];
  status: 'open' | 'probing' | 'resolved' | 'snoozed';
  created: number; updated: number; misses: number;
  probes: { t: number; ok: boolean; typed: boolean }[];
  snoozeUntil?: number;
}

export interface LessonProgress {
  opened?: number; conceptsSeen: Record<string, number>; practiceBest?: number; practiceLast?: { score: number; total: number; t: number }; completed?: number;
}

export interface DayLog { planned: number; done: number; frozenAt?: number }

export interface MockResult {
  t: number; test: TestId; score: number; total: number; pass: boolean; unseenShare: number; predicted?: number; items: { id: string; ok: boolean }[];
}

export interface AppState {
  schema: 1;
  profile: Profile;
  cards: Record<string, CardState>;
  bkt: Record<string, { p: number; n: number; lastT?: number; lastDayGain?: string }>;
  attempts: Attempt[];
  notebook: Record<string, NotebookRow>;
  lessons: Record<string, LessonProgress>;
  days: Record<string, DayLog>;
  mocks: MockResult[];
  stamps: Record<string, number>;
  widgets: Record<string, { done: number; best?: number }>;
  cursor: { route: string; param?: string; scroll?: number };
  prefs: { textSize: 0 | 1 | 2; theme: 'system' | 'light' | 'dark'; reducedMotion: boolean };
  updatedAt: number;
}

export const ATTEMPT_CAP = 2500;

export function emptyState(now: number): AppState {
  return {
    schema: 1,
    profile: { cls: null, airBrakesPassed: true, tests: ['GK', 'CV'], examDates: {}, minutesPerDay: 45, offDays: [], onboarded: false, createdAt: now },
    cards: {}, bkt: {}, attempts: [], notebook: {}, lessons: {}, days: {}, mocks: [], stamps: {}, widgets: {},
    cursor: { route: 'today' }, prefs: { textSize: 0, theme: 'system', reducedMotion: false }, updatedAt: now,
  };
}

export function testsForClass(cls: Cls | null): TestId[] {
  if (cls === 'A') return ['GK', 'CV'];
  return ['GK'];
}

/** Local-time day key with 03:00 rollover (late-night study counts for the previous day). */
export function dayKey(t: number): string {
  const d = new Date(t - 3 * 3600_000);
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
}
export function dayKeyToDate(k: string): Date { const [y, m, d] = k.split('-').map(Number); return new Date(y, m - 1, d, 12); }
export function addDays(k: string, n: number): string { const d = dayKeyToDate(k); d.setDate(d.getDate() + n); return dayKey(d.getTime() + 3 * 3600_000); }
export function daysBetween(a: string, b: string): number { return Math.round((dayKeyToDate(b).getTime() - dayKeyToDate(a).getTime()) / 86400_000); }
