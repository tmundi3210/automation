import { C, go, mutate, now, S, startSession } from '../app';
import { dayKey, addDays, CAUSE_HELP, CAUSE_LABEL, daysBetween } from '../engine/model';
import type { Cause } from '../engine/model';
import { buildPlan, today, activeDaysLast30, fmtDay } from '../engine/plan';
import { readiness, TEST_FORMAT } from '../engine/readiness';
import { openRows, lessonDone, newSurfaces } from '../engine/learner';
import { Bar, Calendar, Html, Ring, Shield } from './bits';
import { startTest } from './Lesson';
import type { TestId } from '../content/types';
import { WIDGETS } from '../widgets/registry';
import { planToIcs } from '../engine/ics';
import { TESTS, TEST_ORDER, isWritten } from '../content/tests';

const inArtifact = () => !!(globalThis as unknown as { claude?: unknown }).claude;
function downloadIcs(text: string) { try { const a = document.createElement('a'); a.href = URL.createObjectURL(new Blob([text], { type: 'text/calendar' })); a.download = 'cdl-study-plan.ics'; document.body.appendChild(a); a.click(); a.remove(); } catch { /* */ } }

const TNAME = (t: TestId) => TESTS[t].name;

function lessonMastery(id: string) {
  const s = S(); const L = C.lessons.find((l) => l.id === id)!;
  return L.conceptIds.reduce((a, c) => a + (s.bkt[c]?.p ?? 0), 0) / L.conceptIds.length;
}

