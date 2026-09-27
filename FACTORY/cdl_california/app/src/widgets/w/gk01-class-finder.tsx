import { useState } from 'preact/hooks';
import type { ComponentChildren } from 'preact';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk01-class-finder', title: 'Class finder (Figure 1.1)', lesson: 'GK-01', anchor: /which class do you need/i,
  summary: 'Describe a vehicle and follow the handbook’s yes/no chart to the license class. Then sort 7 real vehicles.',
  stamp: { id: 'class-sorter', name: 'Class sorter', rule: 'Sort all 7 vehicles to the right class with no mistakes.' },
};

export type ClassAnswer = 'A' | 'A-88' | 'B' | 'C' | 'none';
export const CLASS_LABEL: Record<ClassAnswer, string> = { A: 'Class A', 'A-88': 'Class A, Restriction 88', B: 'Class B', C: 'Class C', none: 'No CDL needed' };

export interface VehicleInput { towing: boolean; powerGvwr: number; towedGvwr: number; gcwr: number; threeAxle: boolean; weight: number; placardedHazmat: boolean; people: number; paid: boolean; farmLabor: boolean }

/** Figure 1.1 (DL 650 p. 1-8), in the handbook's order. GCWR: use the maker's rating, else power GVWR + loaded towed weight. */
export function classify(v: VehicleInput): { cls: ClassAnswer; why: string } {
  if (v.towing && v.towedGvwr >= 10001) {
    return v.gcwr >= 26001
      ? { cls: 'A', why: `Combination with GCWR ${fmt(v.gcwr)} lb (26,001 or more) and a towed unit of ${fmt(v.towedGvwr)} lb (10,001 or more).` }
      : { cls: 'A-88', why: `Towed unit is ${fmt(v.towedGvwr)} lb (10,001 or more) but GCWR is only ${fmt(v.gcwr)} lb, under 26,001. California adds Restriction 88.` };
  }
  if (v.powerGvwr >= 26001) return { cls: 'B', why: `Power unit GVWR ${fmt(v.powerGvwr)} lb is 26,001 or more${v.towing ? `, towing a unit under 10,001 lb` : ''}.` };
  if (!v.towing && v.threeAxle && v.weight > 6000) return { cls: 'B', why: 'California rule: a single vehicle with 3 axles weighing more than 6,000 lb needs a Class B.' };
  if (v.placardedHazmat) return { cls: 'C', why: 'Not big enough for A or B, but it carries HazMat that needs placards.' };
  if ((v.farmLabor && v.people >= 10) || (v.people > 10 && v.paid)) return { cls: 'B', why: v.farmLabor ? 'Farm labor vehicle for 10 or more people, driver included.' : 'Carries more than 10 people (driver included) for pay, profit or a nonprofit group.' };
  return { cls: 'none', why: 'None of the chart’s conditions apply.' };
}
const fmt = (n: number) => n.toLocaleString('en-US');

const BASE: VehicleInput = { towing: false, powerGvwr: 11000, towedGvwr: 0, gcwr: 11000, threeAxle: false, weight: 8000, placardedHazmat: false, people: 1, paid: false, farmLabor: false };
export const SCENARIOS: { text: string; v: VehicleInput }[] = [
  { text: 'Tractor (GVWR 35,000 lb) pulling a semitrailer (GVWR 45,000 lb)', v: { ...BASE, towing: true, powerGvwr: 35000, towedGvwr: 45000, gcwr: 80000 } },
  { text: 'Box truck (GVWR 30,000 lb) towing a trailer with GVWR 7,000 lb', v: { ...BASE, towing: true, powerGvwr: 30000, towedGvwr: 7000, gcwr: 37000 } },
  { text: 'Pickup (GVWR 11,000 lb) + horse trailer (GVWR 14,000 lb) weighing 12,000 lb loaded; no maker’s GCWR', v: { ...BASE, towing: true, powerGvwr: 11000, towedGvwr: 14000, gcwr: 23000 } },
  { text: 'Nonprofit church van built for 12 people, driver included', v: { ...BASE, powerGvwr: 9500, people: 12, paid: true } },
  { text: 'Single 3-axle vehicle that weighs 7,000 lb', v: { ...BASE, powerGvwr: 9000, threeAxle: true, weight: 7000 } },
  { text: 'Pickup (GVWR 10,000 lb) hauling HazMat that requires placards', v: { ...BASE, powerGvwr: 10000, placardedHazmat: true } },
  { text: 'Pickup (GVWR 9,000 lb) towing a 5,000 lb trailer', v: { ...BASE, towing: true, powerGvwr: 9000, towedGvwr: 5000, gcwr: 14000 } },
];
const CHOICES: ClassAnswer[] = ['A', 'A-88', 'B', 'C', 'none'];

