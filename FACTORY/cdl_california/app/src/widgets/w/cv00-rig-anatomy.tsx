import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';
import { PARTS, PART, ROUNDS, type PartId, type View } from './cv00-rig-anatomy.parts';

export const meta: WidgetMeta = {
  id: 'cv00-rig-anatomy', title: 'Rig anatomy: name the parts', lesson: 'CV-01', anchor: /^start here$/i,
  summary: 'Tap any part of a tractor-semitrailer to learn its name and what to check. Then name 8 highlighted parts.',
  stamp: { id: 'parts-namer', name: 'Parts namer', rule: 'Name all 8 highlighted rig parts with no mistakes.' },
};

const W = 600, H = 352;
const INK = 'var(--ink)';
/** Tire rubber: dark in both themes; a light --ink-2 rim keeps it visible on the dark background. */
const TIRE = '#262b28';

/** Stroke for a part: amber + thick when it is the focused one. */
function sk(sel: PartId | null, id: PartId, w = 2) {
  return sel === id ? { stroke: 'var(--amber)', 'stroke-width': w + 2.5 } : { stroke: INK, 'stroke-width': w };
}
function Wheel({ x, y, r = 24, on = false }: { x: number; y: number; r?: number; on?: boolean }) {
  return <g><circle cx={x} cy={y} r={r} fill={TIRE} stroke={on ? 'var(--amber)' : 'var(--ink-2)'} stroke-width={on ? 4.5 : 2} /><circle cx={x} cy={y} r={r * 0.5} fill="var(--surface-2)" stroke="var(--ink-2)" stroke-width="2" /><circle cx={x} cy={y} r={r * 0.16} fill="var(--ink-2)" /></g>;
}

