#!/bin/sh
# One-shot microphone Allow, then Deny, for M0-AC1.
# Raises the SecurityAgent sheet through scripts/m0-launch-taco-app.sh.
# The watcher does not click. TIMEOUT is not Don't Allow.
# A TIMEOUT or an unproven exit does not start another open.
# OPEN_MS and HOLD_MS are the charter values, not the inherited environment.
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
BUNDLE=ai.syndai.taco
OPEN_MS=300000
HOLD_MS=1800000
export TACO_CAPTURE_SPIKE_OPEN_MS=$OPEN_MS
export TACO_CAPTURE_SPIKE_HOLD_MS=$HOLD_MS

usage() {
  echo "usage: $0 {self-check|instructions|classify|plan|evidence|format-row|simulate|--arm}" >&2
  exit 2
}

classify() {
  elapsed=$1
  row=$2
  pair=$3
  open_ms=$4
  if [ "$elapsed" -ge "$open_ms" ]; then
    echo TIMEOUT
    return 0
  fi
  if [ "$row" = yes ] && [ "$pair" = granted ]; then
    echo Allow
    return 0
  fi
  if [ "$row" = yes ] && [ "$pair" = denied ]; then
    echo Deny
    return 0
  fi
  echo UNPROVEN
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

click_cell() {
  case $1 in
    Allow|Deny) echo "operator click" ;;
    *) echo "not a click" ;;
  esac
}

evidence() {
  outcome=$1
  cat <<EOF
# Mic operator resume session

Status: **Partial** until both Allow and Deny are on a tip.
TIMEOUT is not Don't Allow.
A pair-query denied with no Microphone service row is not an operator Deny.
Allow tip SHA: none
Deny tip SHA: none
OPEN_MS: $OPEN_MS
HOLD_MS: $HOLD_MS
Bundle: $BUNDLE
Outcome: $outcome
Click: $(click_cell "$outcome")
AC1 requires both Allow and Deny. One outcome does not close AC1.
M0-AC1 stays unchecked.
- [ ] **M0-AC1:** microphone Allow and Deny from Taco.app
EOF
}

format_row() {
  outcome=$1
  pid=$2
  elapsed=$3
  row=$4
  pair=$5
  printf '| %s | %s | %s | %s | %s | %s |\n' \
    "$outcome" "$pid" "$elapsed" "$row" "$pair" "$(click_cell "$outcome")"
}

instructions() {
  cat <<EOF
M0-AC1 one-shot. Watcher does not click.
Bundle: $BUNDLE
OPEN_MS=$OPEN_MS HOLD_MS=$HOLD_MS
TIMEOUT is not Don't Allow. TIMEOUT does not start the next open.

When the SecurityAgent sheet is up for Taco:
  1. Click Allow. Leave Don't Allow for the second sheet.
  2. After that Allow is copied, this script ends the hold, resets Microphone, and raises the sheet again.
  3. Click Don't Allow.
Both clicks are required. M0-AC1 stays unchecked until both are on a tip.
If a taco run --source default is already live, this script does not start another open.
EOF
}

simulate_pair() {
  first=$1
  second=${2:-}
  echo "mac: not-run"
  echo "recorded: none"
  echo "prompt 1 outcome $first"
  evidence "$first"
  if ! may_open_again "$first"; then
    echo "stop: $first does not start another OPEN_MS"
    return 3
  fi
  if [ -z "$second" ]; then
    echo "stop: missing second outcome does not start another OPEN_MS"
    return 3
  fi
  echo "prompt 2 outcome $second"
  evidence "$second"
  case $second in
    TIMEOUT|UNPROVEN)
      echo "stop: $second does not start another OPEN_MS"
      return 3
      ;;
  esac
  if pair_complete "$first" "$second"; then
    echo "both outcomes observed"
    echo "M0-AC1 stays unchecked"
    return 0
  fi
  echo "AC1 still open: need both Allow and Deny"
  echo "stop: not starting a third OPEN_MS"
  return 3
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

darwin() {
  [ "$(uname -s)" = Darwin ]
}

