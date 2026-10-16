# AudioCapture prompt prep (2026-10-17)

M0-AC1 stays unchecked. This note is the operator path for the AudioCapture
prompt on `ai.syndai.taco`. The code on this branch starts the aggregate
under the spike. No Mac run of that start is recorded here. No dialog was
clicked. No `tccutil reset` was run.

Host: macOS 27.0.1 (26A434), arm64. This machine is not C1 and it is not a
named C2. Tip `cd9a4c8` (`origin/cursor/m0-process-tap-b19b`).

## Why the tap was not started

At 01:55 KST pid 76134 was:

```text
/Users/jwu/workspace/syndai/taco-m0-tap/target/debug/taco run --source default --from en --to zh
```

`codesign -dv` on that binary: `Identifier=ai.syndai.taco`, ad-hoc,
`Info.plist entries=5`. That pid was gone at 01:59:48 KST. Pid 82331 was
already the same command from the same binary (still running while this note
was written). Neither pid was signaled.

`taco run --source default` is the microphone spike. It does not press
Allow or Deny. A pid exit is not an Allow and it is not a Deny.

Prior `system` / Chrome `app:` / `pid:` spikes in `process-tap.md` sent a
`kTCCServiceMicrophone` preflight and no `kTCCServiceAudioCapture` line.
Starting one while a microphone open is in flight can take that prompt.
The dry-run stays undone until that `taco run --source default` is gone and
the microphone decision is already copied down.

## Start gap and the fix

On tip `cd9a4c8` the spike created the private process tap and the private
aggregate with `kAudioAggregateDeviceTapAutoStartKey` set to 0, then dropped
both. It did not call `AudioDeviceStart` on that aggregate. The
`AudioDeviceStart` in the same file was only the microphone input proc.
Apple's tap sample ties the AudioCapture prompt to the first start of an
aggregate that contains a tap. Exit 0 on that tip was create-and-drop. It
was not an Allow and it was not a Deny. The recorded `system` and Chrome
`app:` / `pid:` runs in `process-tap.md` logged a Microphone preflight and
no `kTCCServiceAudioCapture` line.

This tree still opens a tap only when `TACO_CAPTURE_SPIKE` is set. After the
aggregate exists, and after the default-output check, it registers a no-op
IO proc and calls `AudioDeviceStart` on the aggregate. The auto-start key
stays 0 so that start does not wait for a tapped process to deliver audio.
`AudioDeviceStart` returns before an operator answers, so the aggregate stays
started for `TACO_CAPTURE_SPIKE_OPEN_MS` milliseconds (default 300000, values
above 600000 or not a whole number use that default, `0` starts and stops
without waiting). Then `AudioDeviceStop`, the IO proc is destroyed, and Rust
`Drop` destroys the aggregate and the tap. Nothing in this path presses
Allow or Deny.

No Mac run of this start is recorded. No dialog was clicked. Exit 0 after
the hold is still not an Allow. `error capture_not_available` is still a
miss or an OS failure, including a failed `AudioDeviceStart`. It is not a
Deny.

## What the CLI does

`TACO_CAPTURE_SPIKE` unset: `start_capture` returns `capture_not_available`
and does not open or start a tap.

`TACO_CAPTURE_SPIKE=1` and `--source system`, `app:<bundle-id>`, or `pid:<pid>`:
the path above. Recorded Chrome `app:` and `pid:` runs on the `cd9a4c8`
wiring exited 0 with no AudioCapture request and no dialog. Those runs did
not call `AudioDeviceStart`. Exit 0 is an Allow only after an operator press
and an AudioCapture row for `ai.syndai.taco`.

AudioCapture is one TCC service for the client bundle. `system` and
`app:` / `pid:` are two selectors of that service. A grant on one selector
covers `ai.syndai.taco` for the other until AudioCapture is reset. AC1 still
needs both selectors exercised. The privacy prompt is the service, not a
second service per selector.

`NSAudioCaptureUsageDescription` is the AudioCapture usage string in
`assets/Info.plist`. ScreenCapture is a different service. This tap path does
not request it.

## Read TCC

`tccutil` on this host:

```text
tccutil list -s SERVICE
tccutil list -b BUNDLE_ID
tccutil list -s SERVICE -b BUNDLE_ID
```

`SERVICE` may omit the `kTCCService` prefix. Reads below are safe during the
microphone watch. `tccutil reset` is not.

A recorded decision is a bundle id printed by `tccutil list -s SERVICE`.
The pair query prints a word even when that list has no row. Checked at
01:59 KST:

