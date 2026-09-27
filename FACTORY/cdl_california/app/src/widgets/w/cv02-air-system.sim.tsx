// Air-system model + live schematic for cv02-air-system. Facts: CV-02 (DL 650 pp. 6-5 to 6-8).
import type { JSX } from 'preact';

/** Example maker setting inside the handbook band (the TPV closes somewhere between 20 and 45 psi, p. 6-5). */
export const POP = 32;
export const BAND = [20, 45] as const;
export const MAX_PSI = 120;

export interface AirState {
  psi: number; knobIn: boolean; pedal: boolean; hand: boolean;
  emBreak: boolean; svcBreak: boolean; crossed: boolean;
  spring: boolean; leaked: boolean; doubles: boolean; v1Open: boolean; vLastOpen: boolean;
}
export const START: AirState = { psi: 100, knobIn: true, pedal: false, hand: false, emBreak: false, svcBreak: false, crossed: false, spring: true, leaked: false, doubles: false, v1Open: true, vLastOpen: false };

export interface UnitOut { supply: boolean; signal: boolean; tank: 'charged' | 'stored' | 'empty'; emerg: boolean; service: boolean; noBrakes: boolean }
export interface AirOut { tpv: boolean; redT: boolean; blueT: boolean; leakingSvc: boolean; units: UnitOut[]; escapeRear: boolean; msgs: { t: string; p: string; tone: 'ok' | 'warn' | 'bad' }[] }

function unit(supply: boolean, signal: boolean, s: AirState): UnitOut {
  const tankAir = supply || (!s.spring && !s.leaked);
  const emerg = s.spring ? !supply : !supply && tankAir;
  return { supply, signal, tank: supply ? 'charged' : tankAir ? 'stored' : 'empty', emerg, service: signal && supply && !s.crossed, noBrakes: !s.spring && !supply && !tankAir };
}