/* ---------- Figure 1.1 as a yes/no path (same order as classify; facts from GK-01's table, p. 1-8) */
type NodeId = 'comb' | 'towed' | 'gcwr' | 'power' | 'axle' | 'hazmat' | 'people';
const NODES: { id: NodeId; q: string; yes?: ClassAnswer; no?: ClassAnswer; ca?: boolean }[] = [
  { id: 'comb', q: 'A combination (towing a unit)?' },
  { id: 'towed', q: 'Towed unit GVWR 10,001 lb or more?' },
  { id: 'gcwr', q: 'GCWR 26,001 lb or more?', yes: 'A', no: 'A-88' },
  { id: 'power', q: 'Power unit GVWR 26,001 lb or more?', yes: 'B' },
  { id: 'axle', q: 'Single vehicle with 3 axles, over 6,000 lb?', yes: 'B', ca: true },
  { id: 'hazmat', q: 'Placarded HazMat?', yes: 'C' },
  { id: 'people', q: 'Farm labor vehicle for 10+ people, or more than 10 people (driver included) for pay, profit or a nonprofit?', yes: 'B' },
];
const SHORT: Record<ClassAnswer, string> = { A: 'Class A', 'A-88': 'A + R88', B: 'Class B', C: 'Class C', none: 'No CDL' };
/** Answers along the chart until a result; unasked nodes are absent. Mirrors classify() step for step. */
export function trace(v: VehicleInput): { ans: Partial<Record<NodeId, boolean>>; end: ClassAnswer; at: NodeId | 'end' } {
  const ans: Partial<Record<NodeId, boolean>> = {};
  ans.comb = v.towing;
  if (v.towing) {
    ans.towed = v.towedGvwr >= 10001;
    if (ans.towed) { ans.gcwr = v.gcwr >= 26001; return { ans, end: ans.gcwr ? 'A' : 'A-88', at: 'gcwr' }; }
  }
  if ((ans.power = v.powerGvwr >= 26001)) return { ans, end: 'B', at: 'power' };
  if ((ans.axle = !v.towing && v.threeAxle && v.weight > 6000)) return { ans, end: 'B', at: 'axle' };
  if ((ans.hazmat = v.placardedHazmat)) return { ans, end: 'C', at: 'hazmat' };
  if ((ans.people = (v.farmLabor && v.people >= 10) || (v.people > 10 && v.paid))) return { ans, end: 'B', at: 'people' };
  return { ans, end: 'none', at: 'end' };
}

