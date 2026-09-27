import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk13-emergency-scene', title: 'First on scene: crash, fire, alcohol', lesson: 'GK-13', anchor: /^accident procedures$/i,
  summary: 'Put the 5 accident steps in order, match extinguishers to fires, fight a truck fire, and test the .04 alcohol rule. Then an 8-question challenge.',
  stamp: { id: 'first-on-scene', name: 'First on scene', rule: 'Answer all 8 crash, fire and alcohol questions with no mistakes.' },
};

const P = ({ p }: { p: string }) => <span class="plate">p. {p}</span>;
const T = { 'font-size': 13, fill: 'var(--ink)' } as const;

// ---- Part A: accident steps (p. 2-44) ----
export const STEPS = [
  { id: 'protect', name: 'Protect the area', detail: 'First job: stop a second crash at the same spot. Move your truck to the side of the road (or, if you stopped to help, park away from the wreck). Turn on 4-way flashers. Place reflective triangles so other drivers see them in time (where to place them: GK-06, section 2.5).' },
  { id: 'notify', name: 'Notify authorities', detail: 'Have a phone or CB? Call before you get out of the truck. No phone? Protect the scene first, then call or send someone. Know your exact location.' },
  { id: 'care', name: 'Care for the injured', detail: 'Let trained people work. Otherwise: don’t move a badly injured person unless fire or passing traffic makes it necessary; direct pressure on heavy bleeding; keep them warm.' },
  { id: 'collect', name: 'Collect information', detail: 'Drivers’ names, addresses, DL numbers; plates and vehicle types; owners; damage; injured and witnesses; officer’s name, badge number and agency; exact location; direction of travel.' },
  { id: 'report', name: 'Report the accident', detail: 'File the required accident report. [CA] The DMV SR 1 report is covered in GK-03.' },
] as const;
const WRONG: Record<string, string> = {
  protect: 'Nothing warns traffic yet. A car comes over the rise onto the wreck — a second crash at the same spot, maybe hitting you and the injured.',
  notify: 'Help isn’t called yet. With a phone or CB, the handbook says call before you even get out of the truck.',
  care: 'Paperwork can wait. People who are hurt come before collecting information.',
  collect: 'You can’t report yet — first collect the information the report needs.',
};

export function CrashVis({ done, second }: { done: number; second?: boolean }) {
  return (
    <svg viewBox="0 0 360 130" width="100%" style={{ maxWidth: '560px' }} role="img" aria-label={`Crash scene. Steps done: ${done} of 5.${second ? ' A car hits the unprotected wreck: a second crash.' : ''}${done >= 1 ? ' Flashers on and triangles placed.' : ''}`}>
      <rect x="0" y="0" width="360" height="130" fill="var(--surface)" />
      <rect x="0" y="30" width="360" height="70" fill="var(--ink-2)" fill-opacity="0.28" />
      <line x1="0" y1="65" x2="360" y2="65" stroke="var(--amber)" stroke-width="2" stroke-dasharray="10 6" />
      <rect x="0" y="100" width="360" height="12" fill="var(--surface-2)" />
      <g transform="rotate(20 250 76)"><rect x="232" y="68" width="36" height="18" rx="3" fill="var(--surface-2)" stroke="var(--ink)" stroke-width="1.4" /></g>
      <path d="M 262 60 l 5 -8 l 3 7 l 6 -4 l -2 8" fill="none" stroke="var(--red)" stroke-width="2" />
      <rect x="276" y="98" width="50" height="16" rx="2" fill="var(--accent)" stroke="var(--ink)" /><rect x="328" y="99" width="14" height="14" rx="2" fill="var(--accent)" stroke="var(--ink)" />
      {done >= 1 && <>{[[274, 100], [274, 112], [344, 100], [344, 112]].map(([x, y]) => <circle cx={x} cy={y} r="3" fill="var(--amber)" stroke="var(--ink)" stroke-width="0.6" />)}
        {[200, 130, 40].map((x) => <path d={`M ${x} 70 l 7 12 l -14 0 Z`} fill="var(--red)" stroke="var(--ink)" stroke-width="0.8" />)}<text x="120" y="24" {...T}>▲ triangles + flashers</text></>}
      {done >= 2 && <text x="250" y="124" {...T}>✆ help called</text>}
      {done >= 3 && <><rect x="300" y="36" width="16" height="16" rx="3" fill="var(--surface)" stroke="var(--red)" /><path d="M 308 39 v10 M 303 44 h10" stroke="var(--red)" stroke-width="3" /></>}
      {done >= 4 && <text x="4" y="124" {...T}>✎ info collected</text>}
      {done >= 5 && <text x="120" y="124" {...T}>✓ report filed</text>}
      {second && <><rect x="150" y="68" width="32" height="16" rx="3" fill="var(--surface)" stroke="var(--ink)" /><path d="M 186 76 L 226 76" stroke="var(--red)" stroke-width="3" /><circle cx="228" cy="76" r="12" fill="var(--red)" /><text x="228" y="81" text-anchor="middle" font-size="14" font-weight="700" fill="var(--surface)">!</text><text x="10" y="24" font-size="13" font-weight="700" fill="var(--red)">SECOND CRASH</text></>}
    </svg>
  );
}

