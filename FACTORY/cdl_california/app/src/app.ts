// App-level state: one mutable AppState + a version signal that components read to re-render.
import { signal, computed } from '@preact/signals';
import contentJson from './content/content.json';
import type { Content } from './content/types';
import type { AppState } from './engine/model';
import { emptyState } from './engine/model';
import { save, flush } from './store/store';

export const C = contentJson as unknown as Content;
export const now = () => Date.now();

let state: AppState = emptyState(now());
export const version = signal(0);
export const ready = signal(false);

export function S(): AppState { void version.value; return state; }
export function peek(): AppState { return state; }
export function replaceState(s: AppState) { state = s; version.value++; applyPrefs(); }
/** Adopt a restored/erased state and persist it immediately as the newest copy (so a reload keeps it). */
export function adoptState(s: AppState) { s.updatedAt = now(); state = s; version.value++; applyPrefs(); save(s); void flush(s); }
export function mutate(fn: (s: AppState) => void) {
  fn(state);
  state.updatedAt = now();
  version.value++;
  save(state);
}

// ---------- routing (hash tokens restricted to [A-Za-z0-9._~-] so they survive the Artifact viewer)
export type RouteName = 'today' | 'path' | 'lesson' | 'practice' | 'session' | 'mock' | 'notebook' | 'progress' | 'guide' | 'glossary' | 'settings' | 'more' | 'onboarding' | 'widget';
export interface Route { name: RouteName; param?: string; sub?: string }
export const route = signal<Route>({ name: 'today' });

export function go(name: RouteName, param?: string, sub?: string) {
  route.value = { name, param, sub };
  const tok = [name, param, sub].filter(Boolean).join('.').replace(/[^A-Za-z0-9._~-]/g, '');
  try { if (location.hash.slice(1) !== tok) history.pushState(null, '', '#' + tok); } catch { /* sandboxed */ }
  // the cursor includes session/mock so a reload or reopen returns to an unfinished test (boot checks the run still exists)
  if (name !== 'onboarding') mutateQuiet((s) => { s.cursor = { route: name, param, scroll: 0 }; });
  try { window.scrollTo(0, 0); } catch { /* */ }
}
export function routeFromHash(): Route | null {
  const h = (location.hash || '').slice(1);
  if (!h) return null;
  const [name, ...rest] = h.split('.');
  const known: RouteName[] = ['widget', 'today', 'path', 'lesson', 'practice', 'notebook', 'progress', 'guide', 'glossary', 'settings', 'more'];
  if (!known.includes(name as RouteName)) return null;
  // lesson ids contain '-', concept ids contain '.', so rejoin
  if (name === 'lesson') return { name: 'lesson', param: rest[0], sub: rest.slice(1).join('.') || undefined };
  return { name: name as RouteName, param: rest.join('.') || undefined };
}
function mutateQuiet(fn: (s: AppState) => void) { fn(state); save(state); }

// ---------- practice sessions are passed by reference (not in the URL)
export interface SessionSpec { title: string; ids: string[]; mode: 'learn' | 'practice' | 'review' | 'fix' | 'cards'; lesson?: string; back: Route; shuffleOptions?: boolean; testLesson?: string }
export const session = signal<SessionSpec | null>(null);
export function startSession(spec: SessionSpec) {
  session.value = spec;
  mutate((s) => { s.sessionRun = { ...spec, back: { ...spec.back }, i: 0, results: [] }; });
  go('session');
}
/** Reopen an unfinished session after a reload. */
export function restoreSession() {
  const r = state.sessionRun;
  if (!r) return;
  session.value = { title: r.title, ids: r.ids, mode: r.mode, lesson: r.lesson, back: r.back as Route, shuffleOptions: r.shuffleOptions, testLesson: r.testLesson };
  route.value = { name: 'session' };
}
/** Lesson practice test result: best score, and the lesson is complete at 90 %+. */
export function finishLessonTest(id: string, score: number, total: number) {
  const pct = total ? score / total : 0;
  mutate((st) => {
    const lp = st.lessons[id] ?? { conceptsSeen: {} };
    lp.practiceLast = { score, total, t: now() };
    lp.practiceBest = Math.max(lp.practiceBest ?? 0, pct);
    if (pct >= 0.9 && !lp.completed) lp.completed = now();
    st.lessons[id] = lp;
  });
}

// ---------- toasts
export const toast = signal<string | null>(null);
let tt: ReturnType<typeof setTimeout> | null = null;
export function say(msg: string) { toast.value = msg; if (tt) clearTimeout(tt); tt = setTimeout(() => (toast.value = null), 2600); }

export function applyPrefs() {
  try {
    const r = document.documentElement;
    const p = state.prefs;
    // "system" leaves the host's own data-theme (the Artifact viewer stamps one) and only undoes our own override
    if (p.theme === 'system') { if (r.dataset.themeByApp) { r.removeAttribute('data-theme'); delete r.dataset.themeByApp; } }
    else { r.setAttribute('data-theme', p.theme); r.dataset.themeByApp = '1'; }
    r.setAttribute('data-size', String(p.textSize));
    if (p.reducedMotion) r.setAttribute('data-motion', 'reduce'); else r.removeAttribute('data-motion');
  } catch { /* */ }
}

export const lessonById = (id: string) => C.lessons.find((l) => l.id === id)!;
export const conceptMastery = computed(() => { void version.value; return state.bkt; });

// ---------- tap-to-define glossary popover
export const glossOpen = signal<number | null>(null);
