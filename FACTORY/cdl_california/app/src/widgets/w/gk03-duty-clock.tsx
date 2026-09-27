import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk03-duty-clock', title: 'Duty clock: federal vs California', lesson: 'GK-03', anchor: /hours of service\W+federal vs/i,
  summary: 'Pick when you come on duty, paint your driving hours on a 24-hour log, and see where federal (11/14) and California (12/16) rules stop you. Then answer 5 clock checks.',
  stamp: { id: 'clock-master', name: 'Clock master', rule: 'Answer all 5 hours-of-service checks with no mistakes.' },
};

type Cell = 'D' | 'N';
export const RULES = { fed: { name: 'Federal (interstate)', drive: 11, window: 14, color: 'var(--blue)' }, ca: { name: 'California (intrastate)', drive: 12, window: 16, color: 'var(--accent)' } };
type Sys = keyof typeof RULES;

export function clock(h: number): string {
  const hh = ((h % 24) + 24) % 24, ap = hh < 12 ? 'a.m.' : 'p.m.', t = hh % 12 === 0 ? 12 : hh % 12;
  return (hh === 0 ? '12:00 midnight' : hh === 12 ? '12:00 noon' : `${t}:00 ${ap}`) + (h >= 24 ? ' (next day)' : '');
}
const short = (h: number) => { const hh = h % 24; return hh === 0 ? '12a' : hh === 12 ? '12p' : `${hh % 12}${hh < 12 ? 'a' : 'p'}`; };

/** Checks one driving hour (hour index h after coming on duty, covering h..h+1). Assumes you stay on duty all day, as in the handbook example (p. 1-27). */
export function check(log: Cell[], h: number, sys: Sys): string | null {
  if (log[h] !== 'D') return null;
  const r = RULES[sys], driven = log.slice(0, h + 1).filter((c) => c === 'D').length;
  if (h + 1 > r.window) return sys === 'fed' ? 'after the 14th hour on duty' : 'after 16 hours on duty';
  if (driven > r.drive) return `driving hour ${driven}, over ${r.drive}`;
  return null;
}

const CW = 15, W = CW * 24, FS = 14; // 14 font units in a 360-wide viewBox = 11.4 px on a 294 px phone column
/** Stop-time label: "7 p.m.", "midnight", "noon". */
const stopAt = (h: number) => { const hh = ((h % 24) + 24) % 24; return hh === 0 ? 'midnight' : hh === 12 ? 'noon' : `${hh % 12} ${hh < 12 ? 'a.m.' : 'p.m.'}`; };
/** A cross drawn with lines (shape, so an illegal hour is not shown by color alone). */
const Cross = ({ x, y }: { x: number; y: number }) => <path d={`M${x + 3} ${y + 3} l${CW - 6} 12 M${x + CW - 3} ${y + 3} l${-(CW - 6)} 12`} stroke="var(--surface)" stroke-width={2} />;
export function Lanes({ start, log, pick }: { start: number; log: Cell[]; pick?: number }) {
  const rows: Sys[] = ['fed', 'ca'];
  const H = pick !== undefined ? 196 : 178;
  return (
    <svg viewBox={`0 0 ${W} ${H}`} width="100%" role="img" style={{ display: 'block', maxWidth: '430px' }}
      aria-label={`On duty at ${clock(start)}. Federal: no driving after ${clock(start + 14)}. California: no driving after ${clock(start + 16)}.${log.some((c) => c === 'D') ? ` Illegal driving hours: federal ${log.filter((_, h) => check(log, h, 'fed')).length}, California ${log.filter((_, h) => check(log, h, 'ca')).length}.` : ''}`}>
      {Array.from({ length: 24 }, (_, h) => <rect x={h * CW} y={0} width={CW} height={150} fill={h % 2 ? 'var(--surface)' : 'var(--surface-2)'} />)}
      <text x={2} y={15} font-size={FS} font-weight={700} fill="var(--ink)">Your log <tspan font-weight={400} fill="var(--ink-2)">(dark = driving)</tspan></text>
      {log.map((c, h) => <rect x={h * CW + 1} y={21} width={CW - 2} height={18} rx={2} fill={c === 'D' ? 'var(--ink)' : 'var(--surface)'} stroke="var(--ink-2)" stroke-width={0.8} />)}
      {rows.map((s, i) => {
        const r = RULES[s], top = 46 + i * 52, x = r.window * CW;
        return (
          <g>
            <text x={2} y={top + 15} font-size={FS} font-weight={700} fill="var(--ink)">{s === 'fed' ? 'Federal 11 / 14' : 'California 12 / 16'}</text>
            <text x={x + 4} y={top + 15} font-size={FS} font-weight={700} fill="var(--red)">stop {stopAt(start + r.window)}</text>
            <rect x={0} y={top + 21} width={x} height={18} fill={r.color} opacity={0.25} />
            <rect x={x} y={top + 21} width={W - x} height={18} fill="var(--red-soft)" />
            {log.map((_, h) => check(log, h, s) ? <g><rect x={h * CW + 1} y={top + 21} width={CW - 2} height={18} fill="var(--red)" /><Cross x={h * CW} y={top + 22} /></g>
              : log[h] === 'D' ? <rect x={h * CW + 3} y={top + 25} width={CW - 6} height={10} rx={2} fill={r.color} /> : null)}
            <line x1={x} x2={x} y1={top + 2} y2={top + 44} stroke="var(--red)" stroke-width={2.5} />
          </g>
        );
      })}
      {pick !== undefined && <g><line x1={pick * CW} x2={pick * CW} y1={44} y2={150} stroke="var(--amber)" stroke-width={2.5} stroke-dasharray="4 3" /><text x={Math.min(Math.max(pick * CW - 36, 0), W - 76)} y={190} font-size={FS} font-weight={700} fill="var(--amber-ink)">your pick</text></g>}
      {[0, 4, 8, 12, 16, 20].map((h) => <g><line x1={h * CW} x2={h * CW} y1={150} y2={156} stroke="var(--ink-2)" /><text x={h * CW + 1} y={170} font-size={FS} fill="var(--ink-2)">{short(start + h)}</text></g>)}
    </svg>
  );
}

