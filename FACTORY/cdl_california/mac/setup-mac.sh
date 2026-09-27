#!/usr/bin/env bash
# CDL Workshop CA — one-step setup for macOS.
#   bash setup-mac.sh            install/update, build, put launchers on the Desktop, open the app
#   bash setup-mac.sh --no-open  same, but do not start/open it
# Safe to run again any time (it also picks up updates if this folder is a git checkout).
set -euo pipefail

OPEN=1; [[ "${1:-}" == "--no-open" ]] && OPEN=0
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP="$(cd "$HERE/../app" && pwd)"
DESK="${CDLWS_DESKTOP:-$HOME/Desktop}"
PORT="${CDLWS_PORT:-4173}"
say() { printf '\n\033[1;32m==>\033[0m %s\n' "$*"; }
die() { printf '\n\033[1;31mSetup stopped:\033[0m %s\n' "$*" >&2; exit 1; }

if [[ "$(uname -s)" != "Darwin" && -z "${CDLWS_ALLOW_NON_MAC:-}" ]]; then die "this script is for macOS."; fi

# 1) Node.js 20.19+ or 22.12+ (needed to build the app)
need_node() {
  command -v node >/dev/null 2>&1 || return 0
  node -e 'const [a,b]=process.versions.node.split(".").map(Number);process.exit((a>22||(a===22&&b>=12)||(a===20&&b>=19)||a>=23)?1:0)'
}
if need_node; then
  if command -v brew >/dev/null 2>&1; then
    say "Installing Node.js with Homebrew (one time)…"
    brew install node
  else
    die "Node.js 22 is needed. Install the LTS version from https://nodejs.org (the macOS installer), then run this script again."
  fi
fi
say "Node $(node -v) found."

# 2) Update (if this is a git checkout) and install dependencies
if git -C "$HERE" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  say "Checking for updates…"
  git -C "$HERE" pull --ff-only 2>/dev/null || echo "   (no update pulled — continuing with the copy you have)"
fi
cd "$APP"
say "Installing app dependencies (first time takes a minute)…"
npm ci --no-audit --no-fund --loglevel=error

# 3) Build both versions: installable offline app (dist/) and one-file version (dist-single/)
say "Building the app…"
npm run -s build:pwa >/dev/null
npm run -s build:single >/dev/null
[[ -f dist/index.html && -f dist-single/index.html ]] || die "the build did not produce the app files."

# 4) Desktop launchers
mkdir -p "$DESK"
cp dist-single/index.html "$DESK/CDL Workshop (offline file).html"
LAUNCHER="$DESK/Start CDL Workshop.command"
cat > "$LAUNCHER" <<LAUNCH
#!/usr/bin/env bash
# Double-click to start CDL Workshop. Keep this window open while you study; close it to stop.
cd "$APP"
( sleep 2; open "http://localhost:$PORT/" ) &
echo "CDL Workshop is running at http://localhost:$PORT/  (close this window to stop)"
exec npx vite preview --outDir dist --port $PORT --strictPort
LAUNCH
chmod +x "$LAUNCHER"
say "Put two things on your Desktop:"
echo "   • Start CDL Workshop.command  — double-click to open the app (installable, works offline)"
echo "   • CDL Workshop (offline file).html — the whole app in one file; opens in any browser, no setup"

# 5) Open it now
if [[ $OPEN -eq 1 ]]; then
  say "Opening CDL Workshop…"
  open "$LAUNCHER"
fi
say "Done. Your progress is saved in the browser you use; use Settings → resume code to move it."
