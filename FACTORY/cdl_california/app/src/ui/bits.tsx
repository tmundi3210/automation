import type { ComponentChildren } from 'preact';
import { C, S } from '../app';
import { addDays, dayKey, dayKeyToDate } from '../engine/model';
import { dayCell } from '../engine/plan';

export function Html({ html, class: cls, tag = 'div' }: { html: string; class?: string; tag?: 'div' | 'span' | 'p' | 'li' }) {
  const T = tag as 'div';
  return <T class={cls} dangerouslySetInnerHTML={{ __html: html }} />;
}

export function Pages({ pages }: { pages: string[] }) {
  if (!pages.length) return null;
  const txt = pages.length > 2 ? `${pages[0]} – ${pages[pages.length - 1]}` : pages.join(', ');
  return <span class="plate" title="Page in the CA Commercial Driver Handbook (DL 650)">p. {txt}</span>;
}

export function Shield({ id, done }: { id: string; done?: boolean }) {
  return <span class={`shield ${id.startsWith('CV') ? 'cv' : ''} ${done ? 'done' : ''}`} aria-label={`Lesson ${id}`}>{id}</span>;
}

export function Ring({ p, label }: { p: number; label?: string }) {
  const pct = Math.round(p * 100);
  return <span class="ring" style={{ '--p': pct } as unknown as string} role="img" aria-label={label ?? `${pct}% mastered`}><span>{pct}%</span></span>;
}

export function Bar({ p, label }: { p: number; label: string }) {
  return <div class="bar" role="meter" aria-valuemin={0} aria-valuemax={100} aria-valuenow={Math.round(p * 100)} aria-label={label}><span style={{ width: `${Math.round(p * 100)}%` }} /></div>;
}

export function Card({ children, class: cls = '', label }: { children: ComponentChildren; class?: string; label?: string }) {
  return <section class={`card ${cls}`} aria-label={label}>{children}</section>;
}

const DOW = ['S', 'M', 'T', 'W', 'T', 'F', 'S'];
/** Week strip or month grid of day boxes: empty → fills in 5 % steps → tick at 100 %. */
export function Calendar({ start, days, examDay, nowT }: { start: string; days: number; examDay?: string; nowT: number }) {
  const s = S();
  const t = dayKey(nowT);
  const first = dayKeyToDate(start).getDay();
  const cells = [] as preact.JSX.Element[];
  for (let i = 0; i < first; i++) cells.push(<div aria-hidden="true" />);
  for (let i = 0; i < days; i++) {
    const k = addDays(start, i);
    const c = dayCell(s, k, nowT);
    const d = dayKeyToDate(k).getDate();
    const isExam = examDay === k;
    const label = `${dayKeyToDate(k).toLocaleDateString(undefined, { month: 'short', day: 'numeric' })}: ${isExam ? 'test day, ' : ''}${c.state === 'done' ? 'complete' : c.state === 'partial' ? `${c.pct}% done` : c.state}`;
    cells.push(
      <div class={`day ${c.state} ${k === t ? 'today' : ''} ${isExam ? 'exam' : ''}`} title={label} aria-label={label} role="img">
        {c.state === 'partial' && <span class="fill" style={{ height: `${c.pct}%` }} />}
        <span class="d">{c.state === 'done' ? '✓' : d}</span>
        {c.state === 'partial' && <span class="p">{c.pct}%</span>}
        {isExam && <span class="p">TEST</span>}
      </div>,
    );
  }
  return (
    <div class="stack" style={{ gap: '6px' }}>
      <div class="cal-head" aria-hidden="true">{DOW.map((d) => <span>{d}</span>)}</div>
      <div class="cal">{cells}</div>
    </div>
  );
}

export function YouTube({ q }: { q: string }) {
  const url = `https://www.youtube.com/results?search_query=${encodeURIComponent(q + ' CDL')}`;
  return (
    <a class="yt" href={url} target="_blank" rel="noopener noreferrer" title="Opens a YouTube search in a new tab. Videos are extra: the handbook wins if they differ.">
      <svg width="18" height="18" viewBox="0 0 24 24" aria-hidden="true"><rect x="2" y="5" width="20" height="14" rx="4" fill="currentColor" opacity=".18" /><path d="M10 9l5 3-5 3z" fill="currentColor" /></svg>
      Watch videos on this
    </a>
  );
}

export const lessonTitle = (id: string) => C.lessons.find((l) => l.id === id)?.title ?? id;

export function Icon({ name }: { name: 'today' | 'path' | 'practice' | 'notebook' | 'more' | 'back' | 'check' }) {
  const p: Record<string, preact.JSX.Element> = {
    today: <><rect x="3" y="5" width="18" height="16" rx="2" fill="none" stroke="currentColor" stroke-width="2" /><path d="M3 10h18M8 3v4M16 3v4" stroke="currentColor" stroke-width="2" /><path d="M8 15l3 3 5-6" fill="none" stroke="currentColor" stroke-width="2" /></>,
    path: <><path d="M6 21c0-6 12-6 12-12V3" fill="none" stroke="currentColor" stroke-width="2" /><circle cx="6" cy="21" r="1.5" fill="currentColor" /><path d="M14 6l4-3 4 3" fill="none" stroke="currentColor" stroke-width="2" /></>,
    practice: <><rect x="4" y="3" width="16" height="18" rx="2" fill="none" stroke="currentColor" stroke-width="2" /><path d="M8 8h8M8 12h8M8 16h5" stroke="currentColor" stroke-width="2" /></>,
    notebook: <><path d="M12 3l9 16H3z" fill="none" stroke="currentColor" stroke-width="2" stroke-linejoin="round" /><path d="M12 10v4M12 16.5v.5" stroke="currentColor" stroke-width="2" /></>,
    more: <><circle cx="5" cy="12" r="2" fill="currentColor" /><circle cx="12" cy="12" r="2" fill="currentColor" /><circle cx="19" cy="12" r="2" fill="currentColor" /></>,
    back: <path d="M15 5l-7 7 7 7" fill="none" stroke="currentColor" stroke-width="2.5" />,
    check: <path d="M5 12l5 5 9-10" fill="none" stroke="currentColor" stroke-width="2.5" />,
  };
  return <svg viewBox="0 0 24 24" aria-hidden="true">{p[name]}</svg>;
}

export function Back({ onClick, label = 'Back' }: { onClick: () => void; label?: string }) {
  return <button class="btn ghost sm" onClick={onClick} style={{ alignSelf: 'flex-start', marginLeft: '-8px' }}><span style={{ width: '18px', display: 'inline-flex' }}><Icon name="back" /></span>{label}</button>;
}
