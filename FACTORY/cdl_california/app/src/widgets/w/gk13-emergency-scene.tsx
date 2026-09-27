import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk13-emergency-scene', title: 'First on scene: crash, fire, alcohol', lesson: 'GK-13', anchor: /^accident procedures$/i,
  summary: 'Walk a crash scene through the 5 accident steps, match extinguishers to fires, fight a truck fire, and test the .04 alcohol rule. Then order the steps and answer 8 questions.',
  stamp: { id: 'first-on-scene', name: 'First on scene', rule: 'Put the 5 accident steps in order and answer all 8 crash, fire and alcohol questions with no mistakes.' },
};

const P = ({ p }: { p: string }) => <span class="plate">p. {p}</span>;
const T = { 'font-size': 13, fill: 'var(--ink)' } as const;
const cols = (n: number) => ({ display: 'grid', gridTemplateColumns: `repeat(${n}, minmax(0, 1fr))`, gap: '6px' });
const segOn = { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' };

// ---- Part A: accident steps (p. 2-44) ----
export const STEPS = [
  { id: 'protect', name: 'Protect the area', short: 'Protect', detail: 'First job: stop a second crash at the same spot. Move your truck to the side of the road (or, if you stopped to help, park away from the wreck). Turn on 4-way flashers. Place reflective triangles so other drivers see them in time (where to place them: GK-06, section 2.5).' },
  { id: 'notify', name: 'Notify authorities', short: 'Notify', detail: 'Have a phone or CB? Call for help before you get out of the truck. No phone? Protect the scene first, then phone the police or send someone. Know your exact location.' },
  { id: 'care', name: 'Care for the injured', short: 'Care', detail: 'A trained person is helping? Let them work. Otherwise: don’t move a badly injured person unless fire or passing traffic makes it necessary; direct pressure on heavy bleeding; keep them warm.' },
  { id: 'collect', name: 'Collect information', short: 'Collect', detail: 'Drivers’ names, addresses, DL numbers; plates and vehicle types; owners; damage; injured and witnesses; officer’s name, badge number and agency; exact location; direction of travel.' },
  { id: 'report', name: 'Report the accident', short: 'Report', detail: 'If you were in the accident, you must file an accident report, using the information you collected. [CA] The DMV SR 1 report is covered in GK-03.' },
] as const;
const WRONG: Record<string, string> = {
  protect: 'Nothing warns traffic yet. A driver comes up on the wreck — a second crash at the same spot, maybe hitting you and the injured.',
  notify: 'Help isn’t called yet. With a phone or CB, the handbook says call before you even get out of the truck.',
  care: 'Paperwork can wait. People who are hurt come before collecting information.',
  collect: 'You can’t report yet — first collect the information the report needs.',
  report: 'The information sits unused: if you were in the accident, you must file an accident report.',
};

/** One picture of the whole scene, in step order: step 1 on the road, steps 2–5 as cards left to right. `cur` is highlighted; steps before it
 *  are done; later steps are dimmed (or hidden when `hide` is set, for the ordering check). `skip` shows what goes wrong without the current step. */
export function CrashScene({ cur, hide, skip }: { cur: number; hide?: boolean; skip?: boolean }) {
  const st = (k: number) => (k < cur ? 'done' : k === cur ? 'cur' : 'todo');
  const show = (k: number) => !hide || k < cur;
  const lost = (k: number) => !!skip && k === cur;
  const edge = (k: number) => (lost(k) ? 'var(--red)' : st(k) === 'cur' ? 'var(--amber)' : st(k) === 'done' ? 'var(--ok)' : 'var(--line)');
  const Tag = ({ k, x, y }: { k: number; x: number; y: number }) => (
    <g><circle cx={x} cy={y} r="10" fill={st(k) === 'done' ? 'var(--ok)' : st(k) === 'cur' ? 'var(--accent)' : 'var(--surface-2)'} stroke="var(--ink)" stroke-width="1.2" />
      <text x={x} y={y + 5} text-anchor="middle" font-size="14" font-weight="700" fill={st(k) === 'todo' ? 'var(--ink)' : 'var(--surface)'}>{st(k) === 'done' ? '✓' : k + 1}</text></g>
  );
  const safe = show(0) && !lost(0);
  const done = STEPS.filter((_, k) => k < cur).map((x) => x.short).join(', ');
  /** Steps 2–5: one card each, left to right in order. */
  const Card = ({ k, lines, bad, children }: { k: number; lines: string[]; bad: string; children: preact.ComponentChildren }) => {
    const x = 4 + (k - 1) * 79;
    if (!show(k)) return <g><rect x={x} y="178" width="75" height="100" rx="8" fill="var(--surface-2)" stroke="var(--line)" stroke-dasharray="4 3" /><text x={x + 37.5} y="234" text-anchor="middle" font-size="14" font-weight="700" fill="var(--ink-2)">?</text></g>;
    return (
      <g>
        <rect x={x} y="178" width="75" height="100" rx="8" fill={lost(k) ? 'var(--red-soft)' : st(k) === 'cur' ? 'var(--amber-soft)' : st(k) === 'todo' ? 'var(--surface-2)' : 'var(--surface)'} stroke={edge(k)} stroke-width={st(k) === 'todo' ? 1.2 : 2.5} stroke-dasharray={lost(k) ? '5 3' : st(k) === 'todo' ? '4 3' : '0'} />
        <g transform={`translate(${x} 178)`}>{children}</g>
        {(lost(k) ? [bad] : lines).map((l, j) => <text x={x + 37.5} y={252 + j * 16} text-anchor="middle" font-size="13" font-weight={lost(k) ? 700 : 400} fill={lost(k) ? 'var(--red)' : st(k) === 'todo' ? 'var(--ink-2)' : 'var(--ink)'}>{l}</text>)}
        <Tag k={k} x={x + 13} y={191} />
      </g>
    );
  };
  return (
    <svg viewBox="0 0 320 282" width="100%" style={{ display: 'block', maxWidth: '400px', marginInline: 'auto' }} role="img"
      aria-label={`Crash scene with the 5 accident steps in order. ${done ? `Done: ${done}. ` : ''}${cur < 5 ? `Now: step ${cur + 1}, ${hide ? 'not yet chosen' : STEPS[cur].name}.` : 'All 5 steps done.'}${lost(0) ? ' Skipped: a car runs into the unprotected wreck — a second crash.' : ''}`}>
      <rect width="320" height="282" fill="var(--surface)" />
      {/* step rail */}
      <line x1="30" y1="17" x2="290" y2="17" stroke="var(--line)" stroke-width="3" />
      {STEPS.map((x, k) => { const cx = 30 + k * 65; const s = st(k); return (
        <g key={x.id}>
          {s === 'cur' && <circle cx={cx} cy="17" r="16" fill="none" stroke={skip ? 'var(--red)' : 'var(--amber)'} stroke-width="3" />}
          <circle cx={cx} cy="17" r="12" fill={s === 'done' ? 'var(--ok)' : s === 'cur' ? 'var(--accent)' : 'var(--surface-2)'} stroke="var(--ink)" stroke-width="1.2" />
          <text x={cx} y="22" text-anchor="middle" font-size="14" font-weight="700" fill={s === 'todo' ? 'var(--ink)' : 'var(--surface)'}>{s === 'done' ? '✓' : k + 1}</text>
          <text x={cx} y="48" text-anchor="middle" font-size="13" font-weight={s === 'cur' ? 700 : 400} fill={s === 'todo' ? 'var(--ink-2)' : 'var(--ink)'}>{show(k) ? x.short : '?'}</text>
        </g>); })}
      {/* road: traffic drives left → right in the lower lane; shoulder below */}
      <rect x="0" y="58" width="320" height="84" fill="var(--ink-2)" fill-opacity="0.28" />
      <line x1="0" y1="100" x2="320" y2="100" stroke="var(--amber)" stroke-width="2" stroke-dasharray="10 6" />
      <rect x="0" y="142" width="320" height="26" fill="var(--amber-soft)" />
      <text x="4" y="94" font-size="13" fill="var(--ink-2)">oncoming</text><text x="4" y="120" font-size="13" fill="var(--ink-2)">traffic →</text>
      <g transform="rotate(-18 206 120)"><rect x="186" y="109" width="40" height="22" rx="4" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.4" /></g>
      <path d="M 230 104 l 5 -8 l 3 7 l 6 -4 l -2 8" fill="none" stroke="var(--red)" stroke-width="2" />
      <text x="206" y="84" text-anchor="middle" font-size="13" font-weight="700" fill="var(--ink)">wreck</text>
      {/* step 1 — protect: truck on the shoulder, 4-way flashers, triangles behind the scene */}
      {(show(0) || lost(0)) && <rect x="2" y="139" width="316" height="32" rx="6" fill="none" stroke={edge(0)} stroke-width={st(0) === 'cur' || lost(0) ? 2.5 : 0} stroke-dasharray={lost(0) ? '5 3' : '0'} />}
      <g transform={safe ? '' : 'translate(0 -30)'}>
        <rect x="240" y="146" width="50" height="19" rx="2" fill="var(--accent)" stroke="var(--ink)" /><rect x="292" y="147" width="17" height="17" rx="2" fill="var(--accent)" stroke="var(--ink)" />
        {safe && [[238, 148], [238, 163], [311, 148], [311, 163]].map(([x, y]) => <circle cx={x} cy={y} r="4" fill="var(--amber)" stroke="var(--ink)" stroke-width="0.8" />)}
      </g>
      <text x="265" y={safe ? 136 : 110} text-anchor="middle" font-size="13" font-weight="700" fill="var(--ink)" stroke="var(--surface)" stroke-width="3" paint-order="stroke">your truck</text>
      {safe && [70, 118, 166].map((x) => <path d={`M ${x} 146 l 8 15 l -16 0 Z`} fill="var(--red)" stroke="var(--ink)" stroke-width="0.8" />)}
      {safe && <text x="118" y="136" text-anchor="middle" font-size="13" fill="var(--ink)" stroke="var(--surface-2)" stroke-width="3" paint-order="stroke">triangles (GK-06)</text>}
      {show(0) && <Tag k={0} x={222} y={156} />}
      {lost(0) && <g><rect x="120" y="110" width="38" height="20" rx="4" fill="var(--surface)" stroke="var(--ink)" /><path d="M 160 120 L 180 120" stroke="var(--red)" stroke-width="3" />
        <circle cx="184" cy="120" r="11" fill="var(--red)" /><text x="184" y="125" text-anchor="middle" font-size="14" font-weight="700" fill="var(--surface)">!</text>
        <text x="100" y="80" text-anchor="middle" font-size="14" font-weight="700" fill="var(--red)" stroke="var(--surface)" stroke-width="3" paint-order="stroke">SECOND CRASH</text></g>}
      {/* steps 2–5 */}
      <Card k={1} lines={['call from', 'the cab']} bad="✗ no call">
        <rect x="30" y="18" width="16" height="28" rx="4" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.6" /><rect x="33" y="22" width="10" height="10" rx="1" fill="var(--blue-soft)" />
        {!lost(1) && <path d="M 52 24 q 6 8 0 16 M 58 19 q 11 13 0 26" fill="none" stroke="var(--blue)" stroke-width="2.2" />}
      </Card>
      <Card k={2} lines={['pressure,', 'keep warm']} bad="✗ no care">
        <circle cx="18" cy="42" r="6" fill="var(--surface)" stroke="var(--ink)" /><rect x="25" y="35" width="40" height="14" rx="6" fill={lost(2) ? 'var(--surface)' : 'var(--blue-soft)'} stroke="var(--ink)" />
        {!lost(2) && <g><circle cx="52" cy="18" r="5.5" fill="var(--amber)" stroke="var(--ink)" /><path d="M 52 24 L 48 34 M 50 27 L 40 38" stroke="var(--ink)" stroke-width="2.6" stroke-linecap="round" /></g>}
      </Card>
      <Card k={3} lines={['names,', 'plates, DL']} bad="✗ blank">
        <rect x="24" y="16" width="30" height="38" rx="3" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.6" /><rect x="32" y="12" width="14" height="7" rx="2" fill="var(--ink-2)" />
        {!lost(3) && [26, 33, 40, 47].map((y) => <line x1="29" x2="49" y1={y} y2={y} stroke="var(--ink-2)" stroke-width="1.8" />)}
      </Card>
      <Card k={4} lines={['file the', 'report']} bad="✗ not filed">
        <path d="M 24 12 h 22 l 8 8 v 34 h -30 Z" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.6" />
        <text x="39" y="42" text-anchor="middle" font-size="14" font-weight="700" fill={lost(4) ? 'var(--red)' : 'var(--ok)'}>{lost(4) ? '✗' : '✓'}</text>
      </Card>
    </svg>
  );
}

function PartA() {
  const [cur, setCur] = useState(0); const [skip, setSkip] = useState(false);
  const s = STEPS[cur];
  const go = (k: number) => { setCur(k); setSkip(false); };
  return (
    <div class="stack">
      <p class="small">You were in a crash but are not badly hurt. The scene shows the <strong>5 steps in order</strong>; the highlighted one is the step you are on. <P p="2-44" /></p>
      <CrashScene cur={cur} skip={skip} />
      <div role="group" aria-label="Accident steps" style={cols(5)}>{STEPS.map((x, k) => (
        <button class="btn sm" aria-pressed={k === cur} aria-label={`Step ${k + 1}: ${x.name}`} style={{ paddingInline: '2px', ...(k === cur ? segOn : {}) }} onClick={() => go(k)}>{k + 1}</button>))}</div>
      <div class="card tint stack" style={{ gap: '6px' }} role="status" aria-live="polite">
        <div class="eyebrow">Step {cur + 1} of 5</div>
        <strong>{s.name}</strong>
        <p class="small" style={{ margin: 0 }}>{s.detail} <P p="2-44" /></p>
      </div>
      <div style={cols(2)}>
        <button class="btn sm" aria-pressed={skip} onClick={() => setSkip(!skip)} style={skip ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}}>{skip ? 'Show it done right' : 'What if I skip it?'}</button>
        <button class="btn primary sm" onClick={() => go((cur + 1) % 5)}>{cur < 4 ? 'Next step →' : 'Start again'}</button>
      </div>
      {skip && <div class="feedback bad" role="status"><div class="verdict">Skipping “{s.name}”</div><p class="small" style={{ margin: 0 }}>{WRONG[s.id]} <P p="2-44" /></p></div>}
      <p class="small muted" style={{ margin: 0 }}>Time order: make the scene safe → get help coming → help people → paperwork (collect, then report).</p>
    </div>
  );
}

