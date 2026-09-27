import { useEffect, useRef, useState } from 'preact/hooks';
import type { JSX } from 'preact';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'tk01-surge-baffles', title: 'Surge lab: bulkheads, baffles & smooth bore', lesson: 'TK-01', anchor: /danger of surge/i,
  summary: 'Pick a tank type, liquid and fill level, then brake, start, warm the load or take a curve and watch the liquid push the truck. Then solve 8 tanker scenarios.',
  stamp: { id: 'surge-master', name: 'Surge master', rule: 'Get all 8 tanker scenarios right with no mistakes.' },
};

/* ---------- Model (TK-01 · pp. 8-2, 8-3). Strengths are illustrative only; the rules shown are the handbook's. */
type Tank = 'bulk' | 'baffle' | 'smooth';
type Liquid = 'milk' | 'fuel' | 'acid';
type Pattern = 'even' | 'front' | 'rear';
type Ev = 'brake' | 'early' | 'start' | 'warm' | 'curveFast' | 'curveSlow';
interface Scene { tank: Tank; fill: number; liquid: Liquid; pattern: Pattern; ice: boolean }
const TANK_NAME: Record<Tank, string> = { bulk: 'Bulkheads', baffle: 'Baffled', smooth: 'Smooth bore (unbaffled)' };
const TANK_NOTE: Record<Tank, string> = {
  bulk: 'Solid walls divide the tank into smaller separate tanks. When loading and unloading, watch weight distribution: not too much weight on the front or rear.',
  baffle: 'Bulkheads with holes let liquid flow through. They control forward-and-back surge, but side-to-side surge can still happen and cause a rollover.',
  smooth: 'Nothing inside slows the liquid, so forward-and-back surge is very strong. Be extremely cautious, especially when starting and stopping. Usually hauls food products like milk.',
};
const LIQ_NAME: Record<Liquid, string> = { milk: 'Milk (food)', fuel: 'Fuel', acid: 'Dense acid' };
const EV_NAME: Record<Ev, string> = { brake: 'Brake to a stop (steady pressure)', early: 'Let off the brakes too soon', start: 'Start from a stop', warm: 'Load warms up', curveFast: 'Curve at the posted speed', curveSlow: 'Curve well below posted speed' };
const STATIC_T: Record<Ev, number> = { brake: 0.62, early: 0.62, start: 0.45, warm: 1, curveFast: 1, curveSlow: 1 };

const sm = (x: number) => { const u = Math.max(0, Math.min(1, x)); return u * u * (3 - 2 * u); };
/** Fore-aft surge strength 0..1: only a partly filled tank surges; baffles slow it most, bulkheads split it, smooth bore does nothing. */
const fillF = (f: number) => (f >= 100 ? 0 : Math.min(1, (100 - f) / 35));
const foreAft = (s: Scene) => fillF(s.fill) * { smooth: 1, bulk: 0.55, baffle: 0.22 }[s.tank];

function side(s: Scene, ev: Ev | null, t: number) {
  const sf = foreAft(s);
  let dx = 0, tilt = 0, push = 0, rise = 0;
  if (ev === 'brake' || ev === 'early') {
    const u = Math.min(1, t / 0.42);
    const P = sf * (ev === 'early' ? 38 : 4) + (s.ice ? sf * 34 : 0);
    push = P * sm((t - 0.42) / 0.18);
    dx = -70 * (1 - u) * (1 - u) + push;
    tilt = t < 0.42 ? 0.9 * sf * sm(u) : 0.9 * sf * Math.cos((2 * Math.PI * (t - 0.42)) / 0.4) * Math.exp(-2.5 * (t - 0.42));
  } else if (ev === 'start') {
    push = -sf * 12 * sm((t - 0.25) / 0.15);
    dx = 40 * t * t + push - 20;
    tilt = t < 0.25 ? -0.9 * sf * sm(t / 0.25) : -0.9 * sf * Math.cos((2 * Math.PI * (t - 0.25)) / 0.4) * Math.exp(-2.5 * (t - 0.25));
  } else if (ev === 'warm') rise = 7 * sm(t / 0.6);
  return { dx, tilt, push, rise, over: ev === 'warm' && s.fill + rise > 100 };
}

