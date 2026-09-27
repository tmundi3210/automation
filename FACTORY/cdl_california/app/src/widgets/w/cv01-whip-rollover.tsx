import { useEffect, useRef, useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'cv01-whip-rollover', title: 'Crack-the-whip & rollover lab', lesson: 'CV-01', anchor: /steer gently/i,
  summary: 'Run a quick lane change with 3 rigs from Figure 6.1 and watch the swing grow toward the rear. Then load a trailer and see what makes it roll.',
  stamp: { id: 'whip-tamed', name: 'Whip tamed', rule: 'Answer all 5 crack-the-whip and rollover checks with no mistakes.' },
};

/* ---------- Figure 6.1 rigs used here (DL 650 p. 6-1, 6-2) */
interface Rig { id: string; name: string; short: string; ra: number; trailers: number[]; ca?: boolean }
const RIGS: Rig[] = [
  { id: 'semi', name: '5-axle tractor-semitrailer, 45-ft trailer', short: 'Tractor-semitrailer', ra: 1.0, trailers: [90] },
  { id: 'dbl', name: '65-ft conventional double, 27-ft trailers', short: 'Conventional double', ra: 2.0, trailers: [54, 54] },
  { id: 'tri', name: 'Triples, 27-ft trailers', short: 'Triples', ra: 3.5, trailers: [54, 54, 54], ca: true },
];

const clamp01 = (x: number) => Math.max(0, Math.min(1, x));
const smooth = (x: number) => { const u = clamp01(x); return u * u * (3 - 2 * u); };
/** Lane path: from the lower lane (y 120) to the upper lane (y 60). */
const path = (t: number) => 120 - 60 * smooth((t - 0.08) / 0.42);
/** Extra sideways swing of a unit, scaled by its share of the rig's rearward amplification. */
const bump = (t: number) => Math.sin(Math.PI * clamp01((t - 0.2) / 0.5));
const LAG = 0.06, K = 17;

interface Unit { fx: number; fy: number; len: number; deg: number; rx: number; ry: number }
function layout(rig: Rig, t: number): Unit[] {
  const out: Unit[] = [];
  const tf = { x: 336, y: path(t) }, tlen = 34;
  const try_ = path(t - 0.035);
  const tdeg = Math.asin(Math.max(-1, Math.min(1, (try_ - tf.y) / tlen)));
  const tr = { x: tf.x - tlen * Math.cos(tdeg), y: tf.y + tlen * Math.sin(tdeg) };
  out.push({ fx: tf.x, fy: tf.y, len: tlen, deg: (-tdeg * 180) / Math.PI, rx: tr.x, ry: tr.y });
  let hx = tf.x - 26 * Math.cos(tdeg), hy = tf.y + 26 * Math.sin(tdeg);
  const n = rig.trailers.length;
  rig.trailers.forEach((len, i) => {
    const share = (rig.ra - 1) * ((i + 1) / n);
    const u = t - LAG * (i + 1);
    const target = path(u) - share * K * bump(u);
    const s = Math.asin(Math.max(-1, Math.min(1, (target - hy) / len)));
    const rx = hx - len * Math.cos(s), ry = hy + len * Math.sin(s);
    out.push({ fx: hx, fy: hy, len, deg: (-s * 180) / Math.PI, rx, ry });
    hx = rx - 8; hy = ry; // converter dolly gap
  });
  return out;
}
/** Range of sideways travel of each unit's rear over the whole lane change. */
function envelopes(rig: Rig) {
  const lo: number[] = [], hi: number[] = [], xs: number[] = [];
  for (let k = 0; k <= 60; k++) {
    layout(rig, k / 60).forEach((u, i) => { lo[i] = Math.min(lo[i] ?? 999, u.ry); hi[i] = Math.max(hi[i] ?? -999, u.ry); xs[i] = u.rx; });
  }
  return lo.map((l, i) => ({ x: xs[i], lo: l, hi: hi[i], span: hi[i] - l }));
}

