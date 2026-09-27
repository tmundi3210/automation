import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk09-hazard-clues', title: 'Hazard clue spotter', lesson: 'GK-09', anchor: /drivers who are hazards/i,
  summary: 'Tap things on the street to learn each hazard clue and your plan. Then find 5 clues and choose the plan for 3.',
  stamp: { id: 'hazard-spotter', name: 'Hazard spotter', rule: 'Find 5 hazard clues with no false alarms, then pick the right plan 3 times.' },
};

interface Obj { id: string; box: [number, number, number, number]; mark?: [number, number]; name: string; hazard: boolean; clue: string; expect: string; plan: string; page: string }
/** Every fact below is from GK-09 (DL 650 pp. 2-21 to 2-27). Boxes are tap targets in viewBox units (320 × 300); each is at least 40 × 40, so ≥ 36 px on a 390 px phone. `mark` is where the number sits, in empty street space. */
export const OBJS: Obj[] = [
  { id: 'alley', box: [230, 38, 82, 56], mark: [298, 60], name: 'Car nosing out of a covered alley', hazard: true, clue: 'You can see part of the car, but not the driver.', expect: 'If you can’t see the driver, assume the driver can’t see you. The car may pull out into your path.', plan: 'Be ready to stop. Slow down now so you have time to act calmly.', page: '2-22' },
  { id: 'parked', box: [76, 202, 70, 40], mark: [86, 211], name: 'Parked car with exhaust and brake lights', hazard: true, clue: 'Someone moving inside, brake lights on, exhaust coming out.', expect: 'A door may open, a person may step out, or the car may pull out.', plan: 'Check your mirrors, slow down and give it room, so a door or a pull-out does not force a sudden swerve.', page: '2-22' },
  { id: 'delivery', box: [160, 202, 72, 42], mark: [169, 210], name: 'Delivery van with its door open', hazard: true, clue: 'Packages and an open door block the delivery driver’s view.', expect: 'A rushed driver may jump out, or pull out into your lane.', plan: 'Slow down and be ready to stop or change lanes.', page: '2-22' },
  { id: 'rental', box: [150, 158, 98, 40], mark: [237, 167], name: 'Rental truck ahead', hazard: true, clue: 'Blocked vision: rental truck drivers are not used to the limited view to the sides and rear.', expect: 'They may move into you without seeing you.', plan: 'Picture them changing lanes into you. Keep space around them and be ready to slow down.', page: '2-22' },
  { id: 'kids', box: [142, 50, 80, 58], mark: [212, 62], name: 'Children and an ice cream truck', hazard: true, clue: 'Children playing near an ice cream truck. Their attention is elsewhere.', expect: 'Children move quickly without checking traffic, especially when playing.', plan: 'Slow down and be ready to stop.', page: '2-23' },
  { id: 'confused', box: [188, 116, 80, 40], mark: [258, 128], name: 'Car with car-top luggage, backup lights on', hazard: true, clue: 'Tourist clue (car-top luggage) plus an odd action: backup lights suddenly on mid-block.', expect: 'A confused driver may turn or stop without warning.', plan: 'Keep extra space and be ready to slow or stop.', page: '2-23' },
  { id: 'drift', box: [86, 118, 64, 44], mark: [98, 130], name: 'Car drifting over the lane line', hazard: true, clue: 'Drifting over lane lines and within the lane — a distracted-driver sign.', expect: 'The driver may not know you are there and could drift in front of you.', plan: 'Give plenty of room, keep a safe following distance, and pass very carefully.', page: '2-27' },
  { id: 'work', box: [254, 156, 64, 86], mark: [306, 232], name: 'Work zone ahead', hazard: true, clue: 'Cones, a narrowed lane and a worker in the road.', expect: 'Narrow lanes, rough surface, distracted drivers, workers and equipment in your path.', plan: 'Drive slowly; warn drivers behind with 4-way flashers or brake lights. [CA] Move-over law: slow down and move to a lane not next to it if safe.', page: '2-22' },
  { id: 'empty', box: [4, 76, 62, 40], name: 'Parked car, dark and empty', hazard: false, clue: 'No one inside, no brake or backup lights, no exhaust.', expect: 'None of the handbook’s parked-car clues are here.', plan: 'Keep scanning ahead.', page: '2-22' },
  { id: 'tree', box: [126, 244, 46, 44], name: 'Street tree on the sidewalk', hazard: false, clue: 'A tree beside the sidewalk.', expect: 'It will not move into your path.', plan: 'Keep scanning ahead.', page: '2-21' },
];
const HAZ = OBJS.filter((o) => o.hazard);
const NEED = 5;
const VW = 320, VH = 300;