/* ---------- Side view */
function SideView({ s, ev, t, mask }: { s: Scene; ev: Ev | null; t: number; mask?: boolean }) {
  const g = side(s, ev, t);
  const X0 = 40, X1 = 236, YT = 58, YB = 122, H = YB - YT;
  const walls = s.tank === 'smooth' ? [] : [X0 + (X1 - X0) / 3, X0 + (2 * (X1 - X0)) / 3];
  const comps: [number, number, number][] = s.tank === 'bulk'
    ? [0, 1, 2].map((i) => { const a = X0 + (i * (X1 - X0)) / 3, b = a + (X1 - X0) / 3; const on = s.pattern === 'even' || (s.pattern === 'front' && i === 2) || (s.pattern === 'rear' && i === 0); return [a, b, on ? s.fill : 0]; })
    : [[X0, X1, s.fill]];
  const liquid = comps.map(([a, b, f], i) => {
    if (f <= 0) return null;
    const lvl = YB - (H * Math.min(f + g.rise, 100)) / 100;
    const r = g.tilt * 34 * ((b - a) / (X1 - X0)) * (s.tank === 'bulk' ? 1.6 : 1);
    return <polygon key={i} points={`${a},${YB + 2} ${a},${lvl + r / 2} ${b},${lvl - r / 2} ${b},${YB + 2}`} fill="var(--blue)" opacity={0.6} />;
  });
  const hitFront = Math.abs(g.push) > 3 && g.push > 0, hitRear = g.push < -3;
  const aria = mask ? 'Side view: tanker with its inside hidden until you answer.' : `Side view: ${TANK_NAME[s.tank]} tanker, ${s.fill}% full${ev ? `, ${EV_NAME[ev]}` : ''}. ${hitFront ? `The surge wave hits the front and pushes the truck forward ${g.push > 20 ? 'past the stop line' : 'a little'}.` : ''}${hitRear ? 'The wave hits the rear and jerks the rig.' : ''}${g.over ? ' Liquid has no room to expand and spills out.' : ''}`;
  return (
    <svg viewBox="0 0 360 180" width="100%" style={{ display: 'block' }} role="img" aria-label={aria}>
      <rect x="0" y="0" width="360" height="180" fill="var(--surface-2)" />
      <rect x="0" y="146" width="360" height="34" fill={s.ice ? 'var(--blue-soft)' : 'var(--surface)'} />
      <text x="6" y="172" font-size="13" fill="var(--ink-2)">{s.ice ? 'ice (slippery)' : 'dry road'}</text>
      {(ev === 'brake' || ev === 'early') && <g><rect x="308" y="146" width="52" height="34" fill="var(--amber-soft)" /><line x1="308" x2="308" y1="118" y2="180" stroke="var(--ink)" stroke-width="3" /><text x="304" y="174" font-size="13" text-anchor="end" fill="var(--ink)">stop line</text><text x="334" y="160" font-size="13" text-anchor="middle" fill="var(--amber-ink)">inter-</text><text x="334" y="174" font-size="13" text-anchor="middle" fill="var(--amber-ink)">section</text></g>}
      <g transform={`translate(${g.dx.toFixed(1)} 0)`}>
        <rect x="30" y="126" width="282" height="8" fill="var(--ink-2)" />
        <path d="M244 72 h40 l20 26 v38 h-60 z" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" />
        <path d="M284 78 l14 20 h-14 z" fill="var(--surface)" stroke="var(--ink)" />
        <defs><clipPath id="tk-clip"><rect x={X0} y={YT} width={X1 - X0} height={H} rx="26" /></clipPath></defs>
        <rect x={X0} y={YT} width={X1 - X0} height={H} rx="26" fill="var(--surface)" />
        <g clip-path="url(#tk-clip)">{liquid}
          {s.fill < 100 && <text x={(X0 + X1) / 2} y={YT + 12} font-size="13" text-anchor="middle" fill="var(--ink-2)">outage</text>}
        </g>
        {walls.map((x, i) => s.tank === 'bulk'
          ? <line key={i} x1={x} x2={x} y1={YT} y2={YB} stroke="var(--ink)" stroke-width="3" />
          : <line key={i} x1={x} x2={x} y1={YT + 2} y2={YB - 2} stroke="var(--ink)" stroke-width="3" stroke-dasharray="8 6" />)}
        {mask && <g><rect x={X0} y={YT} width={X1 - X0} height={H} rx="26" fill="var(--surface)" /><text x={(X0 + X1) / 2} y={(YT + YB) / 2 + 9} font-size="26" font-weight="700" text-anchor="middle" fill="var(--ink-2)">?</text></g>}
        <rect x={X0} y={YT} width={X1 - X0} height={H} rx="26" fill="none" stroke="var(--ink)" stroke-width="2" />
        <rect x={(X0 + X1) / 2 - 10} y={YT - 7} width="20" height="7" fill="var(--surface-2)" stroke="var(--ink)" />
        {g.over && <g><path d={`M${(X0 + X1) / 2} ${YT - 8} q-6 -14 -18 -10 M${(X0 + X1) / 2} ${YT - 8} q6 -14 18 -10`} stroke="var(--red)" stroke-width="3" fill="none" /><text x={(X0 + X1) / 2} y={YT - 24} font-size="13" font-weight="700" text-anchor="middle" fill="var(--red)">no room to expand!</text></g>}
        {[70, 110, 212, 282].map((x) => <circle key={x} cx={x} cy="140" r="11" fill="var(--ink)" stroke="var(--surface)" stroke-width="2" />)}
        {hitFront && <g><path d={`M200 40 h${Math.min(60, 12 + g.push)}`} stroke="var(--red)" stroke-width="4" /><path d={`M${204 + Math.min(60, 12 + g.push)} 40 l-9 -6 v12 z`} fill="var(--red)" /><text x="196" y="36" font-size="13" font-weight="700" text-anchor="end" fill="var(--red)">wave pushes truck</text></g>}
        {hitRear && <g><path d="M90 40 h-40" stroke="var(--red)" stroke-width="4" /><path d="M46 40 l9 -6 v12 z" fill="var(--red)" /><text x="96" y="44" font-size="13" font-weight="700" fill="var(--red)">wave hits rear: jerk</text></g>}
      </g>
      <text x="354" y="16" font-size="13" text-anchor="end" fill="var(--ink-2)">front →</text>
    </svg>
  );
}

