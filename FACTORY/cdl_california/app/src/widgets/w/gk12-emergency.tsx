import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk12-emergency', title: 'Emergency decisions', lesson: 'GK-12', anchor: /steering to avoid an accident/i,
  summary: 'Six emergencies seen from above. Pick your move and watch the path it gives you — obstacle, oncoming driver, shoulder, blowout, ABS, drive-wheel skid.',
  stamp: { id: 'cool-in-a-crisis', name: 'Cool in a crisis', rule: 'Make all 8 emergency decisions correctly with no mistakes.' },
};

type SceneId = 'obstacle' | 'oncoming' | 'shoulder' | 'blowout' | 'abs' | 'skid';
const P: Record<string, { d: string; end: [number, number]; tag: string }> = {
  straight: { d: 'M215 150 L215 84', end: [215, 84], tag: 'crash' },
  around: { d: 'M215 150 C215 112 268 112 268 76 L268 18', end: [268, 18], tag: 'clear' },
  swerveSkid: { d: 'M215 150 C215 118 262 110 300 70 L336 50', end: [336, 50], tag: 'skid' },
  left: { d: 'M215 150 C215 115 146 115 146 80 L146 44', end: [146, 44], tag: 'head-on' },
  shoulderGood: { d: 'M215 150 C215 124 256 120 256 90 L256 40', end: [256, 40], tag: 'stopped' },
  shoulderSkid: { d: 'M215 150 C215 124 262 118 282 84 L318 40', end: [318, 40], tag: 'skid' },
  edgeBack: { d: 'M215 150 C215 124 264 118 264 90 C264 70 240 58 146 36', end: [146, 36], tag: 'lost control' },
  blowGood: { d: 'M215 150 C215 104 222 90 258 62 L258 34', end: [258, 34], tag: 'stopped' },
  blowBad: { d: 'M215 150 C215 120 190 100 142 64', end: [142, 64], tag: 'lost control' },
  absGood: { d: 'M215 150 L215 106', end: [215, 106], tag: 'stopped' },
  absLong: { d: 'M215 150 L215 86', end: [215, 86], tag: 'crash' },
  stab: { d: 'M215 150 L212 124 L218 112 L215 100', end: [215, 100], tag: 'wrong method' },
  countersteer: { d: 'M215 150 C214 118 218 96 215 30', end: [215, 30], tag: 'back in line' },
  jackknife: { d: 'M215 150 C215 120 246 108 300 104', end: [300, 104], tag: 'jackknife' },
  power: { d: 'M215 150 C215 120 250 110 290 96', end: [290, 96], tag: 'drive wheels spin' },
  noCounter: { d: 'M215 150 C215 118 204 98 146 86', end: [146, 86], tag: 'skids other way' },
};

/** Situation caption for each scene, drawn in the empty verge on the left so the scene reads before any tap. */
const CAPTION: Record<SceneId, string[]> = {
  obstacle: ['Stalled car', 'in your lane.', 'Not enough', 'room to stop.'],
  oncoming: ['Oncoming car', 'drifted into', 'your lane.'],
  shoulder: ['Crash ahead.', 'You must', 'leave the', 'road.'],
  blowout: ['Bang! A front', 'tire blew out.', 'Steering', 'feels heavy.'],
  abs: ['Car pulls out.', 'Tractor: ABS.', 'Trailer: no', 'ABS.'],
  skid: ['Drive wheels', 'locked. The', 'rear is', 'sliding out.'],
};

