import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk10-conditions', title: 'Night, fog, winter, heat', lesson: 'GK-10', anchor: /night driving procedures/i,
  summary: 'Switch conditions and try the handbook rules: dim at 500 ft, fog lights, tread depth, hot tires and the radiator cap. Then sort 8 do/don’t cards.',
  stamp: { id: 'all-weather', name: 'All-weather', rule: 'Sort all 8 do/don’t cards for night, fog, winter and heat with no mistakes.' },
};

type Cond = 'night' | 'fog' | 'winter' | 'heat';
const COND: { id: Cond; label: string }[] = [{ id: 'night', label: 'Night' }, { id: 'fog', label: 'Fog' }, { id: 'winter', label: 'Winter' }, { id: 'heat', label: 'Hot weather' }];
const T = { 'font-size': 13, fill: 'var(--ink)' } as const; // 240-wide diagrams: ≥ 14 px on a phone
const T14 = { 'font-size': 14, fill: 'var(--ink)' } as const; // 360-wide diagrams: ≥ 11 px on a phone

/** Handbook dimming rule (p. 2-31): dim within 500 ft of oncoming AND of a vehicle you follow. */
export const mustDim = (distFt: number) => distFt <= 500;

function Truck({ x, y }: { x: number; y: number }) {
  return <g><rect x={x} y={y} width="40" height="18" rx="2" fill="var(--accent)" stroke="var(--ink)" /><rect x={x + 42} y={y + 2} width="12" height="14" rx="2" fill="var(--accent)" stroke="var(--ink)" /></g>;
}
export function NightVis({ kind, d, high }: { kind: 'oncoming' | 'following'; d: number; high: boolean }) {
  const px = (ft: number) => 66 + ft * 0.26; const reach = high ? 500 : 250; const glare = high && mustDim(d);
  const ox = px(d); const oy = kind === 'oncoming' ? 40 : 88;
  return (
    <svg viewBox="0 0 360 140" width="100%" style={{ maxWidth: '460px', display: 'block', marginInline: 'auto' }} role="img" aria-label={`Night road. ${kind === 'oncoming' ? 'Oncoming' : 'Leading'} vehicle ${d} feet away. Your ${high ? 'high' : 'low'} beams reach about ${high ? '350 to 500' : '250'} feet.${glare ? ' Your high beams glare into the other driver’s eyes.' : ''}`}>
      <rect x="0" y="0" width="360" height="140" fill="#101813" />
      <rect x="0" y="24" width="360" height="96" fill="#2a352e" />
      <line x1="0" y1="72" x2="360" y2="72" stroke="var(--amber)" stroke-width="2" stroke-dasharray="10 6" />
      <path d={`M 66 88 L ${px(reach)} ${high ? 70 : 78} L ${px(reach)} ${high ? 116 : 108} Z`} fill="var(--amber)" fill-opacity={high ? 0.45 : 0.35} />
      {high && <path d={`M ${px(350)} 72 L ${px(500)} 70 L ${px(500)} 116 L ${px(350)} 112 Z`} fill="var(--amber)" fill-opacity="0.2" />}
      <Truck x={10} y={80} />
      <line x1={px(500)} y1="24" x2={px(500)} y2="120" stroke="#e6ece7" stroke-width="1" stroke-dasharray="3 3" />
      <text x={px(500)} y="136" text-anchor="middle" font-size="14" fill="#e6ece7">500 ft</text>
      <line x1={px(250)} y1="24" x2={px(250)} y2="120" stroke="#e6ece7" stroke-width="0.8" stroke-dasharray="2 4" /><text x={px(250)} y="136" text-anchor="middle" font-size="14" fill="#e6ece7">250 ft</text><text x="8" y="16" font-size="14" fill="#e6ece7">{high ? 'High beams: see ≈ 350–500 ft' : 'Low beams: see ≈ 250 ft'}</text>
      <g><rect x={ox} y={oy} width="30" height="16" rx="3" fill="#cfd8d2" stroke="#101813" />
        {kind === 'oncoming' ? <><circle cx={ox} cy={oy + 3} r="2.5" fill="#fff" /><circle cx={ox} cy={oy + 13} r="2.5" fill="#fff" /></> : <><rect x={ox + 28} y={oy + 1} width="3" height="4" fill="var(--red)" /><rect x={ox + 28} y={oy + 11} width="3" height="4" fill="var(--red)" /></>}</g>
      {glare && <g><circle cx={ox + 8} cy={oy + 8} r="16" fill="var(--amber)" fill-opacity="0.6" /><text x={ox + 40} y={oy + 13} font-size="14" font-weight="700" fill="#f2c230">GLARE</text></g>}
    </svg>
  );
}
export function FogVis({ high, flashers }: { high: boolean; flashers: boolean }) {
  return (
    <svg viewBox="0 0 360 120" width="100%" style={{ maxWidth: '460px', display: 'block', marginInline: 'auto' }} role="img" aria-label={`Truck in fog with ${high ? 'high beams: light bounces off the fog back into your eyes' : 'low beams and fog lights'}${flashers ? ', 4-way flashers on' : ''}. Roadside reflectors mark the curve.`}>
      <rect x="0" y="0" width="360" height="120" fill="var(--surface-2)" />
      <path d="M 0 96 C 140 96 220 90 360 50" stroke="var(--ink-2)" stroke-width="30" fill="none" stroke-opacity="0.4" />
      {[[170, 78], [230, 68], [290, 52], [340, 36]].map(([x, y]) => <rect x={x} y={y} width="4" height="8" fill="var(--amber)" stroke="var(--ink)" stroke-width="0.6" />)}
      <path d={`M 64 94 L ${high ? 200 : 140} ${high ? 70 : 84} L ${high ? 200 : 140} ${high ? 118 : 106} Z`} fill="var(--amber)" fill-opacity="0.35" />
      <Truck x={8} y={86} />
      {flashers && <><circle cx="8" cy="88" r="4" fill="var(--amber)" /><circle cx="8" cy="102" r="4" fill="var(--amber)" /><circle cx="66" cy="88" r="3" fill="var(--amber)" /></>}
      {[0, 1, 2].map((i) => <ellipse cx={110 + i * 90} cy={40 + (i % 2) * 30} rx="80" ry="22" fill="var(--surface)" fill-opacity="0.75" />)}
      {high && <><path d="M 130 80 L 72 90" stroke="var(--amber)" stroke-width="3" marker-end="url(#g10a)" /><text x="140" y="116" font-size="14" font-weight="700" fill="var(--red)">Light bounces back — glare</text></>}
      <text x="356" y="114" text-anchor="end" {...T14}>{high ? '' : 'Reflectors show the curve'}</text>
      <defs><marker id="g10a" viewBox="0 0 10 10" refX="5" refY="5" markerWidth="5" markerHeight="5" orient="auto"><path d="M0 0 L10 5 L0 10 Z" fill="var(--amber)" /></marker></defs>
    </svg>
  );
}
export function TreadVis({ depth, front }: { depth: number; front: boolean }) {
  const min = front ? 4 : 2; const ok = depth >= min;
  return (
    <svg viewBox="0 0 240 90" width="100%" style={{ maxWidth: '320px', display: 'block', marginInline: 'auto' }} role="img" aria-label={`Tread cross-section: ${depth}/32 inch deep on a ${front ? 'front' : 'non-front'} tire. Minimum ${min}/32. ${ok ? 'Meets it.' : 'Too shallow.'}`}>
      <rect x="0" y="0" width="240" height="90" fill="var(--surface)" />
      <rect x="10" y="20" width="220" height="60" rx="6" fill="var(--ink-2)" />
      {[50, 110, 170].map((x) => <rect x={x} y="20" width="18" height={depth * 5} fill="var(--surface)" stroke="var(--ink)" stroke-width="0.8" />)}
      <line x1="10" y1={20 + min * 5} x2="230" y2={20 + min * 5} stroke={ok ? 'var(--ok)' : 'var(--red)'} stroke-width="2" stroke-dasharray="5 3" />
      <text x="120" y="14" text-anchor="middle" {...T}>{`Groove ${depth}/32″ · minimum ${min}/32″ ${ok ? '✓' : '✗'}`}</text>
    </svg>
  );
}
export function CapVis({ cool, opened }: { cool: boolean; opened: boolean }) {
  const burn = opened && !cool;
  return (
    <svg viewBox="0 0 240 110" width="100%" style={{ maxWidth: '320px', display: 'block', marginInline: 'auto' }} role="img" aria-label={burn ? 'Radiator cap opened hot: steam and boiling coolant blast out.' : opened ? 'Cool cap turned slowly to the first stop; pressure escapes safely while you step back.' : `Radiator cap, ${cool ? 'cool enough to touch bare-handed' : 'hot'}, closed.`}>
      <rect x="0" y="0" width="240" height="110" fill="var(--surface)" />
      <rect x="60" y="40" width="120" height="60" rx="4" fill="var(--surface-2)" stroke="var(--ink)" />
      {[0, 1, 2, 3, 4, 5].map((i) => <line x1={70 + i * 20} y1="46" x2={70 + i * 20} y2="94" stroke="var(--ink-2)" />)}
      <rect x="108" y={opened ? 18 : 30} width="24" height="10" rx="3" fill={cool ? 'var(--blue)' : 'var(--red)'} stroke="var(--ink)" />
      <rect x="10" y="30" width="12" height="70" rx="6" fill="var(--surface-2)" stroke="var(--ink)" /><rect x="12" y={cool ? 80 : 36} width="8" height={cool ? 18 : 62} rx="4" fill={cool ? 'var(--blue)' : 'var(--red)'} />
      <text x="30" y="24" {...T}>{cool ? 'Cool to touch' : 'Hot'}</text>
      {burn && <>{[0, 1, 2].map((i) => <ellipse cx={110 + i * 12} cy={14 - i * 3} rx="16" ry="9" fill="var(--surface-2)" stroke="var(--red)" stroke-width="1.5" />)}<text x="200" y="30" text-anchor="middle" font-size="13" font-weight="700" fill="var(--red)">BURNS</text></>}
      {opened && cool && <text x="200" y="30" text-anchor="middle" {...T}>1st stop ✓</text>}
    </svg>
  );
}

