import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk08-following-distance', title: 'Following-distance calculator', lesson: 'GK-08', anchor: /^space ahead$/i,
  summary: 'Set your rig’s length and speed and see the seconds of cushion you need, as a timeline. Then size the gap for 6 rigs.',
  stamp: { id: 'safe-cushion', name: 'Safe cushion', rule: 'Get all 6 following-distance checks right with no mistakes.' },
};

/** Heavy vehicle formula (Figure 2.12, p. 2-19): 1 s per 10 ft of length; above 40 mph add 1 s (once). */
export const followSeconds = (lengthFt: number, mph: number) => lengthFt / 10 + (mph > 40 ? 1 : 0);

/** GK-08 worked examples (the handbook itself gives 40 ft, 60 ft, and 50 ft above 40 mph; 30 ft at 55 mph). */
const EXAMPLES = [30, 40, 50, 60, 70];

function Gap({ len, sec, tail, slick, need, label, ask }: { len: number; sec: number; tail?: boolean; slick?: boolean; need?: number; label: string; ask?: boolean }) {
  const truckW = 22 + len * 0.6;
  const more = tail || slick;
  const behind = tail ? 34 : 4;
  const n = Math.max(sec, need ?? 0);
  const per = Math.min(36, (392 - behind - truckW - 30 - (more ? 30 : 0)) / Math.max(n, 1));
  const x0 = behind + truckW;
  const carX = ask ? 362 : x0 + per * sec + (more ? 30 : 0);
  const short = need != null && sec < need;
  return (
    <svg viewBox="0 0 400 118" width="100%" role="img" aria-label={label} style={{ display: 'block' }}>
      <rect x="0" y="30" width="400" height="48" fill="var(--surface-2)" stroke="none" />
      <line x1="0" y1="54" x2="400" y2="54" stroke="var(--ink-2)" stroke-width="1.5" stroke-dasharray="10 8" />
      {tail && <g><rect x="2" y="60" width="26" height="14" rx="4" fill="var(--amber)" stroke="var(--ink)" stroke-width="1.2" /><text x="15" y="96" text-anchor="middle" font-size="11" fill="var(--ink)">tailgater</text></g>}
      <g aria-hidden="true">
        <rect x={behind} y="58" width={truckW - 12} height="18" rx="2" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x={behind + truckW - 12} y="60" width="11" height="14" rx="2" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" />
      </g>
      <text x={behind + truckW / 2} y="96" text-anchor="middle" font-size="12" fill="var(--ink)">{`you · ${len} ft`}</text>
      {Array.from({ length: n }, (_, k) => {
        const inside = k < sec;
        return (
          <g key={k}>
            <rect x={x0 + per * k + 1} y="36" width={per - 2} height="14" fill={inside ? (short ? 'var(--red-soft)' : 'var(--accent-soft)') : 'var(--surface)'} stroke={inside ? (short ? 'var(--red)' : 'var(--accent)') : 'var(--ink-2)'} stroke-width="1" stroke-dasharray={inside ? undefined : '3 2'} />
            <text x={x0 + per * k + per / 2} y="47" text-anchor="middle" font-size="11" font-weight="700" fill="var(--ink)">{k + 1}</text>
          </g>
        );
      })}
      {ask && <text x={(x0 + carX) / 2} y="47" text-anchor="middle" font-size="14" font-weight="700" fill="var(--ink)">? seconds</text>}
      {more && <g><rect x={x0 + per * sec + 2} y="36" width="26" height="14" fill="var(--amber-soft)" stroke="var(--amber)" stroke-width="1.2" stroke-dasharray="3 2" /><text x={x0 + per * sec + 15} y="47" text-anchor="middle" font-size="11" font-weight="700" fill="var(--ink)">+</text></g>}
      <g aria-hidden="true">
        <rect x={carX} y="60" width="26" height="14" rx="4" fill="var(--blue)" stroke="var(--ink)" stroke-width="1.2" />
        <line x1={carX} y1="28" x2={carX} y2="82" stroke="var(--ink)" stroke-width="1" />
        <rect x={carX - 4} y="18" width="8" height="10" fill="var(--amber)" stroke="var(--ink)" stroke-width="1" />
      </g>
      <text x={Math.min(carX + 13, 388)} y="96" text-anchor="middle" font-size="11" fill="var(--ink)">landmark</text>
      <text x={x0} y="114" font-size="12" fill="var(--ink-2)">{ask ? 'count: “one thousand-and-one…” → ?' : `count: “one thousand-and-one…” → ${sec} s${short ? ' (too close)' : ''}`}</text>
    </svg>
  );
}

