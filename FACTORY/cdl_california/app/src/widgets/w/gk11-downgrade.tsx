import { useState } from 'preact/hooks';
import type { ComponentChildren } from 'preact';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk11-downgrade', title: 'Brake down a long grade', lesson: 'GK-11', anchor: /proper braking technique/i,
  summary: 'Pick your gear, then step down the hill one second at a time. Brake at your safe speed, release about 5 mph below it, repeat.',
  stamp: { id: 'safe-descent', name: 'Safe descent', rule: 'Do one brake application the handbook way and answer 3 downgrade checks, with no mistakes.' },
};

type Gear = 'low' | 'high' | 'none';
interface Sim { t: number; v: number; brake: boolean; heat: number; gear: Gear; hist: { t: number; v: number; b: boolean }[] }
const RISE: Record<Gear, number> = { low: 0.5, high: 1.5, none: 2.5 }; // mph gained per second with brakes off
const BRAKE = 5 / 3 + 0.5; // low gear: net −5 mph in about 3 s
export const fade = (heat: number) => (heat > 60 ? Math.max(0.2, 1 - (heat - 60) / 80) : 1);
export function tick(s: Sim): Sim {
  const dv = RISE[s.gear] - (s.brake ? BRAKE * fade(s.heat) : 0);
  const v = Math.max(0, Math.round((s.v + dv) * 100) / 100);
  const heat = Math.max(0, s.heat + (s.brake ? 10 : -4));
  return { ...s, t: s.t + 1, v, heat, hist: [...s.hist, { t: s.t + 1, v, b: s.brake }] };
}
const start = (safe: number, gear: Gear): Sim => ({ t: 0, v: safe - 4, brake: false, heat: 0, gear, hist: [{ t: 0, v: safe - 4, b: false }] });

/** The handbook pattern for one safe speed (p. 2-38): rise to safe speed, brake ≈3 s to about 5 below, release, repeat. Drawn before any interaction. */
const pattern = (safe: number) => [[0, safe - 3], [6, safe], [9, safe - 5], [19, safe], [22, safe - 5], [32, safe], [35, safe - 5], [40, safe - 2.5]] as const;
const W = 360;

/** Side view of the grade with the truck on it: at the top (idle) until you start, then moving down with time. */
function GradeStrip({ s }: { s: Sim | null }) {
  const x0 = 44, y0 = 34, x1 = 300, y1 = 80;
  const f = s ? Math.min(1, s.t / 60) : 0;
  const tx = x0 + (x1 - x0) * f, ty = y0 + (y1 - y0) * f, ang = (Math.atan2(y1 - y0, x1 - x0) * 180) / Math.PI;
  const gearTxt = !s ? 'at the top' : s.gear === 'none' ? 'no gear!' : `${s.gear} gear`;
  return (
    <svg viewBox={`0 0 ${W} 100`} width="100%" style={{ display: 'block', maxWidth: '460px', marginInline: 'auto' }} role="img" aria-label={`Side view of a long, steep downgrade with an escape ramp at the bottom. Your truck is ${s ? `${Math.round(f * 100)} percent of the way down, ${gearTxt}` : 'waiting at the top of the grade'}.`}>
      <rect width={W} height="100" fill="var(--surface)" />
      <path d={`M 0 ${y0} L ${x0} ${y0} L ${x1} ${y1} L ${W} ${y1} L ${W} 100 L 0 100 Z`} fill="var(--surface-2)" stroke="none" />
      <path d={`M 0 ${y0} L ${x0} ${y0} L ${x1} ${y1} L ${W} ${y1}`} fill="none" stroke="var(--ink-2)" stroke-width="3" />
      <path d={`M ${x1 + 8} ${y1} L 352 ${y1 - 22}`} fill="none" stroke="var(--amber)" stroke-width="5" stroke-linecap="round" />
      <text x="354" y="46" text-anchor="end" font-size="14" font-weight="700" fill="var(--ink)">escape ramp</text>
      <text x={x0 + 44} y={y0 + 30} font-size="14" fill="var(--ink-2)" transform={`rotate(${ang.toFixed(1)} ${x0 + 44} ${y0 + 30})`}>long, steep downgrade</text>
      <g transform={`translate(${tx.toFixed(1)} ${ty.toFixed(1)}) rotate(${ang.toFixed(1)})`}>
        <rect x="-44" y="-15" width="32" height="13" rx="1.5" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.3" />
        <rect x="-11" y="-15" width="11" height="13" rx="2" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.3" />
      </g>
      {s ? <text x="354" y="20" text-anchor="end" font-size="14" font-weight="700" fill={s.gear === 'none' ? 'var(--red)' : 'var(--ink)'}>{`Truck: ${gearTxt}`}</text>
        : <text x="54" y="22" font-size="14" font-weight="700" fill="var(--ink)">← pick your gear here, at the top</text>}
    </svg>
  );
}

