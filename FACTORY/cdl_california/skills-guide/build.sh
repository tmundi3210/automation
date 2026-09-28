#!/usr/bin/env bash
# Two-pass build: HTML -> PDF -> TOC page numbers -> final PDF. Needs app/node_modules and PyMuPDF.
set -euo pipefail
cd "$(dirname "$0")"
PY=${PY:-python3}
OUT=CA-CDL-Skills-Test-Behind-the-Wheel-Guide.pdf
TITLE='CA CDL · Skills test (behind the wheel) guide'
TMP=$(mktemp -d)
python3 build.py
(cd ../app && node scripts/study-guide-pdf.mjs "$TMP/pass1.pdf" "$PWD/../skills-guide/guide.html" "$TITLE")
python3 build.py "$($PY ../study-guide/pagemap.py "$TMP/pass1.pdf" toc.json)"
(cd ../app && node scripts/study-guide-pdf.mjs "$PWD/../skills-guide/$OUT" "$PWD/../skills-guide/guide.html" "$TITLE")
rm -rf "$TMP"
echo "built $OUT"
