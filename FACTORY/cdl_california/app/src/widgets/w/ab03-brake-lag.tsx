import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'ab03-brake-lag', title: 'Brake lag: build the stop', lesson: 'AB-03', anchor: /stopping distance and brake lag/i,
  summary: 'Pick a speed and switch the 4 parts of an air-brake stop on and off on a to-scale road (Figure 5.6). Then answer 7 checks.',
  stamp: { id: 'lag-aware', name: 'Lag aware', rule: 'Order the 4 parts and answer every brake-lag check with no mistakes.' },
};

/** Figure 5.6 (DL 650 p. 5-12) as given in AB-03: perception, reaction, brake lag, braking, total (ft). */
export const FIG_5_6: Record<number, [number, number, number, number, number]> = {
  15: [39, 16, 9, 17, 81], 25: [65, 28, 15, 47, 155], 35: [91, 39, 21, 92, 243], 45: [117, 50, 27, 152, 346], 55: [142, 61, 32, 216, 451],
};
const SPEEDS = [15, 25, 35, 45, 55];
type Key = 'P' | 'R' | 'L' | 'B';
const PARTS: { key: Key; name: string; color: string; ink: string; what: string }[] = [
  { key: 'P', name: 'Perception', color: 'var(--blue)', ink: 'var(--surface)', what: 'from seeing the hazard until your brain knows it is a hazard' },
  { key: 'R', name: 'Reaction', color: 'var(--amber)', ink: 'var(--ink)', what: 'from knowing about the hazard until your foot hits the brake' },
  { key: 'L', name: 'Brake lag', color: 'var(--accent)', ink: 'var(--accent-ink)', what: 'pedal is down, but air is still flowing through the lines to the brakes (½ second or more)' },
  { key: 'B', name: 'Braking', color: 'var(--red)', ink: 'var(--on-red)', what: 'from the brakes working until the truck stops' },
];
const X0 = 46, K = 306 / 470; // 470 ft of road = 306 viewBox units, same scale at every speed
const fx = (ft: number) => X0 + ft * K;
const segRow = (n: number) => ({ display: 'grid', gridTemplateColumns: `repeat(${n}, minmax(0, 1fr))`, gap: '6px' });
const segOn = { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' };

function Road({ mph, on, label, marks, pick, hide, hydraulic }: { mph: number; on: Record<Key, boolean>; label: string; marks?: number[]; pick?: number | null; hide?: boolean; hydraulic?: boolean }) {
  const d = FIG_5_6[mph];
  let x = 0;
  const segs = PARTS.map((p, i) => { const s = { p, from: x, len: on[p.key] ? d[i] : 0 }; x += s.len; return s; });
  const total = x;
  return (
    <svg viewBox="0 0 360 150" width="100%" role="img" aria-label={label} style={{ display: 'block', maxWidth: '520px', marginInline: 'auto' }}>
      <rect x="0" y="38" width="360" height="58" fill="var(--surface-2)" stroke="none" />
      <line x1="0" y1="38" x2="360" y2="38" stroke="var(--ink-2)" stroke-width="1.5" />
      <line x1="0" y1="96" x2="360" y2="96" stroke="var(--ink-2)" stroke-width="1.5" />
      <g aria-hidden="true">
        <rect x="2" y="52" width="30" height="30" rx="3" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x="32" y="56" width="13" height="22" rx="3" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x="40" y="59" width="4" height="15" fill="var(--surface)" stroke="none" />
      </g>
      {!hide && segs.map((s) => {
        const w = s.len * K;
        return w > 0 && (
          <g key={s.p.key}>
            <rect x={fx(s.from)} y="56" width={w} height="22" fill={s.p.color} stroke="var(--ink)" stroke-width="1" />
            {w >= 13 && <text x={fx(s.from) + w / 2} y="72" text-anchor="middle" font-size="13" font-weight="700" fill={s.p.ink}>{s.p.key}</text>}
          </g>
        );
      })}
      {!hide && hydraulic && on.L && <g>
        <line x1={fx(total - d[2])} y1="84" x2={fx(total - d[2])} y2="92" stroke="var(--ink)" stroke-width="2" />
        <text x={Math.min(fx(total - d[2]), 318)} y="92" text-anchor="end" font-size="11" fill="var(--ink)" dx="-3">{`no lag: ${total - d[2]} ft`}</text>
      </g>}
      {!hide && <g><line x1={fx(total)} y1="44" x2={fx(total)} y2="90" stroke="var(--ink)" stroke-width="2.5" />
        <text x={Math.max(Math.min(fx(total), 300), 90)} y="28" text-anchor="middle" font-size="15" font-weight="700" fill="var(--ink)">{`Stops: ${total} ft`}</text></g>}
      {pick != null && <g>
        <line x1={fx(pick)} y1="38" x2={fx(pick)} y2="96" stroke={pick === total ? 'var(--ok)' : 'var(--red)'} stroke-width="3" stroke-dasharray="5 3" />
        <text x={Math.max(Math.min(fx(pick), 316), 44)} y="14" text-anchor="middle" font-size="13" font-weight="700" fill={pick === total ? 'var(--ok)' : 'var(--red)'}>{pick === total ? '✓ your line' : '✗ your line'}</text>
      </g>}
      {[0, 100, 200, 300, 400].map((t) => (
        <g key={t} aria-hidden="true"><line x1={fx(t)} y1="96" x2={fx(t)} y2="104" stroke="var(--ink-2)" stroke-width="1" /><text x={fx(t)} y="119" text-anchor="middle" font-size="13" fill="var(--ink-2)">{t}</text></g>
      ))}
      <text x="356" y="119" text-anchor="end" font-size="13" fill="var(--ink-2)">ft</text>
      {marks && marks.map((m, i) => <g key={m}><line x1={fx(m)} y1="38" x2={fx(m)} y2="96" stroke="var(--ink-2)" stroke-width="1.2" stroke-dasharray="2 3" /><text x={fx(m)} y="138" text-anchor="middle" font-size="13" font-weight="700" fill="var(--ink)">{String.fromCharCode(65 + i)}</text></g>)}
    </svg>
  );
}

const ALL: Record<Key, boolean> = { P: true, R: true, L: true, B: true };

function Explore() {
  const [mph, setMph] = useState(55);
  const [on, setOn] = useState<Record<Key, boolean>>(ALL);
  const d = FIG_5_6[mph];
  const total = PARTS.reduce((s, p, i) => s + (on[p.key] ? d[i] : 0), 0);
  const allOn = PARTS.every((p) => on[p.key]);
  return (
    <div class="stack">
      <div class="stack" style={{ gap: '6px' }}>
        <span class="small" style={{ fontWeight: 700 }} id="bl-sp">Speed (dry pavement, good traction and brakes)</span>
        <div role="group" aria-labelledby="bl-sp" style={segRow(5)}>{SPEEDS.map((s) => (
          <button class="btn sm num" aria-pressed={mph === s} style={{ paddingInline: '2px', ...(mph === s ? segOn : {}) }} onClick={() => setMph(s)}>{s} mph</button>
        ))}</div>
      </div>
      <Road mph={mph} on={on} hydraulic label={`Road to scale at ${mph} mph. ${PARTS.filter((p) => on[p.key]).map((p) => `${p.name} ${d[PARTS.indexOf(p)]} feet`).join(', ')}. Total ${total} feet.`} />
      <fieldset class="stack" style={{ border: '1px solid var(--line)', borderRadius: '8px', padding: '8px 10px', gap: '2px', margin: 0 }}>
        <legend class="small" style={{ fontWeight: 700 }}>Build the stop — switch each part on or off</legend>
        {PARTS.map((p, i) => (
          <label key={p.key} class="toggle" style={{ minHeight: '44px', display: 'grid', gridTemplateColumns: 'auto auto 1fr auto', gap: '8px', alignItems: 'center' }}>
            <input type="checkbox" checked={on[p.key]} onChange={(e) => setOn({ ...on, [p.key]: (e.target as HTMLInputElement).checked })} />
            <span aria-hidden="true" style={{ display: 'inline-grid', placeItems: 'center', width: '22px', height: '22px', background: p.color, color: p.ink, border: '1px solid var(--ink)', font: '700 .8rem/1 var(--body)' }}>{p.key}</span>
            <span class="small"><strong>{p.name}</strong>{p.key === 'L' && <span class="muted"> — air brakes only</span>}<br /><span class="muted">{p.what}</span></span>
            <strong class="num small" style={{ opacity: on[p.key] ? 1 : 0.45 }}>{d[i]} ft</strong>
          </label>
        ))}
        <div class="spread small num" style={{ borderTop: '1px solid var(--line)', paddingTop: '6px' }}><strong>Total stopping distance</strong><strong>{total} ft{allOn ? '' : ` (full: ${d[4]} ft)`}</strong></div>
      </fieldset>
      <div class={`card ${on.L ? 'tint' : 'warn'} small`} role="status" aria-live="polite">
        {on.L
          ? <p>{d[0]} + {d[1]} + <strong>{d[2]}</strong> + {d[3]} = <strong>{d[4]} ft</strong>. The <strong>{d[2]} ft</strong> of brake lag is road a car with hydraulic brakes would not need: hydraulic brakes work <strong>instantly</strong>, air brakes take <strong>½ second or more</strong> for the air to flow through the lines. {mph === 55 && <>At 55 mph: lag about <strong>32 ft</strong>, total <strong>over 450 ft</strong>.</>} <span class="plate">p. 5-12</span></p>
          : <p><strong>Brake lag switched off = pretending you have hydraulic brakes.</strong> An air-brake truck cannot skip it: the pedal is down but the brakes are not working yet, so it rolls {d[2]} ft more at {mph} mph and hits whatever is at the {total} ft line. <span class="plate">p. 5-12</span></p>}
      </div>
    </div>
  );
}

interface Q { q: string; opts: string[]; a: number; why: string; bad?: string; place?: number[]; mph?: number }
const ORDER: Key[] = ['P', 'R', 'L', 'B'];
const SHUF: Key[] = ['L', 'B', 'P', 'R'];
const QS: Q[] = [
  { q: 'A hazard appears when your truck front is at 0 ft. At 55 mph with air brakes, at which line do you stop?', opts: ['216 ft', '419 ft', '451 ft'], place: [216, 419, 451], mph: 55, a: 2,
    why: 'Perception 142 + reaction 61 + brake lag 32 + braking 216 = 451 ft — over 450 ft. 419 ft forgets brake lag; 216 ft is only the braking part.', bad: 'At your line the truck is still moving — it rolls on to 451 ft.' },
  { q: 'At 55 mph on dry pavement, about how much distance does brake lag add?', opts: ['32 ft', '61 ft', '142 ft'], a: 0,
    why: 'Brake lag adds about 32 ft at 55 mph. 61 ft is reaction distance and 142 ft is perception distance in Figure 5.6.' },
  { q: 'Why do air brakes have brake lag?', opts: ['The driver needs time to see the hazard', 'Air has to flow through the lines to the brakes', 'The brakes are too hot'], a: 1,
    why: 'Brake lag is the time for the brakes to work after the pedal is pushed, because the air must flow through the lines. Seeing the hazard is perception, not lag.' },
  { q: 'How long is brake lag on air brakes?', opts: ['None — they work instantly', '½ second or more', 'Up to 1 second, then it stops'], a: 1,
    why: 'Air brakes take ½ second or more. Hydraulic brakes (cars, light/medium trucks) work instantly. “Up to 1 second” is how long wheels may take to start rolling in stab braking.' },
  { q: 'Your truck is doing 35 mph with air brakes. What is the total stopping distance in Figure 5.6?', opts: ['222 ft', '243 ft', '346 ft'], place: [222, 243, 346], mph: 35, a: 1,
    why: '91 + 39 + 21 + 92 = 243 ft. 222 ft leaves out the 21 ft of brake lag; 346 ft is the 45 mph total.', bad: 'At your line the truck is still moving — it rolls on to 243 ft.' },
  { q: 'Which is NOT one of the 4 parts of total stopping distance with air brakes?', opts: ['Brake lag distance', 'Following distance', 'Perception distance'], a: 1,
    why: 'The 4 parts are perception + reaction + brake lag + braking. Following distance is the space you keep ahead — not part of the stop.' },
];

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [i, setI] = useState(-1); // -1 = ordering task
  const [built, setBuilt] = useState<Key[]>([]);
  const [wrongKey, setWrongKey] = useState<Key | null>(null);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const n = QS.length + 1;
  const reset = () => { setI(-1); setBuilt([]); setWrongKey(null); setPick(null); setMisses(0); };
  if (i === -1) {
    const tap = (k: Key) => {
      const ok = ORDER[built.length] === k;
      onEvidence({ concepts, ok });
      if (ok) { setBuilt([...built, k]); setWrongKey(null); } else { setMisses(misses + 1); setWrongKey(k); }
    };
    const done = built.length === 4;
    const on = { P: built.includes('P'), R: built.includes('R'), L: built.includes('L'), B: built.includes('B') };
    const wp = wrongKey ? PARTS.find((p) => p.key === wrongKey)! : null;
    const exp = PARTS[built.length];
    return (
      <div class="stack">
        <span class="small muted num">Check 1 of {n}</span>
        <strong>Build a 55 mph air-brake stop: tap the 4 parts in the order they happen.</strong>
        <Road mph={55} on={on} label={`Built so far: ${built.map((k) => PARTS.find((p) => p.key === k)!.name).join(', ') || 'nothing'}.`} />
        <div role="group" aria-label="Parts to place" style={segRow(2)}>{SHUF.map((k) => {
          const p = PARTS.find((x) => x.key === k)!;
          const used = built.includes(k);
          return <button key={k} class="btn sm" disabled={used || done} aria-label={`${p.name}${used ? ', placed' : ''}`} style={{ justifyContent: 'flex-start', ...(used ? { opacity: 0.5 } : wrongKey === k ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }} onClick={() => tap(k)}>
            <span aria-hidden="true" style={{ display: 'inline-grid', placeItems: 'center', width: '20px', height: '20px', marginRight: '6px', background: p.color, color: p.ink, border: '1px solid var(--ink)', font: '700 .75rem/1 var(--body)' }}>{p.key}</span>{p.name}{used ? ' ✓' : ''}</button>;
        })}</div>
        {wp && !done && <div class="feedback bad" role="status"><div class="verdict">Not yet — {wp.name} comes later</div>
          <p class="small">Next is <strong>{exp.name}</strong>: {exp.what}. {wp.key === 'B' ? 'Braking cannot start until the air has reached the brakes.' : wp.key === 'L' ? 'Lag starts only once your foot is on the pedal.' : 'You cannot react before you perceive.'} <span class="plate">p. 5-12</span></p></div>}
        {done && <div class="feedback good" role="status"><div class="verdict">Perception → reaction → brake lag → braking</div>
          <p class="small">142 + 61 + 32 + 216 = <strong>451 ft</strong>. Brake lag sits between your foot hitting the pedal and the brakes working. <span class="plate">p. 5-12</span></p>
          <button class="btn primary sm" onClick={() => setI(0)}>Next</button></div>}
      </div>
    );
  }
  const q = QS[i];
  if (!q) return (
    <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All checks right — stamp earned: Lag aware' : `${misses} mistake${misses > 1 ? 's' : ''} — need a clean run for the stamp`}</div>
      {misses > 0 && <p class="small">Try again with no mistakes to earn the “Lag aware” stamp.</p>}
      <button class="btn sm" onClick={reset}>Try again</button></div>
  );
  const answer = (c: number) => { if (pick !== null) return; setPick(c); const ok = c === q.a; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const ok = pick === q.a;
  return (
    <div class="stack">
      <span class="small muted num">Check {i + 2} of {n}</span>
      <strong>{q.q}</strong>
      {q.place && <Road mph={q.mph!} on={ALL} hide={pick === null} marks={q.place} pick={pick !== null ? q.place[pick] : null} label={pick === null ? 'Road with candidate stop lines A to C.' : `Truck stops at ${FIG_5_6[q.mph!][4]} feet; your line is at ${q.place[pick]} feet.`} />}
      {q.place && pick === null && <p class="small muted">Only the scale shows until you answer. Letters mark the choices.</p>}
      <div role="group" aria-label="Answers" style={{ display: 'grid', gap: '6px' }}>{q.opts.map((o, c) => (
        <button class="btn sm" disabled={pick !== null} aria-pressed={pick === c}
          style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick !== null && c === q.a ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)', opacity: 1 } : pick === c ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}) }}
          onClick={() => answer(c)}>{q.place ? `${String.fromCharCode(65 + c)} · ` : ''}{o}{pick !== null && c === q.a ? ' ✓' : pick === c ? ' ✗' : ''}</button>
      ))}</div>
      {pick !== null && <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
        <div class="verdict">{ok ? 'Right' : `No — ${q.opts[q.a]}`}</div>
        {!ok && q.bad && <p class="small"><strong>Consequence:</strong> {q.bad}</p>}
        <p class="small">{q.why} <span class="plate">p. 5-12</span></p>
        <button class="btn primary sm" onClick={() => { if (i + 1 === QS.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === QS.length ? 'Finish' : 'Next'}</button>
      </div>}
    </div>
  );
}

export default function BrakeLag(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Challenge: 7 checks</button></div>
      {mode === 'explore' ? <Explore /> : <Challenge {...props} />}
    </div>
  );
}
