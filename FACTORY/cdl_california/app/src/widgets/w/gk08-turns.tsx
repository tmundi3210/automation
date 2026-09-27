import { useEffect, useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';
import { BUTTON, JUG, CAR, JUG_OPEN, JUG_HIT, startIdx, endIdx, leftTurn, firstHit, type Box, type Frame, type P } from './gk08-turns.sim';

/** Left-turn scene: a car turning beside you in the inside lane (on your left), or in the right-hand lane when you start from the inside. */
const CAR_LEFT: Box = { x: 150, y: 160, w: 22, h: 40 };
const CAR_RIGHT: Box = { x: 188, y: 130, w: 22, h: 40 };

export const meta: WidgetMeta = {
  id: 'gk08-turns', title: 'Turning a rig: button hook vs jug handle', lesson: 'GK-08', anchor: /^space for turns$/i,
  summary: 'Scrub a tractor-semitrailer through a right turn and watch the rear wheels off-track. Compare the button hook with the jug handle, then left turns.',
  stamp: { id: 'wide-right', name: 'Wide right', rule: 'Solve all 4 turning scenarios with no mistakes.' },
};

type Kind = 'button' | 'jug';
const FR: Record<Kind, Frame[]> = { button: BUTTON, jug: JUG };
const RANGE: Record<Kind, [number, number]> = { button: [startIdx(BUTTON), endIdx(BUTTON)], jug: [startIdx(JUG), JUG_HIT] };
const at = (k: Kind, p: number) => { const [a, b] = RANGE[k]; return Math.round(a + (b - a) * p / 100); };
const pts = (ps: P[]) => ps.map((q) => `${q[0].toFixed(1)},${q[1].toFixed(1)}`).join(' ');
const deg = (a: P, b: P) => Math.atan2(a[1] - b[1], a[0] - b[0]) * 180 / Math.PI;

function Body({ a, b, w, fill, over = 0 }: { a: P; b: P; w: number; fill: string; over?: number }) {
  const len = Math.hypot(a[0] - b[0], a[1] - b[1]) + over;
  return <rect transform={`translate(${(a[0] + b[0]) / 2},${(a[1] + b[1]) / 2}) rotate(${deg(a, b)})`} x={-len / 2} y={-w / 2} width={len} height={w} rx="2" fill={fill} stroke="var(--ink)" stroke-width="1.5" />;
}
function Rig({ fr }: { fr: Frame }) {
  return <g aria-hidden="true"><Body a={fr.h} b={fr.r} w={18} fill="var(--surface)" over={14} /><Body a={fr.f} b={fr.h} w={18} fill="var(--accent)" /></g>;
}
function Trace({ frames, i }: { frames: Frame[]; i: number }) {
  const s = frames.slice(0, i + 1);
  return (
    <g aria-hidden="true">
      <polygon points={pts([...s.map((x) => x.f), ...s.map((x) => x.r).reverse()])} fill="var(--amber)" stroke="none" opacity="0.28" />
      <polyline points={pts(s.map((x) => x.f))} fill="none" stroke="var(--ink)" stroke-width="2" stroke-dasharray="6 4" />
      <polyline points={pts(s.map((x) => x.r))} fill="none" stroke="var(--red)" stroke-width="3" />
    </g>
  );
}
const Car = ({ x, y, w = 22, h = 40, fill = 'var(--blue)' }: { x: number; y: number; w?: number; h?: number; fill?: string }) =>
  <rect x={x} y={y} width={w} height={h} rx="6" fill={fill} stroke="var(--ink)" stroke-width="1.5" />;
const Crash = ({ x, y }: { x: number; y: number }) => (
  <g><polygon points={`${x},${y - 18} ${x + 6},${y - 6} ${x + 18},${y - 8} ${x + 9},${y + 2} ${x + 14},${y + 14} ${x},${y + 7} ${x - 14},${y + 14} ${x - 9},${y + 2} ${x - 18},${y - 8} ${x - 6},${y - 6}`} fill="var(--red)" stroke="var(--ink)" stroke-width="1" />
    <text x={x + 22} y={y + 5} font-size="14" font-weight="700" fill="var(--red)">CRASH</text></g>
);

function RightRoads() {
  return (
    <g aria-hidden="true">
      <rect x="0" y="20" width="360" height="320" fill="var(--surface-2)" />
      {[[-100, -100, 160, 210], [140, -100, 400, 210], [-100, 190, 160, 300], [140, 190, 400, 300]].map(([x, y, w, h]) => <rect x={x} y={y} width={w} height={h} rx="24" fill="var(--accent-soft)" stroke="var(--ink-2)" stroke-width="2" />)}
      <path d="M100 20 V110 M100 190 V340 M0 150 H60 M140 150 H360" stroke="var(--amber)" stroke-width="2" fill="none" />
      <line x1="100" y1="196" x2="140" y2="196" stroke="var(--ink-2)" stroke-width="3" />
    </g>
  );
}
function RightScene({ k, p, label, choices, oncoming }: { k: Kind | null; p: number; label: string; choices?: boolean; oncoming?: boolean }) {
  const frames = k ? FR[k] : null;
  const i = k ? at(k, p) : 0;
  const hit = k === 'jug' && i >= JUG_HIT;
  return (
    <svg viewBox="0 20 360 320" width="100%" role="img" aria-label={label} style={{ display: 'block', maxWidth: '520px' }}>
      <RightRoads />
      {choices && ([['jug', 'A'], ['button', 'B']] as const).map(([c, l]) => {
        const s = FR[c].slice(FR[c].findIndex((x) => x.f[1] <= 335), endIdx(FR[c]));
        const lp = s[Math.round(s.length * (c === 'jug' ? 0.12 : 0.62))].f;
        return <g key={c}><polyline points={pts(s.map((x) => x.f))} fill="none" stroke={c === 'jug' ? 'var(--blue)' : 'var(--ink)'} stroke-width="3" stroke-dasharray={c === 'jug' ? '2 5' : '8 4'} stroke-linecap="round" />
          <circle cx={lp[0] + (c === 'jug' ? -16 : 0)} cy={lp[1] - (c === 'jug' ? 0 : 16)} r="11" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
          <text x={lp[0] + (c === 'jug' ? -16 : 0)} y={lp[1] - (c === 'jug' ? 0 : 16) + 5} text-anchor="middle" font-size="14" font-weight="700" fill="var(--ink)">{l}</text></g>;
      })}
      {frames && <Trace frames={frames} i={i} />}
      {k === 'jug' && i >= JUG_OPEN && <Car {...CAR} />}
      {oncoming && <Car x={250} y={119} w={40} h={22} fill="var(--amber)" />}
      {frames && <Rig fr={frames[i]} />}
      {hit && <Crash x={CAR.x + 11} y={CAR.y + 8} />}
    </svg>
  );
}

type Lane = 'inside' | 'right';
type Start = 'soon' | 'center';
function LeftScene({ lane, start, label, choices }: { lane: Lane | null; start: Start | null; label: string; choices?: 'start' | 'lane' }) {
  const fr = lane && start ? leftTurn(lane, start) : null;
  const car = lane === 'inside' ? CAR_RIGHT : CAR_LEFT;
  const hit = fr ? firstHit(fr, car) : -1;
  const show = fr ? (hit >= 0 ? hit : fr.findIndex((x) => x.f[0] <= 70)) : 0;
  return (
    <svg viewBox="0 20 360 320" width="100%" role="img" aria-label={label} style={{ display: 'block', maxWidth: '520px' }}>
      <g aria-hidden="true">
        <rect x="0" y="20" width="360" height="320" fill="var(--surface-2)" />
        {[[-100, -100, 160, 140], [220, -100, 400, 140], [-100, 200, 160, 300], [220, 200, 400, 300]].map(([x, y, w, h]) => <rect x={x} y={y} width={w} height={h} rx="24" fill="var(--accent-soft)" stroke="var(--ink-2)" stroke-width="2" />)}
        <path d="M140 20 V40 M140 200 V340 M0 120 H60 M220 120 H360" stroke="var(--amber)" stroke-width="2" fill="none" />
        <path d="M100 206 V340 M180 206 V340 M0 80 H60 M0 160 H60 M220 80 H360 M220 160 H360" stroke="var(--ink-2)" stroke-width="1.5" stroke-dasharray="8 8" fill="none" />
        <line x1="140" y1="206" x2="220" y2="206" stroke="var(--ink-2)" stroke-width="3" />
        <text x="160" y="232" text-anchor="middle" font-size="20" fill="var(--ink-2)">↰</text><text x="200" y="232" text-anchor="middle" font-size="20" fill="var(--ink-2)">↰</text>
      </g>
      {choices === 'start' && ([['soon', 'A'], ['center', 'B']] as const).map(([s, l]) => {
        const f = leftTurn('right', s).map((x) => x.f).filter((q) => q[0] > 30 && q[1] < 330);
        const y0 = s === 'soon' ? 215 : 125;
        return <g key={s}><polyline points={pts(f)} fill="none" stroke={s === 'soon' ? 'var(--blue)' : 'var(--ink)'} stroke-width="3" stroke-dasharray={s === 'soon' ? '2 5' : '8 4'} stroke-linecap="round" />
          <circle cx="238" cy={y0} r="11" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" /><text x="238" y={y0 + 5} text-anchor="middle" font-size="14" font-weight="700" fill="var(--ink)">{l}</text></g>;
      })}
      {choices === 'lane' && <g><Body a={[160, 250]} b={[160, 330]} w={18} fill="var(--surface)" /><Body a={[200, 250]} b={[200, 330]} w={18} fill="var(--surface)" />
        <text x="160" y="296" text-anchor="middle" font-size="14" font-weight="700" fill="var(--ink)">A</text><text x="200" y="296" text-anchor="middle" font-size="14" font-weight="700" fill="var(--ink)">B</text></g>}
      {fr && <Trace frames={fr} i={show} />}
      {fr && <Car {...car} fill={lane === 'inside' ? 'var(--amber)' : 'var(--blue)'} />}
      {fr && <Rig fr={fr[show]} />}
      <g aria-hidden="true"><circle cx="140" cy="120" r="5" fill="var(--accent)" stroke="var(--ink)" /><text x="136" y="112" text-anchor="end" font-size="13" font-weight="700" fill="var(--ink)">center</text></g>
      {hit >= 0 && <Crash x={car.x + 11} y={car.y + 10} />}
      {fr && <text x={lane === 'right' ? 232 : 110} y={start === 'soon' ? 220 : 130} font-size="16" font-weight="700" text-anchor={lane === 'right' ? 'start' : 'end'} fill={start === 'soon' ? 'var(--red)' : 'var(--ok)'}>{start === 'soon' ? '✗ too soon' : '✓'}</text>}
    </svg>
  );
}

const Legend = () => (
  <p class="small muted" aria-hidden="true">
    <span style={{ borderTop: '2px dashed var(--ink)', display: 'inline-block', width: '22px', verticalAlign: 'middle' }} /> front wheels ·{' '}
    <span style={{ borderTop: '3px solid var(--red)', display: 'inline-block', width: '22px', verticalAlign: 'middle' }} /> trailer rear wheels ·{' '}
    <span style={{ background: 'var(--amber)', opacity: 0.4, display: 'inline-block', width: '14px', height: '12px', verticalAlign: 'middle' }} /> swept path
  </p>
);
const seg = (on: boolean) => on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {};

function Explore({ reducedMotion }: { reducedMotion: boolean }) {
  const [view, setView] = useState<Kind | 'left'>('button');
  const [p, setP] = useState(0);
  const [play, setPlay] = useState(false);
  const [lane, setLane] = useState<Lane>('right');
  const [start, setStart] = useState<Start>('center');
  useEffect(() => {
    if (!play) return;
    if (reducedMotion) { setP(100); setPlay(false); return; }
    let raf = 0; let v = p;
    const step = () => { v = Math.min(100, v + 0.6); setP(v); if (v < 100) raf = requestAnimationFrame(step); else setPlay(false); };
    raf = requestAnimationFrame(step);
    return () => cancelAnimationFrame(raf);
  }, [play]);
  const pick = (v: Kind | 'left') => { setView(v); setP(0); setPlay(false); };
  const k = view === 'left' ? null : view;
  const i = k ? at(k, p) : 0;
  const msg = k === 'button'
    ? p < 35 ? 'Go slowly, straight past the corner. The back of the trailer stays near the curb, so no one can sneak past on your right.'
      : p < 80 ? 'Turn wide as you COMPLETE the turn — the front uses part of the far lane. If that is the oncoming lane, give oncoming cars room or stop; never back up.'
        : 'Done. The rear wheels cut inside (off-tracking) but stayed off the curb, and the right side stayed closed.'
    : k === 'jug'
      ? i >= JUG_HIT ? 'Crash: as you finish the turn, the trailer cuts in to the right and hits the car that pulled up beside you.'
        : i >= JUG_OPEN ? 'A driver behind thinks you are turning left and pulls up on your right side — into the gap you opened.'
          : 'Swinging out to the left at the START of the turn opens a gap on your right.'
      : '';
  return (
    <div class="stack">
      <div class="row" role="group" aria-label="Choose a turn">
        <button class="btn sm" aria-pressed={view === 'button'} style={seg(view === 'button')} onClick={() => pick('button')}>✓ Button hook</button>
        <button class="btn sm" aria-pressed={view === 'jug'} style={seg(view === 'jug')} onClick={() => pick('jug')}>✗ Jug handle</button>
        <button class="btn sm" aria-pressed={view === 'left'} style={seg(view === 'left')} onClick={() => pick('left')}>Left turns</button>
      </div>
      {k && <>
        <RightScene k={k} p={p} label={`Top-down right turn, ${k === 'button' ? 'button hook' : 'jug handle'}, ${Math.round(p)} percent through. ${msg}`} />
        <Legend />
        <div class="row">
          <div class="field" style={{ flex: '1 1 180px' }}><label for="tn-p">Turn progress</label>
            <input id="tn-p" type="range" min={0} max={100} step={1} value={p} aria-valuetext={`${Math.round(p)} percent`} onInput={(e) => { setPlay(false); setP(+(e.target as HTMLInputElement).value); }} /></div>
          <button class="btn sm" onClick={() => { if (p >= 100) setP(0); setPlay(!play); }}>{play ? 'Pause' : p >= 100 ? 'Replay' : 'Play'}</button>
        </div>
        <div class={`feedback ${k === 'button' ? 'good' : 'bad'}`} role="status" aria-live="polite">
          <div class="verdict">{k === 'button' ? 'Button hook — correct' : 'Jug handle — incorrect'}</div>
          <p class="small">{msg} <span class="plate">p. 2-20</span> <span class="plate">Fig. 2.13</span></p>
        </div>
        <p class="small muted">Off-tracking: the rear wheels follow a shorter path than the front wheels, cutting inside the turn. Longer rigs off-track more. <span class="plate">p. 6-3</span></p>
      </>}
      {view === 'left' && <>
        <div class="row" role="group" aria-label="When to start turning"><span class="small"><strong>Start turning:</strong></span>
          <button class="btn sm" aria-pressed={start === 'soon'} style={seg(start === 'soon')} onClick={() => setStart('soon')}>At the stop line</button>
          <button class="btn sm" aria-pressed={start === 'center'} style={seg(start === 'center')} onClick={() => setStart('center')}>At the center</button></div>
        <div class="row" role="group" aria-label="Which left-turn lane"><span class="small"><strong>Two left-turn lanes:</strong></span>
          <button class="btn sm" aria-pressed={lane === 'inside'} style={seg(lane === 'inside')} onClick={() => setLane('inside')}>Inside (left) lane</button>
          <button class="btn sm" aria-pressed={lane === 'right'} style={seg(lane === 'right')} onClick={() => setLane('right')}>Right-hand lane</button></div>
        <LeftScene lane={lane} start={start} label={`Left turn from the ${lane === 'right' ? 'right-hand' : 'inside'} lane, starting ${start === 'soon' ? 'at the stop line (too soon)' : 'at the center of the intersection'}.`} />
        <Legend />
        <div class={`feedback ${start === 'center' ? 'good' : 'bad'}`} role="status"><p class="small">{start === 'center' ? '✓ Reach the center of the intersection before you turn.' : '✗ Turning too soon: off-tracking makes the left side of your trailer cut in and hit the vehicle on your left.'} <span class="plate">p. 2-20</span></p></div>
        <div class={`feedback ${lane === 'right' ? 'good' : 'bad'}`} role="status"><p class="small">{lane === 'right' ? '✓ Right-hand lane: drivers on your left are easier to see.' : '✗ From the inside lane you may have to swing right to finish the turn — into the traffic in the other lane.'} <span class="plate">p. 2-21</span> <span class="plate">Fig. 2.14</span></p></div>
      </>}
    </div>
  );
}

interface Sc { q: string; opts: string[]; a: number; why: string; page: string; bad: string[] }
const SC: Sc[] = [
  { q: 'Your rig cannot make this right turn without using part of another lane. Which path do you drive?', opts: ['Path A', 'Path B'], a: 1, page: '2-20',
    why: 'Path B is the button hook: keep the back near the curb and turn wide as you complete the turn, so no one can sneak past on your right.',
    bad: ['Path A is the jug handle: you swing left first, a driver pulls up on your right, and you hit them as you finish the turn.'] },
  { q: 'Mid-turn, your front is in the oncoming lane and a car is coming toward you. What do you do?', opts: ['Back up to give it room', 'Leave it room to get by, or stop and wait', 'Speed up to finish before it arrives'], a: 1, page: '2-20',
    why: 'Leave oncoming vehicles room to get by, or stop and wait. Do not back up.',
    bad: ['Backing up: you could hit someone behind you.', '', 'Speeding up: turn slowly — it gives you and others more time to avoid problems.'] },
  { q: 'You are turning left. Where do you start the turn?', opts: ['A — at the stop line', 'B — at the center of the intersection'], a: 1, page: '2-20',
    why: 'Don’t begin a left turn until you reach the center of the intersection.',
    bad: ['Too soon: off-tracking can make the left side of your vehicle hit another vehicle.'] },
  { q: 'There are two left-turn lanes. Which one do you use?', opts: ['A — inside (left) lane', 'B — right-hand lane'], a: 1, page: '2-21',
    why: 'Take the right-hand turn lane. Drivers on your left are easier to see.',
    bad: ['From the inside lane you may have to swing right to make the turn, into the other lane’s traffic.'] },
];

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const sc = SC[i];
  if (!sc) return (
    <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 4 right — stamp earned' : `${SC.length - misses} of ${SC.length} right`}</div>
      {misses > 0 && <p class="small">Try again with no mistakes to earn the “Wide right” stamp.</p>}
      <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); }}>Try again</button></div>
  );
  const ok = pick === sc.a;
  const answer = (c: number) => { if (pick !== null) return; setPick(c); if (c !== sc.a) setMisses(misses + 1); onEvidence({ concepts, ok: c === sc.a }); };
  const vis = i === 0 ? (pick === null ? <RightScene k={null} p={0} choices label="Two possible front-wheel paths, A and B, for a right turn." />
      : <RightScene k={pick === 1 ? 'button' : 'jug'} p={100} label={pick === 1 ? 'Button hook completed safely.' : 'Jug handle: the trailer hits the car on the right.'} />)
    : i === 1 ? <RightScene k="button" p={52} oncoming label="Truck mid-turn in the oncoming lane with a car approaching." />
      : i === 2 ? <LeftScene lane={pick === null ? null : 'right'} start={pick === null ? null : pick === 1 ? 'center' : 'soon'} choices={pick === null ? 'start' : undefined} label="Left turn: path A starts at the stop line, path B at the center." />
        : <LeftScene lane={pick === null ? null : pick === 1 ? 'right' : 'inside'} start={pick === null ? null : 'center'} choices={pick === null ? 'lane' : undefined} label="Two left-turn lanes: A is the inside lane, B is the right-hand lane." />;
  return (
    <div class="stack">
      <span class="small muted num">Scenario {i + 1} of {SC.length}</span>
      <strong>{sc.q}</strong>
      {vis}
      <div class="row" role="group" aria-label="Answers">{sc.opts.map((o, c) => (
        <button class="btn sm" disabled={pick !== null} aria-pressed={pick === c}
          style={pick !== null && c === sc.a ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)' } : pick === c ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}}
          onClick={() => answer(c)}>{o}{pick !== null && c === sc.a ? ' ✓' : pick === c ? ' ✗' : ''}</button>
      ))}</div>
      {pick !== null && <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
        <div class="verdict">{ok ? 'Right' : 'Not that one'}</div>
        {!ok && sc.bad[pick] && <p class="small"><strong>Consequence:</strong> {sc.bad[pick]}</p>}
        <p class="small">{sc.why} <span class="plate">p. {sc.page}</span></p>
        <button class="btn primary sm" onClick={() => { if (i + 1 === SC.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === SC.length ? 'Finish' : 'Next'}</button>
      </div>}
    </div>
  );
}

export default function Turns(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Challenge: 4 turns</button></div>
      {mode === 'explore' ? <Explore reducedMotion={props.reducedMotion} /> : <Challenge {...props} />}
    </div>
  );
}
