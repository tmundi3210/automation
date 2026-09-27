import type { AppState } from '../engine/model';
import { emptyState } from '../engine/model';

/** Fill any fields added after a save was written; drop unknown schema versions to a fresh state. */
export function migrate(s: AppState, now: number): AppState {
  if (!s || (s as { schema?: number }).schema !== 1) return emptyState(now);
  const base = emptyState(now);
  return {
    ...base, ...s,
    profile: { ...base.profile, ...s.profile },
    prefs: { ...base.prefs, ...(s.prefs ?? {}) },
    cursor: { ...base.cursor, ...(s.cursor ?? {}) },
    widgets: s.widgets ?? {}, stamps: s.stamps ?? {}, mocks: s.mocks ?? [], days: s.days ?? {},
    notebook: s.notebook ?? {}, lessons: s.lessons ?? {}, bkt: s.bkt ?? {}, cards: s.cards ?? {}, attempts: s.attempts ?? [],
  };
}
