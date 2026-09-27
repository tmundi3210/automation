import { useEffect, useState } from 'preact/hooks';
import { adoptState, go, mutate, now, peek, S, say } from '../app';
import { dayKey, emptyState, testsForClass } from '../engine/model';
import type { Cls } from '../engine/model';
import { makeResumeCode, readResumeCode, exportFile, importFile } from '../store/resume';
import { flush, getStatus, onStatus, wipeLocal } from '../store/store';
import type { SyncStatus } from '../store/store';

const DAYS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

async function saveFile(name: string, text: string): Promise<string> {
  const rt = (globalThis as unknown as { claude?: { use(n: string): Promise<unknown> } }).claude;
  if (rt?.use) {
    const dl = (await rt.use('downloads')) as { save(r: { filename: string; data: string }): Promise<void> } | null;
    if (dl) { try { await dl.save({ filename: name, data: text }); return 'Saved.'; } catch (e) { const c = (e as { code?: string }).code; return c === 'declined' ? 'Save cancelled.' : 'Saving files is not available here. Use the resume code instead.'; } }
  }
  try {
    const a = document.createElement('a');
    a.href = URL.createObjectURL(new Blob([text], { type: 'application/json' }));
    a.download = name; document.body.appendChild(a); a.click(); a.remove();
    return 'Downloaded.';
  } catch { return 'Saving files is not available here. Use the resume code instead.'; }
}