function PartA() {
  const [done, setDone] = useState(0); const [bad, setBad] = useState<string | null>(null);
  const order = ['collect', 'care', 'report', 'protect', 'notify'];
  return (
    <div class="stack">
      <p class="small">You were in a crash but are not badly hurt. Tap the step that comes <strong>next</strong>. <P p="2-44" /></p>
      <CrashVis done={done} second={bad === 'protect'} />
      <div class="row" role="group" aria-label="Accident steps">{order.map((id) => { const i = STEPS.findIndex((s) => s.id === id); const s = STEPS[i]; const placed = i < done; return (
        <button class={`btn sm ${placed ? 'primary' : ''}`} disabled={placed || done === 5} onClick={() => { if (i === done) { setDone(done + 1); setBad(null); } else setBad(STEPS[done].id); }}>{placed ? `${i + 1}. ` : ''}{s.name}</button>); })}</div>
      {bad && <div class="feedback bad" role="status"><div class="verdict">Not yet — {STEPS[STEPS.findIndex((s) => s.id === bad)].name} comes first</div><p class="small">{WRONG[bad]} <P p="2-44" /></p></div>}
      {done > 0 && <ol class="small">{STEPS.slice(0, done).map((s) => <li><strong>{s.name}.</strong> {s.detail}</li>)}</ol>}
      {done === 5 && <button class="btn sm" onClick={() => { setDone(0); setBad(null); }}>Reset scene</button>}
    </div>
  );
}