// ------------------------------------------------------------------ Today
export function TodayScreen() {
  const s = S();
  const t = now();
  const q = today(s, C, t);
  const plan = buildPlan(s, C, t);
  const exam = plan.examDay;
  const daysLeft = exam ? daysBetween(dayKey(t), exam) : null;
  const lessonsLeft = C.lessons.filter((l) => s.profile.tests.includes(l.test) && !lessonDone(s, l.id)).length;
  const reviewIds = q.reviews.slice(0, 40).map((x) => x.id);
  const nl = q.lesson ? C.lessons.find((l) => l.id === q.lesson)! : null;
  const nlStarted = nl ? !!s.lessons[nl.id]?.opened : false;
  // next best step (one primary action)
  let step: { title: string; why: string; cta: string; run: () => void };
  const behind = plan.feasibility === 'tight' || plan.feasibility === 'no-go';
  if (q.reviews.length >= (behind && nl ? 25 : 5)) step = { title: `Review ${Math.min(40, q.reviews.length)} cards that are due`, why: 'Reviewing right before you forget is what makes facts stick. It takes about ' + Math.max(2, Math.round(Math.min(40, q.reviews.length) * 0.35)) + ' minutes.', cta: 'Start review', run: () => startSession({ title: 'Due review', ids: reviewIds, mode: 'review', back: { name: 'today' }, shuffleOptions: true }) };
  else if (q.fixes.length) { const r = q.fixes[0]; step = { title: `Fix: ${C.concepts[r.concept].title}`, why: `${CAUSE_LABEL[r.cause]}. ${CAUSE_HELP[r.cause]}`, cta: 'Start fix drill', run: () => fixDrill(r.concept, r.lesson) }; }
  else if (nl) step = { title: `${nlStarted ? 'Continue' : 'Start'} ${nl.id}: ${nl.title}`, why: `About ${nl.minutes} minutes. Learn it, try the interactive parts, then take the practice test.`, cta: nlStarted ? 'Continue lesson' : 'Start lesson', run: () => go('lesson', nl.id) };
  else step = { title: 'Take a full mock test', why: 'All lessons are done. Mock tests use the real format and show how ready you are.', cta: 'Open mock tests', run: () => go('practice') };
  const weekStart = addDays(dayKey(t), -((new Date(t).getDay() + 7) % 7));
  return (
    <div class="page">
      <header class="stack" style={{ gap: '6px' }}>
        <span class="eyebrow">{new Date(t).toLocaleDateString(undefined, { weekday: 'long', month: 'long', day: 'numeric' })}</span>
        <h1>{daysLeft !== null && daysLeft >= 0 ? (daysLeft === 0 ? 'Test day. You have prepared for this.' : `${daysLeft} day${daysLeft === 1 ? '' : 's'} to your ${s.profile.tests.length > 1 ? 'tests' : 'test'}`) : 'Your study plan for today'}</h1>
      </header>
      <section class="card stack" aria-label="Next best step" style={{ borderColor: 'var(--accent)', borderWidth: '2px' }}>
        <span class="eyebrow" style={{ color: 'var(--accent)' }}>Next best step</span>
        <h2>{step.title}</h2>
        <p class="muted">{step.why}</p>
        <button class="btn primary" onClick={step.run}>{step.cta}</button>
      </section>
      <section class="card stack" aria-label="This week">
        <div class="spread"><h3>This week</h3><span class="small muted num">Today: {q.pct}% of {q.plannedMin} min planned (budget {S().profile.minutesPerDay} min)</span></div>
        <Calendar start={weekStart} days={7} examDay={exam} nowT={t} />
        <p class="small muted">Each box fills as you study (5% steps). A tick means that day's plan is done. Rest days never break anything.</p>
      </section>
      <div class="grid2">
        {s.profile.tests.filter(isWritten).map((tid) => <ReadinessCard test={tid} />)}
      </div>
      <section class="card stack" aria-label="Today's list">
        <h3>Today's list</h3>
        <div class="list">
          <button class="li" onClick={() => reviewIds.length ? startSession({ title: 'Due review', ids: reviewIds, mode: 'review', back: { name: 'today' }, shuffleOptions: true }) : go('practice')}>
            <span class="chip" style={{ background: 'var(--accent-soft)', color: 'var(--accent)' }}>{q.reviews.length}</span>
            <span><span class="li-title">Cards due for review</span><br /><span class="small muted">{q.reviews.length ? 'Spaced review of what you studied' : 'Nothing due. New cards appear as you study lessons.'}</span></span><span aria-hidden="true">›</span>
          </button>
          <button class="li" onClick={() => go('notebook')}>
            <span class="chip" style={{ background: 'var(--amber-soft)', color: 'var(--amber-ink)' }}>{q.fixes.length}</span>
            <span><span class="li-title">Mistakes to fix</span><br /><span class="small muted">{q.fixes.length ? `Top: ${C.concepts[q.fixes[0].concept].title}` : 'No open mistakes.'}</span></span><span aria-hidden="true">›</span>
          </button>
          {nl && <button class="li" onClick={() => go('lesson', nl.id)}><Shield id={nl.id} /><span><span class="li-title">{nl.title}</span><br /><span class="small muted">{lessonsLeft} lesson{lessonsLeft === 1 ? '' : 's'} left · ~{nl.minutes} min</span></span><span aria-hidden="true">›</span></button>}
        </div>
      </section>
      <section class="card info stack" aria-label="Plan">
        <strong>{plan.feasibility === 'no-date' ? 'No test date yet' : plan.feasibility === 'go' ? 'Plan: on track' : plan.feasibility === 'tight' ? 'Plan: tight' : 'Plan: not enough time'}</strong>
        <p class="small">{plan.message}</p>
        <button class="linkbtn small" style={{ alignSelf: 'flex-start' }} onClick={() => go('progress')}>See the full schedule</button>
      </section>
    </div>
  );
}

