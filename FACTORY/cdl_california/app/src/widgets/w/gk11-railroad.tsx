import { useState } from 'preact/hooks';
import type { ComponentChildren } from 'preact';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk11-railroad', title: 'Stop and cross the tracks', lesson: 'GK-11', anchor: /stopping safely at crossings/i,
  summary: 'Slide your truck to a stop before a crossing with no stop line, then cross one or two tracks. Then answer 5 crossing checks.',
  stamp: { id: 'clear-the-tracks', name: 'Clear the tracks', rule: 'Stop in the 15–50 ft zone and answer all 5 crossing checks right the first time.' },
};

const RAIL = 250, PX = 2; // nearest rail x; px per foot
const ftX = (ft: number) => RAIL - ft * PX;
type Crossed = null | 'ok' | 'stall';

export function stopVerdict(d: number): { ok: boolean; text: string } {
  if (d < 15) return { ok: false, text: `${d} ft is too close. With no white stop line, stop no closer than 15 ft from the nearest rail.` };
  if (d > 50) return { ok: false, text: `${d} ft is too far back. With no white stop line, stop no farther than 50 ft from the nearest rail.` };
  return { ok: true, text: `${d} ft — inside the 15–50 ft zone from the nearest rail.` };
}

function Truck({ front, stalled }: { front: number; stalled?: boolean }) {
  return (
    <g>
      <rect x={front - 64} y={84} width={48} height={24} rx={2} fill={stalled ? 'var(--red-soft)' : 'var(--surface-2)'} stroke={stalled ? 'var(--red)' : 'var(--ink)'} stroke-width="1.8" />
      <rect x={front - 14} y={85} width={14} height={22} rx={3} fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" />
    </g>
  );
}

function Scene({ d, double, crossed, flash }: { d: number; double: boolean; crossed: Crossed; flash: boolean }) {
  const rails = double ? [RAIL, RAIL + 12, RAIL + 30, RAIL + 42] : [RAIL, RAIL + 12];
  const front = crossed === 'ok' ? 372 : crossed === 'stall' ? RAIL + (double ? 40 : 26) : ftX(d);
  const v = stopVerdict(d);
  return (
    <svg viewBox="0 0 360 184" width="100%" style={{ display: 'block', maxWidth: '460px', marginInline: 'auto' }} role="img" aria-label={`Top view of a railroad crossing with ${double ? 'two tracks' : 'one track'}. Stop zone 15 to 50 feet from the nearest rail. ${crossed === 'stall' ? 'The truck is stalled on the tracks.' : crossed === 'ok' ? 'The truck has cleared the tracks.' : `Truck front is ${d} feet from the nearest rail.`}`}>
      <rect width="360" height="184" fill="var(--accent-soft)" />
      <rect y="72" width="360" height="48" fill="var(--surface)" /><line x1="0" x2="360" y1="72" y2="72" stroke="var(--ink-2)" stroke-width="2" /><line x1="0" x2="360" y1="120" y2="120" stroke="var(--ink-2)" stroke-width="2" />
      <rect x={ftX(50)} y="72" width={35 * PX} height="48" fill="var(--ok)" opacity=".22" />
      <line x1={ftX(50)} x2={ftX(50)} y1="66" y2="126" stroke="var(--ok)" stroke-width="2" stroke-dasharray="4 3" /><line x1={ftX(15)} x2={ftX(15)} y1="66" y2="126" stroke="var(--ok)" stroke-width="2" stroke-dasharray="4 3" />
      <text x={ftX(32.5)} y="61" text-anchor="middle" font-size="14" font-weight="700" fill="var(--ink)">stop zone</text>
      <text x={ftX(50)} y="137" text-anchor="middle" font-size="14" fill="var(--ink)">50 ft</text><text x={ftX(15)} y="137" text-anchor="middle" font-size="14" fill="var(--ink)">15 ft</text>
      {[...Array(13)].map((_, i) => <rect x={RAIL - 5} y={i * 15 - 2} width={22} height={5} fill="var(--ink-2)" opacity=".6" />)}
      {double && [...Array(13)].map((_, i) => <rect x={RAIL + 25} y={i * 15 - 2} width={22} height={5} fill="var(--ink-2)" opacity=".6" />)}
      {rails.map((x) => <line x1={x} x2={x} y1="0" y2="184" stroke="var(--ink)" stroke-width="2.5" />)}
      <text x={RAIL - 5} y="178" text-anchor="end" font-size="14" fill="var(--ink)">nearest rail →</text>
      <g transform="translate(222 22)" aria-hidden="true">
        <line x1="0" y1="0" x2="0" y2="44" stroke="var(--ink)" stroke-width="2" />
        <line x1="-13" y1="-8" x2="13" y2="8" stroke="var(--ink)" stroke-width="7" /><line x1="-13" y1="-8" x2="13" y2="8" stroke="var(--surface)" stroke-width="4.5" />
        <line x1="-13" y1="8" x2="13" y2="-8" stroke="var(--ink)" stroke-width="7" /><line x1="-13" y1="8" x2="13" y2="-8" stroke="var(--surface)" stroke-width="4.5" />
        {double && <g><rect x="-11" y="11" width="22" height="18" fill="var(--surface)" stroke="var(--ink)" /><text x="0" y="25" text-anchor="middle" font-size="14" font-weight="700" fill="var(--ink)">2</text></g>}
      </g>
      {crossed === 'stall' && <g><rect x={RAIL - 4} y="0" width={double ? 50 : 20} height="30" rx="3" fill="var(--red)" /><text x={RAIL + (double ? 52 : 22)} y="20" font-size="14" font-weight="700" fill="var(--red)">↓ train</text></g>}
      <Truck front={front} stalled={crossed === 'stall'} />
      {flash && crossed === null && [[front - 64, 84], [front - 64, 108], [front, 85], [front, 107]].map(([x, y]) => <circle cx={x} cy={y} r="3.5" fill="var(--amber)" stroke="var(--ink)" stroke-width=".8" />)}
      {crossed === null && <text x={Math.max(34, front - 32)} y="156" text-anchor="middle" font-size="14" font-weight="700" fill={v.ok ? 'var(--ok)' : 'var(--red)'}>{v.ok ? '✓' : '✕'} {d} ft</text>}
    </svg>
  );
}

