# M0-AC1 full Mac session

Status: **Partial**. No Allow was recorded here. No Deny was recorded here. This file is the contract for `scripts/m0-ac1-full-session.sh`. It is not a Mac session. A Linux run does not pack Taco.app, does not raise a sheet, and does not record a click.

Allow tip SHA: none
Deny tip SHA: none

Microphone tip SHAs live in [`mic-operator-resume.md`](mic-operator-resume.md). AudioCapture tip SHAs live in [`audiocapture-measure.md`](audiocapture-measure.md). Both pairs are `none`. Acceptance is microphone, system, and app tap, so either pair still at `none` keeps M0-AC1 unchecked.

TIMEOUT is not Deny. A pair-query `denied` with no service row is not Deny. Process exit is not an outcome.

When the Mac worker is online and no `taco run --source default` is live, Josh starts the agent worker once:

```sh
cargo build && ./scripts/sign-debug.sh
./scripts/m0-ac1-full-session.sh --arm
```

That command:

1. Packs `packaging/Taco.app` (`ai.syndai.taco`, `NSAudioCaptureUsageDescription`, aggregate `AudioDeviceStart` from PR #47 tip `ce5690f`).
2. Runs `scripts/m0-ac1-mac-resume.sh --arm` (PR #68 tip `a1c5c32`). Click **Allow**, then **Don't Allow**. The watcher does not click. `OPEN_MS` is `300000` and `HOLD_MS` is `1800000`. A mic `TIMEOUT` does not start another mic open and is not Don't Allow.
3. After that mic process is gone, runs `scripts/m0-audiocapture-measure.sh` (PR #67 tip `652614b`) `gate`, then `system`. Click **Allow**, then **Don't Allow**. The script resets AudioCapture only between those sheets. If the first AudioCapture window has no service row, that is TIMEOUT, not Deny, and the script does not start another AudioCapture open.

If a `taco run --source default` process is still up, the script does not start AudioCapture, does not kill that pid, and does not reset TCC.

Off macOS, `--arm` exits 2 and prints `platform_unsupported`. That is not a CoreAudio result.

Flip guard cite: PR #77 tip `595dced` (`595dcedafa5e7f2dcc829670e41c462f8ca6dab1`). PR #51 tip `7c71d50` and PR #62 tip `8c24c99` stay. #77 owns `ac1_cannot_flip_while_tip_shas_are_none`. This branch does not copy that session script and does not record a Mac click. Partial — mic and AudioCapture Allow/Deny tip SHAs are none; consequence AC1 open.

- [ ] **M0-AC1:** microphone, system, and app tap Allow and Deny from Taco.app