/* ---------- Rear view: high center of gravity + side-to-side surge in a curve (p. 8-2 Fig. 8.1). Labels sit left of the drawing. */
function RearView({ s, ev, t, mask }: { s: Scene; ev: Ev | null; t: number; mask?: boolean }) {
  const fast = ev === 'curveFast', curve = fast || ev === 'curveSlow';
  const side = curve ? fillF(s.fill) * 0.9 * sm(t / 0.4) * (fast ? 1 : 0.35) : 0; // baffles do not stop side-to-side surge
  const ang = fast ? 90 * sm((t - 0.3) / 0.55) : curve ? 2 * sm(t / 0.5) : 0; // 90° = rig lying on its side
  const tipped = ang > 45;
  const cx = 136, cy = 88, R = 40, cg = cy + 6;
  const rad = (ang * Math.PI) / 180, rot = (x: number, y: number) => [180 + (x - 180) * Math.cos(rad) - (y - 160) * Math.sin(rad), 160 + (x - 180) * Math.sin(rad) + (y - 160) * Math.cos(rad)];
  const [wx, wy] = rot(cx, cy), [gx, gy] = rot(cx + side * 10, cg);
  const lvl = wy + R - (2 * R * Math.min(s.fill, 100)) / 100, rise = side * 40 * (1 - ang / 90);
  return (
    <svg viewBox="0 0 300 200" width="100%" style={{ display: 'block', maxWidth: '360px', marginInline: 'auto' }} role="img" aria-label={`Rear view: from Figure 8.1, a tanker's center of gravity is about 60 to 78 inches high; a pickup's is about 18 to 24 inches.${fast ? ` At the posted curve speed the liquid surges sideways and the tanker ${tipped ? 'has rolled over onto its side' : 'is rolling over'}.` : ev === 'curveSlow' ? ' Well below the posted speed the tanker leans only slightly.' : ''}`}>
      <rect x="0" y="0" width="300" height="200" fill="var(--surface-2)" />
      <rect x="0" y="160" width="300" height="40" fill="var(--surface)" />
      <line x1="0" x2="300" y1="160" y2="160" stroke="var(--ink-2)" stroke-width="1.5" />
      <g transform={`rotate(${ang.toFixed(1)} 180 160)`}>
        <rect x="94" y="128" width="84" height="10" fill="var(--ink-2)" />
        <rect x="92" y="136" width="20" height="24" rx="4" fill="var(--ink)" /><rect x="160" y="136" width="20" height="24" rx="4" fill="var(--ink)" />
      </g>
      {/* Tank drawn in the road's frame so the liquid surface stays level as the rig rolls. */}
      <defs><clipPath id="tk-rc"><circle cx={wx} cy={wy} r={R} /></clipPath></defs>
      <circle cx={wx} cy={wy} r={R} fill="var(--surface)" />
      {!mask && <polygon clip-path="url(#tk-rc)" points={`${wx - R},${wy + R} ${wx - R},${lvl + rise / 2} ${wx + R},${lvl - rise / 2} ${wx + R},${wy + R}`} fill="var(--blue)" opacity={0.6} />}
      {mask && <text x={wx} y={wy + 8} font-size="24" font-weight="700" text-anchor="middle" fill="var(--ink-2)">?</text>}
      <circle cx={wx} cy={wy} r={R} fill="none" stroke="var(--ink)" stroke-width="2" />
      {!mask && <circle cx={gx} cy={gy} r="5" fill="var(--amber)" stroke="var(--ink)" />}
      <g aria-hidden="true">
        <line x1="80" x2="80" y1="160" y2={cg} stroke="var(--amber)" stroke-width="3" />
        {ang < 3 && <line x1="80" x2={cx - R - 3} y1={cg} y2={cg} stroke="var(--amber)" stroke-width="1.5" stroke-dasharray="3 3" />}
        <text x="74" y={cg - 4} font-size="13" font-weight="700" text-anchor="end" fill="var(--ink)">tanker CG</text>
        <text x="74" y={cg + 11} font-size="13" text-anchor="end" fill="var(--ink)">60–78 in</text>
        <line x1="70" x2="88" y1="136" y2="136" stroke="var(--ink-2)" stroke-width="1.5" stroke-dasharray="3 3" />
        <text x="66" y="134" font-size="13" text-anchor="end" fill="var(--ink-2)">pickup CG</text>
        <text x="66" y="149" font-size="13" text-anchor="end" fill="var(--ink-2)">18–24 in</text>
        <text x="6" y="178" font-size="12" fill="var(--ink-2)">Heights from Figure 8.1 (p. 8-2)</text>
        {curve && <text x="294" y="195" font-size="13" text-anchor="end" fill="var(--ink-2)">outside of curve →</text>}
      </g>
      {fast && tipped && <text x="150" y="22" font-size="15" font-weight="700" text-anchor="middle" fill="var(--red)">⚠ ROLLOVER: rig on its side</text>}
    </svg>
  );
}