function RigView({ sel }: { sel: PartId | null }) {
  const s = (id: PartId, w?: number) => sk(sel, id, w);
  return (
    <g>
      <rect x="0" y="300" width={W} height={H - 300} fill="var(--surface-2)" />
      <line x1="0" y1="300" x2={W} y2="300" stroke="var(--ink-2)" stroke-width="2" />
      {/* converter dolly inset */}
      <g>
        <rect x="6" y="6" width="156" height="96" rx="8" fill="var(--surface)" stroke="var(--ink-2)" stroke-width="1.5" stroke-dasharray="5 4" />
        <text x="14" y="31" font-size="24" font-weight="700" fill="var(--ink)" font-family="var(--display)">Dolly</text>
        <line x1="22" y1="72" x2="74" y2="72" {...s('dolly', 3)} />
        <circle cx="18" cy="72" r="6" fill="var(--surface)" {...s('dolly')} />
        <rect x="72" y="62" width="72" height="12" rx="2" fill="var(--ink-2)" {...s('dolly')} />
        <rect x="90" y="54" width="42" height="8" rx="2" fill="var(--surface-2)" {...s('dolly')} />
        <rect x="80" y="75" width="26" height="9" rx="4" fill="var(--surface-2)" {...s('dolly', 1.5)} />
        <Wheel x={122} y={86} r={13} on={sel === 'dolly'} />
      </g>
      {/* trailer */}
      <rect x="205" y="62" width="392" height="156" rx="4" fill="var(--surface)" stroke={INK} stroke-width="2" />
      {[245, 285, 325, 365, 405, 445, 485, 525, 565].map((x) => <line x1={x} y1="66" x2={x} y2="214" stroke="var(--line)" stroke-width="2" />)}
      <rect x="205" y="218" width="392" height="10" fill="var(--surface-2)" stroke={INK} stroke-width="2" />
      <rect x="484" y="228" width="96" height="12" fill="var(--ink-2)" />
      <circle cx="213" cy="208" r="6" fill="var(--amber)" {...s('abs', 1.5)} />
      {/* landing gear + crank */}
      <g>
        <line x1="344" y1="228" x2="370" y2="252" {...s('landing', 3)} />
        <rect x="366" y="228" width="12" height="36" fill="var(--surface-2)" {...s('landing')} />
        <rect x="368" y="258" width="8" height="12" fill="var(--ink-2)" />
        <rect x="358" y="268" width="28" height="6" rx="2" fill="var(--ink-2)" {...s('landing', 1.5)} />
        <path d="M378 240 L392 240 L392 252" fill="none" {...s('landing', 3)} />
      </g>
      <rect x="408" y="232" width="60" height="18" rx="9" fill="var(--surface-2)" {...s('tanks')} />
      <line x1="468" y1="241" x2="472" y2="241" stroke={INK} stroke-width="2" />
      <rect x="470" y="231" width="14" height="12" rx="2" fill="var(--surface-2)" {...s('relay')} />
      <Wheel x={505} y={276} on={sel === 'tires'} /><Wheel x={553} y={276} on={sel === 'tires'} />
      <rect x="521" y="234" width="17" height="16" rx="4" fill="var(--surface-2)" {...s('spring')} />
      <line x1="529" y1="250" x2="529" y2="258" stroke={INK} stroke-width="2" />
      <rect x="584" y="254" width="8" height="42" rx="2" fill="var(--ink-2)" {...s('tires', 1.5)} />
      {/* tractor */}
      <rect x="18" y="242" width="306" height="12" fill="var(--ink-2)" stroke={INK} stroke-width="1.5" />
      <rect x="112" y="252" width="70" height="22" rx="11" fill="var(--surface-2)" stroke={INK} stroke-width="2" />
      <path d="M18 244 L18 202 Q20 190 34 188 L86 184 L86 244 Z" fill="var(--accent)" stroke={INK} stroke-width="2" />
      <rect x="84" y="116" width="82" height="128" rx="6" fill="var(--accent)" stroke={INK} stroke-width="2" />
      <rect x="94" y="128" width="46" height="40" rx="4" fill="var(--blue-soft)" stroke={INK} stroke-width="2" />
      <rect x="92" y="176" width="54" height="62" rx="3" fill="none" stroke="var(--accent-ink)" stroke-opacity=".5" stroke-width="1.5" />
      <rect x="170" y="98" width="8" height="146" fill="var(--ink-2)" />
      <rect x="20" y="204" width="8" height="12" fill="var(--amber)" />
      <rect x="12" y="240" width="12" height="22" rx="2" fill="var(--ink-2)" />
      <path d="M178 150 C 192 176 196 176 205 164" fill="none" stroke="var(--ink-2)" stroke-width="3" />
      <path d="M178 160 C 192 196 198 196 205 176" fill="none" stroke="var(--red)" stroke-width="3.5" />
      <path d="M178 170 C 192 208 200 208 205 190" fill="none" stroke="var(--blue)" stroke-width="3.5" />
      <rect x="232" y="228" width="66" height="8" rx="2" fill="var(--ink-2)" stroke={INK} stroke-width="1.5" />
      <rect x="248" y="236" width="36" height="7" fill="var(--ink-2)" />
      <Wheel x={70} y={276} on={sel === 'tires'} /><Wheel x={242} y={276} on={sel === 'tires'} /><Wheel x={292} y={276} on={sel === 'tires'} />
      <rect x="318" y="254" width="7" height="42" rx="2" fill="var(--ink-2)" {...s('tires', 1.5)} />
    </g>
  );
}

