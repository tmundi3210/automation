// Render a guide's HTML to PDF (Chromium). Usage: node scripts/study-guide-pdf.mjs <out.pdf> [guide.html] [footer title]
import { chromium } from 'playwright';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const html = resolve(process.argv[3] ?? resolve(import.meta.dirname, '../../study-guide/guide.html'));
const title = process.argv[4] ?? 'CA CDL · General Knowledge + Combination study guide';
const out = resolve(process.argv[2] ?? resolve(import.meta.dirname, '../../study-guide/guide.pdf'));
const browser = await chromium.launch();
const page = await browser.newPage();
await page.goto(pathToFileURL(html).href, { waitUntil: 'networkidle' });
await page.evaluate(() => document.fonts.ready);
await page.pdf({
  path: out, format: 'Letter', printBackground: true, preferCSSPageSize: true, outline: true, tagged: true,
  displayHeaderFooter: true, headerTemplate: '<span></span>',
  footerTemplate: `<div style="width:100%;font-size:8px;color:#4b5a51;padding:0 0.6in;display:flex;justify-content:space-between;font-family:sans-serif"><span>${title}</span><span><span class="pageNumber"></span> / <span class="totalPages"></span></span></div>`,
});
await browser.close();
console.log('pdf', out);
