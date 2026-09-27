import { useState } from 'preact/hooks';
import { C, go, mutate, now, S } from '../app';
import { addDays, dayKey, testsForClass } from '../engine/model';
import type { Cls } from '../engine/model';
import { ClassFinderForm } from '../widgets/w/gk01-class-finder';
import { buildPlan } from '../engine/plan';
import { peek } from '../app';

export function OnboardingScreen() {
  const s = S();
  const [step, setStep] = useState(0);
  const [cls, setCls] = useState<Cls | null>(s.profile.cls);
  const [finder, setFinder] = useState(false);
  const [ab, setAb] = useState(s.profile.airBrakesPassed);
  const [date, setDate] = useState(s.profile.examDates.GK ?? addDays(dayKey(now()), 14));
  const [minutes, setMinutes] = useState(s.profile.minutesPerDay);
  const finish = () => {
    mutate((st) => {
      st.profile.cls = cls ?? 'A';
      st.profile.tests = testsForClass(cls ?? 'A');
      st.profile.airBrakesPassed = ab;
      st.profile.examDates = {}; for (const t of st.profile.tests) st.profile.examDates[t] = date;
      st.profile.minutesPerDay = minutes;
      st.profile.onboarded = true;
    });
    go('today');
  };
  const choices: [Cls, string, string][] = [
    ['A', 'Class A', 'Tractor-trailers and other combinations where the towed unit is over 10,000 lb.'],
    ['B', 'Class B', 'One heavy truck or bus (over 26,000 lb), or a 3-axle vehicle over 6,000 lb, or a bus for more than 10 people.'],
    ['C', 'Class C', 'A smaller vehicle that hauls placarded HazMat or carries passengers.'],
  ];
  return (
    <div class="page" style={{ maxWidth: '640px' }}>
      <div class="row small muted num" aria-label={`Step ${step + 1} of 4`}>{[0, 1, 2, 3].map((k) => <span style={{ flex: 1, height: '5px', borderRadius: '9px', background: k <= step ? 'var(--accent)' : 'var(--surface-2)' }} />)}</div>
      {step === 0 && (
        <section class="stack-lg">
          <div class="stack"><span class="eyebrow">California CDL · written tests</span><h1>Pass your CDL knowledge tests, and understand what you learn.</h1></div>
          <div class="card stack">
            <p>This workshop walks you from zero to test-ready for the <strong>General Knowledge</strong> test and the <strong>Combination Vehicles</strong> test.</p>
            <ul class="core">
              <li>18 short lessons that follow the California Commercial Driver Handbook, with the page for every fact.</li>
              <li>Hands-on parts: couple a trailer, trace the air lines, place warning triangles, find your license class.</li>
              <li>Practice and mock tests in the real 3-choice format, with the reason for every answer.</li>
              <li>A mistake list that finds <em>why</em> you missed, and a daily plan that counts down to your test.</li>
            </ul>
            <p class="small muted">Free, no account, no ads. Your progress stays on your device.</p>
          </div>
          <button class="btn primary block" onClick={() => setStep(1)}>Get started</button>
          <button class="linkbtn small" style={{ alignSelf: 'center' }} onClick={() => { mutate((st) => { st.profile.onboarded = true; st.profile.cls = st.profile.cls ?? 'A'; st.profile.tests = testsForClass(st.profile.cls); }); go('path'); }}>Skip setup and look around</button>
        </section>
      )}
      {step === 1 && (
        <section class="stack-lg">
          <div class="stack"><span class="eyebrow">Step 1 of 3</span><h1>Which license do you need?</h1><p class="muted">Pick the biggest vehicle you plan to drive.</p></div>
          <div class="stack">{choices.map(([c, t, d]) => (
            <button class="choice" aria-pressed={cls === c} onClick={() => setCls(c)}><span class="shield">{c}</span><span><span class="t">{t}</span><br /><span class="small muted">{d}</span></span></button>
          ))}</div>
          <button class="linkbtn" style={{ alignSelf: 'flex-start' }} onClick={() => setFinder(!finder)}>{finder ? 'Hide the class finder' : 'Not sure? Use the handbook’s class finder'}</button>
          {finder && <div class="widget"><ClassFinderForm onResult={(r) => { if (r.cls === 'A' || r.cls === 'A-88') setCls('A'); else if (r.cls === 'B' || r.cls === 'C') setCls(r.cls); }} /></div>}
          {cls && cls !== 'A' && <p class="card info small">Class {cls} drivers take General Knowledge, plus endorsement tests (for example Passenger or HazMat) that this version does not teach yet. The Combination lessons stay open if you want them.</p>}
          <div class="row"><button class="btn" onClick={() => setStep(0)}>Back</button><button class="btn primary" style={{ flex: 1 }} disabled={!cls} onClick={() => setStep(2)}>Next</button></div>
        </section>
      )}
      {step === 2 && (
        <section class="stack-lg">
          <div class="stack"><span class="eyebrow">Step 2 of 3</span><h1>What have you passed?</h1></div>
          <label class="choice" style={{ cursor: 'pointer' }}><input type="checkbox" checked={ab} onChange={(e) => setAb((e.target as HTMLInputElement).checked)} style={{ width: '22px', height: '22px', accentColor: 'var(--accent)' }} /><span><span class="t">I already passed the Air Brakes test</span><br /><span class="small muted">This version teaches General Knowledge{cls === 'A' ? ' and Combination Vehicles' : ''}. Air Brakes lessons are planned for later. Combination lesson CV-02 builds on air brakes.</span></span></label>
          <div class="row"><button class="btn" onClick={() => setStep(1)}>Back</button><button class="btn primary" style={{ flex: 1 }} onClick={() => setStep(3)}>Next</button></div>
        </section>
      )}
      {step === 3 && (
        <section class="stack-lg">
          <div class="stack"><span class="eyebrow">Step 3 of 3</span><h1>When is your test?</h1><p class="muted">The plan works backward from this date. You can change it any time.</p></div>
          <div class="card stack">
            <div class="field"><label for="ob-date">Knowledge test date</label><input id="ob-date" type="date" min={dayKey(now())} value={date} onInput={(e) => setDate((e.target as HTMLInputElement).value)} /></div>
            <div class="field"><label for="ob-min">Minutes you can study a day: <strong class="num">{minutes}</strong></label><input id="ob-min" type="range" min={15} max={180} step={5} value={minutes} onInput={(e) => setMinutes(+(e.target as HTMLInputElement).value)} /></div>
            {(() => { const tmp = JSON.parse(JSON.stringify(peek())); tmp.profile.tests = testsForClass(cls ?? 'A'); tmp.profile.examDates = {}; for (const t of tmp.profile.tests) tmp.profile.examDates[t] = date; tmp.profile.minutesPerDay = minutes; const pl = buildPlan(tmp, C, now());
              return <div class={`card ${pl.feasibility === 'go' ? 'tint' : 'warn'}`} role="status"><strong>{pl.feasibility === 'go' ? 'That works.' : pl.feasibility === 'tight' ? 'Tight but possible.' : 'Not enough time yet.'}</strong> <span class="small">{pl.feasibility === 'go' ? `About ${pl.needMinPerDay} minutes a day gets you there.` : `You need about ${pl.needMinPerDay} minutes a day for this date. Raise the minutes or pick a later date.`}</span>{pl.feasibility !== 'go' && <button class="btn sm" style={{ marginTop: '8px' }} onClick={() => setMinutes(Math.min(180, Math.ceil(pl.needMinPerDay / 5) * 5 + 5))}>Use {Math.min(180, Math.ceil(pl.needMinPerDay / 5) * 5 + 5)} minutes a day</button>}</div>; })()}
            <p class="small muted">The {cls === 'A' ? 18 : 14} lessons take about {Math.round(C.lessons.filter((l) => cls === 'A' || l.test === 'GK').reduce((a, l) => a + l.minutes, 0) / 60)} hours in total, plus review. Lessons stop a few days before the test so the last days are for review and mock tests.</p>
          </div>
          <div class="row"><button class="btn" onClick={() => setStep(2)}>Back</button><button class="btn primary" style={{ flex: 1 }} onClick={finish}>Build my plan</button></div>
        </section>
      )}
    </div>
  );
}
