// 3-tier StorageAdapter: memory → browser storage (IndexedDB, then localStorage) → Artifact per-viewer db.
// Boot from local immediately; when the Artifact runtime answers, hydrate + merge by updatedAt (newest wins),
// then mirror writes (debounced, one write at a time per doc). Every browser call is wrapped in try/catch.
import { get as idbGet, set as idbSet, del as idbDel } from 'idb-keyval';
import type { AppState } from '../engine/model';
import { emptyState } from '../engine/model';
import { migrate } from './migrate';

const KEY = 'cdlws.state.v1';
export type SyncStatus = 'memory' | 'device' | 'device+account';

interface ClaudeRuntime { use(name: string): Promise<unknown> }
type DocRef = { get(): Promise<{ exists: boolean; data(): Record<string, unknown> | undefined }>; set(d: Record<string, unknown>): Promise<void> };
type DB = { doc(path: string): DocRef };
type UserNS = { id(): Promise<string | null> };

let remote: { core: DocRef; cards: DocRef; log: DocRef } | null = null;
let status: SyncStatus = 'memory';
const listeners = new Set<(s: SyncStatus) => void>();
export const onStatus = (f: (s: SyncStatus) => void) => { listeners.add(f); f(status); return () => listeners.delete(f); };
const setStatus = (s: SyncStatus) => { status = s; listeners.forEach((f) => f(s)); };
export const getStatus = () => status;

export async function loadLocal(now: number): Promise<AppState> {
  let raw: unknown = null;
  try { raw = await idbGet(KEY); if (raw) setStatus('device'); } catch { /* blocked */ }
  if (!raw) {
    try { const s = localStorage.getItem(KEY); if (s) { raw = JSON.parse(s); setStatus('device'); } } catch { /* blocked */ }
  }
  if (status === 'memory') { try { await idbSet(KEY + '.probe', 1); await idbDel(KEY + '.probe'); setStatus('device'); } catch { /* memory only */ } }
  return raw ? migrate(raw as AppState, now) : emptyState(now);
}

let saveTimer: ReturnType<typeof setTimeout> | null = null;
let remoteBusy = false;
let remotePending: AppState | null = null;
let lastRemote = { core: '', cards: '', log: '' };

export function save(state: AppState) {
  if (saveTimer) clearTimeout(saveTimer);
  saveTimer = setTimeout(() => void flush(state), 600);
}

export async function flush(state: AppState) {
  const snapshot = JSON.parse(JSON.stringify(state)) as AppState;
  try { await idbSet(KEY, snapshot); } catch {
    try { localStorage.setItem(KEY, JSON.stringify(snapshot)); } catch { /* memory only */ }
  }
  void pushRemote(snapshot);
}

function split(s: AppState) {
  const { cards, attempts, ...core } = s;
  return {
    core: { ...core } as Record<string, unknown>,
    cards: { cards, updatedAt: s.updatedAt } as Record<string, unknown>,
    // keep the per-viewer log doc under the 256 KiB cap: last 1200 attempts
    log: { attempts: attempts.slice(-1200), updatedAt: s.updatedAt } as Record<string, unknown>,
  };
}

async function pushRemote(s: AppState) {
  if (!remote) return;
  if (remoteBusy) { remotePending = s; return; }
  remoteBusy = true;
  try {
    const parts = split(s);
    for (const k of ['core', 'cards', 'log'] as const) {
      const body = JSON.stringify(parts[k]);
      if (body === lastRemote[k]) continue; // only on change
      await remote[k].set(parts[k]);
      lastRemote[k] = body;
    }
  } catch { /* transient; next save retries */ }
  remoteBusy = false;
  if (remotePending) { const p = remotePending; remotePending = null; void pushRemote(p); }
}

/** Connect to the Artifact runtime's per-viewer private store, if this page runs inside one. */
export async function connectRemote(local: AppState, now: number): Promise<AppState | null> {
  const rt = (globalThis as unknown as { claude?: ClaudeRuntime }).claude;
  if (!rt || typeof rt.use !== 'function') return null;
  try {
    const [db, user] = (await Promise.all([rt.use('db'), rt.use('user')])) as [DB | null, UserNS | null];
    if (!db || !user) return null;
    const uid = await user.id();
    if (!uid) return null;
    const base = `data/users/${uid}`;
    remote = { core: db.doc(`${base}/core`), cards: db.doc(`${base}/cards`), log: db.doc(`${base}/log`) };
    const [core, cards, log] = await Promise.all([remote.core.get(), remote.cards.get(), remote.log.get()]);
    setStatus('device+account');
    if (core.exists) {
      const c = core.data() as unknown as AppState;
      if ((c.updatedAt ?? 0) > (local.updatedAt ?? 0)) {
        const merged = {
          ...c,
          cards: ((cards.exists ? cards.data()?.cards : null) ?? {}) as AppState['cards'],
          attempts: ((log.exists ? log.data()?.attempts : null) ?? []) as AppState['attempts'],
        } as AppState;
        return migrate(merged, now);
      }
    }
    void pushRemote(local);
    return null;
  } catch {
    remote = null;
    return null;
  }
}

export async function wipeLocal() {
  try { await idbDel(KEY); } catch { /* */ }
  try { localStorage.removeItem(KEY); } catch { /* */ }
}
