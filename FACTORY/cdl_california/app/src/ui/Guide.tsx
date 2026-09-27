import { useState } from 'preact/hooks';
import { C, go, S } from '../app';
import { Html, Shield } from './bits';
import { ENDOS, TESTS } from '../content/tests';

const VERIFIED = 'September 2026';

export function GuideScreen() {
  const s = S();
  const cls = s.profile.cls;
  const steps: { t: string; d: string; lesson?: string }[] = [
    { t: 'Get a DOT medical exam', d: 'A certified medical examiner gives you a Medical Examination Report and a Medical Examiner’s Certificate. Bring both to the DMV; both must be dated within the last 2 years.', lesson: 'GK-01' },
    { t: 'Apply at the DMV and pass the knowledge tests', d: `Everyone takes General Knowledge (50 questions, pass 40).${cls === 'A' ? ' Class A also takes Combination Vehicles (20 questions, pass 16), and Air Brakes (25 questions, pass 20) if the truck has air brakes.' : ' Take Air Brakes (25 questions, pass 20) if your vehicle has air brakes.'} You get 3 tries per test on one application. (Question counts are the DMV test format; the handbook does not print them, so confirm when you book.)`, lesson: 'GK-01' },
    ...(s.profile.airBrakesPassed ? [] : s.profile.noAirBrakes ? [{ t: 'Skipping Air Brakes', d: 'If you skip or fail the Air Brakes knowledge test your license gets restriction L (no air-brake CMV); testing in a vehicle with air-over-hydraulic brakes gives Z. The Air Brakes lessons stay open if you change your mind.', lesson: 'AB-01' }]
      : [{ t: `Study Air Brakes (${TESTS.AB.n} questions, pass ${TESTS.AB.pass})`, d: `${cls === 'A' ? 'Most Class A trucks have air brakes, and the Combination lessons build on it.' : 'Take this test if the vehicle you will drive or test in has air brakes.'} Without it you get restriction L (no air-brake CMV).`, lesson: 'AB-01' }]),
    ...ENDOS.filter((x) => s.profile.tests.includes(x.test)).map((x) => ({ t: `${TESTS[x.test].name} endorsement (${x.e})`, d: `${TESTS[x.test].blurb} Knowledge test: ${TESTS[x.test].n} questions, pass ${TESTS[x.test].pass}.${x.e === 'H' ? ' You also need a TSA background check and must be 21 or older.' : ''}${x.e === 'P' || x.e === 'S' ? ' Also needs a skills test in that kind of vehicle.' : ''}`, lesson: C.lessons.find((l) => l.test === x.test)?.id })),
    ...(cls === 'C' && !(s.profile.endorsements ?? []).some((e) => e === 'H' || e === 'P' || e === 'N') ? [{ t: 'Pick your endorsement', d: 'A Class C CDL is for a smaller vehicle that needs an endorsement: placarded HazMat (H), passengers (P) or a tank (N). Add it in Settings to get its lessons.' }] : []),
    { t: 'Get your commercial learner’s permit (CLP)', d: 'Once you pass the knowledge tests you get a CLP (you need a regular Class C license first). It lasts 180 days, and a CDL holder must ride along whenever you drive.', lesson: 'GK-01' },
    { t: 'Wait at least 14 days', d: 'You must hold the CLP for at least 14 days before the skills test.', lesson: 'GK-01' },
    { t: 'Finish entry-level driver training (ELDT)', d: 'Federal rules require training from a school listed on the FMCSA Training Provider Registry before a first Class A or B skills test. This app is study help, not an ELDT provider.' },
    { t: 'Pass the skills tests', d: 'Vehicle inspection, basic control (backing), and the road test, in the kind of vehicle you want to drive. Skills tests are given in English.', lesson: 'SK-01' },
  ];
  return (
    <div class="page">
      <header class="stack" style={{ gap: '6px' }}><span class="eyebrow">Guide</span><h1>From here to your CDL</h1>
        <p class="muted">{cls ? `Your path for a Class ${cls} license in California.` : 'Pick your license class in Settings to tailor this page.'}</p></header>
      <section class="card stack" aria-label="Steps">
        <ol class="stack" style={{ margin: 0, paddingLeft: '1.2em' }}>
          {steps.map((x) => <li><strong>{x.t}</strong><br /><span class="small">{x.d}</span>{x.lesson && C.lessons.some((l) => l.id === x.lesson) && <> <button class="linkbtn small" onClick={() => go('lesson', x.lesson)}>Details in {x.lesson}</button></>}</li>)}
        </ol>
        <p class="small muted">Rules and fees change. Check the current steps and book appointments with the DMV at <a href="https://www.dmv.ca.gov" target="_blank" rel="noopener noreferrer">dmv.ca.gov</a> or 1-800-777-0133 (checked {VERIFIED}). Find a registered ELDT school at <a href="https://tpr.fmcsa.dot.gov" target="_blank" rel="noopener noreferrer">tpr.fmcsa.dot.gov</a>. Knowledge-test languages are changing under federal English-proficiency rules, so confirm the language when you book.</p>
      </section>
      <section class="card stack" aria-label="Test day">
        <h2>On test day</h2>
        <ul class="core">
          <li>The knowledge test is on a touchscreen. It shows right away when an answer is wrong and how many questions are left.</li>
          <li>There is no official time limit, but the DMV will not start a knowledge test in about the last 30 minutes before closing.</li>
          <li>A phone, smartwatch, notes, or anyone helping you is an <strong>automatic fail</strong>.</li>
          <li>Tests you already passed stay passed. You get 3 tries per test on one application.</li>
          <li>Read every question twice. The test often flips the wording: “Which is <strong>NOT</strong>…”, “all <strong>EXCEPT</strong>…”, “True or <strong>False</strong>”.</li>
        </ul>
      </section>
      <section class="card stack" aria-label="Answer the handbook way">
        <h2>Answer the handbook way</h2>
        <p class="small">Websites, apps, federal law and the car handbook sometimes give a different answer. The DMV test follows the California Commercial Driver Handbook.</p>
        <div class="tbl" tabindex={0} role="region" aria-label="Table (scrolls sideways)"><table><thead><tr><th>Topic</th><th>Test answer (handbook)</th><th>What you may see elsewhere</th></tr></thead><tbody>
          {C.handbookWay.map((r) => <tr><td><Html tag="span" html={r.topic} /></td><td><Html tag="span" html={r.answer} /></td><td><Html tag="span" html={r.elsewhere} /></td></tr>)}
        </tbody></table></div>
      </section>
      <section class="card stack" aria-label="Most missed">
        <h2>The most-missed questions</h2>
        <MostMissed />
      </section>
      <p class="small muted">Not affiliated with the California DMV, CHP or FMCSA. Lessons follow the California Commercial Driver Handbook (DL 650, 2019 edition) with page numbers so you can check every fact.</p>
    </div>
  );
}

