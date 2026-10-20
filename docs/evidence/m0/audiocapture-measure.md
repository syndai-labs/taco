# AudioCapture Allow then Deny

Status: **Unmeasured**. No AudioCapture dialog was shown on this tip. No button was pressed. Mac was offline, so this note does not record Allow, Deny, or TIMEOUT. M0-AC1 stays open.

Allow tip SHA: none
Deny tip SHA: none

Legal outcomes are Allow, Deny, and TIMEOUT. TIMEOUT is not Deny.

Checking M0-AC1 requires both an Allow tip SHA and a Deny tip SHA for AudioCapture, written above as git SHAs after a Mac operator click. A blank line, the word `none`, a process exit, or `tccutil` printing `denied` with no service row does not fill either line.

Unmeasured — AudioCapture Allow/Deny not recorded; consequence AC1 open.

## When to run

Run this only after the microphone watch has exited. The watch is any `taco run --source default` process, including one inside Taco.app. Do not kill it. Do not `tccutil reset` while it is up.

```sh
./scripts/m0-audiocapture-measure.sh gate
```

`gate` prints `mic settled` when that process is gone. If it prints `mic LIVE`, stop. The last watched mic pid is not an AudioCapture result.

Pack Taco.app from this worktree. The microphone app can be a build from before `AudioDeviceStart`, and a raw debug binary is attributed to Cursor (`com.todesktop.230313mzl4w4u92`).

```sh
cargo build
./scripts/sign-debug.sh
./scripts/m0-pack-taco-app.sh
```

The measure script launches `packaging/Taco.app` with `open -W`. Bundle id is `ai.syndai.taco`. A missing app, or an Identifier other than that, refuses to launch. `TACO_APP` overrides the path. Off macOS, pack and measure both exit 2 and print `platform_unsupported`.

## Window

`OPEN_MS` and `TACO_CAPTURE_SPIKE_OPEN_MS` default to `300000`. `HOLD_MS` and `TACO_CAPTURE_SPIKE_HOLD_MS` default to `1800000`. The script passes both, matching the mic harness. The tap uses `OPEN_MS` only: native `spike_open_ms` keeps a whole number from 0 through 600000 and otherwise uses 300000. This measure rejects 0, because 0 starts and stops without an operator window. `HOLD_MS` is the mic hold after open Ok. The tap does not read it.

## Allow, then Deny

One source at a time. `system` is the first run. `app:` and `pid:` are the same `kTCCServiceAudioCapture` service on this bundle. Do this in Terminal on the Mac. The script does not click.

1. Snapshot Microphone and leave it. Do not reset Microphone, ScreenCapture, or SystemAudioRecording.

```sh
tccutil list -s Microphone
tccutil list -s Microphone -b ai.syndai.taco
tccutil list -s AudioCapture
tccutil list -s AudioCapture -b ai.syndai.taco
```

A pair-query `denied` with `ai.syndai.taco` absent from `tccutil list -s AudioCapture` means the bundle has not been asked. That is not Deny.

2. Reset AudioCapture only, then confirm the Microphone pair word is unchanged and the AudioCapture list omits `ai.syndai.taco`.

```sh
tccutil reset AudioCapture ai.syndai.taco
```

3. Launch. Josh clicks **Allow** once inside the window. The usage string must be `taco captures one system or application audio source for local subtitles.` If it is the microphone string, leave the dialog untouched and do not record it here.

```sh
./scripts/m0-audiocapture-measure.sh system
```

4. Copy `tccutil` again. Fill one record. `outcome` is Allow only when the click was Allow, the list has an `ai.syndai.taco` row, and the pair query prints `granted`.

5. Reset AudioCapture only, the same command as step 2. Launch the same source again. Josh clicks **Don't Allow** once. `outcome` is Deny only when that click happened, the row is present, and the pair query prints `denied`.

6. If the window ends and nobody clicked, `outcome` is TIMEOUT. TIMEOUT is not Deny. Exit 0 is not Allow. `error capture_not_available` is not Deny.

Then run the other selector once (`app:` or `pid:` if the clicks were `system`) and record whether it prompts. A missing second prompt while AudioCapture is still `granted` or `denied` is the same service, not a new decision. AC1 still needs the system selector and an app selector exercised. It does not need a second Allow and a second Deny.

## Record

The script prints this blank block after `open` returns. Copy it into the Mac tip. Leave the cells below empty until that tip exists. `process_exit` is not an outcome.

```text
outcome:
bundle: ai.syndai.taco
source:
open_ms: 300000
hold_ms: 1800000
click:
tcc_list:
tcc_pair:
row_present:
pid:
tip_sha:
```

| Run | outcome | source | click | row_present | tcc_pair | tip_sha |
| --- | --- | --- | --- | --- | --- | --- |
| Allow | | | | | | |
| Deny | | | | | | |

Off macOS, `scripts/m0-audiocapture-measure.sh` exits 2 and prints `platform_unsupported`. That exit is not Allow, Deny, or TIMEOUT, and it is not a CoreAudio success.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