export function ChartPath({ v }: { v: VehicleInput }) {
  const t = trace(v);
  const last = NODES.findIndex((n) => n.id === t.at);
  const row = (key: string, reached: boolean, hit: boolean, badge: string, body: ComponentChildren, out?: ComponentChildren) => (
    <li key={key} style={{ display: 'grid', gridTemplateColumns: '44px 1fr', gap: '8px', alignItems: 'center', padding: '3px 8px 3px 6px', borderLeft: `4px solid ${reached ? 'var(--accent)' : 'var(--line)'}`, background: hit ? 'var(--accent-soft)' : 'transparent', borderRadius: '0 6px 6px 0', color: reached ? 'var(--ink)' : 'var(--ink-2)' }}>
      <span class="num" style={{ font: '700 .78rem/1 var(--body)', textAlign: 'center', padding: '4px 0', borderRadius: '4px', border: `1.5px solid ${badge === 'YES' ? 'var(--accent)' : reached ? 'var(--ink-2)' : 'var(--line)'}`, background: badge === 'YES' ? 'var(--accent)' : 'transparent', color: badge === 'YES' ? 'var(--accent-ink)' : 'inherit' }}>{badge}</span>
      <span style={{ display: 'flex', flexWrap: 'wrap', gap: '4px 8px', alignItems: 'center', justifyContent: 'space-between' }}><span style={{ fontSize: '.85rem', lineHeight: 1.3, fontWeight: reached ? 700 : 400 }}>{body}</span>{out}</span>
    </li>
  );
  const chip = (c: ClassAnswer, label: string, on: boolean) => <span class="num" style={{ fontSize: '.78rem', fontWeight: 700, whiteSpace: 'nowrap', padding: '2px 6px', borderRadius: '4px', border: `1.5px solid ${on ? 'var(--accent)' : 'var(--line)'}`, background: on ? 'var(--accent)' : 'var(--surface)', color: on ? 'var(--accent-ink)' : 'var(--ink-2)' }}>{label} → {SHORT[c]}</span>;
  return (
    <figure class="stack" style={{ gap: '6px', margin: 0 }}>
      <figcaption class="spread"><span class="eyebrow">Figure 1.1 path</span><span class="plate">p. 1-8</span></figcaption>
      <ol aria-label={`Chart path: ${NODES.filter((n) => t.ans[n.id] !== undefined).map((n) => `${n.q} ${t.ans[n.id] ? 'Yes' : 'No'}`).join(' ')} Result: ${CLASS_LABEL[t.end]}.`} style={{ listStyle: 'none', margin: 0, padding: 0, display: 'flex', flexDirection: 'column', gap: '3px' }}>
        {NODES.map((n, i) => {
          const a = t.ans[n.id], reached = a !== undefined, hit = i === last;
          const badge = !reached ? (i < last || last < 0 ? 'skip' : '—') : a ? 'YES' : 'NO';
          const out = n.id === 'gcwr' ? <span class="row" style={{ gap: '4px' }}>{chip('A', 'YES', hit && a === true)}{chip('A-88', 'NO', hit && a === false)}</span>
            : n.yes ? chip(n.yes, 'YES', hit) : <span class="small muted" style={{ whiteSpace: 'nowrap' }}>{n.id === 'comb' ? 'NO: skip to 4' : 'NO: to 4'}</span>;
          return row(n.id, reached, hit, badge, <>{i + 1}. {n.q}{n.ca && <span class="ca-tag">CA</span>}</>, out);
        })}
        {row('end', t.at === 'end', t.at === 'end', t.at === 'end' ? 'END' : '—', <>None of these</>, <span class="num" style={{ fontSize: '.78rem', fontWeight: 700, padding: '2px 6px', borderRadius: '4px', border: `1.5px solid ${t.at === 'end' ? 'var(--accent)' : 'var(--line)'}`, background: t.at === 'end' ? 'var(--accent)' : 'var(--surface)', color: t.at === 'end' ? 'var(--accent-ink)' : 'var(--ink-2)' }}>No CDL</span>)}
      </ol>
    </figure>
  );
}

const pressed = (on: boolean) => (on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {});

