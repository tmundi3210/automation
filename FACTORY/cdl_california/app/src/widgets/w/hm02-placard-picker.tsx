import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';
import { MATS, mat, placardsFor, type Pkg, type Result } from './hm02-placard-picker.rules';

export const meta: WidgetMeta = {
  id: 'hm02-placard-picker', title: 'Placard picker: build a shipment', lesson: 'HM-02', anchor: /placard table 2/i,
  summary: 'Load packages of different hazard classes and weights and watch Placard Tables 1 and 2 pick the placards for all 4 sides. Then placard 8 loads.',
  stamp: { id: 'placard-pro', name: 'Placard pro', rule: 'Pick the right placards for all 8 loads (Table 1, the 1,001-lb total, DANGEROUS, bulk and the special rules) with no mistakes.' },
};

const fmt = (n: number) => n.toLocaleString('en-US');
const pressed = (on: boolean) => (on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {});

/** One placard, square-on-point. Class number sits in the lower corner (p. 9-10). Colors are not taught in the lesson, so all are drawn neutral. */
export function Placard({ label, cls, faded, size = 92 }: { label: string; cls: string; faded?: boolean; size?: number }) {
  const words = label.toUpperCase().replace(/ \d\.\d$/, '').split(' ').filter(Boolean);
  const lines = words.length > 3 ? [words.slice(0, 2).join(' '), words.slice(2).join(' ')] : words;
  const y0 = 50 - (lines.length - 1) * 7 - (cls ? 4 : 0);
  return (
    <svg viewBox="0 0 100 100" width={size} height={size} aria-hidden="true" style={{ flex: 'none', opacity: faded ? 0.45 : 1 }}>
      <rect x="15" y="15" width="70" height="70" transform="rotate(45 50 50)" fill="var(--surface)" stroke="var(--ink)" stroke-width="3" stroke-dasharray={faded ? '6 4' : undefined} />
      <rect x="21" y="21" width="58" height="58" transform="rotate(45 50 50)" fill="none" stroke="var(--ink-2)" stroke-width="1.2" />
      {lines.map((w, i) => {
        const est = w.length * 8.4;
        return <text x="50" y={y0 + i * 14} text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--ink)" textLength={est > 60 ? 60 : undefined} lengthAdjust="spacingAndGlyphs">{w}</text>;
      })}
      {cls && <text x="50" y="83" text-anchor="middle" font-size="13" font-weight="700" fill="var(--ink)">{cls}</text>}
    </svg>
  );
}

/** Top view of the rig with a placard set on each of the 4 sides (both sides, both ends, pp. 9-5, 9-9). */
function Rig({ set }: { set: string[] }) {
  const n = set.length;
  const spots: [number, number, string][] = [[20, 76, 'Front'], [344, 76, 'Rear'], [206, 22, 'Left side'], [206, 132, 'Right side']];
  const d = (x: number, y: number) => <g><rect x={x - 12} y={y - 12} width="24" height="24" transform={`rotate(45 ${x} ${y})`} fill={n ? 'var(--amber)' : 'var(--surface)'} stroke="var(--ink)" stroke-width="2" stroke-dasharray={n ? undefined : '4 3'} />
    <text x={x} y={y + 5} text-anchor="middle" font-size="14" font-weight="700" fill={n ? '#14201a' : 'var(--ink-2)'}>{n ? `×${n}` : '—'}</text></g>;
  return (
    <svg viewBox="0 0 364 156" width="100%" role="img" style={{ display: 'block', maxWidth: '520px' }}
      aria-label={n ? `Top view of the tractor-trailer. The same ${n} placard${n > 1 ? 's' : ''} (${set.join(', ')}) go on the front, rear, left side and right side: 4 identical sets.` : 'Top view of the tractor-trailer. No placards are required for this load.'}>
      <rect width="364" height="156" fill="var(--surface-2)" />
      <rect x="44" y="52" width="58" height="48" rx="6" fill="var(--accent)" stroke="var(--ink)" stroke-width="2" />
      <text x="73" y="81" text-anchor="middle" font-size="12" font-weight="700" fill="var(--accent-ink)">tractor</text>
      <rect x="106" y="46" width="222" height="60" rx="3" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
      <text x="217" y="81" text-anchor="middle" font-size="13" fill="var(--ink-2)">trailer (top view)</text>
      {spots.map(([x, y]) => d(x, y))}
      <text x="20" y="114" text-anchor="middle" font-size="13" fill="var(--ink)">front</text>
      <text x="344" y="114" text-anchor="middle" font-size="13" fill="var(--ink)">rear</text>
      <text x="234" y="27" font-size="13" fill="var(--ink)">left side</text>
      <text x="234" y="137" font-size="13" fill="var(--ink)">right side</text>
    </svg>
  );
}

