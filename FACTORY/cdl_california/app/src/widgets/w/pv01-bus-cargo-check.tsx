import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'pv01-bus-cargo-check', title: 'Bus loading desk', lesson: 'PV-01', anchor: /hazmat a bus may carry/i,
  summary: 'Riders and packages arrive at the door. Allow, refuse, or fix each one by the handbook, and test the 100 lb / 500 lb HazMat limits.',
  stamp: { id: 'safe-boarding', name: 'Safe boarding', rule: 'Decide all 11 arrivals at the loading desk correctly, first try.' },
};

type Act = 'allow' | 'refuse' | 'fix';
type Spot = 'aisle' | 'exit' | 'front' | 'door' | 'seat';
type Look = 'bag' | 'box' | 'cyl' | 'rider' | 'battery' | 'pet' | 'dog';
interface Item {
  id: string; text: string; spot: Spot; look: Look; label?: string; act: Act;
  why: string; fix?: string; oops: string; page: string;
}
/** Every rule below is from PV-01 (DL 650 pp. 4-2 to 4-3). */
export const ITEMS: Item[] = [
  { id: 'aisle', text: 'A rider drops a big carry-on bag in the aisle and sits down.', spot: 'aisle', look: 'bag', act: 'fix', fix: 'Have it stowed out of the aisle.',
    why: 'Carry-ons may ride, but never in a doorway or the aisle. Nothing in the aisle that could trip someone.', oops: 'Left there, a rider trips over it, and in an emergency the aisle is blocked.', page: '4-2' },
  { id: 'ammo', text: 'A box marked ORM-D: small-arms ammunition.', spot: 'door', look: 'box', label: 'ORM-D', act: 'allow',
    why: 'A bus may carry small-arms ammunition labeled ORM-D.', oops: 'Refusing it turns away cargo the handbook says a bus may carry.', page: '4-3' },
  { id: 'gas23', text: 'A cylinder with a diamond label: Division 2.3 poison gas.', spot: 'door', look: 'cyl', label: '2.3', act: 'refuse',
    why: 'Division 2.3 poison gas is on the never list, with liquid Class 6 poison, tear gas and irritating material.', oops: 'Loaded, a leak would put poison gas in the bus with the riders.', page: '4-3' },
  { id: 'standee', text: 'A standing rider waits beside your seat, ahead of the standee line.', spot: 'front', look: 'rider', act: 'fix', fix: 'Move the rider behind the standee line.',
    why: 'No rider may stand forward of the rear of the driver’s seat. Standing riders stay behind the 2-inch standee line.', oops: 'Riders up front block your view and can fall into you or the windshield.', page: '4-3' },
  { id: 'battery', text: 'A rider carries a car battery up the steps.', spot: 'door', look: 'battery', act: 'refuse',
    why: 'Riders bring unlabeled HazMat. Do not let riders carry on common hazards such as car batteries or gasoline.', oops: 'Battery acid is a corrosive. It can spill on riders in a hard stop.', page: '4-3' },
  { id: 'oxygen', text: 'A rider has medically prescribed oxygen in a personal-use container and keeps it with them.', spot: 'seat', look: 'cyl', label: 'O₂', act: 'allow',
    why: 'Oxygen is allowed when it is medically prescribed, in the rider’s possession, and in a container made for personal use.', oops: 'Refusing it denies a rider oxygen the handbook allows.', page: '4-3' },
  { id: 'pet', text: 'A rider brings a cat in a pet carrier.', spot: 'door', look: 'pet', act: 'refuse',
    why: 'Carrying animals is prohibited, except certified service, guide, or signal dogs.', oops: 'A pet is not a certified service, guide or signal dog, so it may not ride.', page: '4-3' },
  { id: 'guide', text: 'A blind passenger boards with a certified guide dog.', spot: 'seat', look: 'dog', act: 'allow',
    why: 'Certified service, guide, or signal dogs used by passengers with disabilities may ride (Civil Code §54.2).', oops: 'Refusing breaks the California rule that allows certified guide dogs.', page: '4-3' },
  { id: 'exit', text: 'Luggage is stacked against the rear emergency door.', spot: 'exit', look: 'bag', act: 'fix', fix: 'Move it so the exit is clear.',
    why: 'Secure baggage so riders can get out by any window or door in an emergency.', oops: 'In a fire or crash, riders at the back cannot get out that door.', page: '4-2' },
  { id: 'solid6', text: 'A shipment of 120 lb of solid Class 6 poison (pesticide).', spot: 'door', look: 'box', label: '6', act: 'refuse',
    why: 'A bus must never carry more than 100 lb of solid Class 6 poisons.', oops: '120 lb is over the 100 lb limit for solid Class 6 poison.', page: '4-3' },
  { id: 'split', text: 'Two small shipments the shipper can’t send any other way: 90 lb of one allowed class, 90 lb of another.', spot: 'door', look: 'box', label: '90+90', act: 'allow',
    why: 'Each class is under 100 lb and the total, 180 lb, is under 500 lb. Both limits are met.', oops: 'Nothing is over: no class tops 100 lb and the total is far under 500 lb.', page: '4-3' },
];
const ACTS: { a: Act; label: string; icon: string }[] = [
  { a: 'allow', label: 'Allow on board', icon: '✓' }, { a: 'refuse', label: 'Refuse', icon: '✕' }, { a: 'fix', label: 'Fix first', icon: '⚠' },
];
const SPOT: Record<Spot, [number, number]> = { aisle: [222, 96], exit: [40, 96], front: [296, 96], door: [334, 156], seat: [200, 58] };
const STOW: [number, number] = [150, 196];
const BEHIND: [number, number] = [250, 96];
const OFF: [number, number] = [334, 200];

