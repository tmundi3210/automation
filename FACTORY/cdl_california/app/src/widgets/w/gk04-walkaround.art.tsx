// SVG art for gk04-walkaround (helper; no meta, so the registry ignores it).
export const STOPS: Record<string, [number, number]> = {
  A: [40, 292], '1': [160, 72], '2': [180, 40], '3': [200, 72], '4': [160, 95], '7': [200, 95],
  '5a': [110, 62], '5b': [180, 12], '5c': [252, 150], '5d': [252, 238], '5e': [180, 284], '5f': [108, 180], '6': [292, 284],
};
const LOOP = ['5a', '5b', '5c', '5d', '5e', '5f'];

export function TruckMap({ sel, engine, onPick, reducedMotion }: { sel: string; engine: string; onPick: (k: string) => void; reducedMotion: boolean }) {
  const loop = LOOP.map((k) => STOPS[k].join(',')).join(' ') + ' ' + STOPS['5a'].join(',');
  const on = engine.startsWith('ON');
  return (
    <svg viewBox="0 0 360 310" width="100%" role="img" aria-label={`Top view of a truck, front at the top. Selected: ${sel}. Engine ${engine}. Walk-around order: left front, front, right side, right rear, rear, left side.`} style={{ display: 'block', maxWidth: '460px', margin: '0 auto' }}>
      <rect x={0} y={0} width={360} height={310} fill="var(--surface)" />
      {/* truck, front at top */}
      <rect x={150} y={28} width={60} height={32} rx={6} fill="var(--surface-2)" stroke="var(--ink)" stroke-width={1.5} />
      <rect x={140} y={58} width={80} height={50} rx={5} fill="var(--surface-2)" stroke="var(--ink)" stroke-width={1.5} />
      <rect x={133} y={114} width={94} height={150} rx={3} fill="var(--surface-2)" stroke="var(--ink)" stroke-width={1.5} />
      <text x={180} y={196} font-size={12} text-anchor="middle" fill="var(--ink-2)">cargo box</text>
      {[[124, 36], [226, 36], [124, 206], [226, 206], [124, 236], [226, 236]].map(([x, y]) => <rect x={x} y={y} width={10} height={24} rx={2} fill="var(--ink)" />)}
      <circle cx={146} cy={28} r={3} fill="var(--amber)" /><circle cx={214} cy={28} r={3} fill="var(--amber)" />
      <circle cx={138} cy={264} r={3} fill="var(--red)" /><circle cx={222} cy={264} r={3} fill="var(--red)" />
      {/* walk-around loop and approach */}
      <polyline points={loop} fill="none" stroke="var(--accent)" stroke-width={2} stroke-dasharray="6 5" opacity={sel.startsWith('5') ? 1 : 0.5}>
        {!reducedMotion && sel.startsWith('5') && <animate attributeName="stroke-dashoffset" from="22" to="0" dur="1s" repeatCount="indefinite" />}
      </polyline>
      <line x1={40} y1={292} x2={140} y2={100} stroke="var(--ink-2)" stroke-width={1.2} stroke-dasharray="2 4" />
      <text x={296} y={200} font-size={11} fill="var(--ink-2)" text-anchor="middle">right</text><text x={64} y={130} font-size={11} fill="var(--ink-2)" text-anchor="middle">left (driver)</text>
      <text x={330} y={18} font-size={11} fill="var(--ink-2)" text-anchor="end">↑ front</text>
      {Object.entries(STOPS).map(([k, [x, y]]) => {
        const active = k === sel || (sel === '5' && k.startsWith('5'));
        return (
          <g onClick={() => onPick(k)} style={{ cursor: 'pointer' }} aria-hidden="true">
            <circle cx={x} cy={y} r={k.length > 1 ? 11 : 10} fill={active ? 'var(--accent)' : 'var(--surface)'} stroke={active ? 'var(--accent)' : 'var(--ink)'} stroke-width={1.5} />
            <text x={x} y={y + 4} font-size={11} font-weight={700} text-anchor="middle" fill={active ? 'var(--accent-ink)' : 'var(--ink)'}>{k}</text>
          </g>
        );
      })}
      <g>
        <rect x={6} y={6} width={112} height={40} rx={6} fill={on ? 'var(--ok-soft)' : 'var(--surface-2)'} stroke={on ? 'var(--ok)' : 'var(--ink-2)'} stroke-width={1.5} />
        <text x={14} y={22} font-size={12} font-weight={700} fill="var(--ink)">{on ? '● ENGINE ON' : '○ ENGINE OFF'}</text>
        <text x={14} y={38} font-size={11} fill="var(--ink-2)">{on ? 'running' : engine.includes('key') ? 'key in your pocket' : 'not running'}</text>
      </g>
    </svg>
  );
}

