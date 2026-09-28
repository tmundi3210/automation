import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'sb01-danger-zones', title: 'Danger zones & mirrors', lesson: 'SB-01', anchor: /^danger zones$/i,
  summary: 'Place a child around the school bus and see the danger zone and which mirror shows them. Then run a stop in the right order.',
  stamp: { id: 'every-child-seen', name: 'Every child seen', rule: 'Answer all 9 danger-zone, mirror and stop checks right the first time.' },
};

type SpotId = 'frontNear' | 'frontFar' | 'door' | 'rightMid' | 'rearTire' | 'rear' | 'leftMid' | 'lane' | 'walk' | 'inside';
type MirrorId = 'flat' | 'convex' | 'cross' | 'overhead';
type See = 'yes' | 'blind';
interface Spot { x: number; y: number; name: string; zone: string; worst?: boolean; see: Partial<Record<MirrorId | 'direct', See>>; note: string; page: string }
/** Figure 10.1 drawn at 5 units per foot: front zone 30 ft (first 12 ft most dangerous), 12 ft each side and behind. */
const S: Record<SpotId, Spot> = {
  frontNear: { x: 180, y: 142, name: 'Just in front of the bumper', zone: 'Front zone, first 12 ft: MOST dangerous', worst: true, see: { cross: 'yes' }, note: 'You can’t see the ground right in front of the bumper directly. The crossview mirrors show it, from the bumper at ground level up to where your direct view starts.', page: '10-1, 10-3' },
  frontFar: { x: 180, y: 62, name: 'About 20 ft ahead', zone: 'Front danger zone (can reach 30 ft)', see: { direct: 'yes' }, note: 'Out here you see the child directly through the windshield. Your direct view and the crossview view should overlap.', page: '10-1, 10-3' },
  door: { x: 226, y: 196, name: 'Right front tire, by the service door', zone: 'Right side zone (12 ft)', see: { cross: 'yes', flat: 'blind' }, note: 'Crossview mirrors show the front tires touching the ground and the area from the front of the bus to the service door. The flat mirror has a blind spot right below and in front of it.', page: '10-2, 10-3' },
  rightMid: { x: 240, y: 272, name: 'Right side, mid-bus', zone: 'Right side zone (12 ft)', see: { flat: 'yes', convex: 'yes' }, note: 'Flat mirrors check for students along the sides. Convex mirrors show the entire side at a wide angle, but not at true size or distance.', page: '10-2' },
  rearTire: { x: 226, y: 348, name: 'At the right rear tire', zone: 'Right side zone (12 ft)', see: { flat: 'yes', convex: 'yes' }, note: 'Flat mirrors show the rear tires touching the ground. Convex mirrors show the front of the rear tires touching the ground.', page: '10-2' },
  rear: { x: 180, y: 402, name: 'Right behind the rear bumper', zone: 'Rear zone, 12 ft: marked MOST dangerous', worst: true, see: { flat: 'blind', overhead: 'blind' }, note: 'No mirror shows this child. The flat mirrors have a blind spot directly behind the rear bumper (50 to 150 ft, up to 400 ft), and the overhead mirror’s blind spot starts at the rear bumper.', page: '10-2, 10-3' },
  leftMid: { x: 120, y: 272, name: 'Left side, mid-bus', zone: 'Left side zone (12 ft) + passing cars', see: { flat: 'yes', convex: 'yes' }, note: 'Seen in the left flat and convex mirrors. The left side also has the danger of passing cars.', page: '10-1, 10-2' },
  lane: { x: 52, y: 236, name: 'In the traffic lane on the left', zone: 'Left of the bus: ALWAYS dangerous (passing cars)', worst: true, see: { convex: 'yes', flat: 'yes' }, note: 'The whole area to the left of the bus is always dangerous because of passing cars. Convex mirrors must show at least 1 traffic lane on each side; flat mirrors watch traffic.', page: '10-1, 10-2' },
  walk: { x: 316, y: 108, name: 'Walking area, well off the right front', zone: 'Walking area in Figure 10.1', see: { direct: 'yes' }, note: 'After getting off, students walk at least 10 ft away from the side of the bus, to a spot where you can plainly see all of them.', page: '10-1, 10-5' },
  inside: { x: 180, y: 240, name: 'Seated inside the bus', zone: 'Inside the bus', see: { overhead: 'yes' }, note: 'The overhead inside mirror is for the students inside: all of them, including the heads of the students right behind you.', page: '10-3' },
};
const IDS = Object.keys(S) as SpotId[];
const LETTER: Record<SpotId, string> = { frontFar: 'A', frontNear: 'B', walk: 'C', door: 'D', lane: 'E', leftMid: 'F', inside: 'G', rightMid: 'H', rearTire: 'I', rear: 'J' };
const MIRRORS: { id: MirrorId; name: string; where: string; must: string[]; page: string }[] = [
  { id: 'flat', name: 'Flat', where: 'Front corners, at the side or front of the windshield', must: ['200 ft (4 bus lengths) behind the bus', 'Along the sides of the bus', 'The rear tires touching the ground'], page: '10-2' },
  { id: 'convex', name: 'Convex', where: 'Below the outside flat mirrors (not true size or distance)', must: ['The entire side, up to the mirror mounts', 'The front of the rear tires touching the ground', 'At least 1 traffic lane on each side'], page: '10-2' },
  { id: 'cross', name: 'Crossview', where: 'Left and right front corners; [CA] required on every school bus', must: ['In front of the bus from the bumper at ground level to where direct view starts (views overlap)', 'Right and left front tires touching the ground', 'From the front of the bus to the service door'], page: '10-2, 10-3' },
  { id: 'overhead', name: 'Overhead inside', where: 'Above the windshield, driver’s side', must: ['The top of the rear window in the top of the mirror', 'All the students, including heads right behind you'], page: '10-3' },
];
const MNAME: Record<MirrorId | 'direct', string> = { flat: 'Flat', convex: 'Convex', cross: 'Crossview', overhead: 'Overhead inside', direct: 'Direct view' };

