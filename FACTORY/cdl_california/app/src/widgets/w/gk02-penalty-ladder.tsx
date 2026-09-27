import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk02-penalty-ladder', title: 'Penalty ladder: violations on a timeline', lesson: 'GK-02', anchor: /summary: every disqualification period/i,
  summary: 'Drop violations onto a 12-year strip and watch the rolling window decide how long you lose your CDL. Then judge 6 driver records.',
  stamp: { id: 'clean-record', name: 'Clean record', rule: 'Judge all 6 driver records correctly with no mistakes.' },
};

export type Kind = 'serious' | 'oos' | 'rr' | 'major' | 'drugfel' | 'alc' | 'phone';
export interface Drop { id: number; kind: Kind; year: number; hz?: boolean }
interface KindInfo { label: string; short: string; window: number | null; color: string; hzLabel?: string }
export const KINDS: Record<Kind, KindInfo> = {
  serious: { label: 'Serious traffic violation in a CMV (e.g. 15+ mph over, following too closely)', short: 'Serious violation', window: 3, color: 'var(--amber)' },
  oos: { label: 'Out-of-service order violation', short: 'Out-of-service violation', window: 10, color: 'var(--red)', hzLabel: 'Hauling HazMat or 16+ passengers' },
  rr: { label: 'Railroad-highway grade crossing violation', short: 'Railroad crossing', window: 3, color: 'var(--blue)' },
  major: { label: 'Major offense (BAC .04+, DUI, test refusal, leaving the scene, felony with a CMV, driving while suspended, fatal negligence)', short: 'Major offense', window: null, color: 'var(--ink)', hzLabel: 'CMV placarded for HazMat' },
  drugfel: { label: 'Felony involving controlled substances, using a CMV', short: 'Drug felony with a CMV', window: null, color: 'var(--ink)' },
  alc: { label: 'Measurable alcohol under .04 (e.g. BAC .02)', short: 'Alcohol under .04', window: null, color: 'var(--amber)' },
  phone: { label: 'Hands-free / texting violation, any vehicle', short: 'Hands-free / texting', window: 3, color: 'var(--accent)' },
};
const KIND_ORDER: Kind[] = ['serious', 'oos', 'rr', 'major', 'drugfel', 'alc', 'phone'];
const Y0 = 2015, NY = 12, COL = 30, W = COL * NY;
const YEARS = Array.from({ length: NY }, (_, i) => Y0 + i);
const ORD = ['', '1st', '2nd', '3rd'];
const ord = (n: number) => ORD[n] ?? `${n}th`;