/** The live class form. `showPath` adds the Figure 1.1 path beside it (the widget); onboarding uses the form alone. */
export function ClassFinderForm({ onResult, showPath }: { onResult?: (r: { cls: ClassAnswer; why: string }) => void; showPath?: boolean }) {
  const [v, setV] = useState<VehicleInput>({ ...BASE });
  const [maker, setMaker] = useState(false); // "I have the maker's GCWR"
  const [makerGcwr, setMakerGcwr] = useState(0);
  const [load, setLoad] = useState<number | null>(null); // towed unit's loaded weight; null = same as its GVWR
  // No maker's figure: GCWR = power unit GVWR + weight of the towed unit and its load (p. 1-8).
  const derive = (nv: VehicleInput, m: boolean, mg: number, ld: number | null): VehicleInput => ({ ...nv, gcwr: !nv.towing ? nv.powerGvwr : m ? mg : nv.powerGvwr + (ld ?? nv.towedGvwr) });
  const invalid = (f: VehicleInput, m: boolean) => f.towing && m && f.gcwr < f.powerGvwr;
  const full = derive(v, maker, makerGcwr, load);
  const bad = invalid(full, maker);
  const r = classify(full);
  const commit = (nv: VehicleInput, m = maker, mg = makerGcwr, ld = load) => {
    if (!nv.towing) { nv.towedGvwr = 0; nv.gcwr = nv.powerGvwr; }
    setV(nv); setMaker(m); setMakerGcwr(mg); setLoad(ld);
    const f = derive(nv, m, mg, ld);
    if (!invalid(f, m)) onResult?.(classify(f));
  };
  const set = (patch: Partial<VehicleInput>) => commit({ ...v, ...patch });
  const val = (e: Event) => Math.max(0, +(e.target as HTMLInputElement).value || 0);
  const num = (id: string, label: string, value: number, onIn: (n: number) => void, hint?: string, err?: string) => (
    <div class="field"><label for={id}>{label}</label><input id={id} type="number" inputMode="numeric" min={0} step={500} value={value} aria-invalid={err ? true : undefined} aria-describedby={hint || err ? `${id}-h` : undefined} style={err ? { borderColor: 'var(--red)' } : {}} onInput={(e) => onIn(val(e))} />
      {(hint || err) && <span id={`${id}-h`} class="small" style={{ color: err ? 'var(--red)' : 'var(--ink-2)', fontWeight: err ? 700 : 400 }}>{err ?? hint}</span>}</div>
  );
  const autoG = v.powerGvwr + (load ?? v.towedGvwr);
  const form = (
    <div class="stack">
      <div class="grid2">
        {num('cf-p', v.towing ? 'Power unit GVWR (lb)' : 'Vehicle GVWR (lb)', v.powerGvwr, (n) => set({ powerGvwr: n }))}
        {v.towing && num('cf-t', 'Towed unit GVWR (lb)', v.towedGvwr, (n) => set({ towedGvwr: n }))}
      </div>
      {v.towing && <div class="card flat stack" style={{ padding: '10px 12px', gap: '8px' }}>
        <label class="toggle" style={{ minHeight: '36px' }}><input type="checkbox" checked={maker} onChange={(e) => { const on = (e.target as HTMLInputElement).checked; commit({ ...v }, on, on && !makerGcwr ? autoG : makerGcwr, load); }} />I have the maker’s GCWR</label>
        {maker ? num('cf-g', 'Maker’s GCWR (lb)', makerGcwr, (n) => commit({ ...v }, true, n, load), 'Use the manufacturer’s rating.', bad ? `A GCWR can’t be less than the power unit’s GVWR (${fmt(v.powerGvwr)} lb). Check the number.` : undefined)
          : <>{num('cf-l', 'Towed unit’s loaded weight (lb)', load ?? v.towedGvwr, (n) => commit({ ...v }, false, makerGcwr, n), 'Starts at its GVWR. Enter what it weighs with its load.')}
            <p class="small num" role="status"><strong>GCWR (auto: power GVWR + loaded towed weight)</strong> = {fmt(v.powerGvwr)} + {fmt(load ?? v.towedGvwr)} = <strong>{fmt(autoG)} lb</strong> <span class="plate">p. 1-8</span></p></>}
      </div>}
      <div class="row">
        {!v.towing && <label class="toggle" style={{ minHeight: '36px' }}><input type="checkbox" checked={v.threeAxle} onChange={(e) => set({ threeAxle: (e.target as HTMLInputElement).checked })} />3 axles</label>}
        {!v.towing && v.threeAxle && num('cf-w', 'Actual weight (lb)', v.weight, (n) => set({ weight: n }))}
        <label class="toggle" style={{ minHeight: '36px' }}><input type="checkbox" checked={v.placardedHazmat} onChange={(e) => set({ placardedHazmat: (e.target as HTMLInputElement).checked })} />Placarded HazMat</label>
        <label class="toggle" style={{ minHeight: '36px' }}><input type="checkbox" checked={v.farmLabor} onChange={(e) => set({ farmLabor: (e.target as HTMLInputElement).checked })} />Farm labor vehicle</label>
      </div>
      <div class="grid2">
        {num('cf-n', 'People carried, driver included', v.people, (n) => set({ people: n }))}
        <label class="toggle" style={{ alignSelf: 'end', minHeight: '44px' }}><input type="checkbox" checked={v.paid} onChange={(e) => set({ paid: (e.target as HTMLInputElement).checked })} />For pay or profit, or used by a nonprofit</label>
      </div>
      <div class={`card ${bad ? 'warn' : 'tint'}`} role="status" aria-live="polite"><div class="eyebrow">Result</div>
        {bad ? <p class="small"><strong>Check the GCWR first.</strong> It can’t be less than the power unit’s GVWR.</p>
          : <><div style={{ font: '700 1.5rem/1.1 var(--display)' }}>{CLASS_LABEL[r.cls]}</div><p class="small">{r.why}</p></>}</div>
    </div>
  );
  return (
    <div class="stack">
      <div class="row" role="group" aria-label="Vehicle type">
        <button class="btn sm" aria-pressed={!v.towing} style={pressed(!v.towing)} onClick={() => set({ towing: false })}>Single vehicle</button>
        <button class="btn sm" aria-pressed={v.towing} style={pressed(v.towing)} onClick={() => set({ towing: true, towedGvwr: v.towedGvwr || 12000 })}>Towing a trailer</button>
      </div>
      {showPath ? <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '16px', alignItems: 'start' }}>{form}{bad ? <p class="small muted">Fix the GCWR to see the chart path.</p> : <ChartPath v={full} />}</div> : form}
    </div>
  );
}