const COVER: Record<MirrorId, string[]> = {
  flat: ['148,178 148,466 116,466', '212,178 212,466 244,466'],
  convex: ['146,178 146,372 22,340 70,186', '214,178 214,372 338,340 290,186'],
  cross: ['110,110 250,110 250,170 110,170', '212,170 238,170 238,208 212,208', '122,170 148,170 148,200 122,200'],
  overhead: ['154,178 206,178 206,366 154,366'],
};
const MCOLOR: Record<MirrorId, string> = { flat: 'var(--blue)', convex: 'var(--amber)', cross: 'var(--accent)', overhead: 'var(--ok)' };

function Plan({ child, mirror, onSpot, pickable, marked }: { child: SpotId | null; mirror: MirrorId | null; onSpot: (s: SpotId) => void; pickable: boolean; marked?: { id: SpotId; ok: boolean } | null }) {
  const aria = `Top view of a school bus, front at the top. Danger zones: up to 30 ft in front (first 12 ft most dangerous), 12 ft on the left and right sides, 12 ft behind (most dangerous). The area to the left is always dangerous from passing cars. A walking area leads away from the right front.${child ? ` A child is placed: ${S[child].name}.` : ''}${mirror ? ` Showing the ${MNAME[mirror]} mirror view.` : ''}`;
  return (
    <div style={{ position: 'relative', width: '100%', maxWidth: '460px', margin: '0 auto' }}>
      <svg viewBox="0 0 360 470" width="100%" role="img" aria-label={aria} style={{ display: 'block' }}>
        <rect width="360" height="470" fill="var(--surface-2)" />
        <rect x="6" y="6" width="84" height="458" fill="var(--red-soft)" stroke="var(--red)" stroke-dasharray="5 4" />
        <text x="30" y="300" font-size="13" font-weight="700" fill="var(--red)" transform="rotate(-90 30 300)">Danger from passing cars</text>
        <rect x="90" y="20" width="180" height="150" fill="var(--amber-soft)" stroke="var(--amber-ink)" stroke-dasharray="4 3" />
        <rect x="90" y="110" width="180" height="60" fill="var(--red-soft)" stroke="var(--red)" />
        <text x="96" y="36" font-size="13" fill="var(--amber-ink)">front zone: up to 30 ft</text>
        <text x="96" y="126" font-size="13" font-weight="700" fill="var(--red)">first 12 ft: most dangerous</text>
        <rect x="90" y="170" width="60" height="200" fill="var(--amber-soft)" stroke="var(--amber-ink)" stroke-dasharray="4 3" />
        <rect x="210" y="170" width="60" height="200" fill="var(--amber-soft)" stroke="var(--amber-ink)" stroke-dasharray="4 3" />
        <text x="120" y="318" text-anchor="middle" font-size="13" fill="var(--amber-ink)">12 ft</text><text x="240" y="318" text-anchor="middle" font-size="13" fill="var(--amber-ink)">12 ft</text>
        <rect x="90" y="370" width="180" height="60" fill="var(--red-soft)" stroke="var(--red)" />
        <text x="180" y="446" text-anchor="middle" font-size="13" font-weight="700" fill="var(--red)">behind: 12 ft, most dangerous</text>
        <path d="M272 162 L344 84 L356 98 L284 172 Z" fill="var(--accent-soft)" stroke="var(--accent)" /><text x="356" y="74" text-anchor="end" font-size="13" fill="var(--ink)">walking area</text>
        {mirror && COVER[mirror].map((p) => <polygon key={p} points={p} fill={MCOLOR[mirror]} opacity=".35" stroke={MCOLOR[mirror]} stroke-width="1.5" />)}
        {mirror === 'flat' && <><rect x="150" y="370" width="60" height="96" fill="none" stroke="var(--red)" stroke-width="2" stroke-dasharray="3 3" /><text x="180" y="462" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--red)">✕ blind</text></>}
        <rect x="150" y="170" width="60" height="200" rx="6" fill="var(--amber)" stroke="var(--ink)" stroke-width="2" />
        <rect x="154" y="174" width="52" height="16" rx="2" fill="var(--blue-soft)" stroke="var(--ink)" />
        <rect x="156" y="194" width="14" height="14" fill="var(--surface)" stroke="var(--ink)" /><text x="163" y="205" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--ink)">D</text>
        <rect x="207" y="190" width="6" height="22" fill="var(--accent)" stroke="var(--ink)" />
        <g transform="rotate(-90 180 322)"><rect x="134" y="312" width="92" height="20" rx="4" fill="var(--surface)" stroke="var(--ink)" /><text x="180" y="327" text-anchor="middle" font-size="13" font-weight="700" fill="var(--ink)">SCHOOL BUS</text></g><text x="180" y="166" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--ink)">▲ front</text>
        {[146, 214].map((x) => <circle key={x} cx={x} cy="176" r="4" fill="var(--ink)" />)}
        {child && <g transform={`translate(${S[child].x} ${S[child].y})`} aria-hidden="true"><circle r="12" fill="var(--blue)" stroke="var(--ink)" stroke-width="2" /><circle cy="-3" r="4" fill="var(--surface)" /></g>}
      </svg>
      {IDS.map((id) => {
        const on = child === id || marked?.id === id; const bad = marked?.id === id && !marked.ok;
        return (
          <button key={id} class="btn sm" aria-label={`Spot ${LETTER[id]}: ${S[id].name}`} aria-pressed={on} disabled={!pickable && !on}
            onClick={() => onSpot(id)}
            style={{ position: 'absolute', left: `${(S[id].x / 360) * 100}%`, top: `${(S[id].y / 470) * 100}%`, transform: 'translate(-50%, -50%)', minHeight: '32px', minWidth: '32px', width: '32px', height: '32px', padding: 0, borderRadius: '50%', font: '700 .8rem/1 var(--body)',
              opacity: child === id ? 0.35 : 1, background: bad ? 'var(--red)' : on ? 'var(--accent)' : 'var(--surface)', color: bad ? 'var(--on-red)' : on ? 'var(--accent-ink)' : 'var(--ink)', borderColor: 'var(--ink)' }}>{LETTER[id]}</button>
        );
      })}
    </div>
  );
}

