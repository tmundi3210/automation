/* Foundation brake drawings for ab01-foundation-brake (original SVG after the parts in Figure 5.2, DL 650 p. 5-2, and the p. 5-3 text). */
import type { ComponentChildren } from 'preact';

export type PartId = 'chamber' | 'rod' | 'slack' | 'nut' | 'camshaft' | 'scam' | 'roller' | 'shoe' | 'lining' | 'spring' | 'drum' | 'axle';
const CX = 140, CY = 145;
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
/** Where the challenge ring sits for each part. */
export const MARK: Record<PartId, [number, number]> = {
  chamber: [387, 166], rod: [338, 166], slack: [310, 200], nut: [310, 240], camshaft: [270, 254], scam: [140, 229],
  roller: [119, 218], shoe: [70, 150], lining: [227, 110], spring: [140, 190], drum: [140, 39], axle: [140, 145],
};

interface ScamProps { stage: number; outAdj: boolean; labels: boolean; hi?: PartId | null; mark?: PartId | null; reduced: boolean; label: string }
/** S-cam drum brake. stage: 0 rest · 1 air in chamber · 2 push rod out · 3 slack adjuster moved · 4 camshaft + S-cam turned · 5 shoes on drum. */
export function SCam({ stage, outAdj, labels, hi, mark, reduced, label }: ScamProps) {
  const sk = (id: PartId) => (hi === id ? 'var(--accent)' : 'var(--ink)');
  const sw = (id: PartId, w: number) => (hi === id ? w + 2.5 : w);
  const full = outAdj ? -26 : -12;
  const theta = stage >= 3 ? full : stage === 2 ? full / 2 : 0;
  const camA = stage >= 4 ? (outAdj ? 40 : 28) : 0;
  const shift = outAdj ? (stage >= 5 ? 3 : -10) : stage >= 5 ? 6 : 0; // 6 = touching the drum
  const touch = stage >= 5 && !outAdj;
  const rad = (theta * Math.PI) / 180, tipX = 310 + 96 * Math.sin(rad), tipY = 262 - 96 * Math.cos(rad);
  const air = stage >= 1;
  const L = (x: number, y: number, t: ComponentChildren, id: PartId, anchor = 'middle') =>
    labels && <text x={x} y={y} text-anchor={anchor} font-size="12.5" font-weight={hi === id ? 800 : 600} fill={hi === id ? 'var(--accent)' : 'var(--ink)'}>{t}</text>;
  const lead = (x1: number, y1: number, x2: number, y2: number) => labels && <line x1={x1} y1={y1} x2={x2} y2={y2} stroke="var(--ink-2)" stroke-width="1" />;
  const shoe = (side: -1 | 1) => {
    const [a0, a1] = side < 0 ? [105, 255] : [-75, 75];
    return (
      <g transform={`translate(${side * shift} 0)`}>
        <path d={band(76, 86, a0, a1)} fill="var(--surface)" stroke={sk('shoe')} stroke-width={sw('shoe', 1.5)} />
        <path d={band(86, 94, a0, a1)} fill="var(--ink-2)" stroke={sk('lining')} stroke-width={sw('lining', 1.2)} />
        <circle cx={f(P(81, side < 0 ? 105 : 75)[0])} cy={f(P(81, 75)[1])} r="5.5" fill="var(--surface)" stroke={sk('roller')} stroke-width={sw('roller', 1.5)} />
        <circle cx={f(P(81, side < 0 ? 255 : 285)[0])} cy={f(P(81, 255)[1])} r="4" fill="var(--ink)" stroke="none" />
      </g>
    );
  };
  const sx = 61; // inner edge of shoe at the spring height
  return (
    <svg viewBox="0 0 420 300" width="100%" role="img" aria-label={label} style={{ display: 'block', maxWidth: '560px', marginInline: 'auto' }}>
      <line x1={CX} y1="229" x2="310" y2="262" stroke={sk('camshaft')} stroke-width={sw('camshaft', 4)} stroke-dasharray="7 4" />
      <circle cx={CX} cy={CY} r="112" fill="var(--surface)" stroke={sk('drum')} stroke-width={sw('drum', 2)} />
      <circle cx={CX} cy={CY} r="100" fill="var(--surface-2)" stroke={touch ? 'var(--amber)' : sk('drum')} stroke-width={touch ? 5 : sw('drum', 1.5)} />
      {shoe(-1)}{shoe(1)}
      <path d={zig(CX - sx - shift, CX + sx + shift, 190)} fill="none" stroke={sk('spring')} stroke-width={sw('spring', 2)} />
      <g transform={`rotate(${camA} ${CX} 229)`}>
        <path d="M-14 -2 C-14 -13 -2 -13 0 0 C2 13 14 13 14 2" transform={`translate(${CX} 229)`} fill="none" stroke={sk('scam')} stroke-width={sw('scam', 5)} stroke-linecap="round" />
      </g>
      <circle cx={CX} cy={CY} r="16" fill="var(--surface)" stroke={sk('axle')} stroke-width={sw('axle', 2)} />
      <circle cx={CX} cy={CY} r="5" fill="var(--ink)" stroke="none" />
      {/* air line + chamber */}
      <path d="M404 26 V140" fill="none" stroke={air ? 'var(--blue)' : 'var(--ink-2)'} stroke-width={air ? 4 : 2} stroke-dasharray={air ? '8 5' : '3 4'}>
        {air && !reduced && stage < 5 && <animate attributeName="stroke-dashoffset" from="26" to="0" dur="0.6s" repeatCount="indefinite" />}
      </path>
      <line x1="364" y1="166" x2={f(tipX)} y2={f(tipY)} stroke={sk('rod')} stroke-width={sw('rod', 5)} stroke-linecap="round" />
      <rect x="364" y="140" width="46" height="52" rx="10" fill={air ? 'var(--blue-soft)' : 'var(--surface)'} stroke={sk('chamber')} stroke-width={sw('chamber', 2)} />
      <line x1="376" y1="144" x2="376" y2="188" stroke="var(--ink-2)" stroke-width="1.5" />
      {air && <text x="393" y="171" text-anchor="middle" font-size="11" font-weight="700" fill="var(--blue)">AIR</text>}
      <g transform={`rotate(${theta} 310 262)`}>
        <rect x="303" y="160" width="14" height="108" rx="7" fill="var(--surface)" stroke={sk('slack')} stroke-width={sw('slack', 2)} />
        <polygon points="310,232 317,236 317,244 310,248 303,244 303,236" fill="var(--ink-2)" stroke={sk('nut')} stroke-width={sw('nut', 1.2)} />
      </g>
      <circle cx="310" cy="262" r="9" fill="var(--surface)" stroke={sk('camshaft')} stroke-width={sw('camshaft', 2)} />
      <circle cx={f(tipX)} cy={f(tipY)} r="4" fill="var(--ink)" stroke="none" />
      {labels && <g>
        <text x="400" y="17" text-anchor="end" font-size="12" fill="var(--blue)" font-weight="700">Air from brake pedal ↓</text>
        {L(387, 118, 'Brake', 'chamber')}{L(387, 132, 'chamber', 'chamber')}
        {L(338, 156, 'Push rod', 'rod')}
        {L(326, 212, 'Slack', 'slack', 'start')}{L(326, 226, 'adjuster', 'slack', 'start')}
        {L(326, 254, 'Adjusting nut', 'nut', 'start')}
        {L(268, 290, 'Camshaft (behind drum)', 'camshaft')}
        {L(CX, 24, 'Brake drum', 'drum')}
        {L(CX, 176, 'Axle', 'axle')}
        {L(100, 122, 'Brake shoe', 'shoe')}
        {L(190, 104, 'Lining', 'lining')}{lead(209, 100, 219, 100)}
        {L(CX, 209, 'Return spring', 'spring')}
        {L(CX, 285, 'S-cam', 'scam')}{lead(CX, 273, CX, 240)}
        {L(58, 268, 'Cam roller', 'roller', 'end')}{lead(56, 262, 114, 222)}
      </g>}
      {mark && <g>
        <circle cx={MARK[mark][0]} cy={MARK[mark][1]} r="17" fill="none" stroke="var(--accent)" stroke-width="3.5" stroke-dasharray="5 3" />
        <circle cx={MARK[mark][0] + 16} cy={MARK[mark][1] - 16} r="9" fill="var(--accent)" stroke="none" />
        <text x={MARK[mark][0] + 16} y={MARK[mark][1] - 12} text-anchor="middle" font-size="12" font-weight="800" fill="var(--accent-ink)">?</text>
      </g>}
    </svg>
  );
}

