import { useEffect, useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';
import { DEMO, EX, RUNS, tally, type Ev, type Ex, type Frame, type Pose } from './sk02-backing-course.data';

export const meta: WidgetMeta = {
  id: 'sk02-backing-course', title: 'Examiner’s view: score the backing test', lesson: 'SK-02', anchor: /scoring: encroachments/i,
  summary: 'Watch backing exercises on a coned course step by step with the examiner’s tally. Then score 6 test attempts yourself: encroachments, pull-ups, looks, final position.',
  stamp: { id: 'examiners-eye', name: 'Examiner’s eye', rule: 'Score all 6 replayed test attempts exactly as the examiner would, with no mistakes.' },
};

const H: Record<string, number> = { straight: 92, off: 64, par: 54, alley: 108 };
/** Cropped view box per course [x, y, w, h] so the rig reads large at 390 px (course is not to scale). */
const VB: Record<string, [number, number, number, number]> = { straight: [16, 22, 176, 58], off: [2, 0, 196, 64], par: [18, 0, 164, 54], alley: [16, 0, 120, 108] };
const RS = 1.25; // rig drawing scale
const pressed = (on: boolean) => (on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {});
const TAG: Record<Ev, [string, string]> = { enc: ['ENCROACHMENT', 'var(--red)'], pull: ['PULL-UP', 'var(--amber)'], stop: ['STOP — not a pull-up', 'var(--ink-2)'], look: ['LOOK', 'var(--blue)'], lookBad: ['LOOK — not secured', 'var(--red)'], fwd: ['Forward: part of the exercise', 'var(--ink-2)'] };

function Rig({ p, rm }: { p: Pose; rm: boolean }) {
  const tr = rm ? 'none' : 'transform .7s ease-in-out';
  return (<g>
    <g style={{ transform: `translate(${p.x}px,${p.y}px) rotate(${p.t}deg) scale(${RS})`, transition: tr }}>
      <rect x="-3" y="-4.25" width="40" height="8.5" rx="0.8" fill="var(--surface)" stroke="var(--ink)" stroke-width="0.9" />
      <line x1="30" x2="36" y1="0" y2="0" stroke="var(--ink-2)" stroke-width="0.6" /><rect x="35.5" y="-4.25" width="1.5" height="8.5" fill="var(--red)" />
    </g>
    <g style={{ transform: `translate(${p.x}px,${p.y}px) rotate(${p.c}deg) scale(${RS})`, transition: tr }}>
      <rect x="-17" y="-4" width="19" height="8" rx="1.5" fill="var(--accent)" stroke="var(--ink)" stroke-width="0.9" />
      <rect x="-16" y="-3.2" width="4" height="6.4" rx="0.6" fill="var(--surface-2)" />
      <rect x="-13" y="-5.4" width="1.4" height="1.4" fill="var(--ink)" /><rect x="-13" y="4" width="1.4" height="1.4" fill="var(--ink)" />
    </g>
  </g>);
}

const cone = (x: number, y: number) => <circle cx={x} cy={y} r="2.1" fill="var(--amber)" stroke="var(--ink)" stroke-width="0.6" />;
function courseArt(base: string) {
  const cones: [number, number][] = [], lines: string[] = [];
  const lbl: [number, number, string][] = [];
  if (base === 'straight') { for (let x = 60; x <= 180; x += 12) cones.push([x, 40], [x, 60]); lbl.push([120, 33, '2 rows of cones'], [120, 74, '← cab · trailer backs this way →']); }
  if (base === 'off') { [120, 190].forEach((x) => [10, 30, 50].forEach((y) => cones.push([x, y]))); lines.push('M120 10 H190 M120 30 H190 M120 50 H190'); lbl.push([120, 58, 'first set of cones'], [8, 58, 'outer boundary'], [106, 20, 'lane B'], [106, 40, 'lane A']); }
  if (base === 'par') { [[90, 12], [90, 26], [160, 12], [160, 26], [113, 12], [137, 12]].forEach((c) => cones.push(c as [number, number])); lbl.push([125, 5, 'space'], [100, 48, 'drive past, then back in']); }
  if (base === 'alley') { for (let y = 45; y <= 93; y += 12) cones.push([88, y], [112, y]); lines.push('M88 95 H112'); lbl.push([100, 7, 'outer boundary'], [100, 102, 'back of alley'], [128, 90, '3 ft']); }
  return { cones, lines, lbl };
}

function Course({ ex, frames, k, rm, reveal }: { ex: Ex; frames: Frame[]; k: number; rm: boolean; reveal: boolean }) {
  const e = EX[ex], h = H[e.base], my = (y: number) => (e.mirror ? h - y : y);
  const { cones, lines, lbl } = courseArt(e.base);
  const hit = frames.slice(0, k + 1).filter((f) => f.cone).map((f) => f.cone!);
  const f = frames[k], vb = VB[e.base];
  return (
    <svg viewBox={vb.join(' ')} width="100%" role="img" style={{ display: 'block', background: 'var(--surface-2)', borderRadius: '8px' }}
      aria-label={`${e.name} course, top view, not to scale. Step ${k + 1}: ${f.say}${reveal && hit.length ? ` ${hit.length} cone${hit.length > 1 ? 's' : ''} touched so far.` : ''}`}>
      <g transform={e.mirror ? `translate(0 ${h}) scale(1 -1)` : undefined}>
        {e.base === 'off' && <line x1="8" x2="8" y1="10" y2="50" stroke="var(--ink)" stroke-width="0.8" stroke-dasharray="3 2" />}
        {e.base === 'alley' && <><line x1="10" x2="190" y1="10" y2="10" stroke="var(--ink)" stroke-width="0.8" stroke-dasharray="3 2" /><rect x="88" y="92" width="24" height="3" fill="var(--ok)" opacity=".35" /></>}
        {e.base === 'par' && <path d="M160 12 h14 v14 h-14 z" fill="none" stroke="var(--ink-2)" stroke-width="0.6" />}
        {e.base === 'par' && [0, 4, 8, 12].map((d) => <line x1={160 + d} y1="26" x2={162 + d} y2="12" stroke="var(--ink-2)" stroke-width="0.6" />)}
        {lines.map((d) => <path d={d} fill="none" stroke="var(--ink)" stroke-width="0.8" />)}
        {cones.map(([x, y]) => cone(x, y))}
        <Rig p={f.p} rm={rm} />
      </g>
      {!reveal && f.cone && <circle cx={f.cone[0]} cy={my(f.cone[1])} r="4.5" fill="none" stroke="var(--ink)" stroke-width="1" stroke-dasharray="1.5 1.5" />}
      {reveal && hit.map(([x, y]) => <g><circle cx={x} cy={my(y)} r="3.4" fill="var(--red)" stroke="var(--surface)" stroke-width="0.8" /><text x={x} y={my(y) + 2.4} text-anchor="middle" font-size="6.5" font-weight="700" fill="var(--on-red)">✕</text></g>)}
      {lbl.map(([x, y, t]) => <text x={x} y={my(y)} dominant-baseline="middle" text-anchor={x < 20 ? 'start' : 'middle'} font-size="7" fill="var(--ink)" stroke="var(--surface-2)" stroke-width="2" paint-order="stroke">{t}</text>)}
    </svg>
  );
}

function Replay({ ex, frames, k, setK, rm, reveal = true }: { ex: Ex; frames: Frame[]; k: number; setK: (n: number) => void; rm: boolean; reveal?: boolean }) {
  const [play, setPlay] = useState(false);
  useEffect(() => {
    if (!play) return;
    if (k >= frames.length - 1) { setPlay(false); return; }
    const id = setTimeout(() => setK(k + 1), 1400);
    return () => clearTimeout(id);
  }, [play, k]);
  const f = frames[k];
  return (
    <div class="stack" style={{ gap: '8px' }}>
      <div style={{ maxWidth: '640px', width: '100%', margin: '0 auto' }}><Course ex={ex} frames={frames} k={k} rm={rm} reveal={reveal} /></div>
      <div class="card" style={{ padding: '8px 12px' }} aria-live="polite">
        <div class="spread"><span class="small muted num">Step {k + 1} of {frames.length}</span>
          {reveal && f.ev && <span class="chip" style={{ border: `1.5px solid ${TAG[f.ev][1]}`, color: 'var(--ink)' }}>{TAG[f.ev][0]}</span>}</div>
        <p class="small">{f.say}</p>
      </div>
      <div class="row" role="group" aria-label="Replay controls">
        <button class="btn sm" disabled={k === 0} onClick={() => { setPlay(false); setK(k - 1); }}>◀ Back a step</button>
        <button class="btn sm primary" disabled={k === frames.length - 1} onClick={() => { setPlay(false); setK(k + 1); }}>Next step ▶</button>
        <button class="btn sm" onClick={() => { if (k >= frames.length - 1) setK(0); setPlay(!play); }}>{play ? 'Pause' : 'Play all'}</button>
        <button class="btn sm" onClick={() => { setPlay(false); setK(0); }}>Restart</button>
      </div>
    </div>
  );
}

const ORDER: Ex[] = ['straight', 'offR', 'offL', 'parC', 'parD', 'alley'];
function Explore({ rm }: { rm: boolean }) {
  const [ex, setEx] = useState<Ex>('straight');
  const [k, setK] = useState(0);
  const e = EX[ex], frames = DEMO[e.base], t = tally(frames, k), end = k === frames.length - 1;
  return (
    <div class="stack">
      <div class="row" role="group" aria-label="Exercise" style={{ gap: '6px' }}>{ORDER.map((x) => <button class="btn sm" aria-pressed={x === ex} style={pressed(x === ex)} onClick={() => { setEx(x); setK(0); }}>{EX[x].name}</button>)}</div>
      <div class="card tint"><div class="spread"><strong>{e.name}</strong><span class="plate">p. {e.page}</span></div><p class="small">{e.finish}</p></div>
      <Replay ex={ex} frames={frames} k={k} setK={setK} rm={rm} />
      <div class="card stack" style={{ gap: '4px' }}>
        <div class="spread"><span class="eyebrow">Examiner’s tally so far</span><span class="plate">p. 12-1</span></div>
        <p class="small num">Encroachments: <strong>{t.enc}</strong> · Pull-ups: <strong>{t.pull}</strong> · Looks: <strong>{t.looks} of {e.looks} allowed</strong> · Final position: <strong>{end ? 'met' : 'not yet'}</strong></p>
        <p class="small muted">Each touch of a line or cone = 1 error. A pull-up is stop <em>and</em> pull forward; first ones are free, too many count. A look = open the door, leave the seat, or walk to the back of a bus.</p>
      </div>
    </div>
  );
}

interface Card { enc: number; pull: number; looks: number; within: boolean | null; final: boolean | null; auto: boolean | null }
const BLANK: Card = { enc: 0, pull: 0, looks: 0, within: null, final: null, auto: null };

function Step({ label, v, set }: { label: string; v: number; set: (n: number) => void }) {
  return (<div class="spread" style={{ gap: '8px' }}><span class="small" style={{ fontWeight: 700 }}>{label}</span>
    <span class="row" style={{ gap: '6px', flexWrap: 'nowrap' }}><button class="btn sm" aria-label={`${label}: fewer`} disabled={v === 0} onClick={() => set(v - 1)}>−</button>
      <span class="num" aria-live="polite" aria-label={`${label}: ${v}`} style={{ minWidth: '1.6em', textAlign: 'center', font: '700 1.2rem/1 var(--body)' }}>{v}</span>
      <button class="btn sm" aria-label={`${label}: more`} disabled={v === 5} onClick={() => set(v + 1)}>+</button></span></div>);
}
function YN({ label, v, set, yes, no }: { label: string; v: boolean | null; set: (b: boolean) => void; yes: string; no: string }) {
  return (<div class="spread" style={{ gap: '8px' }}><span class="small" style={{ fontWeight: 700 }}>{label}</span>
    <span class="row" role="group" aria-label={label} style={{ gap: '6px' }}><button class="btn sm" aria-pressed={v === true} style={pressed(v === true)} onClick={() => set(true)}>{yes}</button><button class="btn sm" aria-pressed={v === false} style={pressed(v === false)} onClick={() => set(false)}>{no}</button></span></div>);
}

function Challenge({ onEvidence, onChallenge, concepts, reducedMotion }: WidgetProps) {
  const [i, setI] = useState(0);
  const [k, setK] = useState(0);
  const [c, setC] = useState<Card>(BLANK);
  const [done, setDone] = useState(false);
  const [miss, setMiss] = useState(0);
  const run = RUNS[i];
  if (!run) return (
    <div class={`feedback ${miss === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{miss === 0 ? `${RUNS.length} of ${RUNS.length} scored right — stamp earned: Examiner’s eye` : `${RUNS.length - miss} of ${RUNS.length} scored right — need all ${RUNS.length} for the stamp`}</div>
      <button class="btn sm" onClick={() => { setI(0); setMiss(0); setK(0); setC(BLANK); setDone(false); }}>Score again</button></div>
  );
  const e = EX[run.ex], t = tally(run.frames), within = t.looks <= e.looks;
  const rows: [string, boolean, string][] = [
    ['Encroachments', c.enc === t.enc, `${t.enc}. Each time any part of the vehicle (trailer corner, mirror…) touches or crosses a line or cone is 1 error.`],
    ['Pull-ups', c.pull === t.pull, `${t.pull}. Only stop-and-pull-forward counts. A stop without changing direction is not one, and the forward drive the exercise itself calls for is not a correction.`],
    ['Looks', c.looks === t.looks, `${t.looks}. Opening the door or leaving the seat each count as a look.`],
    ['Within the look limit', c.within === within, `${within ? 'Yes' : 'No'}: ${e.looks === 1 ? 'straight line backing allows only 1 look' : `${e.name} allows at most 2 looks`}.`],
    ['Final position', c.final === run.final.ok, `${run.final.ok ? 'Met' : 'Not met'}: ${run.final.why}`],
    ['Automatic-failure risk', c.auto === t.unsafe, t.unsafe ? 'Yes: getting out without Neutral and the parking brake(s) set may be an automatic failure — the truck could roll.' : 'None: every look was taken with the truck secured and a safe exit (or there were no looks).'],
  ];
  const ok = rows.every((r) => r[1]);
  const ready = c.within !== null && c.final !== null && c.auto !== null;
  return (
    <div class="stack">
      <span class="small muted num">Attempt {i + 1} of {RUNS.length} · {e.name}</span>
      <p class="small">{e.finish} <span class="plate">p. {e.page}</span></p>
      <Replay ex={run.ex} frames={run.frames} k={k} setK={setK} rm={reducedMotion} reveal={done} />
      {!done && <p class="small muted">Watch every step and score it yourself. The examiner’s labels appear after you submit.</p>}
      <fieldset class="card stack" style={{ gap: '8px', margin: 0 }} disabled={done}>
        <legend class="eyebrow">Your score sheet</legend>
        <Step label="Encroachments" v={c.enc} set={(n) => setC({ ...c, enc: n })} />
        <Step label="Pull-ups" v={c.pull} set={(n) => setC({ ...c, pull: n })} />
        <Step label="Looks" v={c.looks} set={(n) => setC({ ...c, looks: n })} />
        <YN label="Looks within the limit?" v={c.within} set={(b) => setC({ ...c, within: b })} yes="Yes" no="No" />
        <YN label="Final position" v={c.final} set={(b) => setC({ ...c, final: b })} yes="Met" no="Not met" />
        <YN label="Automatic-failure risk?" v={c.auto} set={(b) => setC({ ...c, auto: b })} yes="Yes" no="None" />
        {!done && <button class="btn primary" disabled={!ready} onClick={() => { setDone(true); setK(run.frames.length - 1); if (!ok) setMiss(miss + 1); onEvidence({ concepts, ok }); }}>{ready ? 'Submit score sheet' : 'Answer all 3 yes/no rows first'}</button>}
      </fieldset>
      {done && <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status"><div class="verdict">{ok ? 'Scored like the examiner' : 'Not quite — compare with the examiner'}</div>
        <ul style={{ margin: '4px 0', paddingLeft: '1.1em' }}>{rows.map(([n, good, why]) => <li class="small"><strong>{good ? '✓' : '✗'} {n}:</strong> {why}</li>)}</ul>
        <p class="small">Encroachments, pull-ups, looks and final position are the 4 things scored. <span class="plate">p. 12-1</span></p>
        <button class="btn primary sm" onClick={() => { if (i + 1 === RUNS.length && miss === 0) onChallenge?.(); setI(i + 1); setK(0); setC(BLANK); setDone(false); }}>{i + 1 === RUNS.length ? 'Finish' : 'Next attempt'}</button></div>}
    </div>
  );
}

export default function BackingCourse(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Watch runs</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Score 6 runs</button></div>
      {mode === 'explore' ? <Explore rm={props.reducedMotion} /> : <Challenge {...props} />}
    </div>
  );
}
