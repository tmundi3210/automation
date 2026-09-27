// App-level state: one mutable AppState + a version signal that components read to re-render.
import { signal, computed } from '@preact/signals';
import contentJson from './content/content.json';
import type { Content } from './content/types';
import type { AppState } from './engine/model';
import { emptyState } from './engine/model';
import { save } from './store/store';

export const C = contentJson as unknown as Content;
export const now = () => Date.now();

let state: AppState = emptyState(now());
export const version = signal(0);
export const ready = signal(false);

export function S(): AppState { void version.value; return state; }
export function peek(): AppState { return state; }
export function replaceState(s: AppState) { state = s; version.value++; applyPrefs(); }
export function mutate(fn: (s: AppState) => void) {
  fn(state);
  state.updatedAt = now();
  version.value++;
  save(state);
}

// ---------- routing (hash tokens restricted to [A-Za-z0-9._~-] so they survive the Artifact viewer)
export type RouteName = 'today' | 'path' | 'lesson' | 'practice' | 'session' | 'mock' | 'notebook' | 'progress' | 'guide' | 'glossary' | 'settings' | 'more' | 'onboarding';
export interface Route { name: RouteName; param?: string; sub?: string }
export const route = signal<Route>({ name: 'today' });

export function go(name: RouteName, param?: string, sub?: string) {
  route.value = { name, param, sub };
  const tok = [name, param, sub].filter(Boolean).join('.').replace(/[^A-Za-z0-9._~-]/g, '');
  try { if (location.hash.slice(1) !== tok) history.pushState(null, '', '#' + tok); } catch { /* sandboxed */ }
  if (name !== 'session' && name !== 'mock') mutateQuiet((s) => { s.cursor = { route: name, param, scroll: 0 }; });
  try { window.scrollTo(0, 0); } catch { /* */ }
}
export function routeFromHash(): Route | null {
  const h = (location.hash || '').slice(1);
  if (!h) return null;
  const [name, ...rest] = h.split('.');
  const known: RouteName[] = ['today', 'path', 'lesson', 'practice', 'notebook', 'progress', 'guide', 'glossary', 'settings', 'more'];
  if (!known.includes(name as RouteName)) return null;
  // lesson ids contain '-', concept ids contain '.', so rejoin
  if (name === 'lesson') return { name: 'lesson', param: rest[0], sub: rest.slice(1).join('.') || undefined };
  return { name: name as RouteName, param: rest.join('.') || undefined };
}
function mutateQuiet(fn: (s: AppState) => void) { fn(state); save(state); }

// ---------- practice sessions are passed by reference (not in the URL)
export interface SessionSpec { title: string; ids: string[]; mode: 'learn' | 'practice' | 'review' | 'fix' | 'cards'; lesson?: string; back: Route; shuffleOptions?: boolean; onFinish?: (score: number, total: number) => void }
export const session = signal<SessionSpec | null>(null);
export function startSession(spec: SessionSpec) { session.value = spec; go('session'); }

// ---------- toasts
export const toast = signal<string | null>(null);
let tt: ReturnType<typeof setTimeout> | null = null;
export function say(msg: string) { toast.value = msg; if (tt) clearTimeout(tt); tt = setTimeout(() => (toast.value = null), 2600); }

export function applyPrefs() {
  try {
    const r = document.documentElement;
    const p = state.prefs;
    if (p.theme === 'system') r.removeAttribute('data-theme'); else r.setAttribute('data-theme', p.theme);
    r.setAttribute('data-size', String(p.textSize));
    if (p.reducedMotion) r.setAttribute('data-motion', 'reduce'); else r.removeAttribute('data-motion');
  } catch { /* */ }
}

export const lessonById = (id: string) => C.lessons.find((l) => l.id === id)!;
export const conceptMastery = computed(() => { void version.value; return state.bkt; });
