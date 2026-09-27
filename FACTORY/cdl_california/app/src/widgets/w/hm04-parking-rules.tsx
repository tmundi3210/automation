import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';
import { ATT, DEV, FEAT_NAME, LOAD, SMOKE, SPOTS, broken, check, spot, type Att, type Dev, type Feat, type Key, type Load, type Smoke, type State, type Stop } from './hm04-parking-rules.rules';

export const meta: WidgetMeta = {
  id: 'hm04-parking-rules', title: 'Park the placarded truck', lesson: 'HM-04', anchor: /parking with division 1\.1/i,
  summary: 'Place a placarded truck on the map, pick who watches it, and check the 5 ft, 300 ft, 100 ft, 25 ft and no-flares rules. Then judge 10 parked trucks.',
  stamp: { id: 'parked-by-book', name: 'Parked by the book', rule: 'Judge all 10 parked HazMat trucks — legal, or which rule is broken — with no mistakes.' },
};

const pressed = (on: boolean) => (on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {});
const W = 360, H = 250;
const FEAT_XY: Record<Feat, [number, number, number, number]> = { bridge: [30, 26, 38, 48], tunnel: [318, 24, 42, 50], building: [98, 150, 58, 36], crowd: [160, 150, 34, 36], fire: [300, 98, 26, 26] };

function Truck({ x, y, load }: { x: number; y: number; load: Load }) {
  return (<g>
    <rect x={x - 26} y={y - 7} width="38" height="14" rx="2" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.8" />
    <rect x={x + 14} y={y - 6} width="12" height="12" rx="2" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.8" />
    <rect x={x - 11} y={y - 6} width="10" height="10" transform={`rotate(45 ${x - 6} ${y - 1})`} fill="var(--amber)" stroke="var(--ink)" stroke-width="1" />
    <text x={x - 6} y={y + 3} text-anchor="middle" font-size="7" font-weight="700" fill="#14201a">{load === 'exp' ? '1' : '3'}</text>
  </g>);
}