export interface Outcome { text: string; days: number; disq: boolean; why: string; page: string }
const Y = 365, LIFE = Infinity;
/** Nth violation of the same kind inside the kind's window (drops are dated mid-year; "within N years" = less than N years apart). */
export function countFor(drops: Drop[], d: Drop): number {
  const k = KINDS[d.kind];
  return drops.filter((x) => x.kind === d.kind && (x.year < d.year || (x.year === d.year && x.id <= d.id)) && (k.window === null || d.year - x.year < k.window)).length;
}
export function outcome(drops: Drop[], d: Drop): Outcome {
  const n = countFor(drops, d);
  const nth = ord(n);
  switch (d.kind) {
    case 'serious': return n < 2 ? { text: 'No disqualification yet', days: 0, disq: false, why: `This is the ${nth} serious violation within 3 years. One alone brings no disqualification; it takes 2 in 3 years.`, page: '1-14' }
      : { text: n === 2 ? 'At least 60 days' : 'At least 120 days', days: n === 2 ? 60 : 120, disq: true, why: `${nth} serious violation in a CMV within 3 years: ${n === 2 ? '2 → at least 60 days' : '3 or more → at least 120 days'}. Repeats show a risky pattern.`, page: '1-14' };
    case 'oos': if (d.hz) {
      const t = n === 1 ? ['At least 180 days (180 days to 2 years)', 180] : ['3 to 5 years', 3 * Y];
      return { text: t[0] as string, days: t[1] as number, disq: true, why: `${nth} out-of-service violation within 10 years while hauling HazMat or 16+ passengers (driver included). GK-03 table: 1st 180 days to 2 years; 2nd and 3rd+ 3 to 5 years.`, page: '1-19' };
    }
      return { text: n === 1 ? 'At least 90 days' : n === 2 ? 'At least 1 year' : 'At least 3 years', days: n === 1 ? 90 : n === 2 ? Y : 3 * Y, disq: true, why: `${nth} out-of-service order violation within 10 years (handbook: 90 days / 1 year / 3 years). The order was given because driving right then was unsafe. Note the 10-year window.`, page: '1-14' };
    case 'rr': return { text: n === 1 ? 'At least 60 days' : n === 2 ? 'At least 120 days' : 'At least 1 year', days: n === 1 ? 60 : n === 2 ? 120 : Y, disq: true, why: `${nth} railroad crossing violation within 3 years (60 days / 120 days / 1 year). Unlike serious violations, the first one already costs 60 days. A train cannot stop quickly.`, page: '1-14' };
    case 'major': return n >= 2 ? { text: 'Life', days: LIFE, disq: true, why: 'A 2nd major offense = CDL lost for life. No time window is stated.', page: '1-14' }
      : { text: d.hz ? 'At least 3 years' : 'At least 1 year', days: d.hz ? 3 * Y : Y, disq: true, why: d.hz ? '1st major offense in a CMV placarded for HazMat = at least 3 years.' : '1st major offense = at least 1 year. (Refusing the test counts too: implied consent.)', page: '1-14' };
    case 'drugfel': return { text: 'Life', days: LIFE, disq: true, why: 'Using a CMV for a felony involving controlled substances = life, even the first time.', page: '1-14' };
    case 'alc': return { text: '24 hours out of service', days: 1, disq: false, why: 'Any measurable alcohol under .04 = out of service for 24 hours. It is not a CDL disqualification, but you may not drive.', page: '1-14' };
    case 'phone': return n < 2 ? { text: 'No disqualification yet', days: 0, disq: false, why: `${nth} hands-free/texting violation within 3 years; the handbook lists penalties from the 2nd.`, page: '1-16' }
      : { text: n === 2 ? 'At least 60 days + 1 point' : 'At least 120 days + 1 point', days: n === 2 ? 60 : 120, disq: true, why: `${nth} hands-free/texting violation within 3 years, in any vehicle, your own car included.`, page: '1-16' };
  }
}

