import { useEffect, useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';
import { AirDiagram, AirState, BAND, MAX_PSI, POP, START, simulate } from './cv02-air-system.sim';

export const meta: WidgetMeta = {
  id: 'cv02-air-system', title: 'Live trailer air system', lesson: 'CV-02', anchor: /^tractor protection valve$/i,
  summary: 'Work the red knob, brake pedal, hand valve and air pressure, then break or cross the lines and watch what the trailer brakes do. Then predict 8 scenarios.',
  stamp: { id: 'air-lines-traced', name: 'Air lines traced', rule: 'Predict all 8 air-system scenarios with no mistakes.' },
};

interface Scenario { q: string; setup: Partial<AirState>; after: Partial<AirState>; choices: string[]; right: number; why: string; p: string }
const SCEN: Scenario[] = [
  { q: 'You are driving. The trailer breaks loose and tears the emergency (red) line. What happens?', setup: {}, after: { emBreak: true, knobIn: false },
    choices: ['Nothing until you press the brake pedal', 'The tractor protection valve closes, the knob pops out, and the trailer emergency brakes come on', 'The tractor protection valve opens to send more air to the trailer'], right: 1,
    why: 'Losing emergency-line pressure closes the tractor protection valve (knob pops out) and sets the trailer emergency brakes. It closes, not opens — it protects the tractor’s air.', p: '6-5' },
  { q: 'The service (blue) line comes apart. You have not touched the brakes yet. What do you notice?', setup: {}, after: { svcBreak: true },
    choices: ['Probably nothing yet — a service line leak may not show until you brake', 'The trailer emergency brakes lock at once', 'The knob pops out right away'], right: 0,
    why: 'The service line only carries air when you brake. A major leak there may go unnoticed until you put the brakes on.', p: '6-7' },
  { q: 'Same broken service line. Now you press the brake pedal. What happens?', setup: { svcBreak: true }, after: { svcBreak: true, pedal: true, psi: POP, knobIn: false },
    choices: ['The trailer brakes apply normally', 'Only the tractor brakes work; air pressure stays steady', 'Air rushes out, tank pressure falls fast, and if it gets low enough the trailer emergency brakes come on'], right: 2,
    why: 'Braking sends air into the broken line. The leak lowers tank pressure quickly; once it drops into the 20–45 psi band the knob pops out and the trailer emergency brakes come on.', p: '6-7' },
  { q: 'A leak slowly drops tractor air pressure into the 20–45 psi range. What does the trailer air supply knob do?', setup: { psi: 60 }, after: { psi: POP, knobIn: false },
    choices: ['It pops out: the tractor protection valve closes and the trailer emergency brakes come on', 'Nothing until pressure reaches 0 psi', 'It pushes itself in to feed more air to the trailer'], right: 0,
    why: 'The tractor protection valve closes by itself somewhere between 20 and 45 psi (the knob pops out). No air leaves the tractor, and the trailer emergency brakes come on.', p: '6-5' },
  { q: 'The glad hands are crossed (red to blue). The trailer has spring brakes. You push the knob in. What happens?', setup: { crossed: true, knobIn: false }, after: { crossed: true },
    choices: ['The trailer brakes release and you can drive normally', 'The trailer spring brakes stay on — supply air went into the service line', 'The knob pops back out at once'], right: 1,
    why: 'Crossed lines send supply air into the service line instead of the trailer tanks, so no air releases the spring brakes. Knob in, spring brakes still on → check the line connections.', p: '6-6' },
  { q: 'Crossed lines again, but this is an old trailer without spring brakes, and its tank air has leaked away. What is the danger?', setup: { crossed: true, spring: false, leaked: true, knobIn: false }, after: { crossed: true, spring: false, leaked: true },
    choices: ['You could drive away with no trailer brakes at all', 'The trailer brakes lock and you cannot move', 'The tractor cannot build air pressure'], right: 0,
    why: 'With no stored air it has no emergency brakes and the wheels turn freely — you can drive away with no trailer brakes. That is why you always test the trailer brakes before driving.', p: '6-6' },
  { q: 'Pulling doubles, the shut-off valves at the back of the last trailer are left open. What happens?', setup: { doubles: true }, after: { doubles: true, vLastOpen: true },
    choices: ['Nothing — every shut-off valve should be open', 'Air escapes out the back of the rig; those valves must be closed', 'The second trailer gets no air at all'], right: 1,
    why: 'All shut-off valves stay open except the ones at the rear of the last trailer, which must be closed — otherwise air escapes.', p: '6-7' },
  { q: 'You park an old trailer without spring brakes by pulling out the knob, and leave it for a long time. What happens?', setup: { spring: false }, after: { spring: false, knobIn: false, leaked: true },
    choices: ['The emergency brakes hold it as long as needed', 'The hand valve takes over automatically', 'The tank air leaks away and it has no brakes — you must chock the wheels'], right: 2,
    why: 'Its emergency brakes work only from air stored in the trailer tank, and it has no parking brake. The air leaks away, then there are no brakes. Always chock the wheels.', p: '6-7' },
];

function Controls({ s, set }: { s: AirState; set: (p: Partial<AirState>) => void }) {
  const on = { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' };
  const tb = (label: string, k: 'pedal' | 'hand', hint: string) => <button class="btn sm" aria-pressed={s[k]} style={s[k] ? on : {}} onClick={() => set({ [k]: !s[k] })} title={hint}>{s[k] ? '✓ ' : ''}{label}</button>;
  return (
    <div class="stack">
      <div class="row" role="group" aria-label="Cab controls">
        <button class="btn sm" style={{ borderColor: 'var(--red)' }} onClick={() => set({ knobIn: !s.knobIn })}>{s.knobIn ? 'Pull knob out' : 'Push knob in'}</button>
        {tb('Press brake pedal', 'pedal', 'Foot brake: works every brake on the rig')}
        {tb('Trailer hand valve', 'hand', 'Trolley valve / Johnson bar: trailer brakes only')}
      </div>
      <div class="field">
        <label for="cv02-psi">Tractor air pressure: <span class="num">{s.psi} psi</span></label>
        <input id="cv02-psi" type="range" min={0} max={MAX_PSI} step={1} value={s.psi} onInput={(e) => set({ psi: +(e.target as HTMLInputElement).value })} />
        <span class="small muted">Slide down into the {BAND[0]}–{BAND[1]} psi band. This truck’s maker set the pop-out at {POP} psi.</span>
      </div>
      <fieldset class="stack" style={{ border: '1px solid var(--line)', borderRadius: '8px', padding: '10px', gap: '6px' }}>
        <legend class="small" style={{ fontWeight: 700 }}>Faults</legend>
        <label class="toggle"><input type="checkbox" checked={s.emBreak} onChange={(e) => set({ emBreak: (e.target as HTMLInputElement).checked })} />Emergency (red) line breaks</label>
        <label class="toggle"><input type="checkbox" checked={s.svcBreak} onChange={(e) => set({ svcBreak: (e.target as HTMLInputElement).checked })} />Service (blue) line comes apart</label>
        <label class="toggle"><input type="checkbox" checked={s.crossed} onChange={(e) => set({ crossed: (e.target as HTMLInputElement).checked })} />Glad hands crossed (red ↔ blue)</label>
      </fieldset>
      <div class="row" role="group" aria-label="Trailer type">
        <button class="btn sm" aria-pressed={s.spring} style={s.spring ? on : {}} onClick={() => set({ spring: true })}>Trailer with spring brakes</button>
        <button class="btn sm" aria-pressed={!s.spring} style={!s.spring ? on : {}} onClick={() => set({ spring: false })}>Old trailer, no spring brakes</button>
      </div>
      {!s.spring && <label class="toggle"><input type="checkbox" checked={s.leaked} onChange={(e) => set({ leaked: (e.target as HTMLInputElement).checked })} />Time passes: trailer tank air has leaked away</label>}
      <div class="row" role="group" aria-label="Rig">
        <button class="btn sm" aria-pressed={!s.doubles} style={!s.doubles ? on : {}} onClick={() => set({ doubles: false })}>Single trailer</button>
        <button class="btn sm" aria-pressed={s.doubles} style={s.doubles ? on : {}} onClick={() => set({ doubles: true })}>Doubles</button>
      </div>
      {s.doubles
        ? <div class="row" role="group" aria-label="Shut-off valves">
            <button class="btn sm" onClick={() => set({ v1Open: !s.v1Open })}>Trailer 1 rear valves: {s.v1Open ? 'OPEN' : 'CLOSED'}</button>
            <button class="btn sm" onClick={() => set({ vLastOpen: !s.vLastOpen })}>Last trailer rear valves: {s.vLastOpen ? 'OPEN' : 'CLOSED'}</button>
          </div>
        : <span class="small muted">Shut-off valves sit at the back of trailers that tow other trailers — pick Doubles to set them. <span class="plate">p. 6-7</span></span>}
    </div>
  );
}

function Status({ s }: { s: AirState }) {
  const o = simulate(s);
  return (
    <div class="card flat stack" role="status" aria-live="polite" style={{ gap: '8px' }}>
      <div class="eyebrow">What happens</div>
      <ul class="stack" style={{ gap: '6px', margin: 0, paddingLeft: '1.1em' }}>
        {o.msgs.map((m) => <li class="small" style={{ color: m.tone === 'bad' ? 'var(--red)' : 'var(--ink)' }}><strong>{m.tone === 'bad' ? '⚠ ' : m.tone === 'ok' ? '✓ ' : '• '}</strong>{m.t} <span class="plate">p. {m.p}</span></li>)}
      </ul>
    </div>
  );
}

function Explore({ reducedMotion }: { reducedMotion: boolean }) {
  const [s, setS] = useState<AirState>({ ...START });
  const set = (p: Partial<AirState>) => setS((cur) => ({ ...cur, ...p }));
  // TPV closes by itself: emergency line lost, or pressure in the band.
  useEffect(() => { if (s.knobIn && (s.emBreak || s.psi <= POP)) set({ knobIn: false }); }, [s.knobIn, s.emBreak, s.psi]);
  // Braking with a broken service line bleeds the tanks fast.
  const leaking = simulate(s).leakingSvc;
  useEffect(() => {
    if (!leaking) return;
    if (reducedMotion) { set({ psi: POP }); return; }
    const id = setInterval(() => setS((c) => ({ ...c, psi: Math.max(POP, c.psi - 3) })), 120);
    return () => clearInterval(id);
  }, [leaking, reducedMotion]);
  return (
    <div class="stack">
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: '16px', alignItems: 'start' }}>
        <div class="stack"><Controls s={s} set={set} /><Status s={s} /></div>
        <AirDiagram s={s} motion={!reducedMotion} />
      </div>
      <div class="row"><button class="btn sm" onClick={() => setS({ ...START })}>Reset rig</button><span class="small muted">Solid coloured line = air flowing; dotted grey = no air.</span></div>
    </div>
  );
}