function MapSvg({ s, hot }: { s: State; hot: Feat[] }) {
  const sp = spot(s.spot);
  const t = { 'font-size': 12, fill: 'var(--ink)' } as const;
  const hl = (f: Feat) => { const [x, y, w, h] = FEAT_XY[f]; return hot.includes(f) ? <g><rect x={x - 4} y={y - 4} width={w + 8} height={h + 8} rx="4" fill="none" stroke="var(--red)" stroke-width="2.5" stroke-dasharray="5 3" /><text x={x + w / 2} y={y - 7} text-anchor="middle" font-size="12" font-weight="700" fill="var(--red)" stroke="var(--surface)" stroke-width="3" paint-order="stroke">{sp.d[f]} ft</text></g> : null; };
  return (
    <svg viewBox={`0 0 ${W} ${H}`} width="100%" role="img" style={{ display: 'block' }}
      aria-label={`Map, not to scale. Highway across the top with a bridge over a river on the left and a tunnel on the right. Below: a diner with people outside, a fuel island, a brush fire, a roadside turnout, a shipper's yard and a safe haven. Your ${LOAD[s.load]} truck is at spot ${sp.n}, ${sp.name}.${hot.length ? ` Within 300 ft: ${hot.map((f) => FEAT_NAME[f]).join(', ')}.` : ''}`}>
      <rect width={W} height={H} fill="var(--accent-soft)" />
      <rect x="34" y="0" width="30" height={H} fill="var(--blue-soft)" /><text x="49" y="244" text-anchor="middle" {...t}>river</text>
      <rect y="30" width={W} height="40" fill="var(--surface)" /><rect y="70" width={W} height="12" fill="var(--surface-2)" />
      <line x1="0" x2={W} y1="30" y2="30" stroke="var(--ink-2)" stroke-width="1.5" /><line x1="0" x2={W} y1="70" y2="70" stroke="var(--ink-2)" stroke-width="1.5" />
      <line x1="0" x2={W} y1="50" y2="50" stroke="var(--amber)" stroke-width="2" stroke-dasharray="10 7" />
      <text x="120" y="45" {...t}>traveled part of the road</text><text x="236" y="80" font-size="11" fill="var(--ink-2)">shoulder</text>
      <rect x="30" y="26" width="38" height="4" fill="var(--ink)" /><rect x="30" y="70" width="38" height="4" fill="var(--ink)" /><text x="72" y="22" {...t}>bridge</text>
      <path d="M318 0 H360 V100 H318 Z" fill="var(--ok)" opacity=".45" /><path d="M318 70 V38 a10 10 0 0 1 10 -10 H360 V70 Z" fill="var(--ink-2)" /><text x="314" y="96" text-anchor="end" {...t}>tunnel</text>
      <rect x="98" y="150" width="58" height="36" fill="var(--surface-2)" stroke="var(--ink)" stroke-width="1.5" /><text x="127" y="172" text-anchor="middle" {...t}>diner</text>
      {[166, 176, 186].map((x) => <g><circle cx={x} cy="158" r="3.5" fill="var(--ink)" /><rect x={x - 3} y="163" width="6" height="12" rx="2" fill="var(--ink)" /></g>)}<text x="200" y="198" text-anchor="middle" font-size="11" fill="var(--ink)">people</text>
      <rect x="228" y="118" width="50" height="8" fill="var(--ink-2)" /><rect x="244" y="142" width="8" height="10" fill="var(--blue)" /><text x="253" y="166" text-anchor="middle" {...t}>fuel</text>
      <rect x="284" y="140" width="36" height="24" fill="var(--surface-2)" stroke="var(--ink)" stroke-width="1.2" /><text x="302" y="178" text-anchor="middle" font-size="11" fill="var(--ink)">store</text>
      <path d="M313 122 q-10 -8 -4 -20 q2 8 6 4 q-2 -8 6 -12 q-2 10 6 12 q4 10 -6 16 Z" fill="var(--red)" stroke="var(--ink)" stroke-width="1" /><text x="330" y="136" text-anchor="middle" font-size="11" fill="var(--ink)">fire</text>
      <rect x="20" y="200" width="56" height="40" fill="var(--surface-2)" stroke="var(--ink)" stroke-width="1.5" /><text x="48" y="224" text-anchor="middle" font-size="11" fill="var(--ink)">shipper</text>
      <rect x="258" y="202" width="96" height="42" rx="4" fill="none" stroke="var(--accent)" stroke-width="2" stroke-dasharray="6 4" /><text x="306" y="246" text-anchor="middle" font-size="11" font-weight="700" fill="var(--accent)">SAFE HAVEN</text>
      {(Object.keys(FEAT_XY) as Feat[]).map(hl)}
      <Truck x={sp.x} y={sp.y} load={s.load} />
      {s.dev === 'flare' && <g>{[-50, -40].map((dx) => <circle cx={sp.x + dx} cy={sp.y + 4} r="3" fill="var(--red)" stroke="var(--amber)" stroke-width="2" />)}</g>}
      {(s.dev === 'tri' || s.dev === 'red') && <g>{[-50, -40].map((dx) => s.dev === 'tri' ? <path d={`M${sp.x + dx} ${sp.y - 2} l4 7 h-8 z`} fill="var(--red)" stroke="var(--ink)" stroke-width=".8" /> : <rect x={sp.x + dx - 3} y={sp.y} width="6" height="6" fill="var(--red)" stroke="var(--ink)" stroke-width=".8" />)}</g>}
      {s.smoke !== 'none' && <g><circle cx={sp.x + (s.smoke === 's10' ? 34 : 48)} cy={sp.y - 14} r="3.5" fill="var(--ink)" /><rect x={sp.x + (s.smoke === 's10' ? 31 : 45)} y={sp.y - 10} width="6" height="10" rx="2" fill="var(--ink)" /><circle cx={sp.x + (s.smoke === 's10' ? 39 : 53)} cy={sp.y - 8} r="2" fill="var(--amber)" /></g>}
    </svg>
  );
}

