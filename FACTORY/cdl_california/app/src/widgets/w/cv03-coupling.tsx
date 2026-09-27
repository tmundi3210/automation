import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';
import { COUPLE, UNCOUPLE, START_COUPLE, START_UNCOUPLE, rigAfter, type Rig, type Fx, type Kind, type Step } from './cv03-coupling.steps';

export const meta: WidgetMeta = {
  id: 'cv03-coupling', title: 'Coupling simulator: 16 steps, then 10 to uncouple', lesson: 'CV-03', anchor: /^coupling a tractor-semitrailer/i,
  summary: 'Step through the handbook’s coupling and uncoupling order on a live rig. Then pick each next action yourself — wrong picks show what goes wrong.',
  stamp: { id: 'coupled-clean', name: 'Coupled clean', rule: 'Pick all 16 coupling steps in the handbook’s order with no wrong picks.' },
};

const W = 640, H = 290, GY = 262;
const INK = 'var(--ink)';
/** Tire rubber: dark in both themes; the light --ink-2 rim keeps it visible on the dark background. */
const TIRE = '#262b28';
const TX: Record<Rig['tractor'], number> = { far: 196, front: 236, touch: 300, under: 390, partly: 295, clear: 196 };
const FX_TEXT: Record<Fx, string> = {
  roll: 'Trailer rolls', low: 'Hits the trailer nose', high: 'May not couple', gap: 'Gap: kingpin on closed jaws', angle: 'Landing gear damaged',
  snag: 'Gear can snag tracks', drop: 'Trailer drops', torn: 'Air lines torn', kingpin: 'Hits kingpin hard', nobrakes: 'No trailer brakes', hurt: 'Injury risk',
  level: 'Fifth wheel not tilted', dirt: 'Dirt and water get in', order: '',
};
const KIND: Record<Kind, string> = { order: 'Out of order', setup: 'Wrong setup', skip: 'Skipped check' };
const verdictOf = (w: { kind: Kind; fx: Fx }) => KIND[w.kind] + (w.fx !== 'order' ? ` — ${FX_TEXT[w.fx]}` : '');

function Wheel({ x, r = 20 }: { x: number; r?: number }) {
  return <g><circle cx={x} cy={GY - r} r={r} fill={TIRE} stroke="var(--ink-2)" stroke-width="2" /><circle cx={x} cy={GY - r} r={r * 0.5} fill="var(--surface-2)" stroke="var(--ink-2)" stroke-width="2" /></g>;
}

