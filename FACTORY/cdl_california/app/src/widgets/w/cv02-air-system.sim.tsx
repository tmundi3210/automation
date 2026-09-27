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

export function simulate(s: AirState, hints = false): AirOut {
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
  const canPushIn = !s.knobIn && !s.emBreak && s.psi > POP;
  if (tpv) msgs.push({ t: 'Knob pushed in → tractor protection valve open: air goes to the trailer.', p: '6-5', tone: 'ok' });
  else if (s.emBreak) msgs.push({ t: 'Emergency line lost its air → tractor protection valve closed and the knob popped out. Trailer emergency brakes came on — you could lose control.', p: '6-5', tone: 'bad' });
  else if (s.psi < BAND[0]) msgs.push({ t: `Pressure is ${s.psi} psi — below the ${band} band. The knob already popped out on the way down (this truck’s maker set ${POP} psi): valve closed, no air can leave the tractor, trailer emergency brakes on.`, p: '6-5', tone: 'bad' });
  else if (s.psi <= POP) msgs.push({ t: `Pressure fell into the ${band} band (this truck’s maker set ${POP} psi) → knob popped out by itself, valve closed: no air can leave the tractor, the trailer emergency line is vented, trailer emergency brakes on.${s.svcBreak ? ' The broken service line drained the air fast the moment you braked.' : ''}`, p: s.svcBreak ? '6-5, 6-7' : '6-5', tone: 'bad' });
  else msgs.push({ t: 'Knob out → air to the trailer is shut off and the trailer emergency brakes are on.', p: '6-5', tone: 'warn' });
  if (hints && !tpv && !s.emBreak && s.psi <= POP) msgs.push({ t: `Knob will not stay in yet: in this sim the valve stays closed until the pressure is above ${POP} psi. Slide the pressure up, then push the knob in.`, p: '6-5', tone: 'warn' });
  if (hints && canPushIn) msgs.push({ t: (s.crossed || s.svcBreak) ? 'The knob is out, so no air reaches the glad hands and this fault cannot show. Push the knob in to see it.' : 'Push the knob in to recharge the trailer.', p: '6-5', tone: 'warn' });
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

/* ---------- drawing (viewBox 360 wide; every label ≥ 14 units so it stays ≥ 11 px in a 294 px lesson column) ---------- */
const T = (p: JSX.SVGAttributes<SVGTextElement> & { children: any }) => <text font-size="14" fill="var(--ink)" {...p} />;
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
  return <g><circle cx={x} cy={y} r="9" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" /><line x1={open ? x : x - 6} y1={open ? y - 6 : y} x2={open ? x : x + 6} y2={open ? y + 6 : y} stroke="var(--ink)" stroke-width="2.5" /></g>;
}
/** A label on a surface chip so lines passing underneath never cut through the text. */
function Tag({ x, y, w, anchor = 'start', color, children }: { x: number; y: number; w: number; anchor?: 'start' | 'middle' | 'end'; color: string; children: any }) {
  const left = anchor === 'start' ? x - 4 : anchor === 'end' ? x - w + 4 : x - w / 2;
  return <g><rect x={left} y={y - 16} width={w} height="22" rx="5" fill="var(--surface)" stroke={color} stroke-width="1.5" /><T x={x} y={y} text-anchor={anchor} font-weight="700" fill={color}>{children}</T></g>;
}
const brakeText = (u: UnitOut, spring: boolean) => u.noBrakes ? 'NO BRAKES — wheels free' : u.emerg ? (spring ? 'Spring brakes ON' : 'Emergency brakes ON') : u.service ? 'Service brakes APPLIED' : 'Brakes released';

const X_RED = 90, X_BLUE = 250, T_Y = 282, U_H = 186;

