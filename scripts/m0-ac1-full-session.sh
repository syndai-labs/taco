#!/bin/sh
# One Mac session for M0-AC1: pack, mic Allow-then-Deny, AudioCapture, then process-tap.
# Process-tap runs only after the mic watch is gone and the AudioCapture pair completed.
# That system sheet is the process-tap TCC prompt. after-pair does not open system again.
# The watcher does not click. TIMEOUT is not Deny. Drop is not a Cleanup tip SHA.
# Off macOS --arm exits 2 with platform_unsupported. That is not a CoreAudio result.
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
BUNDLE=ai.syndai.taco

fail_closed() {
  echo "platform_unsupported" >&2
  echo "AC1 full session runs only on macOS. No CoreAudio result was produced." >&2
  exit 2
}

usage() {
  echo "usage: $0 {self-check|instructions|classify|plan|--arm}" >&2
  exit 2
}

darwin() {
  [ "$(uname -s 2>/dev/null || echo unknown)" = Darwin ]
}

live_mic() {
  ps aux 2>/dev/null | awk '/[t]aco run --source default/ { found=1 } END { exit found ? 0 : 1 }'
}

classify() {
  row=$1
  pair=$2
  if [ "$row" = yes ] && [ "$pair" = granted ]; then
    echo Allow
    return 0
  fi
  if [ "$row" = yes ] && [ "$pair" = denied ]; then
    echo Deny
    return 0
  fi
  # No service row is not Deny, including a pair-query that prints denied.
  echo TIMEOUT
}

plan() {
  case $1 in
    Allow) echo reset-and-prompt-deny ;;
    Deny) echo reset-and-prompt-allow ;;
    *) echo stop ;;
  esac
}

may_open_again() {
  [ "$(plan "$1")" != stop ]
}

pair_complete() {
  if [ "$1" = Allow ] && [ "$2" = Deny ]; then
    return 0
  fi
  if [ "$1" = Deny ] && [ "$2" = Allow ]; then
    return 0
  fi
  return 1
}

audio_row() {
  if tccutil list -s AudioCapture 2>/dev/null | grep -F "$BUNDLE" >/dev/null 2>&1; then
    echo yes
  else
    echo no
  fi
}

audio_pair() {
  raw=$(tccutil list -s AudioCapture -b "$BUNDLE" 2>/dev/null | tr -d '[:space:]' || true)
  case $raw in
    *granted*) echo granted ;;
    *denied*) echo denied ;;
    *) echo empty ;;
  esac
}

instructions() {
  cat <<EOF
M0-AC1 full session. Watcher does not click.
TIMEOUT is not Deny. A pair-query denied with no service row is not Deny.
Allow tip SHA: none
Deny tip SHA: none
Cleanup tip SHA: none
M0-AC1 stays unchecked.
M0-AC1 stays unchecked while Allow, Deny, or Cleanup tip SHA is still none.

1. Pack Taco.app (scripts/m0-pack-taco-app.sh).
2. Mic Allow, then Don't Allow (scripts/m0-ac1-mac-resume.sh --arm).
3. After mic LIVE is gone, AudioCapture (scripts/m0-audiocapture-measure.sh).
Click Allow on the first AudioCapture sheet, then Don't Allow on the next.
If that first window has no AudioCapture service row, it is TIMEOUT, not Deny, and this script does not open AudioCapture again.
4. After that pair, and only if mic LIVE is still gone, process-tap (scripts/m0-process-tap-measure.sh after-pair).
Not prompting system again. Not resetting TCC.
The app source is app:com.google.Chrome. A missing sheet is the same service, not Deny.
Exit 0 is not a Cleanup tip SHA. Drop is not a Cleanup tip SHA.
EOF
}

prompt_audio() {
  click=$1
  if live_mic; then
    echo "mic LIVE: not starting AudioCapture. Do not kill that pid. Do not reset TCC." >&2
    echo TIMEOUT
    return 0
  fi
  echo "tccutil reset AudioCapture $BUNDLE" >&2
  if ! tccutil reset AudioCapture "$BUNDLE" >&2; then
    echo "reset failed. Not starting another AudioCapture open." >&2
    echo TIMEOUT
    return 0
  fi
  echo "Click $click. Watcher does not click." >&2
  set +e
  # Keep the measure transcript off this function's stdout. The caller
  # captures only the classify word (Allow, Deny, or TIMEOUT).
  sh "$root/scripts/m0-audiocapture-measure.sh" system >&2
  launch_rc=$?
  set -e
  echo "process_exit=$launch_rc is not an outcome. TIMEOUT is not Deny." >&2
  if [ "$launch_rc" -eq 2 ] || [ "$launch_rc" -eq 3 ]; then
    echo "measure refused ($launch_rc). Not Allow. Not Deny." >&2
    echo TIMEOUT
    return 0
  fi
  classify "$(audio_row)" "$(audio_pair)"
}

