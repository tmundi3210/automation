/* Foundation brake drawings for ab01-foundation-brake (original SVG after the parts in Figure 5.2, DL 650 p. 5-2, and the p. 5-3 text). */
import type { ComponentChildren } from 'preact';

export type PartId = 'chamber' | 'rod' | 'slack' | 'nut' | 'camshaft' | 'scam' | 'roller' | 'shoe' | 'lining' | 'spring' | 'drum' | 'axle';
const CX = 128, CY = 150, R_OUT = 106, R_IN = 95, S_A = 71, S_B = 80, L_B = 89; // lining 6 units off the drum at rest
const FS = 14;
const P = (r: number, a: number): [number, number] => [CX + r * Math.cos((a * Math.PI) / 180), CY + r * Math.sin((a * Math.PI) / 180)];
const f = (n: number) => n.toFixed(1);
function band(ra: number, rb: number, a0: number, a1: number) {
  const [x1, y1] = P(rb, a0), [x2, y2] = P(rb, a1), [x3, y3] = P(ra, a1), [x4, y4] = P(ra, a0);
  return `M${f(x1)} ${f(y1)} A${rb} ${rb} 0 0 1 ${f(x2)} ${f(y2)} L${f(x3)} ${f(y3)} A${ra} ${ra} 0 0 0 ${f(x4)} ${f(y4)}Z`;
}
function zig(x1: number, x2: number, y: number) {
  const n = 8, w = (x2 - x1) / n;
  let d = `M${f(x1)} ${y}`;
  for (let i = 1; i < n; i++) d += ` L${f(x1 + i * w)} ${y + (i % 2 ? -6 : 6)}`;
  return `${d} L${f(x2)} ${y}`;
}
const T = (x: number, y: number, s: ComponentChildren, anchor = 'middle', fill = 'var(--ink)', w = 600) => <text x={x} y={y} text-anchor={anchor} font-size={FS} font-weight={w} fill={fill}>{s}</text>;
const Lead = ({ d }: { d: [number, number, number, number] }) => <line x1={d[0]} y1={d[1]} x2={d[2]} y2={d[3]} stroke="var(--ink-2)" stroke-width="1.2" />;

/** Where the challenge ring sits for each part. */
export const MARK: Record<PartId, [number, number]> = {
  chamber: [344, 192], rod: [308, 192], slack: [292, 232], nut: [292, 258], camshaft: [240, 266], scam: [128, 228],
  roller: [108, 222], shoe: [64, 150], lining: [183, 85], spring: [128, 184], drum: [128, 50], axle: [128, 150],
};