function UnitDraw({ y, u, s, o, idx, motion }: { y: number; u: UnitOut; s: AirState; o: AirOut; idx: number; motion: boolean }) {
  const name = s.doubles ? (idx === 0 ? 'TRAILER 1' : 'DOLLY + TRAILER 2') : 'TRAILER';
  const bad = u.noBrakes, on = u.emerg || u.service;
  const isLast = idx === o.units.length - 1;
  const vOpen = isLast ? s.vLastOpen : s.v1Open;
  return (
    <g>
      <rect x="4" y={y} width="352" height={s.doubles ? 174 : 150} rx="8" fill="var(--surface-2)" stroke="var(--ink-2)" stroke-width="1.5" />
      <g aria-hidden="true"><circle cx="18" cy={y + 16} r="7" fill="var(--amber)" stroke="var(--ink)" stroke-width="1" /></g>
      <T x="170" y={y + 21} text-anchor="middle" font-size={s.doubles ? 14 : 15} font-weight="700">{name}</T>
      {['ABS lamp:', 'yellow,', 'left side', 'corner'].map((l, k) => <T x="10" y={y + 98 + k * 15} fill="var(--ink-2)">{l}</T>)}
      {s.doubles && <><Line d={`M${X_RED} ${y + 68} V${y + 150}`} on={u.supply} color="var(--red)" motion={motion} /><Line d={`M${X_BLUE} ${y + 66} V${y + 150}`} on={u.signal} color="var(--blue)" motion={motion} /></>}
      {/* tank */}
      <rect x="16" y={y + 32} width="150" height="36" rx="18" fill={u.tank === 'empty' ? 'var(--surface)' : 'var(--red-soft)'} stroke="var(--ink)" stroke-width="1.5" />
      <T x="91" y={y + 48} text-anchor="middle" font-weight="700">Trailer air tank</T>
      <T x="91" y={y + 63} text-anchor="middle">{u.tank === 'charged' ? 'filling (red line)' : u.tank === 'stored' ? 'stored air only' : 'EMPTY'}</T>
      {/* relay */}
      <rect x="190" y={y + 36} width="120" height="30" rx="4" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
      <T x={X_BLUE} y={y + 56} text-anchor="middle">Relay valve</T>
      <Line d={`M166 ${y + 51} H190`} on={u.tank !== 'empty'} color="var(--red)" motion={motion && u.service} />
      <Line d={`M${X_BLUE} ${y + 66} V${y + 86}`} on={u.service} color="var(--ink)" motion={false} />
      {/* brakes */}
      <rect x="120" y={y + 86} width="232" height="44" rx="6" fill={bad ? 'var(--red-soft)' : on ? 'var(--amber-soft)' : 'var(--surface)'} stroke={bad ? 'var(--red)' : 'var(--ink)'} stroke-width={bad ? 2.5 : 1.5} />
      <T x="236" y={y + 104} text-anchor="middle" font-weight="700">{bad ? '⚠ ' : on ? '■ ' : '○ '}{brakeText(u, s.spring)}</T>
      <T x="236" y={y + 122} text-anchor="middle" fill="var(--ink-2)">{s.spring ? 'service + spring brakes' : 'no spring brakes'}</T>
      {s.doubles && (
        <g>
          <Valve x={X_RED} y={y + 156} open={vOpen} /><Valve x={X_BLUE} y={y + 156} open={vOpen} />
          <T x="170" y={y + 161} text-anchor="middle" font-weight="700">valves: {vOpen ? 'OPEN' : 'CLOSED'}</T>
        </g>
      )}
    </g>
  );
}