function Scene({ id, path, ok }: { id: SceneId; path?: string; ok?: boolean }) {
  const p = path ? P[path] : null;
  const col = ok ? 'var(--ok)' : 'var(--red)';
  const rot = id === 'skid' ? 18 : 0;
  const lab = { 'font-size': 14, stroke: 'var(--surface)', 'stroke-width': 3, 'paint-order': 'stroke' } as const;
  const desc: Record<SceneId, string> = {
    obstacle: 'A stalled car blocks your lane; not enough room to stop; the shoulder on your right is clear.',
    oncoming: 'An oncoming car has drifted into your lane.',
    shoulder: 'A crash blocks the road ahead; you must leave the road onto the shoulder.',
    blowout: 'Bang: a front tire blows out and the steering feels heavy.',
    abs: 'A car pulls out ahead. Your tractor has ABS; the trailer does not.',
    skid: 'Your rear drive wheels have locked under braking; the rear is sliding out.',
  };
  return (
    <svg viewBox="0 0 360 230" width="100%" style={{ display: 'block', maxWidth: '460px', marginInline: 'auto' }} role="img" aria-label={`Top view, you drive up the right lane. ${desc[id]}${p ? ` Your path ends: ${p.tag}.` : ''}`}>
      <rect x="0" y="0" width="360" height="230" fill="var(--surface-2)" />
      <rect x="110" y="0" width="140" height="230" fill="var(--surface)" stroke="var(--ink-2)" />
      <rect x="250" y="0" width="50" height="230" fill="var(--amber-soft)" stroke="var(--ink-2)" />
      {[20, 70, 120, 170, 210].map((y) => <g><circle cx="262" cy={y} r="2" fill="var(--ink-2)" /><circle cx="286" cy={y + 22} r="2" fill="var(--ink-2)" /></g>)}
      <line x1="180" y1="0" x2="180" y2="230" stroke="var(--amber)" stroke-width="2" stroke-dasharray="14 10" />
      <text x="4" y="24" font-size="14" font-weight="700" fill="var(--ink)">{CAPTION[id].map((l, i) => <tspan x="4" dy={i ? 18 : 0}>{l}</tspan>)}</text>
      <text x="145" y="224" font-size="14" fill="var(--ink-2)" text-anchor="middle">oncoming</text>
      <text x="275" y="224" font-size="14" fill="var(--ink-2)" text-anchor="middle">shoulder</text>
      <text x="330" y="16" font-size="14" fill="var(--ink-2)" text-anchor="middle">off</text><text x="330" y="32" font-size="14" fill="var(--ink-2)" text-anchor="middle">road</text>
      {id === 'obstacle' && <g><rect x="200" y="46" width="30" height="24" rx="5" fill="var(--amber)" stroke="var(--ink)" /><text x="215" y="40" text-anchor="middle" {...lab} font-weight="700" fill="var(--ink)">stalled car</text>
        <text x="275" y="112" text-anchor="middle" {...lab} fill="var(--ok)" font-weight="700">clear</text></g>}
      {id === 'oncoming' && <g><rect x="198" y="40" width="26" height="40" rx="6" fill="var(--red-soft)" stroke="var(--red)" stroke-width="2" /><path d="M200 38 L186 12" stroke="var(--red)" stroke-width="2" fill="none" /><text x="226" y="98" {...lab} font-weight="700" fill="var(--red)">drifted in ↓</text></g>}
      {id === 'shoulder' && <g><g transform="rotate(-24 200 50)"><rect x="170" y="40" width="30" height="20" rx="4" fill="var(--amber)" stroke="var(--ink)" /><rect x="202" y="38" width="36" height="24" rx="4" fill="var(--red-soft)" stroke="var(--red)" stroke-width="2" /></g><text x="200" y="96" text-anchor="middle" {...lab} font-weight="700" fill="var(--red)">crash</text></g>}
      {id === 'abs' && <g><rect x="190" y="58" width="44" height="24" rx="5" fill="var(--amber)" stroke="var(--ink)" /><text x="212" y="52" text-anchor="middle" {...lab} fill="var(--ink)">pulls out</text></g>}
      {id === 'blowout' && <g><path d="M 197 160 l -8 -6 l 2 8 l -9 1 l 8 5 l -6 7 l 10 -3" fill="var(--red)" stroke="var(--red)" stroke-width="1.5" /><text x="176" y="140" text-anchor="end" {...lab} font-weight="700" fill="var(--red)">BANG!</text></g>}
      {p && <g>
        <path d={p.d} fill="none" stroke={col} stroke-width="4" stroke-dasharray={ok ? '0' : '8 6'} stroke-linecap="round" />
        <circle cx={p.end[0]} cy={p.end[1]} r="11" fill={col} />
        <text x={p.end[0]} y={p.end[1] + 5} font-size="14" font-weight="700" text-anchor="middle" fill="var(--surface)">{ok ? '✓' : '✕'}</text>
        <text x={Math.min(Math.max(p.end[0], 60), 300)} y={p.end[1] + 28} font-size="14" font-weight="700" text-anchor="middle" fill={col} stroke="var(--surface)" stroke-width="4" paint-order="stroke">{p.tag}</text>
      </g>}
      <g transform={`rotate(${rot} 215 180)`}>
        <rect x="199" y="150" width="32" height="28" rx="4" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x="203" y="153" width="24" height="8" rx="2" fill="var(--surface)" />
        <rect x="199" y="180" width="32" height="48" rx="2" fill="var(--surface-2)" stroke="var(--ink)" stroke-width="1.5" />
        <text x="215" y="209" font-size="14" font-weight="700" text-anchor="middle" fill="var(--ink)">YOU</text>
      </g>
      {id === 'skid' && <text x="236" y="214" {...lab} fill="var(--red)" font-weight="700">← rear sliding</text>}
    </svg>
  );
}

