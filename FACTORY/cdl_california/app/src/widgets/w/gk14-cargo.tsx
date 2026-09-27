import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk14-cargo', title: 'Tie-down calculator', lesson: 'GK-14', anchor: /cargo tie-down/i,
  summary: 'Set cargo length and weight, then add straps until the load is legal: 1 per 10 ft (never fewer than 2) and working load limit at least ½ the weight. See the re-check schedule and the bridge formula.',
  stamp: { id: 'load-secured', name: 'Load secured', rule: 'Answer all 6 tie-down, re-check and bridge-formula questions with no mistakes.' },
};

const P = ({ p }: { p: string }) => <span class="plate">p. {p}</span>;
const T = { 'font-size': 14, fill: 'var(--ink)' } as const; // 360-wide diagrams: ≥ 11 px on a 390 px phone
const cols = (n: number) => ({ display: 'grid', gridTemplateColumns: `repeat(${n}, minmax(0, 1fr))`, gap: '6px' });
const segOn = { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' };
const fmt = (n: number) => n.toLocaleString('en-US');

/** p. 3-3: at least 1 tie-down per 10 ft of cargo, never fewer than 2. A leftover part of 10 ft gets one more (lesson: “the safe choice”). */
export const minTieDowns = (lengthFt: number) => Math.max(2, Math.ceil(lengthFt / 10));
/** p. 3-3: aggregate working load limit at least ½ the cargo weight. */
export const minWll = (weightLb: number) => Math.ceil(weightLb / 2);

export function FlatbedVis({ len, straps, ok }: { len: number; straps: number; ok: boolean }) {
  const w = Math.max(24, Math.min(300, len * 6.2)); const x0 = 40 + (300 - w) / 2;
  const xs = Array.from({ length: straps }, (_, i) => x0 + (w * (i + 0.5)) / straps);
  return (
    <svg viewBox="0 0 360 132" width="100%" style={{ display: 'block', maxWidth: '460px', marginInline: 'auto' }} role="img" aria-label={`Flatbed with ${len}-foot cargo and ${straps} tie-down${straps === 1 ? '' : 's'}. ${ok ? 'Secured.' : 'Not enough: the cargo can shift or fall off.'}`}>
      <rect x="0" y="0" width="360" height="132" fill="var(--surface)" />
      <rect x="30" y="96" width="320" height="8" fill="var(--ink-2)" />{[60, 300, 325].map((x) => <circle cx={x} cy="112" r="10" fill="var(--ink-2)" stroke="var(--ink)" />)}
      <rect x="4" y="72" width="24" height="32" rx="3" fill="var(--accent)" stroke="var(--ink)" />
      <g transform={ok ? '' : `rotate(4 ${x0 + w} 96) translate(12 -3)`}>
        <rect x={x0} y="58" width={w} height="38" rx="2" fill="var(--amber-soft)" stroke="var(--ink)" stroke-width="1.2" />
        {xs.map((x) => <path d={`M ${x - 6} 96 L ${x} 58 L ${x + 6} 96`} fill="none" stroke={ok ? 'var(--ok)' : 'var(--red)'} stroke-width="2.5" />)}
      </g>
      <text x="180" y="20" text-anchor="middle" font-size="14" font-weight="700" fill={ok ? 'var(--ok)' : 'var(--red)'}>{ok ? '✓ Secured' : '✗ Load can shift or fall off'}</text>
      <text x="180" y="129" text-anchor="middle" {...T}>{len} ft cargo · {straps} tie-down{straps === 1 ? '' : 's'}</text>
    </svg>
  );
}

/** p. 3-1: check before the trip, within the first 50 miles, then after 3 hours or 150 miles, and after every break. */
export function TimelineVis({ brk, missFirst }: { brk: number | null; missFirst?: boolean }) {
  const X = (mi: number) => 30 + mi * 0.6;
  const checks = [0, 50, 200, 350, 500].filter((m) => !(missFirst && m === 50));
  const lab = (m: number) => (m === 0 ? 'Pre-trip' : m === 50 ? '≤ 50 mi' : '+150 mi');
  return (
    <svg viewBox="0 0 360 112" width="100%" style={{ display: 'block', maxWidth: '460px', marginInline: 'auto' }} role="img" aria-label={`Trip timeline, 500 miles. Cargo checks at ${checks.join(', ')} miles${brk !== null ? ` and after the break at mile ${brk}` : ''}.${missFirst ? ' The 50-mile check is skipped: straps that loosened as the cargo settled are not caught.' : ''}`}>
      <rect x="0" y="0" width="360" height="112" fill="var(--surface)" />
      <line x1={X(0)} y1="54" x2={X(500)} y2="54" stroke="var(--ink-2)" stroke-width="4" stroke-linecap="round" />
      {checks.map((m) => { const up = m === 0 || m === 200 || m === 500; return <g><circle cx={X(m)} cy="54" r="10" fill="var(--accent)" stroke="var(--ink)" /><text x={X(m)} y="59" text-anchor="middle" font-size="14" font-weight="700" fill="var(--accent-ink)">✓</text>
        <text x={m === 0 ? 8 : X(m)} y={up ? 34 : 84} text-anchor={m === 0 ? 'start' : 'middle'} {...T}>{lab(m)}</text></g>; })}
      {brk !== null && <g><rect x={X(brk) - 10} y="44" width="20" height="20" rx="3" fill="var(--blue)" stroke="var(--ink)" /><text x={X(brk)} y="59" text-anchor="middle" font-size="14" font-weight="700" fill="var(--surface)">B</text><text x={X(brk)} y="106" text-anchor="middle" {...T}>break → check</text></g>}
      {missFirst && <g><circle cx={X(50)} cy="54" r="10" fill="var(--red)" /><text x={X(50)} y="59" text-anchor="middle" font-size="14" font-weight="700" fill="var(--surface)">!</text><text x="8" y="106" font-size="14" font-weight="700" fill="var(--red)">✗ 50-mile check skipped: loose strap missed</text></g>}
    </svg>
  );
}

export function BridgeVis({ gap }: { gap: number }) {
  const cx = 180; const a = cx - gap / 2; const b = cx + gap / 2; const tight = gap < 60;
  return (
    <svg viewBox="0 0 360 124" width="100%" style={{ display: 'block', maxWidth: '460px', marginInline: 'auto' }} role="img" aria-label={`Bridge with two axles ${tight ? 'close together: weight presses on one spot' : 'spread apart: weight is shared over more of the bridge'}.`}>
      <rect x="0" y="0" width="360" height="124" fill="var(--surface)" />
      <rect x="20" y="78" width="320" height="10" fill="var(--surface-2)" stroke="var(--ink)" />
      <path d="M 40 88 L 40 120 M 320 88 L 320 120" stroke="var(--ink)" stroke-width="4" />
      <path d={`M 20 88 Q 180 ${tight ? 110 : 98} 340 88`} fill="none" stroke={tight ? 'var(--red)' : 'var(--ok)'} stroke-width="2" stroke-dasharray="4 3" />
      {[a, b].map((x) => <g><circle cx={x} cy="70" r="8" fill="var(--ink-2)" stroke="var(--ink)" /><path d={`M ${x} 38 L ${x} 58`} stroke="var(--amber)" stroke-width={tight ? 5 : 3} marker-end="url(#g14a)" /></g>)}
      <text x="180" y="16" text-anchor="middle" {...T} font-weight="700" fill={tight ? 'var(--red)' : 'var(--ok)'}>{tight ? 'Close axles:' : 'Spread axles:'}</text>
      <text x="180" y="33" text-anchor="middle" {...T}>{tight ? 'weight bunched on one part of the bridge' : 'weight shared along the bridge'}</text>
      <defs><marker id="g14a" viewBox="0 0 10 10" refX="5" refY="5" markerWidth="4" markerHeight="4" orient="auto"><path d="M0 0 L10 5 L0 10 Z" fill="var(--amber)" /></marker></defs>
    </svg>
  );
}

function Calc() {
  const [len, setLen] = useState(25); const [wt, setWt] = useState(10000);
  const [n, setN] = useState(2); const [each, setEach] = useState(1500);
  const need = minTieDowns(len); const needW = minWll(wt); const agg = n * each;
  const countOk = n >= need; const wOk = agg >= needW; const ok = countOk && wOk;
  return (
    <div class="stack">
      <div class="grid2">
        <div class="field"><label for="g14-l">Cargo length: <span class="num">{len} ft</span></label><input id="g14-l" type="range" min={2} max={48} step={1} value={len} onInput={(e) => setLen(+(e.target as HTMLInputElement).value)} /></div>
        <div class="field"><label for="g14-w">Cargo weight: <span class="num">{fmt(wt)} lb</span></label><input id="g14-w" type="range" min={1000} max={40000} step={500} value={wt} onInput={(e) => setWt(+(e.target as HTMLInputElement).value)} /></div>
      </div>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, minmax(0, 1fr))', gap: '12px', alignItems: 'end' }}>
        <div class="field"><span class="small" id="g14-n"><strong>Tie-downs</strong></span>
          <div class="row" role="group" aria-labelledby="g14-n" style={{ gap: '6px', flexWrap: 'nowrap' }}>
            <button class="btn sm" style={{ minWidth: '44px' }} aria-label="One fewer tie-down" disabled={n <= 1} onClick={() => setN(n - 1)}>−</button><strong class="num" aria-live="polite" style={{ minWidth: '1.5em', textAlign: 'center', fontSize: '1.2rem' }}>{n}</strong><button class="btn sm" style={{ minWidth: '44px' }} aria-label="One more tie-down" disabled={n >= 8} onClick={() => setN(n + 1)}>+</button></div></div>
        <div class="field"><label for="g14-e" class="small"><strong>WLL each</strong></label><select id="g14-e" value={each} onChange={(e) => setEach(+(e.target as HTMLSelectElement).value)}>{[1000, 1500, 2000, 3000, 5000].map((v) => <option value={v}>{fmt(v)} lb</option>)}</select></div>
      </div>
      <FlatbedVis len={len} straps={n} ok={ok} />
      <div class="grid2">
        <div class={`feedback ${countOk ? 'good' : 'bad'}`} role="status"><div class="verdict num">Count: {n} / need {need}</div><p class="small">At least 1 per 10 ft, never fewer than 2: {len} ft → <strong>{need}</strong>.{len > 20 && len % 10 !== 0 ? ' The handbook doesn’t say how to count a leftover part of 10 ft; adding one for it is the safe choice.' : ''} <P p="3-3" /></p></div>
        <div class={`feedback ${wOk ? 'good' : 'bad'}`} role="status"><div class="verdict num">WLL: {fmt(agg)} / need {fmt(needW)} lb</div><p class="small">Aggregate working load limit (all ratings added) must be at least ½ the cargo weight: ½ × {fmt(wt)} = <strong>{fmt(needW)} lb</strong>. <P p="3-3" /></p></div>
      </div>
      <p class="small muted">Some question banks say “1½ times the weight.” The handbook says <strong>½</strong> — use that on the test. <P p="3-3" /></p>
    </div>
  );
}
function Schedule() {
  const [brk, setBrk] = useState(true);
  return (
    <div class="stack">
      <TimelineVis brk={brk ? 280 : null} />
      <label class="toggle" style={{ minHeight: '44px' }}><input type="checkbox" checked={brk} onChange={(e) => setBrk((e.target as HTMLInputElement).checked)} />Take a break at mile 280</label>
      <ul class="small"><li>Before the trip: cargo is part of the vehicle inspection.</li><li>Within the first <strong>50 miles</strong>: check cargo and securing devices again.</li><li>Then after every <strong>3 hours or 150 miles</strong>, and after <strong>every break</strong> — and as often as needed. <P p="3-1" /></li></ul>
      <p class="small muted">Why: once the truck moves, cargo settles and shifts, and straps or chains can loosen. <P p="3-1" /></p>
    </div>
  );
}
function Bridge() {
  const [gap, setGap] = useState(160); const lvl = Math.round(((gap - 20) / 220) * 100);
  return (
    <div class="stack">
      <div class="field"><label for="g14-b">Axle spacing: <span>{gap < 60 ? 'close together' : gap < 150 ? 'medium' : 'far apart'}</span></label><input id="g14-b" type="range" min={20} max={240} step={10} value={gap} onInput={(e) => setGap(+(e.target as HTMLInputElement).value)} /></div>
      <BridgeVis gap={gap} />
      <div class="stack" style={{ gap: '4px' }}><span class="small"><strong>Weight the bridge formula allows on these axles</strong></span>
        <div aria-hidden="true" style={{ height: '14px', borderRadius: '7px', background: 'var(--surface-2)', border: '1px solid var(--line)', overflow: 'hidden' }}><div style={{ width: `${10 + lvl * 0.9}%`, height: '100%', background: gap < 60 ? 'var(--red)' : 'var(--accent)' }} /></div>
        <span class="small" role="status" aria-live="polite">{gap < 60 ? 'Less' : gap < 150 ? 'More' : 'Most'} — closer axles are allowed <strong>less</strong> weight. The same weight on a short space presses harder on one part of a bridge or road. <P p="3-2" /></span></div>
      <p class="small muted">Legal isn’t always safe: in bad weather or the mountains, the legal maximum weight may not be safe. <P p="3-2" /></p>
    </div>
  );
}

