# M0 microphone spike hold

`open_microphone_spike` still opens only when `TACO_CAPTURE_SPIKE` is set. After `open_default_microphone` returns `Ok`, the stream stays up for `TACO_CAPTURE_SPIKE_HOLD_MS` milliseconds, parsed as `u64`. Unset or not a `u64` uses `1800000` (30 minutes).

The hold does not cover a CPAL wait inside open. That wait is bounded by `TACO_CAPTURE_SPIKE_OPEN_MS` (see `mic-open-timeout.md`). This hold only applies after open returns `Ok`. It does not press Allow or Deny.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