| Query | Printed |
| --- | --- |
| `tccutil list -s AudioCapture` | `com.google.AIEdgeForesight` only |
| `tccutil list -s AudioCapture -b com.google.AIEdgeForesight` | `granted` |
| `tccutil list -s AudioCapture -b ai.syndai.not-a-bundle` | `denied` |
| `tccutil list -s AudioCapture -b ai.syndai.taco` | `denied` |
| `tccutil list -b ai.syndai.taco` | empty |
| `tccutil list -s SystemAudioRecording` | empty |
| `tccutil list -s SystemAudioRecording -b ai.syndai.taco` | `denied` |
| `tccutil list -s Microphone` | no `ai.syndai.taco` row (Cursor `com.todesktop.230313mzl4w4u92` is listed; its pair query printed `granted`) |
| `tccutil list -s Microphone -b ai.syndai.taco` | `denied` |
| `tccutil list -s ScreenCapture` | no `ai.syndai.taco` row |
| `tccutil list -s ScreenCapture -b ai.syndai.taco` | `denied` |

`denied` from the pair query, with the bundle absent from `tccutil list -s`,
is the same output as a bundle that has never been asked. Copy both commands
after every click. An operator Deny is a row in `tccutil list -s AudioCapture`
whose pair query prints `denied`. An operator Allow is that row with `granted`.

Snapshot Microphone before any AudioCapture reset, then again after each
click. The Microphone line has to match. This prep does not reset
Microphone, ScreenCapture, or SystemAudioRecording.

## After the microphone watch has exited

Confirm the watch is idle:

```sh
ps aux | awk '/target\/debug\/taco run --source default/ && !/awk/'
tccutil list -s Microphone
tccutil list -s Microphone -b ai.syndai.taco
```

Copy the Microphone list and the pair word. Leave Microphone as it is.

Build and bind the plist in this worktree (or the `cd9a4c8` tree), then check
the identifier:

```sh
cargo build
./scripts/sign-debug.sh
codesign -dv --verbose=2 ./target/debug/taco
```

`Identifier=ai.syndai.taco` and `Info.plist entries=5`. Launch from
Terminal.app. Spikes launched under Cursor were attributed to responsible
bundle `com.todesktop.230313mzl4w4u92`, preflighted Microphone, and wrote no
`ai.syndai.taco` service row.

Resolve Chrome's main pid in that Terminal. Helpers are
`com.google.Chrome.helper` and are the wrong `app:` target. Pid 688 was the
main Chrome process at 01:59 KST; resolve it again:

```sh
pgrep -x 'Google Chrome'
```

In a second Terminal, before the tap:

```sh
log stream --style compact --predicate 'subsystem == "com.apple.TCC" AND eventMessage CONTAINS "ai.syndai.taco"'
```

One tap at a time. The service that counts is `kTCCServiceAudioCapture`.
A `kTCCServiceMicrophone` line is the microphone watch's service. If the
dialog text is the microphone usage string, leave it untouched.

```sh
# System process tap.
TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source system --from en --to zh

# Chrome app tap. One main process only.
TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source app:com.google.Chrome --from en --to zh

# Chrome pid tap. Substitute the pgrep pid.
TACO_CAPTURE_SPIKE=1 ./target/debug/taco run --source pid:PID --from en --to zh
```

These commands call `AudioDeviceStart` and hold the aggregate for
`TACO_CAPTURE_SPIKE_OPEN_MS` (default 300000) before teardown. A dialog is
possible and is not recorded yet. If the process exits with no
`kTCCServiceAudioCapture` prompting line, record that and stop. That exit
is not Allow and not Deny. The tool does not press a button. If a dialog
appears, press one button once.

If a dialog does appear, press one button once. Then:

```sh
tccutil list -s AudioCapture
tccutil list -s AudioCapture -b ai.syndai.taco
tccutil list -s SystemAudioRecording
tccutil list -s SystemAudioRecording -b ai.syndai.taco
tccutil list -s Microphone
tccutil list -s Microphone -b ai.syndai.taco
tccutil list -b ai.syndai.taco
```

## Allow and Deny on AudioCapture only

Do this only after the Microphone snapshot above is saved and no
`--source default` taco is running.

Either order works. Allow first:

```sh
tccutil reset AudioCapture ai.syndai.taco
tccutil list -s AudioCapture
tccutil list -s Microphone -b ai.syndai.taco
```

The AudioCapture list should omit `ai.syndai.taco`. The Microphone pair word
should match the snapshot. Then run one tap command from Terminal. Press
Allow once. Re-read. `ai.syndai.taco` should appear under
`tccutil list -s AudioCapture` with pair word `granted`.

Then the other decision:

```sh
tccutil reset AudioCapture ai.syndai.taco
```

Run one tap command again. Press Don't Allow once. Re-read.
`ai.syndai.taco` should appear under `tccutil list -s AudioCapture` with pair
word `denied`.

Deny first is the same pair of resets, with Don't Allow on the first run and
Allow on the second. Reset AudioCapture only. A bare `tccutil reset`, a
Microphone reset, or a ScreenCapture reset mixes the microphone evidence.

After both clicks, run the other selector once (system if the prompts were
Chrome, or Chrome if the prompts were system) and record whether it prompts.
A missing second prompt with AudioCapture still `granted` or `denied` is the
same service, not a new decision.

AC1 stays open until microphone, system, and app each have operator Allow and
Deny, plus cleanup, from the intended CLI. This prep is none of those clicks.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
