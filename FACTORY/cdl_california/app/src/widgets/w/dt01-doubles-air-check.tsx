import { useState } from 'preact/hooks';
import type { JSX } from 'preact';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'dt01-doubles-air-check', title: 'Doubles air check: air all the way back', lesson: 'DT-01', anchor: /does air reach every trailer/i,
  summary: 'Open and close the shut-off valves on a double (or triple), run the handbook’s rear-of-rig air test, and find the closed valve. Then diagnose 7 rigs.',
  stamp: { id: 'air-all-the-way-back', name: 'Air all the way back', rule: 'Diagnose all 7 doubles/triples air setups with no mistakes.' },
};

/* ---------- Model (DT-01 · p. 7-3, 7-5). Valve key `${trailer}${line}`: shut-off at the REAR of that trailer. */
type Line = 'e' | 's';
interface Rig { n: 2 | 3; open: Record<string, boolean>; drain: boolean[]; parked: boolean; pressure: boolean; knobIn: boolean; handbrake: boolean }
const LINE_NAME: Record<Line, string> = { e: 'emergency (supply)', s: 'service' };
const vk = (t: number, l: Line) => `${t}${l}`;

/** Handbook driving setting (p. 7-5): rear of front trailers OPEN, rear of last trailer CLOSED, dolly drain CLOSED. */
function specRig(n: 2 | 3, extra: Partial<Rig> = {}): Rig {
  const open: Record<string, boolean> = {};
  for (let t = 0; t < n; t++) for (const l of ['e', 's'] as Line[]) open[vk(t, l)] = t < n - 1;
  return { n, open, drain: Array(n - 1).fill(false), parked: false, pressure: false, knobIn: false, handbrake: false, ...extra };
}
/** Air enters the emergency line when the red knob is pushed in; the service line when the trailer handbrake (or pedal) is on. */
function flow(r: Rig, l: Line) {
  const src = r.pressure && (l === 'e' ? r.knobIn : r.handbrake);
  if (!src) return { reach: -1, out: false, stop: null as string | null };
  let reach = 0;
  while (reach < r.n - 1 && r.open[vk(reach, l)]) reach++;
  const out = reach === r.n - 1 && !!r.open[vk(r.n - 1, l)];
  return { reach, out, stop: reach < r.n - 1 ? vk(reach, l) : null };
}
function specFaults(r: Rig): string[] {
  const f: string[] = [];
  for (let t = 0; t < r.n; t++) for (const l of ['e', 's'] as Line[]) {
    const want = t < r.n - 1;
    if (!!r.open[vk(t, l)] !== want) f.push(`Rear of trailer ${t + 1}, ${LINE_NAME[l]}: should be ${want ? 'OPEN' : 'CLOSED'}`);
  }
  r.drain.forEach((d, i) => { if (d) f.push(`Dolly ${i + 1} air tank drain valve: should be CLOSED`); });
  return f;
}
const valveName = (k: string, n: number) => { const t = +k[0]; return `rear of trailer ${t + 1}${t === n - 1 ? ' (last)' : ''}, ${LINE_NAME[k[1] as Line]} line`; };