function Explore() {
  const [len, setLen] = useState(40);
  const [mph, setMph] = useState(55);
  const [tail, setTail] = useState(false);
  const [slick, setSlick] = useState(false);
  const base = len / 10;
  const sec = followSeconds(len, mph);
  return (
    <div class="stack">
      <div class="grid2">
        <div class="field"><label for="fd-len">Vehicle length: <span class="num">{len} ft</span></label>
          <input id="fd-len" type="range" min={20} max={80} step={10} value={len} aria-valuetext={`${len} feet`} onInput={(e) => setLen(+(e.target as HTMLInputElement).value)} /></div>
        <div class="field"><label for="fd-mph">Speed: <span class="num">{mph} mph</span></label>
          <input id="fd-mph" type="range" min={20} max={70} step={5} value={mph} aria-valuetext={`${mph} miles per hour`} onInput={(e) => setMph(+(e.target as HTMLInputElement).value)} /></div>
      </div>
      <Gap len={len} sec={sec} tail={tail} slick={slick} label={`${len}-foot vehicle at ${mph} mph needs at least ${sec} seconds behind the vehicle ahead${tail || slick ? ', plus more' : ''}.`} />
      <div class="card tint" role="status" aria-live="polite">
        <div class="eyebrow">You need at least</div>
        <div class="num" style={{ font: '700 1.6rem/1.1 var(--display)' }}>{sec} seconds{tail || slick ? ' + more' : ''}</div>
        <p class="small num">{len} ft ÷ 10 = {base} s{mph > 40 ? ` + 1 s (over 40 mph) = ${sec} s` : mph === 40 ? ' (at 40 mph: no extra second — it is only added above 40)' : ' (40 mph or slower: no extra second)'}. <span class="plate">p. 2-19</span></p>
        {mph > 40 && <p class="small muted">One extra second in total — not one per 10 mph. {len} ft at 70 mph is still {sec} s.</p>}
      </div>
      <div class="row">
        <label class="toggle"><input type="checkbox" checked={tail} onChange={(e) => setTail((e.target as HTMLInputElement).checked)} />Tailgater behind you</label>
        <label class="toggle"><input type="checkbox" checked={slick} onChange={(e) => setSlick((e.target as HTMLInputElement).checked)} />Slippery road</label>
      </div>
      {tail && <div class="card warn small"><strong>Tailgated? Increase YOUR following distance</strong> (the “+” box). Then you won’t need to brake or swerve suddenly, and the tailgater can pass more easily. Signal early, slow very gradually, don’t speed up, no taillight or brake-light tricks. <span class="plate">p. 2-19</span> <span class="plate">p. 2-20</span></div>}
      {slick && <div class="card warn small"><strong>Slippery road:</strong> you need more space than the rule gives, because it takes longer to stop. <span class="plate">p. 2-19</span></div>}
      <table class="small num" style={{ width: '100%', borderCollapse: 'collapse' }}>
        <caption class="small muted" style={{ textAlign: 'left', paddingBottom: '4px' }}>Worked examples (your row is highlighted)</caption>
        <thead><tr style={{ borderBottom: '1px solid var(--line)' }}><th style={{ textAlign: 'left', padding: '4px' }}>Length</th><th style={{ textAlign: 'right', padding: '4px' }}>40 mph or less</th><th style={{ textAlign: 'right', padding: '4px' }}>Above 40 mph</th></tr></thead>
        <tbody>{EXAMPLES.map((l) => {
          const on = l === len;
          return (
            <tr key={l} style={{ borderBottom: '1px solid var(--line)', background: on ? 'var(--accent-soft)' : 'transparent', fontWeight: on ? 700 : 400 }}>
              <td style={{ padding: '4px' }}>{on ? '▸ ' : ''}{l} ft</td>
              <td style={{ textAlign: 'right', padding: '4px' }}>{l / 10} s</td>
              <td style={{ textAlign: 'right', padding: '4px' }}>{l / 10 + 1} s</td>
            </tr>
          );
        })}</tbody>
      </table>
      <p class="small muted">Measure it: when the vehicle ahead passes a fixed landmark (shadow, pavement marking, sign), count “one thousand-and-one, one thousand-and-two…” until your front reaches it. <span class="plate">p. 2-19</span></p>
    </div>
  );
}