export function simulate(s: AirState): AirOut {
  const tpv = s.knobIn && s.psi > POP && !s.emBreak;
  const apply = s.pedal || s.hand;
  const redT = tpv, blueT = tpv && apply;
  // At the glad hands: crossed lines swap what the trailer receives.
  const sup1 = s.crossed ? false : redT && !s.emBreak;
  const sig1 = s.crossed ? redT : blueT && !s.svcBreak;
  const units = [unit(sup1, sig1, s)];
  if (s.doubles) units.push(unit(sup1 && s.v1Open, sig1 && s.v1Open, s));
  const last = units[units.length - 1];
  const escapeRear = s.doubles && s.vLastOpen && (last.supply || last.signal);
  const leakingSvc = s.svcBreak && blueT && !s.crossed;
  const msgs: AirOut['msgs'] = [];
  const band = `${BAND[0]}–${BAND[1]} psi`;
  if (tpv) msgs.push({ t: 'Knob pushed in → tractor protection valve open: air goes to the trailer.', p: '6-5', tone: 'ok' });
  else if (s.emBreak) msgs.push({ t: 'Emergency line lost its air → tractor protection valve closed and the knob popped out. Trailer emergency brakes came on — you could lose control.', p: '6-5', tone: 'bad' });
  else if (s.psi <= POP) msgs.push({ t: `Pressure fell into the ${band} band (this truck’s maker set ${POP} psi) → knob popped out by itself, valve closed: no air can leave the tractor, the trailer emergency line is vented, trailer emergency brakes on.${s.svcBreak ? ' The broken service line drained the air fast the moment you braked.' : ''}`, p: s.svcBreak ? '6-5, 6-7' : '6-5', tone: 'bad' });
  else msgs.push({ t: 'Knob pulled out → air to the trailer is shut off and the trailer emergency brakes come on.', p: '6-5', tone: 'warn' });
  if (s.svcBreak && tpv && !apply) msgs.push({ t: 'Service line is apart, yet nothing shows. A major service line leak may go unnoticed until you brake. Press the brake to see.', p: '6-7', tone: 'warn' });
  if (leakingSvc) msgs.push({ t: 'You braked: air rushes out of the broken service line and tank pressure falls fast. If it falls low enough, the trailer emergency brakes come on.', p: '6-7', tone: 'bad' });
  if (s.crossed && tpv) {
    msgs.push({ t: 'Lines crossed: supply air goes into the service line instead of filling the trailer air tanks.', p: '6-6', tone: 'bad' });
    msgs.push(s.spring
      ? { t: 'Trailer has spring brakes: no air reaches them, so they will not release. Knob in but spring brakes stay on → check the line connections.', p: '6-6', tone: 'bad' }
      : s.leaked
        ? { t: 'Old trailer, no spring brakes, tank air leaked away: no emergency brakes, wheels turn freely. You could drive away with NO trailer brakes. Always test before driving.', p: '6-6', tone: 'bad' }
        : { t: 'Old trailer: its emergency brakes hold only on stored tank air, and that air leaks away.', p: '6-6, 6-7', tone: 'bad' });
  }
  if (s.hand) msgs.push({ t: 'Trailer hand valve: trailer brakes only. Use it only to test the trailer brakes — never while driving (the trailer can skid), never to park (air leaks off and the brakes let go).', p: '6-5', tone: 'warn' });
  if (s.pedal && tpv && !s.svcBreak && !s.crossed) msgs.push({ t: 'Foot brake: the harder you press, the more service-line pressure. It tells the relay valves how much tank air to send to the trailer brakes — every brake on the rig works together.', p: '6-5, 6-6', tone: 'ok' });
  if (!s.spring && !units[0].supply && !s.crossed) msgs.push(s.leaked
    ? { t: 'No spring brakes and the tank air has leaked away: NO brakes, the trailer can roll. Always chock the wheels when you park a trailer without spring brakes.', p: '6-7', tone: 'bad' }
    : { t: 'No spring brakes: the emergency brakes run on air stored in the trailer tank and hold only while it lasts. No parking brake — chock the wheels when parked.', p: '6-7', tone: 'warn' });
  if (s.doubles && !s.v1Open) msgs.push({ t: 'Shut-off valves at the back of trailer 1 are closed → no air reaches the dolly and trailer 2. Keep them open; only the last trailer’s are closed.', p: '6-7, 6-17', tone: 'bad' });
  if (escapeRear) msgs.push({ t: 'Shut-off valves at the back of the LAST trailer are open → air escapes out the back. Those must be closed.', p: '6-7', tone: 'bad' });
  else if (s.doubles && s.v1Open && !s.vLastOpen) msgs.push({ t: 'Shut-off valves set right: open everywhere except the back of the last trailer (closed).', p: '6-7', tone: 'ok' });
  return { tpv, redT, blueT, leakingSvc, units, escapeRear, msgs };
}

/* ---------- drawing ---------- */
const T = (p: JSX.SVGAttributes<SVGTextElement> & { children: any }) => <text font-size="13.5" fill="var(--ink)" {...p} />;
function Line({ d, on, color, motion }: { d: string; on: boolean; color: string; motion: boolean }) {
  return on
    ? <path d={d} fill="none" stroke={color} stroke-width="5" stroke-linecap="round" stroke-dasharray={motion ? '10 5' : undefined}>
        {motion && <animate attributeName="stroke-dashoffset" from="30" to="0" dur="0.9s" repeatCount="indefinite" />}
      </path>
    : <path d={d} fill="none" stroke="var(--ink-2)" stroke-width="2" stroke-dasharray="2 5" />;
}
function Octagon({ x, y, r }: { x: number; y: number; r: number }) {
  const pts = Array.from({ length: 8 }, (_, i) => { const a = Math.PI / 8 + (i * Math.PI) / 4; return `${x + r * Math.cos(a)},${y + r * Math.sin(a)}`; }).join(' ');
  return <polygon points={pts} fill="var(--red)" stroke="var(--ink)" stroke-width="1.5" />;
}
function Valve({ x, y, open }: { x: number; y: number; open: boolean }) {
  return <g><circle cx={x} cy={y} r="8" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" /><line x1={open ? x : x - 6} y1={open ? y - 6 : y} x2={open ? x : x + 6} y2={open ? y + 6 : y} stroke="var(--ink)" stroke-width="2.5" /></g>;
}
const brakeText = (u: UnitOut, spring: boolean) => u.noBrakes ? 'NO BRAKES — wheels free' : u.emerg ? (spring ? 'Spring brakes ON' : 'Emergency brakes ON (tank air)') : u.service ? 'Service brakes APPLIED' : 'Brakes released';