export function TireVis({ letOut }: { letOut: boolean }) {
  return (
    <svg viewBox="0 0 240 100" width="100%" style={{ maxWidth: '320px', display: 'block', marginInline: 'auto' }} role="img" aria-label={letOut ? 'Air let out while hot: after cooling the tire is underinflated and sags.' : 'Hot tire left alone: pressure returns to normal as it cools.'}>
      <rect x="0" y="0" width="240" height="100" fill="var(--surface)" />
      <ellipse cx="70" cy={letOut ? 60 : 55} rx="44" ry={letOut ? 34 : 40} fill="var(--ink-2)" stroke="var(--ink)" /><circle cx="70" cy={letOut ? 60 : 55} r="16" fill="var(--surface-2)" stroke="var(--ink)" />
      <rect x="20" y="92" width="100" height="4" fill="var(--ink)" />
      <text x="126" y="36" {...T}>After it cools:</text>
      <text x="126" y="60" font-size="14" font-weight="700" fill={letOut ? 'var(--red)' : 'var(--ok)'}><tspan x="126">{letOut ? '✗ Pressure' : '✓ Pressure'}</tspan><tspan x="126" dy="18">{letOut ? 'too low' : 'normal'}</tspan></text>
    </svg>
  );
}
export function IceVis({ retarder }: { retarder: boolean }) {
  return (
    <svg viewBox="0 0 240 90" width="100%" style={{ maxWidth: '320px', display: 'block', marginInline: 'auto' }} role="img" aria-label={retarder ? 'Retarder on over ice: the drive wheels lock and skid.' : 'Retarder off: the drive wheels keep rolling on the ice.'}>
      <rect x="0" y="0" width="240" height="90" fill="var(--blue-soft)" />
      <rect x="20" y="30" width="110" height="30" rx="2" fill="var(--surface)" stroke="var(--ink)" /><rect x="134" y="34" width="36" height="26" rx="3" fill="var(--accent)" stroke="var(--ink)" />
      {[40, 100, 150].map((x) => <circle cx={x} cy="64" r="9" fill="var(--ink-2)" stroke="var(--ink)" />)}
      {retarder && <><path d="M 60 74 L 108 74 M 60 79 L 108 79" stroke="var(--red)" stroke-width="2" stroke-dasharray="4 3" /><text x="190" y="24" text-anchor="middle" font-size="13" font-weight="700" fill="var(--red)">SKID</text></>}
      <text x="190" y="84" text-anchor="middle" {...T}>{retarder ? 'Retarder ON' : 'Retarder OFF ✓'}</text>
    </svg>
  );
}

