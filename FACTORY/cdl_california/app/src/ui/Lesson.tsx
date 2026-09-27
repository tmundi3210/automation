import { useEffect, useState } from 'preact/hooks';
import { C, go, lessonById, mutate, now, S, startSession, say } from '../app';
import { recordCheck } from '../engine/learner';
import { band } from '../engine/srs';
import { Back, Html, Pages, Ring, Shield, YouTube } from './bits';
import { widgetsForLesson, widgetById, WIDGETS } from '../widgets/registry';
import type { Concept } from '../content/types';

type Tab = 'learn' | 'numbers' | 'traps' | 'cards' | 'test' | 'recap';

export function LessonScreen({ id, focus }: { id: string; focus?: string }) {
  const L = lessonById(id);
  const s = S();
  const [tab, setTab] = useState<Tab>('learn');
  useEffect(() => { mutate((st) => { const lp = st.lessons[id] ?? { conceptsSeen: {} }; lp.opened = lp.opened ?? now(); st.lessons[id] = lp; }); }, [id]);
  useEffect(() => {
    if (focus) { setTab('learn'); setTimeout(() => { try { document.getElementById(`c-${focus}`)?.scrollIntoView({ block: 'start' }); } catch { /* */ } }, 60); }
  }, [focus]);
  if (!L) return <div class="page"><p>Lesson not found.</p></div>;
  const lp = s.lessons[id];
  const mastery = L.conceptIds.map((c) => s.bkt[c]?.p ?? 0);
  const avg = mastery.reduce((a, b) => a + b, 0) / mastery.length;
  const idx = C.lessons.findIndex((l) => l.id === id);
  const nextL = C.lessons[idx + 1];
  const tabs: [Tab, string][] = [['learn', 'Learn'], ['numbers', `Numbers (${L.numberIds.length})`], ['traps', `Traps (${L.trapIds.length})`], ['cards', 'Cards'], ['test', 'Practice test'], ['recap', 'Recap']];
  return (
    <div class="page">
      <Back onClick={() => go('path')} label="All lessons" />
      <header class="stack" style={{ gap: '10px' }}>
        <div class="row"><Shield id={L.id} done={!!lp?.completed} /><span class="eyebrow">{L.test === 'GK' ? 'General Knowledge' : 'Combination Vehicles'} · {L.weight === 'High' ? 'High exam weight' : 'Medium exam weight'} · ~{L.minutes} min</span></div>
        <h1>{L.title}</h1>
        <div class="row small muted"><span>Handbook {L.handbook}</span><Pages pages={[L.pages[0], L.pages[1]]} /></div>
        <div class="row">
          <Ring p={avg} label={`Lesson mastery ${Math.round(avg * 100)}%`} />
          <div class="small">
            <div><strong>Practice test best:</strong> {lp?.practiceBest !== undefined ? `${Math.round(lp.practiceBest * 100)}%` : 'not taken yet'}</div>
            <div class="muted">Goal: 90% on the practice test (the real pass mark is 80%).</div>
          </div>
        </div>
      </header>
      <div class="tabs" role="tablist" aria-label="Lesson sections">
        {tabs.map(([k, label]) => <button role="tab" aria-selected={tab === k} onClick={() => setTab(k)}>{label}</button>)}
      </div>
      {tab === 'learn' && <LearnTab id={id} />}
      {tab === 'numbers' && <NumbersTab id={id} />}
      {tab === 'traps' && <TrapsTab id={id} />}
      {tab === 'cards' && <CardsTab id={id} />}
      {tab === 'test' && <TestTab id={id} />}
      {tab === 'recap' && <RecapTab id={id} />}
      {nextL && lp?.completed && <button class="btn block" onClick={() => go('lesson', nextL.id)}>Next lesson: {nextL.id} · {nextL.title}</button>}
    </div>
  );
}

function LearnTab({ id }: { id: string }) {
  const L = lessonById(id);
  const s = S();
  const widgets = widgetsForLesson(id);
  const concepts = L.conceptIds.map((c) => C.concepts[c]);
  const hostOf = new Map<string, string>();
  for (const w of widgets) { const c = concepts.find((x) => w.meta.anchor.test(x.title)); if (c) hostOf.set(w.meta.id, c.id); }
  const unanchored = widgets.filter((w) => !hostOf.has(w.meta.id));
  return (
    <div class="stack-lg">
      <div class="card tint stack">
        <strong>After this lesson you can answer:</strong>
        <ul class="core">{L.objectives.map((o) => <Html tag="li" html={o} />)}</ul>
        <p class="small">Read the short version of each part. Open <em>Dive deeper</em> when you want the full explanation, examples and handbook notes.</p>
      </div>
      {unanchored.map((w) => <WidgetFrame mod={w} concepts={L.conceptIds} />)}
      {concepts.map((c, i) => (
        <ConceptCard c={c} n={i + 1} seen={!!s.lessons[id]?.conceptsSeen?.[c.id]}>
          {widgets.filter((w) => hostOf.get(w.meta.id) === c.id).map((w) => <WidgetFrame mod={w} concepts={[c.id]} />)}
        </ConceptCard>
      ))}
      <div class="card stack">
        <h3>Ready to check yourself?</h3>
        <p>Take the {L.itemIds.length}-question practice test. It uses the same 3-choice style as the DMV test.</p>
        <button class="btn primary" onClick={() => startTest(id)}>Start the practice test</button>
      </div>
    </div>
  );
}

