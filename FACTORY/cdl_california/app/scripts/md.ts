// Minimal, dependency-free markdown → HTML renderer for the lesson corpus.
// Supports: paragraphs, #-headings (h3–h5 inside lessons), bold/italic/inline code,
// nested bullet + numbered lists (2-space indent), pipe tables, blockquotes, fenced code.
// Content is trusted (our own lesson files) but we still escape HTML.

export function esc(s: string): string {
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}

export function inline(s: string): string {
  let out = esc(s);
  out = out.replace(/`([^`]+)`/g, '<code>$1</code>');
  out = out.replace(/\*\*([^*]+?)\*\*/g, '<strong>$1</strong>');
  out = out.replace(/(^|[^*\w])\*([^*\s][^*]*?)\*(?!\w)/g, '$1<em>$2</em>');
  out = out.replace(/(^|[\s(])_([^_\s][^_]*?)_(?=[\s).,;:]|$)/g, '$1<em>$2</em>');
  // [CA] tag → badge
  out = out.replace(/\[CA\]/g, '<span class="ca-tag" title="California-specific rule">CA</span>');
  return out;
}

/** Plain text (strip markdown emphasis) — used for search, TTS, comparisons. */
export function plain(s: string): string {
  return s.replace(/\*\*([^*]+?)\*\*/g, '$1').replace(/(^|[^*])\*([^*]+?)\*/g, '$1$2').replace(/`([^`]+)`/g, '$1').replace(/\s+/g, ' ').trim();
}

function splitRow(line: string): string[] {
  let t = line.trim();
  if (t.startsWith('|')) t = t.slice(1);
  if (t.endsWith('|')) t = t.slice(0, -1);
  return t.split('|').map((c) => c.trim());
}

export function isTableSep(line: string): boolean {
  return /^\s*\|?\s*:?-{2,}:?\s*(\|\s*:?-{2,}:?\s*)*\|?\s*$/.test(line);
}

export function parseTable(lines: string[]): { head: string[]; rows: string[][] } {
  const head = splitRow(lines[0]);
  const rows = lines.slice(2).map(splitRow);
  return { head, rows };
}

function renderTable(lines: string[]): string {
  const { head, rows } = parseTable(lines);
  const th = head.map((h) => `<th>${inline(h)}</th>`).join('');
  const tb = rows.map((r) => `<tr>${r.map((c) => `<td>${inline(c)}</td>`).join('')}</tr>`).join('');
  return `<div class="tbl" tabindex="0" role="region" aria-label="Table (scrolls sideways)"><table><thead><tr>${th}</tr></thead><tbody>${tb}</tbody></table></div>`;
}

type LItem = { indent: number; ordered: boolean; text: string[]; children: LItem[] };

function renderList(lines: string[]): string {
  // Build a tree by indent
  const root: LItem = { indent: -1, ordered: false, text: [], children: [] };
  const stack: LItem[] = [root];
  for (const raw of lines) {
    const m = raw.match(/^(\s*)([-*]|\d+[.)])\s+(.*)$/);
    if (m) {
      const indent = m[1].length;
      const item: LItem = { indent, ordered: /\d/.test(m[2]), text: [m[3]], children: [] };
      while (stack.length > 1 && stack[stack.length - 1].indent >= indent) stack.pop();
      stack[stack.length - 1].children.push(item);
      stack.push(item);
    } else {
      // continuation line
      const cur = stack[stack.length - 1];
      if (cur !== root) cur.text.push(raw.trim());
    }
  }
  const emit = (items: LItem[]): string => {
    if (!items.length) return '';
    const ordered = items[0].ordered;
    const tag = ordered ? 'ol' : 'ul';
    return `<${tag}>${items.map((it) => `<li>${it.text.map(inline).join('<br>')}${emit(it.children)}</li>`).join('')}</${tag}>`;
  };
  return emit(root.children);
}

export function render(md: string): string {
  const lines = md.replace(/\r/g, '').split('\n');
  const out: string[] = [];
  let i = 0;
  while (i < lines.length) {
    const line = lines[i];
    if (!line.trim()) { i++; continue; }
    if (/^```/.test(line)) {
      const buf: string[] = [];
      i++;
      while (i < lines.length && !/^```/.test(lines[i])) buf.push(lines[i++]);
      i++;
      out.push(`<pre class="diagram"><code>${esc(buf.join('\n'))}</code></pre>`);
      continue;
    }
    const h = line.match(/^(#{3,6})\s+(.*)$/);
    if (h) { const lvl = Math.min(6, h[1].length + 1); out.push(`<h${lvl}>${inline(h[2])}</h${lvl}>`); i++; continue; }
    if (/^\s*\|/.test(line) && i + 1 < lines.length && isTableSep(lines[i + 1])) {
      const buf: string[] = [];
      while (i < lines.length && /^\s*\|/.test(lines[i])) buf.push(lines[i++]);
      out.push(renderTable(buf));
      continue;
    }
    if (/^>/.test(line)) {
      const buf: string[] = [];
      while (i < lines.length && /^>/.test(lines[i])) buf.push(lines[i++].replace(/^>\s?/, ''));
      const txt = buf.join('\n');
      const cls = /beyond the handbook/i.test(txt) ? 'callout beyond' : /handbook (vs|conflict)|wording note/i.test(txt) ? 'callout conflict' : 'callout';
      out.push(`<blockquote class="${cls}">${render(txt)}</blockquote>`);
      continue;
    }
    if (/^\s*([-*]|\d+[.)])\s+/.test(line)) {
      const buf: string[] = [];
      while (i < lines.length && lines[i].trim() && (/^\s*([-*]|\d+[.)])\s+/.test(lines[i]) || /^\s{2,}\S/.test(lines[i]))) buf.push(lines[i++]);
      out.push(renderList(buf));
      continue;
    }
    // paragraph
    const buf: string[] = [];
    while (i < lines.length && lines[i].trim() && !/^(#{3,6}\s|>|```|\s*\|)/.test(lines[i]) && !/^\s*([-*]|\d+[.)])\s+/.test(lines[i])) buf.push(lines[i++]);
    if (!buf.length) { buf.push(lines[i++]); }
    out.push(`<p>${buf.map(inline).join(' ')}</p>`);
  }
  return out.join('\n');
}
