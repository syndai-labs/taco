#!/bin/sh
# Process-tap Allow, Deny, and Cleanup measure for bundle ai.syndai.taco.
# Launch is `open -W` on Taco.app: product `taco run` → session::start_capture.
# `system` is open_system_tap. `app:` is open_bundle_tap. `pid:` is open_pid_tap.
# The watcher does not click. TIMEOUT is not Deny. No service row is not Deny.
# Exit 0 is not Allow. Drop is not a Cleanup tip SHA.
# Off macOS this exits 2 with platform_unsupported. That is not a CoreAudio result.
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
BUNDLE=ai.syndai.taco
# Documented HAL client (process-tap.md). Terminal is not on that list.
APP_HAL=app:com.google.Chrome
# Charter pins (#50). Inherited OPEN_MS / HOLD_MS are ignored.
OPEN_MS=300000
HOLD_MS=1800000
export TACO_CAPTURE_SPIKE_OPEN_MS=$OPEN_MS
export TACO_CAPTURE_SPIKE_HOLD_MS=$HOLD_MS

fail_closed() {
  echo "platform_unsupported" >&2
  echo "Process-tap measure runs only on macOS. No CoreAudio result was produced." >&2
  echo "Allow tip SHA: none" >&2
  echo "Deny tip SHA: none" >&2
  echo "Cleanup tip SHA: none" >&2
  echo "M0-AC1 stays unchecked while Allow, Deny, or Cleanup tip SHA is still none." >&2
  exit 2
}

if [ "$(uname -s 2>/dev/null || echo unknown)" != "Darwin" ]; then
  fail_closed
fi

usage() {
  echo "usage: $0 {gate|system|app:BUNDLE|pid:PID|after-pair|--arm}" >&2
  echo "TIMEOUT is not Deny. No service row is not Deny. Watcher does not click." >&2
  echo "Drop is not a Cleanup tip SHA." >&2
  exit 2
}

mic_live() {
  ps aux 2>/dev/null | awk '/[t]aco run --source default/ { found=1 } END { exit found ? 0 : 1 }'
}

refuse_mic() {
  if mic_live; then
    echo "mic LIVE: a taco run --source default process is still up." >&2
    echo "Do not start a process tap. Do not kill that pid. Do not reset TCC." >&2
    ps aux 2>/dev/null | awk '/[t]aco run --source default/ { print }' >&2 || true
    exit 3
  fi
}

tip_none() {
  echo "Allow tip SHA: none"
  echo "Deny tip SHA: none"
  echo "Cleanup tip SHA: none"
  echo "M0-AC1 stays unchecked while Allow, Deny, or Cleanup tip SHA is still none."
  echo "Exit 0 is not Allow. Exit 0 is not a Cleanup tip SHA. Drop is not a Cleanup tip SHA."
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

launch_tap() {
  cmd=$1
  case $cmd in
    system|app:*|pid:*) ;;
    *) usage ;;
  esac
  app=${TACO_APP:-$root/packaging/Taco.app}
  if [ ! -d "$app" ]; then
    echo "missing Taco.app: $app" >&2
    echo "Pack this tip first: ./scripts/m0-pack-taco-app.sh" >&2
    return 2
  fi
  ident=$(codesign -dv "$app" 2>&1 | awk -F= '/^Identifier=/{print $2; exit}')
  if [ "$ident" != "$BUNDLE" ]; then
    echo "refusing launch: Identifier=${ident:-missing} (want $BUNDLE)" >&2
    return 2
  fi
  plist=$app/Contents/Info.plist
  if [ ! -f "$plist" ] || ! grep -a -q "NSAudioCaptureUsageDescription" "$plist"; then
    echo "refusing launch: Taco.app has no NSAudioCaptureUsageDescription" >&2
    return 2
  fi
  echo "Watcher does not click."
  echo "before the click, copy:"
  echo "  tccutil list -s AudioCapture"
  echo "  tccutil list -s AudioCapture -b $BUNDLE"
  echo "  tccutil list -s Microphone"
  echo "  tccutil list -s Microphone -b $BUNDLE"
  echo "launch source=$cmd open_ms=$OPEN_MS hold_ms=$HOLD_MS bundle=$BUNDLE"
  echo "Dialog usage string must be: taco captures one system or application audio source for local subtitles."
  echo "If the string is the microphone usage, leave it untouched."
  echo "TIMEOUT is not Deny. No service row is not Deny."
  echo "capture_not_available is not Deny. A Terminal miss is not a HAL client."
  set +e
  open -W \
    --env TACO_CAPTURE_SPIKE=1 \
    --env "TACO_CAPTURE_SPIKE_OPEN_MS=$OPEN_MS" \
    --env "TACO_CAPTURE_SPIKE_HOLD_MS=$HOLD_MS" \
    "$app" --args run --source "$cmd" --from en --to zh
  rc=$?
  set -e
  echo "process_exit=$rc"
  echo "process_exit is not an outcome. Exit 0 is not Allow. Exit 1 is not Deny."
  echo "Exit 0 is not a Cleanup tip SHA. Drop is not a Cleanup tip SHA."
  echo "AudioDeviceStop runs inside taco_process_tap_open and is not a separate log."
  echo "Drop calls AudioHardwareDestroyAggregateDevice and AudioHardwareDestroyProcessTap."
  echo "Those calls are not traced here. destroy_traced stays no."
  cat <<EOF