function SpotInfo({ id }: { id: SpotId }) {
  const s = S[id];
  return (
    <div class={`card ${s.worst ? 'warn' : 'tint'} stack`} role="status" aria-live="polite" style={{ gap: '6px' }}>
      <div class="eyebrow">Spot {LETTER[id]}: {s.name}</div>
      <strong>{s.worst ? '⚠ ' : ''}{s.zone}</strong>
      <ul class="small" style={{ margin: 0, paddingLeft: '18px' }}>
        {(['flat', 'convex', 'cross', 'overhead', 'direct'] as const).map((m) => s.see[m] && <li key={m}>{s.see[m] === 'yes' ? '✓ Seen in' : '✕ Blind spot of'} <strong>{MNAME[m]}</strong>{m === 'direct' ? '' : ' mirror'}</li>)}
        {!Object.values(s.see).includes('yes') && <li><strong>✕ No mirror shows this child.</strong></li>}
      </ul>
      <p class="small">{s.note} <span class="plate">p. {s.page}</span></p>
    </div>
  );
}

const STEPS = [
  'Amber lights on, at least 200 ft (5–10 s) before the stop',
  'Right turn signal on, about 100–300 ft (3–5 s) before pulling over',
  'Move as far right as you can',
  'Stop with the front bumper at least 10 ft from the students',
  'Park (or Neutral) and set the parking brake',
  'Red lights on when traffic is a safe distance away; stop arm out',
  'Final check that all traffic has stopped, then open the door',
];
const SHUF = [3, 0, 5, 2, 6, 1, 4];
/** Consequence of tapping a step too early (facts p. 10-4). */
const WHY_NOT: Record<number, string> = {
  1: 'The amber lights go on first, at least 200 ft out, to warn drivers the bus is about to stop.',
  2: 'Warn traffic first (amber lights, right signal) before you pull over.',
  3: 'You aren’t pulled over yet. Signal, then move as far right as you can.',
  4: 'The bus is still rolling. Stop first, 10 ft from the students, then Park and set the brake at every stop.',
  5: 'Red lights come only when you are stopped, in Park with the brake set, and traffic is a safe distance away.',
  6: 'Opening now puts students in front of traffic that may not have stopped. Red lights and stop arm first, then a final check.',
};