function Chart({ s, safe }: { s: Sim | null; safe: number }) {
  const H = 202, L = 34, R = 350, T = 12, B = 150, span = 40;
  const t0 = s ? Math.max(0, s.t - span) : 0;
  const lo = safe - 15, hi = safe + 15;
  const X = (t: number) => L + ((t - t0) / span) * (R - L);
  const Y = (v: number) => B - ((Math.min(hi, Math.max(lo, v)) - lo) / (hi - lo)) * (B - T);
  const pts = s ? s.hist.filter((p) => p.t >= t0) : [];
  const heat = s?.heat ?? 0;
  const heatW = Math.min(1, heat / 120) * 150;
  const ghost = pattern(safe);
  return (
    <svg viewBox={`0 0 ${W} ${H}`} width="100%" style={{ display: 'block', maxWidth: '460px', marginInline: 'auto' }} role="img" aria-label={s ? `Speed over time. Safe speed ${safe} mph, 5 below is ${safe - 5} mph. Now ${s.v.toFixed(1)} mph at ${s.t} seconds, brakes ${s.brake ? 'on' : 'off'}.` : `Speed chart before you start. Safe speed line at ${safe} mph, release line at ${safe - 5} mph. A faint example shows the handbook pattern: brake at ${safe}, release near ${safe - 5}, let speed come back up, repeat.`}>
      <rect width={W} height={H} fill="var(--surface)" />
      <rect x={L} y={Y(safe)} width={R - L} height={Y(safe - 5) - Y(safe)} fill="var(--ok)" opacity=".14" />
      {[lo, lo + 10, lo + 20, hi].map((v) => <g><line x1={L} x2={R} y1={Y(v)} y2={Y(v)} stroke="var(--line)" /><text x={L - 4} y={Y(v) + 5} text-anchor="end" font-size="14" fill="var(--ink-2)">{v}</text></g>)}
      <line x1={L} x2={R} y1={Y(safe)} y2={Y(safe)} stroke="var(--amber)" stroke-width="2.5" stroke-dasharray="7 4" />
      <line x1={L} x2={R} y1={Y(safe - 5)} y2={Y(safe - 5)} stroke="var(--ok)" stroke-width="2.5" stroke-dasharray="3 3" />
      <text x={R - 2} y={Y(safe) - 6} text-anchor="end" font-size="14" font-weight="700" fill="var(--ink)" stroke="var(--surface)" stroke-width="3" paint-order="stroke">safe {safe} mph — brake here</text>
      <text x={R - 2} y={Y(safe - 5) + 17} text-anchor="end" font-size="14" font-weight="700" fill="var(--ink)" stroke="var(--surface)" stroke-width="3" paint-order="stroke">{safe - 5} mph — release</text>
      {!s && <g aria-hidden="true">
        {ghost.slice(1).map((p, i) => p[1] < ghost[i][1] && <rect x={X(ghost[i][0])} y={B + 2} width={X(p[0]) - X(ghost[i][0])} height={8} fill="var(--red)" opacity=".45" />)}
        <polyline points={ghost.map((p) => `${X(p[0])},${Y(p[1])}`).join(' ')} fill="none" stroke="var(--blue)" stroke-width="2.5" stroke-dasharray="5 4" opacity=".6" />
        <text x={L + 6} y={T + 16} font-size="14" font-weight="700" fill="var(--blue)">example: the handbook pattern</text>
        <text x={X(7.5)} y={Y(safe - 5) + 36} text-anchor="middle" font-size="14" fill="var(--ink)">≈3 s</text>
      </g>}
      {pts.slice(1).map((p, i) => p.b && <rect x={X(pts[i].t)} y={B + 2} width={X(p.t) - X(pts[i].t)} height={8} fill="var(--red)" />)}
      {s && <polyline points={pts.map((p) => `${X(p.t)},${Y(p.v)}`).join(' ')} fill="none" stroke="var(--blue)" stroke-width="3" stroke-linejoin="round" />}
      {s && <circle cx={X(s.t)} cy={Y(s.v)} r="5" fill={s.v > safe + 0.01 ? 'var(--red)' : 'var(--blue)'} stroke="var(--ink)" />}
      <text x={L} y={B + 26} font-size="14" fill="var(--ink-2)"><tspan fill="var(--red)" font-weight="700">■</tspan> brakes on · time →</text>
      <text x={L} y={H - 5} font-size="14" font-weight="700" fill="var(--ink)">Brake heat</text>
      <rect x={122} y={H - 17} width={150} height={12} rx={3} fill="var(--surface-2)" stroke="var(--ink-2)" />
      <rect x={122} y={H - 17} width={heatW} height={12} rx={3} fill={heat > 60 ? 'var(--red)' : 'var(--amber)'} />
      <line x1={122 + 75} x2={122 + 75} y1={H - 20} y2={H - 2} stroke="var(--ink)" stroke-width="1.5" />
      <text x={278} y={H - 5} font-size="14" font-weight="700" fill={heat > 60 ? 'var(--red)' : 'var(--ink-2)'}>{heat > 60 ? 'FADING' : 'ok'}</text>
    </svg>
  );
}

