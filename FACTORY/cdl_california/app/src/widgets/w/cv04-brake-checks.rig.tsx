// Side-view rig for cv04-brake-checks. Facts: CV-04 (DL 650 pp. 6-16, 6-17).
export interface Rig {
  psi: string; knob: 'in' | 'out'; engine: boolean; pedal: 'up' | 'pumping'; hand: boolean; parking: boolean; chocks: boolean;
  emValve: boolean; svcValve: boolean; emAir: boolean; svcAir: boolean; move: 'still' | 'slow' | 'tug'; trailer: 'free' | 'held' | 'grab' | 'moves' | 'none'; flag?: string; bad?: boolean;
}
export const IDLE: Rig = { psi: 'low', knob: 'out', engine: true, pedal: 'up', hand: false, parking: false, chocks: false, emValve: false, svcValve: false, emAir: false, svcAir: false, move: 'still', trailer: 'none' };

const TRAILER: Record<Rig['trailer'], string> = { free: 'Trailer rolls freely', held: 'Trailer brakes HOLD', grab: 'Trailer brakes GRAB', moves: '⚠ Trailer MOVES — brakes not holding', none: '' };

export function RigView({ r, motion }: { r: Rig; motion: boolean }) {
  const label = `Rig: air ${r.psi}, knob ${r.knob}, engine ${r.engine ? 'on' : 'off'}${r.hand ? ', trailer hand valve on' : ''}${r.parking ? ', parking brake set' : ''}${r.chocks ? ', wheels chocked' : ''}. ${TRAILER[r.trailer]}. ${r.flag ?? ''}`;
  const t = (x: number, y: number, s: string, o: Record<string, string | number> = {}) => <text x={x} y={y} font-size="13.5" fill="var(--ink)" {...o}>{s}</text>;
  return (
    <svg viewBox="0 0 360 196" width="100%" role="img" aria-label={label} style={{ maxWidth: '560px', display: 'block', margin: '0 auto' }}>
      <rect x="0" y="0" width="360" height="196" rx="8" fill="var(--surface-2)" />
      <g>
        {motion && r.move !== 'still' && <animateTransform attributeName="transform" type="translate" values={r.move === 'slow' ? '0 0;14 0;0 0' : '0 0;4 0;0 0'} dur={r.move === 'slow' ? '2.4s' : '0.8s'} repeatCount="indefinite" />}
        {/* trailer */}
        <rect x="108" y="44" width="190" height="72" rx="3" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
        <text x="206" y="64" font-size="13.5" font-weight="700" fill="var(--ink)" text-anchor="middle">LAST TRAILER</text>
        {r.trailer !== 'none' && <text x="206" y="84" font-size="13.5" font-weight="700" text-anchor="middle" fill={r.trailer === 'moves' ? 'var(--red)' : 'var(--ok)'}>{TRAILER[r.trailer]}</text>}
        {r.hand && t(206, 104, 'hand valve → service air', { 'text-anchor': 'middle', 'font-size': 12.5, fill: 'var(--blue)' })}
        <circle cx="252" cy="128" r="12" fill="var(--ink-2)" stroke="var(--ink)" /><circle cx="280" cy="128" r="12" fill="var(--ink-2)" stroke="var(--ink)" />
        {/* tractor */}
        <path d="M14 116 V64 Q14 44 34 44 H70 V78 H112 V116 Z" fill="var(--surface)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x="22" y="52" width="40" height="24" rx="3" fill="var(--blue-soft)" stroke="var(--ink-2)" />
        <circle cx="38" cy="128" r="12" fill="var(--ink-2)" stroke="var(--ink)" /><circle cx="92" cy="128" r="12" fill="var(--ink-2)" stroke="var(--ink)" />
        {/* air lines tractor → trailer */}
        <path d="M70 70 C86 60 96 58 108 58" fill="none" stroke={r.emAir ? 'var(--red)' : 'var(--ink-2)'} stroke-width={r.emAir ? 4 : 2} stroke-dasharray={r.emAir ? undefined : '2 4'} />
        <path d="M70 76 C86 70 96 70 108 70" fill="none" stroke={r.svcAir ? 'var(--blue)' : 'var(--ink-2)'} stroke-width={r.svcAir ? 4 : 2} stroke-dasharray={r.svcAir ? undefined : '2 4'} />
      </g>
      {r.move !== 'still' && <text x={r.move === 'slow' ? 40 : 30} y="160" font-size="13.5" font-weight="700" fill="var(--ink)">{r.move === 'slow' ? 'Moving forward slowly →' : 'Tractor tugs gently →'}</text>}
      {r.chocks && <><path d="M72 140 l8 -12 l8 12 Z" fill="var(--amber)" stroke="var(--ink)" /><path d="M258 140 l8 -12 l8 12 Z" fill="var(--amber)" stroke="var(--ink)" />{t(262, 156, 'chocks', { 'font-size': 12.5 })}</>}
      {/* rear shut-off valves */}
      <g>
        <line x1="298" y1="58" x2="318" y2="58" stroke="var(--red)" stroke-width="4" /><line x1="298" y1="70" x2="318" y2="70" stroke="var(--blue)" stroke-width="4" />
        {r.emValve && <text x="326" y="62" font-size="12.5" font-weight="700" fill="var(--red)">{r.emAir ? 'air!' : 'none'}</text>}
        {r.svcValve && <text x="326" y="76" font-size="12.5" font-weight="700" fill="var(--blue)">{r.svcAir ? 'air!' : 'none'}</text>}
        {t(356, 96, 'valves', { 'font-size': 12.5, 'text-anchor': 'end' })}
        {t(356, 110, `red ${r.emValve ? 'open' : 'shut'}`, { 'font-size': 12.5, 'text-anchor': 'end', fill: 'var(--red)' })}
        {t(356, 124, `blue ${r.svcValve ? 'open' : 'shut'}`, { 'font-size': 12.5, 'text-anchor': 'end', fill: 'var(--blue)' })}
      </g>
      {/* dashboard strip */}
      <rect x="6" y="166" width="348" height="26" rx="4" fill="var(--surface)" stroke="var(--line)" />
      <polygon points="22,171 28,171 32,175 32,183 28,187 22,187 18,183 18,175" fill="var(--red)" stroke="var(--ink)" />
      {t(38, 184, `Knob ${r.knob.toUpperCase()}`, { 'font-weight': 700, 'font-size': 12.5 })}
      {t(100, 184, `${r.psi}`, { 'font-size': 12.5 })}
      {t(196, 184, `Engine ${r.engine ? 'ON' : 'OFF'}`, { 'font-size': 12.5 })}
      {t(262, 184, r.pedal === 'pumping' ? 'Pumping pedal' : r.parking ? 'Park brake SET' : 'Park brake off', { 'font-size': 12.5 })}
      {r.flag && <text x="180" y="20" font-size="13.5" font-weight="700" text-anchor="middle" fill={r.bad ? 'var(--red)' : 'var(--ok)'}>{r.flag}</text>}
    </svg>
  );
}