interface PlanQ { id: string; q: string; opts: { t: string; ok: boolean; why: string }[]; from: [number, number]; to: [number, number] }
export const PLANS: PlanQ[] = [
  { id: 'alley', from: [256, 92], to: [256, 176], q: 'You see the front of a car sticking out of the alley. You can’t see its driver. What is your plan?', opts: [
    { t: 'Keep your speed — a truck this big is easy to see.', ok: false, why: 'If you can’t see the driver, the driver can’t see you. The car pulls out and the hazard becomes an emergency: you must brake hard or swerve, and sudden moves are more likely to cause a crash.' },
    { t: 'Assume the driver can’t see you. Slow down and be ready to stop.', ok: true, why: 'That is the handbook plan for a partly hidden vehicle. Seeing it early gives you time to slow down calmly.' },
    { t: 'Speed up to get past the alley before it moves.', ok: false, why: 'More speed leaves less time to react. If it pulls out now it cuts right in front of you — an emergency.' }] },
  { id: 'parked', from: [148, 224], to: [124, 180], q: 'A parked car just ahead has exhaust coming out, its brake lights on, and someone moving inside. What is your plan?', opts: [
    { t: 'It is parked, so keep your line close beside it.', ok: false, why: 'Movement, brake lights and exhaust are the clues that a door will open or the car will pull out. Close beside it, a door or pull-out leaves you only a sudden swerve.' },
    { t: 'Expect a door to open or the car to pull out: check mirrors, slow down, give it room.', ok: true, why: 'Right. You pictured the emergency and decided what to do before it happened — defensive driving.' },
    { t: 'Nothing to plan — clues only matter at intersections.', ok: false, why: 'Parked vehicles are one of the handbook’s listed hazards. The car pulls out and you must brake hard.' }] },
  { id: 'delivery', from: [226, 224], to: [240, 180], q: 'A delivery van is stopped at the curb with its door open and packages stacked inside. What is your plan?', opts: [
    { t: 'Pass close at speed — delivery drivers always watch for traffic.', ok: false, why: 'Packages and doors block the delivery driver’s view, and a rushed driver may jump out or pull into your lane — right in front of you.' },
    { t: 'Assume the van will wait because it is parked.', ok: false, why: 'Rushed delivery drivers may pull out into your lane without seeing you.' },
    { t: 'Slow down and be ready to stop or change lanes if the driver jumps out or pulls in.', ok: true, why: 'Right. Early planning lets you check mirrors, signal and change lanes or slow down calmly.' }] },
];

function Car({ x, y, w, h, fill = 'var(--surface)', face = 'right' }: { x: number; y: number; w: number; h: number; fill?: string; face?: 'right' | 'left' | 'down' }) {
  const ws = face === 'right' ? <rect x={x + w * 0.6} y={y + 2} width={w * 0.14} height={h - 4} fill="var(--blue-soft)" stroke="var(--ink)" stroke-width="0.6" />
    : face === 'left' ? <rect x={x + w * 0.26} y={y + 2} width={w * 0.14} height={h - 4} fill="var(--blue-soft)" stroke="var(--ink)" stroke-width="0.6" />
    : <rect x={x + 2} y={y + h * 0.6} width={w - 4} height={h * 0.14} fill="var(--blue-soft)" stroke="var(--ink)" stroke-width="0.6" />;
  return <g><rect x={x} y={y} width={w} height={h} rx="3" fill={fill} stroke="var(--ink)" stroke-width="1.2" />{ws}</g>;
}

