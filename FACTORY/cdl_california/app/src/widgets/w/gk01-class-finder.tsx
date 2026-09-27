import { useState } from 'preact/hooks';
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

export function ClassFinderForm({ onResult }: { onResult?: (r: { cls: ClassAnswer; why: string }) => void }) {
  const [v, setV] = useState<VehicleInput>({ ...BASE });
  const r = classify(v);
  const set = (patch: Partial<VehicleInput>) => { const nv = { ...v, ...patch }; if (!nv.towing) { nv.towedGvwr = 0; nv.gcwr = nv.powerGvwr; } setV(nv); onResult?.(classify(nv)); };
  const num = (id: string, label: string, val: number, k: keyof VehicleInput, hint?: string) => (
    <div class="field"><label for={id}>{label}</label><input id={id} type="number" inputMode="numeric" min={0} step={500} value={val} onInput={(e) => set({ [k]: Math.max(0, +(e.target as HTMLInputElement).value || 0) } as Partial<VehicleInput>)} />{hint && <span class="small muted">{hint}</span>}</div>
  );
  return (
    <div class="stack">
      <div class="row" role="group" aria-label="Vehicle type">
        <button class="btn sm" aria-pressed={!v.towing} style={!v.towing ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}} onClick={() => set({ towing: false })}>Single vehicle</button>
        <button class="btn sm" aria-pressed={v.towing} style={v.towing ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}} onClick={() => set({ towing: true, towedGvwr: v.towedGvwr || 12000, gcwr: v.powerGvwr + (v.towedGvwr || 12000) })}>Towing a trailer</button>
      </div>
      <div class="grid2">
        {num('cf-p', v.towing ? 'Power unit GVWR (lb)' : 'Vehicle GVWR (lb)', v.powerGvwr, 'powerGvwr')}
        {v.towing && num('cf-t', 'Towed unit GVWR (lb)', v.towedGvwr, 'towedGvwr')}
        {v.towing && num('cf-g', 'GCWR (lb)', v.gcwr, 'gcwr', 'Maker’s rating, or power unit GVWR + loaded trailer weight.')}
      </div>
      <div class="row">
        {!v.towing && <label class="toggle"><input type="checkbox" checked={v.threeAxle} onChange={(e) => set({ threeAxle: (e.target as HTMLInputElement).checked })} />3 axles</label>}
        {!v.towing && v.threeAxle && num('cf-w', 'Actual weight (lb)', v.weight, 'weight')}
        <label class="toggle"><input type="checkbox" checked={v.placardedHazmat} onChange={(e) => set({ placardedHazmat: (e.target as HTMLInputElement).checked })} />Placarded HazMat</label>
        <label class="toggle"><input type="checkbox" checked={v.farmLabor} onChange={(e) => set({ farmLabor: (e.target as HTMLInputElement).checked })} />Farm labor vehicle</label>
      </div>
      <div class="grid2">
        {num('cf-n', 'People carried, driver included', v.people, 'people')}
        <label class="toggle" style={{ alignSelf: 'end', minHeight: '44px' }}><input type="checkbox" checked={v.paid} onChange={(e) => set({ paid: (e.target as HTMLInputElement).checked })} />For pay or profit, or used by a nonprofit</label>
      </div>
      <div class="card tint" role="status" aria-live="polite"><div class="eyebrow">Result</div><div style={{ font: '700 1.5rem/1.1 var(--display)' }}>{CLASS_LABEL[r.cls]}</div><p class="small">{r.why}</p></div>
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
      {mode === 'explore' && <ClassFinderForm />}
      {mode === 'sort' && (sc ? (
        <div class="stack">
          <span class="small muted num">Vehicle {i + 1} of {SCENARIOS.length}</span>
          <strong>{sc.text}</strong>
          <div class="row" role="group" aria-label="Pick a class">{CHOICES.map((c) => (
            <button class={`btn sm ${pick ? (c === truth!.cls ? 'primary' : c === pick ? '' : '') : ''}`} style={pick && c === pick && c !== truth!.cls ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}} disabled={!!pick} onClick={() => { setPick(c); const ok = c === truth!.cls; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); }}>{CLASS_LABEL[c]}</button>
          ))}</div>
          {pick && <div class={`feedback ${pick === truth!.cls ? 'good' : 'bad'}`} role="status"><div class="verdict">{pick === truth!.cls ? 'Right' : `It is ${CLASS_LABEL[truth!.cls]}`}</div><p class="small">{truth!.why}</p>
            <button class="btn primary sm" onClick={() => { if (i + 1 === SCENARIOS.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === SCENARIOS.length ? 'Finish' : 'Next vehicle'}</button></div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`}><div class="verdict">{misses === 0 ? 'All 7 right — stamp earned' : `${SCENARIOS.length - misses} of ${SCENARIOS.length} right`}</div>
          <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); }}>Sort again</button></div>
      ))}
    </div>
  );
}