interface Card { cond: Cond; t: string; ok: boolean; why: string; page: string; vis: (bad: boolean) => preact.JSX.Element }
export const CARDS: Card[] = [
  { cond: 'night', t: 'At night, switch to low beams when the vehicle you are following is 400 ft ahead.', ok: true, why: 'Dim within 500 ft of a vehicle you follow, not just oncoming ones — your high beams hit their mirrors. (The car handbook says 300 ft; this test uses 500.)', page: '2-31', vis: (bad) => <NightVis kind="following" d={400} high={bad} /> },
  { cond: 'night', t: 'An oncoming driver won’t dim, so flash your high beams back at them.', ok: false, why: 'Never “get back at them.” It adds glare for them and raises the chance of a crash. Look slightly to the right, at the lane or edge line.', page: '2-31', vis: (bad) => <NightVis kind="oncoming" d={300} high={bad} /> },
  { cond: 'fog', t: 'If you must drive in fog, use low beams and fog lights, plus your 4-way flashers.', ok: true, why: 'High beams reflect off the fog back into your eyes. Flashers help drivers behind notice you sooner.', page: '2-32', vis: (bad) => <FogVis high={bad} flashers={!bad} /> },
  { cond: 'fog', t: 'Fog is thick, so stop on the side of the road until it lifts.', ok: false, why: 'Don’t stop at the roadside unless you absolutely must — drivers behind may not see you in time. Best: pull into a rest area or truck stop.', page: '2-32', vis: () => <FogVis high={false} flashers /> },
  { cond: 'winter', t: 'Front tires with 3/32 inch of tread are fine for winter driving.', ok: false, why: 'Front tires need at least 4/32 inch in every major groove; other tires at least 2/32. Steering tires need grip to steer.', page: '2-32', vis: () => <TreadVis depth={3} front /> },
  { cond: 'winter', t: 'On an icy road, don’t use the engine brake or speed retarder.', ok: true, why: 'On a slippery road they can make the drive wheels skid.', page: '2-33', vis: (bad) => <IceVis retarder={bad} /> },
  { cond: 'heat', t: 'A tire is hot and its pressure is high — let some air out.', ok: false, why: 'Never let air out. Pressure rises with heat; when the tire cools it will be too low. If it is too hot to touch, stay stopped until it cools, or it may blow out or catch fire.', page: '2-34', vis: (bad) => <TireVis letOut={bad} /> },
  { cond: 'heat', t: 'Open the radiator cap only when it is cool enough to touch bare-handed, turning it slowly to the first stop.', ok: true, why: 'A hot pressurized system can blast out steam and boiling coolant and badly burn you. Cool cap → gloves or thick cloth, first stop, step back.', page: '2-34', vis: (bad) => <CapVis cool={!bad} opened /> },
];

