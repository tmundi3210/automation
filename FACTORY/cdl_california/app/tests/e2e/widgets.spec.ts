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

// Legibility + touch targets, measured the way evaluators did: widget hosted in its lesson, phone width.
for (const id of IDS) {
  test(`legible at phone width: ${id}`, async ({ page }, info) => {
    test.skip(info.project.name !== 'mobile', 'phone-width check');
    await page.goto(URL + '#widget.' + id);
    await expect(page.locator('.widget')).toBeVisible();
    const r = await page.evaluate(() => {
      const w = document.querySelector('.widget')!;
      const small: string[] = [];
      for (const t of w.querySelectorAll('svg text')) {
        const el = t as SVGTextElement; if (!el.textContent?.trim()) continue;
        const box = el.getBoundingClientRect(); if (box.width === 0 && box.height === 0) continue;
        const svg = el.ownerSVGElement!; const vb = svg.viewBox.baseVal; const scale = vb && vb.width ? svg.getBoundingClientRect().width / vb.width : 1;
        const px = parseFloat(getComputedStyle(el).fontSize) * scale;
        if (px < 10.95) small.push(`${el.textContent!.trim().slice(0, 20)}@${px.toFixed(1)}`);
      }
      const tiny: string[] = [];
      for (const b of w.querySelectorAll('button, [role="button"], input[type="range"], input[type="checkbox"]')) {
        const box = (b as HTMLElement).getBoundingClientRect(); if (!box.width) continue;
        const target = b.closest('label') ? (b.closest('label') as HTMLElement).getBoundingClientRect() : box;
        if (Math.max(target.height, box.height) < 24 || Math.max(target.width, box.width) < 24) tiny.push(`${(b.textContent || b.getAttribute('aria-label') || '').trim().slice(0, 18)}:${box.width.toFixed(0)}x${box.height.toFixed(0)}`);
      }
      return { small: small.slice(0, 6), tiny: tiny.slice(0, 6), overflow: document.documentElement.scrollWidth > innerWidth + 1 };
    });
    expect(r.small, 'SVG text under 11px').toEqual([]);
    expect(r.tiny, 'targets under 24px (WCAG 2.2 AA)').toEqual([]);
    expect(r.overflow).toBe(false);
  });
}