/** Tread depth cross-section; v and min in 32nds of an inch. Ribs stand v/32 above the groove floor. */
export function Tread({ v, min, verdict }: { v: number; min: number; verdict?: string }) {
  const s = 12, base = 150, rt = base - v * s, ok = v >= min;
  return (
    <svg viewBox="0 0 300 175" width="100%" role="img" aria-label={min < 0 ? `Tread gauge reads ${v}/32 inch.` : `Tread gauge reads ${v}/32 inch; minimum here is ${min}/32 inch. ${ok ? 'Meets' : 'Below'} the minimum.`} style={{ display: 'block', maxWidth: '420px' }}>
      <rect x={20} y={rt} width={70} height={v * s + 0.01} fill="var(--ink)" /><rect x={130} y={rt} width={70} height={v * s + 0.01} fill="var(--ink)" />
      <rect x={20} y={base} width={180} height={10} fill="var(--ink-2)" />
      <text x={110} y={base + 23} font-size={11} text-anchor="middle" fill="var(--ink-2)">groove floor / tire body</text>
      {min >= 0 && <line x1={14} x2={206} y1={base - min * s} y2={base - min * s} stroke="var(--red)" stroke-width={2} stroke-dasharray="5 3" />}
      <rect x={106} y={rt - 20} width={8} height={v * s + 20} fill="var(--blue)" />
      <rect x={66} y={rt - 20} width={88} height={14} rx={3} fill="var(--blue)" />
      {min >= 0 && <text x={212} y={base - min * s + 4} font-size={12} font-weight={700} fill="var(--red)">min {min}/32</text>}
      <text x={212} y={min < 0 ? rt - 8 : Math.min(rt - 8, base - min * s - 14)} font-size={12} font-weight={700} fill="var(--ink)">reads {v}/32</text>
      <text x={212} y={22} font-size={13} font-weight={700} fill={ok ? 'var(--ok)' : 'var(--red)'}>{verdict ?? (ok ? '✓ OK' : '✗ DEFECT')}</text>
    </svg>
  );
}

export function Spring({ leaves, missing, verdict }: { leaves: number; missing: number; verdict?: string }) {
  return (
    <svg viewBox="0 0 300 130" width="100%" role="img" aria-label={`Leaf spring with ${leaves} leaves, ${missing} missing.${verdict ? ' ' + verdict : ''}`} style={{ display: 'block', maxWidth: '420px' }}>
      {Array.from({ length: leaves }, (_, i) => { const w = 260 - i * 24, gone = i >= leaves - missing; return (
        <rect x={150 - w / 2} y={14 + i * 11} width={w} height={8} rx={4} fill={gone ? 'var(--surface)' : 'var(--ink-2)'} stroke={gone ? 'var(--red)' : 'var(--ink)'} stroke-dasharray={gone ? '4 3' : undefined} stroke-width={1.2} />); })}
      <text x={10} y={124} font-size={12} fill="var(--ink)">{leaves} leaves · {missing} missing (dashed)</text>
      {verdict && <text x={290} y={124} font-size={13} font-weight={700} text-anchor="end" fill="var(--red)">{verdict}</text>}
    </svg>
  );
}

export function Wheel({ inches, verdict }: { inches: number; verdict?: string }) {
  const deg = (inches / (Math.PI * 20)) * 360, r = 60, cx = 110, cy = 80;
  const pt = (a: number) => [cx + r * Math.sin((a * Math.PI) / 180), cy - r * Math.cos((a * Math.PI) / 180)];
  const [x1, y1] = pt(deg), [lx, ly] = pt(10);
  return (
    <svg viewBox="0 0 300 160" width="100%" role="img" aria-label={`20-inch steering wheel with ${inches} inches of free play at the rim, about ${Math.round(deg)} degrees. Limit 10 degrees, about 2 inches.`} style={{ display: 'block', maxWidth: '420px' }}>
      <circle cx={cx} cy={cy} r={r} fill="none" stroke="var(--ink)" stroke-width={8} />
      <line x1={cx - r} x2={cx + r} y1={cy} y2={cy} stroke="var(--ink)" stroke-width={5} /><circle cx={cx} cy={cy} r={12} fill="var(--ink)" />
      <path d={`M ${cx} ${cy - r} A ${r} ${r} 0 0 1 ${x1} ${y1}`} fill="none" stroke={deg > 10 ? 'var(--red)' : 'var(--ok)'} stroke-width={8} />
      <line x1={cx} y1={cy} x2={lx} y2={ly} stroke="var(--amber)" stroke-width={2} stroke-dasharray="3 3" />
      <text x={190} y={46} font-size={12} font-weight={700} fill="var(--ink)">play: {inches} in at rim</text>
      <text x={190} y={64} font-size={12} fill="var(--ink-2)">≈ {Math.round(deg)}°</text>
      <text x={190} y={86} font-size={12} fill="var(--amber-ink)">limit 10° ≈ 2 in</text>
      {verdict && <text x={190} y={112} font-size={13} font-weight={700} fill={deg > 10 ? 'var(--red)' : 'var(--ok)'}>{verdict}</text>}
    </svg>
  );
}