function notes(s: Scene): { warn: boolean; text: string; page: string }[] {
  const n: { warn: boolean; text: string; page: string }[] = [];
  if (s.fill >= 100) n.push({ warn: true, text: 'Never load a cargo tank totally full. Liquids expand as they warm; the empty room you leave is outage.', page: '8-2' });
  if (s.liquid === 'milk' && s.tank === 'baffle') n.push({ warn: true, text: 'Food tanks (like milk) are smooth bore: sanitation rules forbid baffles because they make the inside hard to clean.', page: '8-2' });
  if (s.liquid === 'acid' && s.fill >= 85) n.push({ warn: true, text: 'A full tank of a dense liquid (some acids) may be over the legal weight limit, so heavy liquids are often loaded only part way.', page: '8-2' });
  if (s.tank === 'bulk' && s.pattern !== 'even') n.push({ warn: true, text: `All the load in the ${s.pattern} compartment: too much weight on the ${s.pattern}. Watch weight distribution when loading and unloading.`, page: '8-2' });
  if (s.fill < 100 && s.tank !== 'bulk') n.push({ warn: false, text: 'A partly filled tank surges. When the wave hits the end of the tank, it pushes the truck the way the wave is moving.', page: '8-2' });
  return n;
}
const RESULT: Record<Ev, (s: Scene) => string> = {
  brake: (s) => s.ice && foreAft(s) > 0 ? 'Even stopped, the wave shoved the truck on the ice. On a slippery road surge can push a stopped truck out into an intersection.' : 'Steady pressure held the truck against the wave. Keep steady brake pressure, brake far ahead of the stop, and increase following distance.',
  early: (s) => foreAft(s) > 0 ? 'You let off too soon: the wave hit the front and pushed the truck forward past the stop line. Don’t release the brakes too soon.' : 'No surge in a full tank, but a full tank breaks the outage rule.',
  start: (s) => s.tank === 'smooth' ? 'Smooth bore: the wave slammed the rear. Be extremely cautious starting and stopping; start very smoothly.' : 'The liquid still sloshed back. Start, slow down and stop very smoothly.',
  warm: (s) => s.fill >= 100 ? 'No outage: the warming liquid had nowhere to go. Never load a tank totally full.' : 'The liquid expanded into the outage space. That is why you leave outage.',
  curveFast: () => 'Tests show tankers can turn over at the posted curve speed. High center of gravity plus side-to-side surge (baffles don’t stop it) tipped the rig.',
  curveSlow: () => 'Slow down before the curve, then accelerate slightly through it, well below the posted speed. The rig stays upright.',
};
const isBad = (e: Ev, s: Scene) => e === 'early' || e === 'curveFast' || (e === 'brake' && s.ice && foreAft(s) > 0) || (e === 'warm' && s.fill >= 100) || (e === 'start' && s.tank === 'smooth');
const pageOf = (e: Ev) => (e === 'warm' ? '8-2' : e === 'curveFast' ? '8-2' : '8-3');
const pressed = (on: boolean) => (on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {});