interface ScamProps { stage: number; outAdj: boolean; labels: boolean; hi?: PartId | null; mark?: PartId | null; reduced: boolean; label: string }
/** S-cam drum brake. stage: 0 rest · 1 air in chamber · 2 push rod out · 3 slack adjuster moved · 4 camshaft + S-cam turned · 5 shoes on drum. */
export function SCam({ stage, outAdj, labels, hi, mark, reduced, label }: ScamProps) {
  const sk = (id: PartId) => (hi === id ? 'var(--accent)' : 'var(--ink)');
  const sw = (id: PartId, w: number) => (hi === id ? w + 2.5 : w);
  const full = outAdj ? -30 : -14;
  const theta = stage >= 3 ? full : stage === 2 ? full / 2 : 0;
  const camA = stage >= 4 ? (outAdj ? 44 : 30) : 0;
  const shift = outAdj ? (stage >= 5 ? 3 : -10) : stage >= 5 ? 6 : 0; // 6 = linings on the drum
  const touch = stage >= 5 && !outAdj;
  const rad = (theta * Math.PI) / 180, tipX = 292 + 90 * Math.sin(rad), tipY = 282 - 90 * Math.cos(rad);
  const air = stage >= 1;
  const L = (x: number, y: number, t: string, id: PartId, anchor = 'middle') => labels && T(x, y, t, anchor, hi === id ? 'var(--accent)' : 'var(--ink)', hi === id ? 800 : 600);
  const shoe = (side: -1 | 1) => {
    const [a0, a1] = side < 0 ? [105, 255] : [-75, 75];
    const [rx, ry] = P((S_A + S_B) / 2, side < 0 ? 105 : 75), [px, py] = P((S_A + S_B) / 2, side < 0 ? 255 : 285);
    return (
      <g transform={`translate(${side * shift} 0)`}>
        <path d={band(S_A, S_B, a0, a1)} fill="var(--surface)" stroke={sk('shoe')} stroke-width={sw('shoe', 1.5)} />
        <path d={band(S_B, L_B, a0, a1)} fill="var(--ink-2)" stroke={sk('lining')} stroke-width={sw('lining', 1.2)} />
        <circle cx={f(rx)} cy={f(ry)} r="6" fill="var(--surface)" stroke={sk('roller')} stroke-width={sw('roller', 2)} />
        <circle cx={f(px)} cy={f(py)} r="4" fill="var(--ink)" stroke="none" />
      </g>
    );
  };
  const sx = Math.sqrt(S_A * S_A - 34 * 34); // inner shoe edge at the spring height (y = CY + 34)
  return (
    <svg viewBox="0 0 380 326" width="100%" role="img" aria-label={label} style={{ display: 'block', maxWidth: '520px', marginInline: 'auto' }}>
      <line x1={CX} y1="228" x2="292" y2="282" stroke={sk('camshaft')} stroke-width={sw('camshaft', 4)} stroke-dasharray="7 4" />
      <circle cx={CX} cy={CY} r={R_OUT} fill="var(--surface)" stroke={sk('drum')} stroke-width={sw('drum', 2)} />
      <circle cx={CX} cy={CY} r={R_IN} fill="var(--surface-2)" stroke={touch ? 'var(--amber)' : sk('drum')} stroke-width={touch ? 5 : sw('drum', 1.5)} />
      {shoe(-1)}{shoe(1)}
      <path d={zig(CX - sx - shift, CX + sx + shift, CY + 34)} fill="none" stroke={sk('spring')} stroke-width={sw('spring', 2.2)} />
      <path d="M-21 -3 C-21 -20 -3 -20 0 0 C3 20 21 20 21 3" transform={`translate(${CX} 228) rotate(${camA})`} fill="none" stroke={sk('scam')} stroke-width={sw('scam', 6)} stroke-linecap="round" />
      <circle cx={CX} cy={CY} r="15" fill="var(--surface)" stroke={sk('axle')} stroke-width={sw('axle', 2)} />
      <circle cx={CX} cy={CY} r="5" fill="var(--ink)" stroke="none" />
      <path d="M374 34 V192 H366" fill="none" stroke={air ? 'var(--blue)' : 'var(--ink-2)'} stroke-width={air ? 4 : 2} stroke-dasharray={air ? '8 5' : '3 4'}>
        {air && !reduced && stage < 5 && <animate attributeName="stroke-dashoffset" from="26" to="0" dur="0.6s" repeatCount="indefinite" />}
      </path>
      <line x1="322" y1="192" x2={f(tipX)} y2={f(tipY)} stroke={sk('rod')} stroke-width={sw('rod', 5)} stroke-linecap="round" />
      <rect x="322" y="168" width="44" height="48" rx="10" fill={air ? 'var(--blue-soft)' : 'var(--surface)'} stroke={sk('chamber')} stroke-width={sw('chamber', 2)} />
      <line x1="333" y1="172" x2="333" y2="212" stroke="var(--ink-2)" stroke-width="1.5" />
      {air && T(350, 197, 'AIR', 'middle', 'var(--blue)', 800)}
      <g transform={`rotate(${theta} 292 282)`}>
        <rect x="285" y="186" width="14" height="102" rx="7" fill="var(--surface)" stroke={sk('slack')} stroke-width={sw('slack', 2)} />
        <polygon points="292,250 299,254 299,262 292,266 285,262 285,254" fill="var(--ink-2)" stroke={sk('nut')} stroke-width={sw('nut', 1.2)} />
      </g>
      <circle cx="292" cy="282" r="9" fill="var(--surface)" stroke={sk('camshaft')} stroke-width={sw('camshaft', 2)} />
      <circle cx={f(tipX)} cy={f(tipY)} r="4" fill="var(--ink)" stroke="none" />
      {stage >= 5 && T(CX, 100, outAdj ? '✗ not pressing' : '✓ pressing', 'middle', outAdj ? 'var(--red)' : 'var(--ok)', 800)}
      {labels && <g>
        {T(374, 22, 'Air from brake pedal ↓', 'end', 'var(--blue)', 700)}
        {L(340, 142, 'Brake', 'chamber')}{L(340, 158, 'chamber', 'chamber')}
        {L(282, 178, 'Push rod', 'rod')}
        {L(306, 236, 'Slack', 'slack', 'start')}{L(306, 252, 'adjuster', 'slack', 'start')}
        {L(306, 280, 'Adjusting', 'nut', 'start')}{L(306, 296, 'nut', 'nut', 'start')}
        {L(214, 320, 'Camshaft', 'camshaft')}
        {L(CX, 30, 'Brake drum', 'drum')}
        {L(CX, 128, 'Axle', 'axle')}
        {L(92, 155, 'Shoe', 'shoe')}
        {L(232, 52, 'Lining', 'lining', 'start')}<Lead d={[230, 50, 185, 84]} />
        {L(4, 238, 'Return', 'spring', 'start')}{L(4, 254, 'spring', 'spring', 'start')}<Lead d={[36, 226, 64, 190]} />
        {L(CX, 304, 'S-cam', 'scam')}<Lead d={[CX, 292, CX, 250]} />
        {L(40, 292, 'Cam roller', 'roller')}<Lead d={[52, 279, 103, 227]} />
      </g>}
      {mark && <g>
        <circle cx={MARK[mark][0]} cy={MARK[mark][1]} r="18" fill="none" stroke="var(--accent)" stroke-width="3.5" stroke-dasharray="5 3" />
        <circle cx={MARK[mark][0] + 17} cy={MARK[mark][1] - 17} r="10" fill="var(--accent)" stroke="none" />
        {T(MARK[mark][0] + 17, MARK[mark][1] - 12, '?', 'middle', 'var(--accent-ink)', 800)}
      </g>}
    </svg>
  );
}