export function Kit({ triangles }: { triangles: number }) {
  return (
    <svg viewBox="0 0 300 110" width="100%" role="img" aria-label={`Cab kit: 1 fire extinguisher, spare electrical fuses, ${triangles} red reflective triangles.`} style={{ display: 'block', maxWidth: '420px' }}>
      <rect x={14} y={20} width={24} height={62} rx={8} fill="var(--red)" /><rect x={20} y={10} width={12} height={12} fill="var(--ink)" />
      <text x={26} y={100} font-size={11} text-anchor="middle" fill="var(--ink)">extinguisher</text>
      <rect x={62} y={38} width={56} height={40} rx={4} fill="var(--surface-2)" stroke="var(--ink)" />
      {[0, 1, 2, 3].map((i) => <rect x={68 + i * 12} y={48} width={8} height={20} rx={2} fill="var(--amber)" stroke="var(--ink)" stroke-width={0.8} />)}
      <text x={90} y={100} font-size={11} text-anchor="middle" fill="var(--ink)">spare fuses</text>
      {Array.from({ length: triangles }, (_, i) => <polygon points={`${150 + i * 44},80 ${172 + i * 44},38 ${194 + i * 44},80`} fill="none" stroke="var(--red)" stroke-width={6} />)}
      <text x={172 + (triangles - 1) * 22} y={100} font-size={11} text-anchor="middle" fill="var(--ink)">{triangles} red reflective triangles</text>
    </svg>
  );
}

export function Air({ minutes, verdict }: { minutes: number; verdict?: string }) {
  const cx = 80, cy = 80, r = 60, ang = (p: number) => -225 + (p / 150) * 270;
  const pt = (p: number, rr = r) => { const a = (ang(p) * Math.PI) / 180; return [cx + rr * Math.cos(a), cy + rr * Math.sin(a)]; };
  const [nx, ny] = pt(90, 48);
  return (
    <svg viewBox="0 0 300 160" width="100%" role="img" aria-label={`Air pressure gauge: 50 to 90 psi took ${minutes} minutes. Handbook: within 3 minutes.`} style={{ display: 'block', maxWidth: '420px' }}>
      <circle cx={cx} cy={cy} r={r + 8} fill="var(--surface-2)" stroke="var(--ink)" stroke-width={2} />
      {[0, 30, 60, 90, 120, 150].map((p) => { const [a, b] = pt(p), [c, d] = pt(p, r - 10), [tx, ty] = pt(p, r - 22); return <g><line x1={a} y1={b} x2={c} y2={d} stroke="var(--ink)" stroke-width={2} /><text x={tx} y={ty + 4} font-size={11} text-anchor="middle" fill="var(--ink)">{p}</text></g>; })}
      <line x1={cx} y1={cy} x2={nx} y2={ny} stroke="var(--red)" stroke-width={3} /><circle cx={cx} cy={cy} r={5} fill="var(--ink)" />
      <text x={cx} y={cy + 40} font-size={11} text-anchor="middle" fill="var(--ink-2)">psi</text>
      <text x={170} y={50} font-size={12} font-weight={700} fill="var(--ink)">50 → 90 psi</text>
      <text x={170} y={70} font-size={12} fill="var(--ink)">took {minutes} min</text>
      <text x={170} y={90} font-size={12} fill="var(--amber-ink)">rule: within 3 min</text>
      {verdict && <text x={170} y={116} font-size={13} font-weight={700} fill={minutes > 3 ? 'var(--red)' : 'var(--ok)'}>{verdict}</text>}
    </svg>
  );
}