function Challenge({ onEvidence, onChallenge, concepts, reducedMotion }: WidgetProps) {
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const sc = SCEN[i];
  if (!sc) return (
    <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 8 predicted — stamp earned: Air lines traced' : `${SCEN.length - misses} of ${SCEN.length} right`}</div>
      {misses > 0 && <p class="small">Play with the faults in Explore, then try again for the stamp.</p>}
      <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); }}>Try again</button></div>
  );
  const shown: AirState = { ...START, ...(pick === null ? sc.setup : sc.after) };
  const ok = pick === sc.right;
  return (
    <div class="stack">
      <span class="small muted num">Scenario {i + 1} of {SCEN.length}</span>
      <strong>{sc.q}</strong>
      <div class="stack" role="group" aria-label="Predict the outcome" style={{ gap: '8px' }}>
        {sc.choices.map((c, k) => (
          <button class="btn" style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick !== null && k === sc.right ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)' } : pick === k ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }}
            disabled={pick !== null} onClick={() => { setPick(k); const good = k === sc.right; if (!good) setMisses(misses + 1); onEvidence({ concepts, ok: good }); }}>
            {pick !== null && k === sc.right ? '✓ ' : pick === k ? '✕ ' : ''}{c}</button>
        ))}
      </div>
      {pick !== null && (
        <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
          <div class="verdict">{ok ? 'Right' : 'Not quite — watch the diagram'}</div>
          <p class="small">{sc.why} <span class="plate">p. {sc.p}</span></p>
          <button class="btn primary sm" onClick={() => { if (i + 1 === SCEN.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === SCEN.length ? 'Finish' : 'Next scenario'}</button>
        </div>
      )}
      <div class="eyebrow">{pick === null ? 'The rig right now' : 'What the sim shows'}</div>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: '16px', alignItems: 'start' }}>
        <AirDiagram s={shown} motion={!reducedMotion} />
        {pick !== null && <Status s={shown} />}
      </div>
    </div>
  );
}

export default function AirSystem(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Explore the air system</button>
        <button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Predict 8 scenarios</button>
      </div>
      {mode === 'explore' ? <Explore reducedMotion={props.reducedMotion} /> : <Challenge {...props} />}
    </div>
  );
}