// ---- Part B: fire (p. 2-44 to 2-45) ----
type Fire = 'elec' | 'liquid' | 'wood'; type Agent = 'BC' | 'ABC' | 'water';
export const MATCH: Record<Fire, Record<Agent, [boolean, string]>> = {
  elec: { BC: [true, 'B:C is made for electrical fires and burning liquids.'], ABC: [true, 'A:B:C covers electrical, liquids, and wood, paper, cloth.'], water: [false, 'Never water on an electrical fire — you can get a shock.'] },
  liquid: { BC: [true, 'B:C is made for burning liquids and electrical fires.'], ABC: [true, 'A:B:C covers burning liquids too.'], water: [false, 'Never water on a gasoline fire — it spreads the flames.'] },
  wood: { BC: [false, 'B:C is not made for wood, paper or cloth. The fire keeps burning. Use A:B:C or water.'], ABC: [true, 'A:B:C adds wood, paper and cloth.'], water: [true, 'Water is OK on wood, paper or cloth.'] },
};
const FIRE_L: Record<Fire, string> = { elec: 'Electrical wiring', liquid: 'Burning fuel or oil', wood: 'Wood pallets, paper, cloth' };
const AGENT_L: Record<Agent, string> = { BC: 'B:C extinguisher', ABC: 'A:B:C extinguisher', water: 'Water' };
export const DRILL = [
  { q: 'Where do you stop?', good: 'Open area, away from buildings, trees, brush, vehicles', bad: 'The nearest service station', why: 'Never pull into a service station — it is full of fuel that can catch fire and explode.' },
  { q: 'Engine fire:', good: 'Engine off, hood closed; spray through louvers, radiator or from under', bad: 'Open the hood to reach the fire', why: 'Opening the hood feeds the fire fresh air.' },
  { q: 'Cargo fire in a van trailer:', good: 'Keep the doors shut', bad: 'Open the doors to find it', why: 'Opening the doors gives the fire oxygen and it can burn very fast — above all with HazMat.' },
  { q: 'Using the extinguisher:', good: 'Far away, upwind, aim at the base', bad: 'Get close and aim at the flames', why: 'Keep your distance, let the wind carry the spray, and hit what is actually burning.' },
];
export function FireVis({ size, station }: { size: number; station?: boolean }) {
  const f = 0.7 + size * 0.28;
  return (
    <svg viewBox="0 0 300 136" width="100%" style={{ maxWidth: '460px' }} role="img" aria-label={`Truck fire, ${size === 0 ? 'small and under control' : size >= 3 ? 'large and spreading' : 'growing'}.${station ? ' Stopped beside fuel pumps: explosion risk.' : ''}`}>
      <rect x="0" y="0" width="300" height="136" fill="var(--surface)" /><g transform="translate(0 26)"><rect x="0" y="92" width="300" height="18" fill="var(--surface-2)" />
      <rect x="40" y="44" width="150" height="46" rx="2" fill="var(--surface-2)" stroke="var(--ink)" /><rect x="192" y="54" width="40" height="36" rx="3" fill="var(--accent)" stroke="var(--ink)" />
      {[70, 170, 216].map((x) => <circle cx={x} cy="92" r="9" fill="var(--ink-2)" stroke="var(--ink)" />)}
      <g transform={`translate(212 54) scale(${f}) translate(-212 -54)`}><path d="M 200 54 C 196 38 206 32 206 20 C 214 30 222 30 218 12 C 232 26 230 42 224 54 Z" fill="var(--amber)" stroke="var(--red)" stroke-width="2" /></g>
      {station && <><rect x="254" y="50" width="18" height="40" fill="var(--blue-soft)" stroke="var(--ink)" /><text x="263" y="44" text-anchor="middle" font-size="13" font-weight="700" fill="var(--red)">FUEL</text></>}</g>
      <text x="8" y="16" font-size="13" font-weight="700" fill={size === 0 ? 'var(--ok)' : 'var(--red)'}>{size === 0 ? '✓ Kept small' : `✗ Fire growing (${size} mistake${size > 1 ? 's' : ''})`}</text>
    </svg>
  );
}
export function MatchVis({ fire, agent, ok }: { fire: Fire; agent: Agent; ok: boolean }) {
  const res = ok ? 'Fire out' : agent === 'water' ? (fire === 'elec' ? 'Shock!' : 'Flames spread') : 'Still burning';
  return (
    <svg viewBox="0 0 300 64" width="100%" style={{ maxWidth: '420px' }} role="img" aria-label={`${AGENT_L[agent]} on ${FIRE_L[fire]}: ${res}.`}>
      <rect x="0" y="0" width="300" height="64" fill="var(--surface)" />
      <rect x="14" y="16" width="16" height="36" rx="5" fill="var(--red)" stroke="var(--ink)" /><path d="M 30 22 L 52 18" stroke="var(--ink)" stroke-width="3" />
      <path d="M 56 20 Q 80 26 96 34" fill="none" stroke={agent === 'water' ? 'var(--blue)' : 'var(--ink-2)'} stroke-width="3" stroke-dasharray="3 3" />
      {!ok && agent === 'water' && fire !== 'elec' ? [0, 1, 2].map((i) => <path d={`M ${100 + i * 26} 56 C ${96 + i * 26} 44 ${108 + i * 26} 40 ${106 + i * 26} 28 C ${116 + i * 26} 40 ${120 + i * 26} 46 ${116 + i * 26} 56 Z`} fill="var(--amber)" stroke="var(--red)" />)
        : <path d={ok ? 'M 100 56 L 124 56 L 118 52 L 106 52 Z' : 'M 100 56 C 96 40 110 36 108 18 C 122 34 128 42 122 56 Z'} fill={ok ? 'var(--ink-2)' : 'var(--amber)'} stroke={ok ? 'var(--ink)' : 'var(--red)'} />}
      {!ok && agent === 'water' && fire === 'elec' && <path d="M 150 10 L 140 32 L 150 32 L 138 56" fill="none" stroke="var(--amber)" stroke-width="3" />}
      <text x="296" y="38" text-anchor="end" font-size="13" font-weight="700" fill={ok ? 'var(--ok)' : 'var(--red)'}>{ok ? '✓ ' : '✗ '}{res}</text>
    </svg>
  );
}
function PartB() {
  const [fire, setFire] = useState<Fire>('elec'); const [agent, setAgent] = useState<Agent | null>(null);
  const [ch, setCh] = useState<(boolean | null)[]>([null, null, null, null]);
  const wrong = ch.filter((c) => c === false).length; const m = agent ? MATCH[fire][agent] : null;
  return (
    <div class="stack">
      <strong>Match the extinguisher to the fire</strong>
      <div class="row" role="group" aria-label="What is burning">{(Object.keys(FIRE_L) as Fire[]).map((k) => <button class="btn sm" aria-pressed={fire === k} style={fire === k ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}} onClick={() => { setFire(k); setAgent(null); }}>{FIRE_L[k]}</button>)}</div>
      <div class="row" role="group" aria-label="What you use">{(Object.keys(AGENT_L) as Agent[]).map((k) => <button class="btn sm" aria-pressed={agent === k} style={agent === k ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}} onClick={() => setAgent(k)}>{AGENT_L[k]}</button>)}</div>
      {m && agent && <div class={`feedback ${m[0] ? 'good' : 'bad'}`} role="status"><div class="verdict">{m[0] ? '✓ Works' : '✗ Wrong choice'}</div><MatchVis fire={fire} agent={agent} ok={m[0]} /><p class="small">{m[1]} <P p="2-45" /></p></div>}
      <p class="small muted">Burning tire: it must be cooled — it may take a lot of water. Not sure which extinguisher, above all with HazMat? Wait for firefighters. <P p="2-45" /></p>
      <strong>Truck fire drill</strong>
      <FireVis size={wrong} station={ch[0] === false} />
      {DRILL.map((d, i) => (
        <div class="stack" style={{ gap: '6px' }}><span class="small"><strong>{d.q}</strong></span>
          <div class="row" role="group" aria-label={d.q}>{(i % 2 ? [false, true] : [true, false]).map((g) => { const lab = g ? d.good : d.bad; return (
            <button class="btn sm" aria-pressed={ch[i] === g} style={{ textAlign: 'left', ...(ch[i] === g ? { borderColor: g ? 'var(--ok)' : 'var(--red)', background: g ? 'var(--ok-soft)' : 'var(--red-soft)' } : {}) }} onClick={() => { const n = [...ch]; n[i] = g; setCh(n); }}>{ch[i] === g ? (g ? '✓ ' : '✗ ') : ''}{lab}</button>); })}</div>
          {ch[i] === false && <p class="small" role="status">{d.why} <P p="2-45" /></p>}
        </div>))}
      <p class="small muted">Keep going until the burning material has cooled — no smoke or flame doesn’t mean it can’t flare up again. <P p="2-45" /></p>
    </div>
  );
}