function Board({ r, useD, setUseD }: { r: Result; useD: boolean; setUseD?: (b: boolean) => void }) {
  const d = r.dangerous;
  const shown = useD && d ? [...r.required.filter((p) => !d.covers.includes(p)), 'DANGEROUS'] : r.required;
  const clsOf = (p: string) => r.lines.find((l) => l.placard === p)?.cls ?? '';
  return (
    <div class="stack" style={{ gap: '10px' }}>
      <div class="card tint stack" style={{ gap: '8px' }} role="status" aria-live="polite">
        <div class="spread"><span class="eyebrow">Placards for this load</span><span class="plate">p. 9-10</span></div>
        {shown.length ? <div class="row" style={{ gap: '4px 10px', alignItems: 'flex-start' }}>{shown.map((p) => (
          <figure style={{ margin: 0, width: '96px', textAlign: 'center' }}><Placard label={p} cls={p === 'DANGEROUS' ? '' : clsOf(p)} /><figcaption class="small" style={{ fontWeight: 700 }}>{p}</figcaption></figure>))}</div>
          : <p style={{ font: '700 1.2rem/1.2 var(--display)' }}>No placards required</p>}
        {d && <div class="card warn stack" style={{ gap: '6px', padding: '10px 12px' }}>
          <p class="small"><strong>DANGEROUS option:</strong> 1,001 lb or more of 2+ Table 2 classes. You <em>may</em> use DANGEROUS instead of {d.covers.join(' + ')}. It is an option, not a requirement.
            {d.keep.length > 0 && <> {d.keep.join(', ')}: 2,205 lb or more loaded at one place, so it keeps its <strong>specific</strong> placard.</>}</p>
          {setUseD && <label class="toggle"><input type="checkbox" checked={useD} onChange={(e) => setUseD((e.target as HTMLInputElement).checked)} />Use the DANGEROUS option</label>}
        </div>}
      </div>
      <Rig set={shown} />
      <p class="small muted">Front placard may be on the tractor or the trailer. Each placard: at least <strong>9.84 in (250 mm)</strong> square, <strong>on point</strong>, 3 in from other markings, level and left to right, clean, contrasting background, no slogans. <span class="plate">pp. 9-5, 9-9</span></p>
      {r.lines.length > 0 && <ul class="stack" style={{ gap: '4px', margin: 0, paddingLeft: '1.1em' }}>{r.lines.map((l) => (
        <li class="small"><strong>{l.status === 'req' ? '✓ ' : '✗ '}{l.placard}</strong>{l.status === 'req' ? ' — required. ' : l.status === 'under' ? ' — not needed. ' : l.status === 'not-req' ? ' — not required. ' : ' — '}{l.why}</li>))}</ul>}
    </div>
  );
}

