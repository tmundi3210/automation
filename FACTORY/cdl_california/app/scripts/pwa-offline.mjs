// PWA offline check: serves dist/ on localhost, waits for the service worker, goes offline, reloads.
import { chromium } from '@playwright/test';
import { spawn } from 'node:child_process';
const srv = spawn('npx', ['vite', 'preview', '--outDir', 'dist', '--port', '4179', '--strictPort'], { stdio: 'ignore' });
await new Promise((r) => setTimeout(r, 3000));
const browser = await chromium.launch();
const ctx = await browser.newContext({ viewport: { width: 390, height: 844 } });
const page = await ctx.newPage();
let ok = false, detail = '';
try {
  await page.goto('http://localhost:4179/');
  await page.waitForFunction(() => navigator.serviceWorker && navigator.serviceWorker.controller !== null || (navigator.serviceWorker.ready && false), null, { timeout: 5000 }).catch(() => {});
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await page.reload(); // now controlled
  const controlled = await page.evaluate(() => !!navigator.serviceWorker.controller);
  await ctx.setOffline(true);
  const resp = await page.reload();
  const fromSW = resp ? resp.fromServiceWorker() : false;
  const txt = await page.getByRole('button', { name: 'Get started' }).isVisible();
  const manifest = await page.evaluate(() => !!document.querySelector('link[rel="manifest"]'));
  ok = controlled && fromSW && txt && manifest;
  detail = JSON.stringify({ controlled, fromSW, rendersOffline: txt, manifest });
} catch (e) { detail = String(e); }
console.log(ok ? 'PWA OFFLINE PASS' : 'PWA OFFLINE FAIL', detail);
await browser.close(); srv.kill();
process.exit(ok ? 0 : 1);