function Scene({ rig, fx, focus, reducedMotion }: { rig: Rig; fx: Fx | null; focus: string; reducedMotion: boolean }) {
  const tr = reducedMotion ? 'none' : 'transform .7s ease';
  let cx = TX[rig.tractor];
  let py = rig.tractor === 'under' ? 206 : rig.height === 'ok' ? 210 : 196;
  let gap = false, rollX = 0, rot = 0;
  if (fx === 'high') py = 186;
  if (fx === 'low') { py = 224; cx = 312; }
  if (fx === 'gap') { py = 196; cx = 390; gap = true; }
  if (fx === 'drop' && (rig.tractor === 'under' || rig.tractor === 'partly')) cx = TX.clear;
  if (fx === 'roll') rollX = 46;
  if (fx === 'angle') rot = -4;
  const dropping = fx === 'drop';
  const coupled = rig.jaws === 'locked' && (rig.tractor === 'under' || gap);
  const foot = dropping ? GY : rig.gear === 'down' || rig.tractor !== 'under' ? GY : rig.gear === 'slight' ? GY - 7 : py + 30;
  const gearY = fx === 'snag' ? GY - 16 : foot;
  const nose = 330 + rollX;
  const lines = rig.air && fx !== 'torn' && fx !== 'dirt';
  const cab = cx - 78;
  const hl = (k: string) => (focus === k ? { stroke: 'var(--amber)', 'stroke-width': 4 } : { stroke: INK, 'stroke-width': 2 });
  const tilt = rig.tilt && !coupled && fx !== 'level' ? 7 : 0;
  return (
    <svg viewBox={`0 56 ${W} ${H - 56}`} width="100%" role="img" style={{ borderRadius: '8px', background: 'var(--bg)', display: 'block' }}
      aria-label={`Side view. Tractor ${rig.tractor === 'under' ? 'under the trailer' : rig.tractor === 'partly' ? 'partly clear, frame still under the trailer' : rig.tractor === 'touch' ? 'just touching the trailer' : 'in front of the trailer'}; landing gear ${rig.gear}; air lines ${rig.air ? 'connected' : 'not connected'}; chocks ${rig.chocks ? 'in place' : 'off'}.${fx && fx !== 'order' ? ' Problem: ' + FX_TEXT[fx] + '.' : ''}`}>
      <rect x="0" y={GY} width={W} height={H - GY} fill="var(--surface-2)" />
      <line x1="0" y1={GY} x2={W} y2={GY} stroke="var(--ink-2)" stroke-width="2" />
      {rig.look === 'supports' && <path d={`M${cx - 60} 150 L${462} ${GY - 4} L${440} ${GY - 4} Z`} fill="var(--amber)" opacity=".35" />}
      {/* trailer */}
      <g style={{ transform: `translate(${rollX}px, ${dropping ? 26 : 0}px) rotate(${dropping ? 4 : 0}deg)`, transformOrigin: '600px 240px', transition: tr }}>
        <g style={{ transform: `translate(0px, ${py - 206}px)`, transition: tr }}>
          <rect x="330" y="60" width="316" height="128" rx="4" fill="var(--surface)" stroke={INK} stroke-width="2" />
          {[370, 410, 450, 490, 530, 570, 610].map((x) => <line x1={x} y1="64" x2={x} y2="184" stroke="var(--line)" stroke-width="2" />)}
          <rect x="330" y="188" width="316" height="18" fill="var(--surface-2)" {...hl('upper')} />
          <path d="M383 206 L397 206 L397 214 L401 220 L379 220 L383 214 Z" fill="var(--surface)" {...hl('kingpin')} />
          <rect x="324" y="100" width="8" height="12" rx="2" fill="var(--ink-2)" />
          <rect x="322" y="122" width="10" height="11" rx="3" fill="var(--red)" stroke={INK} stroke-width="1.5" />
          <rect x="322" y="138" width="10" height="11" rx="3" fill="var(--blue)" stroke={INK} stroke-width="1.5" />
          <rect x="560" y="206" width="80" height="12" fill="var(--ink-2)" />
        </g>
        <g {...hl('gear')}>
          <rect x="455" y={py} width="12" height={Math.max(8, gearY - py - 5)} fill="var(--surface-2)" />
          <rect x="447" y={gearY - 6} width="28" height="6" rx="2" fill="var(--ink-2)" />
          <path d={`M467 ${py + 14} l14 0 l0 12`} fill="none" stroke-width="3" />
        </g>
        {fx === 'angle' && <path d={`M452 ${py + 30} l8 8 l-6 6 l10 10`} fill="none" stroke="var(--red)" stroke-width="4" />}
        <Wheel x={574} /><Wheel x={618} />
      </g>
      {rig.chocks && <g fill="var(--amber)" stroke={INK} stroke-width="1.5"><path d={`M540 ${GY} l14 -16 l0 16 Z`} /><path d={`M608 ${GY} l-14 -16 l0 16 Z`} /></g>}
      {fx === 'snag' && <g stroke="var(--red)" stroke-width="4"><line x1="420" y1={GY - 2} x2="500" y2={GY - 2} /><line x1="430" y1={GY - 8} x2="430" y2={GY} /><line x1="490" y1={GY - 8} x2="490" y2={GY} /></g>}
      {/* tractor */}
      <g style={{ transform: `translate(${cx}px, 0px) rotate(${rot}deg)`, transformOrigin: `${0}px ${GY}px`, transition: tr }}>
        <rect x="-200" y="216" width="262" height="10" fill="var(--ink-2)" stroke={INK} stroke-width="1.5" />
        <path d="M-200 218 L-200 184 Q-198 174 -186 172 L-150 170 L-150 218 Z" fill="var(--accent)" stroke={INK} stroke-width="2" />
        <rect x="-152" y="104" width="72" height="114" rx="6" fill="var(--accent)" stroke={INK} stroke-width="2" />
        <rect x="-142" y="116" width="40" height="36" rx="4" fill="var(--blue-soft)" stroke={INK} stroke-width="2" />
        <rect x="-76" y="88" width="7" height="130" fill="var(--ink-2)" />
        <rect x="-132" y="222" width="60" height="18" rx="9" fill="var(--surface-2)" stroke={INK} stroke-width="1.5" />
        {!lines && fx !== 'dirt' && <g fill="none" stroke-width="4"><path d="M-80 150 q10 14 0 22" stroke="var(--red)" /><path d="M-80 160 q14 14 0 24" stroke="var(--blue)" /></g>}
        {fx === 'dirt' && <g fill="none" stroke-width="4"><path d="M-80 150 q16 40 6 100" stroke="var(--red)" /><path d="M-80 160 q24 40 12 90" stroke="var(--blue)" /><circle cx="-70" cy="246" r="14" stroke="var(--red)" stroke-dasharray="4 3" /></g>}
        <g style={{ transform: `rotate(${tilt}deg)`, transformOrigin: '0px 214px', transition: tr }}>
          <rect x="-32" y="205" width="64" height="9" rx="2" fill="var(--ink-2)" {...hl('fifth')} />
          {coupled
            ? <rect x="-9" y="204" width="18" height="8" fill="var(--ink)" {...hl('jaws')} />
            : <path d="M-12 204 L-4 212 M12 204 L4 212" stroke={focus === 'jaws' ? 'var(--amber)' : 'var(--surface)'} stroke-width="3" />}
        </g>
        {fx === 'level' && <circle cx="0" cy="210" r="22" fill="none" stroke="var(--red)" stroke-width="4" stroke-dasharray="5 4" />}
        <rect x="-16" y="214" width="32" height="4" fill="var(--ink-2)" />
        <Wheel x={-172} /><Wheel x={-22} /><Wheel x={24} />
      </g>
      {/* air lines + cord (connect cab back to trailer nose) */}
      {lines && <g fill="none" stroke-linecap="round" style={focus === 'air' ? { filter: 'drop-shadow(0 0 3px var(--amber))' } : {}}>
        <path d={`M${cab} 150 C ${cab + 20} 196, ${nose - 30} 196, ${nose - 6} ${py - 78}`} stroke="var(--red)" stroke-width="4" />
        <path d={`M${cab} 160 C ${cab + 24} 206, ${nose - 30} 206, ${nose - 6} ${py - 62}`} stroke="var(--blue)" stroke-width="4" />
      </g>}
      {rig.cord && <path d={`M${cab} 138 C ${cab + 20} 170, ${nose - 30} 160, ${nose - 4} ${py - 100}`} fill="none" stroke="var(--ink-2)" stroke-width="3.5" stroke-dasharray="8 3" />}
      {fx === 'torn' && <g stroke="var(--red)" stroke-width="4" stroke-dasharray="6 6"><line x1={cab + 4} y1="156" x2={cab + 40} y2="186" /></g>}
      {rig.look === 'coupling' && <path d={`M${420} ${250} L${cx - 8} 210 L${cx + 26} 206 Z`} fill="var(--amber)" opacity=".4" />}
      {gap && <g><line x1="360" y1="199" x2="360" y2="207" stroke="var(--red)" stroke-width="3" /></g>}
      {fx && <g>
        {(fx === 'kingpin' || fx === 'low' || fx === 'hurt' || fx === 'nobrakes') && <circle cx={fx === 'low' ? 332 : fx === 'hurt' ? cx + 2 : fx === 'nobrakes' ? 596 : 390} cy={fx === 'nobrakes' ? 238 : fx === 'hurt' ? 238 : 212} r="18" fill="none" stroke="var(--red)" stroke-width="4" stroke-dasharray="5 4" />}
        {fx === 'roll' && <path d="M520 150 l60 0 m-12 -10 l12 10 l-12 10" fill="none" stroke="var(--red)" stroke-width="5" />}
      </g>}
    </svg>
  );
}