function Meter({ total }: { total: number }) {
  const max = Math.max(2400, total * 1.1), x = (v: number) => 8 + (v / max) * 344;
  return (
    <svg viewBox="0 0 360 54" width="100%" role="img" style={{ display: 'block', maxWidth: '520px' }} aria-label={`Table 2 total (non-bulk, packages included): ${fmt(total)} lb. Placards needed at 1,001 lb or more. ${total >= 1001 ? 'Over the line.' : 'Under the line.'}`}>
      <rect x="8" y="16" width="344" height="14" rx="3" fill="var(--surface-2)" stroke="var(--line)" />
      <rect x="8" y="16" width={Math.max(0, x(total) - 8)} height="14" rx="3" fill={total >= 1001 ? 'var(--amber)' : 'var(--accent)'} />
      <line x1={x(1001)} x2={x(1001)} y1="10" y2="36" stroke="var(--red)" stroke-width="2.5" />
      <text x={x(1001)} y="9" text-anchor="middle" font-size="12" font-weight="700" fill="var(--red)">1,001 lb</text>
      <text x="8" y="50" font-size="13" fill="var(--ink)">Table 2 total: {fmt(total)} lb {total >= 1001 ? '(placard)' : '(no Table 2 placard)'}</text>
    </svg>
  );
}

const pkgText = (p: Pkg) => `${p.bulk && p.lb === 0 ? 'residue only' : `${fmt(p.lb)} lb`} · ${mat(p.mat).name}${p.bulk ? ' · bulk' : ''}${p.inh ? ' · INHALATION HAZARD' : ''}${p.dww ? ' · 2nd hazard: dangerous when wet' : ''} · loaded at ${p.stop}`;

function Builder() {
  const [pkgs, setPkgs] = useState<Pkg[]>([{ mat: '3', lb: 600, stop: 'A' }, { mat: '8', lb: 500, stop: 'A' }]);
  const [f, setF] = useState<Pkg>({ mat: '5.1', lb: 400, stop: 'A' });
  const [useD, setUseD] = useState(false);
  const r = placardsFor(pkgs);
  const m = mat(f.mat);
  return (
    <div class="stack">
      <div class="card stack" style={{ gap: '10px' }}>
        <div class="eyebrow">Add a package</div>
        <div class="grid2">
          <div class="field"><label for="pp-m">Hazard class</label>
            <select id="pp-m" value={f.mat} onChange={(e) => setF({ ...f, mat: (e.target as HTMLSelectElement).value })}>
              <optgroup label="Table 1 — any amount">{MATS.filter((x) => x.table === 1).map((x) => <option value={x.id}>{x.name}</option>)}</optgroup>
              <optgroup label="Table 2 — 1,001 lb or more">{MATS.filter((x) => x.table === 2).map((x) => <option value={x.id}>{x.name}</option>)}</optgroup>
            </select></div>
          <div class="field"><label for="pp-w">Weight with package (lb)</label>
            <input id="pp-w" type="number" inputMode="numeric" min={0} step={100} value={f.lb} onInput={(e) => setF({ ...f, lb: Math.max(0, +(e.target as HTMLInputElement).value || 0) })} /></div>
        </div>
        <div class="row">
          <div role="group" aria-label="Loaded at" class="row" style={{ gap: '6px' }}><span class="small">Loaded at:</span>
            {(['A', 'B'] as const).map((s) => <button class="btn sm" aria-pressed={f.stop === s} style={pressed(f.stop === s)} onClick={() => setF({ ...f, stop: s })}>Stop {s}</button>)}</div>
          {m.table === 2 && <label class="toggle"><input type="checkbox" checked={!!f.bulk} onChange={(e) => setF({ ...f, bulk: (e.target as HTMLInputElement).checked })} />Bulk (over 119 gal)</label>}
          <label class="toggle"><input type="checkbox" checked={!!f.inh} onChange={(e) => setF({ ...f, inh: (e.target as HTMLInputElement).checked })} />INHALATION HAZARD on paper</label>
          <label class="toggle"><input type="checkbox" checked={!!f.dww} onChange={(e) => setF({ ...f, dww: (e.target as HTMLInputElement).checked })} />2nd hazard: dangerous when wet</label>
        </div>
        <p class="small muted">{m.table === 1 ? 'Table 1: placard any amount.' : 'Table 2: counts toward the 1,001-lb total.'}{m.note ? ' ' + m.note : ''}</p>
        <div class="row"><button class="btn primary sm" onClick={() => setPkgs([...pkgs, { ...f, bulk: m.table === 2 && f.bulk }])}>Add package</button>
          <button class="btn sm" onClick={() => setPkgs([])}>Empty the truck</button></div>
      </div>
      <div class="stack" style={{ gap: '6px' }}>
        <div class="eyebrow">On the truck ({pkgs.length})</div>
        {pkgs.length === 0 && <p class="small muted">Nothing loaded yet.</p>}
        <ul style={{ listStyle: 'none', margin: 0, padding: 0, display: 'flex', flexDirection: 'column', gap: '4px' }}>{pkgs.map((p, i) => (
          <li class="spread" style={{ gap: '8px', padding: '4px 8px', border: '1px solid var(--line)', borderRadius: '6px', background: 'var(--surface)' }}>
            <span class="small"><span class="chip" style={{ background: mat(p.mat).table === 1 ? 'var(--red-soft)' : 'var(--blue-soft)', color: 'var(--ink)' }}>Table {mat(p.mat).table}</span> {pkgText(p)}</span>
            <button class="btn sm" aria-label={`Remove ${pkgText(p)}`} onClick={() => setPkgs(pkgs.filter((_, j) => j !== i))}>Remove</button></li>))}</ul>
        <Meter total={r.total2} />
      </div>
      <Board r={r} useD={useD} setUseD={setUseD} />
    </div>
  );
}