function useAnim(motion: boolean) {
  const [ev, setEv] = useState<Ev | null>(null);
  const [t, setT] = useState(1);
  const raf = useRef(0);
  useEffect(() => () => cancelAnimationFrame(raf.current), []);
  const run = (e: Ev | null) => {
    cancelAnimationFrame(raf.current); setEv(e);
    if (!e) { setT(1); return; }
    if (!motion) { setT(STATIC_T[e]); return; }
    const t0 = performance.now();
    const tick = (now: number) => { const u = Math.min(1, (now - t0) / 2600); setT(u); if (u < 1) raf.current = requestAnimationFrame(tick); };
    setT(0); raf.current = requestAnimationFrame(tick);
  };
  return { ev, t, run };
}

function Views({ s, ev, t, mask }: { s: Scene; ev: Ev | null; t: number; mask?: boolean }) {
  return <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(240px, 1fr))', gap: '10px', alignItems: 'center' }}><SideView s={s} ev={ev} t={t} mask={mask} /><RearView s={s} ev={ev} t={t} mask={mask} /></div>;
}

function Explore({ motion }: { motion: boolean }) {
  const [s, setS] = useState<Scene>({ tank: 'smooth', fill: 70, liquid: 'milk', pattern: 'even', ice: false });
  const { ev, t, run } = useAnim(motion);
  const set = (p: Partial<Scene>) => { setS({ ...s, ...p }); run(null); };
  const seg = <K extends keyof Scene>(k: K, opts: [Scene[K], string][], label: string) => (
    <div class="row" role="group" aria-label={label} style={{ gap: '6px' }}><span class="small muted" style={{ minWidth: '64px' }}>{label}</span>{opts.map(([v, l]) => <button key={String(v)} class="btn sm" aria-pressed={s[k] === v} style={pressed(s[k] === v)} onClick={() => set({ [k]: v } as Partial<Scene>)}>{l}</button>)}</div>
  );
  return (
    <div class="stack">
      {seg('tank', [['bulk', 'Bulkheads'], ['baffle', 'Baffled'], ['smooth', 'Smooth bore']], 'Tank')}
      {s.tank === 'bulk' && seg('pattern', [['even', 'All 3 loaded'], ['front', 'Front only'], ['rear', 'Rear only']], 'Load')}
      {seg('liquid', [['milk', LIQ_NAME.milk], ['fuel', LIQ_NAME.fuel], ['acid', LIQ_NAME.acid]], 'Liquid')}
      <div class="field"><label for="tk-fill">Fill level: <strong class="num">{s.fill}%</strong> {s.fill < 100 ? `(outage ${100 - s.fill}%)` : '(no outage!)'}</label>
        <input id="tk-fill" type="range" min={40} max={100} step={5} value={s.fill} onInput={(e) => set({ fill: +(e.target as HTMLInputElement).value })} /></div>
      <label class="toggle"><input type="checkbox" checked={s.ice} onChange={(e) => set({ ice: (e.target as HTMLInputElement).checked })} />Slippery road (ice)</label>
      <Views s={s} ev={ev} t={t} />
      <div class="row" role="group" aria-label="Drive" style={{ gap: '6px' }}>{(Object.keys(EV_NAME) as Ev[]).map((e) => <button key={e} class={`btn sm ${ev === e ? 'primary' : ''}`} onClick={() => run(e)}>{EV_NAME[e]}</button>)}</div>
      {ev && <div class={`feedback ${isBad(ev, s) ? 'bad' : 'good'}`} role="status"><div class="verdict">{EV_NAME[ev]}</div><p class="small">{RESULT[ev](s)} <span class="plate">p. {pageOf(ev)}</span></p></div>}
      <div class="card flat stack" style={{ gap: '6px' }}>
        <div class="spread"><span class="eyebrow">{TANK_NAME[s.tank]}</span><span class="plate">p. 8-2</span></div>
        <p class="small">{TANK_NOTE[s.tank]}</p>
        {notes(s).map((n) => <p key={n.text} class="small" style={n.warn ? { color: 'var(--red)', fontWeight: 700 } : {}}>{n.warn ? '⚠ ' : ''}{n.text} <span class="plate">p. {n.page}</span></p>)}
        <p class="small muted">How much to load depends on: how much the liquid will expand in transit, its weight, legal weight limits, and the load’s temperature. <span class="plate">p. 8-2</span></p>
      </div>
    </div>
  );
}

