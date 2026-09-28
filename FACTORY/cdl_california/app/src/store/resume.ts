// Resume code: versioned + checksummed state snapshot → deflate → base64url. File export carries the full state.
import { deflateSync, inflateSync, strToU8, strFromU8 } from 'fflate';
import type { AppState } from '../engine/model';
import { migrate } from './migrate';

const PREFIX = 'CDLWS1';

function crc(s: string): string {
  let h = 0x811c9dc5;
  for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 0x01000193) >>> 0; }
  return h.toString(36);
}
function b64url(u: Uint8Array): string {
  let bin = ''; for (let i = 0; i < u.length; i++) bin += String.fromCharCode(u[i]);
  return btoa(bin).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}
function unb64url(s: string): Uint8Array {
  const b = atob(s.replace(/-/g, '+').replace(/_/g, '/') + '==='.slice((s.length + 3) % 4));
  const u = new Uint8Array(b.length); for (let i = 0; i < b.length; i++) u[i] = b.charCodeAt(i); return u;
}

/** Compact snapshot: everything except the long attempt log (keeps the last 150 for diagnosis context). */
export function makeResumeCode(s: AppState): string {
  const snap = { ...s, attempts: s.attempts.slice(-150) };
  const json = JSON.stringify(snap);
  return `${PREFIX}.${crc(json)}.${b64url(deflateSync(strToU8(json), { level: 9 }))}`;
}

export function readResumeCode(code: string, now: number): AppState {
  const clean = code.trim().replace(/\s+/g, '');
  const [p, sum, body] = clean.split('.');
  if (p !== PREFIX || !sum || !body) throw new Error('This is not a CDL Workshop resume code.');
  let json: string;
  try { json = strFromU8(inflateSync(unb64url(body))); } catch { throw new Error('The code is incomplete. Copy the whole code and try again.'); }
  if (crc(json) !== sum) throw new Error('The code was changed or cut off. Copy the whole code and try again.');
  return migrate(JSON.parse(json), now);
}

export function exportFile(s: AppState): string {
  return JSON.stringify({ app: 'cdl-workshop-ca', exportedAt: new Date(s.updatedAt).toISOString(), state: s }, null, 0);
}
export function importFile(text: string, now: number): AppState {
  const j = JSON.parse(text);
  if (j?.app !== 'cdl-workshop-ca' || !j.state) throw new Error('This file is not a CDL Workshop progress file.');
  return migrate(j.state, now);
}
