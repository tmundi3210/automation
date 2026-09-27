import { useState } from 'preact/hooks';
import type { JSX } from 'preact';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk06-warning-triangles', title: 'Place the warning triangles', lesson: 'GK-06', anchor: /communicating your presence/i,
  summary: 'Your truck is stopped at the roadside. Tap a triangle, then tap a spot on the road. Then set out all 3 on three different roads.',
  stamp: { id: 'triangles-placed', name: 'Triangles placed', rule: 'Place all 3 triangles right on the divided, two-lane and curve roads, and know the 10-minute rule, with no mistakes.' },
};

interface Slot { id: string; x: number; y: number; label: string; ly?: number; lx?: number }
interface Scene {
  id: string; name: string; tag: string; page: string; fig: string;
  slots: Slot[];
  /** Each group is met by one triangle placed on any slot in it. */
  need: { ids: string[]; ok: string; missing: string }[];
  wrong: Record<string, string>;
  layout: string;
}

const SCENES: Scene[] = [
  {
    id: 'div', name: 'Divided or one-way highway', tag: 'Traffic comes from behind only', page: '2-14', fig: 'Figure 2.8',
    slots: [
      { id: 'b200', x: 48, y: 133, label: '200 ft' }, { id: 'b150', x: 94, y: 133, label: '150 ft' }, { id: 'b100', x: 140, y: 133, label: '100 ft' },
      { id: 'b50', x: 186, y: 133, label: '50 ft' }, { id: 'b10', x: 226, y: 133, label: '10 ft' }, { id: 'a100', x: 330, y: 133, label: '100 ft ahead', lx: 318 },
    ],
    need: [
      { ids: ['b10'], ok: '10 ft behind — marks where your truck is.', missing: 'Nothing at 10 ft behind: the triangle closest to the truck is missing.' },
      { ids: ['b100'], ok: '100 ft behind — the middle warning.', missing: 'Nothing at 100 ft behind.' },
      { ids: ['b200'], ok: '200 ft behind — the first warning drivers reach.', missing: 'Nothing at 200 ft behind: drivers get their first warning too late.' },
    ],
    wrong: {
      b50: '50 ft is not a handbook spot. Divided or one-way: 10, 100 and 200 ft toward approaching traffic.',
      b150: '150 ft is not a handbook spot. Divided or one-way: 10, 100 and 200 ft toward approaching traffic.',
      a100: 'Ahead of the truck warns no one here: on a one-way or divided road all traffic comes from behind you, so all 3 go behind.',
    },
    layout: 'All 3 behind the truck, toward approaching traffic: 10 ft, 100 ft and 200 ft.',
  },
  {
    id: 'two', name: 'Two-lane road, traffic both ways', tag: 'Traffic comes from both directions', page: '2-15', fig: 'Figure 2.9',
    slots: [
      { id: 'b200', x: 40, y: 142, label: '200 ft' }, { id: 'b100', x: 104, y: 142, label: '100 ft' }, { id: 'r10', x: 154, y: 142, label: '10 ft', ly: 170 },
      { id: 'f10', x: 246, y: 142, label: '10 ft', ly: 170 }, { id: 'a100', x: 296, y: 142, label: '100 ft' }, { id: 'a200', x: 340, y: 142, label: '200 ft' },
    ],
    need: [
      { ids: ['r10', 'f10'], ok: 'Within 10 ft of a corner — marks where the vehicle is.', missing: 'No triangle within 10 ft of the front or rear corner to mark the vehicle.' },
      { ids: ['b100'], ok: '100 ft behind — warns traffic coming up behind you.', missing: 'Nothing 100 ft behind: traffic in your lane gets no early warning.' },
      { ids: ['a100'], ok: '100 ft ahead — warns traffic coming from the other direction.', missing: 'Nothing 100 ft ahead: oncoming traffic from the front gets no warning.' },
    ],
    wrong: {
      b200: '200 ft is the divided-highway pattern. On a two-lane road the rear one goes 100 ft behind.',
      a200: 'Too far. On a two-lane road the front one goes 100 ft ahead.',
      r10: 'Only one triangle goes at a corner. The other two go 100 ft behind and 100 ft ahead.',
      f10: 'Only one triangle goes at a corner. The other two go 100 ft behind and 100 ft ahead.',
    },
    layout: 'One within 10 ft of the front or rear corner, one 100 ft behind, one 100 ft ahead — on the shoulder or in your lane.',
  },
  {
    id: 'curve', name: 'Curve hides the truck', tag: 'Drivers cannot see you within 500 ft', page: '2-15', fig: 'Figure 2.10',
    slots: [
      { id: 'bc', x: 22, y: 26, label: '≈400 ft', ly: 54, lx: 38 }, { id: 'b100', x: 176, y: 150, label: '100 ft', ly: 180 },
      { id: 'r10', x: 226, y: 156, label: '10 ft', ly: 182 }, { id: 'f10', x: 310, y: 156, label: '10 ft', ly: 182 }, { id: 'a100', x: 346, y: 156, label: '100 ft', ly: 196, lx: 340 },
    ],
    need: [
      { ids: ['r10', 'f10'], ok: 'Within 10 ft of a corner — marks where the vehicle is.', missing: 'No triangle within 10 ft of a corner to mark the vehicle.' },
      { ids: ['a100'], ok: '100 ft ahead — warns traffic from the other direction.', missing: 'Nothing 100 ft ahead for oncoming traffic.' },
      { ids: ['bc'], ok: 'Rear triangle moved back beyond the curve (Figure 2.10 shows 100–500 ft), so drivers are warned before they can see the truck.', missing: 'The rear triangle is not beyond the curve: drivers round the bend and meet your truck with no warning.' },
    ],
    wrong: {
      b100: 'Still inside the hidden stretch. Drivers see this triangle and your truck at almost the same moment. Move the rearmost one back beyond the curve.',
      r10: 'Only one triangle goes at a corner.', f10: 'Only one triangle goes at a corner.',
    },
    layout: 'One within 10 ft of a corner, one 100 ft ahead, and the rearmost moved back beyond the curve (100–500 ft) so drivers are warned in time.',
  },
];

