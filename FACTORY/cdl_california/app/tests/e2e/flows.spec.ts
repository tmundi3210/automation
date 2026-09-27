import { test, expect, type Page } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { resolve } from 'node:path';
import { writeFileSync, mkdirSync } from 'node:fs';

const URL = 'file://' + resolve('dist-single/index.html');
const T0 = new Date('2026-09-28T10:00:00');
const DAY = 86400_000;

async function onboard(page: Page, cls: 'A' | 'B' = 'A') {
  await page.goto(URL);
  await page.getByRole('button', { name: 'Get started' }).click();
  await page.locator('button.choice', { hasText: `Class ${cls}` }).click();
  await page.getByRole('button', { name: 'Next' }).click();
  await page.getByRole('button', { name: 'Next' }).click();
  await page.getByRole('button', { name: 'Build my plan' }).click();
  await expect(page.getByText('Next best step')).toBeVisible();
}
async function noOverflow(page: Page) {
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth + 1)).toBe(true);
}

test.beforeEach(async ({ page }) => {
  await page.clock.install({ time: T0 });
  page.on('pageerror', (e) => { throw e; });
});

test('onboarding → class-specific plan (A has CV, B does not)', async ({ page }) => {
  await onboard(page, 'A');
  await expect(page.getByText(/Start GK-01/)).toBeVisible();
  await expect(page.getByLabel('Combination Vehicles readiness')).toBeVisible();
  await noOverflow(page);
  await page.evaluate(() => indexedDB.deleteDatabase('keyval-store'));
});

test('class B path hides Combination readiness', async ({ page }) => {
  await onboard(page, 'B');
  await expect(page.getByLabel('Combination Vehicles readiness')).toHaveCount(0);
});

test('lesson → practice with wrong answers → mistake list with cause → survives reload', async ({ page }) => {
  await onboard(page);
  await page.goto(URL + '#lesson.GK-07');
  await page.getByText('Dive deeper').first().click();
  await page.getByRole('tab', { name: 'Practice test' }).click();
  await page.getByRole('button', { name: 'Start' }).click();
  // answer every question with the first option; some will be wrong
  for (let i = 0; i < 22; i++) {
    await page.locator('.opt').first().click();
    await page.getByRole('button', { name: 'Continue' }).click();
  }
  await expect(page.getByText(/\d+ \/ 22/)).toBeVisible();
  await page.getByRole('button', { name: 'Done' }).click();
  await page.goto(URL + '#notebook');
  const rows = page.getByRole('button', { name: 'Fix drill' });
  const n = await rows.count();
  expect(n).toBeGreaterThan(0);
  await page.reload();
  await expect(page.getByRole('button', { name: 'Fix drill' })).toHaveCount(n);
  await noOverflow(page);
});

test('spaced review comes due after days pass; calendar fills', async ({ page }) => {
  await onboard(page);
  await page.goto(URL + '#lesson.GK-01');
  await page.getByRole('tab', { name: /Numbers/ }).click();
  await page.getByRole('button', { name: /Drill all/ }).click();
  for (let i = 0; i < 6; i++) { await page.getByRole('button', { name: 'Show answer' }).click(); await page.getByRole('button', { name: 'I knew it' }).click(); }
  await page.getByRole('button', { name: 'Leave' }).click();
  await page.goto(URL + '#today');
  await expect(page.locator('.day.today.partial, .day.today.done')).toHaveCount(1);
  await page.clock.setSystemTime(new Date(T0.getTime() + 5 * DAY));
  await page.reload();
  await expect(page.getByText(/Review \d+ cards that are due|Cards due for review/).first()).toBeVisible();
  const due = await page.locator('.li .chip').first().innerText();
  expect(Number(due)).toBeGreaterThan(0);
});

test('DMV-style mock: 50 questions, skip returns at end, result recorded', async ({ page }) => {
  await onboard(page);
  await page.goto(URL + '#practice');
  await page.getByRole('button', { name: 'Start GK mock' }).click();
  await page.getByRole('button', { name: 'Begin' }).click();
  await page.getByRole('button', { name: 'Skip for now' }).click();
  for (let i = 0; i < 50; i++) {
    await page.locator('.opt').nth(1).click();
    await page.getByRole('button', { name: /Next question|See my result/ }).click();
  }
  await expect(page.getByText(/\d+ \/ 50 — (Pass|Not yet)/)).toBeVisible();
  await page.goto(URL + '#progress');
  await expect(page.getByLabel('Mock history')).toBeVisible();
});

test('resume code: erase then restore gives the same progress', async ({ page }) => {
  await onboard(page);
  await page.goto(URL + '#lesson.GK-02');
  await page.getByRole('tab', { name: 'Practice test' }).click();
  await page.getByRole('button', { name: 'Start' }).click();
  for (let i = 0; i < 3; i++) { await page.locator('.opt').first().click(); await page.getByRole('button', { name: 'Continue' }).click(); }
  await page.goto(URL + '#settings');
  await page.getByRole('button', { name: 'Make a resume code' }).click();
  const code = await page.locator('#rc').inputValue();
  expect(code.startsWith('CDLWS1.')).toBe(true);
  await page.getByRole('button', { name: 'Erase my progress…' }).click();
  await page.getByRole('button', { name: 'Erase everything' }).click();
  await expect(page.getByRole('button', { name: 'Get started' })).toBeVisible();
  await page.goto(URL + '#settings');
  await page.evaluate(() => { location.hash = '#settings'; });
  // after erase the app shows onboarding; skip it to reach settings
  await page.getByText('Skip setup and look around').click();
  await page.goto(URL + '#settings');
  await page.locator('#rp').fill(code);
  await page.getByRole('button', { name: 'Restore', exact: true }).click();
  await expect(page.getByText('Progress restored.')).toBeVisible();
  await page.goto(URL + '#progress');
  await expect(page.locator('.stat .v').nth(2)).toHaveText('3');
});

test('sandboxed frame without storage still runs and warns', async ({ page }) => {
  mkdirSync('test-results', { recursive: true });
  const harness = resolve('test-results/sandbox.html');
  writeFileSync(harness, `<!doctype html><iframe id="f" sandbox="allow-scripts allow-popups" src="${URL}" style="width:390px;height:800px;border:0"></iframe>`);
  await page.goto('file://' + harness);
  const f = page.frameLocator('#f');
  await f.getByRole('button', { name: 'Get started' }).click();
  await f.locator('button.choice', { hasText: 'Class A' }).click();
  await f.getByRole('button', { name: 'Next' }).click();
  await f.getByRole('button', { name: 'Next' }).click();
  await f.getByRole('button', { name: 'Build my plan' }).click();
  await expect(f.getByRole('button', { name: /Not saved/ })).toBeVisible();
});

for (const hash of ['today', 'path', 'lesson.CV-03', 'practice', 'notebook', 'progress', 'guide', 'glossary', 'settings']) {
  test(`a11y + no overflow: ${hash}`, async ({ page }) => {
    await onboard(page);
    await page.goto(URL + '#' + hash);
    await page.waitForTimeout(200);
    await noOverflow(page);
    const r = await new AxeBuilder({ page } as never).withTags(['wcag2a', 'wcag2aa']).analyze();
    const bad = r.violations.filter((v) => v.impact === 'serious' || v.impact === 'critical');
    expect(bad.map((v) => `${v.id}: ${v.nodes.length} ${v.nodes[0]?.target}`)).toEqual([]);
  });
}