/* ---------- fifth-wheel close-up (viewBox 320 wide, text ≥ 15 units so it stays ≥ 11 px at 240 px) ---------- */
function closeLines(rig: Rig, fx: Fx | null): [string, string, boolean] {
  if (fx === 'gap') return ['✕ Gap between the plates', 'Kingpin on top of closed jaws', true];
  if (fx === 'kingpin') return rig.jaws === 'open' ? ['✕ Hits the kingpin hard', 'Lowest reverse, slowly', true] : ['✕ More pressure on the jaws', 'Back up gently instead', true];
  if (fx === 'high') return ['✕ Trailer too high', 'May not couple correctly', true];
  if (fx === 'low') return ['✕ Trailer too low', 'Tractor hits the trailer nose', true];
  if (fx === 'level') return ['✕ Fifth wheel level', 'Must tilt down toward rear', true];
  if (rig.tractor === 'under') return rig.jaws === 'locked' ? ['No gap between plates ✓', 'Jaws locked on shank ✓', false] : ['Release handle at “open”', 'Kingpin free to slide out', false];
  if (rig.tractor === 'partly' || rig.tractor === 'clear') return ['Fifth wheel out from under', 'Jaws open, kingpin out', false];
  if (!rig.tilt) return ['Not inspected yet', 'Should: tilt to rear, jaws open', false];
  return ['Tilted down toward rear ✓', `Jaws open${rig.height === 'ok' ? ' · height OK ✓' : ''}`, false];
}