export function fixDrill(concept: string, lesson: string) {
  // least recently seen first, so the drill asks the fact in wordings the learner has not just answered
  const last = new Map<string, number>(); for (const a of S().attempts) last.set(a.id, a.t);
  const ids = [
    ...Object.values(C.items).filter((i) => !i.heldOut && i.concepts.includes(concept)).map((i) => i.id),
    ...Object.values(C.numbers).filter((n) => n.concepts.includes(concept)).map((n) => n.id),
  ].map((id, k) => ({ id, k, t: last.get(id) ?? 0 })).sort((a, b) => a.t - b.t || a.k - b.k).slice(0, 8).map((x) => x.id);
  startSession({ title: `Fix drill: ${C.concepts[concept].title}`, ids, mode: 'fix', lesson, back: { name: 'notebook' }, shuffleOptions: true });
}

export function ReadinessCard({ test }: { test: TestId }) {
  const s = S();
  const r = readiness(s, C, test, examAt(test), 1500);
  const f = TEST_FORMAT[test];
  const label = r.band === 'likely' ? 'Likely to pass' : r.band === 'borderline' ? 'Borderline' : 'Not yet';
  return (
    <section class="card stack" aria-label={`${f.name} readiness`}>
      <span class="eyebrow">{f.name} · {f.n} questions, pass {f.pass}</span>
      <div class="spread"><span class={`band ${r.band}`} style={{ fontSize: '1.3rem', fontFamily: 'var(--display)' }}>{!S().attempts.some((a) => C.items[a.id]?.test === test) ? 'Not started' : label}</span>{S().attempts.some((a) => C.items[a.id]?.test === test) && <span class="num small muted">est. {pctRange(r.low, r.high)} chance to pass</span>}</div>
      <Bar p={r.studiedShare} label="Share of test topics studied" />
      <p class="small muted">{r.studiedShare < 0.1 ? 'Study a few lessons and the app will estimate your chance of passing. ' : ''}Based on {Math.round(r.studiedShare * 100)}% of the {C.lessons.filter((l) => l.test === test).reduce((a, l) => a + l.itemIds.length, 0)} practice questions studied and {S().mocks.filter((m) => m.test === test).length} mock test{S().mocks.filter((m) => m.test === test).length === 1 ? '' : 's'}.{r.strongReady ? ' Strong ready: last two mock tests at 90%+.' : ' This is an estimate; mock tests make it more accurate.'}</p>
    </section>
  );
}
/** Always a range (the estimate is uncertain): at least 1 point wide, clamped to 0–100. */
function pctRange(a: number, b: number) { let x = Math.round(a * 100), y = Math.round(b * 100); if (y <= x) { if (x >= 100) x = 99; y = x + 1; } return `${x}–${y}%`; }
function examAt(test: TestId) {
  const d = S().profile.examDates[test];
  if (!d) return now();
  const [y, m, dd] = d.split('-').map(Number);
  return new Date(y, m - 1, dd, 9).getTime();
}

// ------------------------------------------------------------------ Path
export function PathScreen() {
  const s = S();
  const mine = TEST_ORDER.filter((t) => s.profile.tests.includes(t));
  const other = TEST_ORDER.filter((t) => !s.profile.tests.includes(t) && C.lessons.some((l) => l.test === t));
  return (
    <div class="page">
      <header class="stack" style={{ gap: '6px' }}><span class="eyebrow">Your route</span><h1>Lessons</h1>
        <p class="muted">Go in order, top to bottom: General Knowledge first, then each test your license needs. Each lesson follows the handbook pages it names.</p></header>
      {[...mine, ...other].map((g, gi) => {
        const inPath = s.profile.tests.includes(g);
        const ls = C.lessons.filter((l) => l.test === g);
        const done = ls.filter((l) => lessonDone(s, l.id)).length;
        return (
          <>{gi === mine.length && <h2 style={{ marginTop: '8px' }}>Other tests you can study</h2>}<section class="card stack" aria-label={TNAME(g)}>
            <div class="spread"><h2>{TNAME(g)}</h2><span class="small muted num">{done}/{ls.length} done</span></div>
            <p class="small muted">{TESTS[g].blurb} {TESTS[g].n ? `${TESTS[g].n} questions, pass ${TESTS[g].pass}.` : 'Driving tests; no written test.'} Handbook section {TESTS[g].sections}.</p>
            {!inPath && <p class="small card warn">Not in your plan. You can still study it, or add it in Settings.</p>}
            <div class="list">
              {ls.map((l) => {
                const lp = s.lessons[l.id];
                const m = lessonMastery(l.id);
                return (
                  <button class="li" onClick={() => go('lesson', l.id)}>
                    <Shield id={l.id} done={!!lp?.completed} />
                    <span><span class="li-title">{l.title}</span><br /><span class="small muted">{lp?.completed ? `Done · best ${Math.round((lp.practiceBest ?? 0) * 100)}%` : lp?.opened ? `In progress${lp.practiceBest !== undefined ? ` · best ${Math.round(lp.practiceBest * 100)}%` : ''}` : `~${l.minutes} min · ${l.weight === 'High' ? 'high' : 'medium'} exam weight`}</span></span>
                    <Ring p={m} label={`${l.id} mastery ${Math.round(m * 100)}%`} />
                  </button>
                );
              })}
            </div>
          </section></>
        );
      })}
    </div>
  );
}

