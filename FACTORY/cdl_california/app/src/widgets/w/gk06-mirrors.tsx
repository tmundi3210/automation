import { useState } from 'preact/hooks';
import type { ComponentChildren } from 'preact';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk06-mirrors', title: 'Flat vs convex mirror', lesson: 'GK-06', anchor: /seeing to the sides and rear/i,
  summary: 'Move a car around your truck and compare what the flat mirror and the convex mirror show. Then answer 4 mirror checks.',
  stamp: { id: 'mirror-check', name: 'Mirror check', rule: 'Answer all 4 mirror questions right the first time.' },
};

type Pos = 'far' | 'back' | 'close' | 'beside';
/** Car center x in the top view (left lane, y = 45). Mirror is at x = 300, y = 78. */
const POS: Record<Pos, { x: number; name: string }> = {
  far: { x: 30, name: 'Far back' }, back: { x: 100, name: 'Farther back' }, close: { x: 180, name: 'Near the trailer’s rear' }, beside: { x: 258, name: 'Beside the cab' },
};
const MX = 300, MY = 78, CY = 45;
const FLAT_FOV = Math.atan(58 / 300), CONVEX_FOV = Math.atan(108 / 300);
function view(p: Pos, convex: boolean) {
  const d = MX - POS[p].x, ang = Math.atan((MY - CY) / d), fov = convex ? CONVEX_FOV : FLAT_FOV;
  return { seen: ang <= fov, frac: ang / fov, d };
}

function TopView({ pos }: { pos: Pos }) {
  const car = POS[pos].x;
  return (
    <svg viewBox="0 0 360 150" width="100%" role="img" aria-label={`Top view. Your truck faces right. A car is in the lane to your left: ${POS[pos].name}. The flat mirror covers a narrow strip, the convex mirror a wider one, and a blind spot beside the cab is covered by neither.`}>
      <rect width="360" height="150" fill="var(--surface-2)" />
      <rect y="18" width="360" height="104" fill="var(--surface)" />
      <line x1="0" x2="360" y1="18" y2="18" stroke="var(--ink-2)" stroke-width="2" /><line x1="0" x2="360" y1="122" y2="122" stroke="var(--ink-2)" stroke-width="2" />
      <line x1="0" x2="360" y1="70" y2="70" stroke="var(--ink-2)" stroke-width="2" stroke-dasharray="10 8" />
      <path d={`M${MX} ${MY} L0 ${MY - 300 * Math.tan(CONVEX_FOV)} L0 ${MY} Z`} fill="var(--amber)" opacity=".22" />
      <path d={`M${MX} ${MY} L0 ${MY - 300 * Math.tan(FLAT_FOV)} L0 ${MY} Z`} fill="var(--blue)" opacity=".35" />
      <path d={`M${MX} ${MY} L${MX - 300} ${MY - 300 * Math.tan(CONVEX_FOV)}`} stroke="var(--amber)" stroke-width="1.5" stroke-dasharray="5 4" />
      <path d={`M${MX} ${MY} L0 ${MY - 300 * Math.tan(FLAT_FOV)}`} stroke="var(--blue)" stroke-width="1.5" />
      <path d="M216 20 L300 20 L300 70 L230 70 Z" fill="var(--red)" opacity=".18" stroke="var(--red)" stroke-dasharray="3 3" />
      <text x="262" y="35" text-anchor="middle" font-size="13" font-weight="700" fill="var(--red)">Blind</text><text x="262" y="48" text-anchor="middle" font-size="13" font-weight="700" fill="var(--red)">spot</text>
      <rect x="150" y="82" width="140" height="30" rx="2" fill="var(--surface-2)" stroke="var(--ink)" stroke-width="1.5" />
      <rect x="292" y="83" width="30" height="28" rx="4" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" />
      <rect x="296" y="73" width="8" height="6" fill="var(--ink)" /><text x="176" y="102" font-size="13" fill="var(--ink)">your truck →</text>
      <g transform={`translate(${car} ${CY})`}><rect x="-16" y="-9" width="32" height="18" rx="4" fill="var(--blue)" stroke="var(--ink)" stroke-width="1" /><path d="M20 0 l-5 -5 v10 z" fill="var(--ink)" /></g>
      <text x="6" y="140" font-size="13" fill="var(--ink)"><tspan fill="var(--blue)" font-weight="700">■</tspan> flat view   <tspan fill="var(--amber)" font-weight="700">■</tspan> convex view (wider)</text>
    </svg>
  );
}

