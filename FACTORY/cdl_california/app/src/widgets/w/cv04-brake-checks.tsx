import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';
import { IDLE, Rig, RigView } from './cv04-brake-checks.rig';

export const meta: WidgetMeta = {
  id: 'cv04-brake-checks', title: 'Combination brake check sequencer', lesson: 'CV-04', anchor: /combination vehicle brake check/i,
  summary: 'Step through the four combination brake tests and see what should happen and what a failure means. Then put the steps in order and match each test to its pass result.',
  stamp: { id: 'brake-checks-passed', name: 'Brake checks passed', rule: 'Order all four brake tests and match every pass result with no mistakes.' },
};

interface Step { text: string; rig: Partial<Rig>; skip?: string }
interface Test { name: string; short: string; steps: Step[]; pass: string; passRig: Partial<Rig>; fail: string; failRig: Partial<Rig>; why: string }
const CHARGED: Partial<Rig> = { psi: 'normal', knob: 'in', emAir: true };
const TESTS: Test[] = [
  { name: 'Test 1 · Air flows to all trailers', short: 'Air flow to all trailers',
    steps: [
      { text: 'Hold the rig with the tractor parking brake and/or wheel chocks.', rig: { parking: true, chocks: true }, skip: 'The rig is not held still while you walk to the back.' },
      { text: 'Wait for air pressure to reach normal.', rig: { parking: true, chocks: true, psi: 'normal' }, skip: 'Pressure is not normal yet, so the system is not fully charged.' },
      { text: 'Push in the red trailer air supply knob — air goes to the emergency (supply) lines.', rig: { parking: true, chocks: true, ...CHARGED }, skip: 'With the knob still out, no air goes to the emergency (supply) lines — nothing would rush out at the back.' },
      { text: 'Apply the trailer hand brake — air goes into the service line.', rig: { parking: true, chocks: true, ...CHARGED, hand: true, svcAir: true }, skip: 'Without the hand brake (or pedal) applied, the service line has no air, so its valve gives nothing.' },
      { text: 'At the back of the last trailer, open the emergency-line shut-off valve, listen for air rushing out, then close it.', rig: { parking: true, chocks: true, ...CHARGED, hand: true, svcAir: true, emValve: true, flag: 'Emergency line: air rushing out' }, skip: 'The handbook checks the emergency line first, then the service line.' },
      { text: 'Open the service-line valve, listen for air, then close it.', rig: { parking: true, chocks: true, ...CHARGED, hand: true, svcAir: true, svcValve: true, flag: 'Service line: air coming out' } },
    ],
    pass: 'Air comes out of BOTH lines at the back of the last trailer: the whole system is charged and service pressure reaches every trailer.', passRig: { parking: true, chocks: true, ...CHARGED, hand: true, svcAir: true, flag: 'Both lines had air ✓' },
    fail: 'No air from one or both lines → check that the shut-off valves on the trailers and dollies are OPEN. You must have air all the way to the back for all the brakes to work.', failRig: { parking: true, chocks: true, psi: 'normal', knob: 'in', hand: true, emValve: true, svcValve: true, flag: '⚠ No air at the back', bad: true },
    why: 'Only the valves at the rear of the last trailer stay closed; you open them just for this test.' },
  { name: 'Test 2 · Tractor protection valve', short: 'Tractor protection valve',
    steps: [
      { text: 'Charge the trailer air system: build normal pressure and push the air supply knob in.', rig: { ...CHARGED }, skip: 'The system is not charged — there is no pushed-in knob to pop out.' },
      { text: 'Shut the engine off.', rig: { ...CHARGED, engine: false }, skip: 'The engine must be off so the compressor stops and the pressure can only go down.' },
      { text: 'Step on and off the brake pedal several times to lower the tank pressure.', rig: { ...CHARGED, engine: false, pedal: 'pumping', psi: 'falling' }, skip: 'If you never pump the pedal, the pressure never drops, so the valve is never tested.' },
      { text: 'Watch the trailer air supply knob as the pressure falls.', rig: { engine: false, pedal: 'pumping', psi: '20–45 psi', knob: 'out', flag: 'Knob popped out' } },
    ],
    pass: 'The knob pops out (or a lever goes from “normal” to “emergency”) when pressure falls into the maker’s range — usually 20 to 45 psi.', passRig: { engine: false, psi: '20–45 psi', knob: 'out', flag: 'Popped out in 20–45 psi ✓' },
    fail: 'Knob does not pop out → the tractor protection valve is not working right. An air hose or trailer brake leak could drain ALL the air from the tractor, the emergency brakes would come on, and you could lose control.', failRig: { engine: false, psi: 'below 20 psi', knob: 'in', emAir: true, flag: '⚠ Knob stayed in', bad: true },
    why: 'The valve exists to keep air in the tractor if the trailer breaks away or leaks badly.' },
  { name: 'Test 3 · Trailer emergency brakes', short: 'Trailer emergency brakes',
    steps: [
      { text: 'Charge the trailer air system and check that the trailer rolls freely.', rig: { ...CHARGED, trailer: 'free', move: 'slow' }, skip: 'First charge the system and check the trailer rolls freely (brakes released) — only then does a “hold” mean something.' },
      { text: 'Stop.', rig: { ...CHARGED }, skip: 'The handbook says to stop before you pull the knob.' },
      { text: 'Pull out the trailer air supply control (or put it in “emergency”).', rig: { psi: 'normal', knob: 'out' }, skip: 'With the knob still in, the trailer emergency brakes are off — the trailer would just roll.' },
      { text: 'Pull gently against the trailer with the tractor.', rig: { psi: 'normal', knob: 'out', move: 'tug', trailer: 'held' } },
    ],
    pass: 'The trailer emergency brakes are on and hold the trailer still.', passRig: { psi: 'normal', knob: 'out', move: 'tug', trailer: 'held', flag: 'Brakes held ✓' },
    fail: 'The trailer moves → its emergency brakes are not holding, so they would not stop it if it lost its supply air (for example, if it broke away).', failRig: { psi: 'normal', knob: 'out', move: 'tug', trailer: 'moves', flag: '⚠ Trailer moved', bad: true },
    why: 'Pulling the knob out shuts off trailer air and sets the trailer emergency brakes (p. 6-5).' },
  { name: 'Test 4 · Trailer service brakes', short: 'Trailer service brakes',
    steps: [
      { text: 'Check for normal air pressure.', rig: { ...CHARGED, parking: true }, skip: 'The handbook starts this test by checking for normal air pressure.' },
      { text: 'Release the parking brakes.', rig: { ...CHARGED }, skip: 'With the parking brakes on, the rig cannot roll, so you cannot feel the trailer brakes.' },
      { text: 'Move the vehicle forward slowly.', rig: { ...CHARGED, move: 'slow', trailer: 'free' }, skip: 'Standing still, you cannot feel the trailer brakes grab.' },
      { text: 'Apply the trailer brakes with the hand control (trolley valve).', rig: { ...CHARGED, move: 'slow', hand: true, svcAir: true, trailer: 'grab' } },
    ],
    pass: 'You feel the trailer brakes come on — they are connected and working. (Test with the hand valve; in normal driving brake with the foot pedal, which works the service brakes at all wheels.)', passRig: { ...CHARGED, hand: true, svcAir: true, trailer: 'grab', flag: 'Felt the brakes come on ✓' },
    fail: 'You feel nothing → the test has not shown the trailer brakes are connected and working.', failRig: { ...CHARGED, move: 'slow', hand: true, svcAir: true, trailer: 'free', flag: '⚠ Felt nothing', bad: true },
    why: 'Never use the hand valve while driving — braking the trailer alone can make it skid (p. 6-5).' },
];
const rig = (p: Partial<Rig>): Rig => ({ ...IDLE, ...p });