const DEF_LOG: Cell[] = Array.from({ length: 24 }, (_, h) => (h >= 1 && h <= 6) || (h >= 8 && h <= 13) ? 'D' : 'N');

function Weekly() {
  const [days, setDays] = useState<number[]>([10, 10, 10, 10, 10, 10, 10, 8]);
  const [every, setEvery] = useState(false);
  const [farm, setFarm] = useState(false);
  const [restart, setRestart] = useState(false);
  const sum8 = restart ? 0 : days.reduce((a, b) => a + b, 0), sum7 = restart ? 0 : days.slice(1).reduce((a, b) => a + b, 0);
  const fed = every ? { lim: 70, used: sum8, per: '8 days' } : { lim: 60, used: sum7, per: '7 days' };
  const ca = { lim: farm ? 112 : 80, used: sum8, per: '8 days in a row' };
  /** HTML label (wraps on phones) over a 0–120 h bar with the limit as a red line. */
  const bar = (label: string, x: { lim: number; used: number; per: string }, color: string) => {
    const s = 280 / 120, over = x.used >= x.lim;
    return <div class="stack" style={{ gap: '2px' }}>
      <span class="small num"><strong>{label}:</strong> {x.used} of {x.lim} h in {x.per} · <strong style={{ color: over ? 'var(--red)' : 'inherit' }}>{over ? '✕ NO DRIVING' : `${x.lim - x.used} h left`}</strong></span>
      <svg viewBox="0 0 284 22" width="100%" aria-hidden="true" style={{ display: 'block', maxWidth: '430px' }}><rect x={0} y={5} width={280} height={12} fill="var(--surface-2)" /><rect x={0} y={5} width={Math.min(x.used, 120) * s} height={12} fill={over ? 'var(--red)' : color} />
        <line x1={x.lim * s} x2={x.lim * s} y1={0} y2={22} stroke="var(--red)" stroke-width={2.5} /></svg></div>;
  };
  return (
    <div class="card flat stack">
      <strong>Multi-day limit (hours on duty)</strong>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '6px' }}>{days.map((v, i) => (
        <div class="field" style={{ gap: '2px' }}><label class="small" for={`dc-d${i}`}>{i === 7 ? 'Today' : `Day ${i + 1}`}</label>
          <input id={`dc-d${i}`} type="number" inputMode="numeric" min={0} max={24} value={v} style={{ minWidth: 0, width: '100%' }} onInput={(e) => { const n = [...days]; n[i] = Math.min(24, Math.max(0, +(e.target as HTMLInputElement).value || 0)); setDays(n); }} /></div>))}</div>
      <div class="row">
        <label class="toggle" style={{ minHeight: '36px' }}><input type="checkbox" checked={every} onChange={(e) => setEvery((e.target as HTMLInputElement).checked)} />Carrier runs trucks every day (federal 70/8)</label>
        <label class="toggle" style={{ minHeight: '36px' }}><input type="checkbox" checked={farm} onChange={(e) => setFarm((e.target as HTMLInputElement).checked)} />Hauling farm products (CA 112)</label>
        <label class="toggle" style={{ minHeight: '36px' }}><input type="checkbox" checked={restart} onChange={(e) => setRestart((e.target as HTMLInputElement).checked)} />Just had 34+ hours off in a row</label>
      </div>
      {bar('Federal', fed, 'var(--blue)')}{bar('California', ca, 'var(--accent)')}
      <p class="small">Federal: no driving after <strong>60 hours on duty in 7 days</strong>, or <strong>70 in 8 days</strong> if the carrier runs trucks every day; a <strong>34+ hour</strong> break restarts the period. <span class="ca-tag">CA</span> No driving after <strong>80 hours on duty in any 8 days in a row</strong> (farm products <strong>112</strong>); for truck drivers an 8-day period may end when a 34+ hour off-duty period begins. <span class="plate">p. 1-27</span></p>
    </div>
  );
}

