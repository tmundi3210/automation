import { TESTS } from '../content/tests';
import { useMemo, useRef, useState } from 'preact/hooks';
import { C, go, mutate, now, S } from '../app';
import type { TestId } from '../content/types';
import { buildMock } from '../engine/plan';
import { readiness, TEST_FORMAT } from '../engine/readiness';
import { recordAnswer, examCapDays } from '../engine/learner';
import { Back, Html, Pages, Shield } from './bits';

/** DMV-style knowledge test rehearsal: 3 choices, immediate right/wrong, skip returns at the end, no timer. */
export function MockScreen({ test }: { test: TestId }) {
  const f = TEST_FORMAT[test];
  const saved = useMemo(() => { const r = S().mockRun; return r && r.test === test && r.queue.length ? r : null; }, [test]);
  const seed = useMemo(() => Math.floor(now() / 1000), []);
  const ids = useMemo(() => saved ? saved.ids : buildMock(S(), C, test, seed), [test]);
  const seenBefore = useMemo(() => { if (saved) return saved.seenBefore; const seen = new Set(S().attempts.map((a) => a.id)); return ids.filter((i) => seen.has(i)).length; }, [ids]);
  const predicted = useMemo(() => saved ? saved.predicted : readiness(S(), C, test, now(), 800).low, [test]);
  const [queue, setQueue] = useState<string[]>(saved ? saved.queue : ids);
  const [answers, setAnswers] = useState<Record<string, number>>(saved ? saved.answers : {});
  // an answer is saved when chosen; after a reload the question at the head shows as already answered (never recorded twice)
  const [chosen, setChosen] = useState<number | null>(saved && saved.queue[0] in saved.answers ? saved.answers[saved.queue[0]] : null);
  const [started, setStarted] = useState(!!saved);
  const persist = (q: string[], a: Record<string, number>) => mutate((st) => { st.mockRun = q.length ? { test, ids, queue: q, answers: a, seenBefore, predicted, started: st.mockRun?.started ?? now() } : undefined; });
  const t0 = useRef(now());
  const done = queue.length === 0;
  const answeredIds = Object.keys(answers);
  const wrong = answeredIds.filter((id) => answers[id] !== C.items[id].key).length;
  const right = answeredIds.length - wrong;
  const maxWrong = f.n - f.pass;

  if (!started) return (
    <div class="page">
      <Back onClick={() => go('practice')} />
      <div class="card stack">
        <span class="eyebrow">Mock test</span>
        <h1>{f.name}</h1>
        <ul class="core">
          <li><strong>{f.n} questions</strong>, 3 choices each. You pass with <strong>{f.pass}</strong> right (80%), so you can miss up to {maxWrong}.</li>
          <li>Like the DMV touchscreen, you see right away when an answer is wrong.</li>
          <li>Not sure? <strong>Skip</strong>. Skipped questions come back at the end.</li>
          <li>No time limit. On the real test, phones, notes or help from anyone mean an automatic fail.</li>
        </ul>
        {(() => { const ls = C.lessons.filter((l) => l.test === test); const studied = ls.filter((l) => S().lessons[l.id]?.opened).length; return studied < ls.length / 2 ? <p class="card warn small">You have opened {studied} of the {ls.length} {TESTS[test].name} lessons, so expect a low score. That's fine — a mock now shows where to start, and every miss goes to your mistake list.</p> : null; })()}
        <p class="small muted">{ids.length - seenBefore} of {ids.length} questions are ones you have not answered yet (a few may test a fact you practiced in other words). Questions come from every lesson in proportion to the handbook.</p>
        <button class="btn primary" onClick={() => { setStarted(true); t0.current = now(); persist(queue, answers); }}>Begin</button>
      </div>
    </div>
  );

  if (done) {
    const pass = right >= f.pass;
    return (
      <div class="page">
        <div class={`card stack`} style={{ borderColor: pass ? 'var(--ok)' : 'var(--red)', borderWidth: '2px' }}>
          <span class="eyebrow">{f.name} mock</span>
          <h1 class="num">{right} / {f.n} — {pass ? 'Pass' : 'Not yet'}</h1>
          <p>{pass ? (right / f.n >= 0.9 ? 'Strong result. Two mocks in a row at 90% or more is the best sign you are ready.' : 'You passed. Aim for 90% so new wording on test day does not catch you.') : `You needed ${f.pass}. Your misses are now in your mistake list with fix drills.`}</p>
          <p class="small muted">Before this mock, the app estimated at least a {Math.round(predicted * 100)}% chance of passing. Every mock makes the estimate more accurate.</p>
          <div class="row"><button class="btn primary" onClick={() => go('notebook')}>Fix my mistakes</button><button class="btn" onClick={() => go('practice')}>Back to practice</button></div>
        </div>
        {wrong > 0 && (
          <div class="card stack"><h3>Questions you missed</h3>
            <div class="list">{answeredIds.filter((id) => answers[id] !== C.items[id].key).map((id) => { const it = C.items[id]; return (
              <div class="li" style={{ cursor: 'default', gridTemplateColumns: 'auto 1fr' }}><Shield id={it.lesson} /><div class="stack" style={{ gap: '4px' }}><Html html={it.stem} /><div class="small"><strong>Answer:</strong> <Html tag="span" html={it.options[it.key]} /> <Pages pages={it.pages} /></div><Html class="small muted" html={it.explanation} /></div></div>
            ); })}</div>
          </div>
        )}
      </div>
    );
  }

  const id = queue[0];
  const it = C.items[id];
  const answered = chosen !== null;
  const answer = (k: number) => {
    if (answered) return;
    setChosen(k);
    const rt = now() - t0.current;
    const nextAnswers = { ...answers, [id]: k };
    setAnswers(nextAnswers);
    mutate((st) => { recordAnswer(st, C, { id, ev: 'mock', ok: k === it.key, chosen: k, rt, now: now(), examCapDays: examCapDays(st, test, now()) }); });
    persist(queue, nextAnswers);
  };
  const next = () => {
    const nextAnswers = answers;
    setChosen(null);
    t0.current = now();
    const rest = queue.slice(1);
    setQueue(rest);
    persist(rest, nextAnswers);
    if (!rest.length) {
      const items = Object.keys(nextAnswers).map((q) => ({ id: q, ok: nextAnswers[q] === C.items[q].key }));
      const score = items.filter((x) => x.ok).length;
      mutate((st) => { st.mocks.push({ t: now(), test, score, total: f.n, pass: score >= f.pass, unseenShare: (ids.length - seenBefore) / ids.length, predicted, items }); });
    }
  };
  const skip = () => { const q = [...queue.slice(1), id]; setQueue(q); persist(q, answers); t0.current = now(); };
  const remaining = queue.length;
  // the counter includes the answer on screen, so it updates as soon as a wrong answer shows
  const wrongNow = wrong;   // answers are saved on choice, so the count already includes the question on screen
  return (
    <div class="page">
      <div class="spread">
        <h1 class="eyebrow" style={{ margin: 0 }}>{f.name} mock</h1>
        <span class="small num"><strong>{f.n - remaining + 1}</strong> of {f.n} · <span style={{ color: wrongNow > maxWrong ? 'var(--red)' : 'var(--ink-2)', fontWeight: 700 }}>{wrongNow} wrong</span> (you can miss {maxWrong})</span>
      </div>
      {wrongNow > maxWrong && <div class="card warn small" role="status"><strong>You can no longer pass this mock</strong> — more than {maxWrong} wrong. On the real test that would be a fail. Keep going for practice; every miss goes to your mistake list.</div>}
      <div class="progressbar" aria-hidden="true"><span style={{ width: `${((f.n - remaining) / f.n) * 100}%` }} /></div>
      <div class="card stack">
        <Html class="q-stem" html={it.stem} />
        <div class="opts">
          {it.options.map((o, k) => (
            <button class={`opt ${answered ? (k === it.key ? 'right' : k === chosen ? 'wrong' : '') : ''}`} disabled={answered} onClick={() => answer(k)}>
              <span class="letter" aria-hidden="true">{'abc'[k]}</span><Html tag="span" html={o} />
            </button>
          ))}
        </div>
      </div>
      {answered ? (
        <div class={`feedback ${chosen === it.key ? 'good' : 'bad'}`} role="status" aria-live="polite">
          <div class="verdict">{chosen === it.key ? 'Correct' : 'Incorrect'}</div>
          {chosen !== it.key && <><Html class="small" html={it.explanation} /><div><Pages pages={it.pages} /></div></>}
          <button class="btn primary block" onClick={next} autoFocus>{remaining === 1 ? 'See my result' : 'Next question'}</button>
        </div>
      ) : (
        <div class="row"><button class="btn" onClick={skip} disabled={remaining === 1 || answered}>Skip for now</button></div>
      )}
    </div>
  );
}