/* ---------- Schematic */
interface SchProps { r: Rig; hide?: boolean; mark?: string | null; order?: boolean; heavyFirst?: boolean; motion: boolean; onValve?: (k: string) => void }
function Schematic({ r, hide, mark, order, heavyFirst = true, motion, onValve }: SchProps) {
  const W = 360, x0 = 52, dg = 26, tw = (W - 4 - x0 - dg * (r.n - 1)) / r.n;
  const fx = (t: number) => x0 + t * (tw + dg);
  const Y: Record<Line, number> = { e: 104, s: 128 };
  const fe = flow(r, 'e'), fs = flow(r, 's');
  const F: Record<Line, ReturnType<typeof flow>> = { e: fe, s: fs };
  const col = (l: Line) => (l === 'e' ? 'var(--red)' : 'var(--blue)');
  const seg = (l: Line, a: number, b: number, on: boolean, key: string) => (
    <line key={key} x1={a} x2={b} y1={Y[l]} y2={Y[l]} stroke={on ? col(l) : 'var(--ink-2)'} stroke-width={on ? 4 : 1.5} stroke-dasharray={on ? (motion ? '8 5' : undefined) : '3 4'} opacity={on ? 1 : 0.6}>
      {on && motion && <animate attributeName="stroke-dashoffset" from="26" to="0" dur="0.8s" repeatCount="indefinite" />}
    </line>
  );
  const puff = (x: number, y: number, key: string, label: string) => (
    <g key={key} aria-hidden="true">
      {[0, 1, 2].map((i) => <path key={i} d={`M${x + 2 + i * 5} ${y - 7} q4 7 0 14`} fill="none" stroke="var(--ink)" stroke-width="1.6">{motion && <animate attributeName="opacity" values="1;0.2;1" dur="0.9s" begin={`${i * 0.2}s`} repeatCount="indefinite" />}</path>)}
      <text x={x + 2} y={y - 10} font-size="12" font-weight="700" fill="var(--ink)">{label}</text>
    </g>
  );
  const els: JSX.Element[] = [];
  const brakesOk = (t: number) => fe.reach >= t;
  // tractor
  els.push(<g key="tr"><rect x="4" y="44" width="42" height="36" rx="4" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" /><rect x="8" y="48" width="14" height="14" rx="2" fill="var(--surface)" stroke="var(--ink)" /><circle cx="14" cy="84" r="6" fill="var(--ink)" /><circle cx="38" cy="84" r="6" fill="var(--ink)" /><text x="25" y="36" font-size="12" text-anchor="middle" fill="var(--ink-2)">Tractor</text></g>);
  for (let t = 0; t < r.n; t++) {
    const x = fx(t), ok = brakesOk(t);
    const lbl = order && r.n === 2 ? ((t === 0) === heavyFirst ? 'HEAVIER' : 'lighter') : `Trailer ${t + 1}`;
    els.push(<g key={`t${t}`}>
      <rect x={x} y="30" width={tw} height="46" rx="3" fill={order && r.n === 2 && lbl === 'HEAVIER' ? 'var(--surface)' : 'var(--surface-2)'} stroke="var(--ink)" stroke-width={lbl === 'HEAVIER' ? 3 : 1.5} />
      <text x={x + tw / 2} y="50" font-size="13" font-weight="700" text-anchor="middle" fill="var(--ink)">{lbl}</text>
      <text x={x + tw / 2} y="67" font-size="12" text-anchor="middle" fill={ok ? 'var(--ok)' : 'var(--ink-2)'}>{ok ? 'air ✓' : 'no air'}</text>
      <circle cx={x + tw - 44} cy="82" r="6" fill="var(--ink)" /><circle cx={x + tw - 58} cy="82" r="6" fill="var(--ink)" />
      {r.n === 3 && t === 2 && <text x={x + tw / 2} y="24" font-size="12" font-weight="700" text-anchor="middle" fill="var(--red)">not legal in CA</text>}
    </g>);
    if (t > 0) { // converter dolly in front of trailer t
      const dx = x - dg + 2, di = t - 1, air = fe.reach >= t;
      els.push(<g key={`d${t}`}><rect x={dx} y="70" width={dg - 4} height="10" rx="2" fill="var(--amber-soft)" stroke="var(--ink)" /><circle cx={dx + (dg - 4) / 2} cy="84" r="5" fill="var(--ink)" />
        <text x={dx + (dg - 4) / 2} y="97" font-size="12" text-anchor="middle" fill="var(--ink-2)">dolly</text>
        {!hide && r.drain[di] && air && puff(dx + 4, 150, `dp${t}`, 'drain hiss')}</g>);
    }
    for (const l of ['e', 's'] as Line[]) {
      const f = F[l], y = Y[l], vx = x + tw - (t === r.n - 1 ? 30 : 10), key = vk(t, l), open = !!r.open[key];
      els.push(seg(l, t === 0 ? 40 : x - dg + 2, vx - 7, f.reach >= t, `s${key}`));
      if (t < r.n - 1) els.push(seg(l, vx + 7, fx(t + 1) - dg + 2, f.reach >= t + 1, `h${key}`));
      const hl = mark === key;
      els.push(<g key={`v${key}`} onClick={onValve ? () => onValve(key) : undefined} style={onValve ? { cursor: 'pointer' } : undefined}>
        <circle cx={vx} cy={y} r="9" fill={hl ? 'var(--amber)' : 'var(--surface)'} stroke="var(--ink)" stroke-width="1.5" />
        {hide ? <text x={vx} y={y + 4} font-size="12" font-weight="700" text-anchor="middle" fill="var(--ink)">?</text>
          : <line x1={open ? vx - 6 : vx} x2={open ? vx + 6 : vx} y1={open ? y : y - 6} y2={open ? y : y + 6} stroke="var(--ink)" stroke-width="2.5" />}
        {!hide && <text x={vx - 11} y={y + 16} font-size="12" text-anchor="end" fill="var(--ink-2)">{open ? 'open' : 'shut'}</text>}
      </g>);
      if (t === r.n - 1 && f.out) els.push(puff(vx + 8, y, `o${l}`, 'hiss'));
    }
  }
  const desc = `Schematic: tractor pulling ${r.n} trailers. Emergency line ${fe.reach < 0 ? 'has no air' : `has air to trailer ${fe.reach + 1}${fe.out ? ' and escapes at the rear' : ''}`}. Service line ${fs.reach < 0 ? 'has no air' : `has air to trailer ${fs.reach + 1}${fs.out ? ' and escapes at the rear' : ''}`}.`;
  return (
    <svg viewBox={`0 0 ${W} 166`} width="100%" style={{ display: 'block', maxWidth: '560px', marginInline: 'auto' }} role="img" aria-label={desc}>
      <rect x="0" y="0" width={W} height="166" fill="var(--surface)" />
      <text x="4" y={Y.e + 4} font-size="12" font-weight="700" fill="var(--red)">E</text>
      <text x="4" y={Y.s + 4} font-size="12" font-weight="700" fill="var(--blue)">S</text>
      <text x="14" y={Y.e + 4} font-size="12" fill="var(--ink-2)">red</text>
      <text x="14" y={Y.s + 4} font-size="12" fill="var(--ink-2)">blue</text>
      {els}
    </svg>
  );
}

/* ---------- Explore: cab controls + valves + the p. 7-5 test */
const STEPS = ['Hold the rig: tractor parking brake and/or chocks', 'Wait for normal air pressure', 'Push in the red “trailer air supply” knob', 'Apply the trailer handbrake (air to the service line)', 'Open the EMERGENCY shut-off at the rear of the last trailer, listen, close it', 'Open the SERVICE shut-off at the rear of the last trailer, listen, close it'];
const pressed = (on: boolean) => (on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {});

function Explore({ motion }: { motion: boolean }) {
  const [r, setR] = useState<Rig>(specRig(2));
  const [heavyFirst, setHeavyFirst] = useState(true);
  const [heard, setHeard] = useState<Record<Line, boolean | null>>({ e: null, s: null });
  const [msg, setMsg] = useState<{ good: boolean; text: string; mark?: string | null } | null>(null);
  const set = (p: Partial<Rig>) => { setR({ ...r, ...p }); setMsg(null); };
  const last = r.n - 1;
  const toggle = (k: string) => {
    const nr = { ...r, open: { ...r.open, [k]: !r.open[k] } };
    setR(nr);
    const l = k[1] as Line;
    if (+k[0] === last && nr.open[k]) {
      const f = flow(nr, l);
      setHeard({ ...heard, [l]: f.out });
      if (f.out) setMsg({ good: true, text: `Hiss! Air escapes from the ${LINE_NAME[l]} line at the back: ${l === 'e' ? 'the whole system is charged' : 'service pressure goes through all the trailers'}. Now close the valve.` });
      else {
        const why = !r.pressure ? 'Air pressure is not up to normal yet.' : l === 'e' && !r.knobIn ? 'The red trailer air supply knob is out, so no air goes to the emergency line.' : l === 's' && !r.handbrake ? 'The trailer handbrake (or brake pedal) is not on, so there is no service air to test.' : `Check that the shut-off valves on the trailers and dollies are OPEN. Here the ${valveName(f.stop!, r.n)} is closed.`;
        setMsg({ good: false, text: `Silence. No air from the ${LINE_NAME[l]} line means something is wrong and the brakes will not work. ${why}`, mark: f.stop });
      }
    } else setMsg(null);
  };
  const faults = specFaults(r);
  const done = [r.parked, r.pressure, r.knobIn, r.handbrake, heard.e === true && !r.open[vk(last, 'e')], heard.s === true && !r.open[vk(last, 's')]];
  const drive = r.n === 2 && !heavyFirst;
  return (
    <div class="stack">
      <div class="row" role="group" aria-label="Rig">
        <button class="btn sm" aria-pressed={r.n === 2} style={pressed(r.n === 2)} onClick={() => { setR(specRig(2)); setHeard({ e: null, s: null }); setMsg(null); }}>Double</button>
        <button class="btn sm" aria-pressed={r.n === 3} style={pressed(r.n === 3)} onClick={() => { setR(specRig(3)); setHeard({ e: null, s: null }); setMsg(null); }}>Triple <span class="ca-tag">CA</span> not legal</button>
        {r.n === 2 && <button class="btn sm" onClick={() => setHeavyFirst(!heavyFirst)}>Swap trailer order</button>}
      </div>
      {r.n === 3 && <p class="small"><span class="ca-tag">CA</span> <strong>Triples are not legal in California</strong>, but they are still on the test. <span class="plate">p. 7-1</span></p>}
      <Schematic r={r} order heavyFirst={heavyFirst} motion={motion} mark={msg?.mark} onValve={toggle} />
      {r.n === 2 && <p class={`small ${drive ? '' : 'muted'}`} style={drive ? { color: 'var(--red)', fontWeight: 700 } : {}}>{drive ? '⚠ Lighter trailer up front. The heavier trailer goes first, right behind the tractor, for the safest handling.' : '✓ Heavier trailer first, right behind the tractor; lighter in the rear: safest handling.'} <span class="plate">p. 7-2</span></p>}
      <div class="grid2">
        <div class="card flat stack" style={{ gap: '8px' }}>
          <div class="eyebrow">In the cab</div>
          <button class="btn sm" aria-pressed={r.parked} style={pressed(r.parked)} onClick={() => set({ parked: !r.parked })}>Parking brake / chocks: {r.parked ? 'SET' : 'off'}</button>
          <button class="btn sm" aria-pressed={r.pressure} style={pressed(r.pressure)} onClick={() => set({ pressure: !r.pressure })}>Air pressure: {r.pressure ? 'NORMAL' : 'low (tap to build)'}</button>
          <button class="btn sm" aria-pressed={r.knobIn} style={r.knobIn ? pressed(true) : { borderColor: 'var(--red)', color: 'var(--red)' }} onClick={() => set({ knobIn: !r.knobIn })}>● Red trailer air supply knob: {r.knobIn ? 'IN' : 'OUT'}</button>
          <button class="btn sm" aria-pressed={r.handbrake} style={pressed(r.handbrake)} onClick={() => set({ handbrake: !r.handbrake })}>Trailer handbrake: {r.handbrake ? 'ON' : 'off'}</button>
        </div>
        <div class="card flat stack" style={{ gap: '6px' }}>
          <div class="eyebrow">Shut-off valves (rear of each trailer)</div>
          {Array.from({ length: r.n }, (_, t) => (['e', 's'] as Line[]).map((l) => {
            const k = vk(t, l), o = !!r.open[k];
            return <button key={k} class="btn sm" aria-pressed={o} style={{ justifyContent: 'space-between', borderLeft: `5px solid ${l === 'e' ? 'var(--red)' : 'var(--blue)'}` }} onClick={() => toggle(k)}><span>Trailer {t + 1}{t === last ? ' (last)' : ''} · {l === 'e' ? 'Emergency' : 'Service'}</span><strong>{o ? 'OPEN' : 'CLOSED'}</strong></button>;
          }))}
          {r.drain.map((d, i) => <button key={`d${i}`} class="btn sm" aria-pressed={d} style={{ justifyContent: 'space-between' }} onClick={() => { const dr = [...r.drain]; dr[i] = !d; set({ drain: dr }); }}><span>Dolly {i + 1} air tank drain</span><strong>{d ? 'OPEN' : 'CLOSED'}</strong></button>)}
        </div>
      </div>
      {msg && <div class={`feedback ${msg.good ? 'good' : 'bad'}`} role="status"><div class="verdict">{msg.good ? 'Air all the way back' : 'No air at the back'}</div><p class="small">{msg.text} <span class="plate">p. 7-5</span></p></div>}
      <div class="card flat stack" style={{ gap: '6px' }}>
        <div class="spread"><span class="eyebrow">The air-flow test</span><span class="plate">p. 7-5</span></div>
        <ol style={{ margin: 0, paddingLeft: '22px', display: 'flex', flexDirection: 'column', gap: '3px' }}>
          {STEPS.map((s, i) => <li key={i} class="small" style={{ fontWeight: done[i] ? 700 : 400, color: done[i] ? 'var(--ok)' : 'var(--ink)' }}>{done[i] ? '✓ ' : ''}{s}</li>)}
        </ol>
        <p class="small muted">You must have air all the way to the back for all the brakes to work. No air from a line → check that the shut-offs on the trailers and dollies are OPEN.</p>
      </div>
      <div class={`card ${faults.length ? 'warn' : 'tint'}`} role="status">
        <div class="eyebrow">Inspection setting for driving <span class="plate">p. 7-5</span></div>
        {faults.length ? <ul class="small" style={{ margin: '4px 0 0', paddingLeft: '18px' }}>{faults.map((f) => <li key={f}>{f}</li>)}</ul>
          : <p class="small">✓ Front trailers’ rear valves OPEN, last trailer’s rear valves CLOSED, dolly drain CLOSED.</p>}
      </div>
    </div>
  );
}

/* ---------- Challenge: 7 setups */
type Q =
  | { kind: 'tap'; r: Rig; text: string; key: string; why: string }
  | { kind: 'set'; r: Rig; text: string; why: string }
  | { kind: 'mc'; r?: Rig; after?: Rig; text: string; opts: string[]; ans: number; why: string; order?: boolean };
const T = (r: Rig, k: string[], extra: Partial<Rig> = {}): Rig => { const o = { ...r.open }; k.forEach((x) => (o[x] = !o[x])); return { ...r, open: o, ...extra }; };
const READY = { parked: true, pressure: true, knobIn: true, handbrake: true };
const QS: Q[] = [
  { kind: 'tap', r: T(specRig(2, READY), ['0e', '1e']), text: 'Double, pressure normal, red knob in. You open the emergency shut-off at the rear of trailer 2: silence. Which valve is the problem? (Valve positions are hidden: reason it out.)', key: '0e', why: 'Air flows front to back. If none reaches the rear of the last trailer, a shut-off ahead of it in that line is closed: the emergency valve at the rear of trailer 1. It must be OPEN.' },
  { kind: 'tap', r: T(specRig(2, READY), ['0s', '1s']), text: 'Double. The emergency line hisses at the back. You close it, keep the trailer handbrake on, and open the service shut-off at the rear of trailer 2: silence. Which valve?', key: '0s', why: 'The emergency line proves the system is charged; the service line stops at the closed service shut-off at the rear of trailer 1. Front trailers’ valves must be OPEN.' },
  { kind: 'tap', r: T(specRig(3, READY), ['1e', '2e']), text: 'Triple (not legal in CA, but on the test). Emergency line is silent at the rear of trailer 3. Trailer 2’s brakes have air; trailer 3’s do not. Which valve?', key: '1e', why: 'Air got through trailer 1 to trailer 2, so it stops at the rear of trailer 2, the middle trailer. On a triple, all front trailers’ rear valves are OPEN.' },
  { kind: 'set', r: { ...T(specRig(2), ['0e', '1s']), drain: [true] }, text: 'Pre-trip on a double. Set every shut-off valve and the dolly drain the way the inspection says, then check.', why: 'Rear of the front trailer: OPEN. Rear of the last trailer: CLOSED. Converter dolly air tank drain valve: CLOSED.' },
  { kind: 'mc', r: T(specRig(2, { ...READY, handbrake: false }), ['1e']), after: T(specRig(2, READY), ['1s']), text: 'Double. Emergency hisses at the rear. The service shut-off at the rear is silent. Every front valve is OPEN. What is missing?', opts: ['The trailer handbrake (or brake pedal) is not on', 'The compressor has failed', 'The last trailer’s valves must stay open', 'The dolly drain valve is closed'], ans: 0, why: 'The trailer handbrake sends air to the service line; the service test assumes the handbrake or brake pedal is on.' },
  { kind: 'mc', r: T(specRig(2, { ...READY, knobIn: false }), ['1e']), after: T(specRig(2, READY), ['1e']), text: 'Double, pressure normal, trailer handbrake on. You open the emergency shut-off at the rear: silence. All valves are set right. What do you do?', opts: ['Push in the red “trailer air supply” knob', 'Open the valves at the rear of the last trailer and drive', 'Replace the compressor', 'Open the dolly air tank drain'], ans: 0, why: 'Pushing in the red trailer air supply knob supplies air to the emergency (supply) lines. Without it there is nothing to hear.' },
  { kind: 'mc', order: true, text: 'You are coupling a double: one trailer is loaded heavy, the other is light. Which one goes right behind the tractor?', opts: ['The heavier trailer', 'The lighter trailer', 'It does not matter'], ans: 0, why: 'Put the heavier trailer first, right behind the tractor, and the lighter one in the rear: it gives the safest handling.' },
];

function Challenge({ onEvidence, onChallenge, concepts, motion }: WidgetProps & { motion: boolean }) {
  const [i, setI] = useState(0);
  const [misses, setMisses] = useState(0);
  const [pick, setPick] = useState<string | number | null>(null);
  const [work, setWork] = useState<Rig | null>(null);
  const q = QS[i];
  const answer = (p: string | number, ok: boolean) => { if (pick !== null) return; setPick(p); if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const next = () => { if (i + 1 === QS.length && misses === 0) onChallenge?.(); setPick(null); setWork(null); setI(i + 1); };
  if (!q) return (
    <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`}><div class="verdict">{misses === 0 ? `${QS.length} of ${QS.length} — stamp earned: Air all the way back` : `${QS.length - misses} of ${QS.length} right — need all ${QS.length} for the stamp`}</div>
      <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); setWork(null); }}>Try again</button></div>
  );
  let ok = false; let fixed: Rig | null = null; let body: JSX.Element;
  if (q.kind === 'tap') {
    ok = pick === q.key; fixed = T(q.r, [q.key]);
    const keys = Array.from({ length: q.r.n }, (_, t) => [vk(t, 'e'), vk(t, 's')]).flat();
    body = <>
      <Schematic r={pick !== null ? fixed : q.r} hide={pick === null} mark={pick !== null ? q.key : null} motion={motion} onValve={pick === null ? (k) => answer(k, k === q.key) : undefined} />
      <div class="row" role="group" aria-label="Pick the closed valve">{keys.map((k) => <button key={k} class="btn sm" disabled={pick !== null} style={pick === k && k !== q.key ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : pick !== null && k === q.key ? pressed(true) : { borderLeft: `5px solid ${k[1] === 'e' ? 'var(--red)' : 'var(--blue)'}` }} onClick={() => answer(k, k === q.key)}>Trailer {+k[0] + 1} · {k[1] === 'e' ? 'Emerg.' : 'Service'}</button>)}</div>
    </>;
  } else if (q.kind === 'set') {
    const w = work ?? q.r; ok = pick === 'ok'; const faults = specFaults(w);
    body = <>
      <Schematic r={w} motion={motion} onValve={pick === null ? (k) => setWork({ ...w, open: { ...w.open, [k]: !w.open[k] } }) : undefined} />
      <div class="row" role="group" aria-label="Set the valves">
        {Object.keys(w.open).sort().map((k) => <button key={k} class="btn sm" disabled={pick !== null} aria-pressed={w.open[k]} style={{ borderLeft: `5px solid ${k[1] === 'e' ? 'var(--red)' : 'var(--blue)'}` }} onClick={() => setWork({ ...w, open: { ...w.open, [k]: !w.open[k] } })}>Trailer {+k[0] + 1} · {k[1] === 'e' ? 'Emerg.' : 'Service'}: <strong>{w.open[k] ? 'OPEN' : 'CLOSED'}</strong></button>)}
        <button class="btn sm" disabled={pick !== null} aria-pressed={w.drain[0]} onClick={() => setWork({ ...w, drain: [!w.drain[0]] })}>Dolly drain: <strong>{w.drain[0] ? 'OPEN' : 'CLOSED'}</strong></button>
      </div>
      {pick === null && <button class="btn primary sm" onClick={() => answer(faults.length ? 'bad' : 'ok', !faults.length)}>Check my setting</button>}
      {pick === 'bad' && <ul class="small" style={{ margin: 0, paddingLeft: '18px' }}>{faults.map((f) => <li key={f}>{f}</li>)}</ul>}
    </>;
  } else {
    ok = pick === q.ans;
    const shown = pick !== null && q.after ? q.after : q.r;
    body = <>
      {shown && <Schematic r={shown} motion={motion} />}
      {q.order && <Schematic r={specRig(2)} order heavyFirst={pick !== 1} motion={false} />}
      <div class="stack" role="group" aria-label="Choose an answer" style={{ gap: '6px' }}>{q.opts.map((o, j) => <button key={j} class="btn sm" disabled={pick !== null} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick === j && j !== q.ans ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : pick !== null && j === q.ans ? pressed(true) : {}) }} onClick={() => answer(j, j === q.ans)}>{o}</button>)}</div>
    </>;
  }
  const cite = q.kind === 'mc' && q.order ? 'p. 7-2' : 'p. 7-5';
  return (
    <div class="stack">
      <span class="small muted num">Setup {i + 1} of {QS.length}</span>
      <strong>{q.text}</strong>
      {body}
      {pick !== null && <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
        <div class="verdict">{ok ? 'Right — air all the way back' : q.kind === 'tap' ? `It was the ${valveName(q.key, q.r.n)}` : q.kind === 'set' ? 'Not yet: the valves are set wrong' : `Answer: ${q.opts[q.ans]}`}</div>
        <p class="small">{!ok && q.kind === 'tap' ? 'With that valve left closed, no air reaches the trailers behind it, so their brakes will not work. ' : ''}{!ok && q.kind === 'set' ? 'Open valves at the last trailer let air escape; closed valves up front stop air from reaching the rear brakes. ' : ''}{q.why} <span class="plate">{cite}</span></p>
        <button class="btn primary sm" onClick={next}>{i + 1 === QS.length ? 'Finish' : 'Next setup'}</button>
      </div>}
    </div>
  );
}

export default function DoublesAirCheck(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  const motion = !props.reducedMotion;
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Air lab</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Diagnose 7 rigs</button></div>
      {mode === 'explore' ? <Explore motion={motion} /> : <Challenge {...props} motion={motion} />}
    </div>
  );
}
