import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { C, finishLessonTest, go, mutate, now, session, S } from '../app';
import type { SessionSpec } from '../app';
import { recordAnswer, surface, examCapDays } from '../engine/learner';
import { CAUSE_HELP, CAUSE_LABEL } from '../engine/model';
import type { Cause } from '../engine/model';
import { Back, Html, Pages, Shield } from './bits';
import { mulberry32 } from '../engine/readiness';

type Result = { id: string; ok: boolean; cause?: Cause };

export function SessionScreen() {
  const spec = session.value;
  if (!spec) return <div class="page"><p>Nothing to practice right now.</p><button class="btn" onClick={() => go('today')}>Go to Today</button></div>;
  return <Runner spec={spec} key={spec.title + spec.ids.join()} />;
}

function Runner({ spec }: { spec: SessionSpec }) {
  const saved = S().sessionRun;
  const resume = saved && saved.title === spec.title && saved.ids.join() === spec.ids.join() ? saved : null;
  const [i, setI] = useState(resume ? resume.i : 0);
  const [results, setResults] = useState<Result[]>(resume ? resume.results : []);
  const done = i >= spec.ids.length;
  const back = () => { mutate((st) => { st.sessionRun = undefined; }); const b = spec.back; go(b.name, b.param, b.sub); };
  useEffect(() => {
    if (!done) return;
    if (spec.testLesson) finishLessonTest(spec.testLesson, results.filter((r) => r.ok).length, results.length);
    mutate((st) => { st.sessionRun = undefined; });
  }, [done]);
  if (done) return <Summary spec={spec} results={results} onBack={back} />;
  const id = spec.ids[i];
  const s = surface(C, id)!;
  const next = (r: Result) => {
    const nr = [...results, r];
    setResults(nr); setI(i + 1);
    mutate((st) => { if (st.sessionRun) { st.sessionRun.i = i + 1; st.sessionRun.results = nr; } });
  };
  return (
    <div class="page">
      <div class="spread">
        <Back onClick={back} label="Leave" />
        <span class="muted small num">{i + 1} of {spec.ids.length}</span>
      </div>
      <div class="stack" style={{ gap: '8px' }}>
        <div class="row"><Shield id={s.lesson} /><h2 style={{ fontSize: '1.2rem' }}>{spec.title}</h2></div>
        <div class="progressbar" role="progressbar" aria-valuenow={i} aria-valuemin={0} aria-valuemax={spec.ids.length} aria-label="Session progress"><span style={{ width: `${(i / spec.ids.length) * 100}%` }} /></div>
      </div>
      {s.kind === 'item' ? <ItemQ key={id} id={id} spec={spec} onNext={next} /> : <CardQ key={id} id={id} onNext={next} />}
    </div>
  );
}

const LETTERS = ['a', 'b', 'c'];
const TAG_NOTE: Record<string, string> = {
  neighbor_number: 'A nearby number from the handbook — easy to mix up.',
  unit_match: 'A real handbook number from a different rule.',
  alt_source: 'What other sources say — not the CA handbook answer.',
  trap_named: 'This is the exam trap for this rule.',
};