export function SettingsScreen() {
  const s = S();
  const p = s.profile;
  const [code, setCode] = useState('');
  const [paste, setPaste] = useState('');
  const [msg, setMsg] = useState('');
  const [confirmReset, setConfirmReset] = useState(false);
  const [sync, setSync] = useState<SyncStatus>(getStatus());
  useEffect(() => onStatus(setSync), []);
  const setCls = (c: Cls) => mutate((st) => { st.profile.cls = c; st.profile.tests = testsForClass(c); });
  return (
    <div class="page">
      <header class="stack" style={{ gap: '6px' }}><span class="eyebrow">Settings</span><h1>Your plan and your progress</h1></header>
      <section class="card stack" aria-label="License and tests">
        <h2>License and tests</h2>
        <div class="field"><label for="cls">License class</label>
          <select id="cls" value={p.cls ?? ''} onChange={(e) => setCls((e.target as HTMLSelectElement).value as Cls)}>
            <option value="" disabled>Choose…</option><option value="A">Class A (tractor-trailer)</option><option value="B">Class B (heavy truck or bus)</option><option value="C">Class C (small HazMat/passenger)</option>
          </select>
          <span class="small muted">{p.cls === 'A' ? 'Tests in this app: General Knowledge + Combination Vehicles.' : p.cls ? 'Tests in this app: General Knowledge. Your class also needs endorsement tests this version does not cover yet.' : 'Not sure? The first lesson has a class finder.'}</span>
        </div>
        {p.tests.map((t) => (
          <div class="field"><label for={`d-${t}`}>{t === 'GK' ? 'General Knowledge' : 'Combination Vehicles'} test date</label>
            <input id={`d-${t}`} type="date" min={dayKey(now())} value={p.examDates[t] ?? ''} onChange={(e) => mutate((st) => { const v = (e.target as HTMLInputElement).value; if (v) st.profile.examDates[t] = v; else delete st.profile.examDates[t]; })} /></div>
        ))}
        <label class="toggle"><input type="checkbox" checked={p.airBrakesPassed} onChange={(e) => mutate((st) => { st.profile.airBrakesPassed = (e.target as HTMLInputElement).checked; })} /> I already passed the Air Brakes test</label>
        {!p.airBrakesPassed && p.cls === 'A' && <p class="small card info">Air Brakes (25 questions, pass 20) is needed if your truck has air brakes. This version does not teach it yet: study Section 5 of the California Commercial Driver Handbook.</p>}
        <div class="field"><label for="mpd">Minutes a day</label>
          <input id="mpd" type="number" min={10} max={240} step={5} value={p.minutesPerDay} onChange={(e) => mutate((st) => { st.profile.minutesPerDay = Math.max(10, Math.min(240, +(e.target as HTMLInputElement).value || 45)); })} /></div>
        <fieldset class="field" style={{ border: 0, padding: 0, margin: 0 }}><legend style={{ fontWeight: 700, marginBottom: '6px' }}>Days off (no new lessons)</legend>
          <div class="row">{DAYS.map((d, i) => (
            <label class="toggle"><input type="checkbox" checked={p.offDays.includes(i)} onChange={(e) => mutate((st) => { const on = (e.target as HTMLInputElement).checked; st.profile.offDays = on ? [...st.profile.offDays, i] : st.profile.offDays.filter((x) => x !== i); })} />{d}</label>
          ))}</div>
        </fieldset>
      </section>

      <section class="card stack" aria-label="Save and move progress">
        <h2>Save or move your progress</h2>
        <p class="small">{sync === 'device+account' ? 'Your progress saves automatically to your account and this device.' : sync === 'device' ? 'Your progress saves automatically on this device.' : 'This browser is not saving progress. Copy a resume code before you leave.'} To continue on another device, copy a resume code there, or save a progress file.</p>
        <div class="row">
          <button class="btn primary" onClick={() => { void flush(peek()); setCode(makeResumeCode(peek())); }}>Make a resume code</button>
          <button class="btn" onClick={async () => setMsg(await saveFile(`cdl-workshop-progress-${dayKey(now())}.json`, exportFile(peek())))}>Save progress file</button>
        </div>
        {code && (
          <div class="field"><label for="rc">Your resume code ({code.length.toLocaleString()} characters)</label>
            <textarea id="rc" readOnly value={code} onFocus={(e) => (e.target as HTMLTextAreaElement).select()} />
            <button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={async () => { try { await navigator.clipboard.writeText(code); setMsg('Copied.'); } catch { const el = document.getElementById('rc') as HTMLTextAreaElement; el.focus(); el.select(); setMsg('Press copy on your keyboard or menu.'); } }}>Copy</button>
          </div>
        )}
        <div class="field"><label for="rp">Restore from a resume code</label>
          <textarea id="rp" value={paste} onInput={(e) => setPaste((e.target as HTMLTextAreaElement).value)} placeholder="Paste a code that starts with CDLWS1." />
          <button class="btn sm" style={{ alignSelf: 'flex-start' }} disabled={!paste.trim()} onClick={() => { try { const st = readResumeCode(paste, now()); adoptState(st); setPaste(''); setMsg('Progress restored.'); } catch (e) { setMsg((e as Error).message); } }}>Restore</button>
        </div>
        <div class="field"><label for="rf">Restore from a progress file</label>
          <input id="rf" type="file" accept=".json,application/json" onChange={async (e) => { const f = (e.target as HTMLInputElement).files?.[0]; if (!f) return; try { const st = importFile(await f.text(), now()); adoptState(st); setMsg('Progress restored.'); } catch (er) { setMsg((er as Error).message); } }} />
        </div>
        {msg && <p role="status" class="small"><strong>{msg}</strong></p>}
      </section>

      <section class="card stack" aria-label="Display">
        <h2>Display</h2>
        <div class="field"><label for="th">Theme</label><select id="th" value={s.prefs.theme} onChange={(e) => mutate((st) => { st.prefs.theme = (e.target as HTMLSelectElement).value as 'system'; })}><option value="system">Match my device</option><option value="light">Light</option><option value="dark">Dark</option></select></div>
        <div class="field"><label for="ts">Text size</label><select id="ts" value={s.prefs.textSize} onChange={(e) => mutate((st) => { st.prefs.textSize = +(e.target as HTMLSelectElement).value as 0; })}><option value={0}>Normal</option><option value={1}>Large</option><option value={2}>Extra large</option></select></div>
        <label class="toggle"><input type="checkbox" checked={s.prefs.reducedMotion} onChange={(e) => mutate((st) => { st.prefs.reducedMotion = (e.target as HTMLInputElement).checked; })} /> Reduce motion</label>
      </section>

      <section class="card stack" aria-label="Start over">
        <h2>Start over</h2>
        {!confirmReset ? <button class="btn" onClick={() => setConfirmReset(true)}>Erase my progress…</button> : (
          <div class="card warn stack"><strong>Erase all progress on this device?</strong><p class="small">This cannot be undone. Make a resume code first if you might want it back.</p>
            <div class="row"><button class="btn" onClick={() => setConfirmReset(false)}>Keep my progress</button><button class="btn primary" style={{ background: 'var(--red)', borderColor: 'var(--red)', color: '#fff' }} onClick={async () => { await wipeLocal(); adoptState(emptyState(now())); setConfirmReset(false); say('Progress erased.'); go('onboarding'); }}>Erase everything</button></div></div>
        )}
      </section>
    </div>
  );
}