type Placed = (string | null)[];
interface Judged { slot: string; ok: boolean; text: string }
export function judge(sc: Scene, placed: Placed): { items: Judged[]; missing: string[]; ok: boolean } {
  const met = new Set<number>();
  const items: Judged[] = [];
  placed.forEach((s) => {
    if (!s) return;
    const g = sc.need.findIndex((n, i) => !met.has(i) && n.ids.includes(s));
    if (g >= 0) { met.add(g); items.push({ slot: s, ok: true, text: sc.need[g].ok }); }
    else items.push({ slot: s, ok: false, text: sc.wrong[s] ?? 'Not a handbook spot.' });
  });
  const missing = sc.need.filter((_, i) => !met.has(i)).map((n) => n.missing);
  return { items, missing, ok: items.every((i) => i.ok) && missing.length === 0 };
}
const answer = (sc: Scene): Placed => sc.need.map((n) => n.ids[0]);
const slotName = (sc: Scene, id: string) => { const s = sc.slots.find((x) => x.id === id)!; return id === 'bc' ? 'beyond the curve (≈400 ft back)' : id.startsWith('a') ? `${s.label} ahead`.replace(' ahead ahead', ' ahead') : id === 'r10' ? '10 ft from the rear corner' : id === 'f10' ? '10 ft from the front corner' : `${s.label} behind`; };