function Lights({ n }: { n: number }) {
  const amber = n >= 1, red = n >= 6, sig = n >= 2 && n < 6, door = n >= 7, arm = n >= 6;
  const lamp = (x: number, on: boolean, c: string) => <circle cx={x} cy="22" r="8" fill={on ? c : 'var(--surface)'} stroke="var(--ink)" stroke-width="1.5" />;
  return (
    <svg viewBox="0 0 300 136" width="100%" role="img" aria-label={`Bus seen from the front: amber lights ${amber ? 'flashing' : 'off'}, red lights ${red ? 'flashing' : 'off'}, stop arm ${arm ? 'out' : 'in'}, right signal ${sig ? 'on' : 'off'}, door ${door ? 'open' : 'closed'}.`} style={{ maxWidth: '360px', display: 'block' }}>
      <rect width="300" height="136" fill="var(--surface-2)" />
      <rect x="90" y="10" width="120" height="100" rx="10" fill="var(--amber)" stroke="var(--ink)" stroke-width="2" />
      <rect x="100" y="36" width="100" height="36" rx="4" fill="var(--blue-soft)" stroke="var(--ink)" />
      {lamp(106, red, 'var(--red)')}{lamp(124, amber, 'var(--amber-soft)')}{lamp(176, amber, 'var(--amber-soft)')}{lamp(194, red, 'var(--red)')}
      <text x="150" y="59" text-anchor="middle" font-size="13" font-weight="700" fill="var(--ink)">{red ? 'RED on' : amber ? 'AMBER on' : 'lights off'}</text>
      {arm ? <g><line x1="210" y1="60" x2="232" y2="60" stroke="var(--ink)" stroke-width="3" /><polygon points="230,48 242,36 258,36 270,48 270,64 258,76 242,76 230,64" fill="var(--red)" stroke="var(--ink)" /><text x="250" y="61" text-anchor="middle" font-size="12" font-weight="700" fill="var(--on-red)">STOP</text></g>
        : <rect x="210" y="50" width="6" height="24" fill="var(--red)" stroke="var(--ink)" />}
      {sig && <text x="14" y="64" font-size="15" font-weight="700" fill="var(--ink)">◀ right signal</text>}
      <rect x="100" y="78" width="30" height="32" fill={door ? 'var(--surface)' : 'var(--accent)'} stroke="var(--ink)" /><text x="115" y="98" text-anchor="middle" font-size="11.5" font-weight="700" fill={door ? 'var(--ink)' : 'var(--accent-ink)'}>{door ? 'open' : 'door'}</text>
      <text x="6" y="130" font-size="12.5" fill="var(--ink-2)">front view · {n}/{STEPS.length} steps</text>
    </svg>
  );
}