/** Challenge item 1: tap the steps in order. One evidence call when the order is complete (ok only if no wrong tap). */
const SHUFFLED = ['collect', 'care', 'report', 'protect', 'notify'];
function OrderCheck({ onDone }: { onDone: (ok: boolean) => void }) {
  const [n, setN] = useState(0); const [bad, setBad] = useState<string | null>(null); const [slips, setSlips] = useState(0);
  const tap = (id: string) => {
    if (n >= 5) return;
    const k = STEPS.findIndex((x) => x.id === id);
    if (k === n) { setN(n + 1); setBad(null); if (n + 1 === 5) onDone(slips === 0); } else { setBad(STEPS[n].id); setSlips(slips + 1); }
  };
  return (
    <div class="stack">
      <strong>You were in a crash but are not badly hurt. Tap the 5 steps in the order you do them.</strong>
      <CrashScene cur={n} hide skip={bad === 'protect'} />
      <div role="group" aria-label="Accident steps to order" style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(130px, 1fr))', gap: '6px' }}>{SHUFFLED.map((id) => { const k = STEPS.findIndex((x) => x.id === id); const placed = k < n; return (
        <button class={`btn sm ${placed ? 'primary' : ''}`} style={{ lineHeight: 1.2, ...(placed ? { opacity: 1 } : {}) }} disabled={placed || n === 5} onClick={() => tap(id)}>{placed ? `${k + 1}. ` : ''}{STEPS[k].name}</button>); })}</div>
      {bad && <div class="feedback bad" role="status"><div class="verdict">Not yet — {STEPS[n].name} comes next</div><p class="small" style={{ margin: 0 }}>{WRONG[bad]} <P p="2-44" /></p></div>}
      {n === 5 && <div class={`feedback ${slips === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{slips === 0 ? 'Right order' : `In order now — ${slips} slip${slips > 1 ? 's' : ''}`}</div><p class="small" style={{ margin: 0 }}>Protect → Notify → Care → Collect → Report. <P p="2-44" /></p></div>}
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
    <svg viewBox="0 0 300 136" width="100%" style={{ display: 'block', maxWidth: '400px', marginInline: 'auto' }} role="img" aria-label={`Truck fire, ${size === 0 ? 'small and under control' : size >= 3 ? 'large and spreading' : 'growing'}.${station ? ' Stopped beside fuel pumps: explosion risk.' : ''}`}>
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
    <svg viewBox="0 0 300 64" width="100%" style={{ display: 'block', maxWidth: '400px', marginInline: 'auto' }} role="img" aria-label={`${AGENT_L[agent]} on ${FIRE_L[fire]}: ${res}.`}>
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
      <span class="small" id="g13-burn">What is burning?</span>
      <div role="group" aria-labelledby="g13-burn" style={cols(3)}>{(Object.keys(FIRE_L) as Fire[]).map((k) => <button class="btn sm" aria-pressed={fire === k} style={{ paddingInline: '6px', lineHeight: 1.2, ...(fire === k ? segOn : {}) }} onClick={() => { setFire(k); setAgent(null); }}>{FIRE_L[k]}</button>)}</div>
      <span class="small" id="g13-use">What do you use?</span>
      <div role="group" aria-labelledby="g13-use" style={cols(3)}>{(Object.keys(AGENT_L) as Agent[]).map((k) => <button class="btn sm" aria-pressed={agent === k} style={{ paddingInline: '6px', lineHeight: 1.2, ...(agent === k ? segOn : {}) }} onClick={() => setAgent(k)}>{AGENT_L[k]}</button>)}</div>
      {m && agent && <div class={`feedback ${m[0] ? 'good' : 'bad'}`} role="status"><div class="verdict">{m[0] ? '✓ Works' : '✗ Wrong choice'}</div><MatchVis fire={fire} agent={agent} ok={m[0]} /><p class="small">{m[1]} <P p="2-45" /></p></div>}
      <p class="small muted">Burning tire: it must be cooled — it may take a lot of water. Not sure which extinguisher, above all with HazMat? Wait for firefighters. <P p="2-45" /></p>
      <strong>Truck fire drill</strong>
      <FireVis size={wrong} station={ch[0] === false} />
      {DRILL.map((d, i) => (
        <div class="stack" style={{ gap: '6px' }}><span class="small"><strong>{d.q}</strong></span>
          <div role="group" aria-label={d.q} style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '6px' }}>{(i % 2 ? [false, true] : [true, false]).map((g) => { const lab = g ? d.good : d.bad; return (
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
    <svg viewBox="0 0 320 64" width="100%" style={{ display: 'block', maxWidth: '400px', marginInline: 'auto' }} role="img" aria-label={`BAC gauge at ${b.toFixed(2)}. The CMV limit line is at .04.`}>
      <rect x="0" y="0" width="320" height="64" fill="var(--surface)" />
      <rect x="20" y="20" width="280" height="16" rx="8" fill="var(--surface-2)" stroke="var(--ink-2)" />
      <rect x="20" y="20" width={Math.max(0, x(b) - 20)} height="16" rx="8" fill={b === 0 ? 'var(--ok)' : b < 0.04 ? 'var(--amber)' : 'var(--red)'} />
      <line x1={x(0.04)} y1="12" x2={x(0.04)} y2="44" stroke="var(--red)" stroke-width="2" /><text x={x(0.04)} y="58" text-anchor="middle" {...T} font-size="14">.04 CMV</text>
      <line x1={x(0.08)} y1="16" x2={x(0.08)} y2="40" stroke="var(--ink-2)" stroke-dasharray="3 2" /><text x={x(0.08)} y="58" text-anchor="middle" {...T} font-size="14">.08</text>
      <text x="20" y="12" {...T} font-size="14">0</text><text x={Math.min(x(b), 280)} y="12" {...T} font-size="14" font-weight="700">{b.toFixed(2)}</text>
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
      <div role="group" aria-label="Ways to sober up" style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(120px, 1fr))', gap: '6px' }}>{['Black coffee', 'Cold shower', 'Fresh air', 'Time'].map((k) => <button class="btn sm" aria-pressed={s === k} style={s === k ? segOn : {}} onClick={() => setS(k)}>{k}</button>)}</div>
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
  const [mode, setMode] = useState<'explore' | 'x'>('explore');
  const [tab, setTab] = useState<'a' | 'b' | 'c'>('a');
  // challenge: item 0 = order the 5 steps, items 1..8 = QS
  const [i, setI] = useState(0); const [pick, setPick] = useState<number | null>(null); const [misses, setMisses] = useState(0);
  const [ordered, setOrdered] = useState<boolean | null>(null);
  const total = QS.length + 1;
  const q = i > 0 ? QS[i - 1] : undefined;
  const reset = () => { setI(0); setPick(null); setMisses(0); setOrdered(null); };
  const answer = (k: number) => { if (pick !== null || !q) return; setPick(k); const ok = k === q.a; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const next = () => { if (i + 1 === total && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); };
  const ok = pick !== null && pick === q?.a;
  const crashStep = i === 1 ? 0 : i === 2 ? 1 : 2;
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore</button><button role="tab" aria-selected={mode === 'x'} onClick={() => { setMode('x'); reset(); }}>Challenge ({total})</button></div>
      {mode === 'explore' && <>
        <div role="group" aria-label="Topic" style={cols(3)}>{([['a', 'Crash'], ['b', 'Fire'], ['c', 'Alcohol']] as const).map(([k, l]) => <button class="btn sm" aria-pressed={tab === k} style={tab === k ? segOn : {}} onClick={() => setTab(k)}>{l}</button>)}</div>
        {tab === 'a' && <PartA />}{tab === 'b' && <PartB />}{tab === 'c' && <PartC />}
      </>}
      {mode === 'x' && i === 0 && <div class="stack">
        <span class="small muted num">Check 1 of {total} · Crash</span>
        <OrderCheck onDone={(good) => { setOrdered(good); if (!good) setMisses(misses + 1); onEvidence({ concepts, ok: good }); }} />
        {ordered !== null && <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={next}>Next</button>}
      </div>}
      {mode === 'x' && i > 0 && (q ? (
        <div class="stack">
          <span class="small muted num">Check {i + 1} of {total} · {q.vis === 'crash' ? 'Crash' : q.vis === 'fire' ? 'Fire' : 'Alcohol'}</span>
          <strong>{q.q}</strong>
          <div class="stack" role="group" aria-label="Answers">{q.opts.map((o, k) => (
            <button class={`btn sm ${pick !== null && k === q.a ? 'primary' : ''}`} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick !== null && k === q.a ? { opacity: 1 } : {}), ...(pick === k && k !== q.a ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}) }} disabled={pick !== null} onClick={() => answer(k)}>{pick !== null && (k === q.a ? '✓ ' : k === pick ? '✗ ' : '')}{o}</button>))}</div>
          {pick !== null && q.vis === 'crash' && <CrashScene cur={crashStep} skip={!ok} />}
          {pick !== null && q.vis === 'fire' && <FireVis size={ok ? 0 : 3} station={!ok && pick === 2 && i === 5} />}
          {pick !== null && q.vis === 'bac' && <BacVis b={i === 7 ? 0.02 : 0.04} />}
          {pick !== null && <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
            <div class="verdict">{ok ? 'Right' : 'Not quite'}</div>
            <p class="small">{!ok && <><strong>What happens:</strong> {q.bad} </>}{q.why} <P p={q.p} /></p>
            <button class="btn primary sm" onClick={next}>{i + 1 === total ? 'Finish' : 'Next'}</button></div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? `All ${total} right — stamp earned` : `${total - misses} of ${total} right`}</div>
          <button class="btn sm" onClick={reset}>Try again</button></div>
      ))}
    </div>
  );
}