const T = ({ x, y, bad }: { x: number; y: number; bad?: boolean }) => (
  <g><path d={`M${x} ${y - 9} L${x + 9} ${y + 7} L${x - 9} ${y + 7} Z`} fill="var(--red)" stroke={bad ? 'var(--ink)' : 'var(--surface)'} stroke-width="1.5" />
    <path d={`M${x} ${y - 3} L${x + 4} ${y + 4} L${x - 4} ${y + 4} Z`} fill="var(--surface)" /></g>
);
function Truck({ x, y, w, flash, rm }: { x: number; y: number; w: number; flash: boolean; rm: boolean }) {
  const lights = [[x + 2, y + 2], [x + 2, y + 18], [x + w - 2, y + 2], [x + w - 2, y + 18]];
  return (
    <g>
      <rect x={x} y={y} width={w - 16} height={20} rx={2} fill="var(--surface-2)" stroke="var(--ink)" stroke-width="1.5" />
      <rect x={x + w - 14} y={y + 1} width={14} height={18} rx={3} fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" />
      {flash && lights.map(([cx, cy]) => <circle cx={cx} cy={cy} r={3.5} fill="var(--amber)" stroke="var(--ink)" stroke-width=".8">{!rm && <animate attributeName="opacity" values="1;0.15;1" dur="1s" repeatCount="indefinite" />}</circle>)}
    </g>
  );
}
const Car = ({ x, y, dir }: { x: number; y: number; dir: 1 | -1 }) => (
  <g><rect x={x - 10} y={y - 6} width={20} height={12} rx={3} fill="var(--blue)" /><path d={dir > 0 ? `M${x + 14} ${y} l-4 -4 v8 z` : `M${x - 14} ${y} l4 -4 v8 z`} fill="var(--ink)" /></g>
);

function SceneSvg({ sc, flash, rm }: { sc: Scene; flash: boolean; rm: boolean }) {
  const txt = { 'font-size': 13, fill: 'var(--ink)' } as const;
  const base = <rect width="360" height="200" fill="var(--accent-soft)" />;
  let body: JSX.Element;
  if (sc.id === 'div') body = (<g>
    <rect y="8" width="360" height="18" fill="var(--accent-soft)" stroke="var(--ink-2)" stroke-dasharray="2 3" /><text x="180" y="21" text-anchor="middle" {...txt} font-size="13">Median</text>
    <rect y="30" width="360" height="90" fill="var(--surface)" /><line x1="0" x2="360" y1="75" y2="75" stroke="var(--ink-2)" stroke-width="2" stroke-dasharray="10 8" />
    <rect y="120" width="360" height="26" fill="var(--surface-2)" /><line x1="0" x2="360" y1="120" y2="120" stroke="var(--ink-2)" stroke-width="2" />
    <Car x={40} y={52} dir={1} /><Car x={120} y={98} dir={1} /><text x="64" y="57" {...txt}>traffic →</text>
    <Truck x={240} y={123} w={60} flash={flash} rm={rm} />
    <text x="8" y="190" {...txt} font-size="13">◄ distances behind the truck</text>
  </g>);
  else if (sc.id === 'two') body = (<g>
    <rect y="20" width="360" height="20" fill="var(--surface-2)" /><rect y="40" width="360" height="90" fill="var(--surface)" /><rect y="130" width="360" height="24" fill="var(--surface-2)" />
    <line x1="0" x2="360" y1="40" y2="40" stroke="var(--ink-2)" stroke-width="2" /><line x1="0" x2="360" y1="130" y2="130" stroke="var(--ink-2)" stroke-width="2" />
    <line x1="0" x2="360" y1="85" y2="85" stroke="var(--amber)" stroke-width="3" stroke-dasharray="12 8" />
    <Car x={320} y={62} dir={-1} /><text x="296" y="67" text-anchor="end" {...txt}>← traffic</text>
    <Car x={30} y={108} dir={1} /><text x="54" y="113" {...txt}>traffic →</text>
    <Truck x={170} y={132} w={60} flash={flash} rm={rm} />
    <text x="8" y="192" {...txt} font-size="13">◄ behind</text><text x="352" y="192" text-anchor="end" {...txt} font-size="13">ahead ►</text>
  </g>);
  else body = (<g>
    <path d="M85 8 h92 v66 h-92 z" fill="var(--ok)" opacity=".35" />
    {[[100, 24], [128, 20], [156, 28], [110, 52], [140, 56], [166, 58]].map(([cx, cy]) => <circle cx={cx} cy={cy} r={11} fill="var(--ok)" stroke="var(--ink)" stroke-width=".8" />)}
    <path d="M40 -10 V50 C40 118 80 118 150 118 H370" fill="none" stroke="var(--ink-2)" stroke-width="74" />
    <path d="M40 -10 V50 C40 118 80 118 150 118 H370" fill="none" stroke="var(--surface)" stroke-width="70" />
    <path d="M40 -10 V50 C40 118 80 118 150 118 H370" fill="none" stroke="var(--amber)" stroke-width="3" stroke-dasharray="12 8" />
    <line x1="22" y1="4" x2="240" y2="150" stroke="var(--red)" stroke-width="1.5" stroke-dasharray="4 4" /><text x="92" y="46" font-size="18" font-weight="700" fill="var(--red)">✕</text>
    <text x="186" y="30" {...txt} font-size="13">Trees hide the</text><text x="186" y="44" {...txt} font-size="13">truck until the bend</text>
    <text x="292" y="98" text-anchor="end" {...txt} font-size="13">← traffic</text>
    <text x="22" y="92" text-anchor="middle" font-size="16" font-weight="700" fill="var(--blue)">↓</text><text x="160" y="143" font-size="16" font-weight="700" fill="var(--blue)">→</text>
    <Truck x={240} y={146} w={56} flash={flash} rm={rm} />
  </g>);
  return (
    <svg viewBox="0 0 360 200" width="100%" role="img" aria-label={`${sc.name}, top view. Your truck is stopped at the roadside. ${sc.tag}. Marked spots show distances from the truck.`}>
      {base}{body}
      {sc.slots.map((s) => <text x={s.lx ?? s.x} y={s.ly ?? (sc.id === 'div' ? 164 : 172)} text-anchor="middle" font-size="13" font-weight="700" fill="var(--ink)">{s.label}</text>)}
    </svg>
  );
}

