#!/bin/sh
# Re-sign the debug taco binary so codesign binds the embedded Info.plist.
# Linker-signed builds still report Info.plist=not bound even when
# __TEXT,__info_plist is present. Measured on macOS arm64 at 4969bf6:
# after this script, codesign -dv reports Info.plist entries=5 and
# Identifier=ai.syndai.taco.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
bin=${1:-$root/target/debug/taco}
if [ ! -f "$bin" ]; then
  echo "missing binary: $bin" >&2
  echo "build first: cargo build" >&2
  exit 2
fi
codesign --force -s - --identifier ai.syndai.taco "$bin"
codesign -dv --verbose=4 "$bin" 2>&1 | tee /tmp/taco-sign-debug.txt
if grep -q 'Info.plist=not bound' /tmp/taco-sign-debug.txt; then
  echo "fail: Info.plist still not bound" >&2
  exit 1
fi
if ! grep -q 'Info.plist entries=' /tmp/taco-sign-debug.txt; then
  echo "fail: expected Info.plist entries=N" >&2
  exit 1
fi
echo "ok: plist bound for $bin"