const LANE = 60, AX = 24, FS = 14; // font units: 14 in a 360-wide viewBox stays >= 11 px on a 294 px phone column
/** Timeline of drops. HazMat/passenger drops are drawn as a placard diamond (shape, not only color). */
export function Timeline({ drops, kinds, reveal = true, ghost }: { drops: Drop[]; kinds: Kind[]; reveal?: boolean; ghost?: { kind: Kind; days: number } }) {
  const H = kinds.length * LANE + AX;
  const cx = (d: Drop) => (d.year - Y0) * COL + COL / 2 + (drops.filter((x) => x.kind === d.kind && x.year === d.year && x.id < d.id).length * 7);
  const desc = drops.length ? drops.map((d) => `${d.year} ${KINDS[d.kind].short}${d.hz ? ' (HazMat)' : ''}${reveal ? ': ' + outcome(drops, d).text : ''}`).join('; ') : 'empty';
  return (
    <svg viewBox={`0 0 ${W} ${H}`} width="100%" role="img" aria-label={`Timeline 2015 to 2026: ${desc}`} style={{ display: 'block', maxWidth: '420px' }}>
      {YEARS.map((y, i) => <rect x={i * COL} y={0} width={COL} height={H - AX} fill={i % 2 ? 'var(--surface)' : 'var(--surface-2)'} />)}
      {kinds.map((k, li) => {
        const top = li * LANE, info = KINDS[k];
        const mine = drops.filter((d) => d.kind === k).sort((a, b) => a.year - b.year || a.id - b.id);
        const last = mine[mine.length - 1];
        const wStart = last && info.window ? Math.max(Y0, last.year - info.window + 1) : 0;
        const hasLife = reveal && mine.some((d) => outcome(drops, d).days === LIFE);
        return (
          <g>
            <line x1={0} x2={W} y1={top} y2={top} stroke="var(--line)" stroke-width={1} />
            {reveal && last && info.window && <rect x={(wStart - Y0) * COL + 1} y={top + 21} width={(last.year - wStart + 1) * COL - 2} height={22} rx={4} fill="none" stroke={info.color} stroke-width={1.5} stroke-dasharray="4 3" />}
            <text x={4} y={top + 15} font-size={FS} font-weight={700} fill="var(--ink)">{info.short}{reveal && last && info.window ? ` · ${info.window}-yr window` : ''}</text>
            {hasLife && <text x={W - 4} y={top + 15} font-size={FS} font-weight={700} text-anchor="end" fill="var(--red)">LIFE →</text>}
            {mine.map((d) => {
              const o = outcome(drops, d), x = cx(d), cy = top + 32;
              const len = o.days === LIFE ? W - x : Math.max(2, (o.days / Y) * COL);
              return (
                <g>
                  {reveal && o.days > 0 && <rect x={x} y={top + 47} width={Math.min(len, W - x)} height={6} fill={o.disq ? 'var(--red)' : 'var(--amber)'} />}
                  {d.hz ? <rect x={x - 9} y={cy - 9} width={18} height={18} transform={`rotate(45 ${x} ${cy})`} fill={info.color} stroke="var(--red)" stroke-width={2} />
                    : <circle cx={x} cy={cy} r={11} fill={info.color} stroke="var(--surface)" stroke-width={1.5} />}
                  <text x={x} y={cy + 5} font-size={FS} font-weight={700} text-anchor="middle" fill="var(--surface)">{reveal ? countFor(drops, d) : '•'}</text>
                </g>
              );
            })}
            {ghost && ghost.kind === k && last && ghost.days > 0 && <rect x={cx(last)} y={top + 54} width={Math.min(ghost.days === LIFE ? W : (ghost.days / Y) * COL, W - cx(last))} height={5} fill="none" stroke="var(--ink-2)" stroke-dasharray="3 2" />}
          </g>
        );
      })}
      {YEARS.map((y, i) => <text x={i * COL + COL / 2} y={H - 7} font-size={FS} text-anchor="middle" fill="var(--ink-2)">’{String(y).slice(2)}</text>)}
    </svg>
  );
}