function Road({ sc, placed, setPlaced, sel, setSel, flash, rm, locked, result, showAnswer }: {
  sc: Scene; placed: Placed; setPlaced: (p: Placed) => void; sel: number | null; setSel: (n: number | null) => void; flash: boolean; rm: boolean; locked: boolean;
  result?: ReturnType<typeof judge>; showAnswer?: boolean;
}) {
  const tap = (id: string) => {
    if (locked) return;
    const p = [...placed];
    const at = p.indexOf(id);
    if (sel !== null) { if (at >= 0) p[at] = null; p[sel] = id; }
    else if (at >= 0) p[at] = null;
    else { const free = p.indexOf(null); if (free < 0) return; p[free] = id; }
    setPlaced(p); setSel(null);
  };
  const ans = showAnswer ? answer(sc) : [];
  return (
    <div class="stack" style={{ gap: '8px' }}>
      <div style={{ position: 'relative', maxWidth: '560px', width: '100%', margin: '0 auto' }}>
        <SceneSvg sc={sc} flash={flash} rm={rm} />
        <svg viewBox="0 0 360 200" width="100%" aria-hidden="true" style={{ position: 'absolute', inset: 0, pointerEvents: 'none' }}>
          {ans.map((id) => { const s = sc.slots.find((x) => x.id === id)!; return <circle cx={s.x} cy={s.y} r={15} fill="none" stroke="var(--ok)" stroke-width="3" stroke-dasharray="4 3" />; })}
          {placed.map((id) => { if (!id) return null; const s = sc.slots.find((x) => x.id === id)!; const bad = result?.items.find((i) => i.slot === id && !i.ok); return <g><T x={s.x} y={s.y} bad={!!bad} />{result && <text x={Math.min(s.x + 10, 346)} y={s.y - 8} font-size="14" font-weight="700" fill={bad ? 'var(--red)' : 'var(--ok)'}>{bad ? '✕' : '✓'}</text>}</g>; })}
        </svg>
        {sc.slots.map((s) => {
          const who = placed.indexOf(s.id);
          return <button aria-label={`Spot ${slotName(sc, s.id)}${who >= 0 ? `, has triangle ${who + 1}` : ', empty'}`} disabled={locked}
            onClick={() => tap(s.id)}
            style={{ position: 'absolute', left: `${(s.x / 360) * 100}%`, top: `${(s.y / 200) * 100}%`, transform: 'translate(-50%,-50%)', width: '30px', height: '30px', borderRadius: '50%', padding: 0, cursor: locked ? 'default' : 'pointer', background: 'transparent',
              border: who >= 0 ? '2px solid transparent' : `2px dashed ${sel !== null ? 'var(--accent)' : 'var(--ink)'}`, boxShadow: sel !== null && who < 0 ? '0 0 0 2px var(--accent-soft)' : 'none' }} />;
        })}
      </div>
      <div class="row" role="group" aria-label="Your 3 triangles">
        {placed.map((id, i) => (
          <button class="btn sm" aria-pressed={sel === i} disabled={locked} onClick={() => setSel(sel === i ? null : i)}
            style={sel === i ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}}>
            <span aria-hidden="true" style={{ color: sel === i ? 'inherit' : 'var(--red)' }}>▲</span> {id ? slotName(sc, id) : `Triangle ${i + 1}: in hand`}
          </button>
        ))}
      </div>
      <p class="small muted" style={{ margin: 0 }}>{sel !== null ? `Triangle ${sel + 1} picked up — tap a dashed spot on the road.` : 'Tap a dashed spot to drop the next triangle there, or tap a triangle first to move it. Tap a placed triangle’s spot to pick it back up.'} Not to scale.</p>
    </div>
  );
}

