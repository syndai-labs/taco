#!/bin/sh
# AudioCapture Allow-then-Deny measure for bundle ai.syndai.taco.
# Launch is `open -W` on Taco.app, the same attribution path as the mic harness.
# This script does not click, does not reset TCC, and does not name Allow or Deny.
# Off macOS it exits 2 with platform_unsupported. That is not a CoreAudio result.
set -eu

fail_closed() {
  echo "platform_unsupported" >&2
  echo "AudioCapture measure runs only on macOS. No CoreAudio result was produced." >&2
  exit 2
}

if [ "$(uname -s 2>/dev/null || echo unknown)" != "Darwin" ]; then
  fail_closed
fi

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
# Same defaults as the mic harness (#50): OPEN_MS 300000, HOLD_MS 1800000.
# The tap window is OPEN_MS. HOLD_MS is the mic hold on this bundle; the tap
# path does not read it. Passing both keeps the Taco.app launch identical.
open_ms=${TACO_CAPTURE_SPIKE_OPEN_MS:-${OPEN_MS:-300000}}
hold_ms=${TACO_CAPTURE_SPIKE_HOLD_MS:-${HOLD_MS:-1800000}}

mic_live() {
  ps aux | awk '/[t]aco run --source default/ { print }'
}

usage() {
  echo "usage: $0 gate | system | app:BUNDLE | pid:PID" >&2
  echo "Set TACO_APP to Taco.app (Identifier ai.syndai.taco)." >&2
  echo "TIMEOUT is not Deny. This script does not record an outcome." >&2
  exit 2
}

case "$open_ms" in
  ''|*[!0-9]*)
    echo "OPEN_MS must be a whole number (default 300000)" >&2
    exit 2
    ;;
esac
if [ "$open_ms" -lt 1 ] || [ "$open_ms" -gt 600000 ]; then
  echo "OPEN_MS must be 1..600000 for an Allow/Deny window (default 300000)" >&2
  echo "0 starts and stops without waiting and is not a measure." >&2
  exit 2
fi

live=$(mic_live || true)
if [ -n "$live" ]; then
  echo "mic LIVE: a taco run --source default process is still up." >&2
  echo "Do not start AudioCapture. Do not kill that pid. Do not reset TCC." >&2
  printf '%s\n' "$live" >&2
  exit 3
fi

cmd=${1:-}
if [ "$cmd" = "gate" ] || [ -z "$cmd" ]; then
  echo "mic settled. No taco run --source default process."
  echo "Next: snapshot Microphone, reset AudioCapture only, then:"
  echo "  $0 system"
  echo "Josh clicks Allow, then the opposite run clicks Don't Allow."
  echo "TIMEOUT is not Deny. Copy the blank record. Do not check M0-AC1."
  exit 0
fi

case "$cmd" in
  system|app:*|pid:*) ;;
  *) usage ;;
esac

app=${TACO_APP:-$root/packaging/Taco.app}
if [ ! -d "$app" ]; then
  echo "missing Taco.app: $app" >&2
  echo "Pack this tip first: ./scripts/m0-pack-taco-app.sh" >&2
  echo "The mic-watch app may predate AudioDeviceStart. Do not launch the debug binary." >&2
  exit 2
fi

ident=$(codesign -dv "$app" 2>&1 | awk -F= '/^Identifier=/{print $2; exit}')
if [ "$ident" != "ai.syndai.taco" ]; then
  echo "refusing launch: Identifier=${ident:-missing} (want ai.syndai.taco)" >&2
  exit 2
fi

plist=$app/Contents/Info.plist
if [ ! -f "$plist" ] || ! grep -a -q "NSAudioCaptureUsageDescription" "$plist"; then
  echo "refusing launch: Taco.app has no NSAudioCaptureUsageDescription" >&2
  exit 2
fi

echo "before the click, copy:"
echo "  tccutil list -s AudioCapture"
echo "  tccutil list -s AudioCapture -b ai.syndai.taco"
echo "  tccutil list -s Microphone"
echo "  tccutil list -s Microphone -b ai.syndai.taco"
echo "launch source=$cmd open_ms=$open_ms hold_ms=$hold_ms bundle=ai.syndai.taco"
echo "Dialog usage string must be: taco captures one system or application audio source for local subtitles."
echo "If the string is the microphone usage, leave it untouched. This script does not click."

set +e
open -W \
  --env TACO_CAPTURE_SPIKE=1 \
  --env "TACO_CAPTURE_SPIKE_OPEN_MS=$open_ms" \
  --env "TACO_CAPTURE_SPIKE_HOLD_MS=$hold_ms" \
  "$app" --args run --source "$cmd" --from en --to zh
rc=$?
set -e

echo "process_exit=$rc"
echo "process_exit is not an outcome. Exit 0 is not Allow. Exit 1 is not Deny."
echo "After exit, copy tccutil again. Fill the record by hand:"
cat <<EOF
outcome:
bundle: ai.syndai.taco
source: $cmd
open_ms: $open_ms
hold_ms: $hold_ms
click:
tcc_list:
tcc_pair:
row_present:
pid:
tip_sha:
EOF
echo "TIMEOUT is not Deny."
echo "Allow: click Allow, row present, pair granted."
echo "Deny: click Don't Allow, row present, pair denied."
echo "TIMEOUT: window ended, no click. Pair denied with no row is not Deny."
exit "$rc"
