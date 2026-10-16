# AudioCapture measurement after the mic watch

M0-AC1 stays unchecked. This is the command list for `system`, `app:`, and `pid:` once no `taco run --source default` process is left. It does not press Allow or Deny. It does not reset TCC.

Host: macOS 27.0.1 (26A434), arm64. Worktree `~/workspace/syndai/taco-m0-audiocapture-start`, tip `f9b08be` (PR #47; fetched from `4811926`, then fast-forwarded). Binary:

```text
/Users/jwu/workspace/syndai/taco-m0-audiocapture-start/target/debug/taco
Identifier=ai.syndai.taco
Signature=adhoc
Info.plist entries=5
CDHash=21010fb452aa7504eb974869d1023085328ae773
```

`cargo build` and `./scripts/sign-debug.sh` were run here. The tap was not started.

## Why it was not started

At 2026-10-17 02:15 KST the microphone watch was still live. It was not signaled, and no dialog was clicked:

```text
97484 /Users/jwu/workspace/syndai/taco-m0-app/packaging/Taco.app/Contents/MacOS/taco run --source default --from en --to zh
97480 open -W --env TACO_CAPTURE_SPIKE=1 --env TACO_CAPTURE_SPIKE_OPEN_MS=300000 --env TACO_CAPTURE_SPIKE_HOLD_MS=1800000 .../Taco.app
```

Dialog appeared: no. The spike in this worktree was not launched, so no AudioCapture prompt was requested.

## Gate

Stop if this prints a line. Do not `kill` that pid. Do not `tccutil reset`. A `--source default` process is the microphone watch even when it lives inside `Taco.app`.

```sh
ps aux | awk '/[t]aco run --source default/'
```

Repeat the gate before every source below. `TACO_CAPTURE_SPIKE_HOLD_MS` is not read by this binary. Set `TACO_CAPTURE_SPIKE_OPEN_MS`. `300000` is inside `0..600000`. A value above `600000`, or not a whole number, is replaced with the default `300000`. `0` starts and stops without waiting.

Paste the commands into Terminal.app. A spike started under Cursor is attributed to `com.todesktop.230313mzl4w4u92`.

Rebuild only if the tree changed, then sign again. A linker signature leaves `Info.plist=not bound`.

```sh
cd /Users/jwu/workspace/syndai/taco-m0-audiocapture-start
cargo build
./scripts/sign-debug.sh
codesign -dv --verbose=2 ./target/debug/taco
```

Require `Identifier=ai.syndai.taco` and `Info.plist entries=5`.

## Read, then one source

Snapshot first. Copy the output. Do not reset Microphone.

```sh
tccutil list -s AudioCapture
tccutil list -s AudioCapture -b ai.syndai.taco
tccutil list -s Microphone
tccutil list -s Microphone -b ai.syndai.taco
```

`denied` from the pair query, with `ai.syndai.taco` absent from `tccutil list -s AudioCapture`, is the same output as a bundle that has never been asked.

In a second Terminal, before the tap:

```sh
log stream --style compact --predicate 'subsystem == "com.apple.TCC" AND (eventMessage CONTAINS "ai.syndai.taco" OR eventMessage CONTAINS "kTCCServiceAudioCapture" OR eventMessage CONTAINS "AUTHREQ_PROMPTING")'
```

One source at a time. The service that counts is `kTCCServiceAudioCapture`. A dialog appeared when that log shows `AUTHREQ_PROMPTING` for `kTCCServiceAudioCapture`, or when the on-screen usage string is `taco captures one system or application audio source for local subtitles.` Do not click. If the usage string is `taco uses the microphone you select to show local subtitles.`, that is Microphone; leave it untouched and do not treat it as AudioCapture.

```sh
cd /Users/jwu/workspace/syndai/taco-m0-audiocapture-start
export TACO_CAPTURE_SPIKE=1
export TACO_CAPTURE_SPIKE_OPEN_MS=300000

./target/debug/taco run --source system --from en --to zh

./target/debug/taco run --source app:com.google.Chrome --from en --to zh

PID=$(pgrep -x 'Google Chrome' | head -n 1)
./target/debug/taco run --source pid:"$PID" --from en --to zh
```

`app:` needs exactly one audio process with that bundle id. Chrome helpers are `com.google.Chrome.helper`. More than one match exits 1 with `error capture_not_available` and is not a dialog. Use the `pid:` line with the main Chrome pid from `pgrep -x 'Google Chrome'`.

Each command calls `AudioDeviceStart` and holds for 300000 ms, then `AudioDeviceStop`. The process exits by itself. Exit 0 is not Allow. Exit 1 `error capture_not_available` is not Deny. No `kTCCServiceAudioCapture` line means no AudioCapture request.

After each exit, copy:

```sh
tccutil list -s AudioCapture
tccutil list -s AudioCapture -b ai.syndai.taco
tccutil list -s Microphone
tccutil list -s Microphone -b ai.syndai.taco
```

To stop a spike you started, before the hold ends:

```sh
ps -p SPIKE_PID -o command=
kill SPIKE_PID
```

`kill` only when that command line is `--source system`, `--source app:`, or `--source pid:` from this worktree. Never signal a `--source default` pid.

Allow and Deny clicks stay in `docs/evidence/m0/audiocapture-prep.md`. This measurement does not press a button.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
