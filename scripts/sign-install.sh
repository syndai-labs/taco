#!/bin/sh
# Developer ID install for one arm64 Mach-O named taco, identifier ai.syndai.taco.
# Ad-hoc `codesign -s -` and Apple Development are not this install.
# This script exits 2 before signing when no Developer ID Application identity
# is in the keychain. Measured on 2026-10-17: that is the path taken.
# See docs/evidence/m0/signed-install.md.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
bin=${1:-$root/target/release/taco}
profile=${TACO_NOTARY_PROFILE:-taco-notary}

echo "identities:"
security find-identity -v -p codesigning
ident=$(security find-identity -v -p codesigning | sed -n 's/.*"\(Developer ID Application:[^"]*\)".*/\1/p' | head -n 1)
if [ -z "$ident" ]; then
  echo "fail: Developer ID Application: no identity found" >&2
  exit 2
fi
if [ ! -f "$bin" ]; then
  echo "missing binary: $bin" >&2
  exit 2
fi

echo "sign: codesign --force --options runtime --timestamp --sign \"$ident\" --identifier ai.syndai.taco $bin"
codesign --force --options runtime --timestamp --sign "$ident" --identifier ai.syndai.taco "$bin"
codesign -dv --verbose=4 "$bin"

stage=$(mktemp -d "${TMPDIR:-/tmp}/taco-sign-install.XXXXXX")
cp "$bin" "$stage/taco"
ditto -c -k --keepParent "$stage/taco" "$stage/taco.zip"
echo "notary: xcrun notarytool submit $stage/taco.zip --keychain-profile $profile --no-progress --wait"
xcrun notarytool submit "$stage/taco.zip" --keychain-profile "$profile" --no-progress --wait
echo "staple: xcrun stapler staple $bin"
xcrun stapler staple "$bin"
echo "assess: spctl --assess -vv --type execute $bin"
spctl --assess -vv --type execute "$bin"
echo "ok: $bin"