interface Case { text: string; drops: Drop[]; choices: string[]; answer: string }
const d = (id: number, kind: Kind, year: number, hz = false): Drop => ({ id, kind, year, hz });
export const CASES: Case[] = [
  { text: 'In 2024 a driver is convicted of following too closely in a CMV. It is the only violation on the record. What happens to the CDL?', drops: [d(1, 'serious', 2024)], choices: ['No disqualification', 'At least 60 days', 'At least 120 days'], answer: 'No disqualification' },
  { text: 'CMV convictions: speeding 15 mph over (2023), then an erratic lane change (2025).', drops: [d(1, 'serious', 2023), d(2, 'serious', 2025)], choices: ['No disqualification', 'At least 60 days', 'At least 120 days'], answer: 'At least 60 days' },
  { text: 'A driver ignored an out-of-service order in 2016 and again in 2024 (not HazMat). What is the penalty for the 2024 violation?', drops: [d(1, 'oos', 2016), d(2, 'oos', 2024)], choices: ['At least 90 days', 'At least 1 year', 'At least 3 years'], answer: 'At least 1 year' },
  { text: 'First-ever violation: the driver did not have room to get all the way across a railroad crossing without stopping.', drops: [d(1, 'rr', 2025)], choices: ['No disqualification', 'At least 60 days', 'At least 120 days'], answer: 'At least 60 days' },
  { text: 'At a roadside check the driver’s BAC is .02 while driving a CMV.', drops: [d(1, 'alc', 2026)], choices: ['Nothing: under the limit', '24 hours out of service', 'CDL lost 1 year'], answer: '24 hours out of service' },
  { text: 'CMV DUI in 2019 (served 1 year). In 2025 the driver leaves the scene of an accident involving a CMV.', drops: [d(1, 'major', 2019), d(2, 'major', 2025)], choices: ['At least 1 year', 'At least 3 years', 'Life'], answer: 'Life' },
];
const DAYS: Record<string, number> = { 'No disqualification': 0, 'At least 60 days': 60, 'At least 120 days': 120, 'At least 90 days': 90, 'At least 1 year': Y, 'At least 3 years': 3 * Y, Life: LIFE, 'Nothing: under the limit': 0, '24 hours out of service': 1, 'CDL lost 1 year': Y };

const pressed = (on: boolean) => (on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {});

function Explore() {
  const [drops, setDrops] = useState<Drop[]>([d(1, 'serious', 2021), d(2, 'rr', 2024)]);
  const [kind, setKind] = useState<Kind>('serious');
  const [hz, setHz] = useState(false);
  const [nid, setNid] = useState(3);
  const add = (year: number) => { setDrops([...drops, { id: nid, kind, year, hz: !!KINDS[kind].hzLabel && hz }]); setNid(nid + 1); };
  const sorted = [...drops].sort((a, b) => a.year - b.year || a.id - b.id);
  const latest = drops.reduce<Drop | null>((m, x) => (!m || x.year > m.year || (x.year === m.year && x.id > m.id) ? x : m), null);
  const lo = latest && outcome(drops, latest);
  const shown = KIND_ORDER.filter((k) => k === kind || drops.some((x) => x.kind === k));
  return (
    <div class="stack">
      <div class="field"><span style={{ fontWeight: 700 }}>1. Pick a violation</span>
        <div class="row" role="group" aria-label="Violation type">{KIND_ORDER.map((k) => <button class="btn sm" aria-pressed={kind === k} style={pressed(kind === k)} title={KINDS[k].label} onClick={() => setKind(k)}>{KINDS[k].short}{k === 'phone' && <span class="ca-tag">CA</span>}</button>)}</div>
        <span class="small muted">{KINDS[kind].label}.</span>
        {KINDS[kind].hzLabel && <label class="toggle" style={{ minHeight: '36px' }}><input type="checkbox" checked={hz} onChange={(e) => setHz((e.target as HTMLInputElement).checked)} />{KINDS[kind].hzLabel}</label>}
      </div>
      <div class="stack" style={{ gap: '6px', maxWidth: '420px', width: '100%' }}>
        <span style={{ fontWeight: 700 }}>2. Tap a year to drop it</span>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(6, 1fr)', gap: '4px' }} role="group" aria-label="Conviction year">
          {YEARS.map((y) => <button class="btn sm num" style={{ padding: '0', minWidth: 0, fontSize: '.85rem' }} aria-label={`Drop ${KINDS[kind].short} in ${y}`} onClick={() => add(y)}>{y}</button>)}
        </div>
        <Timeline drops={drops} kinds={shown} />
        <span class="small muted">Number = which violation it is inside its window (dashed box); a diamond marks HazMat or 16+ passengers. Bar = time out of the truck (1 column = 1 year; red = CDL disqualified, amber = out of service). Drops are dated mid-year. Lanes appear for the violation you picked and any on the record.</span>
      </div>
      {lo && latest && <div class={`card ${lo.disq ? 'warn' : 'tint'}`} role="status" aria-live="polite"><div class="eyebrow">Latest: {latest.year} · {KINDS[latest.kind].short}</div>
        <div style={{ font: '700 1.4rem/1.1 var(--display)' }}>{lo.text}</div><p class="small">{lo.why} <span class="plate">p. {lo.page}</span></p></div>}
      {sorted.length > 0 && <ul class="list" style={{ margin: 0, padding: 0, listStyle: 'none' }}>{sorted.map((x) => (
        <li style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '2px 0' }}><span class="small" style={{ flex: 1, minWidth: 0 }}><strong class="num">{x.year}</strong> {KINDS[x.kind].short}{x.hz ? ` (◆ ${KINDS[x.kind].hzLabel})` : ''} → {outcome(drops, x).text}</span>
          <button class="btn sm ghost" style={{ flex: 'none' }} aria-label={`Remove ${x.year} ${KINDS[x.kind].short}`} onClick={() => setDrops(drops.filter((z) => z.id !== x.id))}>Remove</button></li>))}</ul>}
      <div class="row"><button class="btn sm" onClick={() => setDrops([])}>Clear strip</button></div>
    </div>
  );
}