export function Scene({ found, sel, plan, planOk, marks }: { found: Set<string>; sel?: string | null; plan?: PlanQ | null; planOk?: boolean | null; marks?: boolean }) {
  const ring = (id: string) => { const o = OBJS.find((x) => x.id === id)!; const [x, y, w, h] = o.box; return <rect key={`r${id}`} x={x + 1} y={y + 1} width={w - 2} height={h - 2} rx="6" fill="none" stroke={o.hazard ? 'var(--amber)' : 'var(--ink-2)'} stroke-width="2.5" stroke-dasharray={sel === id ? '0' : '5 3'} />; };
  const lbl = { 'font-size': 13, 'font-weight': 700, 'text-anchor': 'middle' } as const;
  return (
    <svg viewBox={`0 0 ${VW} ${VH}`} width="100%" style={{ display: 'block' }} role="img" aria-label="Top-down city street. Your truck drives east (left to right) in the lower lane. Around it: parked cars, a delivery van, a rental truck, an ice cream truck with children, a car in a covered alley, a car drifting over the center line, a car with luggage on top, a street tree, and a work zone ahead.">
      <rect x="0" y="0" width={VW} height={VH} fill="var(--line)" />
      <rect x="0" y="56" width={VW} height="18" fill="var(--surface-2)" /><rect x="0" y="242" width={VW} height="18" fill="var(--surface-2)" />
      <rect x="0" y="74" width={VW} height="168" fill="var(--ink-2)" fill-opacity="0.28" />
      <rect x="232" y="0" width="48" height="74" fill="var(--ink-2)" fill-opacity="0.28" />
      <line x1="0" y1="156" x2={VW} y2="156" stroke="var(--amber)" stroke-width="2" stroke-dasharray="10 6" />
      <text x="8" y="22" font-size="13" fill="var(--ink-2)">Shops</text><text x="312" y="290" font-size="13" text-anchor="end" fill="var(--ink-2)">Houses</text>
      <g aria-hidden="true">
        {/* empty parked car (decoy) */}
        <Car x={12} y={84} w={46} h={22} face="left" />
        {/* confused driver: luggage + backup lights (rear = right side, facing west) */}
        <Car x={196} y={124} w={46} h={22} face="left" />
        <rect x="211" y="127" width="9" height="16" fill="var(--amber)" stroke="var(--ink)" stroke-width="0.6" /><rect x="222" y="127" width="9" height="16" fill="var(--amber)" stroke="var(--ink)" stroke-width="0.6" />
        <rect x="242" y="125" width="4" height="5" fill="#fff" stroke="var(--ink)" stroke-width="0.5" /><rect x="242" y="140" width="4" height="5" fill="#fff" stroke="var(--ink)" stroke-width="0.5" />
        {/* distracted driver drifting over the center line */}
        <path d="M 146 146 q 8 -8 16 0 t 16 0" fill="none" stroke="var(--ink-2)" stroke-width="1.4" stroke-dasharray="3 3" />
        <g transform="rotate(-12 124 152)"><Car x={102} y={142} w={44} h={20} face="left" /></g>
        {/* ice cream truck + children */}
        <Car x={150} y={80} w={54} h={22} face="left" fill="var(--blue-soft)" />
        <path d="M 174 88 L 182 88 L 178 99 Z" fill="var(--amber)" stroke="var(--ink)" stroke-width="0.8" /><circle cx="178" cy="86" r="4.5" fill="var(--surface)" stroke="var(--ink)" />
        {[[158, 66, 'var(--amber)'], [174, 64, 'var(--amber)'], [212, 96, 'var(--red)']].map(([x, y, c]) => <g><rect x={+x - 3.5} y={+y - 2} width="7" height="10" rx="2.5" fill={c as string} stroke="var(--ink)" stroke-width="0.8" /><circle cx={x} cy={+y - 6} r="3.8" fill={c as string} stroke="var(--ink)" stroke-width="0.8" /></g>)}
        {/* covered alley with car nose */}
        <Car x={244} y={34} w={24} h={56} face="down" />
        <rect x="232" y="6" width="48" height="52" fill="var(--line)" stroke="var(--ink-2)" stroke-width="1" /><path d="M 232 6 L 280 58 M 280 6 L 232 58" stroke="var(--ink-2)" stroke-width="0.8" />
        {/* your truck */}
        <rect x="4" y="165" width="58" height="26" rx="2" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.2" />
        <rect x="64" y="167" width="18" height="22" rx="3" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.2" />
        <text x="33" y="183" {...lbl} fill="var(--accent-ink)">YOU</text>
        {/* parked car with exhaust + brake lights + person */}
        <Car x={104} y={214} w={42} h={22} />
        <circle cx="118" cy="225" r="3.5" fill="var(--ink-2)" />
        <rect x="102" y="215" width="3" height="5" fill="var(--red)" /><rect x="102" y="230" width="3" height="5" fill="var(--red)" />
        <circle cx="96" cy="229" r="3.5" fill="var(--ink-2)" fill-opacity="0.5" /><circle cx="89" cy="226" r="4" fill="var(--ink-2)" fill-opacity="0.4" />
        {/* rental truck: box, no rear windows */}
        <rect x="156" y="164" width="52" height="28" rx="2" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.2" />
        <Car x={210} y={166} w={16} h={24} />
        <text x="182" y="183" {...lbl} fill="var(--ink)">RENT</text>
        {/* delivery van, door open, packages */}
        <Car x={181} y={216} w={46} h={22} />
        <rect x="191" y="238" width="14" height="7" fill="var(--surface)" stroke="var(--ink)" stroke-width="1" />
        <rect x="185" y="220" width="7" height="7" fill="var(--amber-soft)" stroke="var(--ink)" stroke-width="0.6" /><rect x="193" y="220" width="7" height="7" fill="var(--amber-soft)" stroke="var(--ink)" stroke-width="0.6" />
        {/* tree (decoy) */}
        <circle cx="149" cy="262" r="12" fill="var(--ok)" fill-opacity="0.6" stroke="var(--ink)" stroke-width="0.8" />
        {/* work zone: cones closing the lane, a worker, equipment */}
        {[[262, 198], [272, 188], [282, 178], [292, 168], [302, 160]].map(([x, y]) => <path d={`M ${x} ${y - 8} L ${x + 6} ${y + 4} L ${x - 6} ${y + 4} Z`} fill="var(--amber)" stroke="var(--ink)" stroke-width="0.8" />)}
        <circle cx="300" cy="192" r="6" fill="var(--amber)" stroke="var(--ink)" /><rect x="272" y="208" width="30" height="12" fill="var(--ink-2)" />
      </g>
      {[...found].map(ring)}
      {sel && !found.has(sel) && ring(sel)}
      {marks && OBJS.map((o, i) => o.hazard && o.mark && <g key={`m${o.id}`} aria-hidden="true">
        <circle cx={o.mark[0]} cy={o.mark[1]} r="9.5" fill={found.has(o.id) ? 'var(--ok)' : 'var(--amber)'} stroke="var(--ink)" stroke-width="1.5" />
        <text x={o.mark[0]} y={o.mark[1] + 4.5} {...lbl} fill={found.has(o.id) ? 'var(--surface)' : 'var(--amber-ink)'}>{found.has(o.id) ? '✓' : i + 1}</text></g>)}
      {plan && planOk === false && <g><path d={`M ${plan.from[0]} ${plan.from[1]} L ${plan.to[0]} ${plan.to[1]}`} stroke="var(--red)" stroke-width="3" stroke-dasharray="6 4" marker-end="url(#g9arrow)" />
        <circle cx={plan.to[0]} cy={plan.to[1]} r="11" fill="var(--red)" /><text x={plan.to[0]} y={plan.to[1] + 5} text-anchor="middle" font-size="15" font-weight="700" fill="var(--surface)">!</text>
        <rect x="40" y="266" width="240" height="26" rx="4" fill="var(--red)" /><text x="160" y="284" {...lbl} fill="var(--surface)">EMERGENCY — no time left</text></g>}
      {plan && planOk === true && <g><path d={`M ${plan.from[0]} ${plan.from[1]} L ${plan.to[0]} ${plan.to[1]}`} stroke="var(--amber)" stroke-width="2" stroke-dasharray="4 4" />
        <rect x="86" y="163" width="6" height="30" fill="var(--ok)" /><rect x="40" y="266" width="240" height="26" rx="4" fill="var(--ok)" /><text x="160" y="284" {...lbl} fill="var(--surface)">Slowed early — room to act</text></g>}
      <defs><marker id="g9arrow" viewBox="0 0 10 10" refX="5" refY="5" markerWidth="5" markerHeight="5" orient="auto"><path d="M0 0 L10 5 L0 10 Z" fill="var(--red)" /></marker></defs>
    </svg>
  );
}

