# M0-AC1 mic resume (not a session)

No Allow was recorded here. No Deny was recorded here. This file is the contract for `scripts/m0-ac1-mac-resume.sh`. It is not a Mac session. A Linux run does not raise SecurityAgent and does not record a click.

When the Mac worker is online and no `taco run --source default` is live:

```sh
./scripts/m0-pack-taco-app.sh
./scripts/m0-ac1-mac-resume.sh --arm
```

Pack uses `packaging/Info.plist` (`ai.syndai.taco`), the same keys as PR #46 tip `7fb5006`. The resume script pins `TACO_CAPTURE_SPIKE_OPEN_MS` to `300000` and `TACO_CAPTURE_SPIKE_HOLD_MS` to `1800000` (PR #50 tip `fdc201f`). It launches through `scripts/m0-launch-taco-app.sh`, which is `open` of `packaging/Taco.app`. Inherited `OPEN_MS` / `HOLD_MS` are not read, so a stale shorter wait cannot replace those charter values.

The script prints the clicks. The watcher does not click.

1. Click **Allow** on the SecurityAgent sheet for Taco.
2. After the Microphone service row is copied, the script ends that hold, runs `tccutil reset Microphone ai.syndai.taco`, and raises the sheet again.
3. Click **Don't Allow**.

TIMEOUT is not Don't Allow. PR #51 tip `7c71d50` stays the Partial for the earlier open-wait exits. A pair-query `denied` with no Microphone service row is not an operator Deny. After `TIMEOUT` or `UNPROVEN` the script does not start another open.

Both an Allow outcome and a Deny outcome are required. The session note written on the Mac is `docs/evidence/m0/mic-operator-resume-session.md`. Allow tip SHA stays `none` and Deny tip SHA stays `none` until that session is committed. M0-AC1 stays unchecked.

Allow tip SHA: none
Deny tip SHA: none

This resume script does not launch AudioCapture. `scripts/m0-ac1-full-session.sh` packs, runs this resume, then runs `scripts/m0-audiocapture-measure.sh` only after mic LIVE is gone. PR #47 tip `ce5690f` starts the aggregate with `AudioDeviceStart`. PR #67 tip `652614b` is the measure harness.

- [ ] **M0-AC1:** microphone Allow and Deny from Taco.app
