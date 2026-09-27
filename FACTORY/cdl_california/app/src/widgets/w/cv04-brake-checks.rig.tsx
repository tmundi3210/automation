// Side-view rig for cv04-brake-checks. Facts: CV-04 (DL 650 pp. 6-7, 6-16, 6-17).
// viewBox 360 wide; every label is ≥ 14 units so it stays ≥ 11 px in a 294 px lesson column.
export interface Rig {
  psi: string; knob: 'in' | 'out'; engine: boolean; pedal: 'up' | 'pumping'; hand: boolean; parking: boolean; chocks: boolean;
  emValve: boolean; svcValve: boolean; emAir: boolean; svcAir: boolean; move: 'still' | 'slow' | 'tug'; trailer: 'free' | 'held' | 'grab' | 'moves' | 'none'; flag?: string; bad?: boolean;
  /** Test 1 is drawn as doubles: tractor, trailer 1, dolly, last trailer. */
  doubles?: boolean;
  /** Shut-off valves at the back of trailer 1 closed by mistake: air stops there (Test 1 failure). */
  midShut?: boolean;
}
export const IDLE: Rig = { psi: 'low', knob: 'out', engine: true, pedal: 'up', hand: false, parking: false, chocks: false, emValve: false, svcValve: false, emAir: false, svcAir: false, move: 'still', trailer: 'none' };

const TIRE = '#262b28';
const STATUS: Record<Rig['trailer'], string> = { free: 'Trailer rolls freely →', held: 'Tugging → trailer brakes HOLD', grab: 'Rolling slowly → brakes GRAB', moves: '⚠ Trailer MOVES — not holding', none: '' };

function Wheel({ x, r = 12 }: { x: number; r?: number }) {
  return <g><circle cx={x} cy={130} r={r} fill={TIRE} stroke="var(--ink-2)" stroke-width="1.5" /><circle cx={x} cy={130} r={r * 0.45} fill="var(--surface-2)" /></g>;
}
function Valve({ x, y, open }: { x: number; y: number; open: boolean }) {
  return <g><circle cx={x} cy={y} r="6" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" /><line x1={open ? x - 5 : x} y1={open ? y : y - 5} x2={open ? x + 5 : x} y2={open ? y : y + 5} stroke="var(--ink)" stroke-width="2" /></g>;
}
function AirLine({ d, on, color }: { d: string; on: boolean; color: string }) {
  return <path d={d} fill="none" stroke={on ? color : 'var(--ink-2)'} stroke-width={on ? 4 : 2} stroke-dasharray={on ? undefined : '2 4'} stroke-linecap="round" />;
}