// ---- Part C: alcohol (p. 2-46 to 2-47; out-of-service rule p. 1-14) ----
export function bacRule(b: number): { ok: boolean; h: string; t: string; p: string } {
  if (b <= 0) return { ok: true, h: 'BAC 0 — the only safe driving limit', t: 'And no alcohol at all within 4 hours before going on duty, and never while on duty.', p: '2-46' };
  if (b < 0.04) return { ok: false, h: 'Out of service for 24 hours', t: 'Any detectable alcohol below .04 still puts you out of service for 24 hours. Below .04 is NOT safe or legal to drive.', p: '1-14' };
  return { ok: false, h: 'Illegal to drive a CMV', t: `.04 or more: immediate Admin Per Se license action (CVC §13353.2), and a possible DUI conviction (CVC §23152(d)).${b >= 0.08 ? ' The chart calls .08 the “drunk driving limit” — for a CMV the limit is .04.' : ''}`, p: '2-47' };
}
export function BacVis({ b }: { b: number }) {
  const x = (v: number) => 20 + v * 2800;
  return (
    <svg viewBox="0 0 320 64" width="100%" style={{ maxWidth: '460px' }} role="img" aria-label={`BAC gauge at ${b.toFixed(2)}. The CMV limit line is at .04.`}>
      <rect x="0" y="0" width="320" height="64" fill="var(--surface)" />
      <rect x="20" y="20" width="280" height="16" rx="8" fill="var(--surface-2)" stroke="var(--ink-2)" />
      <rect x="20" y="20" width={Math.max(0, x(b) - 20)} height="16" rx="8" fill={b === 0 ? 'var(--ok)' : b < 0.04 ? 'var(--amber)' : 'var(--red)'} />
      <line x1={x(0.04)} y1="12" x2={x(0.04)} y2="44" stroke="var(--red)" stroke-width="2" /><text x={x(0.04)} y="58" text-anchor="middle" {...T}>.04 CMV</text>
      <line x1={x(0.08)} y1="16" x2={x(0.08)} y2="40" stroke="var(--ink-2)" stroke-dasharray="3 2" /><text x={x(0.08)} y="58" text-anchor="middle" {...T}>.08</text>
      <text x="20" y="12" {...T}>0</text><text x={Math.min(x(b), 280)} y="12" {...T} font-weight="700">{b.toFixed(2)}</text>
    </svg>
  );
}
function PartC() {
  const [b, setB] = useState(0.02); const [s, setS] = useState<string | null>(null); const r = bacRule(b);
  return (
    <div class="stack">
      <div class="field"><label for="g13-bac">Blood alcohol (BAC): <span class="num">{b.toFixed(2)}</span></label><input id="g13-bac" type="range" min={0} max={0.1} step={0.01} value={b} onInput={(e) => setB(Math.round(+(e.target as HTMLInputElement).value * 100) / 100)} /></div>
      <BacVis b={b} />
      <div class={`feedback ${r.ok ? 'good' : 'bad'}`} role="status" aria-live="polite"><div class="verdict">{r.h}</div><p class="small">{r.t} <P p={r.p} /></p></div>
      <strong class="small">What lowers your BAC?</strong>
      <div class="row" role="group" aria-label="Ways to sober up">{['Black coffee', 'Cold shower', 'Fresh air', 'Time'].map((k) => <button class="btn sm" aria-pressed={s === k} onClick={() => setS(k)}>{k}</button>)}</div>
      {s && <div class={`feedback ${s === 'Time' ? 'good' : 'bad'}`} role="status"><p class="small">{s === 'Time' ? '✓ Only time. ' : `✗ ${s} does nothing. `}The liver handles about 1/3 ounce of alcohol per hour, and that rate never changes. <P p="2-47" /></p></div>}
      <p class="small muted">One drink = 12 oz beer = 5 oz wine = 1½ oz 80-proof liquor — same alcohol. <P p="2-47" /></p>
    </div>
  );
}