const OPTIONS = [
  { t: 'Air comes out of both the emergency and service valves at the rear of the last trailer', test: 0 },
  { t: 'The knob pops out, usually between 20 and 45 psi', test: 1 },
  { t: 'The trailer does not move when you tug gently with the knob out', test: 2 },
  { t: 'You feel the trailer brakes come on while rolling slowly', test: 3 },
  { t: 'The knob stays in until pressure reaches 0 psi', test: -1, why: 'A knob that stays in is a FAILED tractor protection valve test: a leak could drain all the tractor’s air.' },
  { t: 'The trailer rolls freely when you tug with the knob out', test: -1, why: 'A trailer that rolls with the knob out means the emergency brakes are NOT holding — a failed test.' },
];

const WALK = [
  { h: 'Lower fifth wheel (tractor)', items: ['Securely mounted to the frame', 'No missing or damaged parts', 'Enough grease', 'No visible space between upper and lower fifth wheel', 'Locking jaws around the kingpin shank — not the head', 'Release arm seated, safety latch/lock engaged'], why: 'These parts are all that hold the trailer on; a gap or jaws in the wrong place can let it come loose (p. 6-10).' },
  { h: 'Upper fifth wheel (trailer)', items: ['Glide plate securely mounted to the trailer frame', 'Kingpin not damaged'] },
  { h: 'Air and electric lines', items: ['Electrical cord firmly plugged in and secured', 'Air lines connected to the glad hands, no leaks, secured with enough slack for turns', 'All lines free from damage'], why: 'When the rig turns, the distance between connections changes; a line with no slack can be pulled apart.' },
  { h: 'Sliding fifth wheel', items: ['Slide not damaged, no parts missing', 'Properly greased', 'All locking pins present and locked', 'If air powered: no air leaks', 'Not so far forward that the tractor frame hits the landing gear or the cab hits the trailer in turns'] },
  { h: 'Landing gear', items: ['Fully raised, no missing parts, not bent or damaged', 'Crank handle in place and secured', 'If power operated: no air or hydraulic leaks'], why: 'Gear hanging part way down can catch on railroad tracks or other things (p. 6-10).' },
];