function UnitDraw({ y, u, s, o, idx, motion }: { y: number; u: UnitOut; s: AirState; o: AirOut; idx: number; motion: boolean }) {
  const name = s.doubles ? (idx === 0 ? 'TRAILER 1' : 'CONVERTER DOLLY + TRAILER 2') : 'TRAILER';
  const bad = u.noBrakes, on = u.emerg || u.service;
  const isLast = idx === o.units.length - 1;
  return (
    <g>
      <rect x="4" y={y} width="352" height={s.doubles ? 164 : 136} rx="8" fill="var(--surface-2)" stroke="var(--ink-2)" stroke-width="1.5" />
      <T x="36" y={y + 18} font-weight="700">{name}</T>
      <g aria-hidden="true"><circle cx="18" cy={y + 14} r="7" fill="var(--amber)" stroke="var(--ink)" stroke-width="1" /></g>
      <T x="12" y={y + 92} font-size="12.5" fill="var(--ink-2)">▲ ABS lamp:</T>
      <T x="12" y={y + 106} font-size="12.5" fill="var(--ink-2)">yellow, left</T>
      <T x="12" y={y + 120} font-size="12.5" fill="var(--ink-2)">side, corner</T>
      {/* tank */}
      <rect x="22" y={y + 32} width="136" height="34" rx="16" fill={u.tank === 'empty' ? 'var(--surface)' : 'var(--red-soft)'} stroke="var(--ink)" stroke-width="1.5" />
      <T x="90" y={y + 47} text-anchor="middle" font-weight="700">Trailer air tank</T>
      <T x="90" y={y + 61} text-anchor="middle" font-size="12.5">{u.tank === 'charged' ? 'filling from red line' : u.tank === 'stored' ? 'stored air only' : 'EMPTY'}</T>
      {/* relay */}
      <rect x="196" y={y + 36} width="96" height="30" rx="4" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
      <T x="244" y={y + 55} text-anchor="middle">Relay valve</T>
      <Line d={`M158 ${y + 51} H196`} on={u.tank !== 'empty'} color="var(--red)" motion={motion && u.service} />
      {/* brakes */}
      <Line d={`M244 ${y + 66} V${y + 84}`} on={u.service} color="var(--ink)" motion={false} />
      {s.doubles && <><Line d={`M90 ${y + 66} V${y + 140}`} on={u.supply} color="var(--red)" motion={motion} /><Line d={`M244 ${y + 66} V${y + 140}`} on={u.signal} color="var(--blue)" motion={motion} /></>}
      <rect x="120" y={y + 84} width="228" height="38" rx="6" fill={bad ? 'var(--red-soft)' : on ? 'var(--amber-soft)' : 'var(--surface)'} stroke={bad ? 'var(--red)' : 'var(--ink)'} stroke-width={bad ? 2.5 : 1.5} />
      <T x="234" y={y + 100} text-anchor="middle" font-weight="700">{bad ? '⚠ ' : on ? '■ ' : '○ '}{brakeText(u, s.spring)}</T>
      <T x="234" y={y + 115} text-anchor="middle" font-size="12.5" fill="var(--ink-2)">{s.spring ? 'service + spring brakes' : 'air emergency · no parking brake'}</T>
      {s.doubles && (
        <g>
          <Valve x={90} y={y + 148} open={isLast ? s.vLastOpen : s.v1Open} /><Valve x={244} y={y + 148} open={isLast ? s.vLastOpen : s.v1Open} />
          <T x="167" y={y + 152} text-anchor="middle" font-size="12.5" font-weight="700">shut-off: {(isLast ? s.vLastOpen : s.v1Open) ? 'OPEN' : 'CLOSED'}</T>
        </g>
      )}
    </g>
  );
}