export function RigView({ r, motion }: { r: Rig; motion: boolean }) {
  const D = !!r.doubles;
  const rearEm = r.emAir && !r.midShut, rearSvc = r.svcAir && !r.midShut;
  const status = r.trailer !== 'none' ? STATUS[r.trailer] : r.move === 'slow' ? 'Moving forward slowly →' : r.move === 'tug' ? 'Tractor tugs gently →' : '';
  const rear = (open: boolean, air: boolean) => open ? (air ? 'air!' : 'none') : 'shut';
  const label = `Rig${D ? ' (tractor, trailer 1, dolly, last trailer)' : ''}: air ${r.psi}, knob ${r.knob}, engine ${r.engine ? 'on' : 'off'}${r.hand ? ', trailer hand valve on' : ''}${r.parking ? ', parking brake set' : ''}${r.chocks ? ', wheels chocked' : ''}.${D ? ` Trailer 1 rear shut-off valves ${r.midShut ? 'SHUT' : 'open'}. Last trailer rear valves: emergency ${rear(r.emValve, rearEm)}, service ${rear(r.svcValve, rearSvc)}.` : ''} ${status} ${r.flag ?? ''}`;
  const t = (x: number, y: number, s: string, o: Record<string, string | number> = {}) => <text x={x} y={y} font-size="14" fill="var(--ink)" {...o}>{s}</text>;
  // x where the tractor's hoses meet the first trailer, and where each line ends at the rear
  const hx = D ? 66 : 108, backX = D ? 52 : 64;
  return (
    <svg viewBox="0 0 360 240" width="100%" role="img" aria-label={label} style={{ maxWidth: '440px', display: 'block', margin: '0 auto' }}>
      <rect x="0" y="0" width="360" height="240" rx="8" fill="var(--surface-2)" />
      {r.flag && <text x="180" y="21" font-size="15" font-weight="700" text-anchor="middle" fill={r.bad ? 'var(--red)' : 'var(--ok)'}>{r.flag}</text>}
      <line x1="4" y1="143" x2="356" y2="143" stroke="var(--ink-2)" stroke-width="1.5" />
      <g>
        {motion && r.move !== 'still' && <animateTransform attributeName="transform" type="translate" values={r.move === 'slow' ? '0 0;12 0;0 0' : '0 0;4 0;0 0'} dur={r.move === 'slow' ? '2.4s' : '0.8s'} repeatCount="indefinite" />}
        {D ? (
          <g>
            <path d="M6 118 V70 Q6 50 24 50 H52 V118 Z" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
            <rect x="12" y="58" width="30" height="18" rx="3" fill="var(--blue-soft)" stroke="var(--ink-2)" />
            <rect x="52" y="108" width="16" height="10" fill="var(--ink-2)" />
            <Wheel x={22} r={11} /><Wheel x={48} r={11} />
            <rect x="66" y="44" width="96" height="74" rx="3" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
            {t(114, 104, 'TRAILER 1', { 'text-anchor': 'middle', 'font-weight': 700 })}
            <Wheel x={130} r={11} /><Wheel x={151} r={11} />
            <line x1="162" y1="114" x2="176" y2="114" stroke="var(--ink-2)" stroke-width="3" />
            <rect x="172" y="106" width="22" height="8" rx="2" fill="var(--ink-2)" />
            <Wheel x={183} r={11} />
            <rect x="198" y="44" width="102" height="74" rx="3" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
            {t(249, 90, 'LAST', { 'text-anchor': 'middle', 'font-weight': 700 })}
            {t(249, 106, 'TRAILER', { 'text-anchor': 'middle', 'font-weight': 700 })}
            <Wheel x={262} r={11} /><Wheel x={286} r={11} />
          </g>
        ) : (
          <g>
            <path d="M10 118 V66 Q10 46 30 46 H64 V80 H108 V118 Z" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
            <rect x="18" y="54" width="38" height="22" rx="3" fill="var(--blue-soft)" stroke="var(--ink-2)" />
            <Wheel x={34} /><Wheel x={88} />
            <rect x="108" y="44" width="192" height="74" rx="3" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
            {t(204, 100, 'TRAILER', { 'text-anchor': 'middle', 'font-weight': 700 })}
            <Wheel x={256} /><Wheel x={284} />
          </g>
        )}
        {/* air lines: tractor → trailer(s) → rear shut-off valves */}
        <AirLine d={`M${backX} 62 C${backX + 8} 56 ${hx - 8} 54 ${hx} 54${D ? ' H158' : ''}`} on={r.emAir} color="var(--red)" />
        <AirLine d={`M${backX} 70 C${backX + 8} 68 ${hx - 8} 66 ${hx} 66${D ? ' H158' : ''}`} on={r.svcAir} color="var(--blue)" />
        {D && <>
          <AirLine d="M158 54 H300" on={rearEm} color="var(--red)" />
          <AirLine d="M158 66 H300" on={rearSvc} color="var(--blue)" />
          <Valve x={158} y={54} open={!r.midShut} /><Valve x={158} y={66} open={!r.midShut} />
          {r.midShut && t(114, 86, 'valves SHUT', { 'text-anchor': 'middle', 'font-weight': 700, fill: 'var(--red)' })}
        </>}
        <line x1="300" y1="54" x2="306" y2="54" stroke="var(--red)" stroke-width="4" /><line x1="300" y1="66" x2="306" y2="66" stroke="var(--blue)" stroke-width="4" />
        <Valve x={312} y={54} open={r.emValve} /><Valve x={312} y={66} open={r.svcValve} />
      </g>
      {D && <>
        {t(322, 50, rear(r.emValve, rearEm), { 'font-weight': r.emValve ? 700 : 400, fill: r.emValve ? 'var(--red)' : 'var(--ink-2)' })}
        {t(322, 76, rear(r.svcValve, rearSvc), { 'font-weight': r.svcValve ? 700 : 400, fill: r.svcValve ? 'var(--blue)' : 'var(--ink-2)' })}
      </>}
      {r.chocks && <><path d="M56 143 l7 -11 l7 11 Z" fill="var(--amber)" stroke="var(--ink)" /><path d={D ? 'M268 143 l7 -11 l7 11 Z' : 'M264 143 l7 -11 l7 11 Z'} fill="var(--amber)" stroke="var(--ink)" /></>}
      {status && <text x="180" y="165" font-size="14" font-weight="700" text-anchor="middle" fill={r.trailer === 'moves' ? 'var(--red)' : r.trailer === 'none' ? 'var(--ink)' : 'var(--ok)'}>{status}</text>}
      {/* dashboard strip */}
      <rect x="6" y="176" width="348" height="58" rx="6" fill="var(--surface)" stroke="var(--line)" />
      <polygon points="18,184 26,184 31,189 31,197 26,202 18,202 13,197 13,189" fill="var(--red)" stroke="var(--ink)" />
      {t(38, 198, `Knob ${r.knob.toUpperCase()}`, { 'font-weight': 700 })}
      {t(116, 198, `Air: ${r.psi}`)}
      {t(348, 198, `Engine ${r.engine ? 'ON' : 'OFF'}`, { 'text-anchor': 'end' })}
      {t(14, 223, r.parking ? 'Park brake SET' : 'Park brake off')}
      {t(132, 223, r.pedal === 'pumping' ? 'Pumping pedal' : r.chocks ? 'Chocked' : '')}
      {t(348, 223, `Hand valve ${r.hand ? 'ON' : 'off'}`, { 'text-anchor': 'end', 'font-weight': r.hand ? 700 : 400, fill: r.hand ? 'var(--blue)' : 'var(--ink)' })}
    </svg>
  );
}