interface Q { q: string; opts: string[]; a: number; why: ComponentChildren }
const QS: Q[] = [
  { q: 'Single track: how long does a typical tractor-trailer need to clear it?', opts: ['About 5 seconds', 'At least 14 seconds', 'About 10 seconds'], a: 1,
    why: <>A typical tractor-trailer takes at least 14 seconds to clear a single track. Be sure you can get all the way across before you start. <span class="plate">p. 2-36</span></> },
  { q: 'Double track: how long does a typical tractor-trailer need to clear it?', opts: ['About 14 seconds', 'More than 15 seconds', 'About 10 seconds'], a: 1,
    why: <>More than 15 seconds for a double track (at least 14 for a single). The two numbers are close — don’t swap them. <span class="plate">p. 2-36</span></> },
  { q: 'Halfway across the tracks the engine is lugging. What do you do?', opts: ['Downshift for more power', 'Stay in the gear you are in — don’t shift on the tracks', 'Upshift to clear faster'], a: 1,
    why: <>Do not shift gears while crossing railroad tracks. A missed shift could leave you stalled on the tracks. <span class="plate">p. 2-36</span></> },
  { q: 'Your load requires you to stop at the crossing. As you stop, you should:', opts: ['Check traffic behind, stop gradually, turn on 4-way flashers', 'Stop quickly so no one tries to pass', 'Stop with your front bumper at the first rail to see better'], a: 0,
    why: <>Check traffic behind you and stop gradually (use a pullout lane if there is one), and turn on your 4-way emergency flashers. <span class="plate">p. 2-36</span></> },
];

