import { test, expect } from '@playwright/test';
import { resolve } from 'node:path';
import { readdirSync } from 'node:fs';

const URL = 'file://' + resolve('dist-single/index.html');
const IDS = readdirSync('src/widgets/w').filter((f) => /^[a-z0-9-]+\.tsx$/.test(f)).map((f) => f.replace('.tsx', ''));

// Gamification invariant: Explore interactions never write the learner model (BKT/FSRS).
for (const id of IDS) {
  test(`explore mode does not write the learner model: ${id}`, async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', (e) => errors.push(String(e)));
    await page.goto(URL + '#widget.' + id);
    await expect(page.locator('.widget')).toBeVisible();
    const buttons = page.locator('.widget button:not([role="tab"])');
    const n = Math.min(await buttons.count(), 14);
    for (let i = 0; i < n; i++) { const b = buttons.nth(i); if (await b.isVisible() && await b.isEnabled()) await b.click({ timeout: 2000 }).catch(() => {}); }
    await page.waitForTimeout(800);
    const bkt = await page.evaluate(() => { try { const s = JSON.parse(localStorage.getItem('cdlws.state.v1') || 'null'); return s ? Object.keys(s.bkt || {}).length + Object.keys(s.cards || {}).length : 0; } catch { return -1; } });
    expect(errors).toEqual([]);
    expect(bkt).toBe(0);
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1)).toBe(true);
  });
}

test('challenge answers DO count as evidence (class finder sort)', async ({ page }) => {
  await page.goto(URL + '#widget.gk01-class-finder');
  await page.getByRole('tab', { name: /Sort 7 vehicles/ }).click();
  await page.getByRole('button', { name: 'Class A', exact: true }).click();
  await page.waitForTimeout(800);
  const n = await page.evaluate(() => { const s = JSON.parse(localStorage.getItem('cdlws.state.v1') || '{}'); return Object.keys(s.bkt || {}).length; });
  expect(n).toBeGreaterThan(0);
});
