// SVG art for gk04-walkaround (helper; no meta, so the registry ignores it).
// Legibility: every label is >= 13 font units in a 300–320 unit viewBox, so it renders >= 11 px
// in the 260–294 px column a 390 px phone gives the widget. Desktop width is capped so text stays ~15 px.
export const STOPS: Record<string, [number, number]> = {
  A: [34, 322], '2': [160, 64], '1': [140, 106], '3': [180, 106], '4': [140, 146], '7': [180, 146],
  '5a': [72, 100], '5b': [160, 22], '5c': [262, 170], '5d': [262, 262], '5e': [160, 324], '5f': [58, 220], '6': [262, 324],
};
const LOOP = ['5a', '5b', '5c', '5d', '5e', '5f'];
const FS = 13;

export function TruckMap({ sel, engine, onPick, reducedMotion }: { sel: string; engine: string; onPick: (k: string) => void; reducedMotion: boolean }) {
  const loop = LOOP.map((k) => STOPS[k].join(',')).join(' ') + ' ' + STOPS['5a'].join(',');
  const on = engine.startsWith('ON');
  return (
    <svg viewBox="0 0 320 346" width="100%" role="img" aria-label={`Top view of a truck, front at the top. Selected: ${sel}. Engine ${engine}. Walk-around order: left front, front, right side, right rear, rear, left side.`} style={{ display: 'block', maxWidth: '380px', margin: '0 auto' }}>
      <rect x={0} y={0} width={320} height={346} fill="var(--surface)" />
      {/* truck, front at top: hood, cab, cargo box */}
      <rect x={132} y={42} width={56} height={42} rx={6} fill="var(--surface-2)" stroke="var(--ink)" stroke-width={1.5} />
      <rect x={120} y={86} width={80} height={78} rx={5} fill="var(--surface-2)" stroke="var(--ink)" stroke-width={1.5} />
      <rect x={112} y={170} width={96} height={130} rx={3} fill="var(--surface-2)" stroke="var(--ink)" stroke-width={1.5} />
      <text x={160} y={236} font-size={FS} text-anchor="middle" fill="var(--ink-2)">cargo box</text>
      {[[108, 50], [202, 50], [100, 236], [210, 236], [100, 266], [210, 266]].map(([x, y]) => <rect x={x} y={y} width={10} height={24} rx={2} fill="var(--ink)" />)}
      <circle cx={138} cy={44} r={3} fill="var(--amber)" /><circle cx={182} cy={44} r={3} fill="var(--amber)" />
      <circle cx={118} cy={298} r={3} fill="var(--red)" /><circle cx={202} cy={298} r={3} fill="var(--red)" />
      {/* walk-around loop and approach */}
      <polyline points={loop} fill="none" stroke="var(--accent)" stroke-width={2} stroke-dasharray="6 5" opacity={sel.startsWith('5') ? 1 : 0.5}>
        {!reducedMotion && sel.startsWith('5') && <animate attributeName="stroke-dashoffset" from="22" to="0" dur="1s" repeatCount="indefinite" />}
      </polyline>
      <line x1={50} y1={308} x2={104} y2={276} stroke="var(--ink-2)" stroke-width={1.5} stroke-dasharray="2 4" />
      <text x={4} y={164} font-size={FS} fill="var(--ink-2)">left</text><text x={4} y={180} font-size={FS} fill="var(--ink-2)">(driver)</text>
      <text x={300} y={220} font-size={FS} fill="var(--ink-2)" text-anchor="middle">right</text>
      <text x={316} y={18} font-size={FS} fill="var(--ink-2)" text-anchor="end">↑ front</text>
      {Object.entries(STOPS).map(([k, [x, y]]) => {
        const active = k === sel || (sel === '5' && k.startsWith('5'));
        return (
          <g onClick={() => onPick(k)} style={{ cursor: 'pointer' }} aria-hidden="true">
            <circle cx={x} cy={y} r={20} fill="transparent" />
            <circle cx={x} cy={y} r={17} fill={active ? 'var(--accent)' : 'var(--surface)'} stroke={active ? 'var(--accent)' : 'var(--ink)'} stroke-width={1.5} />
            <text x={x} y={y + 5} font-size={14} font-weight={700} text-anchor="middle" fill={active ? 'var(--accent-ink)' : 'var(--ink)'}>{k}</text>
          </g>
        );
      })}
      <g>
        <rect x={4} y={4} width={120} height={46} rx={6} fill={on ? 'var(--ok-soft)' : 'var(--surface-2)'} stroke={on ? 'var(--ok)' : 'var(--ink-2)'} stroke-width={1.5} />
        <text x={11} y={22} font-size={FS} font-weight={700} fill="var(--ink)">{on ? '● ENGINE ON' : '○ ENGINE OFF'}</text>
        <text x={11} y={41} font-size={FS} fill="var(--ink-2)">{on ? 'running' : engine.includes('key') ? 'key with you' : 'not running'}</text>
      </g>
    </svg>
  );
}

