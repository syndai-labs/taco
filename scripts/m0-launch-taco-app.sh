#!/bin/sh
# Launch Taco.app through Launch Services so the mic sheet is attributed to Taco.
# The binary reads TACO_CAPTURE_SPIKE, TACO_CAPTURE_SPIKE_OPEN_MS, and
# TACO_CAPTURE_SPIKE_HOLD_MS. OPEN_MS and HOLD_MS are short names for those two.
# Does not launch unless you run this script.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
app=$root/packaging/Taco.app
if [ "${1:-}" = "--app" ]; then
  app=$2
  shift 2
fi
if [ ! -d "$app" ]; then
  echo "missing app: $app" >&2
  echo "pack first: ./scripts/m0-pack-taco-app.sh" >&2
  exit 2
fi
open_ms=${TACO_CAPTURE_SPIKE_OPEN_MS:-${OPEN_MS:-300000}}
hold_ms=${TACO_CAPTURE_SPIKE_HOLD_MS:-${HOLD_MS:-1800000}}
if [ "$#" -eq 0 ]; then
  set -- run --source default --from en --to zh
fi
exec open -W \
  --env TACO_CAPTURE_SPIKE=1 \
  --env "TACO_CAPTURE_SPIKE_OPEN_MS=$open_ms" \
  --env "TACO_CAPTURE_SPIKE_HOLD_MS=$hold_ms" \
  "$app" --args "$@"
