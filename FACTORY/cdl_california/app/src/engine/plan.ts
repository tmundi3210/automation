// Exam-date planner, today's queue, calendar fill, mock-exam builder.
import type { Content, TestId } from '../content/types';
import type { AppState } from './model';
import { addDays, dayKey, daysBetween, dayKeyToDate } from './model';
import { dueSurfaces, kuIndex, lessonDone, nextLesson, openRows, type Surface } from './learner';
import { mulberry32 } from './readiness';
import { TESTS, isWritten } from '../content/tests';

export const REVIEW_MIN_PER_CARD = 0.35; // ~20 s

export interface PlanDay { day: string; lessons: string[]; lessonMin: number; reviewMin: number; mock?: TestId; rest?: boolean; finalReview?: boolean }
export interface Plan { days: PlanDay[]; feasibility: 'go' | 'tight' | 'no-go' | 'no-date'; needMinPerDay: number; cutoff?: string; examDay?: string; message: string }

/**
 * Back-plans from the earliest exam date: new lessons stop at cutoff = exam − max(2, ceil(0.2·days));
 * lessons fill study days in order; a full mock at ~60% of the timeline and on each of the last 2 days.
 */
export function buildPlan(state: AppState, c: Content, now: number): Plan {
  const tests = state.profile.tests;
  const today = dayKey(now);
  const dates = tests.filter(isWritten).map((t) => state.profile.examDates[t]).filter(Boolean) as string[];
  // skills lessons come after the permit, so they are not part of the knowledge-test countdown
  const remaining = c.lessons.filter((l) => tests.includes(l.test) && isWritten(l.test) && !lessonDone(state, l.id));
  const budget = state.profile.minutesPerDay;
  const dueNow = Object.values(state.cards).filter((x) => x.due <= now + 86400_000).length;
  if (!dates.length) {
    // no exam date: a steady pace plan for the next 14 days
    const days: PlanDay[] = [];
    let i = 0, day = today;
    for (let d = 0; d < 14 && i < remaining.length; d++, day = addDays(day, 1)) {
      if (state.profile.offDays.includes(dayKeyToDate(day).getDay())) { days.push({ day, lessons: [], lessonMin: 0, reviewMin: 0, rest: true }); continue; }
      const pd: PlanDay = { day, lessons: [], lessonMin: 0, reviewMin: Math.round(Math.min(budget * 0.4, (d === 0 ? dueNow : 12) * REVIEW_MIN_PER_CARD)) };
      while (i < remaining.length && (pd.lessons.length === 0 || pd.lessonMin + remaining[i].minutes <= budget - pd.reviewMin)) { pd.lessons.push(remaining[i].id); pd.lessonMin += remaining[i].minutes; i++; }
      days.push(pd);
    }
    return { days, feasibility: 'no-date', needMinPerDay: budget, message: 'Set your test date to get a plan that counts down to it.' };
  }
  const exam = dates.sort()[0];
  const total = daysBetween(today, exam);
  if (total < 0) return { days: [], feasibility: 'no-go', needMinPerDay: 0, examDay: exam, message: 'Your test date has passed. Set a new date in Settings.' };
  const cutoff = addDays(exam, -Math.max(2, Math.ceil(0.2 * total)));
  const studyDays: string[] = [];
  for (let d = today; daysBetween(d, cutoff) >= 0; d = addDays(d, 1)) if (!state.profile.offDays.includes(dayKeyToDate(d).getDay())) studyDays.push(d);
  const lessonMin = remaining.reduce((s, l) => s + l.minutes, 0);
  const reviewPerDay = Math.max(5, Math.round(budget * 0.3));
  const avail = Math.max(1, studyDays.length) * Math.max(1, budget - reviewPerDay);
  // minutes/day that make the plan 'go' under the same rule used below (review share = 30 % of the budget),
  // so accepting the suggestion can never produce "not enough time"
  const nd = Math.max(1, studyDays.length);
  const needMinPerDay = Math.ceil(Math.max(lessonMin / (nd * 0.7 * 0.85), lessonMin / nd + 5)) + 1;
  const feasibility = lessonMin <= avail * 0.85 ? 'go' : lessonMin <= avail ? 'tight' : 'no-go';
  const days: PlanDay[] = [];
  let i = 0;
  const perDay = studyDays.length ? lessonMin / studyDays.length : lessonMin;
  let carry = 0;
  for (let d = today; daysBetween(d, exam) >= 0; d = addDays(d, 1)) {
    const off = state.profile.offDays.includes(dayKeyToDate(d).getDay()) && d !== exam;
    const pd: PlanDay = { day: d, lessons: [], lessonMin: 0, reviewMin: reviewPerDay, rest: off };
    if (!off && studyDays.includes(d)) {
      carry += perDay;
      while (i < remaining.length && (carry >= remaining[i].minutes * 0.6 || studyDays[studyDays.length - 1] === d)) {
        pd.lessons.push(remaining[i].id); pd.lessonMin += remaining[i].minutes; carry -= remaining[i].minutes; i++;
      }
    }
    const left = daysBetween(d, exam);
    const wr = tests.filter(isWritten);
    if (!off && wr.length && (left === 1 || left === 2)) { pd.mock = wr[left % wr.length]; pd.finalReview = true; }
    if (!off && wr.length && total >= 5 && daysBetween(today, d) === Math.round(total * 0.6)) pd.mock = wr[0];
    if (d === exam) { pd.reviewMin = 10; pd.lessons = []; pd.lessonMin = 0; }
    days.push(pd);
  }
  const msg = feasibility === 'go' ? `On track: about ${needMinPerDay} min a day until ${fmtDay(cutoff)}, then review and mock tests.`
    : feasibility === 'tight' ? `Tight: you need about ${needMinPerDay} min a day. Add study time or move the date if you can.`
      : `Not enough time: ${lessonMin} min of lessons left but about ${Math.round(avail)} min available. Raise daily minutes to ${needMinPerDay} or move your test date.`;
  return { days, feasibility, needMinPerDay, cutoff, examDay: exam, message: msg };
}