function Explore({ reducedMotion }: { reducedMotion: boolean }) {
  const [ti, setTi] = useState(0);
  const [k, setK] = useState(-1);
  const [fail, setFail] = useState(false);
  const pick = (i: number) => { setTi(i); setK(-1); setFail(false); };
  const T = TESTS[ti];
  const done = ti < 4 && k === T.steps.length - 1;
  return (
    <div class="stack">
      <div class="row" role="group" aria-label="Choose a check">
        {TESTS.map((t, i) => <button class="btn sm" aria-pressed={ti === i} style={ti === i ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}} onClick={() => pick(i)}>{i + 1}. {t.short}</button>)}
        <button class="btn sm" aria-pressed={ti === 4} style={ti === 4 ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}} onClick={() => pick(4)}>Walk-around extras</button>
      </div>
      {ti === 4 ? (
        <div class="stack">
          <p class="small">Use the same 7-step method as any vehicle, plus these new checks. <span class="plate">p. 6-16</span></p>
          {WALK.map((g) => (
            <div class="card flat stack" style={{ gap: '6px' }}>
              <strong>{g.h}</strong>
              <ul class="small" style={{ margin: 0, paddingLeft: '1.1em' }}>{g.items.map((it) => <li>{it}</li>)}</ul>
              {g.why && <p class="small muted" style={{ margin: 0 }}><strong>Why:</strong> {g.why}</p>}
            </div>
          ))}
        </div>
      ) : (
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: '16px', alignItems: 'start' }}>
          <div class="stack">
            <strong>{T.name} <span class="plate">p. 6-17</span></strong>
            <div class="eyebrow">What you do</div>
            <ol class="small stack" style={{ gap: '6px', margin: 0, paddingLeft: '1.3em' }}>
              {T.steps.map((s, i) => <li style={{ fontWeight: i === k ? 700 : 400, color: i > k ? 'var(--ink-2)' : 'var(--ink)' }} aria-current={i === k ? 'step' : undefined}>{i <= k ? '✓ ' : ''}{s.text}</li>)}
            </ol>
            <div class="row">
              <button class="btn sm" disabled={k < 0} onClick={() => { setK(k - 1); setFail(false); }}>Back</button>
              <button class="btn primary sm" disabled={done} onClick={() => setK(k + 1)}>{k < 0 ? 'Start the test' : 'Do next step'}</button>
            </div>
            {done && (
              <div class="stack" role="status" aria-live="polite">
                <div class="feedback good"><div class="verdict">What should happen</div><p class="small">{T.pass}</p></div>
                <button class="btn sm" aria-pressed={fail} onClick={() => setFail(!fail)}>{fail ? 'Show the pass result' : 'What if it fails?'}</button>
                {fail && <div class="feedback bad"><div class="verdict">Failure means</div><p class="small">{T.fail} <span class="plate">p. 6-17</span></p></div>}
                <p class="small muted">{T.why}</p>
              </div>
            )}
          </div>
          <RigView r={rig(done ? (fail ? T.failRig : T.passRig) : k < 0 ? {} : T.steps[k].rig)} motion={!reducedMotion} />
        </div>
      )}
    </div>
  );
}

