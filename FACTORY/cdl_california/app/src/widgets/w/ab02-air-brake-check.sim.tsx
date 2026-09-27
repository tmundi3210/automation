/* Air brake check simulator: rig limits, sim step and the gauge + dash art for ab02-air-brake-check.
   Every limit below is from AB-02 (DL 650 pp. 5-6 – 5-10) and AB-01 (governor, p. 5-1). */

export type Kind = 'single' | 'two' | 'three' | 'noair';
export const KINDS: { k: Kind; label: string; short: string }[] = [
  { k: 'single', label: 'Single vehicle', short: 'Single' },
  { k: 'two', label: 'Combination of 2 vehicles', short: '2 units' },
  { k: 'three', label: 'Combination of 3 or more', short: '3+ units' },
  { k: 'noair', label: 'Towing units with no air brakes', short: 'Towed: no air' },
];
/** Max drop in 1 minute: applied (p. 5-8) and static (p. 5-10). */
export const APPLIED: Record<Kind, number> = { single: 3, two: 4, three: 6, noair: 3 };
export const STATIC: Record<Kind, number> = { single: 2, two: 3, three: 5, noair: 2 };
export const CUT_OUT = 125, CUT_IN = 100; // governor, p. 5-1 (applied test: cut-out 120–140 psi or maker's level, p. 5-8)
export const WARN_MIN = 55;               // warning must come on before pressure drops below 55 psi (p. 5-8)
export const SPRING: [number, number] = [20, 45]; // spring brakes / valves pop out, normally 20–45 psi (p. 5-9)
export const BUILD = { from: 85, to: 100, max: 45 }; // dual system, normal operating idle (p. 5-9)

export type Fault = 'none' | 'leak' | 'weak' | 'warn' | 'spring';
export const FAULTS: { f: Fault; label: string }[] = [
  { f: 'none', label: 'No fault (healthy rig)' },
  { f: 'leak', label: 'Air leak' },
  { f: 'weak', label: 'Worn compressor (slow buildup)' },
  { f: 'warn', label: 'Low-air warning set too low' },
  { f: 'spring', label: 'Spring brake valve sticks' },
];
export interface Rig { kind: Kind; fault: Fault }
/** Sim-only numbers (not handbook facts): chosen so a healthy rig passes and each fault fails its own check. */
export const rigNums = (r: Rig) => ({
  pump: r.fault === 'weak' ? 0.24 : 0.6,                          // psi per second at idle
  applied: r.fault === 'leak' ? APPLIED[r.kind] + 2 : 2,           // psi lost per minute, pedal held
  stat: r.fault === 'leak' ? STATIC[r.kind] + 2 : 1,               // psi lost per minute, brakes released
  warn: r.fault === 'warn' ? 48 : 62,                              // warning set point
  pop: r.fault === 'spring' ? 12 : 34,                             // spring brake pop-out
});

export interface Sim {
  psi: number; engine: boolean; pumping: boolean; park: boolean; supply: boolean; // park/supply = knob OUT (applied)
  pedal: boolean; t: number; t85: number | null; buildS: number | null; cutAt: number | null;
  warnAt: number | null; popAt: number | null; applied: number | null; stat: number | null;
  parkTest: string | null; svcTest: string | null; note: string | null;
}
export const START: Sim = { psi: 70, engine: false, pumping: true, park: true, supply: true, pedal: false, t: 0, t85: null, buildS: null, cutAt: null, warnAt: null, popAt: null, applied: null, stat: null, parkTest: null, svcTest: null, note: null };

const combo = (k: Kind) => k === 'two' || k === 'three';
/** Falling pressure: warning and spring-brake pop-outs are recorded as they happen. */
export function drop(s: Sim, r: Rig, by: number): Sim {
  const n = rigNums(r), was = s.psi, psi = Math.max(0, +(s.psi - by).toFixed(1));
  const o = { ...s, psi };
  if (was >= n.warn && psi < n.warn && o.warnAt == null) o.warnAt = n.warn;
  if (psi <= n.pop && (!s.park || (combo(r.kind) && !s.supply))) { o.park = true; o.supply = true; if (o.popAt == null) o.popAt = n.pop; o.pedal = false; }
  return o;
}
/** One live tick of `dt` sim seconds with the engine running (governor + compressor). */
export function tick(s: Sim, r: Rig, dt: number): Sim {
  if (!s.engine) return s;
  const n = rigNums(r);
  let pumping = s.pumping;
  if (pumping && s.psi >= CUT_OUT) pumping = false;
  if (!pumping && s.psi <= CUT_IN) pumping = true;
  const t = s.t + dt;
  const psi = pumping ? Math.min(CUT_OUT, +(s.psi + n.pump * dt).toFixed(2)) : s.psi;
  const o: Sim = { ...s, t, pumping, psi };
  if (s.psi < BUILD.from && psi >= BUILD.from) o.t85 = t - (psi - BUILD.from) / n.pump;
  if (s.psi < BUILD.to && psi >= BUILD.to && o.t85 != null) o.buildS = Math.round(t - (psi - BUILD.to) / n.pump - o.t85);
  if (s.pumping && !pumping) o.cutAt = CUT_OUT;
  return o;
}
export const warnOn = (s: Sim, r: Rig) => s.psi < rigNums(r).warn;