interface Item { len: number; mph: number; note: string }
const ITEMS: Item[] = [
  { len: 40, mph: 35, note: 'City street, 40-ft truck' },
  { len: 60, mph: 55, note: 'Highway, 60-ft rig' },
  { len: 30, mph: 55, note: '30-ft truck at 55 mph' },
  { len: 50, mph: 60, note: '50-ft truck on the freeway' },
  { len: 40, mph: 65, note: '40-ft truck at 65 mph' },
];
const CHOICES = [2, 3, 4, 5, 6, 7, 8];
const TAIL = { q: 'A car is following you too closely. What do you do with your following distance?', opts: ['Increase the space in front of you', 'Flash your brake lights to warn the driver', 'Speed up to open a gap behind you'], a: 0,
  why: 'Increase your following distance: you won’t have to stop or swerve suddenly, and the tailgater can pass. No light tricks; don’t speed up — being tailgated at low speed is safer.' };

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const total = ITEMS.length + 1;
  if (i >= total) return (
    <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 6 right — stamp earned' : `${total - misses} of ${total} right`}</div>
      {misses > 0 && <p class="small">Try again with no mistakes to earn the “Safe cushion” stamp.</p>}
      <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); }}>Try again</button></div>
  );
  const it = ITEMS[i];
  const truth = it ? followSeconds(it.len, it.mph) : TAIL.a;
  const answer = (c: number) => { if (pick !== null) return; setPick(c); const ok = c === truth; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const ok = pick === truth;
  const next = () => { if (i + 1 === total && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); };
  const btnStyle = (c: number) => pick !== null && c === truth ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)' } : pick === c ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {};
  return (
    <div class="stack">
      <span class="small muted num">Rig {i + 1} of {total}</span>
      {it ? <>
        <strong>{it.note}: {it.len} ft long, {it.mph} mph. How many seconds of following distance?</strong>
        <Gap len={it.len} sec={pick ?? 0} ask={pick === null} need={pick !== null ? truth : undefined} label={pick === null ? `${it.len}-foot rig at ${it.mph} mph.` : `Your gap ${pick} seconds; needed ${truth} seconds.`} />
        <div class="row" role="group" aria-label="Seconds">{CHOICES.map((c) => (
          <button class="btn sm num" disabled={pick !== null} aria-pressed={pick === c} style={btnStyle(c)} onClick={() => answer(c)}>{c} s{pick !== null && c === truth ? ' ✓' : pick === c ? ' ✗' : ''}</button>
        ))}</div>
      </> : <>
        <strong>{TAIL.q}</strong>
        <Gap len={40} sec={pick === 0 ? 5 : 3} tail label="Your truck with a tailgater close behind." />
        <div class="stack" role="group" aria-label="Answers">{TAIL.opts.map((o, c) => (
          <button class="btn sm" style={{ justifyContent: 'flex-start', ...btnStyle(c) }} disabled={pick !== null} aria-pressed={pick === c} onClick={() => answer(c)}>{o}{pick !== null && c === truth ? ' ✓' : pick === c ? ' ✗' : ''}</button>
        ))}</div>
      </>}
      {pick !== null && <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
        <div class="verdict">{ok ? 'Right' : it ? `No — ${truth} seconds` : 'No — increase the space ahead'}</div>
        {it ? <p class="small num">{it.len} ft ÷ 10 = {it.len / 10} s{it.mph > 40 ? `, + 1 s because ${it.mph} mph is over 40 = ${truth} s. Only one extra second, however fast.` : ' — 40 mph or slower, so no extra second.'}
          {!ok && pick < truth ? ' Consequence: too close — if the smaller vehicle ahead stops suddenly, you may hit it.' : ''}{!ok && pick > truth ? ' More room is never wrong on the road, but the test answer is the rule.' : ''} <span class="plate">p. 2-19</span></p>
          : <p class="small">{!ok && pick === 1 ? 'Consequence: light tricks can confuse or provoke the driver. ' : ''}{!ok && pick === 2 ? 'Consequence: a crash at higher speed does more harm. ' : ''}{TAIL.why} <span class="plate">p. 2-19</span> <span class="plate">p. 2-20</span></p>}
        <button class="btn primary sm" onClick={next}>{i + 1 === total ? 'Finish' : 'Next'}</button>
      </div>}
    </div>
  );
}

export default function FollowingDistance(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Calculator</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Challenge: 6 rigs</button></div>
      {mode === 'explore' ? <Explore /> : <Challenge {...props} />}
    </div>
  );
}