function ItemQ({ id, spec, onNext }: { id: string; spec: SessionSpec; onNext: (r: Result) => void }) {
  const it = C.items[id];
  const order = useMemo(() => {
    const base = it.options.map((_, k) => k);
    if (!spec.shuffleOptions || it.polarity === 'tf') return base;
    const r = mulberry32(id.split('').reduce((a, ch) => a + ch.charCodeAt(0), 0) + S().attempts.length);
    return base.sort(() => r() - 0.5);
  }, [id]);
  const [chosen, setChosen] = useState<number | null>(null);
  const [guessed, setGuessed] = useState(false);
  const [cause, setCause] = useState<Cause | undefined>();
  const [showAll, setShowAll] = useState(false);
  const t0 = useRef(now());
  const answered = chosen !== null;
  const ok = chosen === it.key;
  const answer = (k: number) => {
    if (answered) return;
    setChosen(k);
    const rt = now() - t0.current;
    mutate((st) => {
      const r = recordAnswer(st, C, {
        id, ev: it.polarity === 'tf' ? 'tf' : 'mcq', ok: k === it.key, chosen: k, guessed, rt, now: now(),
        examCapDays: examCapDays(st, it.test, now()), charCount: it.stemText.length + it.optionsText.join('').length,
      });
      setCause(r.cause);
    });
  };
  return (
    <div class="stack-lg">
      <div class="card stack">
        <Html class="q-stem" html={it.stem} />
        <div class="opts" role="group" aria-label="Answer choices">
          {order.map((k, pos) => {
            const cls = answered ? (k === it.key ? 'right' : k === chosen ? 'wrong' : '') : '';
            const tag = it.tags[k];
            return (
              <button class={`opt ${cls}`} disabled={answered} onClick={() => answer(k)} aria-label={`${it.polarity === 'tf' ? '' : LETTERS[pos] + ') '}${it.optionsText[k]}${answered && k === it.key ? ' — correct answer' : ''}${answered && k === chosen && !ok ? ' — your answer' : ''}`}>
                <span class="letter" aria-hidden="true">{it.polarity === 'tf' ? (k === 0 ? 'T' : 'F') : LETTERS[pos]}</span>
                <Html tag="span" html={it.options[k]} />
                {answered && k !== it.key && (k === chosen || showAll) && (it.notes?.[k] ? <Html tag="span" class="note" html={it.notes[k]!} /> : k === chosen && tag && TAG_NOTE[tag] ? <span class="note">{TAG_NOTE[tag]}</span> : null)}
              </button>
            );
          })}
        </div>
        {!answered && (
          <label class="toggle"><input type="checkbox" checked={guessed} onChange={(e) => setGuessed((e.target as HTMLInputElement).checked)} /> I'm guessing on this one</label>
        )}
      </div>
      {answered && (
        <div class={`feedback ${ok && !guessed ? 'good' : 'bad'}`} role="status" aria-live="polite">
          <div class="verdict">{ok ? (guessed ? 'Right — but it was a guess' : 'Correct') : 'Not quite'}</div>
          <Html class="prose" html={it.explanation} />
          {it.notes && it.polarity !== 'tf' && !showAll && <button class="linkbtn small" style={{ alignSelf: 'flex-start' }} onClick={() => setShowAll(true)}>Why the other choices are wrong</button>}
          <div class="row small"><Pages pages={it.pages} />{it.concepts[0] && <button class="linkbtn" onClick={() => go('lesson', it.lesson, it.concepts[0])}>Re-read this in {it.lesson}</button>}</div>
          {cause && (
            <div class="card flat" style={{ padding: '10px 12px' }}>
              <div class="row"><span class="diamond" aria-hidden="true" /><strong>Added to your mistake list: {CAUSE_LABEL[cause]}</strong></div>
              <p class="small muted">{CAUSE_HELP[cause]}</p>
            </div>
          )}
          <button class="btn primary block" onClick={() => onNext({ id, ok: ok && !guessed, cause })} autoFocus>Continue</button>
        </div>
      )}
    </div>
  );
}