function Hotspots({ onPick, found, sel }: { onPick: (id: string) => void; found: Set<string>; sel: string | null }) {
  return <>{OBJS.map((o) => { const [x, y, w, h] = o.box; return (
    <button aria-label={`${o.name}${found.has(o.id) ? ' (found)' : ''}`} aria-pressed={sel === o.id} onClick={() => onPick(o.id)}
      style={{ position: 'absolute', left: `${(x / VW) * 100}%`, top: `${(y / VH) * 100}%`, width: `${(w / VW) * 100}%`, height: `${(h / VH) * 100}%`, background: 'transparent', border: 0, padding: 0, borderRadius: '6px', cursor: 'pointer' }} />
  ); })}</>;
}

function Detail({ o }: { o: Obj }) {
  return (
    <div class={o.hazard ? 'card warn' : 'card'} role="status" aria-live="polite">
      <div class="row" style={{ gap: '6px' }}>{o.hazard && <span class="diamond" aria-hidden="true" />}<strong>{o.name}</strong> <span class="plate">p. {o.page}</span></div>
      <p class="small"><strong>Clue:</strong> {o.clue}</p>
      <p class="small"><strong>{o.hazard ? 'What to expect:' : 'Why it’s not a clue:'}</strong> {o.expect}</p>
      <p class="small"><strong>Your plan:</strong> {o.plan.startsWith('Drive slowly') ? <>Drive slowly; warn drivers behind with 4-way flashers or brake lights. <span class="ca-tag">CA</span> Move-over law: slow down and move to a lane not next to it, if safe.</> : o.plan}</p>
    </div>
  );
}

