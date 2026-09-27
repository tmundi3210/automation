import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk07-stopping-distance', title: 'Stopping distance, to scale', lesson: 'GK-07', anchor: /^stopping distance$/i,
  summary: 'Slide the speed and watch perception, reaction and braking distance grow on a to-scale road. Then answer 6 checks.',
  stamp: { id: 'stopping-distance', name: 'Stopping distance', rule: 'Answer all 6 stopping-distance checks with no mistakes.' },
};

/** Figure 2.11 (DL 650 pp. 2-15 – 2-16) as given in GK-07: perception, reaction, braking, total (ft). */
export const FIG_2_11: Record<number, [number, number, number, number]> = {
  15: [39, 16, 17, 72], 25: [65, 28, 47, 140], 35: [91, 39, 92, 222], 45: [117, 50, 152, 319], 55: [142, 61, 216, 419],
};
const SPEEDS = [15, 25, 35, 45, 55];
/** p. 2-16: speed ×2 / ×3 / ×4 (from 20 mph) → impact and braking distance ×4 / ×9 / ×16. No feet are given. */
const RATIO: [number, number][] = [[20, 1], [40, 4], [60, 9], [80, 16]];

const PARTS = [
  { key: 'P', name: 'Perception', color: 'var(--blue)', time: '1¾ s', what: 'eyes see the hazard → brain knows it is a hazard' },
  { key: 'R', name: 'Reaction', color: 'var(--amber)', time: '¾ s to 1 s', what: 'brain knows → foot presses the brake' },
  { key: 'B', name: 'Braking', color: 'var(--red)', time: '—', what: 'brakes working → truck stops (dry road, good brakes)' },
];
const X0 = 58, K = 320 / 450;
const U = (298 - 62) / 16; // ratio bars: one unit = braking distance at 20 mph (no feet given) // road scale: 450 ft of road = 320 viewBox units, same at every speed
const fx = (ft: number) => X0 + ft * K;

function Road({ mph, empty, marks, pick, label, hide }: { mph: number; empty?: boolean; marks?: number[]; pick?: number | null; label: string; hide?: boolean }) {
  const d = FIG_2_11[mph];
  const segs = [0, d[0], d[0] + d[1], d[3]];
  return (
    <svg viewBox="0 0 400 138" width="100%" role="img" aria-label={label} style={{ display: 'block' }}>
      <rect x="0" y="36" width="400" height="58" fill="var(--surface-2)" stroke="none" />
      <line x1="0" y1="36" x2="400" y2="36" stroke="var(--ink-2)" stroke-width="1.5" />
      <line x1="0" y1="94" x2="400" y2="94" stroke="var(--ink-2)" stroke-width="1.5" />
      <g aria-hidden="true">
        <rect x="8" y="50" width="34" height="30" rx="3" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x="42" y="54" width="14" height="22" rx="3" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x="51" y="57" width="4" height="16" fill="var(--surface)" stroke="none" />
      </g>
      {!hide && PARTS.map((p, i) => {
        const w = (segs[i + 1] - segs[i]) * K;
        return (
          <g key={p.key}>
            <rect x={fx(segs[i])} y="54" width={w} height="22" fill={p.color} stroke="var(--ink)" stroke-width="1" />
            {w >= 16 && <text x={fx(segs[i]) + w / 2} y="70" text-anchor="middle" font-size="14" font-weight="700" fill="var(--surface)">{p.key}</text>}
          </g>
        );
      })}
      {empty && <g>
        <rect x={fx(d[3])} y="54" width={Math.min(36, 392 - fx(d[3]))} height="22" fill="var(--red-soft)" stroke="var(--red)" stroke-width="1.5" stroke-dasharray="4 3" />
        <text x={Math.min(fx(d[3]) + 18, 380)} y="70" text-anchor="middle" font-size="14" font-weight="700" fill="var(--red)">+?</text>
      </g>}
      {!hide && <g><line x1={fx(d[3])} y1="42" x2={fx(d[3])} y2="88" stroke="var(--ink)" stroke-width="2.5" />
      <text x={Math.min(fx(d[3]), 330)} y="28" text-anchor="middle" font-size="15" font-weight="700" fill="var(--ink)">{`Stops: ${d[3]} ft`}</text></g>}
      {pick != null && <g>
        <line x1={fx(pick)} y1="36" x2={fx(pick)} y2="94" stroke={pick === d[3] ? 'var(--ok)' : 'var(--red)'} stroke-width="3" stroke-dasharray="5 3" />
        <text x={Math.max(Math.min(fx(pick), 370), 30)} y="14" text-anchor="middle" font-size="14" fill={pick === d[3] ? 'var(--ok)' : 'var(--red)'}>{pick === d[3] ? '✓ your line' : '✗ your line'}</text>
      </g>}
      {[0, 100, 200, 300, 400].map((t) => (
        <g key={t} aria-hidden="true"><line x1={fx(t)} y1="94" x2={fx(t)} y2="102" stroke="var(--ink-2)" stroke-width="1" /><text x={fx(t)} y="117" text-anchor="middle" font-size="14" fill="var(--ink-2)">{t}</text></g>
      ))}
      <text x={fx(450)} y="117" text-anchor="end" font-size="14" fill="var(--ink-2)">ft</text>
      {marks && marks.map((m, i) => <g key={m}><line x1={fx(m)} y1="36" x2={fx(m)} y2="94" stroke="var(--accent)" stroke-width="1" stroke-dasharray="2 3" /><text x={fx(m)} y="134" text-anchor="middle" font-size="13" font-weight="700" fill="var(--accent)">{String.fromCharCode(65 + i)}</text></g>)}
    </svg>
  );
}