/** One mirror face. The truck edge on the right is the reference point. */
function Face({ x, pos, convex, refOn, angled }: { x: number; pos: Pos; convex: boolean; refOn: boolean; angled: boolean }) {
  const v = view(pos, convex);
  const W = 160, H = 128, hz = 44;
  const k = convex ? 0.55 : 1;
  const w = Math.min(70, (8000 / v.d) * k), h = w * 0.62;
  const cx = x + W - 26 - v.frac * (W - 40);
  const by = hz + ((2600 / v.d) * k);
  const clip = `face-${convex ? 'c' : 'f'}`;
  return (
    <g>
      <clipPath id={clip}><rect x={x} y={4} width={W} height={H} rx={convex ? 26 : 6} /></clipPath>
      <g clip-path={`url(#${clip})`}>
        <rect x={x} y={4} width={W} height={H} fill="var(--blue-soft)" />
        <path d={`M${x} ${hz} H${x + W} V${H + 4} H${x} Z`} fill="var(--surface-2)" />
        {(convex ? [0.15, 0.45, 0.8] : [0.35, 0.9]).map((f) => <line x1={x + W - 14} y1={hz} x2={x + W - 14 - f * (W + 60)} y2={H + 4} stroke="var(--ink-2)" stroke-width="2" stroke-dasharray="8 6" />)}
        {refOn && <path d={angled ? `M${x + W} ${hz - 14} L${x + W - 46} ${hz + 2} L${x + W - 70} ${H + 4} H${x + W} Z` : `M${x + W} ${hz - 8} L${x + W - 14} ${hz} L${x + W - 26} ${H + 4} H${x + W} Z`} fill="var(--ink-2)" stroke="var(--ink)" />}
        {v.seen && <g><rect x={cx - w / 2} y={by - h} width={w} height={h} rx={w / 6} fill="var(--blue)" stroke="var(--ink)" stroke-width="1" /><rect x={cx - w / 3} y={by - h + h * 0.15} width={w * 0.66} height={h * 0.35} rx={2} fill="var(--surface)" /></g>}
        {!v.seen && <g><rect x={x + 8} y={hz + 26} width={104} height={20} rx={4} fill="var(--surface)" stroke="var(--red)" /><text x={x + 60} y={hz + 40} text-anchor="middle" font-size="13" font-weight="700" fill="var(--red)">✕ car not in view</text></g>}
      </g>
      <rect x={x} y={4} width={W} height={H} rx={convex ? 26 : 6} fill="none" stroke="var(--ink)" stroke-width="3" />
      <text x={x + W / 2} y={H + 24} text-anchor="middle" font-size="13" font-weight="700" fill="var(--ink)">{convex ? 'Convex (spot)' : 'Flat mirror'}</text>
    </g>
  );
}
function Mirrors({ pos, refOn = true, angled = false }: { pos: Pos; refOn?: boolean; angled?: boolean }) {
  const f = view(pos, false), c = view(pos, true);
  return (
    <svg viewBox="0 0 360 160" width="100%" role="img" aria-label={`Mirror views. Flat mirror: ${f.seen ? 'car seen at true size' : 'car not in view'}. Convex mirror: ${c.seen ? 'car seen, looking smaller and farther away' : 'car not in view'}.`}>
      <rect width="360" height="160" fill="var(--surface)" />
      <Face x={12} pos={pos} convex={false} refOn={refOn} angled={angled} />
      <Face x={188} pos={pos} convex refOn={refOn} angled={angled} />
    </svg>
  );
}

interface Q { q: string; opts: string[]; a: number; why: ComponentChildren; pic?: ComponentChildren }
const QS: Q[] = [
  { q: 'In the convex mirror this car looks small and far back. What is really true?', pic: <Mirrors pos="close" />, opts: ['It is as far away as it looks', 'It is closer and bigger than it looks', 'It is farther away than it looks'], a: 1,
    why: <>A convex mirror shows a wider area, but everything in it looks smaller and farther away than it really is. Allow for this before you move over. <span class="plate">p. 2-13</span></> },
  { q: 'When can you check your mirror adjustment accurately?', opts: ['When the trailer(s) are straight', 'When the trailer is at an angle so you can see its side', 'Only while moving at highway speed'], a: 0,
    why: <>Check adjustment before the start of any trip, and only with the trailer(s) straight — a trailer at an angle changes what the mirror shows. <span class="plate">p. 2-13</span></> },
  { q: 'Why adjust each mirror so it shows some part of your own truck?', opts: ['It removes every blind spot', 'It gives you a reference point for judging where other things are', 'So you can read your gauges in it'], a: 1,
    why: <>Part of your own vehicle in the mirror is a reference point for judging the position of everything else. Mirrors never remove all blind spots. <span class="plate">p. 2-13</span></> },
  { q: 'A car is in the next lane beside your cab. Which mirror shows it?', pic: <TopView pos="beside" />, opts: ['The flat mirror', 'The convex mirror', 'Neither — it is in a blind spot'], a: 2,
    why: <>There are blind spots your mirrors cannot show (Figure 2.7). Check your mirrors regularly so you know when a vehicle has moved into one. <span class="plate">p. 2-13</span></> },
];

function Btn({ on, onClick, children }: { on: boolean; onClick: () => void; children: ComponentChildren }) {
  return <button class="btn sm" aria-pressed={on} onClick={onClick} style={on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}}>{children}</button>;
}

