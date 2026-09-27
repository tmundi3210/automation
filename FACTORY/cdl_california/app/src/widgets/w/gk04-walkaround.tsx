import { useState } from 'preact/hooks';
import type { ComponentChildren } from 'preact';
import type { WidgetMeta, WidgetProps } from '../registry';
import { TruckMap, Tread, Spring, Wheel, Kit, Air } from './gk04-walkaround.art';

export const meta: WidgetMeta = {
  id: 'gk04-walkaround', title: 'Walk-around: the 7-step inspection', lesson: 'GK-04', anchor: /the 7-step inspection method/i,
  summary: 'Tap each step on the truck to see what to check and whether the engine is on. Then hunt 8 defects with a tread gauge, a spring, the wheel and the gauges.',
  stamp: { id: 'sharp-eyes', name: 'Sharp eyes', rule: 'Find all 8 defects in the defect hunt with no mistakes.' },
};

interface Step { key: string; title: string; engine: string; page: string; checks: string[] }
export const STEPS: Step[] = [
  { key: 'A', title: 'Approach', engine: 'OFF', page: '2-4', checks: ['Damage, or the vehicle leaning to one side', 'Under it: fresh oil, coolant, grease or fuel', 'Area hazards: people, vehicles, objects, low wires, limbs'] },
  { key: '1', title: 'Step 1 · Review the last inspection report', engine: 'OFF', page: '2-4', checks: ['Read the last vehicle inspection report', 'Sign it only when it listed defects and the carrier certified them fixed or not needing repair'] },
  { key: '2', title: 'Step 2 · Engine compartment', engine: 'OFF', page: '2-4', checks: ['FIRST: parking brakes on and/or wheels chocked (it must not roll while you work in front)', 'Engine oil; coolant level and hoses; power steering fluid and hose; washer fluid', 'Battery fluid, connections, tie-downs; automatic transmission fluid', 'Belts (alternator, water pump, air compressor) for tightness and wear; leaks; wiring insulation'] },
  { key: '3', title: 'Step 3 · Start the engine, inspect inside the cab', engine: 'ON', page: '2-4 – 2-6', checks: ['Parking brake on; neutral (park if automatic); start and listen', 'ABS light comes on briefly, then goes out (stays on = ABS fault)', 'Oil pressure normal within seconds; temperatures rise gradually', 'Air pressure 50 → 90 psi within 3 minutes, then to governor cut-out (usually 120–140 psi)', 'Controls, mirrors and windshield, emergency equipment, seat belt'] },
  { key: '4', title: 'Step 4 · Engine off, check lights', engine: 'OFF · key with you', page: '2-6', checks: ['Parking brake set, engine off, take the key with you', 'Turn on low beams and 4-way flashers, then get out'] },
  { key: '5', title: 'Step 5 · Walk-around', engine: 'OFF', page: '2-6 – 2-8', checks: ['At the front: low beams and both 4-ways work; check high beams', 'Then parking, clearance, side-marker and ID lights on, plus the RIGHT turn signal', 'Order: left front → front → right side → right rear → rear → left side (tap 5a–5f)'] },
  { key: '5a', title: '5a · Left front', engine: 'OFF', page: '2-6; 2-2', checks: ['Driver’s door glass clean; latches and locks work', 'Wheel: no missing studs or lugs; wrench-test rust-streaked lug nuts', 'Tire inflated, valve stem and cap, no cuts or bulges; front tread at least 4/32 in', 'Hub oil level; wheel bearings/seals not leaking; springs, shocks, brake adjustment'] },
  { key: '5b', title: '5b · Front', engine: 'OFF', page: '2-6 – 2-7', checks: ['Front axle; steering system: no loose, worn, bent, damaged or missing parts', 'Grab the steering mechanism to test for looseness', 'Windshield, wiper arm spring tension, wiper blades', 'Front lights and reflectors amber (turn signals amber or white)'] },
  { key: '5c', title: '5c · Right side', engine: 'OFF', page: '2-7', checks: ['Cab-over: primary and secondary cab locks engaged', 'Fuel tanks secure, not leaking; caps on; enough fuel', 'Exhaust secure, not touching wires, fuel or air lines; frame not cracked', 'Spare tire secure, right size, inflated; side markers amber (red at rear)'] },
  { key: '5d', title: '5d · Right rear', engine: 'OFF', page: '2-7; 2-2, 2-3', checks: ['Wheel and tire checks (other tires: at least 2/32 in tread)', 'Rear duals: not rubbing, nothing stuck between, same type and size', 'Springs: ¼ or more leaves missing = out of service', 'Brake drums, hoses, brake adjustment; drive axle not leaking gear oil'] },
  { key: '5e', title: '5e · Rear', engine: 'OFF', page: '2-7 – 2-8', checks: ['Rear lights and reflectors red; license plate present and clean', 'Splash guards fastened, not dragging or rubbing', 'Cargo blocked, braced, tied; doors closed and locked'] },
  { key: '5f', title: '5f · Left side', engine: 'OFF', page: '2-8', checks: ['Same as the right side', 'Plus batteries (if not under the hood): box secure, cover on, not leaking'] },
  { key: '6', title: 'Step 6 · Check signal lights', engine: 'OFF', page: '2-8', checks: ['All lights off; stop lights and LEFT turn signal on; get out and check', 'Check brake, turn-signal and 4-way lights separately', 'Papers; secure loose items in the cab; then start the engine'] },
  { key: '7', title: 'Step 7 · Start the engine, check brakes', engine: 'ON', page: '2-8', checks: ['Hydraulic leak test: pump the pedal 3 times, press firmly, hold 5 seconds; pedal must not move', 'Parking brake: low gear, gently pull forward against it; it must hold', 'Service brake at about 5 mph: pulling, odd pedal feel or a late stop = trouble'] },
];
const TOP = ['A', '1', '2', '3', '4', '5', '6', '7'];

