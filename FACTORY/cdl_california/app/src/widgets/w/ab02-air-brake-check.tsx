import { useEffect, useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';
import { APPLIED, BUILD, CUT_OUT, Dash, FAULTS, Fault, Gauge, KINDS, Kind, Rig, SPRING, START, STATIC, Sim, WARN_MIN, combo, drop, rigNums, tick } from './ab02-air-brake-check.sim';

export const meta: WidgetMeta = {
  id: 'ab02-air-brake-check', title: 'Step 7 air brake check, live', lesson: 'AB-02', anchor: /applied leakage test/i,
  summary: 'Run the compressor, hold and fan the brakes, time the leaks and watch the warning and spring brakes on a live gauge. Then judge 8 rigs: pass or repair before driving.',
  stamp: { id: 'air-check-pro', name: 'Air check pro', rule: 'Judge all 8 rigs’ air brake readings correctly with no mistakes.' },
};

const segOn = { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' };
const Mark = ({ ok }: { ok: boolean }) => <strong style={{ color: ok ? 'var(--ok)' : 'var(--red)', whiteSpace: 'nowrap' }}>{ok ? '✓ pass' : '✗ fail'}</strong>;

const Bands = () => <p class="small muted" style={{ margin: 0 }}>Gauge bands: <strong style={{ color: 'var(--red)' }}>▬ 20–45</strong> spring brakes come on · <strong style={{ color: 'var(--amber-ink)' }}>▬ 55–75</strong> low-air warning range · <strong style={{ color: 'var(--ok)' }}>▬ 100–125</strong> governor cut-in to cut-out.</p>;

function Explore({ rm = false }: { rm?: boolean }) {
  const [rig, setRig] = useState<Rig>({ kind: 'two', fault: 'none' });
  const [s, setS] = useState<Sim>(START);
  const cmb = combo(rig.kind), n = rigNums(rig);
  useEffect(() => {
    if (!s.engine) return;
    // 20× speed: each tick = 4 s; with reduced motion the needle moves in fewer, larger steps
    const id = setInterval(() => setS((o) => tick(o, rig, rm ? 20 : 4)), rm ? 1000 : 200);
    return () => clearInterval(id);
  }, [s.engine, rig, rm]);
  const newRig = (r: Rig) => { setRig(r); setS(START); };
  const say = (note: string, patch: Partial<Sim> = {}) => setS({ ...s, ...patch, note });
  const released = !s.park && (!cmb || !s.supply);
  const releaseMsg = cmb ? 'Push in the yellow parking knob and the red trailer air supply knob first' : 'Push in the yellow parking knob first';
  const watch = () => {
    if (s.engine) return say('Shut off the engine first — both leakage tests are timed with the engine off.');
    if (!released) return say(`${releaseMsg} (release the brakes). ${s.pedal ? 'Applied test, step 3. p. 5-8' : 'Static test: release all brakes. p. 5-10'}`);
    if (s.pedal) {
      if (s.cutAt == null) return say('Applied test step 1: with the engine running, build pressure to governor cut-out (about 125 psi here) and say when it cuts out. p. 5-8');
      const o = drop(s, rig, n.applied);
      return setS({ ...o, applied: n.applied, note: `Pedal held 1 minute after the gauge settled: lost ${n.applied} psi. Limit for a ${KINDS.find((k) => k.k === rig.kind)!.label.toLowerCase()}: ${APPLIED[rig.kind]} psi.` });
    }
    if (s.psi < 100) return say('Static test: start with the system basically fully charged (run the engine up into the compressor’s range). p. 5-10');
    const o = drop(s, rig, n.stat);
    setS({ ...o, stat: n.stat, note: `Brakes released, 1 minute: lost ${n.stat} psi. Static limit: ${STATIC[rig.kind]} psi.` });
  };
  const knobPark = () => {
    if (!s.park) return say('Parking brakes set (knob pulled out).', { park: true, pedal: false });
    if (s.psi <= n.pop) return say('The knob pops right back out: too little air to hold the spring brakes off.');
    say('Parking brakes released (knob pushed in).', { park: false });
  };
  const knobSupply = () => {
    if (!s.supply) return say('Trailer air supply pulled out: tractor protection valve closed.', { supply: true });
    if (s.psi <= n.pop) return say('The red knob pops right back out: pressure is too low.');
    say('Trailer air supply pushed in: air goes to the trailer.', { supply: false });
  };
  const pedal = () => {
    if (s.pedal) return say('Foot brake released.', { pedal: false });
    const o = drop(s, rig, 4);
    setS({ ...o, pedal: o.popAt === s.popAt, note: s.park ? 'Foot brake held — but the parking brake is still on. For the applied test release it first (step 3). p. 5-8' : 'Foot brake held down fully. Now watch the gauge settle and time 1 minute.' });
  };
  const fan = () => { const o = drop({ ...s, pedal: false }, rig, 6); setS({ ...o, note: `Fanned: pressed and released the pedal quickly. Each release lets air out.${s.park && (!cmb || s.supply) ? ' For the spring brake test, push the knob(s) in first so they can pop out.' : ''}` }); };
  const parkTest = () => {
    if (!s.engine) return say('Parking brake test: start the engine so you can pull against the brake in a low gear.');
    if (!s.park) return say('Set the parking brake first (pull the yellow knob), then pull gently against it in a low gear. p. 5-10');
    say('Seat belt on, parking brake set, pulled gently in a low gear.', { parkTest: 'held ✓' });
  };
  const svcTest = () => {
    if (!s.engine || s.psi < 100) return say('Service brake test: wait for normal air pressure with the engine running. p. 5-10');
    if (s.park) return say('Release the parking brake before you move forward.');
    say('Moved ahead at about 5 mph and braked firmly with the pedal: no pull to one side, normal feel, no delay.', { svcTest: 'stopped straight ✓' });
  };
  const rows: [string, string, string | null, boolean][] = [
    ['Buildup 85 → 100 psi', `≤ ${BUILD.max} s (dual)`, s.buildS != null ? `${s.buildS} s` : null, (s.buildS ?? 0) <= BUILD.max],
    ['Governor cut-out', '120–140 psi', s.cutAt != null ? `${s.cutAt} psi` : null, true],
    ['Applied leakage, 1 min', `≤ ${APPLIED[rig.kind]} psi`, s.applied != null ? `${s.applied} psi` : null, (s.applied ?? 0) <= APPLIED[rig.kind]],
    ['Static leakage, 1 min', `≤ ${STATIC[rig.kind]} psi`, s.stat != null ? `${s.stat} psi` : null, (s.stat ?? 0) <= STATIC[rig.kind]],
    ['Low-air warning on at', `≥ ${WARN_MIN} psi`, s.warnAt != null ? `${s.warnAt} psi` : null, (s.warnAt ?? 99) >= WARN_MIN],
    ['Spring brakes pop out at', `${SPRING[0]}–${SPRING[1]} psi`, s.popAt != null ? `${s.popAt} psi` : null, (s.popAt ?? 30) >= SPRING[0] && (s.popAt ?? 30) <= SPRING[1]],
  ];
  const b = (label: string, on: boolean, fn: () => void, extra = {}) => <button class="btn sm" aria-pressed={on} style={{ ...(on ? segOn : {}), ...extra }} onClick={fn}>{label}</button>;
  return (
    <div class="stack">
      <div class="stack" style={{ gap: '6px' }}>
        <span class="small" style={{ fontWeight: 700 }} id="ab02-k">Vehicle</span>
        <div role="group" aria-labelledby="ab02-k" style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(80px, 1fr))', gap: '6px' }}>{KINDS.map((k) => (
          <button class="btn sm" aria-pressed={rig.kind === k.k} aria-label={k.label} style={{ paddingInline: '4px', ...(rig.kind === k.k ? segOn : {}) }} onClick={() => newRig({ ...rig, kind: k.k })}>{k.short}</button>
        ))}</div>
        <div class="field"><label for="ab02-f">Hidden fault to find</label>
          <select id="ab02-f" value={rig.fault} onChange={(e) => newRig({ ...rig, fault: (e.target as HTMLSelectElement).value as Fault })}>{FAULTS.map((f) => <option value={f.f}>{f.label}</option>)}</select></div>
      </div>
      <Dash s={s} rig={rig} label={`Tank pressure ${Math.round(s.psi)} psi. Low air warning ${s.psi < n.warn ? 'on' : 'off'}. Engine ${s.engine ? 'running' : 'off'}. Parking knob ${s.park ? 'out, applied' : 'in, released'}${cmb ? `. Trailer air supply knob ${s.supply ? 'out' : 'in'}` : ''}. Foot brake ${s.pedal ? 'held' : 'up'}.`} />
      <Bands />
      <div role="group" aria-label="Cab controls" style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(150px, 1fr))', gap: '6px' }}>
        {b(s.engine ? 'Stop engine' : 'Start engine (idle)', s.engine, () => say(s.engine ? 'Engine off: the compressor stops.' : 'Engine at normal idle: the compressor pumps until governor cut-out.', { engine: !s.engine }))}
        {b(s.pedal ? 'Let foot brake up' : 'Hold foot brake down', s.pedal, pedal)}
        {b(s.park ? 'Push in parking knob' : 'Pull out parking knob', false, knobPark, { borderColor: 'var(--amber)', borderWidth: '2px' })}
        {cmb && b(s.supply ? 'Push in trailer air knob' : 'Pull out trailer air knob', false, knobSupply, { borderColor: 'var(--red)', borderWidth: '2px' })}
        {b('Fan the brakes ×1', false, fan)}
        <button class="btn primary sm" onClick={watch}>Time 1 minute ⏩</button>
        {b('Parking brake test', false, parkTest)}{b('Service brake test', false, svcTest)}
        <button class="btn sm" onClick={() => setS(START)}>Reset rig</button>
      </div>
      {s.note && <p class="card tint small" role="status" aria-live="polite" style={{ margin: 0 }}>{s.note}</p>}
      <table class="small num" style={{ width: '100%', borderCollapse: 'collapse' }} aria-label="Step 7 readings">
        <thead><tr style={{ borderBottom: '1.5px solid var(--ink-2)', textAlign: 'left' }}><th style={{ padding: '4px 4px 4px 0' }}>Check</th><th style={{ padding: '4px' }}>Limit</th><th style={{ padding: '4px 0', textAlign: 'right' }}>Your rig</th></tr></thead>
        <tbody>{rows.map(([name, lim, got, ok]) => (
          <tr style={{ borderBottom: '1px solid var(--line)' }}><td style={{ padding: '4px 4px 4px 0' }}>{name}</td><td style={{ padding: '4px', whiteSpace: 'nowrap' }}>{lim}</td>
            <td style={{ padding: '4px 0', textAlign: 'right' }}>{got == null ? <span class="muted">—</span> : <>{got} <Mark ok={ok} /></>}</td></tr>
        ))}
          <tr style={{ borderBottom: '1px solid var(--line)' }}><td style={{ padding: '4px 4px 4px 0' }}>Parking brake test</td><td style={{ padding: '4px' }}>holds</td><td style={{ padding: '4px 0', textAlign: 'right' }}>{s.parkTest ?? <span class="muted">—</span>}</td></tr>
          <tr><td style={{ padding: '4px 4px 4px 0' }}>Service brake test</td><td style={{ padding: '4px' }}>~5 mph, firm</td><td style={{ padding: '4px 0', textAlign: 'right' }}>{s.svcTest ?? <span class="muted">—</span>}</td></tr>
        </tbody>
      </table>
      <p class="small muted">Try it: start the engine and watch 85 → 100 psi get timed, then the governor cut out. Stop the engine, release the brakes, hold the pedal and time a minute. Then fan the brakes down and watch the warning (must be on before 55 psi) and the knobs pop (20–45 psi). The sim runs 20× fast. <span class="plate">pp. 5-8 – 5-10</span></p>
    </div>
  );
}