function coach(s: Sim, safe: number): { tone: 'good' | 'bad' | 'info'; text: string } {
  if (s.v >= safe + 10) return { tone: 'bad', text: 'Runaway. The brakes cannot hold the truck back. Use an escape ramp if there is one — know where they are before the grade.' };
  if (s.heat > 60) return { tone: 'bad', text: 'Brake fade: hot brakes need more and more pressure for the same stopping power. Too much braking and not enough engine braking caused it.' };
  if (s.v > safe + 0.01) return { tone: 'bad', text: `Over your safe speed of ${safe} mph. Brake now.` };
  if (s.brake && s.v <= safe - 6) return { tone: 'bad', text: 'Too far below — release. Holding the brakes longer only builds heat.' };
  if (s.brake && s.v <= safe - 4.5) return { tone: 'good', text: `About 5 mph below safe speed (${s.v.toFixed(1)} mph). Release the brakes now.` };
  if (s.brake) return { tone: 'info', text: 'Braking: firm enough for a clear, definite slowdown. Each application takes about 3 seconds.' };
  if (s.v >= safe - 0.01) return { tone: 'good', text: `At your safe speed (${safe} mph). Apply the brakes now.` };
  return { tone: 'info', text: 'Brakes off. The engine holds you back while speed slowly rises to your safe speed.' };
}

interface Q { q: string; opts: string[]; a: number; why: ComponentChildren }
const QS: Q[] = [
  { q: 'Your “safe” speed on a downgrade is 45 mph. Using the handbook method you:', opts: ['Wait until 45 mph, brake to about 40 mph, then release', 'Wait until 50 mph, then brake down to 35 mph', 'Keep light, steady pressure on the brakes all the way down'], a: 0,
    why: <>Brake at your safe speed, down to about 5 mph below it, then release; repeat when speed is back up. Steady use builds heat and causes fade. <span class="plate">p. 2-38</span></> },
  { q: 'Each brake application should last about:', opts: ['1 second', '3 seconds', '10 seconds'], a: 1,
    why: <>The application that brings you about 5 mph below your safe speed should last about 3 seconds. <span class="plate">p. 2-38</span></> },
  { q: 'When do you shift into the right low gear for a long, steep downgrade?', opts: ['Before you start down the grade', 'As soon as your speed starts to build', 'Halfway down, when the brakes feel hot'], a: 0,
    why: <>Shift down before you start down. Once speed builds you may not get into a lower gear — or any gear — and you lose all engine braking. <span class="plate">p. 2-37</span></> },
];

const Pick = ({ on, onClick, children, dis }: { on: boolean; onClick: () => void; children: ComponentChildren; dis?: boolean }) =>
  <button class="btn sm" aria-pressed={on} disabled={dis} onClick={onClick} style={{ width: '100%', paddingInline: '6px', lineHeight: 1.2, ...(on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}) }}>{children}</button>;
/** Equal-column control rows, so buttons never wrap raggedly on a phone. */
const cols = (n: number) => ({ display: 'grid', gridTemplateColumns: `repeat(${n}, minmax(0, 1fr))`, gap: '6px' });