function LaneChange({ rig, t }: { rig: Rig; t: number }) {
  const units = layout(rig, t);
  const env = envelopes(rig);
  const last = units.length - 1;
  const offRoad = units[last].ry < 36;
  return (
    <svg viewBox="0 0 360 200" width="100%" role="img" aria-label={`Top view: ${rig.short} in a quick lane change. The sideways swing grows from the tractor to the last trailer, which swings ${rig.ra} times as much.`}>
      <rect x="0" y="0" width="360" height="200" fill="var(--surface-2)" />
      <rect x="0" y="30" width="360" height="120" fill="var(--surface)" stroke="var(--line)" />
      <line x1="0" x2="360" y1="90" y2="90" stroke="var(--amber)" stroke-width="2" stroke-dasharray="14 10" />
      <text x="6" y="22" font-size="13" fill="var(--ink-2)">shoulder</text>
      {env.map((e, i) => (
        <g key={`e${i}`} aria-hidden="true">
          <line x1={e.x} x2={e.x} y1={e.lo} y2={e.hi} stroke={i === last && rig.ra > 1 ? 'var(--red)' : 'var(--blue)'} stroke-width="3" stroke-linecap="round" opacity=".55" />
        </g>
      ))}
      {units.map((u, i) => (
        <g key={i} transform={`translate(${u.fx.toFixed(1)} ${u.fy.toFixed(1)}) rotate(${u.deg.toFixed(2)})`}>
          {i === 0 ? (
            <g><rect x={-u.len} y="-11" width={u.len} height="22" rx="4" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" /><rect x="-12" y="-9" width="9" height="18" rx="2" fill="var(--surface)" stroke="var(--ink)" /></g>
          ) : (
            <rect x={-u.len} y="-12" width={u.len} height="24" rx="2" fill={i === last && rig.ra > 1 ? 'var(--red-soft)' : 'var(--surface-2)'} stroke={i === last && rig.ra > 1 ? 'var(--red)' : 'var(--ink)'} stroke-width="1.5" />
          )}
        </g>
      ))}
      <text x="342" y="178" font-size="13" text-anchor="end" fill="var(--ink)">tractor ×1.0 →</text>
      <text x={Math.max(6, units[last].rx)} y="196" font-size="13" fill={rig.ra > 1 ? 'var(--red)' : 'var(--ink)'} font-weight="700">last trailer ×{rig.ra.toFixed(1)}{offRoad ? ' — swings off the lane!' : ''}</text>
    </svg>
  );
}

function RankBars({ sel }: { sel: string }) {
  return (
    <svg viewBox="0 0 360 96" width="100%" role="img" aria-label="Rearward amplification from Figure 6.1: tractor-semitrailer 1.0, conventional double 2.0, triples 3.5.">
      {RIGS.map((r, i) => {
        const y = 6 + i * 30, w = (r.ra / 3.5) * 150;
        return (
          <g key={r.id}>
            <text x="0" y={y + 16} font-size="13" fill="var(--ink)" font-weight={r.id === sel ? '700' : '400'}>{r.short}</text>
            <rect x="160" y={y + 3} width="150" height="18" rx="3" fill="var(--surface-2)" />
            <rect x="160" y={y + 3} width={w} height="18" rx="3" fill={r.id === sel ? 'var(--accent)' : 'var(--ink-2)'} opacity={r.id === sel ? 1 : 0.45} />
            <text x="316" y={y + 17} font-size="13" fill="var(--ink)" font-weight="700">{r.ra.toFixed(1)}</text>
          </g>
        );
      })}
    </svg>
  );
}

