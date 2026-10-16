#!/bin/sh
# Load the M0 launchd user agent, let it run taco notices, then bootout that label.
# Does not signal, kill, or click anything else. Does not run `taco run`.
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$root"

label=ai.syndai.taco.m0-launch-host
uid=$(id -u)
domain="gui/$uid"
target="$domain/$label"
stub="$root/scripts/m0-launchd-host-stub.sh"
template="$root/assets/launchd/ai.syndai.taco.m0-launch-host.plist"
workdir="$root/target/m0-launchd-host"
plist="$workdir/$label.plist"
stdout="$workdir/stdout.log"
stderr="$workdir/stderr.log"
loaded_by_us=0

ts() { date -u +%Y-%m-%dT%H:%M:%SZ; }

cleanup() {
  status=$?
  if [ "$loaded_by_us" -eq 1 ]; then
    if launchctl print "$target" 2>/dev/null | grep -F "$stub" >/dev/null; then
      launchctl bootout "$target" >/dev/null 2>&1 || true
    fi
  fi
  exit "$status"
}
trap cleanup EXIT

mkdir -p "$workdir"
chmod +x "$stub"

echo "timestamp_start=$(ts)"
echo "hostname=$(hostname)"
echo "uname=$(uname -a)"
sw_vers
echo "uid=$uid"
echo "domain=$domain"
echo "label=$label"
echo "stub=$stub"

if launchctl print "$target" >"$workdir/pre.print" 2>"$workdir/pre.err"; then
  if grep -F "$stub" "$workdir/pre.print" >/dev/null; then
    echo "timestamp_pre_bootout=$(ts)"
    launchctl bootout "$target"
    echo "pre_bootout_exit=$?"
  else
    echo "refusing: $target is already loaded and its program is not $stub" >&2
    exit 3
  fi
else
  echo "pre_print_exit=$?"
  echo "pre_print_stderr=$(cat "$workdir/pre.err")"
fi

echo "timestamp_build=$(ts)"
cargo build --bin taco
bin="$root/target/debug/taco"
echo "taco_bin=$bin"
echo "taco_sha256=$(shasum -a 256 "$bin" | awk '{print $1}')"
echo "timestamp_codesign=$(ts)"
codesign -dv "$bin" >"$workdir/codesign.txt" 2>&1 || true
cat "$workdir/codesign.txt"

rm -f "$stdout" "$stderr"
sed \
  -e "s|__STUB__|$stub|" \
  -e "s|__STDOUT__|$stdout|" \
  -e "s|__STDERR__|$stderr|" \
  "$template" > "$plist"
echo "timestamp_plutil=$(ts)"
plutil -lint "$plist"
echo "plutil_exit=$?"
echo "rendered_plist_begin"
cat "$plist"
echo "rendered_plist_end"

echo "timestamp_bootstrap=$(ts)"
set +e
launchctl bootstrap "$domain" "$plist"
boot_ec=$?
set -e
echo "bootstrap_exit=$boot_ec"
if [ "$boot_ec" -ne 0 ]; then
  exit "$boot_ec"
fi
loaded_by_us=1

found=0
print_ec=1
kickstarted=0
i=0
while [ "$i" -lt 30 ]; do
  i=$((i + 1))
  set +e
  launchctl print "$target" >"$workdir/print.txt" 2>"$workdir/print.err"
  print_ec=$?
  set -e
  if [ "$print_ec" -eq 0 ] && grep -E 'last exit code = [0-9]+$' "$workdir/print.txt" >/dev/null; then
    if [ -s "$stdout" ]; then
      found=1
      break
    fi
  fi
  if [ "$kickstarted" -eq 0 ] && [ "$i" -eq 8 ]; then
    if ! grep -E '^[[:space:]]*pid = [0-9]+$' "$workdir/print.txt" >/dev/null; then
      echo "timestamp_kickstart=$(ts)"
      set +e
      launchctl kickstart -p "$target"
      kick_ec=$?
      set -e
      echo "kickstart_exit=$kick_ec"
      kickstarted=1
    fi
  fi
  sleep 1
done

echo "timestamp_print=$(ts)"
echo "print_exit=$print_ec"
echo "saw_last_exit_code=$found"
echo "print_begin"
cat "$workdir/print.txt"
echo "print_end"
if [ "$print_ec" -ne 0 ]; then
  echo "print_stderr_begin"
  cat "$workdir/print.err"
  echo "print_stderr_end"
fi

echo "timestamp_bootout=$(ts)"
set +e
launchctl bootout "$target"
out_ec=$?
set -e
echo "bootout_exit=$out_ec"
loaded_by_us=0

echo "timestamp_print_after=$(ts)"
set +e
launchctl print "$target" >"$workdir/print-after.txt" 2>"$workdir/print-after.err"
after_ec=$?
set -e
echo "print_after_exit=$after_ec"
echo "print_after_stderr_begin"
cat "$workdir/print-after.err"
echo "print_after_stderr_end"

echo "timestamp_end=$(ts)"
echo "job_stdout_begin"
if [ -f "$stdout" ]; then
  cat "$stdout"
else
  echo "MISSING"
fi
echo "job_stdout_end"
echo "job_stderr_begin"
if [ -f "$stderr" ]; then
  cat "$stderr"
else
  echo "MISSING"
fi
echo "job_stderr_end"

job_exit=$(grep -E 'last exit code = [0-9]+$' "$workdir/print.txt" | awk '{print $NF}' || true)
echo "job_exit_code=${job_exit:-missing}"
if [ "$boot_ec" -ne 0 ] || [ "$found" -ne 1 ] || [ "$out_ec" -ne 0 ] || [ "$after_ec" -eq 0 ]; then
  exit 1
fi
if [ "$job_exit" != "0" ]; then
  exit 1
fi
if ! grep -F "taco shows subtitles" "$stdout" >/dev/null; then
  echo "job stdout missing notices text" >&2
  exit 1
fi