function Thing({ it, x, y }: { it: Item; x: number; y: number }) {
  const s = 'var(--ink)';
  return (
    <g transform={`translate(${x} ${y})`}>
      {it.look === 'bag' && <><rect x="-16" y="-10" width="32" height="22" rx="4" fill="var(--amber-soft)" stroke={s} stroke-width="1.5" /><path d="M-6 -10 v-5 h12 v5" fill="none" stroke={s} stroke-width="1.5" /></>}
      {it.look === 'box' && <><rect x="-23" y="-14" width="46" height="28" rx="2" fill="var(--surface)" stroke={s} stroke-width="1.5" />
        {it.label && it.label !== 'ORM-D' && it.label !== '90+90' ? <><rect x="-8" y="-8" width="16" height="16" transform="rotate(45)" fill="var(--amber)" stroke={s} /><text y="4" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--ink)">{it.label}</text></>
          : <text y="4" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--ink)">{it.label}</text>}</>}
      {it.look === 'cyl' && <><rect x="-9" y="-16" width="18" height="32" rx="8" fill={it.label === 'O₂' ? 'var(--blue-soft)' : 'var(--surface)'} stroke={s} stroke-width="1.5" />
        {it.label === 'O₂' ? <text y="5" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--ink)">O₂</text>
          : <><rect x="-7" y="-7" width="14" height="14" transform="rotate(45)" fill="var(--red-soft)" stroke="var(--red)" /><text y="4" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--ink)">2</text></>}</>}
      {it.look === 'rider' && <><circle r="11" fill="var(--blue)" stroke={s} stroke-width="1.5" /><circle r="4" fill="var(--surface)" /></>}
      {it.look === 'battery' && <><rect x="-14" y="-10" width="28" height="20" rx="2" fill="var(--ink-2)" stroke={s} stroke-width="1.5" /><text y="4" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--surface)">+ −</text></>}
      {it.look === 'pet' && <><rect x="-15" y="-11" width="30" height="22" rx="8" fill="var(--surface)" stroke={s} stroke-width="1.5" /><text y="4" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--ink)">cat</text></>}
      {it.look === 'dog' && <><rect x="-16" y="-10" width="32" height="20" rx="8" fill="var(--accent-soft)" stroke={s} stroke-width="1.5" /><text y="4" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--ink)">dog</text></>}
    </g>
  );
}