function Explore({ reducedMotion }: { reducedMotion: boolean }) {
  const [sel, setSel] = useState('A');
  const [front, setFront] = useState(true);
  const [tread, setTread] = useState(5);
  const st = STEPS.find((s) => s.key === sel)!;
  const idx = STEPS.indexOf(st);
  const on = st.engine.startsWith('ON');
  return (
    <div class="stack">
      <div role="group" aria-label="Inspection step" style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(64px, 1fr))', gap: '4px' }}>{TOP.map((k) => <button class="btn sm" aria-pressed={sel === k || (k === '5' && sel.startsWith('5'))} style={{ minWidth: 0, padding: 0, ...(sel === k || (k === '5' && sel.startsWith('5')) ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}) }} aria-label={STEPS.find((s) => s.key === k)!.title} onClick={() => setSel(k)}>{k}</button>)}</div>
      {sel.startsWith('5') && <div role="group" aria-label="Walk-around stops" style={{ display: 'grid', gridTemplateColumns: 'repeat(6, 1fr)', gap: '4px' }}>{['5a', '5b', '5c', '5d', '5e', '5f'].map((k) => <button class="btn sm" aria-pressed={sel === k} style={{ minWidth: 0, padding: 0, ...(sel === k ? { background: 'var(--accent-soft)', borderColor: 'var(--accent)' } : {}) }} aria-label={STEPS.find((s) => s.key === k)!.title} onClick={() => setSel(k)}>{k}</button>)}</div>}
      <TruckMap sel={sel} engine={st.engine} onPick={setSel} reducedMotion={reducedMotion} />
      <div class="card tint stack" role="status" aria-live="polite" style={{ gap: '6px' }}>
        <div class="spread"><strong>{st.title}</strong><span class="chip" style={{ background: on ? 'var(--ok)' : 'var(--surface)', color: on ? '#fff' : 'var(--ink)', borderColor: 'var(--ink-2)' }}>{on ? '● Engine ON' : `○ Engine ${st.engine}`}</span></div>
        <ul class="core small">{st.checks.map((c) => <li>{c}</li>)}</ul>
        <div class="spread"><span class="plate">p. {st.page}</span>
          <div class="row" style={{ gap: '4px' }}><button class="btn sm" disabled={idx === 0} onClick={() => setSel(STEPS[idx - 1].key)}>← Back</button><button class="btn sm primary" disabled={idx === STEPS.length - 1} onClick={() => setSel(STEPS[idx + 1].key)}>Next →</button></div></div>
      </div>
      <div class="card flat stack">
        <strong>Tool: tread depth gauge</strong>
        <div class="row" role="group" aria-label="Which tire"><button class="btn sm" aria-pressed={front} style={front ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}} onClick={() => setFront(true)}>Front tire</button><button class="btn sm" aria-pressed={!front} style={!front ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {}} onClick={() => setFront(false)}>Any other tire</button></div>
        <div class="field"><label for="wk-tread">Tread in a major groove: <strong class="num">{tread}/32 inch</strong></label><input id="wk-tread" type="range" min={0} max={10} value={tread} aria-valuetext={`${tread}/32 inch`} onInput={(e) => setTread(+(e.target as HTMLInputElement).value)} /></div>
        <Tread v={tread} min={front ? 4 : 2} />
        <p class="small">Front tires need at least <strong>4/32 inch</strong> in every major groove (they steer, so they need the most grip); all other tires <strong>2/32 inch</strong>. <span class="plate">p. 2-2</span></p>
      </div>
    </div>
  );
}