export default function Downgrade({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  const [safe, setSafe] = useState(40);
  const [gear, setGear] = useState<Gear | null>(null);
  const [s, setS] = useState<Sim | null>(null);
  const [late, setLate] = useState(false);
  // challenge
  const [ci, setCi] = useState(0);
  const [cs, setCs] = useState<Sim>(start(40, 'low'));
  const [cres, setCres] = useState<{ ok: boolean; text: string } | null>(null);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const answer = (ok: boolean) => { if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };

  const begin = (g: Gear, sp = safe) => { setGear(g); setS(start(sp, g)); setLate(false); };
  const steps = (n: number) => { if (!s) return; let x = s; for (let k = 0; k < n && x.v < safe + 10; k++) x = tick(x); setS(x); };
  const c = s ? coach(s, safe) : null;
  const runaway = !!s && s.v >= safe + 10;

  const cStep = () => {
    if (cres) return;
    const x = tick(cs); setCs(x);
    if (!x.brake && x.v > 40 + 0.9) { setCres({ ok: false, text: `You let the speed pass 40 mph (now ${x.v.toFixed(1)}). Start braking when you reach your safe speed — not later.` }); answer(false); }
  };
  const cBrake = () => {
    if (cres) return;
    if (!cs.brake) {
      if (cs.v < 40 - 0.01) { setCres({ ok: false, text: `You braked at ${cs.v.toFixed(1)} mph. Don’t apply the brakes until your speed reaches your safe speed (40 mph).` }); answer(false); return; }
      setCs({ ...cs, brake: true });
    } else {
      const ok = cs.v >= 34 && cs.v <= 36;
      setCs({ ...cs, brake: false });
      setCres({ ok, text: ok ? `Released at ${cs.v.toFixed(1)} mph after ${cs.hist.filter((p) => p.b).length} s — about 5 mph below safe speed, about 3 seconds. Now let it rise back to 40 and repeat.` : cs.v > 36 ? `Released at ${cs.v.toFixed(1)} mph — too soon. Brake until about 5 mph below safe speed (40 → 35).` : `Released at ${cs.v.toFixed(1)} mph — held too long. About 5 mph below is enough; longer applications build heat.` });
      answer(ok);
    }
  };
  const resetC = () => { setCi(0); setCs(start(40, 'low')); setCres(null); setPick(null); setMisses(0); };
  const q = QS[ci - 1];

  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => { setMode('challenge'); resetC(); }}>Challenge: 4 checks</button></div>
      {mode === 'explore' && (
        <div class="stack">
          <span class="small" id="dg-safe"><strong>Safe speed for this grade</strong> (set by weight, grade length and steepness, road, weather)</span>
          <div role="group" aria-labelledby="dg-safe" style={cols(3)}>{[35, 40, 45].map((v) => <Pick on={safe === v} onClick={() => { setSafe(v); if (gear) begin(gear, v); }}>{v} mph</Pick>)}</div>
          <span class="small" id="dg-gear"><strong>1. Before you start down, pick a gear:</strong></span>
          <div role="group" aria-labelledby="dg-gear" style={cols(2)}>
            <Pick on={gear === 'low'} onClick={() => begin('low')}>Low gear</Pick><Pick on={gear === 'high'} onClick={() => begin('high')}>Stay in a high gear</Pick></div>
          <GradeStrip s={s} />
          <Chart s={s} safe={safe} />
          {!s && <p class="small muted" style={{ margin: 0 }}>The dashed line is the pattern to copy: brake at your safe speed, release about 5 mph below it (about 3 seconds), let speed come back up, repeat. Engine braking is your main speed control; the service brakes only help it. <span class="plate">p. 2-37</span> <span class="plate">p. 2-38</span></p>}
          {s && <>
            <div class="row" style={{ alignItems: 'baseline' }}><span class="num" style={{ font: '700 1.6rem/1 var(--display)' }}>{s.v.toFixed(1)} mph</span><span class="small muted num">t = {s.t} s · {s.gear === 'none' ? 'no gear!' : `${s.gear} gear`}</span></div>
            <div style={cols(3)}>
              <button class="btn sm" aria-pressed={s.brake} disabled={runaway} onClick={() => setS({ ...s, brake: !s.brake })} style={{ gridColumn: '1 / -1', ...(s.brake ? { background: 'var(--red)', borderColor: 'var(--red)', color: 'var(--surface)' } : {}) }}>{s.brake ? '■ Brakes ON — tap to release' : 'Apply brakes'}</button>
              <button class="btn primary sm" disabled={runaway} onClick={() => steps(1)}>Step +1 s</button>
              <button class="btn sm" disabled={runaway} onClick={() => steps(3)}>+3 s</button>
              <button class="btn sm" onClick={() => begin(s.gear === 'none' ? 'high' : s.gear)}>Restart</button>
            </div>
            {c && <div class={`feedback ${c.tone === 'bad' ? 'bad' : 'good'}`} role="status" aria-live="polite" style={c.tone === 'info' ? { background: 'var(--surface-2)' } : {}}><p class="small" style={{ margin: 0 }}><strong>{c.tone === 'bad' ? '✕ ' : c.tone === 'good' ? '✓ ' : ''}</strong>{c.text} <span class="plate">{c.text.includes('fade') || c.text.includes('Runaway') ? 'p. 2-37' : 'p. 2-38'}</span></p></div>}
            {s.gear === 'high' && <div class="card warn small stack" style={{ padding: '10px 12px', gap: '8px' }}>
              <p style={{ margin: 0 }}><strong>High gear:</strong> little engine braking, so speed climbs fast and the brakes do all the work — they heat up and fade.</p>
              <button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={() => { setLate(true); setS({ ...s, gear: 'none' }); }}>Try to downshift now</button>
            </div>}
            {late && <div class="feedback bad" role="status"><div class="verdict">Missed the downshift</div><p class="small" style={{ margin: 0 }}>Don’t try to downshift after your speed has built up. You won’t get into a lower gear — maybe not into any gear — and you lose all engine braking. Pick the low gear <strong>before</strong> you start down. <span class="plate">p. 2-37</span></p></div>}
          </>}
        </div>
      )}
      {mode === 'challenge' && ci === 0 && (
        <div class="stack">
          <span class="small muted num">Check 1 of 4</span>
          <strong>You are in a low gear. Your safe speed is 40 mph. Step forward and do one brake application the handbook way.</strong>
          <GradeStrip s={cs} />
          <Chart s={cs} safe={40} />
          <div class="row" style={{ alignItems: 'baseline' }}><span class="num" style={{ font: '700 1.6rem/1 var(--display)' }}>{cs.v.toFixed(1)} mph</span><span class="small muted num">t = {cs.t} s</span></div>
          {!cres && <div style={cols(2)}>
            <button class="btn sm" aria-pressed={cs.brake} onClick={cBrake} style={cs.brake ? { background: 'var(--red)', borderColor: 'var(--red)', color: 'var(--surface)' } : {}}>{cs.brake ? '■ Brakes ON — release' : 'Apply brakes'}</button>
            <button class="btn primary sm" onClick={cStep}>Step +1 s</button>
          </div>}
          {cres && <div class={`feedback ${cres.ok ? 'good' : 'bad'}`} role="status"><div class="verdict">{cres.ok ? 'Textbook application' : 'Not the handbook method'}</div><p class="small" style={{ margin: 0 }}>{cres.text} <span class="plate">p. 2-38</span></p>
            <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={() => setCi(1)}>Next</button></div>}
        </div>
      )}
      {mode === 'challenge' && q && (
        <div class="stack">
          <span class="small muted num">Check {ci + 1} of 4</span>
          <strong>{q.q}</strong>
          <div class="stack" role="group" aria-label="Answers" style={{ gap: '6px' }}>{q.opts.map((o, j) => (
            <button class={`btn sm ${pick !== null && j === q.a ? 'primary' : ''}`} disabled={pick !== null} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick !== null && j === q.a ? { opacity: 1 } : {}), ...(pick === j && j !== q.a ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}) }}
              onClick={() => { setPick(j); answer(j === q.a); }}>{pick !== null && j === q.a ? '✓ ' : pick === j ? '✕ ' : ''}{o}</button>
          ))}</div>
          {pick !== null && <div class={`feedback ${pick === q.a ? 'good' : 'bad'}`} role="status"><div class="verdict">{pick === q.a ? 'Right' : `Answer: ${q.opts[q.a]}`}</div><p class="small" style={{ margin: 0 }}>{q.why}</p>
            <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={() => { if (ci === QS.length && misses === 0) onChallenge?.(); setPick(null); setCi(ci + 1); }}>{ci === QS.length ? 'Finish' : 'Next'}</button></div>}
        </div>
      )}
      {mode === 'challenge' && ci > QS.length && (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 4 right — stamp earned' : `${4 - misses} of 4 right`}</div>
          <p class="small" style={{ margin: 0 }}>Low gear before the grade. Brake at safe speed to about 5 below (≈3 s), release, repeat.</p>
          <button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={resetC}>Try again</button></div>
      )}
    </div>
  );
}
