#!/bin/sh
# M0 residency prove scaffolding (100-cycle phys_footprint band).
# A Metal pass or invented footprint on Linux is forbidden.
# Off Darwin this exits 2 with UNSUPPORTED_ON_LINUX and writes no evidence file.
set -eu

out=${1:-}

fail_linux() {
  echo "UNSUPPORTED_ON_LINUX" >&2
  echo "Residency prove requires macOS Apple Silicon. No phys_footprint was measured." >&2
  echo "ac2=0" >&2
  echo "ac3=0" >&2
  echo "Metal pass evidence was not written." >&2
  if [ -n "$out" ]; then
    {
      echo "id=m0-residency-prove"
      echo "verdict=unsupported_on_linux"
      echo "phys_footprint=none"
      echo "ac2=0"
      echo "ac3=0"
      echo "reason=host is not Darwin; no Metal residency measurement"
    } > "$out"
  fi
  exit 2
}

if [ "$(uname -s 2>/dev/null || echo unknown)" != "Darwin" ]; then
  fail_linux
fi

# Darwin path is not a pass until a Mac worker records a real band measurement.
echo "not_run" >&2
echo "Residency prove scaffolding is present but did not execute a Metal cycle series." >&2
echo "ac2=0" >&2
echo "ac3=0" >&2
if [ -n "$out" ]; then
  {
    echo "id=m0-residency-prove"
    echo "verdict=not_run"
    echo "phys_footprint=none"
    echo "ac2=0"
    echo "ac3=0"
    echo "reason=Mac prove cycle series was not executed"
  } > "$out"
fi
exit 2