function Map({ s, onSpot, locked }: { s: State; onSpot?: (id: string) => void; locked?: boolean }) {
  const sp = spot(s.spot);
  const hot = (Object.keys(sp.d) as Feat[]).filter((f) => sp.d[f] < 300 && (s.load === 'exp' || f === 'fire'));
  return (
    <div style={{ position: 'relative', maxWidth: '560px', width: '100%', margin: '0 auto' }}>
      <MapSvg s={s} hot={hot} />
      {SPOTS.map((p) => {
        const on = p.id === s.spot;
        return <button aria-label={`Spot ${p.n}: ${p.name}, ${p.road} ft from the traveled road${on ? ', truck is here' : ''}`} aria-pressed={on} disabled={locked}
          onClick={() => onSpot?.(p.id)}
          style={{ position: 'absolute', left: `${((p.x + (on ? 0 : -6)) / W) * 100}%`, top: `${(p.y / H) * 100}%`, transform: 'translate(-50%,-50%)', minWidth: '28px', height: '28px', padding: '0 4px', borderRadius: '14px', cursor: locked ? 'default' : 'pointer',
            font: '700 .8rem/1 var(--body)', background: on ? 'transparent' : 'var(--surface)', color: 'var(--ink)', border: on ? '2px solid var(--accent)' : '2px dashed var(--ink)', opacity: locked && !on ? 0 : 1 }}>{on ? '' : p.n}</button>;
      })}
    </div>
  );
}

function Checks({ s }: { s: State }) {
  const sp = spot(s.spot), cs = check(s), bad = cs.filter((c) => c.ok === false).length;
  return (
    <div class="stack" style={{ gap: '8px' }} role="status" aria-live="polite">
      <div class={`feedback ${bad ? 'bad' : 'good'}`}><div class="verdict">{bad ? `${bad} rule${bad > 1 ? 's' : ''} broken` : 'Parked by the book'}</div>
        <p class="small">Spot {sp.n}: {sp.name} — {sp.propName}. {LOAD[s.load]}.</p></div>
      <ul style={{ listStyle: 'none', margin: 0, padding: 0, display: 'flex', flexDirection: 'column', gap: '6px' }}>{cs.map((c) => (
        <li style={{ display: 'grid', gridTemplateColumns: '28px 1fr', gap: '8px', padding: '6px 8px', borderRadius: '6px', border: `1.5px solid ${c.ok === false ? 'var(--red)' : c.ok ? 'var(--ok)' : 'var(--line)'}`, background: c.ok === false ? 'var(--red-soft)' : 'var(--surface)' }}>
          <span aria-hidden="true" style={{ font: '700 1.1rem/1.2 var(--body)', color: c.ok === false ? 'var(--red)' : c.ok ? 'var(--ok)' : 'var(--ink-2)' }}>{c.ok === false ? '✗' : c.ok ? '✓' : '–'}</span>
          <span class="small"><strong>{c.ok === false ? 'Broken: ' : c.ok ? 'OK: ' : 'N/A: '}{c.title}.</strong> {c.text} <span class="plate">p. {c.page}</span></span></li>))}</ul>
    </div>
  );
}

function Pick<T extends string>({ label, value, opts, set }: { label: string; value: T; opts: Record<T, string>; set: (v: T) => void }) {
  return (<div class="field"><span class="small" style={{ fontWeight: 700 }}>{label}</span>
    <div class="row" role="group" aria-label={label} style={{ gap: '6px' }}>{(Object.keys(opts) as T[]).map((k) => <button class="btn sm" aria-pressed={value === k} style={pressed(value === k)} onClick={() => set(k)}>{opts[k]}</button>)}</div></div>);
}

function Explore() {
  const [s, setS] = useState<State>({ load: 'exp', spot: 'shoulder', stop: 'park', att: 'cab', dev: 'none', smoke: 'none' });
  const up = (p: Partial<State>) => setS({ ...s, ...p });
  return (
    <div class="stack">
      <Pick<Load> label="Load" value={s.load} opts={LOAD} set={(load) => up({ load })} />
      <p class="small muted">Tap a numbered spot to move the truck. Features within 300 ft that matter for this load are outlined in red.</p>
      <Map s={s} onSpot={(spot) => up({ spot })} />
      <div class="grid2">
        <Pick<Stop> label="Why you stopped" value={s.stop} opts={{ park: 'Parked (meal, rest, overnight)', brief: 'Short stop the job needs (e.g. fueling)' }} set={(stop) => up({ stop })} />
        <Pick<Att> label="Who is watching the truck" value={s.att} opts={ATT} set={(att) => up({ att })} />
        <Pick<Dev> label="Broken down? Warning devices" value={s.dev} opts={DEV} set={(dev) => up({ dev })} />
        <Pick<Smoke> label="Smoking nearby" value={s.smoke} opts={SMOKE} set={(smoke) => up({ smoke })} />
      </div>
      <Checks s={s} />
    </div>
  );
}