function ConceptCard({ c, n, seen, children }: { c: Concept; n: number; seen: boolean; children?: preact.ComponentChildren }) {
  const s = S();
  const items = Object.values(C.items).filter((i) => i.origin === 'pack' && i.concepts.includes(c.id));
  const b = s.bkt[c.id];
  const markSeen = () => mutate((st) => { const lp = st.lessons[c.lesson] ?? { conceptsSeen: {} }; lp.conceptsSeen = { ...(lp.conceptsSeen ?? {}), [c.id]: now() }; st.lessons[c.lesson] = lp; });
  const bnd = band(b?.p, b?.n ?? 0);
  return (
    <article class="card concept" id={`c-${c.id}`} aria-labelledby={`h-${c.id}`}>
      <div class="concept-head">
        <h3 id={`h-${c.id}`}>{c.section && <span class="muted num" style={{ fontSize: '.9em' }}>{c.section} </span>}{c.title} {c.ca && <span class="ca-tag" title="California-specific rule">CA</span>}</h3>
        <div class="row small">{bnd !== 'new' && <span class="chip" style={{ background: 'var(--accent-soft)', color: 'var(--accent)' }}>{bnd}</span>}<Pages pages={c.pages} /></div>
      </div>
      <ul class="core">{c.core.map((h) => <Html tag="li" html={h} />)}</ul>
      {c.coreExtra && <Html class="prose" html={c.coreExtra} />}
      {children}
      <details class="deep" onToggle={(e) => { if ((e.target as HTMLDetailsElement).open && !seen) markSeen(); }}>
        <summary>Dive deeper</summary>
        <Html class="prose" html={c.html} />
      </details>
      <div class="spread">
        <YouTube q={`${c.title}`} />
        {items.length > 0 && <button class="btn sm" onClick={() => { markSeen(); startSession({ title: `Quick check: ${c.title}`, ids: items.slice(0, 3).map((i) => i.id), mode: 'learn', lesson: c.lesson, back: { name: 'lesson', param: c.lesson, sub: c.id } }); }}>Quick check ({Math.min(3, items.length)})</button>}
      </div>
    </article>
  );
}

function WidgetFrame({ mod, concepts }: { mod: ReturnType<typeof widgetsForLesson>[number]; concepts: string[] }) {
  const s = S();
  const W = mod.default;
  const stamp = mod.meta.stamp;
  const got = stamp && s.stamps[stamp.id];
  return (
    <section class="widget" aria-label={mod.meta.title}>
      <div class="widget-head">
        <div class="stack" style={{ gap: '2px' }}><span class="eyebrow">Try it</span><strong>{mod.meta.title}</strong></div>
        {stamp && <span class="chip" style={{ background: got ? 'var(--accent)' : 'var(--surface-2)', color: got ? 'var(--accent-ink)' : 'var(--ink-2)' }} title={stamp.rule}>{got ? `✓ Stamp earned: ${stamp.name}` : `Stamp to earn: ${stamp.name}`}</span>}
      </div>
      <p class="small muted">{mod.meta.summary}</p>
      <W concepts={concepts} reducedMotion={s.prefs.reducedMotion}
        onEvidence={(e) => mutate((st) => recordCheck(st, e.concepts.length ? e.concepts : concepts, e.ok, now()))}
        onChallenge={() => { if (stamp && !s.stamps[stamp.id]) { mutate((st) => { st.stamps[stamp.id] = now(); st.widgets[mod.meta.id] = { done: (st.widgets[mod.meta.id]?.done ?? 0) + 1 }; }); say(`Stamp earned: ${stamp.name}`); } }} />
    </section>
  );
}

function NumbersTab({ id }: { id: string }) {
  const L = lessonById(id);
  return (
    <div class="stack-lg">
      <p>Most misses on the DMV test are exact numbers. Learn these word for word, then drill them.</p>
      <div class="tbl"><table><thead><tr><th>Item</th><th>Value / meaning</th><th>Page</th></tr></thead><tbody>
        {L.numberIds.map((n) => { const f = C.numbers[n]; return <tr><td>{f.item}{f.ca && <span class="ca-tag">CA</span>}</td><td><Html tag="span" html={f.valueHtml} /></td><td class="num">{f.pages.join(', ')}</td></tr>; })}
      </tbody></table></div>
      <button class="btn primary" onClick={() => startSession({ title: `${id} numbers drill`, ids: L.numberIds, mode: 'cards', lesson: id, back: { name: 'lesson', param: id } })}>Drill all {L.numberIds.length} numbers</button>
    </div>
  );
}