/** Tap the stop steps in order. Reports once when done: ok = no wrong taps. */
function StopOrder({ onDone, reset }: { onDone?: (ok: boolean) => void; reset: number }) {
  const [st, setSt] = useState({ n: 0, wrong: -1, miss: 0, key: reset });
  const s = st.key === reset ? st : { n: 0, wrong: -1, miss: 0, key: reset };
  const tap = (k: number) => {
    if (s.n >= STEPS.length) return;
    if (k === s.n) { const n = s.n + 1; setSt({ ...s, n, wrong: -1 }); if (n === STEPS.length) onDone?.(s.miss === 0); }
    else setSt({ ...s, wrong: k, miss: s.miss + 1 });
  };
  return (
    <div class="stack" style={{ gap: '8px' }}>
      <Lights n={s.n} />
      {s.n > 0 && <ol class="small" aria-label="Steps done" style={{ margin: 0, paddingLeft: '22px' }}>{STEPS.slice(0, s.n).map((t) => <li key={t}><strong>✓</strong> {t}</li>)}</ol>}
      {s.n < STEPS.length && <div class="stack" role="group" aria-label="Stop step choices" style={{ gap: '6px' }}>
        <span class="small muted">Step {s.n + 1}: which comes next?</span>
        {SHUF.filter((k) => k >= s.n).map((k) => <button key={k} class="btn sm" onClick={() => tap(k)} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(s.wrong === k ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }}>{s.wrong === k ? '✕ ' : ''}{STEPS[k]}</button>)}
      </div>}
      {s.wrong >= 0 && <div class="feedback bad" role="status"><div class="verdict">✕ Not yet</div><p class="small">Next is: “{STEPS[s.n]}”. {WHY_NOT[s.wrong] ?? 'You skipped a step.'} <span class="plate">p. 10-4</span></p></div>}
      {s.n === STEPS.length && <div class="feedback good" role="status"><div class="verdict">✓ Safe stop{s.miss ? ` (${s.miss} wrong tap${s.miss > 1 ? 's' : ''})` : ''}</div><p class="small">Stopping 10 ft back makes students walk to the bus, so you can see them move. Put it in Park and set the brake at every stop. <span class="plate">p. 10-4</span></p></div>}
    </div>
  );
}

type Q = { kind: 'place'; q: string; ans: SpotId; why: string; page: string }
  | { kind: 'pick'; q: string; show?: SpotId; opts: string[]; ans: number; why: string; page: string }
  | { kind: 'order'; q: string };