function Explore() {
  const [start, setStart] = useState(5);
  const [log, setLog] = useState<Cell[]>(DEF_LOG);
  const fedBad = log.map((_, h) => check(log, h, 'fed')).filter(Boolean), caBad = log.map((_, h) => check(log, h, 'ca')).filter(Boolean);
  const driven = log.filter((c) => c === 'D').length;
  const firstBad = (s: Sys) => { const h = log.findIndex((_, k) => check(log, k, s)); return h < 0 ? '' : `: the ${clock(start + h)} hour is ${check(log, h, s)}`; };
  return (
    <div class="stack">
      <div class="field"><label for="dc-start">Come on duty (after 10 hours off in a row): <strong>{clock(start)}</strong></label>
        <input id="dc-start" type="range" min={0} max={23} value={start} aria-valuetext={clock(start)} onInput={(e) => setStart(+(e.target as HTMLInputElement).value)} /></div>
      <div class="stack" style={{ gap: '4px' }}>
        <span style={{ fontWeight: 700 }}>Tap an hour to switch Driving ↔ On duty (not driving)</span>
        <span class="small muted">D = driving · on = on duty, not driving. 24 hours from when you come on duty.</span>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(max(44px, calc(100% / 12 - 4px)), 1fr))', gap: '4px' }} role="group" aria-label="24 hours after coming on duty">
          {log.map((c, h) => (
            <button class="btn sm num" aria-pressed={c === 'D'} aria-label={`${clock(start + h)} hour: ${c === 'D' ? 'Driving' : 'On duty, not driving'}`} style={{ padding: '2px 0', minWidth: 0, minHeight: '40px', flexDirection: 'column', gap: 0, fontSize: '.75rem', lineHeight: 1.3, ...(c === 'D' ? { background: 'var(--ink)', color: 'var(--surface)', borderColor: 'var(--ink)' } : {}) }}
              onClick={() => { const n = [...log]; n[h] = c === 'D' ? 'N' : 'D'; setLog(n); }}><span>{short(start + h)}</span><span style={{ fontWeight: 700, fontSize: '.85rem' }}>{c === 'D' ? 'D' : 'on'}</span></button>))}
        </div>
        <div class="row"><button class="btn sm" onClick={() => setLog(Array(24).fill('N'))}>Clear driving</button><button class="btn sm" onClick={() => setLog(Array.from({ length: 24 }, (_, h) => (h >= 1 && h <= 15 ? 'D' : 'N')))}>Drive 15 hours straight</button></div>
      </div>
      <Lanes start={start} log={log} />
      <div class="grid2">
        {(['fed', 'ca'] as Sys[]).map((s) => { const bad = s === 'fed' ? fedBad : caBad; const r = RULES[s]; return (
          <div class={`feedback ${bad.length ? 'bad' : 'good'}`} role="status" aria-live="polite"><div class="eyebrow">{r.name}{s === 'ca' && <span class="ca-tag">CA</span>}</div>
            <div class="verdict">No driving after {clock(start + r.window)}</div>
            <p class="small num">Driving: {Math.min(driven, 24)} h logged, max {r.drive}. {bad.length ? `${bad.length} illegal driving hour${bad.length > 1 ? 's' : ''}${firstBad(s)}.` : 'Every driving hour is legal.'}</p></div>); })}
      </div>
      <p class="small">Both systems: after the limit you may still do non-driving work, but you may not drive again until you have had <strong>10 hours off in a row</strong>. This log assumes you stay on duty all day, as in the handbook’s 5:00 a.m. example. <span class="plate">p. 1-27</span></p>
      <Weekly />
    </div>
  );
}