// ------------------------------------------------------------------ Practice hub
export function PracticeScreen() {
  const s = S();
  const t = now();
  const q = today(s, C, t);
  const opened = C.lessons.filter((l) => s.lessons[l.id]?.opened);
  const fresh = opened.flatMap((l) => newSurfaces(s, C, l.id)).slice(0, 15);
  const weak = Object.entries(s.bkt).filter(([, b]) => b.n > 0 && b.p < 0.5).map(([c]) => c);
  const weakIds = Object.values(C.items).filter((i) => i.origin === 'pack' && i.concepts.some((c) => weak.includes(c))).map((i) => i.id).slice(0, 15);
  return (
    <div class="page">
      <header class="stack" style={{ gap: '6px' }}><span class="eyebrow">Practice</span><h1>Test yourself</h1></header>
      <div class="grid2">
        <section class="card stack"><h3>Due review</h3><p class="small muted">{q.reviews.length} card{q.reviews.length === 1 ? '' : 's'} due. Spaced review picks what you are about to forget.</p>
          <button class="btn primary" disabled={!q.reviews.length} onClick={() => startSession({ title: 'Due review', ids: q.reviews.slice(0, 40).map((x) => x.id), mode: 'review', back: { name: 'practice' }, shuffleOptions: true })}>Start review</button></section>
        <section class="card stack"><h3>New cards</h3><p class="small muted">{fresh.length ? `${fresh.length} new cards from lessons you opened.` : 'Open a lesson to unlock its cards.'}</p>
          <button class="btn" disabled={!fresh.length} onClick={() => startSession({ title: 'New cards', ids: fresh, mode: 'cards', back: { name: 'practice' } })}>Learn new cards</button></section>
        <section class="card stack"><h3>Weak spots</h3><p class="small muted">{weakIds.length ? `${weakIds.length} questions from your weakest topics.` : 'No weak spots found yet.'}</p>
          <button class="btn" disabled={!weakIds.length} onClick={() => startSession({ title: 'Weak spots', ids: weakIds, mode: 'fix', back: { name: 'practice' }, shuffleOptions: true })}>Drill weak spots</button></section>
      </div>
      <section class="card stack" aria-label="Mock tests">
        <h2>Mock tests</h2>
        <p>Like the DMV touchscreen test: 3 choices, you see right away if you are wrong, you can skip and come back, and there is no time limit. Pass mark is 80%.</p>
        <div class="grid2">
          {[...TEST_ORDER.filter((t) => isWritten(t) && s.profile.tests.includes(t)), ...TEST_ORDER.filter((t) => isWritten(t) && !s.profile.tests.includes(t) && C.lessons.some((l) => l.test === t))].map((tid) => {
            const f = TEST_FORMAT[tid];
            const last = s.mocks.filter((m) => m.test === tid).slice(-1)[0];
            return (
              <div class="card flat stack">
                <strong>{f.name}</strong>
                <span class="small muted">{f.n} questions · pass {f.pass}{!s.profile.tests.includes(tid) ? ' · not in your plan' : ''}</span>
                {last && <span class="small">Last: <strong class="num">{last.score}/{last.total}</strong> {last.pass ? '(pass)' : '(not yet)'}</span>}
                {s.mockRun?.test === tid ? <button class="btn primary" onClick={() => go('mock', tid)}>Resume {TESTS[tid].short} mock ({s.mockRun.ids.length - s.mockRun.queue.length}/{s.mockRun.ids.length})</button>
                  : <button class="btn primary" onClick={() => go('mock', tid)}>Start {TESTS[tid].short} mock</button>}
              </div>
            );
          })}
        </div>
      </section>
      <section class="card stack" aria-label="Lesson tests">
        <h3>Lesson practice tests</h3>
        <div class="list">{C.lessons.filter((l) => s.profile.tests.includes(l.test)).map((l) => (
          <button class="li" onClick={() => startTest(l.id)}><Shield id={l.id} /><span><span class="li-title">{l.title}</span><br /><span class="small muted">{l.itemIds.length} questions{s.lessons[l.id]?.practiceBest !== undefined ? ` · best ${Math.round(s.lessons[l.id].practiceBest! * 100)}%` : ''}</span></span><span aria-hidden="true">›</span></button>
        ))}</div>
      </section>
    </div>
  );
}