const OPTS: [Key | 'ok', string][] = [['ok', 'Legal — parked by the book'], ['road', 'Too close to the traveled road (5 ft)'], ['r300', 'Breaks a 300 ft rule'], ['attend', 'Not properly attended'], ['flare', 'Wrong warning devices'], ['smoke', 'Smoking within 25 ft']];
const B: State = { load: 'exp', spot: 'shoulder', stop: 'park', att: 'cab', dev: 'none', smoke: 'none' };
export const SCEN: { text: string; s: State }[] = [
  { text: 'Explosives 1.1. You pull onto the highway shoulder for a rest and stay awake in the driver’s seat.', s: { ...B } },
  { text: 'Explosives 1.2. You park on the wide shoulder by the bridge for lunch and eat 60 ft away with the truck in view.', s: { ...B, spot: 'bridge', att: 'near' } },
  { text: 'Explosives 1.3. You stop at the fuel island just long enough to fuel. You stay by the truck, in clear view.', s: { ...B, spot: 'fuel', stop: 'brief', att: 'near' } },
  { text: 'Gasoline cargo tank (Class 3). You park at the gravel pullout for a break and stay in the cab. A brush fire is burning nearby.', s: { ...B, load: 'fl', spot: 'pullout' } },
  { text: 'Class 3 cargo tank breaks down on the wide shoulder by the bridge. You stay in the cab and set out flares.', s: { ...B, load: 'fl', spot: 'bridge', dev: 'flare' } },
  { text: 'Explosives 1.1 at a roadside turnout. A friend who drove along watches it from 50 ft while you go into the diner.', s: { ...B, spot: 'turnout', att: 'other' } },
  { text: 'Explosives 1.1 in the shipper’s yard, 350 ft from the warehouse. A co-worker watches it from 50 ft.', s: { ...B, spot: 'shipper', att: 'other' } },
  { text: 'Class 3 cargo tank at a roadside turnout. You sleep in the sleeper berth.', s: { ...B, load: 'fl', spot: 'turnout', att: 'sleeper' } },
  { text: 'Explosives 1.2 left overnight at a safe haven. Nobody stays with the truck.', s: { ...B, spot: 'haven', att: 'none' } },
  { text: 'Class 3 cargo tank at a roadside turnout. You stand by the cab while a helper smokes 10 ft away.', s: { ...B, load: 'fl', spot: 'turnout', att: 'near', smoke: 's10' } },
];

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<string | null>(null);
  const [miss, setMiss] = useState(0);
  const sc = SCEN[i];
  if (!sc) return (
    <div class={`feedback ${miss === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{miss === 0 ? `${SCEN.length} of ${SCEN.length} right — stamp earned: Parked by the book` : `${SCEN.length - miss} of ${SCEN.length} right — need all ${SCEN.length} for the stamp`}</div>
      <button class="btn sm" onClick={() => { setI(0); setMiss(0); setPick(null); }}>Judge again</button></div>
  );
  const truth = broken(sc.s)[0] ?? 'ok';
  const good = pick === truth;
  return (
    <div class="stack">
      <span class="small muted num">Truck {i + 1} of {SCEN.length}</span>
      <strong>{sc.text}</strong>
      <Map s={sc.s} locked />
      <p class="small">Legal, or which rule is broken?</p>
      <div class="stack" role="group" aria-label="Your ruling" style={{ gap: '6px' }}>{OPTS.map(([k, label]) => (
        <button class={`btn ${pick && k === truth ? 'primary' : ''}`} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick === k && k !== truth ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }} disabled={!!pick}
          onClick={() => { setPick(k); const ok = k === truth; if (!ok) setMiss(miss + 1); onEvidence({ concepts, ok }); }}>{pick && k === truth ? '✓ ' : pick === k ? '✗ ' : ''}{label}</button>))}</div>
      {pick && <>
        <div class={`feedback ${good ? 'good' : 'bad'}`} role="status"><div class="verdict">{good ? 'Right' : `It is: ${OPTS.find(([k]) => k === truth)![1]}`}</div>
          <button class="btn primary sm" onClick={() => { if (i + 1 === SCEN.length && miss === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === SCEN.length ? 'Finish' : 'Next truck'}</button></div>
        <Checks s={sc.s} />
      </>}
    </div>
  );
}

export default function ParkingRules(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Park it</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Judge 10 trucks</button></div>
      {mode === 'explore' ? <Explore /> : <Challenge {...props} />}
    </div>
  );
}
