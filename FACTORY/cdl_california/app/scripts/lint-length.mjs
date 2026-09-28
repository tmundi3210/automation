// Answer-length cue check: share of multiple-choice items whose key is the uniquely longest option.
// Chance is ~33%; lessons above 40% are flagged. Usage: node scripts/lint-length.mjs [LESSON-PREFIX]
import { readFileSync } from 'node:fs';
const C = JSON.parse(readFileSync(new URL('../src/content/content.json', import.meta.url)));
const only = process.argv[2] || '';
const by = {};
for (const it of Object.values(C.items)) {
  if (it.options.length !== 3 || !it.lesson.startsWith(only)) continue;
  const L = it.optionsText.map((o) => o.length); const k = L[it.key];
  const uniq = L.filter((x, i) => i !== it.key).every((x) => k > x * 1.15);
  const b = (by[it.lesson] ??= { n: 0, longest: 0, ids: [] }); b.n++; if (uniq) { b.longest++; b.ids.push(it.id); }
}
let bad = 0;
for (const [l, b] of Object.entries(by).sort()) { const p = b.longest / b.n; if (p > 0.4) bad++; console.log(`${l} ${b.longest}/${b.n} = ${(p * 100).toFixed(0)}%${p > 0.4 ? '  ← over 40%' : ''}${process.env.IDS ? '\n  ' + b.ids.join(' ') : ''}`); }
process.exitCode = bad ? 1 : 0;