/** Top view of the bus: front at the right, so the driver (left) side is the top edge. */
function BusPlan({ it, act, reducedMotion }: { it: Item | null; act: Act | null; reducedMotion: boolean }) {
  const ok = it && act ? act === it.act : null;
  let pos = it ? SPOT[it.spot] : null;
  if (it && act) pos = act === 'refuse' ? OFF : act === 'fix' ? (it.id === 'standee' ? BEHIND : STOW) : it.spot === 'door' ? SPOT.seat : SPOT[it.spot];
  const tag = !it || !act ? null : ok ? (act === 'refuse' ? 'REFUSED' : act === 'fix' ? 'FIXED' : 'ALLOWED') : 'PROBLEM';
  const trans = reducedMotion ? undefined : 'transform .45s ease';
  const aria = `Top view of the bus, front at the right. Driver seat front left, service door front right, a 2-inch standee line just behind the driver's seat, the aisle down the middle, a rear emergency door, and the baggage bay under the floor.${it ? ` Now at the desk: ${it.text}${tag ? ` Result: ${tag}.` : ''}` : ''}`;
  return (
    <svg viewBox="0 0 360 234" width="100%" role="img" aria-label={aria} style={{ maxWidth: '560px', display: 'block', margin: '0 auto' }}>
      <rect width="360" height="234" fill="var(--surface-2)" />
      <text x="6" y="16" font-size="13" fill="var(--ink-2)">◀ rear</text><text x="354" y="16" text-anchor="end" font-size="13" fill="var(--ink-2)">front ▶</text>
      <rect x="22" y="24" width="316" height="146" rx="12" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
      {[0, 1, 2, 3, 4, 5, 6].map((i) => <rect key={`t${i}`} x={54 + i * 32} y="32" width="24" height="40" rx="3" fill="var(--surface-2)" stroke="var(--ink-2)" />)}
      {[0, 1, 2, 3, 4, 5].map((i) => <rect key={`b${i}`} x={54 + i * 32} y="120" width="24" height="40" rx="3" fill="var(--surface-2)" stroke="var(--ink-2)" />)}
      <rect x="40" y="82" width="250" height="28" fill="var(--accent-soft)" opacity=".6" /><text x="62" y="101" font-size="12.5" fill="var(--ink-2)">aisle: keep clear</text>
      <rect x="298" y="32" width="28" height="36" rx="4" fill="var(--blue-soft)" stroke="var(--ink)" /><text x="312" y="54" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--ink)">D</text>
      <line x1="290" y1="30" x2="290" y2="164" stroke="var(--amber)" stroke-width="4" /><text x="286" y="178" text-anchor="end" font-size="12.5" font-weight="700" fill="var(--amber-ink)">standee line ▲</text>
      <rect x="310" y="160" width="28" height="10" fill="var(--accent)" /><text x="324" y="184" text-anchor="middle" font-size="12.5" fill="var(--ink)">door</text>
      <rect x="22" y="82" width="8" height="28" fill="var(--red)" /><text x="12" y="140" font-size="12.5" fill="var(--red)" transform="rotate(-90 12 140)">EXIT</text>
      {[120, 216].map((x) => <rect key={x} x={x} y="22" width="30" height="5" fill="var(--red)" />)}<text x="135" y="20" text-anchor="middle" font-size="12.5" fill="var(--red)">exit window</text>
      <rect x="40" y="186" width="220" height="26" rx="4" fill="var(--surface)" stroke="var(--ink-2)" stroke-dasharray="4 3" /><text x="44" y="228" font-size="12.5" fill="var(--ink-2)">baggage bay / stowed</text>
      <text x="334" y="228" text-anchor="middle" font-size="12.5" fill="var(--ink-2)">off bus</text>
      {it && pos && <g style={{ transition: trans, transform: `translate(${pos[0]}px, ${pos[1]}px)` }}><Thing it={it} x={0} y={0} /></g>}
      {it && tag && pos && <g transform={`translate(${pos[1] >= 180 ? (pos[0] >= 300 ? pos[0] - 70 : pos[0] + 66) : Math.min(Math.max(pos[0], 52), 308)} ${pos[1] >= 180 ? pos[1] : pos[1] + 30})`}>
        <rect x="-46" y="-11" width="92" height="20" rx="4" fill={ok ? 'var(--ok)' : 'var(--red)'} /><text y="4" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--surface)">{ok ? '✓ ' : '✕ '}{tag}</text></g>}
    </svg>
  );
}

function Verdict({ it, act }: { it: Item; act: Act }) {
  const ok = act === it.act;
  const right = ACTS.find((a) => a.a === it.act)!.label;
  return (
    <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
      <div class="verdict">{ok ? `✓ Right: ${right}` : `✕ Should be: ${right}`}</div>
      {!ok && <p class="small"><strong>What happens:</strong> {it.oops}</p>}
      {it.fix && <p class="small"><strong>The fix:</strong> {it.fix}</p>}
      <p class="small">{it.why} <span class="plate">p. {it.page}</span></p>
    </div>
  );
}