interface Opt { t: string; ok?: boolean; path?: string; why: string }
interface Dec { scene?: SceneId; q: string; opts: Opt[]; rule: string; page: string }

/* Explore: one decision per scene */
const EXPLORE: Record<SceneId, { label: string; dec: Dec }> = {
  obstacle: { label: 'Obstacle ahead', dec: { scene: 'obstacle', q: 'A stalled car blocks your lane and there is not enough room to stop. The shoulder is clear.', page: '2-38, 2-39',
    rule: 'Stopping is not always safest: turning to miss an object almost always takes less time than stopping. Do not brake while turning, turn only as much as needed, and be ready to countersteer.',
    opts: [
      { t: 'Steer right around it — no braking in the turn, then countersteer', ok: true, path: 'around', why: 'Turning away is faster than stopping a heavy truck. With a clear shoulder, going right may be best: nobody is likely driving there.' },
      { t: 'Jam on the brakes and hold them', path: 'straight', why: 'A heavy truck needs a long distance to stop, and jamming the brakes only keeps the wheels locked. There is not enough room.' },
      { t: 'Brake hard while you swerve', path: 'swerveSkid', why: 'Wheels lock very easily during a turn. Locked wheels skid and you lose control. Top-heavy rigs may flip.' },
    ] } },
  oncoming: { label: 'Oncoming driver', dec: { scene: 'oncoming', q: 'An oncoming driver has drifted into your lane.', page: '2-39',
    rule: 'Move to your right. When the other driver notices the mistake, they will most likely swerve back into their own lane — on your left.',
    opts: [
      { t: 'Steer to the right', ok: true, path: 'around', why: 'You move away from where the other driver is most likely to go.' },
      { t: 'Steer left, into the empty oncoming lane', path: 'left', why: 'That is the lane the other driver will swerve back into — head-on.' },
      { t: 'Brake hard and hold your lane', path: 'straight', why: 'Not enough room to stop, and a jammed pedal locks the wheels.' },
    ] } },
  shoulder: { label: 'Leaving the road', dec: { scene: 'shoulder', q: 'You have to drive onto the shoulder to avoid a crash. What do you do once you are on it?', page: '2-39',
    rule: 'Avoid braking until your speed is down to about 20 mph, then brake very gently. Keep one set of wheels on the pavement if you can, and stay on the shoulder until you stop.',
    opts: [
      { t: 'Hold off braking until about 20 mph, then brake very gently; keep one set of wheels on the pavement', ok: true, path: 'shoulderGood', why: 'The loose shoulder can’t take hard braking; wheels on the pavement help you keep control. Signal and check mirrors before pulling back on.' },
      { t: 'Brake hard right away so you stop before the shoulder ends', path: 'shoulderSkid', why: 'Hard braking on a loose surface causes a skid.' },
      { t: 'Ease back onto the road slowly while still moving', path: 'edgeBack', why: 'Edging back lets the tires grab the pavement edge — you can lose control. If you must return while moving, turn sharply back on and countersteer as soon as both front tires are on the pavement.' },
    ] } },
  blowout: { label: 'Tire blowout', dec: { scene: 'blowout', q: 'Loud bang. The steering suddenly feels heavy. Nothing is directly ahead.', page: '2-40',
    rule: 'Hold the steering wheel firmly and stay off the brake until the vehicle has slowed down; then brake very gently, pull off the road, stop, and check all tires.',
    opts: [
      { t: 'Hold the wheel firmly; stay off the brake until slowed, then brake gently and pull off', ok: true, path: 'blowGood', why: 'Heavy steering means a front tire — it can twist the wheel out of your hands. Braking after a tire failure can make you lose control.' },
      { t: 'Brake hard to stop as fast as possible', path: 'blowBad', why: 'Braking after a tire failure can make you lose control. Stay off the brake unless you are about to hit something.' },
      { t: 'Check your mirrors to see if the bang came from another vehicle', path: 'blowBad', why: 'Assume the bang was yours. While you look, a failed front tire can twist the wheel out of a loose grip.' },
    ] } },
  abs: { label: 'Braking with ABS', dec: { scene: 'abs', q: 'A car pulls out ahead. You have room to stop. Your tractor has ABS; the trailer does not.', page: '2-39, 2-42',
    rule: 'Brake the way you always have: only as much force as you need, the same whether ABS is on the tractor, the trailer, or both. Watch the trailer and ease off if it swings out. ABS does not necessarily shorten stopping distance. Stab braking is only for vehicles without ABS.',
    opts: [
      { t: 'Brake normally; watch the trailer and ease off if it starts to swing', ok: true, path: 'absGood', why: 'Tractor ABS keeps your steering control and makes a jackknife less likely, but the trailer can still swing — watch it.' },
      { t: 'Stomp and hold the pedal — ABS will stop you shorter', path: 'absLong', why: 'ABS is for control, not a shorter stop. It may or may not stop you sooner. Full brake application is only for working ABS on ALL axles in an emergency.' },
      { t: 'Stab brake: full on, release when wheels lock, repeat', path: 'stab', why: 'Stab braking is only for vehicles WITHOUT ABS.' },
    ] } },
  skid: { label: 'Drive-wheel skid', dec: { scene: 'skid', q: 'You braked hard; the rear drive wheels locked and the rear is swinging out.', page: '2-43',
    rule: 'Stop braking so the rear wheels roll again and stop sliding. Then countersteer: as the vehicle comes back into line, turn the wheel quickly the other way, or you will skid in the opposite direction.',
    opts: [
      { t: 'Stop braking, then countersteer quickly as it comes back in line', ok: true, path: 'countersteer', why: 'Rolling wheels grip better than locked ones; the countersteer stops it rotating past straight.' },
      { t: 'Keep braking and steer away from the skid', path: 'jackknife', why: 'Locked wheels keep sliding; the trailer can push the tractor sideways into a sudden jackknife.' },
      { t: 'Stop braking but hold the wheel where it is', path: 'noCounter', why: 'The vehicle keeps rotating past straight and skids the opposite way.' },
    ] } },
};
const ORDER: SceneId[] = ['obstacle', 'oncoming', 'shoulder', 'blowout', 'abs', 'skid'];