function CloseUp({ rig, fx, focus }: { rig: Rig; fx: Fx | null; focus: string }) {
  const gap = fx === 'gap', hit = fx === 'kingpin';
  const inJaws = rig.tractor === 'under' || gap || hit;
  const kx = hit ? 158 : inJaws ? 150 : fx === 'high' || fx === 'low' || rig.tractor === 'touch' ? 312 : rig.tractor === 'partly' ? 292 : 372;
  const pb = gap ? 100 : fx === 'high' ? 90 : fx === 'low' ? 148 : inJaws ? 118 : rig.height === 'ok' ? 124 : 110;
  const tilt = fx === 'level' || inJaws || !rig.tilt ? 0 : 8;
  const closed = rig.jaws === 'locked' || gap;
  const [l1, l2, bad] = closeLines(rig, fx);
  const hl = (k: string, w = 2) => (focus === k ? { stroke: 'var(--amber)', 'stroke-width': w + 2.5 } : { stroke: INK, 'stroke-width': w });
  return (
    <svg viewBox="0 0 320 216" width="100%" role="img" aria-label={`Fifth wheel close-up: ${l1}. ${l2}.`} style={{ display: 'block', borderRadius: '8px', background: 'var(--surface-2)' }}>
      <text x="12" y="22" font-size="15" font-weight="700" fill="var(--ink-2)">Fifth wheel close-up</text>
      <rect x="10" y="156" width="300" height="12" fill="var(--ink-2)" stroke={INK} stroke-width="1.5" />
      <rect x="134" y="132" width="32" height="24" fill="var(--surface)" stroke={INK} stroke-width="1.5" />
      <g transform={`rotate(${tilt} 150 126)`}>
        <rect x="50" y="118" width="200" height="16" rx="3" fill="var(--ink-2)" {...hl('fifth')} />
        <rect x="122" y="120" width="56" height="12" fill="var(--bg)" />
        <rect x={closed ? 132 : 124} y="121" width="14" height="10" rx="2" fill="var(--surface)" {...hl('jaws', 1.5)} />
        <rect x={closed ? 154 : 162} y="121" width="14" height="10" rx="2" fill="var(--surface)" {...hl('jaws', 1.5)} />
      </g>
      <g>
        <rect x={kx - 60} y="34" width="400" height={pb - 46} fill="var(--surface)" stroke={INK} stroke-width="2" />
        <rect x={kx - 60} y={pb - 12} width="400" height="12" fill="var(--surface-2)" {...hl('upper')} />
        <path d={`M${kx - 5} ${pb} H${kx + 5} V${pb + 11} L${kx + 9} ${pb + 11} L${kx + 6} ${pb + 17} H${kx - 6} L${kx - 9} ${pb + 11} L${kx - 5} ${pb + 11} Z`} fill="var(--surface)" {...hl('kingpin', 1.5)} />
      </g>
      {gap && <path d="M76 101 v16 m-6 -16 h12 m-12 16 h12" fill="none" stroke="var(--red)" stroke-width="3" />}
      {fx === 'high' && <path d={`M300 ${pb + 3} V${136}`} fill="none" stroke="var(--red)" stroke-width="3" stroke-dasharray="5 4" />}
      {(fx === 'low' || hit || fx === 'level') && <circle cx={fx === 'low' ? 252 : fx === 'level' ? 250 : 150} cy={fx === 'low' ? 140 : 126} r="20" fill="none" stroke="var(--red)" stroke-width="3.5" stroke-dasharray="5 4" />}
      <text x="12" y="191" font-size="15" font-weight="700" fill={bad ? 'var(--red)' : 'var(--ink)'}>{l1}</text>
      <text x="12" y="209" font-size="15" fill={bad ? 'var(--red)' : 'var(--ink)'}>{l2}</text>
    </svg>
  );
}