export default function ClassFinder({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'sort'>('explore');
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<ClassAnswer | null>(null);
  const [misses, setMisses] = useState(0);
  const sc = SCENARIOS[i];
  const truth = sc ? classify(sc.v) : null;
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Try the chart</button><button role="tab" aria-selected={mode === 'sort'} onClick={() => setMode('sort')}>Sort 7 vehicles</button></div>
      {mode === 'explore' && <ClassFinderForm showPath />}
      {mode === 'sort' && (sc ? (
        <div class="stack">
          <span class="small muted num">Vehicle {i + 1} of {SCENARIOS.length}</span>
          <strong>{sc.text}</strong>
          <div class="row" role="group" aria-label="Pick a class">{CHOICES.map((c) => (
            <button class={`btn sm ${pick ? (c === truth!.cls ? 'primary' : c === pick ? '' : '') : ''}`} style={pick && c === pick && c !== truth!.cls ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}} disabled={!!pick} onClick={() => { setPick(c); const ok = c === truth!.cls; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); }}>{CLASS_LABEL[c]}</button>
          ))}</div>
          {pick && <ChartPath v={sc.v} />}
          {pick && <div class={`feedback ${pick === truth!.cls ? 'good' : 'bad'}`} role="status"><div class="verdict">{pick === truth!.cls ? 'Right' : `It is ${CLASS_LABEL[truth!.cls]}`}</div><p class="small">{truth!.why} <span class="plate">p. 1-8</span></p>
            <button class="btn primary sm" onClick={() => { if (i + 1 === SCENARIOS.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === SCENARIOS.length ? 'Finish' : 'Next vehicle'}</button></div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`}><div class="verdict">{misses === 0 ? 'All 7 right — stamp earned' : `${SCENARIOS.length - misses} of ${SCENARIOS.length} right`}</div>
          <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); }}>Sort again</button></div>
      ))}
    </div>
  );
}