// ---- Challenge ----
type Vis = 'crash' | 'fire' | 'bac';
interface Q { q: string; opts: string[]; a: number; why: string; bad: string; p: string; vis: Vis }
export const QS: Q[] = [
  { vis: 'crash', q: 'You are in a crash and not badly hurt. What is your first job?', opts: ['Help the injured', 'Prevent a second crash at the same spot', 'Collect witness names'], a: 1, why: 'Protect the area first: move the truck aside, 4-way flashers, reflective triangles.', bad: 'With no warning out, traffic runs into the wreck — a second crash.', p: '2-44' },
  { vis: 'crash', q: 'You have a CB radio in the cab. When do you call for help?', opts: ['Before you get out of the truck', 'After you collect the information', 'Only after the triangles are out'], a: 0, why: 'With a phone or CB, call before getting out. No phone? Protect the scene first, then call.', bad: 'Help is delayed while you do other steps.', p: '2-44' },
  { vis: 'crash', q: 'Someone is badly injured and lying in the road. There is no fire and traffic is stopped. You…', opts: ['Drag them to the shoulder', 'Don’t move them; direct pressure on bleeding, keep them warm', 'Give them water'], a: 1, why: 'Move a badly injured person only if fire or passing traffic makes it necessary.', bad: 'Moving them can make the injuries worse.', p: '2-44' },
  { vis: 'fire', q: 'A B:C extinguisher is made for:', opts: ['Wood, paper and cloth', 'Electrical fires and burning liquids', 'Any fire at all'], a: 1, why: 'B:C = electrical fires and burning liquids. A:B:C adds wood, paper and cloth.', bad: 'On burning wood or cloth a B:C unit is the wrong tool — the fire keeps going.', p: '2-45' },
  { vis: 'fire', q: 'Smoke is coming from under the hood. You stop in an open area. Next?', opts: ['Open the hood to see the fire', 'Engine off; keep the hood closed; spray through louvers, radiator or from under', 'Pull into the next service station'], a: 1, why: 'Opening the hood feeds the fire air. Never pull into a service station.', bad: 'Fresh air (or a station full of fuel) makes the fire bigger.', p: '2-45' },
  { vis: 'fire', q: 'How do you use the extinguisher?', opts: ['Close in, aim at the top of the flames', 'Downwind so smoke blows past you', 'As far away as possible, upwind, aim at the base'], a: 2, why: 'Distance, wind at your back, and hit the source — what is actually burning.', bad: 'Spray at the flames misses the fuel; the fire keeps burning.', p: '2-45' },
  { vis: 'bac', q: 'A roadside test shows BAC .02 while you drive a CMV. What happens?', opts: ['Nothing — you are under .04', 'Out of service for 24 hours', 'Only a warning'], a: 1, why: 'Any detectable alcohol under .04 = out of service 24 hours. Under .04 is not safe or legal.', bad: 'You keep driving with alcohol in your blood — not allowed.', p: '1-14' },
  { vis: 'bac', q: 'You drank last night and must drive at 6 a.m. What lowers your BAC?', opts: ['Only time', 'Black coffee', 'A cold shower'], a: 0, why: 'The liver works at a fixed rate (about 1/3 oz per hour). Coffee and showers do nothing.', bad: 'Your BAC is unchanged — you would drive impaired.', p: '2-47' },
];