export function AirDiagram({ s, motion }: { s: AirState; motion: boolean }) {
  const o = simulate(s);
  const u1 = o.units[0];
  const H = T_Y + (s.doubles ? U_H + 178 : 154) + (o.escapeRear ? 26 : 0);
  const gx = (p: number) => 48 + Math.min(p, MAX_PSI);
  const knobOut = !o.tpv;
  const label = `Air system schematic. Tractor pressure ${s.psi} psi. Trailer air supply knob ${knobOut ? 'out' : 'in'}. Tractor protection valve ${o.tpv ? 'open' : 'closed'}.${s.crossed ? ' Glad hands crossed.' : ''}${s.emBreak ? ' Emergency line broken.' : ''}${s.svcBreak ? ' Service line apart.' : ''} ${o.units.map((u, i) => `Unit ${i + 1}: ${brakeText(u, s.spring)}`).join('. ')}.`;
  // trailer-side inlets: supply to the tank at X_RED, service to the relay at X_BLUE (swapped when crossed)
  const redIn = s.crossed ? X_BLUE : X_RED, blueIn = s.crossed ? X_RED : X_BLUE;
  return (
    <svg viewBox={`0 0 360 ${H}`} width="100%" role="img" aria-label={label} style={{ maxWidth: '440px', display: 'block', margin: '0 auto' }}>
      <rect x="4" y="4" width="352" height="156" rx="8" fill="var(--surface-2)" stroke="var(--ink-2)" stroke-width="1.5" />
      <T x="14" y="24" font-size="15" font-weight="700">TRACTOR</T>
      <g aria-hidden="true"><circle cx="330" cy="19" r="6" fill="var(--surface)" stroke="var(--ink-2)" /><circle cx="346" cy="19" r="6" fill="var(--surface)" stroke="var(--ink-2)" /></g>
      <T x="318" y="24" text-anchor="end" fill="var(--ink-2)">dummy couplers</T>
      <rect x="24" y="32" width="150" height="30" rx="15" fill={s.psi > 0 ? 'var(--red-soft)' : 'var(--surface)'} stroke="var(--ink)" stroke-width="1.5" />
      <T x="99" y="52" text-anchor="middle" font-weight="700" class="num">Air tanks {s.psi} psi</T>
      <rect x="48" y="70" width="120" height="10" fill="var(--surface)" stroke="var(--ink-2)" />
      <rect x={gx(BAND[0])} y="70" width={gx(BAND[1]) - gx(BAND[0])} height="10" fill="var(--amber)" />
      <path d={`M${gx(s.psi)} 66 v18`} stroke="var(--ink)" stroke-width="3" />
      <T x="62" y="99" fill="var(--ink-2)">▲ 20–45 psi band</T>
      <Line d="M32 62 V128 H40" on={s.psi > 0} color="var(--red)" motion={motion && o.tpv} />
      <rect x="40" y="108" width="140" height="42" rx="4" fill={o.tpv ? 'var(--ok-soft)' : 'var(--red-soft)'} stroke="var(--ink)" stroke-width="1.5" />
      <T x="110" y="125" text-anchor="middle">Tractor protection</T>
      <T x="110" y="142" text-anchor="middle" font-weight="700">valve: {o.tpv ? 'OPEN' : 'CLOSED'}</T>
      <Octagon x={218} y={52} r={19} />
      <T x="218" y="57" text-anchor="middle" font-weight="700" fill="var(--surface)">{knobOut ? 'OUT' : 'IN'}</T>
      <T x="244" y="47">Trailer air</T>
      <T x="244" y="63">supply knob</T>
      <T x="200" y="96">Foot brake: <tspan font-weight="700">{s.pedal ? 'PRESSED' : 'up'}</tspan></T>
      <T x="200" y="114">Hand valve: <tspan font-weight="700">{s.hand ? 'ON' : 'off'}</tspan></T>
      <T x="200" y="132">Tractor brakes: <tspan font-weight="700">{s.pedal && s.psi > 0 ? 'on' : 'off'}</tspan></T>
      {/* tractor-side lines to glad hands */}
      <Line d={`M${X_RED} 150 V200`} on={o.redT} color="var(--red)" motion={motion} />
      <Line d={`M180 144 H${X_BLUE} V200`} on={o.blueT} color="var(--blue)" motion={motion} />
      <T x="98" y="178" fill="var(--red)" font-weight="700">Emergency</T>
      <T x="98" y="194" fill="var(--red)">(supply)</T>
      <T x="258" y="178" fill="var(--blue)" font-weight="700">Service</T>
      <T x="258" y="194" fill="var(--blue)">(control)</T>
      {/* glad hands */}
      <rect x={X_RED - 14} y="200" width="28" height="12" rx="3" fill="var(--red)" stroke="var(--ink)" /><rect x={X_BLUE - 14} y="200" width="28" height="12" rx="3" fill="var(--blue)" stroke="var(--ink)" />
      <T x="170" y="211" text-anchor="middle">◄ glad hands ►</T>
      <Line d={`M${X_RED} 212 C${X_RED} 244 ${redIn} 238 ${redIn} ${T_Y - 2}`} on={o.redT && !s.emBreak} color="var(--red)" motion={motion} />
      <Line d={`M${X_BLUE} 212 C${X_BLUE} 244 ${blueIn} 238 ${blueIn} ${T_Y - 2}`} on={o.blueT && !s.svcBreak} color="var(--blue)" motion={motion} />
      {s.emBreak && <Tag x={10} y={240} w={146} color="var(--red)">✕ red line broken</Tag>}
      {s.svcBreak && <Tag x={350} y={240} w={146} anchor="end" color="var(--blue)">{o.leakingSvc ? '✕ air rushing out' : '✕ blue line apart'}</Tag>}
      {s.crossed && <Tag x={170} y={266} w={150} anchor="middle" color="var(--red)">✕ LINES CROSSED</Tag>}
      {o.units.map((u, i) => <UnitDraw y={T_Y + i * U_H} u={u} s={s} o={o} idx={i} motion={motion} />)}
      {/* inside trailer: supply to tank, signal to relay (drawn over the panel so the inflow shows) */}
      <Line d={`M${X_RED} ${T_Y - 2} V${T_Y + 32}`} on={s.crossed ? o.blueT && !s.svcBreak : u1.supply} color={s.crossed ? 'var(--blue)' : 'var(--red)'} motion={motion} />
      <Line d={`M${X_BLUE} ${T_Y - 2} V${T_Y + 36}`} on={s.crossed ? u1.supply || o.redT : u1.signal} color={s.crossed ? 'var(--red)' : 'var(--blue)'} motion={motion} />
      {s.doubles && <>
        <Line d={`M${X_RED} ${T_Y + 166} V${T_Y + U_H + 32}`} on={o.units[1].supply} color="var(--red)" motion={motion} />
        <Line d={`M${X_BLUE} ${T_Y + 166} V${T_Y + U_H + 36}`} on={o.units[1].signal} color="var(--blue)" motion={motion} />
      </>}
      {o.escapeRear && <T x="180" y={H - 8} text-anchor="middle" font-weight="700" fill="var(--red)">⚠ air escaping out the back of the rig</T>}
    </svg>
  );
}