export default function Railroad({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  const [d, setD] = useState(70);
  const [double, setDouble] = useState(false);
  const [flash, setFlash] = useState(false);
  const [shift, setShift] = useState(false);
  const [crossed, setCrossed] = useState<Crossed>(null);
  const [i, setI] = useState(0); // 0 = stop task, 1..4 = questions, 5 = done
  const [done, setDone] = useState<boolean | null>(null);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const v = stopVerdict(d);
  const answer = (ok: boolean) => { if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const startChallenge = () => { setMode('challenge'); setI(0); setD(90); setDone(null); setPick(null); setMisses(0); setCrossed(null); setDouble(false); setFlash(false); };
  const slider = (lock: boolean) => (
    <div class="field">
      <label for="rr-d">Stop point: front bumper <span class="num">{d}</span> ft from the nearest rail</label>
      <input id="rr-d" type="range" min={0} max={100} step={1} value={d} disabled={lock} onInput={(e) => { setD(+(e.target as HTMLInputElement).value); setCrossed(null); }} style={{ width: '100%', accentColor: 'var(--accent)' }} />
    </div>
  );
  const q = QS[i - 1];
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore</button><button role="tab" aria-selected={mode === 'challenge'} onClick={startChallenge}>Challenge: 5 checks</button></div>
      {mode === 'explore' && (
        <div class="stack">
          <div role="group" aria-label="Number of tracks" style={{ display: 'grid', gridTemplateColumns: 'repeat(2, minmax(0, 1fr))', gap: '6px' }}>
            {[false, true].map((dbl) => <button class="btn sm" aria-pressed={double === dbl} onClick={() => { setDouble(dbl); setCrossed(null); }} style={double === dbl ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}}>{dbl ? 'Double track' : 'Single track'}</button>)}
          </div>
          <Scene d={d} double={double} crossed={crossed} flash={flash} />
          {slider(false)}
          <div class={`feedback ${v.ok ? 'good' : 'bad'}`} role="status" aria-live="polite"><div class="verdict">{v.ok ? 'Good stop point' : 'Move your stop point'}</div><p class="small" style={{ margin: 0 }}>{v.text} <span class="plate">p. 2-35</span></p></div>
          <div class="row">
            <label class="toggle" style={{ minHeight: '44px' }}><input type="checkbox" checked={flash} onChange={(e) => setFlash((e.target as HTMLInputElement).checked)} />4-way flashers on while stopping</label>
            <label class="toggle" style={{ minHeight: '44px' }}><input type="checkbox" checked={shift} onChange={(e) => { setShift((e.target as HTMLInputElement).checked); setCrossed(null); }} />Shift gears on the tracks</label>
          </div>
          {!flash && <p class="small muted" style={{ margin: 0 }}>When you stop: check traffic behind, stop gradually (pullout lane if there is one), and turn on your 4-way flashers. <span class="plate">p. 2-36</span></p>}
          <div class="row"><button class="btn primary sm" onClick={() => setCrossed(shift ? 'stall' : 'ok')}>Cross the {double ? 'tracks' : 'track'}</button>{crossed && <button class="btn sm" onClick={() => setCrossed(null)}>Back to the stop</button>}</div>
          {crossed === 'stall' && <div class="feedback bad" role="status"><div class="verdict">Stalled on the tracks</div><p class="small" style={{ margin: 0 }}>The shift was missed and the truck stopped on the rails. <strong>Don’t shift gears while crossing.</strong> If you are ever stuck: get out and well away from the tracks, then call 9-1-1 with the crossing’s DOT number. <span class="plate">p. 2-36</span></p></div>}
          {crossed === 'ok' && <div class="feedback good" role="status"><div class="verdict">Cleared — no shifting</div><p class="small" style={{ margin: 0 }}>A typical tractor-trailer needs <strong>{double ? 'more than 15 seconds' : 'at least 14 seconds'}</strong> to clear a {double ? 'double' : 'single'} track. Before you start, be sure you can get all the way across — never let traffic trap you on the tracks. <span class="plate">p. 2-36</span></p></div>}
        </div>
      )}
      {mode === 'challenge' && i === 0 && (
        <div class="stack">
          <span class="small muted num">Check 1 of 5</span>
          <strong>No white stop line is painted, and your cargo requires a stop. Slide to where your front bumper stops, then press “Stop here”.</strong>
          <Scene d={d} double={false} crossed={null} flash={false} />
          {slider(done !== null)}
          {done === null && <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={() => { setDone(v.ok); answer(v.ok); }}>Stop here</button>}
          {done !== null && <div class={`feedback ${done ? 'good' : 'bad'}`} role="status"><div class="verdict">{done ? 'Legal stop' : 'Outside the stop zone'}</div>
            <p class="small" style={{ margin: 0 }}>{v.text} Measure from the nearest rail of the nearest track. <span class="plate">p. 2-35</span></p>
            <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={() => setI(1)}>Next</button></div>}
        </div>
      )}
      {mode === 'challenge' && q && (
        <div class="stack">
          <span class="small muted num">Check {i + 1} of 5</span>
          <strong>{q.q}</strong>
          <div class="stack" role="group" aria-label="Answers" style={{ gap: '6px' }}>{q.opts.map((o, j) => (
            <button class={`btn sm ${pick !== null && j === q.a ? 'primary' : ''}`} disabled={pick !== null} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick !== null && j === q.a ? { opacity: 1 } : {}), ...(pick === j && j !== q.a ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}) }}
              onClick={() => { setPick(j); answer(j === q.a); }}>{pick !== null && j === q.a ? '✓ ' : pick === j ? '✕ ' : ''}{o}</button>
          ))}</div>
          {pick !== null && <div class={`feedback ${pick === q.a ? 'good' : 'bad'}`} role="status"><div class="verdict">{pick === q.a ? 'Right' : `Answer: ${q.opts[q.a]}`}</div><p class="small" style={{ margin: 0 }}>{q.why}</p>
            <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={() => { if (i === QS.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i === QS.length ? 'Finish' : 'Next'}</button></div>}
        </div>
      )}
      {mode === 'challenge' && i > QS.length && (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 5 right — stamp earned' : `${5 - misses} of 5 right`}</div>
          <p class="small" style={{ margin: 0 }}>No stop line: 15–50 ft from the nearest rail. Single track ≥ 14 s, double track &gt; 15 s. No shifting on the tracks.</p>
          <button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={startChallenge}>Try again</button></div>
      )}
    </div>
  );
}