type Vis = 'tie' | 'time' | 'bridge';
interface Q { q: string; opts: string[]; a: number; why: string; bad: string; p: string; vis: Vis; len?: number; straps?: number[] }
export const QS: Q[] = [
  { vis: 'tie', len: 20, straps: [2, 3, 4], q: 'A flatbed load is 20 feet long. What is the minimum number of tie-downs?', opts: ['2', '3', '4'], a: 0, why: '1 per 10 ft: 20 ÷ 10 = 2, which also meets the minimum of 2. (3 is a common trap.)', bad: '', p: '3-3' },
  { vis: 'tie', len: 4, straps: [1, 2, 3], q: 'A small crate is only 4 feet long. How many tie-downs at minimum?', opts: ['1', '2', '3'], a: 1, why: 'Never fewer than 2, no matter how small the cargo.', bad: 'One strap lets the crate pivot and slide off.', p: '3-3' },
  { vis: 'tie', q: 'A machine weighs 12,000 lb. The working load limits of its tie-downs must add up to at least:', opts: ['6,000 lb', '12,000 lb', '18,000 lb'], a: 0, why: 'Aggregate WLL at least ½ the cargo weight: ½ × 12,000 = 6,000 lb. (“1½ times” is from other sources, not the handbook.)', bad: 'That is more than the rule asks. Extra strength is fine, but the handbook minimum is ½ the weight.', p: '3-3' },
  { vis: 'time', q: 'You start a trip. When is your first cargo re-check?', opts: ['Within the first 25 miles', 'Within the first 50 miles', 'After 150 miles'], a: 1, why: 'Within the first 50 miles — cargo settles and straps loosen early.', bad: 'A strap that loosened as the load settled goes unchecked for far too long.', p: '3-1' },
  { vis: 'time', q: 'After that first check, when do you check the cargo again?', opts: ['Every 2 hours or 100 miles', 'Every 3 hours or 150 miles, and after every break', 'Only at the end of the day'], a: 1, why: 'Every 3 hours or 150 miles, and after every break. (2 hours / 100 miles is the hot-weather tire check, a different rule.)', bad: 'Loose straps can go unchecked between stops.', p: '3-1' },
  { vis: 'bridge', q: 'A bridge formula allows…', opts: ['more weight on axles that are close together', 'less weight on axles that are close together', 'the same weight on every axle'], a: 1, why: 'Less weight on closer axles, to prevent overloading bridges and roadways.', bad: 'Bunching heavy weight on close axles overloads one spot of the bridge.', p: '3-2' },
];

