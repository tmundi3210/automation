import { useState } from 'preact/hooks';
import { C, go, mutate, now, S } from '../app';
import { addDays, dayKey, testsForClass } from '../engine/model';
import { ENDOS, TESTS, isWritten, type Endo } from '../content/tests';
import type { Cls } from '../engine/model';
import { ClassFinderForm } from '../widgets/w/gk01-class-finder';
import { buildPlan } from '../engine/plan';
import { Html } from './bits';
import { peek, adoptState } from '../app';
import { readResumeCode, importFile } from '../store/resume';

export function OnboardingScreen() {
  const s = S();
  const [step, setStep] = useState(0);
  const [cls, setCls] = useState<Cls | null>(s.profile.cls);
  const [finder, setFinder] = useState(false);
  const [ab, setAb] = useState<'need' | 'passed' | 'none'>(s.profile.airBrakesPassed ? 'passed' : s.profile.noAirBrakes ? 'none' : 'need');
  const [endo, setEndo] = useState<Endo[]>(s.profile.endorsements ?? []);
  const [skills, setSkills] = useState(s.profile.skills !== false);
  const choice = () => ({ airBrakesPassed: ab === 'passed', noAirBrakes: ab === 'none', endorsements: endo, skills });
  const tog = (e: Endo) => setEndo((x) => (x.includes(e) ? x.filter((y) => y !== e && !(e === 'P' && y === 'S')) : [...x, e, ...(e === 'S' && !x.includes('P') ? ['P' as Endo] : [])]));
  const [date, setDate] = useState(s.profile.examDates.GK ?? addDays(dayKey(now()), 35));
  const [minutes, setMinutes] = useState(s.profile.minutesPerDay);
  const [paste, setPaste] = useState('');
  const [rmsg, setRmsg] = useState('');
  const restored = (st: ReturnType<typeof readResumeCode>) => { adoptState(st); go(st.profile.onboarded ? 'today' : 'onboarding'); };
  const finish = () => {
    mutate((st) => {
      st.profile.cls = cls ?? 'A';
      Object.assign(st.profile, choice());
      st.profile.tests = testsForClass(cls ?? 'A', choice());
      st.profile.examDates = {}; for (const t of st.profile.tests) st.profile.examDates[t] = date;
      st.profile.minutesPerDay = minutes;
      st.profile.onboarded = true;
    });
    go('today');
  };
  const choices: [Cls, string, string][] = [
    ['A', 'Class A', 'Tractor-trailers and other combinations where the towed unit is over 10,000 lb.'],
    ['B', 'Class B', 'One vehicle of 26,001 lb or more (alone or towing under 10,001 lb), a 3-axle vehicle over 6,000 lb [CA], or a vehicle carrying more than 10 people for pay or a nonprofit.'],
    ['C', 'Class C', 'A vehicle under the Class A and B limits that needs an endorsement: placarded HazMat (H), passengers (P) or a tank (N).'],
  ];
  return (
    <div class="page" style={{ maxWidth: '640px' }}>
      <div class="row small muted num" role="img" aria-label={`Step ${step + 1} of 4`}>{[0, 1, 2, 3].map((k) => <span style={{ flex: 1, height: '5px', borderRadius: '9px', background: k <= step ? 'var(--accent)' : 'var(--surface-2)' }} />)}</div>
      {step === 0 && (
        <section class="stack-lg">
          <div class="stack"><span class="eyebrow">California CDL · written tests</span><h1>Pass your CDL knowledge tests, and understand what you learn.</h1></div>
          <div class="card stack">
            <p>This workshop walks you from zero to test-ready for the California CDL: <strong>General Knowledge</strong>, <strong>Combination</strong>, <strong>Air Brakes</strong>, every endorsement test (Doubles, Tank, Passenger, School Bus, HazMat), and the three <strong>skills tests</strong>.</p>
            <ul class="core">
              <li>{C.lessons.length} short lessons that follow the California Commercial Driver Handbook, with the page for every fact. You only get the ones your license needs.</li>
              <li>Hands-on parts: couple a trailer, trace the air lines, place warning triangles, find your license class.</li>
              <li>Practice and mock tests in the real 3-choice format, with the reason for every answer.</li>
              <li>A mistake list that finds <em>why</em> you missed, and a daily plan that counts down to your test.</li>
            </ul>
            <p class="small muted">Free, no account, no ads. Your progress stays on your device.</p>
          </div>
          <button class="btn primary block" onClick={() => setStep(1)}>Get started</button>
          <details class="card">
            <summary>Coming back? Restore your progress</summary>
            <div class="stack" style={{ marginTop: '10px' }}>
              <label for="ob-code" class="small">Paste your resume code</label>
              <textarea id="ob-code" rows={3} value={paste} onInput={(e) => setPaste((e.target as HTMLTextAreaElement).value)} />
              <button class="btn sm" style={{ alignSelf: 'flex-start' }} disabled={!paste.trim()} onClick={() => { try { restored(readResumeCode(paste, now())); } catch (e) { setRmsg((e as Error).message); } }}>Restore</button>
              <label for="ob-file" class="small">…or open a progress file</label>
              <input id="ob-file" type="file" accept=".json,application/json" onChange={async (e) => { const f = (e.target as HTMLInputElement).files?.[0]; if (!f) return; try { restored(importFile(await f.text(), now())); } catch (er) { setRmsg((er as Error).message); } }} />
              {rmsg && <p class="small" role="alert">{rmsg}</p>}
            </div>
          </details>
          <button class="linkbtn small" style={{ alignSelf: 'center' }} onClick={() => { mutate((st) => { st.profile.onboarded = true; st.profile.cls = st.profile.cls ?? 'A'; st.profile.tests = testsForClass(st.profile.cls, st.profile); }); go('path'); }}>Skip setup and look around</button>
        </section>
      )}
      {step === 1 && (
        <section class="stack-lg">
          <div class="stack"><span class="eyebrow">Step 1 of 3</span><h1>Which license do you need?</h1><p class="muted">Pick the biggest vehicle you plan to drive.</p></div>
          <div class="stack">{choices.map(([c, t, d]) => (
            <button class="choice" aria-pressed={cls === c} onClick={() => setCls(c)}><span class="shield">{c}</span><span><span class="t">{t}</span><br /><span class="small muted">{d}</span></span></button>
          ))}</div>
          <details class="card">
            <summary>What do these words mean?</summary>
            <dl class="kv small" style={{ marginTop: '10px' }}>
              {['GVWR (gross vehicle weight rating)', 'GCWR (gross combination weight rating)', 'Tractor', 'Combination vehicle (also called a rig)', 'Placarded load', 'HazMat'].map((t) => { const g = C.glossary.find((x) => x.term === t); return g ? <><dt><strong>{g.term.replace(/\s*\(.*\)$/, '')}</strong></dt><dd style={{ margin: 0 }}><Html tag="span" html={g.defHtml} /></dd></> : null; })}
              <dt><span class="ca-tag">CA</span></dt><dd style={{ margin: 0 }}>A California rule that may differ from other states or federal rules.</dd>
              <dt><strong>Power unit / towed unit</strong></dt><dd style={{ margin: 0 }}>The vehicle with the engine (truck or tractor) / the trailer it pulls.</dd>
            </dl>
          </details>
          <button class="linkbtn" style={{ alignSelf: 'flex-start' }} onClick={() => setFinder(!finder)}>{finder ? 'Hide the class finder' : 'Not sure? Use the handbook’s class finder'}</button>
          {finder && <div class="widget"><ClassFinderForm onResult={(r) => { if (r.cls === 'A' || r.cls === 'A-88') setCls('A'); else if (r.cls === 'B' || r.cls === 'C') setCls(r.cls); }} /></div>}
          {cls && cls !== 'A' && <p class="card info small">Class {cls} drivers take General Knowledge{cls === 'C' ? ', plus at least one endorsement test (you pick it next)' : ', plus any endorsement tests you pick next'}. The Combination lessons stay open if you want them.</p>}
          <div class="row"><button class="btn" onClick={() => setStep(0)}>Back</button><button class="btn primary" style={{ flex: 1 }} disabled={!cls} onClick={() => setStep(2)}>Next</button></div>
        </section>
      )}
      {step === 2 && (
        <section class="stack-lg">
          <div class="stack"><span class="eyebrow">Step 2 of 3</span><h1>Which tests do you need?</h1><p class="muted">Your lessons, plan and mock tests follow these choices. You can change them later in Settings.</p></div>
          <fieldset class="card stack" style={{ border: 0 }}>
            <legend class="t"><strong>Air brakes</strong></legend>
            {([['need', 'My vehicle has air brakes, and I still need the test', `Air Brakes test: ${TESTS.AB.n} questions, pass ${TESTS.AB.pass}. Most trucks and buses have air brakes.`], ['passed', 'I already passed the Air Brakes test', 'Tests you passed stay passed. The Air Brakes lessons stay open if you want a refresher.'], ['none', 'My vehicle has no air brakes', 'You skip the test, and your license gets an "L" restriction (no air brakes).']] as ['need' | 'passed' | 'none', string, string][]).map(([v, t, d]) => (
              <label class="choice" style={{ cursor: 'pointer' }}><input type="radio" name="ob-ab" checked={ab === v} onChange={() => setAb(v)} style={{ width: '22px', height: '22px', accentColor: 'var(--accent)' }} /><span><span class="t">{t}</span><br /><span class="small muted">{d}</span></span></label>
            ))}
          </fieldset>
          <fieldset class="card stack" style={{ border: 0 }}>
            <legend class="t"><strong>Endorsements</strong> <span class="small muted">(extra tests for special loads; pick any)</span></legend>
            {ENDOS.map((x) => (
              <label class="choice" style={{ cursor: 'pointer' }}><input type="checkbox" checked={endo.includes(x.e)} onChange={() => tog(x.e)} style={{ width: '22px', height: '22px', accentColor: 'var(--accent)' }} /><span><span class="t">{x.label}</span><br /><span class="small muted">{TESTS[x.test].blurb} {TESTS[x.test].n} questions, pass {TESTS[x.test].pass}.</span></span></label>
            ))}
            {cls === 'C' && !endo.some((e) => e === 'H' || e === 'P' || e === 'N' || e === 'S') && <p class="small card warn" role="status">A Class C commercial license needs at least one of H, P or N. Pick the one for the vehicle you will drive.</p>}
          </fieldset>
          <label class="choice" style={{ cursor: 'pointer' }}><input type="checkbox" checked={skills} onChange={(e) => setSkills((e.target as HTMLInputElement).checked)} style={{ width: '22px', height: '22px', accentColor: 'var(--accent)' }} /><span><span class="t">Also prepare me for the skills tests</span><br /><span class="small muted">Vehicle inspection, basic control and road test. You take them after your permit; these lessons come after the written ones and are not counted in your countdown.</span></span></label>
          <div class="row"><button class="btn" onClick={() => setStep(1)}>Back</button><button class="btn primary" style={{ flex: 1 }} onClick={() => setStep(3)}>Next</button></div>
        </section>
      )}
      {step === 3 && (
        <section class="stack-lg">
          <div class="stack"><span class="eyebrow">Step 3 of 3</span><h1>When is your test?</h1><p class="muted">The plan works backward from this date. You can change it any time.</p></div>
          <div class="card stack">
            <div class="field"><label for="ob-date">Knowledge test date</label><input id="ob-date" type="date" min={dayKey(now())} value={date} onInput={(e) => setDate((e.target as HTMLInputElement).value)} /></div>
            <div class="field"><label for="ob-min">Minutes you can study a day: <strong class="num">{minutes}</strong></label><input id="ob-min" type="range" min={15} max={180} step={5} value={minutes} onInput={(e) => setMinutes(+(e.target as HTMLInputElement).value)} /></div>
            {(() => { const tmp = JSON.parse(JSON.stringify(peek())); Object.assign(tmp.profile, choice()); tmp.profile.tests = testsForClass(cls ?? 'A', choice()); tmp.profile.examDates = {}; for (const t of tmp.profile.tests) tmp.profile.examDates[t] = date; tmp.profile.minutesPerDay = minutes; const pl = buildPlan(tmp, C, now());
              return <div class={`card ${pl.feasibility === 'go' ? 'tint' : 'warn'}`} role="status"><strong>{pl.feasibility === 'go' ? 'That works.' : pl.feasibility === 'tight' ? 'Tight but possible.' : 'Not enough time yet.'}</strong> <span class="small">{pl.feasibility === 'go' ? `About ${pl.needMinPerDay} minutes a day gets you there.` : `You need about ${pl.needMinPerDay} minutes a day for this date. Raise the minutes or pick a later date.`}</span>{pl.feasibility !== 'go' && pl.needMinPerDay <= 175 && <button class="btn sm" style={{ marginTop: '8px', display: 'flex' }} onClick={() => setMinutes(Math.min(180, Math.ceil(pl.needMinPerDay / 5) * 5 + 5))}>Use {Math.min(180, Math.ceil(pl.needMinPerDay / 5) * 5 + 5)} minutes a day</button>}</div>; })()}
            {(() => { const ts = testsForClass(cls ?? 'A', choice()); const ls = C.lessons.filter((l) => ts.includes(l.test) && isWritten(l.test)); return <p class="small muted">Your {ls.length} written-test lessons ({ts.filter(isWritten).map((t) => TESTS[t].short).join(', ')}) take about {Math.round(ls.reduce((a, l) => a + l.minutes, 0) / 60)} hours in total, plus review. Lessons stop a few days before the test so the last days are for review and mock tests.</p>; })()}
          </div>
          <div class="row"><button class="btn" onClick={() => setStep(2)}>Back</button><button class="btn primary" style={{ flex: 1 }} onClick={finish}>Build my plan</button></div>
        </section>
      )}
    </div>
  );
}