function CloseView({ sel }: { sel: PartId | null }) {
  const s = (id: PartId, w?: number) => sk(sel, id, w);
  const hose = (id: PartId, d: string, c: string) => <g><path d={d} fill="none" stroke={sel === id ? 'var(--amber)' : INK} stroke-width={sel === id ? 12 : 9} stroke-linecap="round" /><path d={d} fill="none" stroke={c} stroke-width="6" stroke-linecap="round" /></g>;
  return (
    <g>
      <rect x="0" y="0" width={W} height={H} fill="var(--bg)" />
      <circle cx="470" cy="432" r="128" fill={TIRE} stroke="var(--ink-2)" stroke-width="3" /><circle cx="470" cy="432" r="56" fill="var(--surface-2)" stroke="var(--ink-2)" stroke-width="3" />
      {/* cab back wall + frame */}
      <rect x="-4" y="-4" width="116" height="266" fill="var(--accent)" stroke={INK} stroke-width="2" />
      <rect x="18" y="26" width="64" height="40" rx="5" fill="var(--blue-soft)" stroke={INK} stroke-width="2" />
      <rect x="108" y="66" width="22" height="100" rx="3" fill="var(--surface-2)" stroke={INK} stroke-width="2" />
      <rect x="0" y="262" width={W} height="24" fill="var(--ink-2)" stroke={INK} stroke-width="1.5" />
      {/* tractor protection valve + pipes */}
      <path d="M150 226 L150 126 L130 126" fill="none" stroke="var(--red)" stroke-width="3" />
      <path d="M174 226 L174 150 L130 150" fill="none" stroke="var(--blue)" stroke-width="3" />
      <line x1="112" y1="244" x2="140" y2="244" stroke={INK} stroke-width="4" />
      <rect x="140" y="226" width="46" height="32" rx="5" fill="var(--surface-2)" {...s('tpv')} />
      <circle cx="163" cy="242" r="6" fill="var(--ink-2)" />
      {/* trailer nose, upper plate, sockets */}
      <rect x="300" y="-4" width="304" height="180" fill="var(--surface)" stroke={INK} stroke-width="2" />
      {[380, 460, 540].map((x) => <line x1={x} y1="0" x2={x} y2="172" stroke="var(--line)" stroke-width="3" />)}
      <rect x="300" y="176" width="304" height="18" fill="var(--surface-2)" {...s('upper')} />
      <rect x="284" y="48" width="16" height="20" rx="3" fill="var(--surface-2)" stroke={INK} stroke-width="2" />
      <rect x="280" y="92" width="20" height="18" rx="5" fill="var(--red)" {...s('glad')} />
      <rect x="280" y="124" width="20" height="18" rx="5" fill="var(--blue)" {...s('glad')} />
      {/* air hoses + electrical cord (tractor -> trailer) */}
      <path d="M130 84 C 180 142 250 122 284 58" fill="none" stroke={sel === 'cord' ? 'var(--amber)' : 'var(--ink-2)'} stroke-width={sel === 'cord' ? 9 : 6} stroke-dasharray="10 3" stroke-linecap="round" />
      {hose('red', 'M130 112 C 190 190 250 170 280 101', 'var(--red)')}
      {hose('blue', 'M130 142 C 190 222 250 200 280 133', 'var(--blue)')}
      {/* fifth wheel with cutaway, jaws, kingpin, release handle */}
      <path d="M380 230 L548 230 L526 262 L402 262 Z" fill="var(--surface-2)" stroke={INK} stroke-width="2" />
      <rect x="350" y="200" width="222" height="30" rx="4" fill="var(--ink-2)" {...s('fifth')} />
      <rect x="408" y="203" width="94" height="27" fill="var(--bg)" stroke={INK} stroke-width="1" stroke-dasharray="3 2" />
      <rect x="414" y="206" width="30" height="22" rx="3" fill="var(--ink)" {...s('jaws', 1.5)} />
      <rect x="466" y="206" width="30" height="22" rx="3" fill="var(--ink)" {...s('jaws', 1.5)} />
      <g {...s('kingpin', 1.5)}>
        <rect x="433" y="192" width="44" height="10" fill="var(--surface)" />
        <rect x="446" y="202" width="18" height="24" fill="var(--surface)" />
        <path d="M446 226 L464 226 L474 240 L436 240 Z" fill="var(--surface)" />
      </g>
      <line x1="384" y1="240" x2="330" y2="252" {...s('jaws', 6)} stroke-linecap="round" />
      <circle cx="326" cy="253" r="8" fill="var(--surface)" {...s('jaws')} />
      <path d="M350 240 l7 -12 l7 12" fill="none" {...s('jaws', 3)} />
    </g>
  );
}

/** Where the "zoom to coupling" pill sits on the rig view (clear of the dolly badge at 390 px). */
const ZX = 390, ZY = 37;