function Feedback({ sc, r, flash }: { sc: Scene; r: ReturnType<typeof judge>; flash: boolean }) {
  const ok = r.ok && flash;
  return (
    <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
      <div class="verdict">{ok ? 'Drivers are warned in time' : r.ok ? 'Triangles right — but flashers are off' : 'A driver could come up on your truck unwarned'}</div>
      <ul class="small" style={{ margin: 0, paddingLeft: '18px' }}>
        {r.items.map((i) => <li><strong>{i.ok ? '✓' : '✕'} {slotName(sc, i.slot)}:</strong> {i.text}</li>)}
        {r.missing.map((m) => <li><strong>✕ Missing:</strong> {m}</li>)}
        <li><strong>{flash ? '✓' : '✕'} 4-way flashers {flash ? 'on' : 'off'}:</strong> {flash ? 'good — turn them on whenever you stop at the roadside.' : 'turn them on. Don’t trust taillights: drivers have hit parked trucks thinking they were moving.'} <span class="plate">p. 2-14</span></li>
      </ul>
      <p class="small" style={{ margin: 0 }}><strong>Handbook ({sc.fig}):</strong> {sc.layout} <span class="plate">p. {sc.page}</span></p>
    </div>
  );
}

const Q10 = { q: 'You had to stop on the shoulder. How soon must your warning devices be out?', opts: ['Within 5 minutes', 'Within 10 minutes', 'Within 30 minutes'], a: 1 };