live_mic() {
  pgrep -f '[t]aco run --source default' >/dev/null 2>&1
}

app_pids() {
  pgrep -f '[T]aco.app/Contents/MacOS/taco' || true
}

row_present() {
  if tccutil list -s Microphone 2>/dev/null | grep -F "$BUNDLE" >/dev/null 2>&1; then
    echo yes
  else
    echo no
  fi
}

pair_word() {
  raw=$(tccutil list -s Microphone -b "$BUNDLE" 2>/dev/null | tr -d '[:space:]' || true)
  case $raw in
    *granted*) echo granted ;;
    *denied*) echo denied ;;
    *) echo empty ;;
  esac
}

new_pid() {
  before=$1
  for pid in $(app_pids); do
    case " $before " in
      *" $pid "*) ;;
      *)
        echo "$pid"
        return 0
        ;;
    esac
  done
}

end_hold() {
  pid=$1
  if [ -n "$pid" ]; then
    echo "ending pid $pid after the service row was copied" >&2
    kill "$pid" 2>/dev/null || true
  fi
  if [ -n "${launch_pid:-}" ]; then
    kill "$launch_pid" 2>/dev/null || true
  fi
}

prompt_once() {
  which_prompt=$1
  if live_mic; then
    echo "refusing: mic LIVE already. Not starting another OPEN_MS." >&2
    echo UNPROVEN
    return 0
  fi
  echo "tccutil reset Microphone $BUNDLE" >&2
  if ! tccutil reset Microphone "$BUNDLE"; then
    echo "reset failed. Not starting OPEN_MS." >&2
    echo UNPROVEN
    return 0
  fi
  if [ "$(row_present)" = yes ]; then
    echo "Microphone row still present after reset. Not starting OPEN_MS." >&2
    echo UNPROVEN
    return 0
  fi
  app=$root/packaging/Taco.app
  if [ ! -d "$app" ]; then
    echo "missing app: $app" >&2
    echo "pack first: ./scripts/m0-pack-taco-app.sh" >&2
    echo UNPROVEN
    return 0
  fi
  echo "raising SecurityAgent via Taco.app. Watcher does not click." >&2
  if [ "$which_prompt" = first ]; then
    echo "Click Allow." >&2
  else
    echo "Click Don't Allow." >&2
  fi
  before=$(app_pids)
  launch_pid=
  sh "$root/scripts/m0-launch-taco-app.sh" &
  launch_pid=$!
  start=$(date +%s)
  outcome=UNPROVEN
  elapsed_ms=0
  row=no
  pair=empty
  held=
  while kill -0 "$launch_pid" 2>/dev/null; do
    elapsed_ms=$((($(date +%s) - start) * 1000))
    row=$(row_present)
    pair=$(pair_word)
    found=$(new_pid "$before" || true)
    if [ -n "$found" ]; then
      held=$found
    fi
    kind=$(classify "$elapsed_ms" "$row" "$pair" "$OPEN_MS")
    if [ "$kind" = Allow ] || [ "$kind" = Deny ]; then
      outcome=$kind
      end_hold "$held"
      break
    fi
    if [ "$elapsed_ms" -ge "$((OPEN_MS + 15000))" ]; then
      outcome=TIMEOUT
      break
    fi
    sleep 1
  done
  elapsed_ms=$((($(date +%s) - start) * 1000))
  row=$(row_present)
  pair=$(pair_word)
  if [ -z "$held" ]; then
    held=$(new_pid "$before" || true)
  fi
  if [ "$outcome" != Allow ] && [ "$outcome" != Deny ]; then
    outcome=$(classify "$elapsed_ms" "$row" "$pair" "$OPEN_MS")
  fi
  if [ "$outcome" = TIMEOUT ] && [ -n "$held" ] && kill -0 "$held" 2>/dev/null; then
    echo "ending pid $held after OPEN_MS. TIMEOUT is not a click." >&2
    kill "$held" 2>/dev/null || true
    kill "$launch_pid" 2>/dev/null || true
  fi
  wait "$launch_pid" 2>/dev/null || true
  seen=$held
  if [ -z "$seen" ]; then
    seen=-
  fi
  row_line=$(format_row "$outcome" "$seen" "$elapsed_ms" "$row" "$pair")
  echo "$row_line" >&2
  if [ -n "${session:-}" ]; then
    echo "$row_line" >>"$session"
    list_text=$(tccutil list -s Microphone 2>&1 || true)
    pair_text=$(tccutil list -s Microphone -b "$BUNDLE" 2>&1 || true)
    {
      echo "Step $which_prompt"
      echo "tccutil list -s Microphone:"
      printf '%s\n' "$list_text"
      echo "tccutil list -s Microphone -b $BUNDLE:"
      printf '%s\n' "$pair_text"
      echo
    } >>"${session}.tcc"
  fi
  echo "$outcome"
}