/** Segmented control: equal columns, so options never wrap raggedly on a phone. */
function Seg<V extends string>({ label, value, opts, set }: { label: string; value: V | ''; opts: [V, string][]; set: (v: V) => void }) {
  return <div role="group" aria-label={label} style={{ display: 'grid', gridTemplateColumns: `repeat(${opts.length}, minmax(0, 1fr))`, gap: '6px' }}>{opts.map(([v, t]) => (
    <button class="btn sm" aria-pressed={value === v} style={{ width: '100%', paddingInline: '6px', lineHeight: 1.2, ...(value === v ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}) }} onClick={() => set(v)}>{t}</button>))}</div>;
}
const P = ({ p }: { p: string }) => <span class="plate">p. {p}</span>;

function Night() {
  const [kind, setKind] = useState<'oncoming' | 'following'>('oncoming');
  const [d, setD] = useState(800);
  const [high, setHigh] = useState(true);
  const dim = mustDim(d);
  const v = high && dim ? { cls: 'bad', h: 'Dim now', t: kind === 'oncoming' ? 'Your high beams dazzle the oncoming driver.' : 'Your light hits their mirrors and bothers the driver ahead.' }
    : !high && !dim ? { cls: 'bad', h: 'Use your high beams', t: 'Nobody within 500 ft. Low beams cut your view to about 250 ft — use high beams whenever safe and legal.' }
    : { cls: 'good', h: high ? 'High beams — correct' : 'Low beams — correct', t: high ? 'No vehicle within 500 ft, so high beams are safe and legal: you see about 350–500 ft.' : `A vehicle ${kind === 'oncoming' ? 'coming toward you' : 'you are following'} is within 500 ft, so dim before you cause glare.` };
  return (
    <div class="stack">
      <Seg label="Other vehicle" value={kind} set={setKind} opts={[['oncoming', 'Oncoming'], ['following', 'You are following']]} />
      <div class="field"><label for="g10-d">Distance to it: <span class="num">{d} ft</span></label><input id="g10-d" type="range" min={100} max={1000} step={50} value={d} onInput={(e) => setD(+(e.target as HTMLInputElement).value)} /></div>
      <Seg label="Your headlights" value={high ? 'h' : 'l'} set={(x) => setHigh(x === 'h')} opts={[['l', 'Low beams'], ['h', 'High beams']]} />
      <NightVis kind={kind} d={d} high={high} />
      <div class={`feedback ${v.cls}`} role="status" aria-live="polite"><div class="verdict">{v.h}</div><p class="small">{v.t} <P p="2-31" /></p></div>
      <ul class="small">
        <li>Drive slow enough to stop within your headlight range (low ≈ 250 ft, high ≈ 350–500 ft). <P p="2-30" /></li>
        <li>Glare: look slightly right, at the lane or edge line. Never retaliate with high beams. <P p="2-31" /></li>
        <li>No sunglasses or tinted lenses at night. Keep the dome light off. Dirty headlights may give only half the light. <P p="2-31" /></li>
      </ul>
    </div>
  );
}
function Fog() {
  const [high, setHigh] = useState(false); const [fl, setFl] = useState(true);
  return (
    <div class="stack">
      <div class="card warn small"><strong>Best advice: don’t drive in fog.</strong> Pull into a rest area or truck stop and wait. <P p="2-32" /></div>
      <Seg label="Lights in fog" value={high ? 'h' : 'l'} set={(x) => setHigh(x === 'h')} opts={[['l', 'Low beams + fog lights'], ['h', 'High beams']]} />
      <label class="toggle" style={{ minHeight: '44px' }}><input type="checkbox" checked={fl} onChange={(e) => setFl((e.target as HTMLInputElement).checked)} />4-way flashers</label>
      <FogVis high={high} flashers={fl} />
      <div class={`feedback ${!high && fl ? 'good' : 'bad'}`} role="status" aria-live="polite"><p class="small">{high ? 'High beams reflect off the fog back into your eyes. Use low beams and fog lights, even in daytime.' : fl ? 'Right: low beams, fog lights and 4-way flashers so drivers behind see you sooner.' : 'Turn on your 4-way flashers so drivers coming up behind notice you sooner.'} <P p="2-32" /></p></div>
      <ul class="small"><li>Slow down <strong>before</strong> you enter fog; obey fog signs.</li><li>Lights ahead may not show where the road is — use roadside reflectors for the curve.</li><li>Listen for traffic; avoid passing; don’t stop at the roadside unless you absolutely must. <P p="2-32" /></li></ul>
    </div>
  );
}
function Winter() {
  const [front, setFront] = useState(true); const [dp, setDp] = useState(3);
  return (
    <div class="stack">
      <Seg label="Which tire" value={front ? 'f' : 'o'} set={(x) => setFront(x === 'f')} opts={[['f', 'Front tire'], ['o', 'Any other tire']]} />
      <div class="field"><label for="g10-t">Tread depth: <span class="num">{dp}/32 inch</span></label><input id="g10-t" type="range" min={0} max={8} step={1} value={dp} onInput={(e) => setDp(+(e.target as HTMLInputElement).value)} /></div>
      <TreadVis depth={dp} front={front} />
      <p class="small" role="status" aria-live="polite">{dp >= (front ? 4 : 2) ? 'Meets the minimum. Deeper is better.' : `Too shallow: ${front ? 'front tires need 4/32 inch' : 'other tires need 2/32 inch'} in every major groove.`} <P p="2-32" /></p>
      <ul class="small">
        <li>Chains: right number, extra cross-links, fit the drive tires, practice putting them on. Washer antifreeze in the reservoir. <P p="2-32" /></li>
        <li>Winterfront not too tight and shutters free of ice, or the engine can overheat and quit. Exhaust leaks → carbon monoxide. <P p="2-33" /></li>
        <li><strong>No spray</strong> from other tires = ice. Don’t use the engine brake or retarder. Melting ice is <strong>even more</strong> slippery. <P p="2-33" /></li>
      </ul>
    </div>
  );
}
function Heat() {
  const [cool, setCool] = useState(false); const [open, setOpen] = useState(false); const [air, setAir] = useState<'' | 'out' | 'wait'>('');
  return (
    <div class="stack">
      <div class="card tint small">In very hot weather, inspect tires <strong>every 2 hours or every 100 miles</strong>. <P p="2-34" /></div>
      <strong class="small">A tire is hot and its pressure reads high. You…</strong>
      <Seg label="Hot tire" value={air} set={setAir} opts={[['out', 'Let some air out'], ['wait', 'Leave the pressure alone']]} />
      {air && <div class={`feedback ${air === 'wait' ? 'good' : 'bad'}`} role="status"><TireVis letOut={air === 'out'} /><p class="small">{air === 'out' ? '✗ When the tire cools, the pressure will be too low. Never let air out of a hot tire.' : '✓ Pressure goes up as tires heat. If a tire is too hot to touch, stay stopped until it cools, or it may blow out or catch fire.'} <P p="2-34" /></p></div>}
      <strong class="small">Radiator cap</strong>
      <Seg label="Cap temperature" value={cool ? 'c' : 'h'} set={(x) => { setCool(x === 'c'); setOpen(false); }} opts={[['h', 'Engine just shut off (hot)'], ['c', 'Cool enough to touch bare-handed']]} />
      <CapVis cool={cool} opened={open} />
      <button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={() => setOpen(!open)}>{open ? 'Close the cap' : 'Open the cap'}</button>
      {open && <div class={`feedback ${cool ? 'good' : 'bad'}`} role="status"><p class="small">{cool ? '✓ Protect your hands, turn slowly to the first stop, step back while pressure escapes, then remove it.' : '✗ The hot system is under pressure: steam and boiling coolant blast out and cause severe burns. Wait until it has cooled.'} <P p="2-34" /></p></div>}
      <ul class="small"><li>A loose belt can’t turn the water pump or fan right → overheating. <P p="2-34" /></li><li>Bleeding tar is very slippery. Go slowly enough to prevent overheating. <P p="2-34" /></li></ul>
    </div>
  );
}