function Picture({ rig, fx, focus, reducedMotion, caption }: { rig: Rig; fx: Fx | null; focus: string; reducedMotion: boolean; caption?: string }) {
  return (
    <div class="stack" style={{ gap: '8px' }}>
      {caption && <div class="small" role="status" style={{ alignSelf: 'flex-start', padding: '4px 10px', borderRadius: '8px', border: '1.5px solid var(--red)', background: 'var(--red-soft)', color: 'var(--red)', fontWeight: 700 }}>✕ {caption}</div>}
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '12px', alignItems: 'center' }}>
        <div style={{ flex: '3 1 380px', minWidth: 0 }}><Scene rig={rig} fx={fx} focus={focus} reducedMotion={reducedMotion} /></div>
        <div style={{ flex: '1 1 240px', maxWidth: '340px', minWidth: 0 }}><CloseUp rig={rig} fx={fx} focus={focus} /></div>
      </div>
    </div>
  );
}

function Status({ rig }: { rig: Rig }) {
  const items: [string, string, boolean][] = [
    ['Knob', rig.knob === 'in' ? 'IN — air to trailer' : 'OUT — trailer brakes on', rig.knob === 'out'],
    ['Engine', rig.engine, false], ['Key', rig.key ? 'in cab' : 'taken with you', !rig.key], ['Parking brakes', rig.parked ? 'set' : 'off', rig.parked],
    ['Chocks', rig.chocks ? 'in place' : 'off', rig.chocks], ['Landing gear', rig.gear === 'slight' ? 'slightly up' : rig.gear, false],
    ['Air lines', rig.air ? 'connected' : 'stowed', rig.air], ['Cord', rig.cord ? 'plugged in' : 'unplugged', rig.cord],
  ];
  return <div class="row" style={{ gap: '6px' }} aria-label="Rig status">{items.map(([k, v, on]) => <span class="chip" style={{ background: on ? 'var(--accent-soft)' : 'var(--surface-2)', color: 'var(--ink)', fontWeight: 400 }}><strong>{k}:</strong>&nbsp;{v}</span>)}</div>;
}