/* ---------- rollover panel (p. 6-1) */
interface Load { loaded: boolean; high: boolean; side: boolean; fast: boolean }
function RollView({ l }: { l: Load }) {
  const lean = (l.fast ? 7 : 0) + (l.loaded ? 2 : 0) + (l.loaded && l.high ? 5 : 0) + (l.loaded && l.side ? 4 : 0);
  const tip = lean >= 14;
  const cgY = !l.loaded ? 96 : l.high ? 58 : 108;
  const cgX = 180 + (l.loaded && l.side ? 22 : 0);
  return (
    <svg viewBox="0 0 360 170" width="100%" role="img" aria-label={`Rear view of the trailer in a turn. Center of gravity ${l.loaded ? (l.high ? 'high' : 'low') : 'empty'}${l.side ? ', load to one side' : ''}. ${tip ? 'Wheels lifting: rollover.' : 'Trailer stays upright.'}`}>
      <rect x="0" y="0" width="360" height="170" fill="var(--surface-2)" />
      <rect x="0" y="146" width="360" height="24" fill="var(--ink-2)" opacity=".35" />
      <text x="354" y="20" font-size="13" text-anchor="end" fill="var(--ink-2)">turn this way →</text>
      <g transform={`rotate(${lean} 244 146)`}>
        <rect x="116" y="30" width="128" height="100" rx="3" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
        {l.loaded && (l.high
          ? <rect x={l.side ? 176 : 132} y="36" width={l.side ? 62 : 96} height="40" fill="var(--amber-soft)" stroke="var(--amber)" stroke-width="2" />
          : <rect x={l.side ? 176 : 122} y="92" width={l.side ? 64 : 116} height="34" fill="var(--amber-soft)" stroke="var(--amber)" stroke-width="2" />)}
        <circle cx={cgX} cy={cgY} r="7" fill="var(--red)" stroke="var(--ink)" />
        <text x={cgX + 11} y={cgY + 5} font-size="13" fill="var(--ink)" font-weight="700">CG</text>
        <rect x="120" y="130" width="22" height="16" rx="3" fill="var(--ink)" /><rect x="218" y="130" width="22" height="16" rx="3" fill="var(--ink)" />
      </g>
      {tip && <text x="12" y="112" font-size="14" font-weight="700" fill="var(--red)">✕ wheels lift</text>}
      {tip && <text x="12" y="130" font-size="14" font-weight="700" fill="var(--red)">— rollover</text>}
    </svg>
  );
}