const QS: Q[] = [
  { kind: 'place', q: 'Tap the spot in the most dangerous part of the front danger zone.', ans: 'frontNear', why: 'The front zone can reach 30 ft, and the first 12 ft in front of the bumper is the most dangerous part.', page: '10-1' },
  { kind: 'pick', q: 'A child stops right behind the rear bumper (spot J). Which mirror shows the child?', show: 'rear', opts: ['Flat', 'Convex', 'Crossview', 'Overhead inside', 'None: it is a blind spot'], ans: 4, why: 'Flat mirrors have a blind spot directly behind the rear bumper, and the overhead mirror’s blind spot starts at the rear bumper. The drawing marks this zone most dangerous.', page: '10-2, 10-3' },
  { kind: 'place', q: 'Tap the spot that is ALWAYS dangerous, even past 12 ft from the bus.', ans: 'lane', why: 'The whole area to the left of the bus is always dangerous because of passing cars.', page: '10-1' },
  { kind: 'pick', q: 'A child is at the right front tire, by the service door (spot D). Which mirror is set up to show this?', show: 'door', opts: ['Flat', 'Convex', 'Crossview', 'Overhead inside'], ans: 2, why: 'Crossview mirrors show the front tires touching the ground and the area from the front of the bus to the service door. The flat mirror has a blind spot below and in front of it.', page: '10-3' },
  { kind: 'place', q: 'A student just got off. Tap where the student should walk to.', ans: 'walk', why: 'Students walk at least 10 ft away from the side of the bus, to a spot where you can plainly see all of them.', page: '10-5' },
  { kind: 'pick', q: 'How far can the danger zone reach in front of the bumper?', opts: ['12 ft', '20 ft', '30 ft', '50 ft'], ans: 2, why: 'Up to 30 ft in front. The first 12 ft is the most dangerous; the sides and the rear are 12 ft.', page: '10-1' },
  { kind: 'pick', q: 'You adjust the outside flat mirrors. How far behind the bus must they show?', opts: ['12 ft', '100 ft', '200 ft (4 bus lengths)', '400 ft'], ans: 2, why: 'Flat mirrors: 200 ft (4 bus lengths) behind, along the sides, and the rear tires touching the ground. 400 ft is how far the blind spot behind can reach.', page: '10-2' },
  { kind: 'pick', q: 'A child is beside the bus at mid-length (spot H). In the convex mirror the child looks…', show: 'rightMid', opts: ['Exactly true size and distance', 'Not at true size or distance', 'Hidden: convex mirrors only show traffic'], ans: 1, why: 'Convex mirrors watch the sides at a wide angle, including students beside the bus, but people and objects do not look their true size or distance.', page: '10-2' },
  { kind: 'order', q: 'Make a safe stop: tap the steps in order.' },
];