finish_session() {
  if [ -n "${session:-}" ] && [ -f "${session}.tcc" ]; then
    echo >>"$session"
    cat "${session}.tcc" >>"$session"
    rm -f "${session}.tcc"
  fi
}

session_path() {
  echo "$root/docs/evidence/m0/mic-operator-resume-session.md"
}

arm() {
  if ! darwin; then
    echo "recorded: none" >&2
    echo "refusing: not macOS. No SecurityAgent sheet. No Allow. No Deny." >&2
    exit 2
  fi
  instructions >&2
  if live_mic; then
    echo "recorded: none" >&2
    echo "refusing: mic LIVE already. Not starting another OPEN_MS." >&2
    exit 2
  fi
  if ! command -v tccutil >/dev/null 2>&1; then
    echo "recorded: none" >&2
    echo "refusing: tccutil missing. Not starting OPEN_MS." >&2
    exit 2
  fi
  session=$(session_path)
  {
    echo "# Mic operator resume session"
    echo
    echo "Status: **Partial** until both Allow and Deny are on a tip."
    echo "TIMEOUT is not Don't Allow."
    echo "A pair-query denied with no Microphone service row is not an operator Deny."
    echo "Allow tip SHA: none"
    echo "Deny tip SHA: none"
    echo "OPEN_MS: $OPEN_MS"
    echo "HOLD_MS: $HOLD_MS"
    echo "Bundle: $BUNDLE"
    echo "M0-AC1 stays unchecked."
    echo
    echo "| Outcome | Pid | Elapsed_ms | Service row | Pair word | Click |"
    echo "| --- | --- | --- | --- | --- | --- |"
  } >"$session"
  first=$(prompt_once first)
  echo "prompt 1 outcome $first" >&2
  evidence "$first" >&2
  if ! may_open_again "$first"; then
    finish_session
    echo "stop: $first does not start another OPEN_MS" >&2
    echo "recorded: $first" >&2
    echo "M0-AC1 stays unchecked." >&2
    exit 3
  fi
  second=$(prompt_once second)
  echo "prompt 2 outcome $second" >&2
  evidence "$second" >&2
  finish_session
  case $second in
    TIMEOUT|UNPROVEN)
      echo "stop: $second does not start another OPEN_MS" >&2
      echo "recorded: $first" >&2
      echo "recorded: $second" >&2
      echo "M0-AC1 stays unchecked." >&2
      exit 3
      ;;
  esac
  if pair_complete "$first" "$second"; then
    echo "both outcomes observed" >&2
    echo "recorded: $first" >&2
    echo "recorded: $second" >&2
    echo "M0-AC1 stays unchecked." >&2
    exit 0
  fi
  echo "AC1 still open: need both Allow and Deny" >&2
  echo "stop: not starting a third OPEN_MS" >&2
  echo "M0-AC1 stays unchecked." >&2
  exit 3
}