/** Which part of the picture a step is about (drives the amber highlight). */
const FOCUS: Record<string, string> = { 'C1': 'fifth', 'C6': 'upper', 'C7': 'air', 'C10': 'kingpin', 'C11': 'gear', 'C13': 'jaws', 'C15': 'gear', 'U2': 'jaws', 'U4': 'gear', 'U6': 'jaws', 'U9': 'gear' };

function Explore({ reducedMotion }: { reducedMotion: boolean }) {
  const [which, setWhich] = useState<'c' | 'u'>('c');
  const [k, setK] = useState(0);
  const seq = which === 'c' ? COUPLE : UNCOUPLE;
  const st = seq[k];
  const rig = rigAfter(which === 'c' ? START_COUPLE : START_UNCOUPLE, seq, k + 1);
  const sw = (w: 'c' | 'u', label: string) => <button type="button" class="btn sm" aria-pressed={which === w} onClick={() => { setWhich(w); setK(0); }} style={which === w ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}}>{label}</button>;
  return (
    <div class="stack">
      <div class="row" role="group" aria-label="Sequence">{sw('c', 'Coupling · 16')}{sw('u', 'Uncoupling · 10')}</div>
      <Picture rig={rig} fx={null} focus={FOCUS[(which === 'c' ? 'C' : 'U') + st.n] || ''} reducedMotion={reducedMotion} />
      <Status rig={rig} />
      <div class="card tint stack" style={{ gap: '6px' }} role="status" aria-live="polite">
        <div class="spread"><span class="eyebrow num">{which === 'c' ? 'Coupling' : 'Uncoupling'} step {st.n} of {seq.length}</span><span class="plate">p. {st.page}</span></div>
        <strong style={{ font: '700 1.3rem/1.15 var(--display)' }}>{st.label}</strong>
        <p class="small">{st.detail}</p>
        <p class="small"><strong>Why:</strong> {st.why}</p>
      </div>
      <div class="row">
        <button type="button" class="btn sm" disabled={k === 0} onClick={() => setK(k - 1)}>← Back</button>
        <button type="button" class="btn primary sm" disabled={k === seq.length - 1} onClick={() => setK(k + 1)}>Next step →</button>
      </div>
      <ol class="small" style={{ margin: 0, paddingLeft: '1.6em', display: 'grid', gap: '2px' }}>
        {seq.map((s, i) => <li><button type="button" class="linkbtn" aria-current={i === k ? 'step' : undefined} onClick={() => setK(i)} style={{ textDecoration: i === k ? 'none' : 'underline', color: i === k ? 'var(--ink)' : 'var(--accent)', background: i === k ? 'var(--amber-soft)' : 'none', padding: '2px 4px', borderRadius: '4px', textAlign: 'left', minHeight: '28px' }}>{s.label}</button></li>)}
      </ol>
    </div>
  );
}