function Explore() {
  const [mph, setMph] = useState(55);
  const [empty, setEmpty] = useState(false);
  const [rel, setRel] = useState(40);
  const d = FIG_2_11[mph];
  const relF = RATIO.find((r) => r[0] === rel)![1];
  return (
    <div class="stack">
      <div class="field">
        <label for="sd-mph">Speed: <span class="num">{mph} mph</span></label>
        <input id="sd-mph" type="range" min={15} max={55} step={10} value={mph} aria-valuetext={`${mph} miles per hour`}
          onInput={(e) => setMph(+(e.target as HTMLInputElement).value)} />
        <span class="small muted">Steps match the handbook’s Figure 2.11 speeds (15–55 mph). Ideal conditions: these are minimums.</span>
      </div>
      <Road mph={mph} empty={empty} label={`Road drawn to scale at ${mph} mph: perception ${d[0]} feet, reaction ${d[1]} feet, braking ${d[2]} feet, total ${d[3]} feet${empty ? '; empty truck needs longer' : ''}.`} />
      <table class="small num" style={{ width: '100%', borderCollapse: 'collapse' }}>
        <tbody>
          {PARTS.map((p, i) => (
            <tr key={p.key} style={{ borderBottom: '1px solid var(--line)' }}>
              <td style={{ padding: '4px 6px 4px 0', whiteSpace: 'nowrap' }}><span aria-hidden="true" style={{ display: 'inline-block', width: '12px', height: '12px', background: p.color, border: '1px solid var(--ink)', verticalAlign: '-1px', marginRight: '6px' }} /><strong>{p.key} · {p.name}</strong></td>
              <td style={{ padding: '4px 6px' }}>{p.time}</td>
              <td style={{ padding: '4px 0', textAlign: 'right' }}><strong>{d[i]} ft</strong></td>
            </tr>
          ))}
          <tr><td style={{ padding: '4px 0' }}><strong>Total</strong></td><td colSpan={2} style={{ textAlign: 'right', whiteSpace: 'nowrap' }}><strong>{mph === 55 ? 'at least ' : ''}{d[3]} ft</strong></td></tr>
        </tbody>
      </table>
      <div class="card tint small" role="status" aria-live="polite">
        {mph === 55
          ? <p>142 + 61 = <strong>203 ft</strong> rolled at full speed before the brakes even work — almost half. Add 216 ft braking: <strong>419 ft</strong>. <span class="plate">p. 2-15</span> <span class="plate">p. 2-16</span></p>
          : <p>Perception + reaction: <strong class="num">{d[0] + d[1]} ft</strong> at full speed before the brakes work. Braking: <strong class="num">{d[2]} ft</strong>. <span class="plate">p. 2-16</span></p>}
        <p class="muted">25 → 55 mph: perception and reaction a little more than double; braking grows more than 4 times (47 → 216 ft). At 60 mph the handbook says you need more than a football field.</p>
      </div>
      <label class="toggle"><input type="checkbox" checked={empty} onChange={(e) => setEmpty((e.target as HTMLInputElement).checked)} />Truck is empty</label>
      {empty && <div class="card warn small"><strong>Empty trucks need LONGER to stop.</strong> Brakes, tires, springs and shocks are built to work best fully loaded; empty, the tires have less traction. The handbook gives no feet for this, so the extra is shown as “+?”. <span class="plate">p. 2-16</span></div>}
      <div class="card flat stack">
        <div class="eyebrow">Double the speed → 4× the braking</div>
        <div class="row" role="group" aria-label="Compare speed with 20 mph">{RATIO.map(([s]) => (
          <button class="btn sm" aria-pressed={rel === s} style={rel === s ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}} onClick={() => setRel(s)}>{s} mph</button>
        ))}</div>
        <svg viewBox="0 0 300 64" width="100%" role="img" aria-label={`Braking distance and impact at ${rel} mph are ${relF} times those at 20 mph.`} style={{ maxWidth: '520px' }}>
          <text x="0" y="21" font-size="14" fill="var(--ink)">20 mph</text>
          <rect x="62" y="8" width={U} height="18" fill="var(--red)" stroke="var(--ink)" stroke-width="1" />
          <text x={68 + U} y="22" font-size="14" font-weight="700" fill="var(--ink)">×1</text>
          <text x="0" y="53" font-size="14" fill="var(--ink)">{rel} mph</text>
          <rect x="62" y="40" width={U * relF} height="18" fill="var(--red)" stroke="var(--ink)" stroke-width="1" />
          <text x={relF === 16 ? 56 + U * relF : 68 + U * relF} y="54" font-size="14" font-weight="700" fill={relF === 16 ? 'var(--surface)' : 'var(--ink)'} text-anchor={relF === 16 ? 'end' : 'start'}>{`×${relF}`}</text>
        </svg>
        <p class="small">Speed ×{rel / 20} → impact and braking distance ×{relF} ({rel / 20} × {rel / 20}). Relative sizes only: the handbook gives the ratio, not feet. <span class="plate">p. 2-16</span></p>
      </div>
    </div>
  );
}