function CardQ({ id, onNext }: { id: string; onNext: (r: Result) => void }) {
  const s = surface(C, id)!;
  const [shown, setShown] = useState(false);
  const t0 = useRef(now());
  let prompt = '', answer = '', pages: string[] = [], kind = '';
  if (s.kind === 'number') { const n = C.numbers[id]; prompt = `<strong>${n.item}</strong>`; answer = n.valueHtml; pages = n.pages; kind = 'Number to know'; }
  if (s.kind === 'flash') { const f = C.flash[id]; prompt = f.q; answer = f.a; kind = 'Flashcard'; }
  if (s.kind === 'tyk') { const t = C.tyk[id]; prompt = t.q; answer = t.aHtml; pages = t.pages; kind = 'Handbook review question'; }
  const grade = (ok: boolean) => {
    mutate((st) => { recordAnswer(st, C, { id, ev: 'self', ok, rt: now() - t0.current, now: now(), examCapDays: examCapDays(st, s.test, now()) }); });
    onNext({ id, ok });
  };
  return (
    <div class="card flip">
      <span class="eyebrow">{kind}</span>
      <Html class="q-stem" html={prompt} />
      {!shown ? (
        <button class="btn primary" onClick={() => setShown(true)} autoFocus>Show answer</button>
      ) : (
        <div class="answer-reveal stack">
          <Html class="prose" html={answer} />
          <Pages pages={pages} />
          <p class="small muted">Did you know it before you looked?</p>
          <div class="grid2" style={{ gridTemplateColumns: '1fr 1fr' }}>
            <button class="btn" onClick={() => grade(false)}>Not yet — show again soon</button>
            <button class="btn primary" onClick={() => grade(true)} autoFocus>I knew it</button>
          </div>
        </div>
      )}
    </div>
  );
}

function Summary({ spec, results, onBack }: { spec: SessionSpec; results: Result[]; onBack: () => void }) {
  const right = results.filter((r) => r.ok).length;
  const pct = results.length ? Math.round((right / results.length) * 100) : 0;
  const causes = results.filter((r) => r.cause).reduce((m, r) => { m[r.cause!] = (m[r.cause!] ?? 0) + 1; return m; }, {} as Record<string, number>);
  const isTest = spec.mode === 'practice';
  return (
    <div class="page">
      <div class="card stack">
        <span class="eyebrow">{spec.title}</span>
        <h1 class="num">{right} / {results.length} <span class="muted" style={{ fontSize: '1.2rem' }}>({pct}%)</span></h1>
        {isTest && <p>{pct >= 90 ? 'Lesson goal met (90%+). The next lesson is unlocked.' : pct >= 80 ? 'That would pass the real test (80%), but aim for 90% so new wording does not catch you.' : 'Below the 80% pass mark. Check the mistakes below, re-read those parts, then try again.'}</p>}
        {Object.keys(causes).length > 0 && (
          <div class="stack" style={{ gap: '6px' }}>
            <strong>Why you missed</strong>
            {Object.entries(causes).map(([c, n]) => <div class="row small"><span class="diamond" aria-hidden="true" />{CAUSE_LABEL[c as Cause]}: {n}</div>)}
            <p class="small muted">These are in your mistake list, with a fix drill for each.</p>
          </div>
        )}
        <div class="row">
          <button class="btn primary" onClick={onBack}>Done</button>
          {Object.keys(causes).length > 0 && <button class="btn" onClick={() => go('notebook')}>Open mistake list</button>}
        </div>
      </div>
      {results.some((r) => !r.ok) && (
        <div class="card stack">
          <h3>Review your misses</h3>
          <div class="list">
            {results.filter((r) => !r.ok).map((r) => {
              const it = C.items[r.id];
              if (!it) { const sf = surface(C, r.id)!; return <div class="li" style={{ cursor: 'default' }}><Shield id={sf.lesson} /><span>{sf.kind === 'number' ? C.numbers[r.id].item : sf.kind === 'flash' ? <Html tag="span" html={C.flash[r.id].q} /> : <Html tag="span" html={C.tyk[r.id].q} />}</span><span /></div>; }
              return (
                <div class="li" style={{ cursor: 'default', gridTemplateColumns: 'auto 1fr' }}>
                  <Shield id={it.lesson} />
                  <div class="stack" style={{ gap: '4px' }}>
                    <Html html={it.stem} />
                    <div class="small"><strong>Answer:</strong> <Html tag="span" html={it.options[it.key]} /></div>
                    <Html class="small muted" html={it.explanation} />
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
}