self_check() {
  got=$(classify 300000 no denied 300000)
  [ "$got" = TIMEOUT ]
  got=$(classify 300000 yes denied 300000)
  [ "$got" = TIMEOUT ]
  got=$(classify 300000 yes granted 300000)
  [ "$got" = TIMEOUT ]
  got=$(classify 1200 yes granted 300000)
  [ "$got" = Allow ]
  got=$(classify 1200 yes denied 300000)
  [ "$got" = Deny ]
  got=$(classify 1200 no denied 300000)
  [ "$got" = UNPROVEN ]
  [ "$(plan TIMEOUT)" = stop ]
  [ "$(plan UNPROVEN)" = stop ]
  [ "$(plan Allow)" = reset-and-prompt-deny ]
  [ "$(plan Deny)" = reset-and-prompt-allow ]
  block=$(evidence TIMEOUT)
  echo "$block" | grep -q '^Outcome: TIMEOUT$'
  echo "$block" | grep -q "TIMEOUT is not Don't Allow."
  echo "$block" | grep -q '^Click: not a click$'
  echo "$block" | grep -q '^Allow tip SHA: none$'
  echo "$block" | grep -q '^Deny tip SHA: none$'
  echo "$block" | grep -q 'M0-AC1 stays unchecked.'
  if echo "$block" | grep -q '^Outcome: Deny$'; then
    echo "TIMEOUT evidence was labeled Deny" >&2
    exit 1
  fi
  row=$(format_row TIMEOUT - 300000 no denied)
  echo "$row" | grep -q '| TIMEOUT |'
  echo "$row" | grep -q 'not a click'
  if echo "$row" | grep -q '| Deny |'; then
    echo "TIMEOUT row was labeled Deny" >&2
    exit 1
  fi
  sim=$(simulate_pair TIMEOUT || true)
  echo "$sim" | grep -q 'prompt 1 outcome TIMEOUT'
  echo "$sim" | grep -q 'stop: TIMEOUT does not start another OPEN_MS'
  echo "$sim" | grep -q 'mac: not-run'
  echo "$sim" | grep -q 'recorded: none'
  if echo "$sim" | grep -q 'prompt 2'; then
    echo "TIMEOUT started a second open" >&2
    exit 1
  fi
  sim=$(simulate_pair Allow Deny)
  echo "$sim" | grep -q 'prompt 1 outcome Allow'
  echo "$sim" | grep -q 'prompt 2 outcome Deny'
  echo "$sim" | grep -q 'both outcomes observed'
  echo "$sim" | grep -q 'M0-AC1 stays unchecked'
  echo "$sim" | grep -q 'mac: not-run'
  if echo "$sim" | grep -q 'raising SecurityAgent'; then
    echo "simulate raised a sheet" >&2
    exit 1
  fi
  if echo "$sim" | grep -q 'prompt 3'; then
    echo "simulate started a third open" >&2
    exit 1
  fi
  sim=$(simulate_pair Allow TIMEOUT || true)
  echo "$sim" | grep -q 'prompt 2 outcome TIMEOUT'
  echo "$sim" | grep -q 'stop: TIMEOUT does not start another OPEN_MS'
  if echo "$sim" | grep -q 'prompt 3'; then
    echo "second TIMEOUT started another open" >&2
    exit 1
  fi
  if echo "$sim" | grep -q 'both outcomes observed'; then
    echo "TIMEOUT completed the Allow/Deny pair" >&2
    exit 1
  fi
  if echo "$sim" | grep -q '^Outcome: Deny$'; then
    echo "second TIMEOUT was labeled Deny" >&2
    exit 1
  fi
  echo "self-check ok"
}

cmd=${1:-}
case $cmd in
  self-check) self_check ;;
  instructions) instructions ;;
  classify)
    [ "$#" -eq 5 ] || usage
    classify "$2" "$3" "$4" "$5"
    ;;
  plan)
    [ "$#" -eq 2 ] || usage
    plan "$2"
    ;;
  evidence)
    [ "$#" -eq 2 ] || usage
    evidence "$2"
    ;;
  format-row)
    [ "$#" -eq 6 ] || usage
    format_row "$2" "$3" "$4" "$5" "$6"
    ;;
  simulate)
    [ "$#" -ge 2 ] || usage
    simulate_pair "$2" "${3:-}"
    ;;
  --arm) arm ;;
  *) usage ;;
esac