export default function Cargo({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [tab, setTab] = useState<'calc' | 'time' | 'bridge' | 'x'>('calc');
  const [i, setI] = useState(0); const [pick, setPick] = useState<number | null>(null); const [misses, setMisses] = useState(0);
  const q = QS[i]; const ok = pick !== null && pick === q?.a;
  const reset = () => { setI(0); setPick(null); setMisses(0); };
  const answer = (k: number) => { if (pick !== null) return; setPick(k); const good = k === q.a; if (!good) setMisses(misses + 1); onEvidence({ concepts, ok: good }); };
  const next = () => { if (i + 1 === QS.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); };
  const vis = (): preact.JSX.Element | null => {
    if (!q || pick === null) return null;
    if (q.vis === 'tie') { if (q.len) { const s = q.straps![pick]; return <FlatbedVis len={q.len} straps={s} ok={s >= minTieDowns(q.len)} />; } return <FlatbedVis len={16} straps={4} ok />; }
    if (q.vis === 'time') return <TimelineVis brk={ok ? 280 : null} missFirst={!ok && i === 3} />;
    return <BridgeVis gap={ok ? 180 : 30} />;
  };
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={tab !== 'x'} onClick={() => setTab('calc')}>Explore</button><button role="tab" aria-selected={tab === 'x'} onClick={() => { setTab('x'); reset(); }}>Challenge (6)</button></div>
      {tab !== 'x' && <div role="group" aria-label="Explore topic" style={cols(3)}>{([['calc', 'Tie-downs'], ['time', 'Re-check schedule'], ['bridge', 'Bridge formula']] as const).map(([k, l]) => <button class="btn sm" aria-pressed={tab === k} style={{ paddingInline: '6px', lineHeight: 1.2, ...(tab === k ? segOn : {}) }} onClick={() => setTab(k)}>{l}</button>)}</div>}
      {tab === 'calc' && <Calc />}{tab === 'time' && <Schedule />}{tab === 'bridge' && <Bridge />}
      {tab === 'x' && (q ? (
        <div class="stack">
          <span class="small muted num">Question {i + 1} of {QS.length}</span>
          <strong>{q.q}</strong>
          <div class="stack" role="group" aria-label="Answers">{q.opts.map((o, k) => (
            <button class={`btn sm ${pick !== null && k === q.a ? 'primary' : ''}`} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick !== null && k === q.a ? { opacity: 1 } : {}), ...(pick === k && k !== q.a ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}) }} disabled={pick !== null} onClick={() => answer(k)}>{pick !== null && (k === q.a ? '✓ ' : k === pick ? '✗ ' : '')}{o}</button>))}</div>
          {vis()}
          {pick !== null && <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
            <div class="verdict">{ok ? 'Right' : 'Not quite'}</div>
            <p class="small">{!ok && q.bad && <><strong>What happens:</strong> {q.bad} </>}{!ok && q.vis === 'tie' && !q.bad && <><strong>What happens:</strong> {q.len ? 'More straps than needed is fine — but the test asks for the minimum.' : ''} </>}{q.why} <P p={q.p} /></p>
            <button class="btn primary sm" onClick={next}>{i + 1 === QS.length ? 'Finish' : 'Next'}</button></div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 6 right — stamp earned' : `${QS.length - misses} of ${QS.length} right`}</div>
          <button class="btn sm" onClick={reset}>Try again</button></div>
      ))}
    </div>
  );
}
