import { render } from 'preact';
import './ui/fonts.css';
import './ui/styles.css';
import { App } from './ui/App';
import { applyPrefs, now, replaceState, restoreSession, route, routeFromHash, ready } from './app';
import { connectRemote, flush, loadLocal } from './store/store';

async function boot() {
  const local = await loadLocal(now());
  replaceState(local);
  const r = routeFromHash();
  if (location.hash === '#session' && local.sessionRun) { restoreSession(); }
  else if (r) route.value = r;
  else if (local.profile.onboarded && local.cursor?.route && !['session', 'mock', 'onboarding'].includes(local.cursor.route)) route.value = { name: local.cursor.route as never, param: local.cursor.param };
  applyPrefs();
  ready.value = true;
  render(<App />, document.getElementById('app')!);
  window.addEventListener('popstate', () => { const x = routeFromHash(); if (x) route.value = x; });
  window.addEventListener('pagehide', () => { void import('./app').then((m) => flush(m.peek())); });
  document.addEventListener('visibilitychange', () => { if (document.visibilityState === 'hidden') void import('./app').then((m) => flush(m.peek())); });
  // Artifact runtime: hydrate from the viewer's private store when it answers (never blocks first paint)
  const remote = await connectRemote(local, now());
  if (remote) replaceState(remote);
}
void boot();