/* Challenge: 8 decisions (different wording from Explore) */
const QS: Dec[] = [
  { scene: 'obstacle', q: 'Stalled car in your lane, no room to stop, shoulder clear. Which is usually faster: stopping or steering around?', page: '2-38', rule: EXPLORE.obstacle.dec.rule,
    opts: [{ t: 'Steering around — turning is almost always faster than stopping', ok: true, path: 'around', why: 'Right — and don’t brake while you turn.' }, { t: 'Stopping — braking is always the safest move', path: 'straight', why: 'Stopping is not always the safest choice; a heavy truck needs a long distance to stop.' }, { t: 'They take the same time', path: 'straight', why: 'Turning away almost always takes less time than stopping.' }] },
  { scene: 'oncoming', q: 'An oncoming car drifts into your lane. Which way do you steer?', page: '2-39', rule: EXPLORE.oncoming.dec.rule,
    opts: [{ t: 'Left', path: 'left', why: 'The other driver will most likely swerve back into their own lane — your left.' }, { t: 'Right', ok: true, path: 'around', why: 'Right — away from where they will return.' }, { t: 'Neither — brake hard and hold the lane', path: 'straight', why: 'Not enough room, and locked wheels mean no control.' }] },
  { scene: 'shoulder', q: 'You are on the shoulder at highway speed. When may you start braking?', page: '2-39', rule: EXPLORE.shoulder.dec.rule,
    opts: [{ t: 'Right away, as hard as possible', path: 'shoulderSkid', why: 'Hard braking on the loose shoulder causes a skid.' }, { t: 'Pump the brakes the whole time', path: 'shoulderSkid', why: 'The rule is to avoid braking until about 20 mph.' }, { t: 'When speed is down to about 20 mph — then very gently', ok: true, path: 'shoulderGood', why: 'Right — and keep one set of wheels on the pavement if you can.' }] },
  { q: 'You are forced back onto the road before you can stop. How do you get back on?', page: '2-39', rule: 'Grip the wheel tightly and turn sharply enough to get right back on the road. Do not edge back on slowly. As soon as both front tires are on the pavement, countersteer — “steer-countersteer” is one move.',
    opts: [{ t: 'Edge back on slowly and gradually', path: 'edgeBack', why: 'The tires can suddenly grab the pavement edge and you can lose control.' }, { t: 'Turn sharply back on; countersteer as soon as both front tires are on the pavement', ok: true, why: 'Right — steer-countersteer.' }, { t: 'Turn back on and wait until all wheels are on the pavement to countersteer', why: 'Countersteer immediately once both FRONT tires are on the pavement.' }] },
  { scene: 'blowout', q: 'Bang — the steering feels heavy. What first?', page: '2-40', rule: EXPLORE.blowout.dec.rule,
    opts: [{ t: 'Hold the wheel firmly and stay off the brake', ok: true, path: 'blowGood', why: 'Right — a failed front tire can twist the wheel out of your hands.' }, { t: 'Brake hard', path: 'blowBad', why: 'Braking after a tire failure can make you lose control.' }, { t: 'Loosen your grip so the wheel can find its own line', path: 'blowBad', why: 'A failed front tire can twist the wheel out of a loose grip.' }] },
  { scene: 'abs', q: 'Your tractor has ABS. A car pulls out and you need to stop. How do you brake?', page: '2-42', rule: EXPLORE.abs.dec.rule,
    opts: [{ t: 'Stab braking', path: 'stab', why: 'Stab braking is only for vehicles without ABS.' }, { t: 'The way you always have — only the force you need, watching the trailer', ok: true, path: 'absGood', why: 'Right — brake normally with ABS.' }, { t: 'Pump the pedal rapidly', path: 'stab', why: 'ABS does not change how you brake: brake the way you always have.' }] },
  { q: 'Which statement about ABS is TRUE?', page: '2-41, 2-42', rule: 'ABS keeps the wheels from locking so you keep steering control. It does not necessarily shorten stopping distance, and it neither increases nor decreases your stopping power.',
    opts: [{ t: 'ABS always shortens your stopping distance', why: 'You may or may not stop sooner — ABS is about control.' }, { t: 'ABS increases your total braking power', why: 'ABS is an add-on; your stopping power does not go up or down.' }, { t: 'ABS helps you keep steering control during hard braking', ok: true, why: 'Right — you should still be able to steer around an obstacle while braking.' }] },
  { scene: 'skid', q: 'Your drive wheels lock while braking and the rear slides. What do you do?', page: '2-43', rule: EXPLORE.skid.dec.rule,
    opts: [{ t: 'Keep braking and turn away from the skid', path: 'jackknife', why: 'Locked wheels keep sliding — jackknife risk.' }, { t: 'Accelerate to pull the truck straight', path: 'power', why: 'Too much power spins the drive wheels — another skid cause (over-acceleration).' }, { t: 'Stop braking, then countersteer quickly', ok: true, path: 'countersteer', why: 'Right — wheels roll and grip; countersteer stops the over-rotation.' }] },
];