/** Wedge, disc and CamLaster brakes (p. 5-3), apply/release. */
export function OtherBrake({ kind, applied, label }: { kind: 'wedge' | 'disc' | 'camlaster'; applied: boolean; label: string }) {
  const t = (x: number, y: number, s: string, a = 'middle') => <text x={x} y={y} text-anchor={a} font-size="12.5" font-weight="600" fill="var(--ink)">{s}</text>;
  const g = applied ? 6 : 0;
  const drum = (hot: boolean) => <>
    <circle cx={CX} cy={CY} r="112" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
    <circle cx={CX} cy={CY} r="100" fill="var(--surface-2)" stroke={hot ? 'var(--amber)' : 'var(--ink)'} stroke-width={hot ? 5 : 1.5} />
    <circle cx={CX} cy={CY} r="16" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
  </>;
  const shoes = (a: [number, number][]) => a.map(([a0, a1], i) => <g transform={`translate(${(i ? 1 : -1) * g} 0)`}>
    <path d={band(76, 86, a0, a1)} fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" /><path d={band(86, 94, a0, a1)} fill="var(--ink-2)" stroke="var(--ink)" stroke-width="1.2" /></g>);
  const air = applied ? 'var(--blue-soft)' : 'var(--surface)';
  return (
    <svg viewBox="0 0 420 300" width="100%" role="img" aria-label={label} style={{ display: 'block', maxWidth: '560px', marginInline: 'auto' }}>
      {kind === 'wedge' && <g>
        {drum(applied)}{shoes([[100, 255], [-75, 80]])}
        <line x1={CX} y1={applied ? 214 : 226} x2={CX} y2="262" stroke="var(--ink)" stroke-width="5" />
        <polygon points={`${CX - 14},${applied ? 222 : 234} ${CX + 14},${applied ? 222 : 234} ${CX},${applied ? 198 : 210}`} fill="var(--amber)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x={CX - 26} y="262" width="52" height="34" rx="9" fill={air} stroke="var(--ink)" stroke-width="2" />
        {t(CX + 34, 284, 'Brake chamber', 'start')}{t(CX + 20, 246, 'Push rod', 'start')}{t(CX, 190, 'Wedge')}
        {t(CX, 24, 'Brake drum')}{t(100, 122, 'Brake shoe')}{t(330, 120, 'The wedge spreads the', 'middle')}{t(330, 136, '2 shoe ends apart', 'middle')}
      </g>}
      {kind === 'camlaster' && <g>
        {drum(applied)}{shoes([[112, 250], [-70, 68]])}
        <path d={`M${CX - 30} ${applied ? 222 : 230} L${CX - 8} ${applied ? 204 : 212} L${CX + 8} ${applied ? 204 : 212} L${CX + 30} ${applied ? 222 : 230} Z`} fill="var(--amber)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x={CX - 9} y="222" width="18" height="40" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
        {t(CX, 24, 'Brake drum')}{t(100, 122, 'Brake shoe')}{t(CX + 40, 228, 'Cam with sloped ramp', 'start')}
        {t(330, 60, 'Shoes slide down', 'middle')}{t(330, 76, 'the ramp to touch', 'middle')}{t(330, 92, 'the drum evenly', 'middle')}
        <rect x="262" y="150" width="140" height="60" rx="8" fill="var(--accent-soft)" stroke="var(--accent)" stroke-width="1.5" />
        {t(332, 174, 'Adjuster is INSIDE:')}{t(332, 192, 'no outside slack adj.')}
      </g>}
      {kind === 'disc' && <g>
        <rect x="126" y="30" width="18" height="240" rx="3" fill="var(--surface-2)" stroke={applied ? 'var(--amber)' : 'var(--ink)'} stroke-width={applied ? 4 : 2} />
        <line x1="40" y1="220" x2="126" y2="220" stroke="var(--ink)" stroke-width="10" />
        <path d="M96 44 H174 V54 H106 V124 H174 V134 H96 Z" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
        <rect x={112 + (applied ? 5 : 0) - 2} y="58" width="10" height="62" fill="var(--ink-2)" stroke="var(--ink)" stroke-width="1" />
        <rect x={148 - (applied ? 4 : 0)} y="58" width="10" height="62" fill="var(--ink-2)" stroke="var(--ink)" stroke-width="1" />
        <rect x={158 - (applied ? 4 : 0)} y="80" width="70" height="18" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
        {[0, 1, 2, 3, 4, 5].map((i) => <line x1={170 + i * 10 - (applied ? 4 : 0)} y1="80" x2={176 + i * 10 - (applied ? 4 : 0)} y2="98" stroke="var(--ink)" stroke-width="1.2" />)}
        <g transform={`rotate(${applied ? -20 : 0} 240 89)`}><rect x="233" y="89" width="14" height="92" rx="7" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" /></g>
        <circle cx="240" cy="89" r="9" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
        <line x1={applied ? 280 : 300} y1="175" x2="340" y2="175" stroke="var(--ink)" stroke-width="5" />
        <rect x="340" y="150" width="46" height="50" rx="10" fill={air} stroke="var(--ink)" stroke-width="2" />
        {t(135, 22, 'Rotor (disc)')}{t(60, 160, 'Caliper', 'middle')}<line x1="80" y1="152" x2="98" y2="120" stroke="var(--ink-2)" stroke-width="1" />
        {t(190, 150, 'Pads', 'middle')}<line x1="178" y1="140" x2="156" y2="112" stroke="var(--ink-2)" stroke-width="1" />
        {t(196, 74, 'Power screw', 'start')}{t(252, 200, 'Slack', 'start')}{t(252, 214, 'adjuster', 'start')}
        {t(363, 140, 'Brake', 'middle')}{t(363, 216, 'chamber', 'middle')}{t(60, 244, 'Axle', 'middle')}
        {t(290, 262, 'Like a big C-clamp: the', 'middle')}{t(290, 278, 'screw squeezes the pads', 'middle')}
      </g>}
    </svg>
  );
}