function TrapsTab({ id }: { id: string }) {
  const L = lessonById(id);
  const duels = Object.values(C.items).filter((i) => i.origin === 'derived-trap' && i.lesson === id).map((i) => i.id);
  return (
    <div class="stack-lg">
      <p>These are the wrong answers the test is built to catch. The struck-out line is the trap; the line under it is what the handbook says.</p>
      <div class="trap-list">
        {L.trapIds.map((t) => { const x = C.traps[t]; return (
          <div class="trap"><span class="diamond" aria-hidden="true" /><div class="stack" style={{ gap: '4px' }}><span class="wrong"><span class="sr-only">Trap: </span>{x.trap}</span><span><strong>Handbook:</strong> <Html tag="span" html={x.correctHtml} /></span><Pages pages={x.pages} /></div></div>
        ); })}
      </div>
      <button class="btn primary" onClick={() => startSession({ title: `${id} trap duel`, ids: duels, mode: 'practice', lesson: id, back: { name: 'lesson', param: id } })}>Trap duel: true or false ({duels.length})</button>
    </div>
  );
}

function CardsTab({ id }: { id: string }) {
  const L = lessonById(id);
  return (
    <div class="stack-lg">
      <div class="card stack">
        <h3>Flashcards ({L.flashIds.length})</h3>
        <p class="small muted">Say the answer out loud, then flip. Cards you miss come back sooner.</p>
        <button class="btn primary" onClick={() => startSession({ title: `${id} flashcards`, ids: L.flashIds, mode: 'cards', lesson: id, back: { name: 'lesson', param: id } })}>Study flashcards</button>
      </div>
      {L.tykIds.length > 0 ? (
        <div class="card stack">
          <h3>Handbook review questions ({L.tykIds.length})</h3>
          <p class="small muted">The handbook says these “may be on your test”. Answer in your head, then check.</p>
          <button class="btn" onClick={() => startSession({ title: `${id} handbook questions`, ids: L.tykIds, mode: 'cards', lesson: id, back: { name: 'lesson', param: id } })}>Answer the handbook questions</button>
        </div>
      ) : <p class="small muted">This handbook section has no review box of its own.</p>}
    </div>
  );
}

export function startTest(id: string) {
  const L = lessonById(id);
  startSession({ title: `${id} practice test`, ids: L.itemIds, mode: 'practice', lesson: id, back: { name: 'lesson', param: id }, testLesson: id });
}

function TestTab({ id }: { id: string }) {
  const L = lessonById(id);
  const lp = S().lessons[id];
  return (
    <div class="card stack">
      <h3>{L.itemIds.length}-question practice test</h3>
      <p>Same style as the DMV: 3 choices, one best answer. You see right away if you are wrong, with the reason and the handbook page.</p>
      {lp?.practiceLast && <p class="small">Last try: <strong class="num">{lp.practiceLast.score}/{lp.practiceLast.total}</strong>. Best: <strong class="num">{Math.round((lp.practiceBest ?? 0) * 100)}%</strong>.</p>}
      <p class="small muted">Score 90% or more to finish the lesson.</p>
      <button class="btn primary" onClick={() => startTest(id)}>{lp?.practiceLast ? 'Take it again' : 'Start'}</button>
    </div>
  );
}

function RecapTab({ id }: { id: string }) {
  const L = lessonById(id);
  const missed = C.mostMissed.filter((m) => m.lesson === id);
  return (
    <div class="stack-lg">
      <div class="card stack"><h3>One-minute recap</h3><ul class="core">{L.recap.map((r) => <Html tag="li" html={r} />)}</ul></div>
      {missed.length > 0 && <div class="card warn stack"><h3>Most-missed on the real test</h3><ul class="core">{missed.map((m) => <Html tag="li" html={m.text} />)}</ul></div>}
    </div>
  );
}

/** Test/preview route: #widget.<id> renders one widget alone (used by screenshot scoring). */
export function WidgetPreview({ id }: { id: string }) {
  const m = widgetById(id);
  if (!m) return <div class="card"><p>Unknown widget “{id}”. Known: {WIDGETS.map((w) => w.meta.id).join(', ')}</p></div>;
  const L = lessonById(m.meta.lesson);
  const host = L.conceptIds.find((c) => m.meta.anchor.test(C.concepts[c].title));
  return <div class="stack"><div class="row"><Shield id={L.id} /><span class="small muted">{host ? `From the section: ${C.concepts[host].title}` : 'Hosted at lesson top (anchor matched no concept)'}</span></div><WidgetFrame mod={m} concepts={host ? [host] : L.conceptIds} /></div>;
}