export function fmtDay(k: string): string {
  return dayKeyToDate(k).toLocaleDateString(undefined, { weekday: 'short', month: 'short', day: 'numeric' });
}

// ---------- today
export interface TodayQueue { reviews: Surface[]; fixes: ReturnType<typeof openRows>; lesson: string | null; plannedMin: number; doneMin: number; pct: number }

export function today(state: AppState, c: Content, now: number): TodayQueue {
  const tests = state.profile.tests;
  const ku = kuIndex(c);
  const reviews = dueSurfaces(state, c, now, tests, ku);
  const fixes = openRows(state, c, now).filter((r) => tests.includes(c.lessons.find((l) => l.id === r.lesson)!.test));
  const lesson = nextLesson(state, c, tests);
  const k = dayKey(now);
  const plan = buildPlan(state, c, now);
  const pd = plan.days.find((d) => d.day === k);
  const d = state.days[k] ?? { planned: 0, done: 0 };
  if (!d.frozenAt) {
    // freeze today's planned minutes at first open (denominator must not move during the day)
    const lessonMin = pd ? pd.lessonMin : lesson ? c.lessons.find((l) => l.id === lesson)!.minutes : 0;
    d.planned = pd?.rest ? 0 : Math.max(10, Math.round(lessonMin + Math.min(reviews.length, 60) * REVIEW_MIN_PER_CARD + fixes.length * 2));
    d.frozenAt = now;
    state.days[k] = d;
  }
  return { reviews, fixes, lesson, plannedMin: d.planned, doneMin: d.done, pct: fillPct(d.planned, d.done) };
}

/** Calendar fill in 5 % steps; any work shows at least 5 %; 100 % only when done ≥ planned. */
export function fillPct(planned: number, done: number): number {
  if (planned <= 0) return done > 0 ? 100 : 0;
  if (done <= 0) return 0;
  const r = Math.min(1, done / planned);
  if (r >= 1) return 100;
  return Math.max(5, Math.min(95, Math.floor(r * 20) * 5));
}

export type DayCellState = 'rest' | 'future' | 'missed' | 'partial' | 'done' | 'empty';
export function dayCell(state: AppState, k: string, now: number): { state: DayCellState; pct: number } {
  const t = dayKey(now);
  const d = state.days[k];
  if (daysBetween(t, k) > 0) return { state: 'future', pct: 0 };
  if (!d) return { state: daysBetween(k, t) > 0 && k >= dayKey(state.profile.createdAt) ? 'missed' : 'empty', pct: 0 };
  if (d.planned === 0 && d.done === 0) return { state: 'rest', pct: 0 };
  const pct = fillPct(d.planned, d.done);
  return { state: pct >= 100 ? 'done' : pct > 0 ? 'partial' : k === t ? 'empty' : 'missed', pct };
}

export function activeDaysLast30(state: AppState, now: number): number {
  let n = 0; const t = dayKey(now);
  for (let i = 0; i < 30; i++) { const d = state.days[addDays(t, -i)]; if (d && d.done > 0) n++; }
  return n;
}

// ---------- mock exam builder: test-shaped, stratified by lesson, unseen items first
export function buildMock(state: AppState, c: Content, test: TestId, seed: number): string[] {
  const n = TESTS[test].n ?? 20;
  const items = Object.values(c.items).filter((i) => i.test === test && (i.origin === 'pack' || i.heldOut));
  const seen = new Set(state.attempts.map((a) => a.id));
  const byLesson = new Map<string, string[]>();
  const rnd = mulberry32(seed);
  const shuffled = [...items].sort(() => rnd() - 0.5).sort((a, b) => Number(seen.has(a.id)) - Number(seen.has(b.id)));
  for (const it of shuffled) { const a = byLesson.get(it.lesson) ?? []; a.push(it.id); byLesson.set(it.lesson, a); }
  const out: string[] = [];
  const lessons = [...byLesson.keys()];
  const quota = new Map(lessons.map((l) => [l, Math.round((byLesson.get(l)!.length / items.length) * n)]));
  for (const l of lessons) out.push(...byLesson.get(l)!.slice(0, quota.get(l)));
  const rest = shuffled.map((i) => i.id).filter((id) => !out.includes(id));
  while (out.length < n && rest.length) out.push(rest.shift()!);
  return out.slice(0, n).sort(() => rnd() - 0.5);
}