function Challenge({ seq, start, onEvidence, onChallenge, concepts, reducedMotion, doneName }: { seq: Step[]; start: typeof START_COUPLE; doneName: string; reducedMotion: boolean } & Pick<WidgetProps, 'onEvidence' | 'onChallenge' | 'concepts'>) {
  const [k, setK] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const st = seq[k];
  if (!st) return (
    <div class="stack">
      <Picture rig={rigAfter(start, seq, seq.length)} fx={null} focus="" reducedMotion={reducedMotion} />
      <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status">
        <div class="verdict">{misses === 0 ? `${doneName} — all ${seq.length} steps in order` : `${seq.length - misses} of ${seq.length} picked right`}</div>
        <p class="small">{misses === 0 ? 'Every action in the handbook’s order.' : 'Step through Explore once more, then run it again.'} <span class="plate">p. {seq === COUPLE ? '6-9 to 6-10' : '6-11'}</span></p>
        <div><button type="button" class="btn sm" onClick={() => { setK(0); setMisses(0); setPick(null); }}>Run it again</button></div>
      </div>
    </div>
  );
  const opts = [{ label: st.label, ok: true, kind: 'order' as Kind, fx: 'order' as Fx, result: '' }, ...st.wrong.map((w) => ({ ...w, ok: false }))];
  const rotBy = (st.n * 2) % 3;
  const order = [0, 1, 2].map((i) => opts[(i + rotBy) % 3]);
  const chosen = pick === null ? null : order[pick];
  const rig = rigAfter(start, seq, chosen?.ok ? k + 1 : k);
  return (
    <div class="stack">
      <div class="spread"><span class="small muted num">Step {k + 1} of {seq.length}</span><span class="small muted num">{misses === 0 ? 'No wrong picks yet' : `${misses} wrong pick${misses > 1 ? 's' : ''}`}</span></div>
      <Picture rig={rig} fx={chosen && !chosen.ok ? chosen.fx : null} focus="" reducedMotion={reducedMotion} caption={chosen && !chosen.ok ? verdictOf(chosen) : undefined} />
      <Status rig={rig} />
      <strong>{k === 0 ? 'What do you do first?' : `Done: ${seq[k - 1].label}. What next?`}</strong>
      <div class="opts" role="group" aria-label="Pick the next action">{order.map((o, i) => (
        <button type="button" class={`opt ${pick === null ? '' : o.ok ? 'right' : i === pick ? 'wrong' : ''}`} disabled={pick !== null}
          onClick={() => { setPick(i); if (!o.ok) setMisses(misses + 1); onEvidence({ concepts, ok: o.ok }); }}>
          <span class="letter">{'abc'[i]}</span><span>{o.label}</span></button>))}</div>
      {chosen && <div class={`feedback ${chosen.ok ? 'good' : 'bad'}`} role="status">
        <div class="verdict">{chosen.ok ? `Right — step ${st.n}` : verdictOf(chosen)}</div>
        {!chosen.ok && <p class="small">{chosen.result}</p>}
        <p class="small"><strong>Step {st.n}: {st.label}.</strong> {st.detail} <em>Why:</em> {st.why} <span class="plate">p. {st.page}</span></p>
        <div><button type="button" class="btn primary sm" onClick={() => { if (k + 1 === seq.length && misses === 0) onChallenge?.(); setPick(null); setK(k + 1); }}>{k + 1 === seq.length ? 'Finish' : 'Next step'}</button></div>
      </div>}
    </div>
  );
}

export default function Coupling({ onEvidence, onChallenge, concepts, reducedMotion }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'couple' | 'uncouple'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Walk through</button>
        <button role="tab" aria-selected={mode === 'couple'} onClick={() => setMode('couple')}>Couple (16)</button>
        <button role="tab" aria-selected={mode === 'uncouple'} onClick={() => setMode('uncouple')}>Uncouple (10)</button>
      </div>
      {mode === 'explore' && <Explore reducedMotion={reducedMotion} />}
      {mode === 'couple' && <Challenge key="c" seq={COUPLE} start={START_COUPLE} doneName="Coupled clean" onEvidence={onEvidence} onChallenge={onChallenge} concepts={concepts} reducedMotion={reducedMotion} />}
      {mode === 'uncouple' && <Challenge key="u" seq={UNCOUPLE} start={START_UNCOUPLE} doneName="Uncoupled clean" onEvidence={onEvidence} concepts={concepts} reducedMotion={reducedMotion} />}
    </div>
  );
}