export default function PenaltyLadder({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'judge'>('explore');
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<string | null>(null);
  const [misses, setMisses] = useState(0);
  const c = CASES[i];
  const kinds = c ? KIND_ORDER.filter((k) => c.drops.some((x) => x.kind === k)) : [];
  const last = c ? c.drops[c.drops.length - 1] : null;
  const truth = c && last ? outcome(c.drops, last) : null;
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Build a record</button><button role="tab" aria-selected={mode === 'judge'} onClick={() => setMode('judge')}>Judge 6 records</button></div>
      {mode === 'explore' && <Explore />}
      {mode === 'judge' && (c && truth ? (
        <div class="stack">
          <span class="small muted num">Record {i + 1} of {CASES.length}</span>
          <strong>{c.text}</strong>
          <Timeline drops={c.drops} kinds={kinds} reveal={!!pick} ghost={pick && pick !== c.answer && last ? { kind: last.kind, days: DAYS[pick] } : undefined} />
          <div class="row" role="group" aria-label="Pick the result">{c.choices.map((ch) => (
            <button class={`btn sm ${pick && ch === c.answer ? 'primary' : ''}`} style={pick && ch === pick && ch !== c.answer ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}} disabled={!!pick}
              onClick={() => { setPick(ch); const ok = ch === c.answer; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); }}>{pick && ch === c.answer ? '✓ ' : pick && ch === pick ? '✗ ' : ''}{ch}</button>))}</div>
          {pick && <div class={`feedback ${pick === c.answer ? 'good' : 'bad'}`} role="status"><div class="verdict">{pick === c.answer ? 'Right' : `No: ${c.answer}`}</div>
            {pick !== c.answer && <p class="small">{DAYS[pick] === 0 ? 'Your pick would let this driver keep driving' : `Your pick (dashed bar) would ${DAYS[pick] > (truth.days || 0) ? 'bench the driver longer than the rule says' : 'put the driver back in the truck too soon'}`}. The solid bar shows the handbook result.</p>}
            <p class="small">{truth.why} <span class="plate">p. {truth.page}</span></p>
            <button class="btn primary sm" onClick={() => { if (i + 1 === CASES.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === CASES.length ? 'Finish' : 'Next record'}</button></div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`}><div class="verdict">{misses === 0 ? 'All 6 right. Stamp earned: Clean record' : `${CASES.length - misses} of ${CASES.length} right`}</div>
          <p class="small">{misses === 0 ? 'You read every window and ladder correctly.' : 'Try again for the stamp. Watch the window: 3 years for serious and railroad, 10 years for out-of-service.'}</p>
          <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); }}>Judge again</button></div>
      ))}
    </div>
  );
}