const CLASSES = [
  { k: 'c3', name: 'Class 3 flammable liquid' }, { k: 'c4', name: 'Class 4 flammable solid' }, { k: 'c5', name: 'Class 5 oxidizer' }, { k: 'c6', name: 'Class 6 poison (solid)' },
  { k: 'c8', name: 'Class 8 corrosive' }, { k: 'c9', name: 'Class 9 miscellaneous' },
];
/** Limits (p. 4-3): no single class over 100 lb (solid Class 6 poison: never over 100 lb), never over 500 lb in all. */
export function checkLoad(w: Record<string, number>) {
  const total = Object.values(w).reduce((a, b) => a + b, 0);
  const over = CLASSES.filter((c) => (w[c.k] || 0) > 100);
  return { total, over, ok: total <= 500 && over.length === 0 };
}

function Scale() {
  const [w, setW] = useState<Record<string, number>>({ c3: 0, c4: 90, c5: 90, c6: 0, c8: 0, c9: 0 });
  const r = checkLoad(w);
  const bar = (v: number, max: number, lim: number, label: string) => (
    <svg viewBox="0 0 300 22" width="100%" aria-hidden="true">
      <rect x="0" y="4" width="300" height="14" rx="3" fill="var(--surface-2)" stroke="var(--line)" />
      <rect x="0" y="4" width={Math.min(300, (v / max) * 300)} height="14" rx="3" fill={v > lim ? 'var(--red)' : 'var(--ok)'} />
      <line x1={(lim / max) * 300} x2={(lim / max) * 300} y1="0" y2="22" stroke="var(--ink)" stroke-width="2" stroke-dasharray="3 2" /><title>{label}</title>
    </svg>
  );
  return (
    <div class="card stack">
      <div class="spread"><strong>HazMat weight check</strong><span class="plate">p. 4-3</span></div>
      <p class="small muted">Only small amounts the shipper cannot send any other way. Slide each class. Dashed line = the limit. Both rules apply: 6 classes of up to 100 lb each can still add up to more than 500 lb.</p>
      <div class="row" role="group" aria-label="Try a load" style={{ gap: '6px' }}>
        <button class="btn sm" onClick={() => setW({ c3: 0, c4: 90, c5: 90, c6: 0, c8: 0, c9: 0 })}>90 + 90 lb</button>
        <button class="btn sm" onClick={() => setW({ c3: 0, c4: 120, c5: 0, c6: 0, c8: 0, c9: 0 })}>One class 120 lb</button>
        <button class="btn sm" onClick={() => setW({ c3: 90, c4: 90, c5: 90, c6: 90, c8: 90, c9: 90 })}>6 classes × 90 lb</button>
      </div>
      {CLASSES.map((c) => (
        <div class="field" key={c.k} style={{ gap: '2px' }}>
          <label for={`pv-${c.k}`} class="small spread"><span>{c.name}</span><span class="num" style={{ fontWeight: 700, color: (w[c.k] || 0) > 100 ? 'var(--red)' : 'var(--ink)' }}>{w[c.k]} lb{(w[c.k] || 0) > 100 ? ' ✕ over 100' : ''}</span></label>
          <input id={`pv-${c.k}`} type="range" min={0} max={150} step={10} value={w[c.k]} onInput={(e) => setW({ ...w, [c.k]: +(e.target as HTMLInputElement).value })} />
          {bar(w[c.k] || 0, 150, 100, `${c.name}: ${w[c.k]} of 100 lb`)}
        </div>
      ))}
      <div class="small spread"><strong>Total allowed HazMat</strong><span class="num" style={{ fontWeight: 700 }}>{r.total} lb of 500</span></div>
      {bar(r.total, 900, 500, `Total ${r.total} of 500 lb`)}
      <div class={`feedback ${r.ok ? 'good' : 'bad'}`} role="status" aria-live="polite">
        <div class="verdict">{r.ok ? '✓ Within the limits' : '✕ Refuse this load'}</div>
        <p class="small">{r.ok ? `Every class is 100 lb or less, and the total (${r.total} lb) is 500 lb or less.`
          : [r.over.length ? `Over 100 lb of one class: ${r.over.map((c) => c.name).join(', ')}.${r.over.some((c) => c.k === 'c6') ? ' Solid Class 6 poison is never allowed over 100 lb.' : ''}` : '', r.total > 500 ? `Total ${r.total} lb is over 500 lb in all.` : ''].join(' ')}</p>
      </div>
      <p class="small muted">Liquid Class 6 poison and Division 2.3 poison gas are never allowed at any weight. Explosives (except small-arms ammo) and labeled radioactive materials never ride in the space people occupy.</p>
    </div>
  );
}