const Verdict = ({ text, good }: { text?: string; good: boolean }) => (text && text.trim() ? <strong class="num" style={{ color: good ? 'var(--ok)' : 'var(--red)', font: '700 1.15rem/1.2 var(--display)', letterSpacing: '.02em' }}>{text}</strong> : null);

/** Tread depth cross-section; v and min in 32nds of an inch. Ribs stand v/32 above the groove floor. The verdict is HTML under the drawing. */
export function Tread({ v, min, verdict }: { v: number; min: number; verdict?: string }) {
  const s = 10, base = 128, rt = base - v * s, ok = v >= min;
  const shown = verdict ?? (ok ? '✓ OK' : '✗ DEFECT');
  return (
    <div class="stack" style={{ gap: '4px', maxWidth: '360px' }}>
      <svg viewBox="0 0 300 162" width="100%" role="img" aria-label={min < 0 ? `Tread gauge reads ${v}/32 inch.` : `Tread gauge reads ${v}/32 inch; minimum here is ${min}/32 inch. ${ok ? 'Meets' : 'Below'} the minimum.`} style={{ display: 'block' }}>
        <rect x={10} y={rt} width={60} height={v * s + 0.01} fill="var(--ink)" /><rect x={110} y={rt} width={60} height={v * s + 0.01} fill="var(--ink)" />
        <rect x={10} y={base} width={160} height={10} fill="var(--ink-2)" />
        <text x={90} y={base + 28} font-size={14} text-anchor="middle" fill="var(--ink-2)">groove floor / tire body</text>
        {min >= 0 && <line x1={4} x2={178} y1={base - min * s} y2={base - min * s} stroke="var(--red)" stroke-width={2} stroke-dasharray="5 3" />}
        <rect x={86} y={rt - 20} width={8} height={v * s + 20} fill="var(--blue)" />
        <rect x={50} y={rt - 20} width={80} height={14} rx={3} fill="var(--blue)" />
        {min >= 0 && <text x={184} y={base - min * s + 5} font-size={14} font-weight={700} fill="var(--red)">min {min}/32</text>}
        <text x={184} y={min < 0 ? Math.max(rt - 6, 16) : Math.max(Math.min(rt - 6, base - min * s - 16), 16)} font-size={14} font-weight={700} fill="var(--ink)">reads {v}/32</text>
      </svg>
      <Verdict text={shown} good={verdict ? !verdict.startsWith('✗') : ok} />
    </div>
  );
}

export function Spring({ leaves, missing, verdict }: { leaves: number; missing: number; verdict?: string }) {
  return (
    <svg viewBox="0 0 300 150" width="100%" role="img" aria-label={`Leaf spring with ${leaves} leaves, ${missing} missing.${verdict ? ' ' + verdict : ''}`} style={{ display: 'block', maxWidth: '360px' }}>
      {Array.from({ length: leaves }, (_, i) => { const w = 260 - i * 24, gone = i >= leaves - missing; return (
        <rect x={150 - w / 2} y={10 + i * 11} width={w} height={8} rx={4} fill={gone ? 'var(--surface)' : 'var(--ink-2)'} stroke={gone ? 'var(--red)' : 'var(--ink)'} stroke-dasharray={gone ? '4 3' : undefined} stroke-width={1.2} />); })}
      <text x={6} y={120} font-size={14} fill="var(--ink)">{leaves} leaves · {missing} missing (dashed)</text>
      {verdict && <text x={6} y={142} font-size={14} font-weight={700} fill="var(--red)">{verdict}</text>}
    </svg>
  );
}

