import { useEffect, useState } from 'preact/hooks';
import { C, go, route, S, toast } from '../app';
import type { RouteName } from '../app';
import { today } from '../engine/plan';
import { readiness } from '../engine/readiness';
import { Icon } from './bits';
import { LessonScreen } from './Lesson';
import { SessionScreen } from './Session';
import { MockScreen } from './Mock';
import { GuideScreen, GlossaryScreen } from './Guide';
import { SettingsScreen } from './Settings';
import { OnboardingScreen } from './Onboarding';
import { TodayScreen, PathScreen, PracticeScreen, NotebookScreen, ProgressScreen, MoreScreen } from './Screens';
import { getStatus, onStatus } from '../store/store';
import type { TestId } from '../content/types';
import { WidgetPreview } from './Lesson';

const TABS: [RouteName, string, 'today' | 'path' | 'practice' | 'notebook' | 'more', RouteName[]][] = [
  ['today', 'Today', 'today', ['today']],
  ['path', 'Lessons', 'path', ['path', 'lesson']],
  ['practice', 'Practice', 'practice', ['practice', 'mock', 'session']],
  ['notebook', 'Mistakes', 'notebook', ['notebook']],
  ['more', 'More', 'more', ['more', 'progress', 'guide', 'glossary', 'settings']],
];

export function App() {
  const s = S();
  const r = route.value;
  const [sync, setSync] = useState(getStatus());
  useEffect(() => onStatus(setSync), []);
  if (r.name === 'widget') return <main id="main" style={{ paddingBlock: '16px' }}><div class="page"><WidgetPreview id={r.param ?? ''} /></div></main>;
  const onboarding = !s.profile.onboarded || r.name === 'onboarding';
  const q = onboarding ? null : today(s, C, Date.now());
  const openFix = q ? q.fixes.length : 0;
  const chip = !onboarding ? (() => {
    const t = s.profile.tests[0] as TestId;
    const rd = readiness(s, C, t, Date.now(), 400);
    return `${t}: ${rd.band === 'likely' ? 'likely pass' : rd.band === 'borderline' ? 'borderline' : 'not ready yet'}`;
  })() : null;
  let screen;
  if (onboarding) screen = <OnboardingScreen />;
  else switch (r.name) {
    case 'path': screen = <PathScreen />; break;
    case 'lesson': screen = <LessonScreen id={r.param ?? 'GK-01'} focus={r.sub} />; break;
    case 'practice': screen = <PracticeScreen />; break;
    case 'session': screen = <SessionScreen />; break;
    case 'mock': screen = <MockScreen test={(r.param as TestId) ?? 'GK'} key={r.param} />; break;
    case 'notebook': screen = <NotebookScreen />; break;
    case 'progress': screen = <ProgressScreen />; break;
    case 'guide': screen = <GuideScreen />; break;
    case 'glossary': screen = <GlossaryScreen />; break;
    case 'settings': screen = <SettingsScreen />; break;
    case 'more': screen = <MoreScreen />; break;
    default: screen = <TodayScreen />;
  }
  return (
    <div class="shell">
      <header class="topbar">
        <div class="topbar-in">
          <button class="brand" onClick={() => go('today')} aria-label="CDL Workshop, go to Today">
            <span class="brand-mark" aria-hidden="true">CDL</span>
            <span><span class="brand-name">Workshop</span><br /><span class="brand-sub">California</span></span>
          </button>
          <span class="spacer" />
          {chip && <button class="chip" onClick={() => go('progress')} style={{ cursor: 'pointer' }} title="Readiness estimate">{chip}</button>}
          {!onboarding && sync === 'memory' && <button class="chip" onClick={() => go('settings')} style={{ cursor: 'pointer', background: 'var(--amber)', color: '#241a00' }} title="This browser is not saving. Open Settings to copy a resume code.">Not saved</button>}
        </div>
      </header>
      {!onboarding && (
        <nav class="tabbar" aria-label="Main">
          <div class="tabbar-in">
            {TABS.map(([name, label, icon, owns]) => (
              <button class="tab" aria-current={owns.includes(r.name) ? 'page' : undefined} onClick={() => go(name)}>
                <Icon name={icon} />{label}{name === 'notebook' && openFix > 0 && <span class="badge num">{openFix}</span>}
              </button>
            ))}
          </div>
        </nav>
      )}
      <main id="main">{screen}</main>
      {toast.value && <div class="toast" role="status">{toast.value}</div>}
    </div>
  );
}