export default function EmergencyScene({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [tab, setTab] = useState<'a' | 'b' | 'c' | 'x'>('a');
  const [i, setI] = useState(0); const [pick, setPick] = useState<number | null>(null); const [misses, setMisses] = useState(0);
  const q = QS[i];
  const reset = () => { setI(0); setPick(null); setMisses(0); };
  const answer = (k: number) => { if (pick !== null) return; setPick(k); const ok = k === q.a; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const next = () => { if (i + 1 === QS.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); };
  const ok = pick !== null && pick === q?.a;
  return (
    <div class="stack">
      <div class="tabs" role="tablist">{([['a', 'Crash'], ['b', 'Fire'], ['c', 'Alcohol'], ['x', 'Challenge']] as const).map(([k, l]) => <button role="tab" aria-selected={tab === k} onClick={() => { setTab(k); if (k === 'x') reset(); }}>{l}</button>)}</div>
      {tab === 'a' && <PartA />}{tab === 'b' && <PartB />}{tab === 'c' && <PartC />}
      {tab === 'x' && (q ? (
        <div class="stack">
          <span class="small muted num">Question {i + 1} of {QS.length} · {q.vis === 'crash' ? 'Crash' : q.vis === 'fire' ? 'Fire' : 'Alcohol'}</span>
          <strong>{q.q}</strong>
          <div class="stack" role="group" aria-label="Answers">{q.opts.map((o, k) => (
            <button class={`btn sm ${pick !== null && k === q.a ? 'primary' : ''}`} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick === k && k !== q.a ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }} disabled={pick !== null} onClick={() => answer(k)}>{pick !== null && (k === q.a ? '✓ ' : k === pick ? '✗ ' : '')}{o}</button>))}</div>
          {pick !== null && <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
            <div class="verdict">{ok ? 'Right' : 'Not quite'}</div>
            {q.vis === 'crash' && <CrashVis done={ok ? (i === 0 ? 1 : i === 1 ? 2 : 3) : 0} second={!ok && i === 0} />}
            {q.vis === 'fire' && <FireVis size={ok ? 0 : 3} station={!ok && pick === 2 && i === 4} />}
            {q.vis === 'bac' && <BacVis b={i === 6 ? 0.02 : 0.04} />}
            <p class="small">{!ok && <><strong>What happens:</strong> {q.bad} </>}{q.why} <P p={q.p} /></p>
            <button class="btn primary sm" onClick={next}>{i + 1 === QS.length ? 'Finish' : 'Next'}</button></div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 8 right — stamp earned' : `${QS.length - misses} of ${QS.length} right`}</div>
          <button class="btn sm" onClick={reset}>Try again</button></div>
      ))}
    </div>
  );
}