/* ---------- art */
const A0 = -210, SWEEP = 240, MAXP = 150;
const ang = (p: number) => ((A0 + (Math.min(MAXP, Math.max(0, p)) / MAXP) * SWEEP) * Math.PI) / 180;
const pt = (cx: number, cy: number, r: number, p: number) => [cx + r * Math.cos(ang(p)), cy + r * Math.sin(ang(p))];
function arc(cx: number, cy: number, r: number, a: number, b: number) {
  const [x1, y1] = pt(cx, cy, r, a), [x2, y2] = pt(cx, cy, r, b);
  return `M${x1.toFixed(1)} ${y1.toFixed(1)} A${r} ${r} 0 ${((b - a) / MAXP) * SWEEP > 180 ? 1 : 0} 1 ${x2.toFixed(1)} ${y2.toFixed(1)}`;
}

/** Supply pressure gauge (0–150 psi) with the handbook bands. `ghost` = where the needle started. */
export function Gauge({ psi, ghost, cx = 92, cy = 100, r = 80 }: { psi: number; ghost?: number; cx?: number; cy?: number; r?: number }) {
  const needle = (p: number, color: string, w: number, dash?: string) => { const [x, y] = pt(cx, cy, r - 16, p); return <line x1={cx} y1={cy} x2={x} y2={y} stroke={color} stroke-width={w} stroke-linecap="round" stroke-dasharray={dash} />; };
  return (
    <g>
      <circle cx={cx} cy={cy} r={r + 4} fill="var(--surface)" stroke="var(--ink)" stroke-width="2.5" />
      <path d={arc(cx, cy, r - 6, SPRING[0], SPRING[1])} fill="none" stroke="var(--red)" stroke-width="8" />
      <path d={arc(cx, cy, r - 6, 55, 75)} fill="none" stroke="var(--amber)" stroke-width="8" />
      <path d={arc(cx, cy, r - 6, CUT_IN, CUT_OUT)} fill="none" stroke="var(--ok)" stroke-width="8" />
      {Array.from({ length: 16 }, (_, i) => i * 10).map((p) => { const [x1, y1] = pt(cx, cy, r - 1, p), [x2, y2] = pt(cx, cy, r - (p % 50 === 0 ? 14 : 10), p); return <line key={p} x1={x1} y1={y1} x2={x2} y2={y2} stroke="var(--ink)" stroke-width={p % 50 === 0 ? 2 : 1} />; })}
      {[0, 50, 100, 150].map((p) => { const [x, y] = pt(cx, cy, r - 26, p); return <text key={p} x={x} y={y + 4} text-anchor="middle" font-size="12" font-weight="700" fill="var(--ink)">{p}</text>; })}
      {ghost != null && needle(ghost, 'var(--ink-2)', 2, '3 3')}
      {needle(psi, 'var(--red)', 3.5)}
      <circle cx={cx} cy={cy} r="6" fill="var(--ink)" stroke="none" />
      <text x={cx} y={cy + 34} text-anchor="middle" font-size="22" font-weight="700" fill="var(--ink)">{Math.round(psi)}</text>
      <text x={cx} y={cy + 49} text-anchor="middle" font-size="11" fill="var(--ink-2)">psi · tank</text>
    </g>
  );
}

/** Dash: gauge + low-air lamp/buzzer + yellow parking knob (+ red trailer supply knob on combinations) + compressor state. */
export function Dash({ s, rig, label }: { s: Sim; rig: Rig; label: string }) {
  const warn = warnOn(s, rig), cmb = combo(rig.kind);
  const knob = (x: number, out: boolean, color: string, diamond: boolean, name: string) => {
    const y = out ? 142 : 128;
    return (
      <g>
        <rect x={x - 4} y="112" width="8" height={y - 112} fill="var(--ink-2)" stroke="none" />
        {diamond ? <rect x={x - 12} y={y - 12} width="24" height="24" transform={`rotate(45 ${x} ${y})`} fill={color} stroke="var(--ink)" stroke-width="1.5" />
          : <polygon points={[0, 1, 2, 3, 4, 5, 6, 7].map((i) => { const a = (i * 45 + 22.5) * Math.PI / 180; return `${(x + 16 * Math.cos(a)).toFixed(1)},${(y + 16 * Math.sin(a)).toFixed(1)}`; }).join(' ')} fill={color} stroke="var(--ink)" stroke-width="1.5" />}
        <text x={x} y="178" text-anchor="middle" font-size="13" fill="var(--ink)">{name}</text>
        <text x={x} y="194" text-anchor="middle" font-size="13" font-weight="700" fill="var(--ink)">{out ? 'OUT · on' : 'IN · off'}</text>
      </g>
    );
  };
  return (
    <svg viewBox="0 0 360 202" width="100%" role="img" aria-label={label} style={{ display: 'block', maxWidth: '540px', marginInline: 'auto' }}>
      <rect x="0" y="0" width="360" height="202" rx="10" fill="var(--surface-2)" stroke="none" />
      <Gauge psi={s.psi} />
      <rect x="190" y="8" width="164" height="36" rx="6" fill={warn ? 'var(--red)' : 'var(--surface)'} stroke="var(--ink)" stroke-width="1.5" />
      <text x="272" y="31" text-anchor="middle" font-size="14" font-weight="700" fill={warn ? 'var(--on-red)' : 'var(--ink-2)'}>{warn ? '⚠ LOW AIR · BZZZ' : 'Low air: off'}</text>
      <text x="272" y="66" text-anchor="middle" font-size="13" font-weight="700" fill="var(--ink)">{!s.engine ? 'Engine off' : s.pumping ? 'Compressor pumping ▲' : 'Governor cut-out'}</text>
      <text x="272" y="88" text-anchor="middle" font-size="13" fill="var(--ink)">{s.pedal ? 'Foot brake: HELD DOWN' : 'Foot brake: up'}</text>
      {knob(cmb ? 232 : 272, s.park, 'var(--amber)', true, 'Parking')}
      {cmb && knob(314, s.supply, 'var(--red)', false, 'Trailer air')}
    </svg>
  );
}
export { combo };