finish_unchecked() {
  echo "Allow tip SHA: none" >&2
  echo "Deny tip SHA: none" >&2
  echo "Cleanup tip SHA: none" >&2
  echo "M0-AC1 stays unchecked." >&2
  echo "M0-AC1 stays unchecked while Allow, Deny, or Cleanup tip SHA is still none." >&2
}

arm() {
  if ! darwin; then
    echo "recorded: none" >&2
    finish_unchecked
    fail_closed
  fi
  instructions >&2
  sh "$root/scripts/m0-pack-taco-app.sh"
  set +e
  sh "$root/scripts/m0-ac1-mac-resume.sh" --arm
  mic_rc=$?
  set -e
  echo "mic resume status $mic_rc. TIMEOUT is not Deny." >&2
  if live_mic; then
    echo "mic LIVE: not starting AudioCapture. Do not kill that pid. Do not reset TCC." >&2
    finish_unchecked
    exit 3
  fi
  set +e
  sh "$root/scripts/m0-audiocapture-measure.sh" gate
  gate_rc=$?
  set -e
  if [ "$gate_rc" -ne 0 ]; then
    echo "not starting AudioCapture." >&2
    finish_unchecked
    exit "$gate_rc"
  fi
  first=$(prompt_audio Allow)
  echo "audiocapture 1 outcome $first" >&2
  if ! may_open_again "$first"; then
    echo "stop: $first does not start another AudioCapture open" >&2
    echo "TIMEOUT is not Deny." >&2
    finish_unchecked
    if [ "$mic_rc" -ne 0 ]; then
      exit "$mic_rc"
    fi
    exit 3
  fi
  case $(plan "$first") in
    reset-and-prompt-deny) next_click="Don't Allow" ;;
    reset-and-prompt-allow) next_click=Allow ;;
    *)
      echo "stop: $first does not start another AudioCapture open" >&2
      finish_unchecked
      exit 3
      ;;
  esac
  second=$(prompt_audio "$next_click")
  echo "audiocapture 2 outcome $second" >&2
  case $second in
    TIMEOUT|UNPROVEN)
      echo "stop: $second does not start another AudioCapture open" >&2
      echo "TIMEOUT is not Deny." >&2
      finish_unchecked
      if [ "$mic_rc" -ne 0 ]; then
        exit "$mic_rc"
      fi
      exit 3
      ;;
  esac
  if pair_complete "$first" "$second"; then
    echo "both AudioCapture outcomes observed" >&2
    echo "system process-tap TCC sheet is that AudioCapture pair. Not prompting system again." >&2
    if live_mic; then
      echo "mic LIVE: not starting process-tap measure. Do not kill that pid. Do not reset TCC." >&2
      finish_unchecked
      exit 3
    fi
    set +e
    sh "$root/scripts/m0-process-tap-measure.sh" after-pair
    tap_rc=$?
    set -e
    echo "process-tap after-pair status $tap_rc. TIMEOUT is not Deny. Exit 0 is not a Cleanup tip SHA." >&2
    finish_unchecked
    if [ "$tap_rc" -ne 0 ]; then
      exit "$tap_rc"
    fi
    exit "$mic_rc"
  fi
  echo "AC1 still open: need both Allow and Deny" >&2
  echo "stop: not starting a third AudioCapture open" >&2
  finish_unchecked
  exit 3
}

self_check() {
  [ "$(classify no denied)" = TIMEOUT ]
  [ "$(classify no empty)" = TIMEOUT ]
  [ "$(classify yes denied)" = Deny ]
  [ "$(classify yes granted)" = Allow ]
  [ "$(plan TIMEOUT)" = stop ]
  [ "$(plan UNPROVEN)" = stop ]
  [ "$(plan Allow)" = reset-and-prompt-deny ]
  [ "$(plan Deny)" = reset-and-prompt-allow ]
  text=$(instructions)
  echo "$text" | grep -q "Watcher does not click."
  echo "$text" | grep -q "TIMEOUT is not Deny."
  echo "$text" | grep -q "m0-pack-taco-app.sh"
  echo "$text" | grep -q "m0-ac1-mac-resume.sh --arm"
  echo "$text" | grep -q "m0-audiocapture-measure.sh"
  echo "$text" | grep -q "Allow tip SHA: none"
  echo "$text" | grep -q "Deny tip SHA: none"
  echo "$text" | grep -q "Cleanup tip SHA: none"
  echo "$text" | grep -q "M0-AC1 stays unchecked."
  echo "$text" | grep -q "M0-AC1 stays unchecked while Allow, Deny, or Cleanup tip SHA is still none."
  echo "$text" | grep -q "m0-process-tap-measure.sh after-pair"
  echo "$text" | grep -q "Not prompting system again."
  echo "$text" | grep -q "Drop is not a Cleanup tip SHA."
  echo "self-check ok"
}

cmd=${1:-}
case $cmd in
  self-check) self_check ;;
  instructions) instructions ;;
  classify)
    [ "$#" -eq 3 ] || usage
    classify "$2" "$3"
    ;;
  plan)
    [ "$#" -eq 2 ] || usage
    plan "$2"
    ;;
  --arm) arm ;;
  *) usage ;;
esac