function Diagram({ view, sel, focus, badges, onPick, onZoom, reducedMotion }: { view: View; sel: PartId | null; focus: PartId | null; badges: boolean; onPick?: (id: PartId) => void; onZoom?: () => void; reducedMotion: boolean }) {
  const parts = PARTS.filter((p) => p.view === view);
  const f = focus ? PART[focus] : null;
  const label = view === 'rig' ? 'Side view of a tractor-semitrailer (left side) with a converter dolly inset' : 'Close-up of the coupling: back of cab, air lines, glad hands, fifth wheel cut away to show jaws and kingpin';
  return (
    <div style={{ position: 'relative', maxWidth: '560px', width: '100%', margin: '0 auto' }}>
      <svg viewBox={`0 0 ${W} ${H}`} width="100%" role="img" aria-label={label + (f ? `. Highlighted: ${badges ? f.name : 'a part'}.` : '')} style={{ borderRadius: '8px', background: 'var(--bg)' }}>
        {view === 'rig' ? <RigView sel={focus} /> : <CloseView sel={focus} />}
        {badges && parts.map((p) => <g><line x1={p.b[0]} y1={p.b[1]} x2={p.t[0]} y2={p.t[1]} stroke={sel === p.id ? 'var(--amber)' : 'var(--ink-2)'} stroke-width={sel === p.id ? 3 : 1.5} stroke-dasharray={sel === p.id ? '' : '4 3'} /><circle cx={p.t[0]} cy={p.t[1]} r="4" fill={sel === p.id ? 'var(--amber)' : 'var(--ink-2)'} /></g>)}
        {badges && view === 'rig' && <line x1={ZX} y1={ZY + 18} x2="266" y2="226" stroke="var(--ink-2)" stroke-width="1.5" stroke-dasharray="4 3" />}
        {f && <circle cx={f.t[0]} cy={f.t[1]} r="30" fill="none" stroke="var(--amber)" stroke-width="4" stroke-dasharray="8 5">
          {!reducedMotion && <animate attributeName="r" values="26;34;26" dur="1.6s" repeatCount="indefinite" />}
        </circle>}
      </svg>
      {badges && parts.map((p) => {
        const i = PARTS.indexOf(p) + 1, on = sel === p.id;
        return <button type="button" aria-label={`${i}. ${p.name}`} aria-pressed={on} title={p.name} onClick={() => onPick?.(p.id)}
          style={{ position: 'absolute', left: `${(p.b[0] / W) * 100}%`, top: `${(p.b[1] / H) * 100}%`, transform: 'translate(-50%,-50%)', width: '36px', height: '36px', borderRadius: '50%', padding: 0, cursor: 'pointer', font: '700 .9rem/1 var(--body)', border: `2px solid ${on ? 'var(--accent)' : 'var(--ink)'}`, background: on ? 'var(--accent)' : 'var(--surface)', color: on ? 'var(--accent-ink)' : 'var(--ink)', boxShadow: 'var(--shadow)' }}>{i}</button>;
      })}
      {badges && view === 'rig' && <button type="button" aria-label="Zoom in on the coupling: fifth wheel, kingpin, air lines" title="Zoom in on the coupling" onClick={onZoom}
        style={{ position: 'absolute', left: `${(ZX / W) * 100}%`, top: `${(ZY / H) * 100}%`, transform: 'translate(-50%,-50%)', height: '36px', padding: '0 12px', borderRadius: '18px', cursor: 'pointer', font: '700 .85rem/1 var(--body)', border: '2px solid var(--blue)', background: 'var(--blue-soft)', color: 'var(--ink)', whiteSpace: 'nowrap', boxShadow: 'var(--shadow)' }}>⊕ Coupling</button>}
    </div>
  );
}

function PartCard({ id }: { id: PartId }) {
  const p = PART[id];
  return (
    <div class="card tint stack" style={{ gap: '6px' }} role="status" aria-live="polite">
      <div class="spread"><strong style={{ font: '700 1.3rem/1.1 var(--display)' }}>{PARTS.indexOf(p) + 1}. {p.name}</strong><span class="row" style={{ gap: '4px' }}>{p.pages.map((pg) => <span class="plate">p. {pg}</span>)}</span></div>
      <p>{p.def}</p>
      <p class="small"><strong>Check / remember:</strong> {p.check}</p>
    </div>
  );
}

const GROUPS: { name: string; ids: PartId[] }[] = [
  { name: 'Coupling (close-up)', ids: ['fifth', 'jaws', 'kingpin', 'upper'] },
  { name: 'Air & electric', ids: ['glad', 'red', 'blue', 'cord', 'tpv', 'tanks', 'relay'] },
  { name: 'Trailer, brakes & wheels', ids: ['landing', 'spring', 'abs', 'dolly', 'tires'] },
];

