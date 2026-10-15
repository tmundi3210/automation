#!/usr/bin/env bash
# Installs protoc into tools/protoc/ (git-ignored). libsignal-protocol's build.rs needs it.
# .cargo/config.toml points PROTOC at tools/protoc/bin/protoc.
set -euo pipefail
cd "$(dirname "$0")/.."
VER="${PROTOC_VERSION:-29.3}"
if [ -x tools/protoc/bin/protoc ]; then
  echo "protoc already present: $(tools/protoc/bin/protoc --version)"; exit 0
fi
case "$(uname -s)-$(uname -m)" in
  Linux-x86_64)  ASSET="protoc-${VER}-linux-x86_64.zip" ;;
  Linux-aarch64) ASSET="protoc-${VER}-linux-aarch_64.zip" ;;
  Darwin-arm64)  ASSET="protoc-${VER}-osx-aarch_64.zip" ;;
  Darwin-x86_64) ASSET="protoc-${VER}-osx-x86_64.zip" ;;
  *) echo "unsupported platform; install protoc yourself and set PROTOC" >&2; exit 1 ;;
esac
URL="https://github.com/protocolbuffers/protobuf/releases/download/v${VER}/${ASSET}"
TMP="$(mktemp -d)"
echo "downloading $URL"
curl -sSL -o "$TMP/protoc.zip" "$URL"
mkdir -p tools/protoc
python3 -c "import sys,zipfile; zipfile.ZipFile(sys.argv[1]).extractall(sys.argv[2])" "$TMP/protoc.zip" tools/protoc
chmod +x tools/protoc/bin/protoc
rm -rf "$TMP"
tools/protoc/bin/protoc --version