function Decision({ d, pick, onPick, locked }: { d: Dec; pick: number | null; onPick: (k: number) => void; locked: boolean }) {
  const o = pick !== null ? d.opts[pick] : null;
  return (
    <div class="stack">
      {d.scene && <Scene id={d.scene} path={o?.path} ok={o?.ok} />}
      <strong>{d.q}</strong>
      <div class="stack" role="group" aria-label="Choices" style={{ gap: '8px' }}>{d.opts.map((x, k) => (
        <button class="btn" aria-pressed={!locked ? pick === k : undefined} disabled={locked && pick !== null}
          style={{ justifyContent: 'flex-start', textAlign: 'left', ...((locked ? pick !== null && x.ok : pick === k && x.ok) ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)', opacity: 1 } : pick === k ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}) }}
          onClick={() => onPick(k)}>{(locked ? pick !== null && x.ok : pick === k && x.ok) ? '✓ ' : pick === k ? '✕ ' : ''}{x.t}</button>
      ))}</div>
      {o && (
        <div class={`feedback ${o.ok ? 'good' : 'bad'}`} role="status">
          <div class="verdict">{o.ok ? 'Right' : 'Not this one'}{o.path ? ` — ${P[o.path].tag}` : ''}</div>
          <p class="small">{o.why}</p>
          <p class="small"><strong>Handbook:</strong> {d.rule} <span class="plate">p. {d.page}</span></p>
        </div>
      )}
    </div>
  );
}

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [i, setI] = useState(0), [pick, setPick] = useState<number | null>(null), [miss, setMiss] = useState(0);
  const d = QS[i];
  if (!d) return (
    <div class={`feedback ${miss === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{miss === 0 ? 'All 8 right — Cool in a crisis' : `${QS.length - miss} of ${QS.length} right`}</div>
      <p class="small">{miss === 0 ? 'Steer (right) rather than stop, no brakes on the shoulder above ~20 mph, stay off the brake after a blowout, brake normally with ABS, off the brake and countersteer in a skid.' : 'Replay the scenes, then retry for the stamp.'}</p>
      <div><button class="btn sm" onClick={() => { setI(0); setMiss(0); setPick(null); }}>Try again</button></div></div>
  );
  return (
    <div class="stack">
      <span class="small muted num">Decision {i + 1} of {QS.length}</span>
      <Decision d={d} pick={pick} locked onPick={(k) => { if (pick !== null) return; setPick(k); const ok = !!d.opts[k].ok; if (!ok) setMiss(miss + 1); onEvidence({ concepts, ok }); }} />
      {pick !== null && <div><button class="btn primary sm" onClick={() => { if (i + 1 === QS.length && miss === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === QS.length ? 'Finish' : 'Next decision'}</button></div>}
    </div>
  );
}

export default function Emergency(props: WidgetProps) {
  const [mode, setMode] = useState<'try' | 'check'>('try');
  const [sc, setSc] = useState<SceneId>('obstacle');
  const [pick, setPick] = useState<number | null>(null);
  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'try'} onClick={() => setMode('try')}>Try the moves</button>
        <button role="tab" aria-selected={mode === 'check'} onClick={() => setMode('check')}>Challenge (8)</button>
      </div>
      {mode === 'try' && (
        <div class="stack">
          <div role="group" aria-label="Emergency" style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(136px, 1fr))', gap: '6px' }}>{ORDER.map((id, k) => (
            <button class="btn sm" aria-pressed={id === sc} style={{ justifyContent: 'flex-start', textAlign: 'left', lineHeight: 1.2, paddingInline: '10px', ...(id === sc ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}) }} onClick={() => { setSc(id); setPick(null); }}>{k + 1}. {EXPLORE[id].label}</button>
          ))}</div>
          <Decision d={EXPLORE[sc].dec} pick={pick} locked={false} onPick={setPick} />
          <p class="small muted">Explore freely — tap every choice to see where it takes you. Paths are teaching sketches.</p>
        </div>
      )}
      {mode === 'check' && <Challenge {...props} />}
    </div>
  );
}
