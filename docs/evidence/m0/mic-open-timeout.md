# M0 microphone open timeout

Cited from `cursor/m0-mic-open-timeout-b19b` tip `c7e7c2e` (PR #31). This branch does not replay `src/session.rs`. No acceptance box is checked. No Allow or Deny is recorded.

`open_default_microphone` waits at most `TACO_CAPTURE_SPIKE_OPEN_MS` milliseconds, parsed as `u64`. Unset or not a `u64` uses `180000` (3 minutes). Timeout fails closed as `capture_not_available`. The timeout path does not join the CPAL worker and does not press Allow or Deny. `TACO_CAPTURE_SPIKE` still gates the open. `TACO_CAPTURE_SPIKE_HOLD_MS` still applies only after open returns `Ok`.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
