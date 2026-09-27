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
  await page.locator('button.choice').filter({ has: page.locator('.t', { hasText: new RegExp(`^Class ${cls}$`) }) }).click();
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
  await page.getByRole('button', { name: 'Start', exact: true }).click();
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
  await page.getByRole('button', { name: 'Start General Knowledge mock' }).click();
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
  await page.getByRole('button', { name: 'Start', exact: true }).click();
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
  // regression (evaluator 2): the restored progress must survive a reload
  await page.reload();
  await page.goto(URL + '#progress');
  await expect(page.locator('.stat .v').nth(2)).toHaveText('3');
});

test('unfinished practice test resumes after reload', async ({ page }) => {
  await onboard(page);
  await page.goto(URL + '#lesson.GK-02');
  await page.getByRole('tab', { name: 'Practice test' }).click();
  await page.getByRole('button', { name: 'Start', exact: true }).click();
  for (let i = 0; i < 3; i++) { await page.locator('.opt').first().click(); await page.getByRole('button', { name: 'Continue' }).click(); }
  await expect(page.getByText('4 of 15')).toBeVisible();
  await page.reload();
  await expect(page.getByText('4 of 15')).toBeVisible();
});

test('unfinished mock resumes after reload', async ({ page }) => {
  await onboard(page);
  await page.goto(URL + '#practice');
  await page.getByRole('button', { name: 'Start General Knowledge mock' }).click();
  await page.getByRole('button', { name: 'Begin' }).click();
  for (let i = 0; i < 3; i++) { await page.locator('.opt').first().click(); await page.getByRole('button', { name: /Next question/ }).click(); }
  await page.reload();
  await page.goto(URL + '#practice');
  await page.getByRole('button', { name: /Resume General Knowledge mock \(3\/50\)/ }).click();
  await expect(page.getByText(/^4$/).first()).toBeVisible();
});

test('reopening the app (no URL hash) returns to an unfinished practice test and mock', async ({ page }) => {
  await onboard(page);
  await page.goto(URL + '#lesson.GK-02');
  await page.getByRole('tab', { name: 'Practice test' }).click();
  await page.getByRole('button', { name: 'Start', exact: true }).click();
  for (let i = 0; i < 2; i++) { await page.locator('.opt').first().click(); await page.getByRole('button', { name: 'Continue' }).click(); }
  await page.goto('about:blank'); await page.goto(URL);
  await expect(page.getByText('3 of 15')).toBeVisible();
  await page.goto(URL + '#practice');
  await page.getByRole('button', { name: 'Start General Knowledge mock' }).click();
  await page.getByRole('button', { name: 'Begin' }).click();
  for (let i = 0; i < 2; i++) { await page.locator('.opt').first().click(); await page.getByRole('button', { name: /Next question/ }).click(); }
  await page.reload();
  await expect(page.getByRole('button', { name: /Next question|Skip/ }).first()).toBeVisible();
  await page.goto('about:blank'); await page.goto(URL);
  await expect(page.getByRole('button', { name: /Skip/ }).first()).toBeVisible();
});

test('mock: an answer survives a reload before Next and is recorded once', async ({ page }) => {
  await onboard(page);
  await page.goto(URL + '#practice');
  await page.getByRole('button', { name: 'Start General Knowledge mock' }).click();
  await page.getByRole('button', { name: 'Begin' }).click();
  await page.locator('.opt').first().click();
  const n1 = await page.evaluate(() => JSON.parse(localStorage.getItem('cdlws.state.v1') || '{}').attempts?.length);
  await page.reload();
  await expect(page.getByRole('button', { name: /Next question/ })).toBeVisible();   // shown as answered, not re-asked
  await expect(page.locator('.opt').first()).toBeDisabled();
  const n2 = await page.evaluate(() => JSON.parse(localStorage.getItem('cdlws.state.v1') || '{}').attempts?.length);
  expect(n1).toBeGreaterThan(0);
  expect(n2).toBe(n1);
});

test('glossary term in lesson text opens a definition', async ({ page }) => {
  await onboard(page);
  await page.goto(URL + '#lesson.GK-01');
  await page.locator('button.gl:visible').first().click();
  await expect(page.getByRole('dialog')).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(page.getByRole('dialog')).toHaveCount(0);
});

test('sandboxed frame without storage still runs and warns', async ({ page }) => {
  mkdirSync('test-results', { recursive: true });
  const harness = resolve('test-results/sandbox.html');
  writeFileSync(harness, `<!doctype html><iframe id="f" sandbox="allow-scripts allow-popups" src="${URL}" style="width:390px;height:800px;border:0"></iframe>`);
  await page.goto('file://' + harness);
  const f = page.frameLocator('#f');
  await f.getByRole('button', { name: 'Get started' }).click();
  await f.locator('button.choice').first().click();
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

// dark theme: axe on the same screens, with open mistakes so the tab badge is on screen (round-3 review found it at 2.28:1)
test('a11y dark mode: main screens with a mistake badge', async ({ page }) => {
  await page.emulateMedia({ colorScheme: 'dark' });
  await onboard(page);
  await page.goto(URL + '#lesson.GK-02');
  await page.getByRole('tab', { name: 'Practice test' }).click();
  await page.getByRole('button', { name: 'Start', exact: true }).click();
  for (let i = 0; i < 6; i++) { await page.locator('.opt').nth(i % 3).click(); await page.getByRole('button', { name: 'Continue' }).click(); }
  for (const hash of ['today', 'lesson.CV-02', 'practice', 'notebook', 'progress', 'guide', 'settings']) {
    await page.goto(URL + '#' + hash);
    await page.waitForTimeout(200);
    const r = await new AxeBuilder({ page } as never).withTags(['wcag2a', 'wcag2aa']).analyze();
    const bad = r.violations.filter((v) => v.impact === 'serious' || v.impact === 'critical');
    expect(bad.map((v) => `${hash} ${v.id}: ${v.nodes.length} ${v.nodes[0]?.target}`)).toEqual([]);
  }
  await expect(page.locator('.tab .badge')).toBeVisible();
  // axe skips 1–2 character text, so measure the badge directly (WCAG AA 4.5:1)
  const ratio = await page.locator('.tab .badge').evaluate((e) => {
    const rgb = (c: string) => c.match(/\d+(\.\d+)?/g)!.slice(0, 3).map(Number);
    const lum = (c: number[]) => { const [r, g, b] = c.map((v) => { v /= 255; return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4; }); return 0.2126 * r + 0.7152 * g + 0.0722 * b; };
    const cs = getComputedStyle(e); const a = lum(rgb(cs.color)), b = lum(rgb(cs.backgroundColor));
    return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
  });
  expect(ratio).toBeGreaterThanOrEqual(4.5);
});
