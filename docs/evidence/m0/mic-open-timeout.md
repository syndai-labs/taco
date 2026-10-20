# M0 microphone open timeout

Cited from `cursor/m0-mic-open-timeout-b19b` tip `c7e7c2e` (PR #31). This branch does not replay `src/session.rs`. No acceptance box is checked. No Allow or Deny is recorded.

`open_default_microphone` waits at most `TACO_CAPTURE_SPIKE_OPEN_MS` milliseconds, parsed as `u64`. Timeout fails closed as `capture_not_available`. PR #31 tip `c7e7c2e` used an unset value of 180000, and that wait ended `OPEN_MS TIMEOUT` (pids 39380, 54200, and others) before the operator reached the dialog. PR #36 tip `81a2a54` raised the unset default to `300000` (5 minutes). PR #50 tip `fdc201f` locks that unset default and the hold default. The variable still overrides the default. No Allow or Deny. TIMEOUT is not Don't Allow. This branch does not replay `src/capture/microphone.rs`. The timeout path does not join the CPAL worker and does not press Allow or Deny. `TACO_CAPTURE_SPIKE` still gates the open. `TACO_CAPTURE_SPIKE_HOLD_MS` still applies only after open returns `Ok`. `freeze::CAPTURE_SPIKE_OPEN_MS` is `300000`.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