/* ---------- Challenge: 8 scenarios */
interface Q { text: string; s: Scene; ev?: Ev; hide?: 'tank' | 'fill'; opts: string[]; ans: number; why: string; page: string; bad?: { s?: Partial<Scene>; ev: Ev }; good?: { s?: Partial<Scene>; ev: Ev } }
const B: Scene = { tank: 'smooth', fill: 70, liquid: 'fuel', pattern: 'even', ice: false };
const RAW: Q[] = [
  { text: 'You will haul milk. Which kind of tank will it usually be?', s: { ...B, liquid: 'milk' }, hide: 'tank', opts: ['Smooth bore (unbaffled)', 'Baffled, to control surge', 'Any tank, as long as it has baffles for safety'], ans: 0, why: 'Smooth bore tanks usually haul food products like milk. Sanitation rules forbid baffles because they make the inside hard to clean.', page: '8-2', bad: { s: { tank: 'baffle', liquid: 'milk' }, ev: 'brake' } },
  { text: 'Smooth bore tank, partly full, coming to a red light. How do you stop?', s: B, opts: ['Keep steady pressure on the brakes and don’t let off too soon', 'Ease off the brakes just before you stop for a smooth stop', 'Brake late and hard'], ans: 0, why: 'Keep steady brake pressure and don’t release too soon: the wave hits the front and pushes the truck forward. Brake far ahead of the stop.', page: '8-3', bad: { ev: 'early' }, good: { ev: 'brake' } },
  { text: 'You are stopped on an icy road with a partly filled tank. Can surge still move you?', s: { ...B, ice: true }, opts: ['Yes: the wave can shove the stopped truck out into the intersection', 'No: once you’re stopped, surge can’t move the truck', 'Only if the tank is full'], ans: 0, why: 'On a slippery road the surge wave can push a stopped truck out into an intersection.', page: '8-2', good: { ev: 'brake' }, bad: { ev: 'brake' } },
  { text: 'Baffled tank, half full, in a tight curve. What do the baffles do about side-to-side surge?', s: { ...B, tank: 'baffle', fill: 50 }, opts: ['Nothing: side-to-side surge can still happen and cause a rollover', 'They stop all surge', 'They stop side-to-side but not forward-and-back surge'], ans: 0, why: 'Baffles control forward-and-back surge. Side-to-side surge can still happen and cause a rollover.', page: '8-2', good: { ev: 'curveFast' }, bad: { ev: 'curveFast' } },
  { text: 'Loading fuel. How full should the tank be?', s: { ...B, fill: 90 }, hide: 'fill', opts: ['Totally full, so the liquid can’t surge', 'Leave outage: room for the liquid to expand as it warms', 'Outage is only needed for food loads'], ans: 1, why: 'Never load a cargo tank totally full. Liquids expand as they warm; leave outage, and know the outage requirement for your liquid.', page: '8-2', bad: { s: { fill: 100 }, ev: 'warm' }, good: { ev: 'warm' } },
  { text: 'Loading a dense acid. Can you fill the tank to near its capacity?', s: { ...B, liquid: 'acid', fill: 95 }, opts: ['Yes, if the tank does not leak', 'Not always: a full tank of dense liquid may be over the legal weight limit', 'Yes: heavy liquids never surge'], ans: 1, why: 'A full tank of a dense (heavy) liquid, like some acids, may be over the legal weight limit, so it is often loaded only part way.', page: '8-2', good: { s: { fill: 60 }, ev: 'brake' } },
  { text: 'Bulkhead tank. You load only the rear compartment. What must you watch?', s: { ...B, tank: 'bulk', pattern: 'rear', fill: 90 }, opts: ['Nothing: bulkheads keep each load separate', 'Weight distribution: not too much weight on the front or rear', 'Side-to-side surge from the baffle holes'], ans: 1, why: 'Bulkheads divide the tank into smaller tanks. When loading and unloading, watch weight distribution: don’t put too much weight on the front or rear.', page: '8-2', good: { s: { pattern: 'even' }, ev: 'brake' } },
  { text: 'An off-ramp curve has a posted speed sign. How do you take it in a tanker?', s: B, opts: ['At the posted speed: that is what the sign is for', 'Slow down before the curve, well below the posted speed, then accelerate slightly through it', 'Brake hard in the middle of the curve'], ans: 1, why: 'Tests show tankers can turn over at the posted curve speed. Slow down before the curve and accelerate slightly through it.', page: '8-3', bad: { ev: 'curveFast' }, good: { ev: 'curveSlow' } },
];
/** Rotate options so the right answer is not always in the same slot. */
const ROT = [0, 2, 1, 1, 0, 2, 1, 2];
const QS: Q[] = RAW.map((q, i) => { const n = q.opts.length, k = ROT[i] % n; return { ...q, opts: q.opts.map((_, j) => q.opts[(j + k) % n]), ans: (q.ans - k + n) % n }; });