interface Q { q: string; opts: string[]; a: number; why: string; page: string; place?: number[] }
const QS: Q[] = [
  { q: 'A hazard appears when your truck front is at 0 ft. Going 55 mph (dry road, alert driver), at which line do you stop?', opts: ['142 ft', '216 ft', '300 ft', '419 ft'], place: [142, 216, 300, 419], a: 3,
    why: 'Perception 142 + reaction 61 + braking 216 = at least 419 ft. 216 ft is only the braking part; 300 ft is the football-field trap.', page: '2-16' },
  { q: 'Your speed doubles. Which part of stopping distance grows about 4 times?', opts: ['Perception distance', 'Reaction distance', 'Braking distance'], a: 2,
    why: 'Braking distance (and impact) grow 4× when speed doubles. Perception and reaction distance only about double.', page: '2-16' },
  { q: 'Same truck, same dry road. Which one needs more room to stop?', opts: ['Empty truck', 'Fully loaded truck', 'They stop the same'], a: 0,
    why: 'Empty trucks need longer: less weight on the tires means less traction. The brakes, tires, springs and shocks work best fully loaded.', page: '2-16' },
  { q: 'About how long does an alert driver take to PERCEIVE a hazard?', opts: ['¾ second', '1¾ seconds', '3 seconds'], a: 1,
    why: 'Perception averages 1¾ s. ¾ to 1 s is REACTION time.', page: '2-15' },
  { q: 'You go from 20 mph to 60 mph. Impact and braking distance become:', opts: ['3 times greater', '6 times greater', '9 times greater'], a: 2,
    why: 'Speed ×3 → 3 × 3 = 9 times greater.', page: '2-16' },
  { q: 'At 55 mph, how far do you travel before the brakes even begin to work?', opts: ['61 ft', '142 ft', '203 ft'], a: 2,
    why: 'Perception 142 ft + reaction 61 ft = 203 ft at full speed — almost half of the 419 ft total.', page: '2-15' },
];

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const q = QS[i];
  if (!q) return (
    <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 6 right — stamp earned' : `${QS.length - misses} of ${QS.length} right`}</div>
      {misses > 0 && <p class="small">Try again with no mistakes to earn the “Stopping distance” stamp.</p>}
      <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); }}>Try again</button></div>
  );
  const answer = (c: number) => { if (pick !== null) return; setPick(c); const ok = c === q.a; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const ok = pick === q.a;
  return (
    <div class="stack">
      <span class="small muted num">Check {i + 1} of {QS.length}</span>
      <strong>{q.q}</strong>
      {q.place && <Road mph={55} hide={pick === null} marks={q.place} pick={pick !== null ? q.place[pick] : null} label={pick === null ? 'Road with candidate stop lines A to D.' : `Truck stops at 419 feet; your line is at ${q.place[pick]} feet.`} />}
      {q.place && pick === null && <p class="small muted">Only the scale is shown until you answer. Letters mark the choices.</p>}
      <div class="row" role="group" aria-label="Answers">{q.opts.map((o, c) => (
        <button class="btn sm" disabled={pick !== null} aria-pressed={pick === c}
          style={pick !== null && c === q.a ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)' } : pick === c ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}}
          onClick={() => answer(c)}>{q.place ? `${String.fromCharCode(65 + c)} · ` : ''}{o}{pick !== null && c === q.a ? ' ✓' : pick === c ? ' ✗' : ''}</button>
      ))}</div>
      {pick !== null && <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
        <div class="verdict">{ok ? 'Right' : `No — ${q.opts[q.a]}`}</div>
        {!ok && q.place && <p class="small"><strong>Consequence:</strong> at your line the truck is still moving — it rolls on to 419 ft and hits whatever is there.</p>}
        <p class="small">{q.why} <span class="plate">p. {q.page}</span></p>
        <button class="btn primary sm" onClick={() => { if (i + 1 === QS.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === QS.length ? 'Finish' : 'Next'}</button>
      </div>}
    </div>
  );
}

export default function StoppingDistance(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Challenge: 6 checks</button></div>
      {mode === 'explore' ? <Explore /> : <Challenge {...props} />}
    </div>
  );
}