interface Hunt { text: string; art: (verdict?: string) => ComponentChildren; choices: string[]; answer: string; verdict: string; why: string; bad: string; page: string; slider?: { min: number; max: number; unit: string } }
export const HUNT: Hunt[] = [
  { text: 'Front steer tire. Slide the gauge to the LEAST tread the handbook allows in a major groove, then check.', art: () => null, slider: { min: 0, max: 10, unit: '/32 in' }, choices: [], answer: '4', verdict: 'Minimum 4/32', why: 'Front tires need at least 4/32 inch in every major groove; other tires need 2/32.', bad: 'At that setting you would pass a front tire without enough grip to steer.', page: '2-2' },
  { text: 'Right rear (drive) tire: the gauge reads 3/32 inch. Verdict?', art: (v) => <Tread v={3} min={2} verdict={v} />, choices: ['OK', 'Defect'], answer: 'OK', verdict: '✓ OK', why: 'Tires other than the front need only 2/32 inch, so 3/32 passes. (The same 3/32 on a front tire would be a defect.)', bad: 'You would take a legal tire out of service: 4/32 is the front-tire rule.', page: '2-2' },
  { text: 'A leaf spring has 8 leaves; 2 are missing. Verdict?', art: (v) => <Spring leaves={8} missing={2} verdict={v} />, choices: ['OK to drive', 'Out of service'], answer: 'Out of service', verdict: 'OUT OF SERVICE', why: '¼ of 8 = 2. Missing ¼ or more of the leaves = out of service. Broken suspension parts are extremely dangerous.', bad: 'Driving on it risks the axle moving out of position.', page: '2-3' },
  { text: '20-inch steering wheel: it turns 3 inches at the rim before the front tires move. Verdict?', art: (v) => <Wheel inches={3} verdict={v} />, choices: ['Within limits', 'Defect: too much play'], answer: 'Defect: too much play', verdict: '✗ DEFECT', why: 'More than 10 degrees of play (about 2 inches at the rim of a 20-inch wheel) makes the vehicle hard to steer.', bad: 'With that much play the truck is hard to steer.', page: '2-2' },
  { text: 'Emergency kit in the cab (no circuit breakers). What is missing?', art: () => <Kit triangles={2} />, choices: ['Nothing, it is complete', 'A third red reflective triangle', 'Tire chains'], answer: 'A third red reflective triangle', verdict: '', why: 'Required: fire extinguisher, spare electrical fuses (unless circuit breakers), and warning devices such as 3 red reflective triangles. Tire chains are optional.', bad: 'Parked on the shoulder, you could not warn traffic properly.', page: '2-4' },
  { text: 'Hydraulic brakes, step 7 leak test. Which is right?', art: () => null, choices: ['Pump 5 times, hold 3 seconds', 'Pump 3 times, hold 5 seconds', 'Pump once, hold 10 seconds'], answer: 'Pump 3 times, hold 5 seconds', verdict: '', why: 'Pump the pedal 3 times, press firmly and hold 5 seconds. The pedal should not move; if it does, suspect a leak and fix it before driving.', bad: 'The wrong test can miss a leak.', page: '2-8' },
  { text: 'Step 3, engine running: the air gauge took 4 minutes to go from 50 to 90 psi. Verdict?', art: (v) => <Air minutes={4} verdict={v} />, choices: ['Normal', 'Too slow: defect'], answer: 'Too slow: defect', verdict: '✗ TOO SLOW', why: 'For the section 2.1 check, pressure should build from 50 to 90 psi within 3 minutes, then to governor cut-out (usually 120–140 psi).', bad: 'Slow build-up can mean an air system problem; you may not have air for the brakes.', page: '2-5' },
  { text: 'In which steps of the 7-step method is the engine running?', art: () => null, choices: ['Steps 2 and 3', 'Steps 3 and 7', 'Only step 7'], answer: 'Steps 3 and 7', verdict: '', why: 'Step 3 starts the engine to inspect inside the cab; step 4 turns it off and you take the key with you; step 7 starts it again to test the brakes. Step 2 (engine compartment) is engine off.', bad: 'Working in the engine compartment with the engine running is dangerous.', page: '2-4 – 2-8' },
];