const shuffle = (n: number, seed: number) => { const a = Array.from({ length: n }, (_, i) => i); for (let i = n - 1; i > 0; i--) { const j = (i * 7 + seed * 3 + 1) % (i + 1); [a[i], a[j]] = [a[j], a[i]]; } if (a.every((v, i) => v === i)) a.reverse(); return a; };

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [stage, setStage] = useState(0); // 0-3 ordering, 4-7 matching, 8 done
  const [seq, setSeq] = useState<number[]>([]);
  const [checked, setChecked] = useState<boolean | null>(null);
  const [mPick, setMPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const reset = () => { setStage(0); setSeq([]); setChecked(null); setMPick(null); setMisses(0); };
  const next = () => { if (stage === 7 && misses === 0) onChallenge?.(); setStage(stage + 1); setSeq([]); setChecked(null); setMPick(null); };
  if (stage >= 8) return (
    <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'All 8 right — stamp earned: Brake checks passed' : `${8 - misses} of 8 right`}</div>
      {misses > 0 && <p class="small">Step through the tests in Explore, then try again for the stamp.</p>}
      <button class="btn sm" onClick={reset}>Try again</button></div>
  );
  if (stage < 4) {
    const T = TESTS[stage], order = shuffle(T.steps.length, stage + 2);
    const firstBad = seq.findIndex((v, i) => v !== i);
    const ok = checked === true;
    return (
      <div class="stack">
        <span class="small muted num">Part 1 · Put the steps in order ({stage + 1} of 4)</span>
        <strong>{T.name}</strong>
        <p class="small muted" style={{ margin: 0 }}>Tap the steps in the order you do them.</p>
        <div class="stack" style={{ gap: '6px' }} role="group" aria-label="Steps to place">
          {order.map((si) => { const pos = seq.indexOf(si); return (
            <button class="btn" style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pos >= 0 ? { opacity: 0.55 } : {}) }} disabled={pos >= 0 || checked !== null} onClick={() => setSeq([...seq, si])}>
              <span class="num" style={{ minWidth: '1.6em', fontWeight: 700 }}>{pos >= 0 ? `${pos + 1}.` : '＋'}</span>{T.steps[si].text}</button>); })}
        </div>
        <div class="row">
          <button class="btn sm" disabled={!seq.length || checked !== null} onClick={() => setSeq(seq.slice(0, -1))}>Undo last</button>
          <button class="btn primary sm" disabled={seq.length !== T.steps.length || checked !== null} onClick={() => { const good = seq.every((v, i) => v === i); setChecked(good); if (!good) setMisses(misses + 1); onEvidence({ concepts, ok: good }); }}>Check order</button>
        </div>
        {checked !== null && (
          <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
            <div class="verdict">{ok ? 'Right order' : `Step ${firstBad + 1} is out of place`}</div>
            {!ok && <p class="small"><strong>Consequence:</strong> you did “{T.steps[seq[firstBad]].text}” before “{T.steps[firstBad].text}” — {T.steps[firstBad].skip ?? ''} <span class="plate">p. 6-17</span></p>}
            <ol class="small" style={{ margin: 0, paddingLeft: '1.3em' }}>{T.steps.map((s) => <li>{s.text}</li>)}</ol>
            <p class="small">Pass result: {T.pass}</p>
            <button class="btn primary sm" onClick={next}>Next</button>
          </div>
        )}
      </div>
    );
  }
  const ti = stage - 4, T = TESTS[ti];
  const chosen = mPick !== null ? OPTIONS[mPick] : null;
  const good = chosen?.test === ti;
  return (
    <div class="stack">
      <span class="small muted num">Part 2 · Match the pass result ({ti + 1} of 4)</span>
      <strong>{T.name}: which result means it PASSED?</strong>
      <div class="stack" style={{ gap: '6px' }} role="group" aria-label="Pass results">
        {OPTIONS.map((o, i) => { const used = o.test >= 0 && o.test < ti; return (
          <button class="btn" style={{ justifyContent: 'flex-start', textAlign: 'left', ...(mPick !== null && o.test === ti ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)' } : mPick === i ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }}
            disabled={used || mPick !== null} onClick={() => { setMPick(i); const g = o.test === ti; if (!g) setMisses(misses + 1); onEvidence({ concepts, ok: g }); }}>
            {used ? `✓ matched to Test ${o.test + 1}: ` : mPick !== null && o.test === ti ? '✓ ' : mPick === i ? '✕ ' : ''}{o.t}</button>); })}
      </div>
      {chosen && (
        <div class={`feedback ${good ? 'good' : 'bad'}`} role="status">
          <div class="verdict">{good ? 'Matched' : 'Not this one'}</div>
          {!good && <p class="small">{chosen.test >= 0 ? `That is the pass result for ${TESTS[chosen.test].name}.` : chosen.why}</p>}
          <p class="small">{T.pass} <span class="plate">p. 6-17</span></p>
          <button class="btn primary sm" onClick={next}>{stage === 7 ? 'Finish' : 'Next'}</button>
        </div>
      )}
      <RigView r={rig(chosen ? (good ? T.passRig : T.failRig) : {})} motion={false} />
    </div>
  );
}

export default function BrakeChecks(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Step through the checks</button>
        <button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Order &amp; match</button>
      </div>
      {mode === 'explore' ? <Explore reducedMotion={props.reducedMotion} /> : <Challenge {...props} />}
    </div>
  );
}