/** Wedge, disc and CamLaster brakes (p. 5-3), apply/release. */
export function OtherBrake({ kind, applied, label }: { kind: 'wedge' | 'disc' | 'camlaster'; applied: boolean; label: string }) {
  const g = applied ? 6 : 0;
  const drum = <>
    <circle cx={CX} cy={CY} r={R_OUT} fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
    <circle cx={CX} cy={CY} r={R_IN} fill="var(--surface-2)" stroke={applied ? 'var(--amber)' : 'var(--ink)'} stroke-width={applied ? 5 : 1.5} />
    <circle cx={CX} cy={CY} r="15" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
    {T(CX, 30, 'Brake drum')}{T(92, 155, 'Shoe')}
  </>;
  const shoes = (a: [number, number][]) => a.map(([a0, a1], i) => <g transform={`translate(${(i ? 1 : -1) * g} 0)`}>
    <path d={band(S_A, S_B, a0, a1)} fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" /><path d={band(S_B, L_B, a0, a1)} fill="var(--ink-2)" stroke="var(--ink)" stroke-width="1.2" /></g>);
  const air = applied ? 'var(--blue-soft)' : 'var(--surface)';
  const note = (x: number, y: number, lines: string[]) => lines.map((l, i) => T(x, y + i * 18, l));
  return (
    <svg viewBox="0 0 380 326" width="100%" role="img" aria-label={label} style={{ display: 'block', maxWidth: '520px', marginInline: 'auto' }}>
      {kind === 'wedge' && <g>
        {drum}{shoes([[100, 255], [-75, 80]])}
        <line x1={CX} y1={applied ? 214 : 226} x2={CX} y2="266" stroke="var(--ink)" stroke-width="5" />
        <polygon points={`${CX - 15},${applied ? 222 : 234} ${CX + 15},${applied ? 222 : 234} ${CX},${applied ? 198 : 210}`} fill="var(--amber)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x={CX - 26} y="266" width="52" height="34" rx="9" fill={air} stroke="var(--ink)" stroke-width="2" />
        {T(CX + 34, 290, 'Brake chamber', 'start')}{T(CX + 22, 264, 'Push rod', 'start')}{T(CX, 188, 'Wedge', 'middle', 'var(--ink)', 800)}
        {note(312, 110, ['The push rod', 'drives the wedge', 'between the ends', 'of 2 shoes'])}
      </g>}
      {kind === 'camlaster' && <g>
        {drum}{shoes([[112, 250], [-70, 68]])}
        <path d={`M${CX - 30} ${applied ? 226 : 234} L${CX - 8} ${applied ? 208 : 216} L${CX + 8} ${applied ? 208 : 216} L${CX + 30} ${applied ? 226 : 234} Z`} fill="var(--amber)" stroke="var(--ink)" stroke-width="1.5" />
        {T(CX, 300, 'Cam with a sloped ramp')}<Lead d={[CX, 288, CX, 236]} />
        {note(312, 70, ['Shoes slide down', 'the ramp to touch', 'the drum evenly'])}
        <rect x="246" y="168" width="130" height="66" rx="8" fill="var(--accent-soft)" stroke="var(--accent)" stroke-width="1.5" />
        {note(311, 190, ['Adjuster is inside:', 'no outside', 'slack adjuster'])}
      </g>}
      {kind === 'disc' && <g>
        <rect x="100" y="42" width="16" height="240" rx="3" fill="var(--surface-2)" stroke={applied ? 'var(--amber)' : 'var(--ink)'} stroke-width={applied ? 4 : 2} />
        <line x1="14" y1="246" x2="100" y2="246" stroke="var(--ink)" stroke-width="10" />
        <path d="M70 56 H150 V66 H80 V136 H150 V146 H70 Z" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
        <rect x={86 + (applied ? 4 : 0)} y="70" width="10" height="62" fill="var(--ink-2)" stroke="var(--ink)" stroke-width="1" />
        <rect x={120 - (applied ? 4 : 0)} y="70" width="10" height="62" fill="var(--ink-2)" stroke="var(--ink)" stroke-width="1" />
        <rect x={130 - (applied ? 4 : 0)} y="92" width="90" height="18" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
        {[0, 1, 2, 3, 4, 5, 6, 7].map((i) => <line x1={138 + i * 10 - (applied ? 4 : 0)} y1="92" x2={144 + i * 10 - (applied ? 4 : 0)} y2="110" stroke="var(--ink)" stroke-width="1.2" />)}
        <g transform={`rotate(${applied ? 16 : 0} 228 101)`}><rect x="221" y="94" width="14" height="132" rx="7" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" /></g>
        <circle cx="228" cy="101" r="9" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
        <line x1={applied ? 196 : 228} y1="220" x2="306" y2="220" stroke="var(--ink)" stroke-width="5" />
        <rect x="306" y="196" width="48" height="48" rx="10" fill={air} stroke="var(--ink)" stroke-width="2" />
        {T(108, 30, 'Rotor (disc)')}{T(34, 186, 'Caliper')}<Lead d={[40, 174, 72, 144]} />
        {T(150, 176, 'Pads')}<Lead d={[140, 164, 124, 132]} />
        {T(140, 84, 'Power screw', 'start')}{T(244, 150, 'Slack', 'start')}{T(244, 166, 'adjuster', 'start')}
        {T(330, 180, 'Brake', 'middle')}{T(330, 262, 'chamber', 'middle')}{T(40, 272, 'Axle')}
        {note(266, 292, ['Like a big C-clamp:', 'the screw squeezes the pads'])}
      </g>}
    </svg>
  );
}