export default function HazardClues({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  const [seen, setSeen] = useState<Set<string>>(new Set());
  const [sel, setSel] = useState<string | null>(null);
  // challenge state
  const [found, setFound] = useState<Set<string>>(new Set());
  const [last, setLast] = useState<{ id: string; ok: boolean } | null>(null);
  const [misses, setMisses] = useState(0);
  const [qi, setQi] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const reset = () => { setFound(new Set()); setLast(null); setMisses(0); setQi(0); setPick(null); };
  const tapChallenge = (id: string) => {
    if (found.has(id) || found.size >= NEED) return;
    const o = OBJS.find((x) => x.id === id)!;
    onEvidence({ concepts, ok: o.hazard });
    if (o.hazard) setFound(new Set([...found, id])); else setMisses(misses + 1);
    setLast({ id, ok: o.hazard });
  };
  const phase = found.size < NEED ? 'find' : qi < PLANS.length ? 'plan' : 'done';
  const pq = phase === 'plan' ? PLANS[qi] : null;
  const answer = (i: number) => { if (pick !== null || !pq) return; setPick(i); const ok = pq.opts[i].ok; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const next = () => { const n = qi + 1; if (n === PLANS.length && misses === 0) onChallenge?.(); setQi(n); setPick(null); };
  const lastObj = last ? OBJS.find((o) => o.id === last.id)! : null;
  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore the street</button>
        <button role="tab" aria-selected={mode === 'challenge'} onClick={() => { setMode('challenge'); reset(); }}>Challenge</button>
      </div>
      {mode === 'explore' && <>
        <p class="small">A <strong>hazard</strong> is any road condition or road user that <em>could</em> become a danger. Tap each numbered spot (or anything else) to read the clue and the plan. <span class="plate">p. 2-21</span></p>
        <div style={{ position: 'relative', maxWidth: '440px', width: '100%', marginInline: 'auto' }}>
          <Scene found={seen} sel={sel} marks />
          <Hotspots found={seen} sel={sel} onPick={(id) => { setSel(id); if (OBJS.find((o) => o.id === id)!.hazard) setSeen(new Set([...seen, id])); }} />
        </div>
        <span class="small muted num">{seen.size} of {HAZ.length} clues read</span>
        {sel ? <Detail o={OBJS.find((o) => o.id === sel)!} /> : <div class="card tint small">Always have a plan: when you spot a hazard, picture the emergency it could cause, decide what you would do, and be ready to do it. <span class="plate">p. 2-24</span></div>}
      </>}
      {mode === 'challenge' && <>
        {phase === 'find' && <p class="small"><strong>Find {NEED} hazard clues.</strong> No markers this time — tap the things on the street that could become a danger. Not everything is a hazard. <span class="num">Found {found.size} of {NEED}.</span></p>}
        {phase === 'plan' && pq && <p class="small num"><strong>What is your plan?</strong> Question {qi + 1} of {PLANS.length}</p>}
        <div style={{ position: 'relative', maxWidth: '440px', width: '100%', marginInline: 'auto' }}>
          <Scene found={found} sel={phase === 'find' ? last?.id ?? null : pq?.id ?? null} plan={pq && pick !== null ? pq : null} planOk={pq && pick !== null ? pq.opts[pick].ok : null} />
          {phase === 'find' && <Hotspots found={found} sel={last?.id ?? null} onPick={tapChallenge} />}
        </div>
        {phase === 'find' && lastObj && last && <div class={`feedback ${last.ok ? 'good' : 'bad'}`} role="status"><div class="verdict">{last.ok ? `Clue found: ${lastObj.name}` : 'False alarm'}</div><p class="small">{last.ok ? `${lastObj.clue} ${lastObj.expect}` : `${lastObj.name}: ${lastObj.clue} ${lastObj.expect}`} <span class="plate">p. {lastObj.page}</span></p></div>}
        {pq && <div class="stack">
          <strong>{pq.q}</strong>
          <div class="stack" role="group" aria-label="Pick a plan">{pq.opts.map((o, i) => (
            <button class={`btn sm ${pick !== null && o.ok ? 'primary' : ''}`} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick !== null && o.ok ? { opacity: 1 } : {}), ...(pick === i && !o.ok ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}) }} disabled={pick !== null} onClick={() => answer(i)}>{pick !== null && (o.ok ? '✓ ' : pick === i ? '✗ ' : '')}{o.t}</button>
          ))}</div>
          {pick !== null && <div class={`feedback ${pq.opts[pick].ok ? 'good' : 'bad'}`} role="status"><div class="verdict">{pq.opts[pick].ok ? 'Good plan' : 'That plan fails'}</div><p class="small">{pq.opts[pick].why} <span class="plate">p. 2-22</span> <span class="plate">p. 2-24</span></p>
            <button class="btn primary sm" onClick={next}>{qi + 1 === PLANS.length ? 'Finish' : 'Next plan'}</button></div>}
        </div>}
        {phase === 'done' && <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'Clean run — stamp earned' : `Done with ${misses} mistake${misses > 1 ? 's' : ''}`}</div>
          <p class="small">{misses === 0 ? 'You spotted the clues and planned before they became emergencies.' : 'Try again with no false alarms and no wrong plans to earn the stamp.'}</p>
          <button class="btn sm" onClick={reset}>Try again</button></div>}
      </>}
    </div>
  );
}