export default function MirrorsWidget({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  const [pos, setPos] = useState<Pos>('back');
  const [refOn, setRefOn] = useState(true);
  const [angled, setAngled] = useState(false);
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const f = view(pos, false), c = view(pos, true);
  const q = QS[i];
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => { setMode('challenge'); setI(0); setPick(null); setMisses(0); }}>Challenge: 4 questions</button></div>
      {mode === 'explore' && (
        <div class="stack">
          <div class="row" role="group" aria-label="Where is the other car?">{(Object.keys(POS) as Pos[]).map((p) => <Btn on={pos === p} onClick={() => setPos(p)}>{POS[p].name}</Btn>)}</div>
          <div style={{ maxWidth: '560px', width: '100%', margin: '0 auto' }} class="stack"><TopView pos={pos} /><Mirrors pos={pos} refOn={refOn} angled={angled} /></div>
          <div class="card tint small" role="status" aria-live="polite" style={{ padding: '10px 12px' }}>
            {f.seen && c.seen && <><strong>Same car, both mirrors.</strong> In the convex mirror it looks <strong>smaller and farther away</strong> than in the flat one — but it is really just as close. It is closer and bigger than it looks. <span class="plate">p. 2-13</span></>}
            {!f.seen && c.seen && <><strong>Only the convex mirror shows it.</strong> A convex mirror shows a <strong>wider area</strong> than a flat mirror — that is why it helps. It still makes the car look smaller and farther away. <span class="plate">p. 2-13</span></>}
            {!c.seen && <><strong>Neither mirror shows it — blind spot.</strong> Mirrors cannot show everything. Regular checks tell you when a vehicle has moved into a blind spot. <span class="plate">p. 2-13</span></>}
          </div>
          <div class="row">
            <label class="toggle"><input type="checkbox" checked={refOn} onChange={(e) => setRefOn((e.target as HTMLInputElement).checked)} />Mirror shows part of my truck</label>
            <label class="toggle"><input type="checkbox" checked={angled} onChange={(e) => setAngled((e.target as HTMLInputElement).checked)} />Trailer at an angle</label>
          </div>
          {(!refOn || angled) && <div class="card warn small" style={{ padding: '10px 12px' }}>
            {!refOn && <p style={{ margin: 0 }}><strong>No reference point.</strong> Adjust each mirror to show some part of your own vehicle, so you can judge where everything else is. <span class="plate">p. 2-13</span></p>}
            {angled && <p style={{ margin: 0 }}><strong>Trailer at an angle:</strong> the trailer swings into the view, so you cannot check adjustment accurately. Check it before every trip, with the trailer(s) straight. <span class="plate">p. 2-13</span></p>}
          </div>}
          <details class="small"><summary><strong>Mirror facts to know</strong></summary>
            <ul style={{ margin: '6px 0 0', paddingLeft: '18px' }}>
              <li><span class="ca-tag">CA</span> 2 or more mirrors, one on the left side, showing the road behind for at least 200 ft (CVC §26709). <span class="plate">p. 2-13</span></li>
              <li>Quick glances — don’t stare. Switch between mirrors and the road ahead.</li>
              <li>Regular checks: traffic (overtaking cars, blind spots) and your vehicle (tires — a tire fire; open cargo — loose straps, a flapping tarp).</li>
              <li>Lane change: check 4 times — before, after signaling, right after starting, after finishing. Also check more in turns, merges and tight maneuvers.</li>
            </ul></details>
        </div>
      )}
      {mode === 'challenge' && (q ? (
        <div class="stack">
          <span class="small muted num">Question {i + 1} of {QS.length}</span>
          {q.pic && <div style={{ maxWidth: '480px', width: '100%' }}>{q.pic}</div>}
          <strong>{q.q}</strong>
          <div class="stack" role="group" aria-label="Answers" style={{ gap: '6px' }}>{q.opts.map((o, j) => (
            <button class={`btn sm ${pick !== null && j === q.a ? 'primary' : ''}`} disabled={pick !== null} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick === j && j !== q.a ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }}
              onClick={() => { setPick(j); const ok = j === q.a; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); }}>{pick !== null && j === q.a ? '✓ ' : pick === j ? '✕ ' : ''}{o}</button>
          ))}</div>
          {pick !== null && <div class={`feedback ${pick === q.a ? 'good' : 'bad'}`} role="status"><div class="verdict">{pick === q.a ? 'Right' : `Answer: ${q.opts[q.a]}`}</div><p class="small" style={{ margin: 0 }}>{q.why}</p>
            <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={() => { if (i + 1 === QS.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === QS.length ? 'Finish' : 'Next'}</button></div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 4 right — stamp earned' : `${QS.length - misses} of ${QS.length} right`}</div>
          <p class="small" style={{ margin: 0 }}>Convex: wider view, things look smaller and farther away. Adjust with trailers straight, showing part of your truck.</p>
          <button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={() => { setI(0); setPick(null); setMisses(0); }}>Try again</button></div>
      ))}
    </div>
  );
}
