# AudioCapture measurement after the mic watch

Cited from `cursor/m0-audiocapture-start-b19b` tip `ce5690f` (PR #47). That tip is newer than code tip `f9b08be`. This branch does not replay `native/process_tap.m`. No acceptance box is checked.

`4811926` starts the spike aggregate with `AudioDeviceStart` and holds `TACO_CAPTURE_SPIKE_OPEN_MS`. `f9b08be` sets the unset default to `300000` (values above `600000` or not a whole number use that default; `0` starts and stops without waiting). `kAudioAggregateDeviceTapAutoStartKey` stays 0. The IO proc discards samples. No Mac TCC result is recorded on that code tip.

At `ce5690f` the tap was not started. At 2026-10-17 02:15 KST the microphone watch was still live and was not signaled:

```text
97484 .../Taco.app/Contents/MacOS/taco run --source default --from en --to zh
```

No dialog. No button. The built binary was ad-hoc: `Identifier=ai.syndai.taco`, `Signature=adhoc`, `Info.plist entries=5`, `CDHash=21010fb452aa7504eb974869d1023085328ae773`. Host recorded on that note: macOS 27.0.1 (26A434), arm64. It does not name a chip or C2.

Exit 0 would not be Allow. Exit 1 `error capture_not_available` would not be Deny. Neither exit was observed, because the spike was not launched.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
