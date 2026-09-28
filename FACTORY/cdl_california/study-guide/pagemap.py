"""Find the page each TOC entry starts on (pass 1 PDF) -> JSON map for build.py."""
import json, re, sys, pymupdf as fitz
toc = json.load(open(sys.argv[2]))
doc = fitz.open(sys.argv[1])
texts = [' '.join(p.get_text().split()) for p in doc]
start, out = 2, {}
for lvl, sid, title in toc:
    key = title.split('. ', 1)[1] if title[:1].isdigit() and '. ' in title[:4] else title
    if sid.startswith('L-'):
        key = sid[2:] + ' · '
    key = re.sub(r'^Part \d — ', '', key)
    probe = ' '.join(key.split())[:40]
    for i in range(start, len(texts)):
        if probe in texts[i]:
            out[sid] = i + 1; start = i; break
    else:
        print('NOT FOUND', sid, probe, file=sys.stderr)
print(json.dumps(out))