export default function WarningTriangles({ onEvidence, onChallenge, concepts, reducedMotion }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  const [si, setSi] = useState(0);
  const [placed, setPlaced] = useState<Placed>([null, null, null]);
  const [sel, setSel] = useState<number | null>(null);
  const [flash, setFlash] = useState(false);
  const [shown, setShown] = useState<ReturnType<typeof judge> | null>(null);
  const [answerOn, setAnswerOn] = useState(false);
  const [step, setStep] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const reset = () => { setPlaced([null, null, null]); setSel(null); setFlash(false); setShown(null); setAnswerOn(false); };
  const go = (m: 'explore' | 'challenge') => { setMode(m); reset(); setSi(0); setStep(0); setPick(null); setMisses(0); };
  const sc = mode === 'explore' ? SCENES[si] : SCENES[step];
  const allPlaced = placed.every(Boolean);
  const flashBtn = (
    <button class="btn sm" aria-pressed={flash} disabled={mode === 'challenge' && !!shown} onClick={() => setFlash(!flash)}
      style={flash ? { background: 'var(--amber)', borderColor: 'var(--amber)', color: 'var(--amber-ink)' } : {}}>
      <span aria-hidden="true">⚠</span> 4-way flashers: {flash ? 'ON' : 'off'}
    </button>
  );
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => go('explore')}>Explore</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => go('challenge')}>Challenge: 3 roads</button></div>
      <div class="card tint small" style={{ padding: '10px 12px' }}>
        <strong>Stopped on a road or its shoulder?</strong> Turn on your 4-way flashers and set out your 3 triangles <strong>within 10 minutes</strong>. Carry them between yourself and oncoming traffic. <span class="plate">p. 2-14</span> <span class="plate">p. 2-15</span>
      </div>
      {mode === 'explore' && (
        <div class="stack">
          <div class="row" role="group" aria-label="Pick a road">{SCENES.map((s, i) => (
            <button class="btn sm" aria-pressed={si === i} onClick={() => { setSi(i); reset(); }} style={si === i ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}}>{s.name}</button>
          ))}</div>
          <p class="small" style={{ margin: 0 }}><strong>{sc.name}.</strong> {sc.tag}.</p>
          <Road sc={sc} placed={placed} setPlaced={(p) => { setPlaced(p); setShown(null); }} sel={sel} setSel={setSel} flash={flash} rm={reducedMotion} locked={false} result={shown ?? undefined} showAnswer={answerOn} />
          <div class="row">{flashBtn}
            <button class="btn sm" disabled={!allPlaced} onClick={() => setShown(judge(sc, placed))}>Check my layout</button>
            <button class="btn sm" onClick={() => { setPlaced(answer(sc)); setFlash(true); setAnswerOn(true); setShown(judge(sc, answer(sc))); }}>Show the handbook layout</button>
            <button class="btn sm" onClick={reset}>Clear</button>
          </div>
          {shown && <Feedback sc={sc} r={shown} flash={flash} />}
        </div>
      )}
      {mode === 'challenge' && step < 3 && (
        <div class="stack">
          <span class="small muted num">Road {step + 1} of 3</span>
          <p style={{ margin: 0 }}><strong>{sc.name}.</strong> {sc.tag}. Your truck is stopped at the roadside. Warn other drivers — lights and triangles.</p>
          <Road sc={sc} placed={placed} setPlaced={setPlaced} sel={sel} setSel={setSel} flash={flash} rm={reducedMotion} locked={!!shown} result={shown ?? undefined} showAnswer={!!shown && !shown.ok} />
          {!shown && <div class="row">{flashBtn}<button class="btn primary sm" disabled={!allPlaced} onClick={() => { const r = judge(sc, placed); setShown(r); const ok = r.ok && flash; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); }}>{allPlaced ? 'Check' : `Place all 3 (${placed.filter(Boolean).length}/3)`}</button></div>}
          {shown && <><Feedback sc={sc} r={shown} flash={flash} />{!shown.ok && <p class="small muted" style={{ margin: 0 }}>Green dashed rings show the handbook spots.</p>}
            <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={() => { reset(); setStep(step + 1); }}>Next</button></>}
        </div>
      )}
      {mode === 'challenge' && step === 3 && (
        <div class="stack">
          <span class="small muted num">Last check</span>
          <strong>{Q10.q}</strong>
          <div class="row" role="group" aria-label="Answers">{Q10.opts.map((o, i) => (
            <button class={`btn sm ${pick !== null && i === Q10.a ? 'primary' : ''}`} disabled={pick !== null} style={pick === i && i !== Q10.a ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}}
              onClick={() => { setPick(i); const ok = i === Q10.a; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); }}>{o}</button>
          ))}</div>
          {pick !== null && <div class={`feedback ${pick === Q10.a ? 'good' : 'bad'}`} role="status"><div class="verdict">{pick === Q10.a ? 'Right' : 'Within 10 minutes'}</div>
            <p class="small" style={{ margin: 0 }}>Any time you stop on a road or its shoulder, set out your warning devices within 10 minutes. <span class="plate">p. 2-14</span></p>
            <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={() => { if (misses === 0) onChallenge?.(); setStep(4); }}>Finish</button></div>}
        </div>
      )}
      {mode === 'challenge' && step === 4 && (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 3 roads right — stamp earned' : `${4 - misses} of 4 right`}</div>
          <p class="small" style={{ margin: 0 }}>Divided/one-way: 10-100-200 ft behind. Two-lane: corner, 100 behind, 100 ahead. Curve or hill: rear one back beyond it.</p>
          <button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={() => go('challenge')}>Try again</button></div>
      )}
    </div>
  );
}