interface Q { text: string; choices: string[]; answer: string; why: string; page: string; start?: number; log?: Cell[]; picks?: Record<string, number> }
const drive = (a: number, b: number): Cell[] => Array.from({ length: 24 }, (_, h) => (h >= a && h <= b ? 'D' : 'N'));
export const QS: Q[] = [
  { text: 'You drive only inside California with California cargo. You come on duty at 5:00 a.m. after 10 hours off and stay on duty all day. What is the latest clock time you may drive?', choices: ['5:00 p.m.', '7:00 p.m.', '9:00 p.m.'], answer: '9:00 p.m.', start: 5, log: Array(24).fill('N'), picks: { '5:00 p.m.': 12, '7:00 p.m.': 14, '9:00 p.m.': 16 },
    why: 'California intrastate: no driving after 16 hours on duty. 5:00 a.m. + 16 hours = 9:00 p.m. (7:00 p.m. is the federal 14th hour.)', page: '1-27' },
  { text: 'Your freight is going to Nevada (interstate). You come on duty at 6:00 a.m. After which clock time may you no longer drive?', choices: ['6:00 p.m.', '8:00 p.m.', '10:00 p.m.'], answer: '8:00 p.m.', start: 6, log: Array(24).fill('N'), picks: { '6:00 p.m.': 12, '8:00 p.m.': 14, '10:00 p.m.': 16 },
    why: 'Federal: no driving after the 14th hour after coming on duty. 6:00 a.m. + 14 = 8:00 p.m. You may still do non-driving work after that.', page: '1-27' },
  { text: 'You drive Los Angeles to Fresno and never leave California, but the freight came from Arizona. After 10 hours off, how many hours may you drive?', choices: ['11 hours', '12 hours', '14 hours'], answer: '11 hours', start: 5, log: drive(1, 12),
    why: 'Cargo from out of state makes you interstate even inside California, so federal rules apply: at most 11 hours of driving. The 12th driving hour below is legal only under California rules.', page: '1-26, 1-27' },
  { text: 'California intrastate truck driver, not hauling farm products. What is the multi-day on-duty limit?', choices: ['60 hours in 7 days', '70 hours in 8 days', '80 hours in any 8 days in a row'], answer: '80 hours in any 8 days in a row',
    why: 'California: no driving after 80 hours on duty in any 8 days in a row (112 for farm products). 60/7 and 70/8 are the federal limits.', page: '1-27' },
  { text: 'California intrastate: bad weather you did not know about at the start of the trip slows you down. What is the most driving you may do?', choices: ['13 hours', '14 hours', '16 hours'], answer: '14 hours',
    why: 'Adverse weather adds up to 2 hours of driving, but in California never more than 14 hours of driving, and never after 16 hours on duty.', page: '1-27' },
];

export default function DutyClock({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'quiz'>('explore');
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<string | null>(null);
  const [misses, setMisses] = useState(0);
  const q = QS[i];
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Run the clock</button><button role="tab" aria-selected={mode === 'quiz'} onClick={() => setMode('quiz')}>5 clock checks</button></div>
      {mode === 'explore' && <Explore />}
      {mode === 'quiz' && (q ? (
        <div class="stack">
          <span class="small muted num">Check {i + 1} of {QS.length}</span>
          <strong>{q.text}</strong>
          <div class="stack" role="group" aria-label="Choices">{q.choices.map((ch) => (
            <button class={`btn ${pick && ch === q.answer ? 'primary' : ''}`} style={{ justifyContent: 'flex-start', ...(pick && ch === pick && ch !== q.answer ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }} disabled={!!pick}
              onClick={() => { setPick(ch); const ok = ch === q.answer; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); }}>{pick && ch === q.answer ? '✓ ' : pick && ch === pick ? '✗ ' : ''}{ch}</button>))}</div>
          {pick && q.start !== undefined && q.log && <Lanes start={q.start} log={q.log} pick={q.picks ? q.picks[pick] : undefined} />}
          {pick && <div class={`feedback ${pick === q.answer ? 'good' : 'bad'}`} role="status"><div class="verdict">{pick === q.answer ? 'Right' : `No: ${q.answer}`}</div>
            {pick !== q.answer && q.picks && <p class="small">Your pick (amber line) {q.picks[pick] > q.picks[q.answer] ? 'would have you driving past the red stop line: illegal.' : 'would park you hours before the rule does.'}</p>}
            <p class="small">{q.why} <span class="plate">p. {q.page}</span></p>
            <button class="btn primary sm" onClick={() => { if (i + 1 === QS.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === QS.length ? 'Finish' : 'Next check'}</button></div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`}><div class="verdict">{misses === 0 ? 'All 5 right. Stamp earned: Clock master' : `${QS.length - misses} of ${QS.length} right`}</div>
          <p class="small">Federal 11 / 14 / 60–70; California 12 / 16 / 80. Both need 10 hours off in a row.</p>
          <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); }}>Check again</button></div>
      ))}
    </div>
  );
}