/* ---------- challenge */
interface Opt { t: string; ok?: boolean; why: string }
interface Q { q: string; opts: Opt[]; rule: string; page: string }
const QS: Q[] = [
  { q: 'You are pulling doubles and must turn the wheel suddenly. Which unit is most likely to turn over?', page: '6-1', rule: 'Rearward amplification grows the swing toward the back, so the last trailer is most likely to tip (crack-the-whip).',
    opts: [{ t: 'The tractor', why: 'The tractor swings the least — it is the start of the whip, not the tip.' }, { t: 'The first trailer', why: 'It feels more “involved”, but the swing keeps growing past it.' }, { t: 'The rear (last) trailer', ok: true, why: 'Right — the tip of the whip swings hardest.' }] },
  { q: 'In Figure 6.1, a rearward amplification of 2.0 means:', page: '6-1', rule: '2.0 = the rear trailer is twice as likely to turn over as the tractor.',
    opts: [{ t: 'The last trailer tips over 2 times as easily as the tractor', ok: true, why: 'Right — it compares the last trailer with the tractor.' }, { t: 'The rig needs twice as much distance to stop', why: 'The number is about rollover in a quick lane change, not stopping.' }, { t: 'The rear trailer off-tracks twice as far', why: 'Off-tracking is about turns and wheel paths, a different idea.' }] },
  { q: 'Which rig in the chart has the LEAST crack-the-whip effect?', page: '6-2', rule: 'The 5-axle tractor-semitrailer with a 45-ft trailer is at the top of Figure 6.1 with 1.0, the lowest value. Triples are highest at 3.5.',
    opts: [{ t: '5-axle tractor-semitrailer, 45-ft trailer', ok: true, why: 'Right — 1.0, the lowest in the chart.' }, { t: '65-ft conventional double, 27-ft trailers', why: 'That rig sits at 2.0 — twice the tractor.' }, { t: 'Triples, 27-ft trailers', why: 'Triples have the most whip in the chart: 3.5.' }] },
  { q: 'A fully loaded rig is how many times more likely to roll over in a crash than an empty rig?', page: '6-1', rule: 'The handbook says 10 times. Some websites say 5 — use 10 on the test.',
    opts: [{ t: '3.5 times', why: '3.5 is the triples’ rearward amplification, not the loaded-vs-empty number.' }, { t: '5 times', why: 'A number from some practice websites; the handbook says 10.' }, { t: '10 times', ok: true, why: 'Right — fully loaded rigs are 10× more likely to roll over.' }] },
  { q: 'Which two things does the handbook say will help you prevent a rollover?', page: '6-1', rule: 'Keep the cargo as close to the ground as possible, and drive slowly around turns.',
    opts: [{ t: 'Heaviest cargo on top; steady speed through turns', why: 'Cargo on top raises the center of gravity — the rig tips more easily.' }, { t: 'Cargo as low as possible; drive slowly around turns', ok: true, why: 'Right — low center of gravity and slow turns.' }, { t: 'Load toward the curb side; brake hard in the turn', why: 'A load to one side makes the trailer lean, and you should slow down before the turn, not in it.' }] },
];

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [i, setI] = useState(0), [pick, setPick] = useState<number | null>(null), [miss, setMiss] = useState(0);
  const q = QS[i];
  if (!q) return (
    <div class={`feedback ${miss === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{miss === 0 ? 'All 5 right — Whip tamed' : `${QS.length - miss} of ${QS.length} right`}</div>
      <p class="small">{miss === 0 ? 'Steer gently, keep cargo low, slow down before turns.' : 'Run the lane change again, then retry for the stamp.'}</p>
      <div><button class="btn sm" onClick={() => { setI(0); setMiss(0); setPick(null); }}>Try again</button></div></div>
  );
  const ok = pick !== null && !!q.opts[pick].ok;
  return (
    <div class="stack">
      <span class="small muted num">Question {i + 1} of {QS.length}</span>
      <strong>{q.q}</strong>
      <div class="stack" role="group" aria-label="Answer choices" style={{ gap: '8px' }}>{q.opts.map((o, k) => (
        <button class="btn" style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick !== null && o.ok ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)' } : pick === k ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }}
          disabled={pick !== null} onClick={() => { setPick(k); if (!o.ok) setMiss(miss + 1); onEvidence({ concepts, ok: !!o.ok }); }}>
          {pick !== null && o.ok ? '✓ ' : pick === k ? '✕ ' : ''}{o.t}</button>
      ))}</div>
      {pick !== null && (
        <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
          <div class="verdict">{ok ? 'Right' : 'Not this one'}</div>
          <p class="small">{q.opts[pick].why}</p>
          <p class="small"><strong>Handbook:</strong> {q.rule} <span class="plate">p. {q.page}</span></p>
          <div><button class="btn primary sm" onClick={() => { if (i + 1 === QS.length && miss === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === QS.length ? 'Finish' : 'Next question'}</button></div>
        </div>
      )}
    </div>
  );
}

const seg = (on: boolean) => (on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {});

export default function WhipRollover(props: WidgetProps) {
  const { reducedMotion } = props;
  const [mode, setMode] = useState<'whip' | 'roll' | 'check'>('whip');
  const [rigId, setRigId] = useState('dbl');
  const [t, setT] = useState(0.62);
  const [playing, setPlaying] = useState(false);
  const raf = useRef(0);
  const [l, setL] = useState<Load>({ loaded: true, high: true, side: false, fast: true });
  const rig = RIGS.find((r) => r.id === rigId)!;

  useEffect(() => {
    if (!playing || reducedMotion) return;
    let start = 0;
    const step = (ts: number) => {
      if (!start) start = ts;
      const nt = Math.min(1, (ts - start) / 3200);
      setT(nt);
      if (nt < 1) raf.current = requestAnimationFrame(step); else setPlaying(false);
    };
    raf.current = requestAnimationFrame(step);
    return () => cancelAnimationFrame(raf.current);
  }, [playing, reducedMotion]);
  useEffect(() => { if (reducedMotion) { setPlaying(false); setT(0.62); } }, [reducedMotion]);

  const notes: string[] = [];
  if (l.loaded && l.high) notes.push('Cargo piled high puts the center of gravity (CG) high above the road — the rig tips over more easily.');
  if (l.loaded && l.side) notes.push('A load to one side makes the trailer lean, so a rollover is more likely. Keep it centered and spread out.');
  if (l.fast) notes.push('Rollovers happen when you turn too fast. Slow down before corners, on-ramps and off-ramps.');
  if (!l.loaded) notes.push('Empty: a fully loaded rig is 10× more likely to roll over in a crash than an empty one.');
  else notes.push('Fully loaded: 10× more likely to roll over in a crash than an empty rig.');

  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'whip'} onClick={() => setMode('whip')}>Lane change</button>
        <button role="tab" aria-selected={mode === 'roll'} onClick={() => setMode('roll')}>Rollover</button>
        <button role="tab" aria-selected={mode === 'check'} onClick={() => setMode('check')}>Challenge (5)</button>
      </div>

      {mode === 'whip' && (
        <div class="stack">
          <div class="row" role="group" aria-label="Pick a rig">{RIGS.map((r) => (
            <button class="btn sm" aria-pressed={r.id === rigId} style={seg(r.id === rigId)} onClick={() => setRigId(r.id)}>{r.short} {r.ra.toFixed(1)}{r.ca ? ' *' : ''}</button>
          ))}</div>
          <LaneChange rig={rig} t={t} />
          <div class="row">
            {!reducedMotion && <button class="btn primary sm" disabled={playing} onClick={() => { setT(0); setPlaying(true); }}>{playing ? 'Changing lanes…' : '▶ Quick lane change'}</button>}
            <div class="field" style={{ flex: '1 1 180px' }}>
              <label for="whip-t">Step through the lane change</label>
              <input id="whip-t" type="range" min={0} max={100} value={Math.round(t * 100)} onInput={(e) => { setPlaying(false); setT(+(e.target as HTMLInputElement).value / 100); }} />
            </div>
          </div>
          <p class="small"><strong>{rig.name}.</strong> The shaded bars show how far each unit’s rear swings sideways. {rig.ra === 1
            ? 'At 1.0 this rig has the least crack-the-whip in Figure 6.1.'
            : `Rearward amplification ${rig.ra.toFixed(1)}: the last trailer is ${rig.ra.toFixed(1)} times as easy to roll over as the tractor. The swing grows through each unit — the tip of the whip.`} <span class="plate">p. 6-1, 6-2</span></p>
          <RankBars sel={rigId} />
          {rig.ca && <p class="small"><span class="ca-tag">CA</span> * Triple trailers are not legal in California <span class="plate">p. 1-5</span>, but the 3.5 number can still be on the test.</p>}
          <div class="card warn small"><strong>Steer gently:</strong> smooth wheel movements; a sudden jerk can tip the trailer. Leave at least 1 second per 10 ft of rig length (+1 second over 40 mph) and look far ahead so you are never forced into a sudden lane change. <span class="plate">p. 6-1</span></div>
        </div>
      )}

      {mode === 'roll' && (
        <div class="stack">
          <div class="grid2">
            <div class="row" role="group" aria-label="Load"><button class="btn sm" aria-pressed={!l.loaded} style={seg(!l.loaded)} onClick={() => setL({ ...l, loaded: false })}>Empty</button><button class="btn sm" aria-pressed={l.loaded} style={seg(l.loaded)} onClick={() => setL({ ...l, loaded: true })}>Fully loaded</button></div>
            <div class="row" role="group" aria-label="Cargo height"><button class="btn sm" disabled={!l.loaded} aria-pressed={!l.high} style={seg(l.loaded && !l.high)} onClick={() => setL({ ...l, high: false })}>Cargo low</button><button class="btn sm" disabled={!l.loaded} aria-pressed={l.high} style={seg(l.loaded && l.high)} onClick={() => setL({ ...l, high: true })}>Cargo high</button></div>
            <div class="row" role="group" aria-label="Cargo position"><button class="btn sm" disabled={!l.loaded} aria-pressed={!l.side} style={seg(l.loaded && !l.side)} onClick={() => setL({ ...l, side: false })}>Centered</button><button class="btn sm" disabled={!l.loaded} aria-pressed={l.side} style={seg(l.loaded && l.side)} onClick={() => setL({ ...l, side: true })}>To one side</button></div>
            <div class="row" role="group" aria-label="Speed in the turn"><button class="btn sm" aria-pressed={!l.fast} style={seg(!l.fast)} onClick={() => setL({ ...l, fast: false })}>Slowed before turn</button><button class="btn sm" aria-pressed={l.fast} style={seg(l.fast)} onClick={() => setL({ ...l, fast: true })}>Too fast in turn</button></div>
          </div>
          <RollView l={l} />
          <ul class="small stack" style={{ gap: '4px', paddingLeft: '18px', margin: 0 }} aria-live="polite">{notes.map((n) => <li>{n}</li>)}</ul>
          <div class="card tint small"><strong>Two things prevent a rollover:</strong> keep the cargo as low as possible, and drive slowly around turns. Rollovers cause more than half of truck-driver crash deaths. <span class="plate">p. 6-1</span></div>
          <p class="small muted">The lean in the picture is a teaching sketch, not a measured angle.</p>
        </div>
      )}

      {mode === 'check' && <Challenge {...props} />}
    </div>
  );
}