// ------------------------------------------------------------------ Notebook
export function NotebookScreen() {
  const s = S();
  const t = now();
  const rows = openRows(s, C, t);
  const resolved = Object.values(s.notebook).filter((r) => r.status === 'resolved');
  // one entry per concept (a concept can have several causes); ordered by the highest-priority row
  const byConcept: { concept: string; lesson: string; rows: typeof rows }[] = [];
  for (const r of rows) { const g = byConcept.find((x) => x.concept === r.concept); if (g) g.rows.push(r); else byConcept.push({ concept: r.concept, lesson: r.lesson, rows: [r] }); }
  const top = byConcept.slice(0, 3), rest = byConcept.slice(3);
  const Entry = ({ g }: { g: (typeof byConcept)[number] }) => {
    const c = C.concepts[g.concept];
    const misses = g.rows.reduce((a, r) => a + r.misses, 0);
    const proof = Math.max(...g.rows.map((r) => new Set(r.probes.filter((p) => p.ok).map((p) => dayKey(p.t))).size));
    return (
      <div class="li" style={{ cursor: 'default', gridTemplateColumns: 'auto 1fr', alignItems: 'start' }}>
        <Shield id={g.lesson} />
        <div class="stack" style={{ gap: '6px' }}>
          <span class="li-title">{c.title}</span>
          <div class="row">{g.rows.map((r) => <span class="chip" style={{ background: 'var(--amber-soft)', color: 'var(--amber-ink)' }} title={CAUSE_HELP[r.cause]}>{CAUSE_LABEL[r.cause]}</span>)}</div>
          <span class="small muted">Missed {misses}× · {proof > 0 ? `right on ${proof} of 2 days needed` : 'get it right on 2 different days to clear it'}</span>
          <p class="small">{CAUSE_HELP[g.rows[0].cause]}</p>
          {(() => {
            // the learner's own last miss on this topic: what they chose vs the handbook answer
            const a = [...S().attempts].reverse().find((x) => !x.ok && x.chosen !== undefined && C.items[x.id] && (x.concepts ?? C.items[x.id].concepts).includes(g.concept));
            if (!a) return null;
            const it = C.items[a.id];
            return <div class="small" style={{ borderLeft: '3px solid var(--line)', paddingLeft: '10px' }}><Html tag="span" html={it.stem} /><br /><span class="muted">You chose:</span> <Html tag="span" html={it.options[a.chosen!]} /> · <span class="muted">Handbook:</span> <strong><Html tag="span" html={it.options[it.key]} /></strong></div>;
          })()}
          <div class="row">
            <button class="btn sm primary" onClick={() => fixDrill(g.concept, g.lesson)}>Fix drill</button>
            <button class="btn sm" onClick={() => go('lesson', g.lesson, g.concept)}>Re-read</button>
            <button class="btn sm ghost" onClick={() => mutate((st) => { for (const r of g.rows) { const x = st.notebook[r.key]; x.status = 'snoozed'; x.snoozeUntil = now() + 86400_000; } })}>Later</button>
          </div>
        </div>
      </div>
    );
  };
  return (
    <div class="page">
      <header class="stack" style={{ gap: '6px' }}><span class="eyebrow">Mistake list</span><h1>What to fix</h1>
        <p class="muted">Each topic you missed, with the reason you missed it. Fix the top three first. A topic clears after you get it right on two different days.</p></header>
      {!byConcept.length && <div class="card tint"><strong>No open mistakes.</strong> <span class="muted">Misses from practice and mock tests show up here with a fix drill.</span></div>}
      {top.length > 0 && <section class="card stack" aria-label="Fix these first"><h2 style={{ fontSize: '1.3rem' }}>Fix these first</h2><div class="list">{top.map((g) => <Entry g={g} />)}</div></section>}
      {rest.length > 0 && <details class="card deep"><summary>{rest.length} more topic{rest.length === 1 ? '' : 's'} to fix</summary><div class="list">{rest.map((g) => <Entry g={g} />)}</div></details>}
      {resolved.length > 0 && <p class="small muted">{resolved.length} mistake{resolved.length === 1 ? '' : 's'} fixed so far.</p>}
    </div>
  );
}