function MostMissed() {
  const [t, setT] = useState<'GK' | 'CV'>('GK');
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={t === 'GK'} onClick={() => setT('GK')}>General Knowledge</button><button role="tab" aria-selected={t === 'CV'} onClick={() => setT('CV')}>Combination</button></div>
      <ol class="stack" style={{ margin: 0, paddingLeft: '1.4em', gap: '8px' }}>
        {C.mostMissed.filter((m) => m.test === t).map((m) => <li><Html tag="span" html={m.text} /> <button class="linkbtn small" onClick={() => go('lesson', m.lesson)}>{m.lesson}</button></li>)}
      </ol>
    </div>
  );
}

export function GlossaryScreen() {
  const [q, setQ] = useState('');
  const list = C.glossary.filter((g) => !q || (g.term + ' ' + g.defHtml).toLowerCase().includes(q.toLowerCase()));
  return (
    <div class="page">
      <header class="stack" style={{ gap: '6px' }}><span class="eyebrow">Glossary</span><h1>Words you will see</h1></header>
      <div class="field"><label for="gq">Search</label><input id="gq" type="search" value={q} onInput={(e) => setQ((e.target as HTMLInputElement).value)} placeholder="e.g. GVWR, kingpin, bobtail" /></div>
      <div class="card"><div class="list">
        {list.map((g) => <div class="li" style={{ cursor: 'default', gridTemplateColumns: '1fr auto' }}><div><strong>{g.term}</strong><br /><Html tag="span" class="small" html={g.defHtml} /></div><button class="btn sm ghost" onClick={() => go('lesson', g.lesson)} aria-label={`Open ${g.lesson}`}><Shield id={g.lesson} /></button></div>)}
        {!list.length && <p class="muted" style={{ padding: '12px' }}>No match. Try a shorter word.</p>}
      </div></div>
    </div>
  );
}