export default function DangerZones({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'stop' | 'challenge'>('explore');
  const [child, setChild] = useState<SpotId | null>('rear');
  const [mirror, setMirror] = useState<MirrorId | null>(null);
  const [i, setI] = useState(0);
  const [ans, setAns] = useState<{ ok: boolean; spot?: SpotId; opt?: number } | null>(null);
  const [misses, setMisses] = useState(0);
  const [round, setRound] = useState(0);
  const [xRound, setXRound] = useState(0);
  const q = QS[i];
  const answer = (ok: boolean, extra: { spot?: SpotId; opt?: number }) => { if (ans) return; setAns({ ok, ...extra }); if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const next = () => { if (i + 1 === QS.length && misses === 0) onChallenge?.(); setAns(null); setI(i + 1); };
  const m = mirror ? MIRRORS.find((x) => x.id === mirror)! : null;
  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Zones & mirrors</button>
        <button role="tab" aria-selected={mode === 'stop'} onClick={() => setMode('stop')}>Stop sequence</button>
        <button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Challenge: {QS.length} checks</button>
      </div>
      {mode === 'explore' && (
        <div class="stack">
          <p class="small muted">Tap a lettered spot to place a child. Turn on a mirror to see what it covers.</p>
          <div class="row" role="group" aria-label="Show a mirror's view">
            {MIRRORS.map((x) => <button key={x.id} class="btn sm" aria-pressed={mirror === x.id} style={mirror === x.id ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}} onClick={() => setMirror(mirror === x.id ? null : x.id)}>{x.name}</button>)}
          </div>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '16px', alignItems: 'start' }}>
            <Plan child={child} mirror={mirror} onSpot={setChild} pickable />
            <div class="stack">
              {child && <SpotInfo id={child} />}
              {m && <div class="card stack" style={{ gap: '6px' }}><div class="eyebrow">{m.name} mirror</div><p class="small">{m.where}</p><strong class="small">Adjust so you can see:</strong>
                <ul class="small" style={{ margin: 0, paddingLeft: '18px' }}>{m.must.map((t) => <li key={t}>{t}</li>)}</ul><span class="plate" style={{ alignSelf: 'flex-start' }}>p. {m.page}</span></div>}
            </div>
          </div>
        </div>
      )}
      {mode === 'stop' && <div class="stack"><p class="small muted">Approaching a stop to load students. Tap the steps in order; the lights and stop arm change as you go.</p><StopOrder reset={xRound} /><button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={() => setXRound(xRound + 1)}>Start over</button></div>}
      {mode === 'challenge' && (q ? (
        <div class="stack">
          <span class="small muted num">Check {i + 1} of {QS.length}</span>
          <strong>{q.q}</strong>
          {q.kind === 'place' && <Plan child={null} mirror={null} pickable={!ans} marked={ans?.spot ? { id: ans.spot, ok: ans.ok } : null} onSpot={(s) => answer(s === q.ans, { spot: s })} />}
          {q.kind === 'pick' && <>
            {q.show && <Plan child={q.show} mirror={null} pickable={false} onSpot={() => {}} />}
            <div class="row" role="group" aria-label="Answer choices">{q.opts.map((o, k) => (
              <button key={o} class={`btn sm ${ans && k === q.ans ? 'primary' : ''}`} disabled={!!ans} style={ans && ans.opt === k && !ans.ok ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}} onClick={() => answer(k === q.ans, { opt: k })}>{ans && k === q.ans ? '✓ ' : ans && ans.opt === k ? '✕ ' : ''}{o}</button>
            ))}</div></>}
          {q.kind === 'order' && <StopOrder reset={round} onDone={(ok) => answer(ok, {})} />}
          {ans && q.kind !== 'order' && <div class={`feedback ${ans.ok ? 'good' : 'bad'}`} role="status"><div class="verdict">{ans.ok ? '✓ Right' : q.kind === 'place' ? `✕ That was spot ${LETTER[ans.spot!]}. Answer: spot ${LETTER[q.ans]}` : `✕ Answer: ${q.opts[q.ans]}`}</div>
            {q.kind === 'place' && !ans.ok && <p class="small">Spot {LETTER[ans.spot!]} is: {S[ans.spot!].zone}.</p>}
            <p class="small">{q.why} <span class="plate">p. {q.page}</span></p></div>}
          {ans && <button class="btn primary sm" style={{ alignSelf: 'flex-start' }} onClick={next}>{i + 1 === QS.length ? 'Finish' : 'Next check'}</button>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status">
          <div class="verdict">{misses === 0 ? `${QS.length} of ${QS.length} right — stamp earned: Every child seen` : `${QS.length - misses} of ${QS.length} right — need all ${QS.length} for the stamp`}</div>
          <p class="small">Front zone up to 30 ft (first 12 ft worst), 12 ft sides and rear, the left always dangerous. Right behind the bumper, no mirror sees the child. <span class="plate">p. 10-1</span></p>
          <button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={() => { setI(0); setMisses(0); setAns(null); setRound(round + 1); }}>Try again</button>
        </div>
      ))}
    </div>
  );
}