export function AirDiagram({ s, motion }: { s: AirState; motion: boolean }) {
  const o = simulate(s);
  const u1 = o.units[0];
  const tY = 250, uH = 180;
  const H = tY + (s.doubles ? uH + 166 : 140) + (o.escapeRear ? 22 : 0);
  const gx = (p: number) => 40 + (Math.min(p, MAX_PSI) / MAX_PSI) * 100;
  const knobOut = !o.tpv;
  const label = `Air system schematic. Tractor pressure ${s.psi} psi. Tractor protection valve ${o.tpv ? 'open' : 'closed'}. ${o.units.map((u, i) => `Unit ${i + 1}: ${brakeText(u, s.spring)}`).join('. ')}.`;
  // trailer-side line x positions: red to tank at 90, blue to relay at 244
  const redIn = s.crossed ? 244 : 90, blueIn = s.crossed ? 90 : 244;
  return (
    <svg viewBox={`0 0 360 ${H}`} width="100%" role="img" aria-label={label} style={{ maxWidth: '520px', display: 'block', margin: '0 auto' }}>
      <rect x="4" y="4" width="352" height="150" rx="8" fill="var(--surface-2)" stroke="var(--ink-2)" stroke-width="1.5" />
      <T x="14" y="22" font-weight="700">TRACTOR</T>
      <g aria-hidden="true"><circle cx="328" cy="18" r="6" fill="var(--surface)" stroke="var(--ink-2)" /><circle cx="344" cy="18" r="6" fill="var(--surface)" stroke="var(--ink-2)" /></g>
      <T x="316" y="22" text-anchor="end" font-size="12.5" fill="var(--ink-2)">dummy couplers</T>
      <rect x="22" y="30" width="134" height="28" rx="14" fill={s.psi > 0 ? 'var(--red-soft)' : 'var(--surface)'} stroke="var(--ink)" stroke-width="1.5" />
      <T x="89" y="49" text-anchor="middle" font-weight="700" class="num">Air tanks {s.psi} psi</T>
      <rect x="40" y="66" width="100" height="10" fill="var(--surface)" stroke="var(--ink-2)" />
      <rect x={gx(BAND[0])} y="66" width={gx(BAND[1]) - gx(BAND[0])} height="10" fill="var(--amber)" />
      <path d={`M${gx(s.psi)} 62 v18`} stroke="var(--ink)" stroke-width="3" />
      <T x="40" y="94" font-size="12.5" fill="var(--ink-2)">▲ 20–45 psi band</T>
      <Octagon x={186} y={52} r={16} />
      <T x="186" y="56" text-anchor="middle" font-size="12.5" font-weight="700" fill="#fff">{knobOut ? 'OUT' : 'IN'}</T>
      <T x="208" y="42" font-size="12.5">Trailer air supply</T>
      <T x="208" y="56" font-size="12.5">(red 8-sided knob)</T>
      <T x="208" y="70" font-size="12.5" font-weight="700">{knobOut ? 'popped/pulled OUT' : 'pushed IN'}</T>
      <T x="176" y="94" font-size="12.5">Foot brake: <tspan font-weight="700">{s.pedal ? 'PRESSED' : 'up'}</tspan></T>
      <T x="176" y="108" font-size="12.5">Hand valve: <tspan font-weight="700">{s.hand ? 'ON (trailer only)' : 'off'}</tspan></T>
      <T x="176" y="122" font-size="12.5">Tractor brakes: <tspan font-weight="700">{s.pedal && s.psi > 0 ? 'applied' : 'released'}</tspan></T>
      <Line d="M28 58 V129 H36" on={s.psi > 0} color="var(--red)" motion={motion && o.tpv} />
      <rect x="36" y="112" width="128" height="34" rx="4" fill={o.tpv ? 'var(--ok-soft)' : 'var(--red-soft)'} stroke="var(--ink)" stroke-width="1.5" />
      <T x="100" y="126" text-anchor="middle" font-size="12.5" font-weight="700">Tractor protection</T>
      <T x="100" y="140" text-anchor="middle" font-size="12.5" font-weight="700">valve: {o.tpv ? 'OPEN' : 'CLOSED'}</T>
      {/* tractor-side lines to glad hands */}
      <Line d="M90 146 V196" on={o.redT} color="var(--red)" motion={motion} />
      <Line d="M164 138 H244 V196" on={o.blueT} color="var(--blue)" motion={motion} />
      <T x="98" y="172" font-size="12.5" fill="var(--red)" font-weight="700">EMERGENCY /</T>
      <T x="98" y="186" font-size="12.5" fill="var(--red)" font-weight="700">supply (red)</T>
      <T x="252" y="172" font-size="12.5" fill="var(--blue)" font-weight="700">SERVICE /</T>
      <T x="252" y="186" font-size="12.5" fill="var(--blue)" font-weight="700">control (blue)</T>
      {/* glad hands */}
      <rect x="76" y="196" width="28" height="12" rx="3" fill="var(--red)" stroke="var(--ink)" /><rect x="230" y="196" width="28" height="12" rx="3" fill="var(--blue)" stroke="var(--ink)" />
      <T x="167" y="208" text-anchor="middle" font-size="12.5">◄ glad hands ►</T>
      <Line d={`M90 208 C90 226 ${redIn} 222 ${redIn} ${tY - 2}`} on={o.redT && !s.emBreak} color="var(--red)" motion={motion} />
      <Line d={`M244 208 C244 226 ${blueIn} 222 ${blueIn} ${tY - 2}`} on={o.blueT && !s.svcBreak} color="var(--blue)" motion={motion} />

      {s.emBreak && <T x="20" y="232" font-size="12.5" font-weight="700" fill="var(--red)">✕ red line broken</T>}
      {s.svcBreak && <T x="340" y="232" text-anchor="end" font-size="12.5" font-weight="700" fill="var(--blue)">✕ blue line apart{o.leakingSvc ? ' — air rushing out' : ''}</T>}
      {s.crossed && <g><rect x="112" y="214" width="110" height="18" rx="4" fill="var(--surface)" stroke="var(--red)" /><T x="167" y="227" text-anchor="middle" font-size="12.5" font-weight="700" fill="var(--red)">✕ LINES CROSSED</T></g>}
      {/* inside trailer: supply to tank, signal to relay */}
      <Line d={`M90 ${tY - 2} V${tY + 32}`} on={s.crossed ? u1.signal : u1.supply} color={s.crossed ? 'var(--blue)' : 'var(--red)'} motion={motion} />
      <Line d={`M244 ${tY - 2} V${tY + 36}`} on={s.crossed ? u1.supply || o.redT : u1.signal} color={s.crossed ? 'var(--red)' : 'var(--blue)'} motion={motion} />
      {o.units.map((u, i) => <UnitDraw y={tY + i * uH} u={u} s={s} o={o} idx={i} motion={motion} />)}
      {s.doubles && <>
        <Line d={`M90 ${tY + 156} V${tY + uH + 32}`} on={o.units[1].supply} color="var(--red)" motion={motion} />
        <Line d={`M244 ${tY + 156} V${tY + uH + 36}`} on={o.units[1].signal} color="var(--blue)" motion={motion} />
      </>}
      {o.escapeRear && <T x="180" y={H - 8} text-anchor="middle" font-weight="700" fill="var(--red)">⚠ air escaping out the back of the rig</T>}
    </svg>
  );
}
