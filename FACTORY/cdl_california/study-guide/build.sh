#!/usr/bin/env bash
# Two-pass build: HTML -> PDF -> find TOC page numbers -> HTML with numbers -> final PDF.
# Needs: app/node_modules (npm ci in app/) and PyMuPDF (pip install pymupdf) for the page map.
set -euo pipefail
cd "$(dirname "$0")"
PY=${PY:-python3}
OUT=CA-CDL-General-Knowledge-and-Combination-Study-Guide.pdf
TMP=$(mktemp -d)
python3 build.py
(cd ../app && node scripts/study-guide-pdf.mjs "$TMP/pass1.pdf")
python3 build.py "$($PY pagemap.py "$TMP/pass1.pdf" toc.json)"
(cd ../app && node scripts/study-guide-pdf.mjs "$PWD/../study-guide/$OUT")
rm -rf "$TMP"
echo "built $OUT"