interface Load { text: string; pkgs: Pkg[]; opts: string[]; ok: number; why: string; page: string }
export const LOADS: Load[] = [
  { text: 'Two shipping papers, both loaded at Stop A.', pkgs: [{ mat: '3', lb: 600, stop: 'A' }, { mat: '8', lb: 500, stop: 'A' }], ok: 1, page: '9-10',
    opts: ['None — neither class reaches 1,001 lb alone', 'FLAMMABLE + CORROSIVE (or DANGEROUS instead)', 'DANGEROUS placards are required'],
    why: 'Add all Table 2 amounts, packages included: 600 + 500 = 1,100 lb, which is 1,001 or more. DANGEROUS is allowed here (2 classes, under 2,205 lb of either at one place) but it is only an option. Too few placards would leave responders guessing.' },
  { text: 'One Class 8 corrosive shipment, package included.', pkgs: [{ mat: '8', lb: 900, stop: 'A' }], ok: 0, page: '9-10',
    opts: ['No placard required', 'CORROSIVE', 'DANGEROUS'], why: '900 lb is under 1,001, and it is your only HazMat. Class 8 is Table 2, so no placard is required. (You may still show CORROSIVE, since it correctly identifies the hazard — but it is not required.)' },
  { text: 'A small box of Division 1.1 explosives.', pkgs: [{ mat: '1.1', lb: 40, stop: 'A' }], ok: 1, page: '9-10',
    opts: ['None — it is under 1,001 lb', 'EXPLOSIVES 1.1', 'DANGEROUS'], why: '1.1 is Placard Table 1: placard any amount. The 1,001-lb rule is only for Table 2.' },
  { text: 'Dangerous-when-wet material plus drums of flammable liquid.', pkgs: [{ mat: '4.3', lb: 300, stop: 'A' }, { mat: '3', lb: 500, stop: 'A' }], ok: 2, page: '9-10',
    opts: ['DANGEROUS WHEN WET + FLAMMABLE', 'None — 800 lb in all is under 1,001', 'DANGEROUS WHEN WET only'],
    why: '4.3 is Table 1, so it needs its placard at any amount. Only Table 2 counts toward 1,001: the flammable liquid is 500 lb, so no FLAMMABLE placard.' },
  { text: 'Corrosive loaded at Stop A, flammable liquid loaded at Stop B.', pkgs: [{ mat: '8', lb: 2300, stop: 'A' }, { mat: '3', lb: 800, stop: 'B' }], ok: 0, page: '9-10',
    opts: ['CORROSIVE (its own placard) plus a placard for the flammable liquid', 'DANGEROUS placards alone cover both', 'CORROSIVE only — the Class 3 is under 1,001 lb'],
    why: 'Table 2 total is 3,100 lb, so both classes are placarded (the 1,001 lb is the total, not per class). 2,300 lb of Class 8 was loaded at one place — 2,205 or more — so it must have the CORROSIVE placard, not DANGEROUS.' },
  { text: 'An emptied cargo tank (bulk, over 119 gallons) that last hauled gasoline, Class 3. Only residue is left.', pkgs: [{ mat: '3', lb: 0, stop: 'A', bulk: true }], ok: 2, page: '9-11',
    opts: ['None — the tank is empty', 'Only if the residue weighs 1,001 lb or more', 'FLAMMABLE — bulk packages stay placarded with residue'],
    why: 'A bulk package, and the vehicle carrying it, must be placarded even with only residue. The 1,001-lb rule does not apply to bulk packaging.' },
  { text: 'One small Class 8 shipment whose material has a secondary hazard: dangerous when wet.', pkgs: [{ mat: '8', lb: 100, stop: 'A', dww: true }], ok: 1, page: '9-10',
    opts: ['None — under 1,001 lb', 'DANGEROUS WHEN WET (CORROSIVE not needed at 100 lb)', 'CORROSIVE + DANGEROUS WHEN WET'],
    why: 'A dangerous-when-wet secondary hazard needs the DANGEROUS WHEN WET placard in addition to class placards, and the 1,000-lb exception does not apply. The corrosive itself is only 100 lb of Table 2.' },
  { text: 'A 50 lb Class 6.1 shipment; the shipping paper says INHALATION HAZARD.', pkgs: [{ mat: '6.1', lb: 50, stop: 'A', inh: true }], ok: 2, page: '9-10',
    opts: ['None — it is under 1,001 lb', 'DANGEROUS', 'POISON INHALATION (or POISON GAS)'],
    why: 'INHALATION HAZARD on the paper or package means POISON GAS or POISON INHALATION placards, and the 1,000-lb exception does not apply.' },
];

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const [miss, setMiss] = useState(0);
  const L = LOADS[i];
  if (!L) return (
    <div class={`feedback ${miss === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{miss === 0 ? `${LOADS.length} of ${LOADS.length} right — stamp earned: Placard pro` : `${LOADS.length - miss} of ${LOADS.length} right — need all ${LOADS.length} for the stamp`}</div>
      <button class="btn sm" onClick={() => { setI(0); setMiss(0); setPick(null); }}>Placard again</button></div>
  );
  const good = pick === L.ok;
  return (
    <div class="stack">
      <span class="small muted num">Load {i + 1} of {LOADS.length}</span>
      <strong>{L.text}</strong>
      <ul class="card" style={{ margin: 0, padding: '8px 12px 8px 28px' }}>{L.pkgs.map((p) => <li class="small">{pkgText(p)}</li>)}</ul>
      <p class="small">Which placards are <strong>required</strong> on all 4 sides?</p>
      <div class="stack" role="group" aria-label="Pick the placards" style={{ gap: '6px' }}>{L.opts.map((o, k) => (
        <button class={`btn ${pick !== null && k === L.ok ? 'primary' : ''}`} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick === k && k !== L.ok ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }} disabled={pick !== null}
          onClick={() => { setPick(k); const ok = k === L.ok; if (!ok) setMiss(miss + 1); onEvidence({ concepts, ok }); }}>{pick !== null && k === L.ok ? '✓ ' : pick === k ? '✗ ' : ''}{o}</button>))}</div>
      {pick !== null && <>
        <div class={`feedback ${good ? 'good' : 'bad'}`} role="status"><div class="verdict">{good ? 'Right' : `It is: ${L.opts[L.ok]}`}</div>
          <p class="small">{L.why} <span class="plate">p. {L.page}</span></p>
          <button class="btn primary sm" onClick={() => { if (i + 1 === LOADS.length && miss === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === LOADS.length ? 'Finish' : 'Next load'}</button></div>
        <Board r={placardsFor(L.pkgs)} useD={false} />
      </>}
    </div>
  );
}

export default function PlacardPicker(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Build a shipment</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Placard 8 loads</button></div>
      {mode === 'explore' ? <Builder /> : <Challenge {...props} />}
    </div>
  );
}