function Desk({ it, act, onAct, locked }: { it: Item; act: Act | null; onAct: (a: Act) => void; locked: boolean }) {
  return (
    <div class="stack" style={{ gap: '8px' }}>
      <div class="card tint" style={{ padding: '10px 12px' }}><div class="eyebrow">At the loading desk</div><strong>{it.text}</strong></div>
      <div class="row" role="group" aria-label="Your decision choices">
        {ACTS.map((a) => {
          const on = act === a.a, good = act && a.a === it.act;
          return <button key={a.a} class={`btn sm ${good ? 'primary' : ''}`} aria-pressed={on} disabled={locked && !!act}
            style={on && !good ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : good ? { borderColor: 'var(--accent)' } : {}} onClick={() => onAct(a.a)}>{a.icon} {a.label}</button>;
        })}
      </div>
    </div>
  );
}

export default function BusCargoCheck({ onEvidence, onChallenge, concepts, reducedMotion }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'scale' | 'challenge'>('explore');
  const [pick, setPick] = useState(0);
  const [xAct, setXAct] = useState<Act | null>(null);
  const [i, setI] = useState(0);
  const [cAct, setCAct] = useState<Act | null>(null);
  const [misses, setMisses] = useState(0);
  const [done, setDone] = useState(false);
  const cur = ITEMS[i];
  const reset = () => { setI(0); setCAct(null); setMisses(0); setDone(false); };
  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Loading desk</button>
        <button role="tab" aria-selected={mode === 'scale'} onClick={() => setMode('scale')}>Weight limits</button>
        <button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Challenge: {ITEMS.length} arrivals</button>
      </div>
      {mode === 'explore' && (
        <div class="stack">
          <p class="small muted">Pick an arrival, then try any decision. Watch where it ends up on the bus.</p>
          <div class="row" role="group" aria-label="Arrivals">
            {ITEMS.map((t, k) => <button key={t.id} class="btn sm" aria-label={`Arrival ${k + 1}: ${t.text}`} aria-pressed={pick === k} style={pick === k ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}}
              onClick={() => { setPick(k); setXAct(null); }}>{k + 1}</button>)}
          </div>
          <BusPlan it={ITEMS[pick]} act={xAct} reducedMotion={reducedMotion} />
          <Desk it={ITEMS[pick]} act={xAct} onAct={setXAct} locked={false} />
          {xAct && <Verdict it={ITEMS[pick]} act={xAct} />}
        </div>
      )}
      {mode === 'scale' && <Scale />}
      {mode === 'challenge' && (!done ? (
        <div class="stack">
          <span class="small muted num">Arrival {i + 1} of {ITEMS.length}</span>
          <BusPlan it={cur} act={cAct} reducedMotion={reducedMotion} />
          <Desk it={cur} act={cAct} locked onAct={(a) => { if (cAct) return; setCAct(a); const ok = a === cur.act; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); }} />
          {cAct && <Verdict it={cur} act={cAct} />}
          {cAct && <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={() => {
            if (i + 1 === ITEMS.length) { setDone(true); if (misses === 0) onChallenge?.(); } else { setI(i + 1); setCAct(null); }
          }}>{i + 1 === ITEMS.length ? 'Finish' : 'Next arrival'}</button>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status">
          <div class="verdict">{misses === 0 ? `${ITEMS.length} of ${ITEMS.length} right — stamp earned: Safe boarding` : `${ITEMS.length - misses} of ${ITEMS.length} right — need all ${ITEMS.length} for the stamp`}</div>
          <p class="small">Never list: Division 2.3 poison gas, liquid Class 6 poison, tear gas, irritating material; over 100 lb of any one class; over 500 lb in all. Aisle and exits clear, standees behind the line. <span class="plate">p. 4-3</span></p>
          <button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={reset}>Try again</button>
        </div>
      ))}
    </div>
  );
}
