// Gate screenshot set: node scripts/gate-shots.mjs <outDir> [htmlPath]
import { chromium } from '@playwright/test';
import { resolve } from 'node:path';
import { mkdirSync, writeFileSync } from 'node:fs';
const out = process.argv[2]; const html = resolve(process.argv[3] || 'dist-single/index.html');
mkdirSync(out, { recursive: true });
const URL = 'file://' + html;
const browser = await chromium.launch();
const report = { errors: [], overflow: [] };
async function ctxFor(vp, scheme) {
  const ctx = await browser.newContext({ viewport: vp === 'm' ? { width: 390, height: 844 } : { width: 1280, height: 800 }, colorScheme: scheme });
  const page = await ctx.newPage();
  page.on('pageerror', (e) => report.errors.push(`${vp}-${scheme}: ${e}`));
  await page.clock.install({ time: new Date('2026-09-28T10:00:00') });
  return { ctx, page };
}
async function onboard(page) {
  await page.goto(URL);
  await page.getByRole('button', { name: 'Get started' }).click();
  await page.locator('button.choice').first().click();
  await page.getByRole('button', { name: 'Next' }).click();
  await page.getByRole('button', { name: 'Next' }).click();
  await page.getByRole('button', { name: /Use \d+ minutes/ }).click().catch(() => {});
  await page.getByRole('button', { name: 'Build my plan' }).click();
}
async function study(page) {
  // simulate 3 days of study: GK-01 practice test (mixed answers), GK-01 numbers, GK-07 test
  for (const [lesson, n] of [['GK-01', 18], ['GK-07', 22]]) {
    await page.goto(URL + '#lesson.' + lesson);
    await page.getByRole('tab', { name: 'Practice test' }).click();
    await page.getByRole('button', { name: /^Start$|Take it again/ }).click();
    for (let i = 0; i < n; i++) { await page.locator('.opt').nth(i % 3 === 0 ? 1 : 0).click(); await page.getByRole('button', { name: 'Continue' }).click(); }
    await page.getByRole('button', { name: 'Done' }).click();
  }
}
const shoot = async (page, name, full = false) => {
  await page.waitForTimeout(250);
  const ov = await page.evaluate(() => document.documentElement.scrollWidth > innerWidth + 1);
  if (ov) report.overflow.push(name);
  await page.screenshot({ path: `${out}/${name}.png`, fullPage: full, animations: 'disabled' });
};
const screens = ['today', 'path', 'lesson.GK-07', 'lesson.CV-03', 'practice', 'notebook', 'progress', 'guide', 'glossary', 'settings'];
for (const [vp, scheme] of [['m', 'light'], ['m', 'dark'], ['d', 'light']]) {
  const { ctx, page } = await ctxFor(vp, scheme);
  await page.goto(URL); await shoot(page, `${vp}-${scheme}-00-welcome`);
  await onboard(page); await shoot(page, `${vp}-${scheme}-01-today-new`);
  await study(page);
  for (const [i, s] of screens.entries()) { await page.goto(URL + '#' + s); await shoot(page, `${vp}-${scheme}-${String(i + 2).padStart(2, '0')}-${s}`); }
  // a question with feedback, and a mock question
  await page.goto(URL + '#lesson.GK-08'); await page.getByRole('tab', { name: 'Practice test' }).click(); await page.getByRole('button', { name: /^Start$/ }).click();
  await shoot(page, `${vp}-${scheme}-20-question`); await page.locator('.opt').nth(2).click(); await shoot(page, `${vp}-${scheme}-21-feedback`);
  await page.goto(URL + '#practice'); await page.getByRole('button', { name: 'Start GK mock' }).click(); await page.getByRole('button', { name: 'Begin' }).click();
  await page.locator('.opt').first().click(); await shoot(page, `${vp}-${scheme}-22-mock`);
  await ctx.close();
}
// widgets alone
const { ctx, page } = await ctxFor('m', 'light');
await page.goto(URL);
const ids = await page.evaluate(() => [...document.querySelectorAll('*')].length && null);
await ctx.close();
const WIDGETS = process.env.WIDGETS.split(',');
for (const [vp, scheme] of [['m', 'light'], ['d', 'dark']]) {
  const { ctx, page } = await ctxFor(vp, scheme);
  for (const id of WIDGETS) { await page.goto(URL + '#widget.' + id); await page.waitForTimeout(200); await shoot(page, `w-${id}-${vp}-${scheme}`, vp === 'm'); }
  await ctx.close();
}
writeFileSync(`${out}/report.json`, JSON.stringify(report, null, 1));
console.log(JSON.stringify(report));
await browser.close();