interface Case { rig: string; kind?: Kind; test: string; read: string; psi: number; ghost?: number; ok: boolean; why: string; bad: string; page: string }
const CASES: Case[] = [
  { rig: 'Straight truck (single vehicle)', kind: 'single', test: 'Static leakage', read: 'Fully charged, engine off, brakes released. 1 minute: 122 → 120 psi.', psi: 120, ghost: 122, ok: true,
    why: 'Lost 2 psi. The static limit for a single vehicle is 2 psi — right at the limit is still OK.', bad: '', page: '5-10' },
  { rig: 'Tractor + semitrailer (2 vehicles)', kind: 'two', test: 'Applied leakage', read: 'Cut-out at 125 psi. Engine off, brakes released, pedal held. 1 minute after the gauge settled: 118 → 114 psi.', psi: 114, ghost: 118, ok: true,
    why: 'Lost 4 psi. Applied limit for a combination of 2 vehicles is 4 psi.', bad: '', page: '5-8' },
  { rig: 'Tractor pulling triples (3+ vehicles)', kind: 'three', test: 'Static leakage', read: 'Fully charged, engine off, brakes released. 1 minute: 124 → 118 psi.', psi: 118, ghost: 124, ok: false,
    why: 'Lost 6 psi. Static limit for 3 or more vehicles is 5 psi (6 psi is the APPLIED limit).', bad: 'More loss than the limit means the brake system has a problem: find the leak and repair it before driving.', page: '5-10' },
  { rig: 'Pickup-tractor towing a trailer with electric brakes', kind: 'noair', test: 'Applied leakage', read: 'Pedal held 1 minute after the gauge settled: 119 → 115 psi.', psi: 115, ghost: 119, ok: false,
    why: 'Lost 4 psi. When the towed vehicle has no air brakes, the applied limit is 3 psi — the same as a single vehicle, not 4.', bad: 'More than the limit means the brake system has a problem. Repair it before driving.', page: '5-8' },
  { rig: 'Dump truck (single vehicle)', kind: 'single', test: 'Low air warning', read: 'Key on, fanned off the air. The light and buzzer came on at 50 psi.', psi: 50, ok: false,
    why: 'The warning must come on BEFORE pressure drops below 55 psi. 50 psi is too late.', bad: 'With a late warning you could lose air and not know it — in a single-circuit system that can mean sudden emergency braking.', page: '5-8' },
  { rig: 'Large tour bus', test: 'Low air warning', read: 'Fanned off the air. The warning came on at 82 psi.', psi: 82, ok: true,
    why: 'Large buses often warn at 80–85 psi. Say the normal 55–75 psi range and tell the examiner your bus is built to warn higher.', bad: '', page: '5-8' },
  { rig: 'Tractor + semitrailer (2 vehicles)', kind: 'two', test: 'Spring brake test', read: 'Parking brake and trailer air supply pushed in, fanned off the air. Both knobs popped out at 32 psi.', psi: 32, ok: true,
    why: 'On a tractor-trailer the tractor protection valve and parking brake valve should close (pop out) at normally 20–45 psi. 32 psi is inside.', bad: '', page: '5-9' },
  { rig: 'Dual-system box truck', kind: 'single', test: 'Rate of pressure buildup', read: 'Normal idle (700 rpm). The gauge took 70 seconds to go from 85 to 100 psi.', psi: 100, ghost: 85, ok: false,
    why: 'A dual system should build from about 85 to 100 psi within 45 seconds. 70 s is too slow. (50 → 90 psi in 3 minutes is only for pre-1975 single systems.)', bad: 'It did not meet the handbook limit, so the vehicle fails this check. Have it fixed before you drive.', page: '5-9' },
];
const Q_PAGE = (c: Case) => <span class="plate">p. {c.page}</span>;

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<boolean | null>(null);
  const [misses, setMisses] = useState(0);
  const c = CASES[i];
  if (!c) return (
    <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? `${CASES.length} of ${CASES.length} right — stamp earned: Air check pro` : `${CASES.length - misses} of ${CASES.length} right — need all ${CASES.length} for the stamp`}</div>
      <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); }}>Judge again</button></div>
  );
  const answer = (p: boolean) => { if (pick !== null) return; setPick(p); const ok = p === c.ok; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const right = pick === c.ok;
  const limit = c.test === 'Applied leakage' ? `Applied limit: ${APPLIED[c.kind!]} psi` : c.test === 'Static leakage' ? `Static limit: ${STATIC[c.kind!]} psi` : c.test === 'Low air warning' ? 'Must be on before 55 psi' : c.test === 'Spring brake test' ? 'Pop out: 20–45 psi' : '85 → 100 psi in 45 s';
  return (
    <div class="stack">
      <span class="small muted num">Rig {i + 1} of {CASES.length}</span>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(150px, 1fr))', gap: '12px', alignItems: 'center' }}>
        <svg viewBox="0 0 184 184" width="100%" role="img" aria-label={`Gauge reads ${c.psi} psi${c.ghost != null ? `, started at ${c.ghost} psi` : ''}.`} style={{ maxWidth: '200px', marginInline: 'auto' }}>
          <Gauge psi={c.psi} ghost={c.ghost} cx={92} cy={92} r={84} />
        </svg>
        <div class="stack" style={{ gap: '4px' }}>
          <span class="eyebrow">{c.test}</span>
          <strong>{c.rig}</strong>
          <p class="small" style={{ margin: 0 }}>{c.read}</p>
          {c.ghost != null && <p class="small muted" style={{ margin: 0 }}>Dashed needle = start · solid = end</p>}
          <Bands />
        </div>
      </div>
      <div role="group" aria-label="Your call" style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '6px' }}>
        {[true, false].map((p) => (
          <button class="btn sm" disabled={pick !== null} aria-pressed={pick === p} style={pick !== null && p === c.ok ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)', opacity: 1 } : pick === p ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}} onClick={() => answer(p)}>
            {p ? 'Pass — OK to drive' : 'Fail — repair before driving'}{pick !== null && p === c.ok ? ' ✓' : pick === p ? ' ✗' : ''}</button>
        ))}
      </div>
      {pick !== null && <div class={`feedback ${right ? 'good' : 'bad'}`} role="status">
        <div class="verdict">{right ? 'Right' : 'No'} — it {c.ok ? 'passes' : 'fails'}</div>
        <p class="small"><strong>{limit}.</strong> {c.why} {Q_PAGE(c)}</p>
        {!c.ok && <p class="small"><strong>{right ? 'Why it matters' : 'Consequence of driving it'}:</strong> {c.bad}</p>}
        {!right && c.ok && <p class="small"><strong>Consequence of your call:</strong> you would take a good rig out of service. Know the exact limit for this vehicle.</p>}
        <button class="btn primary sm" onClick={() => { if (i + 1 === CASES.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === CASES.length ? 'Finish' : 'Next rig'}</button>
      </div>}
      <p class="small muted">Governor cut-out for the applied test: 120–140 psi (or the maker’s level); about {CUT_OUT} psi here.</p>
    </div>
  );
}

export default function AirBrakeCheck(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore the dash</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Challenge: 8 rigs</button></div>
      {mode === 'explore' ? <Explore rm={props.reducedMotion} /> : <Challenge {...props} />}
    </div>
  );
}
