#!/bin/sh
# Build a minimal Taco.app around a signed debug taco Mach-O.
# Copies the binary. Does not codesign, replace, or signal the source.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
src=${1:-$root/target/debug/taco}
dest=${2:-$root/packaging/Taco.app}
plist=$root/packaging/Info.plist
if [ ! -f "$src" ]; then
  echo "missing binary: $src" >&2
  echo "build first: cargo build && ./scripts/sign-debug.sh" >&2
  exit 2
fi
if [ ! -f "$plist" ]; then
  echo "missing plist: $plist" >&2
  exit 2
fi
src_real=$(CDPATH= cd -- "$(dirname "$src")" && pwd)/$(basename "$src")
dest_bin=$dest/Contents/MacOS/taco
if [ -e "$dest_bin" ]; then
  dest_real=$(CDPATH= cd -- "$(dirname "$dest_bin")" && pwd)/$(basename "$dest_bin")
  if [ "$src_real" = "$dest_real" ]; then
    echo "refusing to pack the bundle executable over itself: $src_real" >&2
    exit 2
  fi
fi
rm -rf "$dest"
mkdir -p "$dest/Contents/MacOS"
cp -p "$src" "$dest_bin"
chmod 755 "$dest_bin"
cp "$plist" "$dest/Contents/Info.plist"
# Sign the copy inside the bundle, then the bundle. Never "$src".
codesign --force --sign - --identifier ai.syndai.taco "$dest_bin"
codesign --force --sign - --identifier ai.syndai.taco "$dest"
echo "ok: $dest"
codesign -dv --verbose=4 "$dest"
