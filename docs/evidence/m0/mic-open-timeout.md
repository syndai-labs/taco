# M0 microphone open timeout

`open_default_microphone` on macOS calls CPAL `default_host`, `default_input_device`, `default_input_config`, `build_input_stream_raw`, and `play`. Those calls block in CoreAudio (`AudioObjectGetPropertyData`, then `audio_unit.start()` inside `play`). CPAL 0.15.3 takes a timeout argument on `build_input_stream_raw` and ignores it (`_timeout`). `cpal::Stream` is not `Send`, so the stream cannot move to the caller. This crate does not request TCC itself and does not press Allow or Deny.

The open runs on a worker. The caller waits `TACO_CAPTURE_SPIKE_OPEN_MS` milliseconds, parsed as `u64`. Unset or not a `u64` uses `180000` (3 minutes). Three minutes is long enough to click a prompt that is already visible, and shorter than the reported ~7.5 minute hang when that dialog is missed. Raise the variable if the prompt is slow to appear. `0` fails closed on the next check.

Timeout, a CPAL error, or a disconnected worker becomes `Err(())`. `open_microphone_spike` maps that to `capture_not_available`. The timeout path does not join the worker. Joining would sit in the CoreAudio wait. Dropping the reply channel makes a late open drop the stream. Process exit ends a worker still blocked in CPAL. `forbid(unsafe_code)` is unchanged.

`TACO_CAPTURE_SPIKE` still gates `start_capture`. `TACO_CAPTURE_SPIKE_HOLD_MS` still applies only after open returns `Ok`.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