export function Wheel({ inches, verdict }: { inches: number; verdict?: string }) {
  const deg = (inches / (Math.PI * 20)) * 360, r = 58, cx = 76, cy = 80;
  const pt = (a: number) => [cx + r * Math.sin((a * Math.PI) / 180), cy - r * Math.cos((a * Math.PI) / 180)];
  const [x1, y1] = pt(deg), [lx, ly] = pt(10);
  return (
    <svg viewBox="0 0 300 160" width="100%" role="img" aria-label={`20-inch steering wheel with ${inches} inches of free play at the rim, about ${Math.round(deg)} degrees. Limit 10 degrees, about 2 inches.`} style={{ display: 'block', maxWidth: '360px' }}>
      <circle cx={cx} cy={cy} r={r} fill="none" stroke="var(--ink)" stroke-width={8} />
      <line x1={cx - r} x2={cx + r} y1={cy} y2={cy} stroke="var(--ink)" stroke-width={5} /><circle cx={cx} cy={cy} r={12} fill="var(--ink)" />
      <path d={`M ${cx} ${cy - r} A ${r} ${r} 0 0 1 ${x1} ${y1}`} fill="none" stroke={deg > 10 ? 'var(--red)' : 'var(--ok)'} stroke-width={8} />
      <line x1={cx} y1={cy} x2={lx} y2={ly} stroke="var(--amber)" stroke-width={2} stroke-dasharray="3 3" />
      <text x={152} y={44} font-size={14} font-weight={700} fill="var(--ink)">play: {inches} in at rim</text>
      <text x={152} y={64} font-size={14} fill="var(--ink-2)">≈ {Math.round(deg)}°</text>
      <text x={152} y={86} font-size={14} fill="var(--amber-ink)">limit 10° ≈ 2 in</text>
      {verdict && <text x={152} y={114} font-size={14} font-weight={700} fill={deg > 10 ? 'var(--red)' : 'var(--ok)'}>{verdict}</text>}
    </svg>
  );
}

export function Kit({ triangles }: { triangles: number }) {
  return (
    <svg viewBox="0 0 300 130" width="100%" role="img" aria-label={`Cab kit: 1 fire extinguisher, spare electrical fuses, ${triangles} red reflective triangles.`} style={{ display: 'block', maxWidth: '360px' }}>
      <rect x={34} y={20} width={24} height={62} rx={8} fill="var(--red)" /><rect x={40} y={10} width={12} height={12} fill="var(--ink)" />
      <text x={46} y={104} font-size={14} text-anchor="middle" fill="var(--ink)">extinguisher</text>
      <rect x={112} y={38} width={56} height={40} rx={4} fill="var(--surface-2)" stroke="var(--ink)" />
      {[0, 1, 2, 3].map((i) => <rect x={118 + i * 12} y={48} width={8} height={20} rx={2} fill="var(--amber)" stroke="var(--ink)" stroke-width={0.8} />)}
      <text x={140} y={104} font-size={14} text-anchor="middle" fill="var(--ink)">spare fuses</text>
      {Array.from({ length: triangles }, (_, i) => <polygon points={`${204 + i * 44},80 ${224 + i * 44},40 ${244 + i * 44},80`} fill="none" stroke="var(--red)" stroke-width={6} />)}
      <text x={246} y={104} font-size={14} text-anchor="middle" fill="var(--ink)">{triangles} red reflective</text>
      <text x={246} y={122} font-size={14} text-anchor="middle" fill="var(--ink)">triangles</text>
    </svg>
  );
}

export function Air({ minutes, verdict }: { minutes: number; verdict?: string }) {
  const cx = 74, cy = 80, r = 60, ang = (p: number) => -225 + (p / 150) * 270;
  const pt = (p: number, rr = r) => { const a = (ang(p) * Math.PI) / 180; return [cx + rr * Math.cos(a), cy + rr * Math.sin(a)]; };
  const [nx, ny] = pt(90, 48);
  return (
    <svg viewBox="0 0 300 160" width="100%" role="img" aria-label={`Air pressure gauge: 50 to 90 psi took ${minutes} minutes. Handbook: within 3 minutes.`} style={{ display: 'block', maxWidth: '360px' }}>
      <circle cx={cx} cy={cy} r={r + 8} fill="var(--surface-2)" stroke="var(--ink)" stroke-width={2} />
      <line x1={cx} y1={cy} x2={nx} y2={ny} stroke="var(--red)" stroke-width={3} /><circle cx={cx} cy={cy} r={5} fill="var(--ink)" />
      {[0, 30, 60, 90, 120, 150].map((p) => { const [a, b] = pt(p), [c, d] = pt(p, r - 10), [tx, ty] = pt(p, r - 25); return <g><line x1={a} y1={b} x2={c} y2={d} stroke="var(--ink)" stroke-width={2} /><text x={tx} y={ty + 5} font-size={13} text-anchor="middle" fill="var(--ink)" stroke="var(--surface-2)" stroke-width={4} paint-order="stroke">{p}</text></g>; })}
      <text x={cx} y={cy + 44} font-size={13} text-anchor="middle" fill="var(--ink-2)">psi</text>
      <text x={156} y={50} font-size={14} font-weight={700} fill="var(--ink)">50 → 90 psi</text>
      <text x={156} y={72} font-size={14} fill="var(--ink)">took {minutes} min</text>
      <text x={156} y={94} font-size={14} fill="var(--amber-ink)">rule: within 3 min</text>
      {verdict && <text x={156} y={122} font-size={14} font-weight={700} fill={minutes > 3 ? 'var(--red)' : 'var(--ok)'}>{verdict}</text>}
    </svg>
  );
}
