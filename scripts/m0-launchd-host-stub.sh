#!/bin/sh
# Alternate terminal host entry. launchd execs this stub; the stub execs taco.
# The command is fixed: notices. No capture, no microphone, no model fetch.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
bin="$root/target/debug/taco"
case "$bin" in
  *taco-m0-tap*)
    echo "m0-launchd-host-stub: refused tap worktree" >&2
    exit 126
    ;;
esac
if [ ! -x "$bin" ]; then
  echo "m0-launchd-host-stub: missing $bin" >&2
  exit 127
fi
exec "$bin" notices