function Challenge({ onEvidence, onChallenge, concepts, motion }: WidgetProps & { motion: boolean }) {
  const [i, setI] = useState(0);
  const [misses, setMisses] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const { ev, t, run } = useAnim(motion);
  const q = QS[i];
  if (!q) return (
    <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`}><div class="verdict">{misses === 0 ? `${QS.length} of ${QS.length} — stamp earned: Surge master` : `${QS.length - misses} of ${QS.length} right — need all ${QS.length} for the stamp`}</div>
      <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); run(null); }}>Try again</button></div>
  );
  const ok = pick === q.ans;
  const shown = pick === null ? null : ok ? q.good : q.bad;
  const scene: Scene = { ...q.s, ...(shown?.s ?? {}) };
  const masked = pick === null && !!q.hide;
  const choose = (j: number) => {
    if (pick !== null) return;
    setPick(j); const good = j === q.ans; if (!good) setMisses(misses + 1); onEvidence({ concepts, ok: good });
    const sh = good ? q.good : q.bad; if (sh) run(sh.ev);
  };
  const next = () => { if (i + 1 === QS.length && misses === 0) onChallenge?.(); setPick(null); run(null); setI(i + 1); };
  const opt = (o: string, j: number): JSX.Element => <button key={j} class="btn sm" disabled={pick !== null} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick === j && j !== q.ans ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : pick !== null && j === q.ans ? pressed(true) : {}) }} onClick={() => choose(j)}>{o}</button>;
  return (
    <div class="stack">
      <span class="small muted num">Scenario {i + 1} of {QS.length}</span>
      <strong>{q.text}</strong>
      <p class="small muted">{masked ? (q.hide === 'tank' ? `${LIQ_NAME[scene.liquid]} · tank type hidden until you answer` : `${LIQ_NAME[scene.liquid]} · fill level hidden until you answer`)
        : <>{TANK_NAME[scene.tank]} · {LIQ_NAME[scene.liquid]} · {scene.fill}% full{scene.tank === 'bulk' && scene.pattern !== 'even' ? ` · ${scene.pattern} compartment only` : ''}{scene.ice ? ' · icy road' : ''}</>}</p>
      <Views s={scene} ev={pick === null ? null : ev} t={pick === null ? 1 : t} mask={masked} />
      <div class="stack" role="group" aria-label="Choose an answer" style={{ gap: '6px' }}>{q.opts.map(opt)}</div>
      {pick !== null && <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
        <div class="verdict">{ok ? 'Right' : `Answer: ${q.opts[q.ans]}`}</div>
        <p class="small">{!ok && shown && isBad(shown.ev, scene) ? `What happens: ${RESULT[shown.ev](scene)} ` : ''}{q.why} <span class="plate">p. {q.page}</span></p>
        {motion && shown && <button class="btn sm" onClick={() => run(shown.ev)}>Replay</button>}
        <button class="btn primary sm" onClick={next}>{i + 1 === QS.length ? 'Finish' : 'Next scenario'}</button>
      </div>}
    </div>
  );
}

export default function SurgeBaffles(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  const motion = !props.reducedMotion;
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Surge lab</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>8 tanker scenarios</button></div>
      {mode === 'explore' ? <Explore motion={motion} /> : <Challenge {...props} motion={motion} />}
    </div>
  );
}