export default function RigAnatomy({ onEvidence, onChallenge, concepts, reducedMotion }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'name'>('explore');
  const [view, setView] = useState<View>('rig');
  const [sel, setSel] = useState<PartId | null>(null);
  const [r, setR] = useState(0);
  const [pick, setPick] = useState<PartId | null>(null);
  const [misses, setMisses] = useState(0);
  const choose = (id: PartId) => { setSel(id); setView(PART[id].view); };
  const round = ROUNDS[r];
  const opts: PartId[] = round ? [round.id, ...round.wrong] : [];
  const order = round ? [0, 1, 2].map((k) => opts[(k + r) % 3]) : [];
  const vbtn = (v: View, label: string) => <button type="button" class="btn sm" aria-pressed={view === v} onClick={() => setView(v)} style={view === v ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}}>{label}</button>;
  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore the rig</button>
        <button role="tab" aria-selected={mode === 'name'} onClick={() => setMode('name')}>Name that part (8)</button>
      </div>
      {mode === 'explore' && (
        <div class="stack">
          <div class="row" role="group" aria-label="Diagram view">{vbtn('rig', 'Whole rig')}{vbtn('close', 'Coupling close-up')}</div>
          <Diagram view={view} sel={sel} focus={sel && PART[sel].view === view ? sel : null} badges onPick={choose} onZoom={() => setView('close')} reducedMotion={reducedMotion} />
          {sel ? <PartCard id={sel} /> : <p class="small muted">Tap a numbered dot, or pick a part name below. Tractor faces left; you are looking at the left (driver’s) side.</p>}
          {GROUPS.map((g) => (
            <div class="stack" style={{ gap: '6px' }}>
              <span class="eyebrow">{g.name}</span>
              <div class="row" style={{ gap: '6px' }}>{g.ids.map((id) => <button type="button" class="btn sm" aria-pressed={sel === id} onClick={() => choose(id)} style={sel === id ? { borderColor: 'var(--accent)', background: 'var(--accent-soft)' } : {}}>{PARTS.indexOf(PART[id]) + 1}. {PART[id].name}</button>)}</div>
            </div>
          ))}
        </div>
      )}
      {mode === 'name' && (round ? (
        <div class="stack">
          <div class="spread"><span class="small muted num">Part {r + 1} of {ROUNDS.length}</span><span class="small muted">{PART[round.id].view === 'rig' ? 'Whole rig' : 'Coupling close-up'}</span></div>
          <Diagram view={PART[round.id].view} sel={null} focus={round.id} badges={false} reducedMotion={reducedMotion} />
          <strong>What is the part inside the dashed amber ring?</strong>
          <div class="opts" role="group" aria-label="Pick the part name">{order.map((id, k) => {
            const cls = pick ? (id === round.id ? 'right' : id === pick ? 'wrong' : '') : '';
            return <button type="button" class={`opt ${cls}`} disabled={!!pick} onClick={() => { setPick(id); const ok = id === round.id; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); }}>
              <span class="letter">{'abc'[k]}</span><span>{PART[id].name}</span></button>;
          })}</div>
          {pick && <div class={`feedback ${pick === round.id ? 'good' : 'bad'}`} role="status">
            <div class="verdict">{pick === round.id ? `Right — ${PART[round.id].name}` : `That is the ${PART[round.id].name}`}</div>
            <p class="small">{PART[round.id].def} {PART[round.id].pages.map((pg) => <span class="plate">p. {pg}</span>)}</p>
            {pick !== round.id && <p class="small"><strong>You picked {PART[pick].name}:</strong> {PART[pick].def}</p>}
            <div><button type="button" class="btn primary sm" onClick={() => { if (r + 1 === ROUNDS.length && misses === 0) onChallenge?.(); setPick(null); setR(r + 1); }}>{r + 1 === ROUNDS.length ? 'Finish' : 'Next part'}</button></div>
          </div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status">
          <div class="verdict">{misses === 0 ? `${ROUNDS.length} of ${ROUNDS.length} named — stamp earned: Parts namer` : `${Math.max(0, ROUNDS.length - misses)} of ${ROUNDS.length} right — need all ${ROUNDS.length} for the stamp`}</div>
          <p class="small">{misses === 0 ? 'You know the vocabulary the CV test uses.' : 'Go back to Explore, tap the parts you missed, then try again.'}</p>
          <div><button type="button" class="btn sm" onClick={() => { setR(0); setMisses(0); setPick(null); }}>Name them again</button></div>
        </div>
      ))}
    </div>
  );
}