export default function Walkaround({ onEvidence, onChallenge, concepts, reducedMotion }: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'hunt'>('explore');
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<string | null>(null);
  const [misses, setMisses] = useState(0);
  const [gauge, setGauge] = useState(6);
  const h = HUNT[i];
  const answer = (ch: string) => { setPick(ch); const ok = ch === h.answer; if (!ok) setMisses(misses + 1); onEvidence({ concepts, ok }); };
  const right = pick === h?.answer;
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Walk the 7 steps</button><button role="tab" aria-selected={mode === 'hunt'} onClick={() => setMode('hunt')}>Defect hunt (8)</button></div>
      {mode === 'explore' && <Explore reducedMotion={reducedMotion} />}
      {mode === 'hunt' && (h ? (
        <div class="stack">
          <span class="small muted num">Item {i + 1} of {HUNT.length}</span>
          <strong>{h.text}</strong>
          {h.slider ? (<>
            <div class="field"><label for="wk-hunt">Gauge setting: <strong class="num">{gauge}{h.slider.unit}</strong></label><input id="wk-hunt" type="range" min={h.slider.min} max={h.slider.max} value={gauge} aria-valuetext={`${gauge}/32 inch`} disabled={!!pick} onInput={(e) => setGauge(+(e.target as HTMLInputElement).value)} /></div>
            <Tread v={pick ? +pick : gauge} min={pick ? 4 : -1} verdict={pick ? (right ? '✓ ' + h.verdict : `✗ ${pick}/32 ≠ 4/32`) : ' '} />
            <div class="row"><button class="btn primary sm" disabled={!!pick} onClick={() => answer(String(gauge))}>Check this setting</button></div>
          </>) : (<>
            {h.art(pick ? h.verdict : undefined)}
            <div class="stack" role="group" aria-label="Choices" style={{ gap: '8px' }}>{h.choices.map((ch) => (
              <button class={`btn ${pick && ch === h.answer ? 'primary' : ''}`} style={{ justifyContent: 'flex-start', ...(pick && ch === pick && !right ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }} disabled={!!pick} onClick={() => answer(ch)}>{pick && ch === h.answer ? '✓ ' : pick && ch === pick ? '✗ ' : ''}{ch}</button>))}</div>
          </>)}
          {pick && <div class={`feedback ${right ? 'good' : 'bad'}`} role="status"><div class="verdict">{right ? 'Found it' : h.slider ? 'No: 4/32 inch' : `No: ${h.answer}`}</div>
            {!right && <p class="small"><strong>Consequence:</strong> {h.bad}</p>}
            <p class="small">{h.why} <span class="plate">p. {h.page}</span></p>
            <button class="btn primary sm" onClick={() => { if (i + 1 === HUNT.length && misses === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === HUNT.length ? 'Finish' : 'Next item'}</button></div>}
        </div>
      ) : (
        <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`}><div class="verdict">{misses === 0 ? 'All 8 right. Stamp earned: Sharp eyes' : `${HUNT.length - misses} of ${HUNT.length} right`}</div>
          <p class="small">4/32 front, 2/32 others · ¼ of leaves · 10° play · 3 triangles · pump 3, hold 5 · 50→90 psi in 3 min.</p>
          <button class="btn sm" onClick={() => { setI(0); setMisses(0); setPick(null); setGauge(6); }}>Hunt again</button></div>
      ))}
    </div>
  );
}