// ------------------------------------------------------------------ Progress
export function ProgressScreen() {
  const s = S();
  const t = now();
  const plan = buildPlan(s, C, t);
  const k = dayKey(t);
  const monthStart = k.slice(0, 8) + '01';
  const dim = new Date(new Date(t).getFullYear(), new Date(t).getMonth() + 1, 0).getDate();
  const mastered = Object.values(s.bkt).filter((b) => b.p >= 0.6).length;
  const total = C.counts.concepts;
  return (
    <div class="page">
      <header class="stack" style={{ gap: '6px' }}><span class="eyebrow">Progress</span><h1>How you are doing</h1></header>
      <div class="grid2">
        <div class="card stat"><span class="eyebrow">Topics proficient</span><span class="v">{mastered}<span class="muted" style={{ fontSize: '1rem' }}>/{total}</span></span></div>
        <div class="card stat"><span class="eyebrow">Days active, last 30</span><span class="v">{activeDaysLast30(s, t)}</span></div>
        <div class="card stat"><span class="eyebrow">Questions answered</span><span class="v">{s.attempts.length}</span></div>
      </div>
      <div class="grid2">{s.profile.tests.filter(isWritten).map((tid) => <ReadinessCard test={tid} />)}</div>
      <section class="card stack" aria-label="Month">
        <h3>{new Date(t).toLocaleDateString(undefined, { month: 'long', year: 'numeric' })}</h3>
        <Calendar start={monthStart} days={dim} examDay={plan.examDay} nowT={t} />
      </section>
      <section class="card stack" aria-label="Schedule">
        <h3>Schedule</h3>
        <p class="small">{plan.message}</p>
        {!inArtifact() && plan.days.length > 0 && <button class="btn sm" style={{ alignSelf: 'flex-start' }} onClick={() => downloadIcs(planToIcs(plan, C, now()))}>Add plan to my calendar (.ics)</button>}
        <div class="list">
          {plan.days.filter((d) => daysBetween(k, d.day) >= 0).slice(0, 21).map((d) => (
            <div class="li" style={{ cursor: 'default', gridTemplateColumns: '110px 1fr' }}>
              <span class="small num"><strong>{fmtDay(d.day)}</strong></span>
              <span class="small">{d.day === plan.examDay ? 'Test day: light review only.' : d.rest ? 'Rest day' : [d.lessons.length ? `Lessons ${d.lessons.join(', ')}` : '', d.reviewMin ? `review ~${d.reviewMin} min` : '', d.mock ? `${d.mock} mock test` : ''].filter(Boolean).join(' · ')}</span>
            </div>
          ))}
        </div>
      </section>
      <section class="card stack" aria-label="Mastery by lesson">
        <h3>Mastery by lesson</h3>
        {C.lessons.filter((l) => s.profile.tests.includes(l.test)).map((l) => (
          <div class="stack" style={{ gap: '4px' }}><div class="spread small"><span><strong>{l.id}</strong> {l.title}</span><span class="num">{Math.round(lessonMastery(l.id) * 100)}%</span></div><Bar p={lessonMastery(l.id)} label={`${l.id} mastery`} /></div>
        ))}
      </section>
      <section class="card stack" aria-label="Stamps">
        <div class="spread"><h3>Topic stamps</h3><span class="small num"><strong>{WIDGETS.filter((w) => w.meta.stamp && s.stamps[w.meta.stamp.id]).length}</strong> of {WIDGETS.filter((w) => w.meta.stamp).length} earned</span></div>
        <p class="small muted">Earned by a clean run of a hands-on challenge. Retry as often as you like; mistakes in Explore or earlier tries never count against you. Stamps never expire.</p>
        <div class="grid2">{WIDGETS.filter((w) => w.meta.stamp).map((w) => { const got = s.stamps[w.meta.stamp!.id]; return (
          <div class={`stamp ${got ? '' : 'locked'}`}><span class="seal">{w.meta.lesson}</span><span class="small"><strong>{w.meta.stamp!.name}</strong><br />{got ? <span class="ok-ink">✓ Earned {new Date(got).toLocaleDateString()}</span> : <span class="muted">Not yet: {w.meta.stamp!.rule}</span>}</span></div>
        ); })}</div>
      </section>
      {s.mocks.length > 0 && (
        <section class="card stack" aria-label="Mock history"><h3>Mock tests</h3>
          <div class="list">{s.mocks.slice().reverse().map((m) => <div class="li" style={{ cursor: 'default', gridTemplateColumns: 'auto 1fr auto' }}><Shield id={m.test} /><span class="small">{new Date(m.t).toLocaleDateString()} · {Math.round(m.unseenShare * 100)}% new questions{m.predicted !== undefined ? ` · app predicted ≥${Math.round(m.predicted * 100)}% pass chance, you ${m.pass ? 'passed' : 'did not pass'}` : ''}</span><strong class="num">{m.score}/{m.total}</strong></div>)}</div>
        </section>
      )}
    </div>
  );
}

// ------------------------------------------------------------------ More
export function MoreScreen() {
  const items: [Parameters<typeof go>[0], string, string][] = [
    ['progress', 'Progress', 'Readiness, calendar, schedule, stamps'],
    ['guide', 'Guide', 'CA steps from permit to license, test-day rules, handbook answers'],
    ['glossary', 'Glossary', 'Plain-English meaning of every term'],
    ['settings', 'Settings', 'Your class and test date, save or move your progress, display'],
  ];
  return (
    <div class="page">
      <h1>More</h1>
      <div class="card"><div class="list">{items.map(([r, t, d]) => <button class="li" onClick={() => go(r)}><span /><span><span class="li-title">{t}</span><br /><span class="small muted">{d}</span></span><span aria-hidden="true">›</span></button>)}</div></div>
      <p class="small muted">Not affiliated with the California DMV, CHP or FMCSA. Built from the California Commercial Driver Handbook (DL 650). When anything here differs from the current handbook, the handbook wins.</p>
    </div>
  );
}

export { Html };