outcome:
bundle: $BUNDLE
source: $cmd
open_ms: $OPEN_MS
hold_ms: $HOLD_MS
click:
tcc_list:
tcc_pair:
row_present:
process_exit: $rc
destroy_traced: no
drop_is_cleanup: no
cleanup_tip_sha: none
tip_sha:
EOF
  return "$rc"
}

# Standalone system Allow, then Don't Allow. Resets AudioCapture only while mic is gone.
# The full session must not call this after its AudioCapture pair. It calls after_pair.
prompt_system() {
  click=$1
  if mic_live; then
    echo "mic LIVE: not resetting TCC. Do not kill that pid." >&2
    echo TIMEOUT
    return 0
  fi
  echo "tccutil reset AudioCapture $BUNDLE" >&2
  if ! tccutil reset AudioCapture "$BUNDLE" >&2; then
    echo "reset failed. Not starting another process-tap open." >&2
    echo TIMEOUT
    return 0
  fi
  echo "Click $click. Watcher does not click." >&2
  set +e
  launch_tap system >&2
  launch_rc=$?
  set -e
  echo "process_exit=$launch_rc is not an outcome. TIMEOUT is not Deny." >&2
  if [ "$launch_rc" -eq 2 ] || [ "$launch_rc" -eq 3 ]; then
    echo "measure refused ($launch_rc). Not Allow. Not Deny. Not cleanup." >&2
    echo TIMEOUT
    return 0
  fi
  classify "$(audio_row)" "$(audio_pair)"
}

# Safe chain step. The system TCC sheet is the AudioCapture pair already taken.
# Does not reset TCC. Does not open system again. One app tap, then a blank cleanup record.
after_pair() {
  refuse_mic
  echo "AudioCapture system pair already covers the system process-tap TCC sheet (kTCCServiceAudioCapture)." >&2
  echo "Not prompting system again. Not resetting TCC." >&2
  echo "Watcher does not click." >&2
  echo "Next source is $APP_HAL. Terminal is not a HAL client." >&2
  echo "A missing sheet is the same service, not Deny. TIMEOUT is not Deny." >&2
  echo "Exit 0 is not a Cleanup tip SHA. Drop is not a Cleanup tip SHA." >&2
  set +e
  launch_tap "$APP_HAL"
  rc=$?
  set -e
  echo "app tap process_exit=$rc is not an outcome." >&2
  tip_none >&2
  return "$rc"
}

arm_measure() {
  refuse_mic
  echo "Watcher does not click." >&2
  echo "Standalone --arm: Click Allow on the system sheet, then Don't Allow." >&2
  echo "Then one $APP_HAL launch with no further reset." >&2
  echo "Do not run --arm after scripts/m0-ac1-full-session.sh. That session calls after-pair." >&2
  echo "TIMEOUT is not Deny. No service row is not Deny." >&2
  echo "Exit 0 is not Allow. Drop is not a Cleanup tip SHA." >&2
  first=$(prompt_system Allow)
  echo "process-tap 1 outcome $first" >&2
  if ! may_open_again "$first"; then
    echo "stop: $first does not start another process-tap open" >&2
    echo "TIMEOUT is not Deny." >&2
    tip_none >&2
    exit 3
  fi
  case $(plan "$first") in
    reset-and-prompt-deny) next_click="Don't Allow" ;;
    reset-and-prompt-allow) next_click=Allow ;;
    *)
      echo "stop: $first does not start another process-tap open" >&2
      tip_none >&2
      exit 3
      ;;
  esac
  second=$(prompt_system "$next_click")
  echo "process-tap 2 outcome $second" >&2
  case $second in
    TIMEOUT|UNPROVEN)
      echo "stop: $second does not start another process-tap open" >&2
      echo "TIMEOUT is not Deny." >&2
      tip_none >&2
      exit 3
      ;;
  esac
  if ! pair_complete "$first" "$second"; then
    echo "stop: need both Allow and Deny. Not starting another system open." >&2
    tip_none >&2
    exit 3
  fi
  echo "both system outcomes observed. Not resetting before the app tap." >&2
  refuse_mic
  set +e
  launch_tap "$APP_HAL"
  rc=$?
  set -e
  echo "app tap process_exit=$rc is not an outcome. A missing sheet is the same service, not Deny." >&2
  tip_none >&2
  return "$rc"
}

cmd=${1:-}
case $cmd in
  gate|--arm|after-pair|system|app:*|pid:*) ;;
  *) usage ;;
esac

refuse_mic

case $cmd in
  gate)
    echo "mic settled. No taco run --source default process."
    echo "Standalone Allow then Don't Allow: $0 --arm"
    echo "After the AudioCapture system pair: $0 after-pair"
    echo "Watcher does not click. TIMEOUT is not Deny. No service row is not Deny."
    echo "Cleanup tip SHA stays none until destroy is separately traced."
    tip_none
    ;;
  --arm)
    arm_measure
    ;;
  after-pair)
    after_pair
    ;;
  system|app:*|pid:*)
    launch_tap "$cmd"
    ;;
esac