export default function Conditions({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  const [cond, setCond] = useState<Cond>('night');
  const [i, setI] = useState(0); const [pick, setPick] = useState<boolean | null>(null); const [misses, setMisses] = useState(0);
  const c = CARDS[i];
  const answer = (v: boolean) => { if (pick !== null) return; setPick(v); const ok = v === c.ok; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const next = () => { if (i + 1 === CARDS.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); };
  const reset = () => { setI(0); setPick(null); setMisses(0); };
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => { setMode('challenge'); reset(); }}>Do or don’t (8)</button></div>
      {mode === 'explore' && <>
        <Seg label="Condition" value={cond} set={setCond} opts={COND.map((x) => [x.id, x.label] as [Cond, string])} />
        {cond === 'night' && <Night />}{cond === 'fog' && <Fog />}{cond === 'winter' && <Winter />}{cond === 'heat' && <Heat />}
      </>}
      {mode === 'challenge' && (c ? (
        <div class="stack">
          <span class="small muted num">Card {i + 1} of {CARDS.length} · {COND.find((x) => x.id === c.cond)!.label}</span>
          <div class="card"><strong>{c.t}</strong></div>
          <div class="row" role="group" aria-label="Do or don’t">
            {[true, false].map((v) => <button class={`btn ${pick !== null && v === c.ok ? 'primary' : ''}`} style={{ flex: 1, ...(pick !== null && v === c.ok ? { opacity: 1 } : {}), ...(pick === v && v !== c.ok ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}) }} disabled={pick !== null} onClick={() => answer(v)}>{v ? '✓ Do' : '✗ Don’t'}</button>)}
          </div>
          {pick !== null && c.vis(pick !== c.ok || !c.ok)}
          {pick !== null && <div class={`feedback ${pick === c.ok ? 'good' : 'bad'}`} role="status">
            <div class="verdict">{pick === c.ok ? 'Right' : `It’s a ${c.ok ? 'do' : 'don’t'}`}</div>
            <p class="small">{c.why} <P p={c.page} /></p>
            <button class="btn primary sm" onClick={next}>{i + 1 === CARDS.length ? 'Finish' : 'Next card'}</button></div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 8 right — stamp earned' : `${CARDS.length - misses} of ${CARDS.length} right`}</div>
          <button class="btn sm" onClick={reset}>Sort again</button></div>
      ))}
    </div>
  );
}
