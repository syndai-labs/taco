# M0 microphone spike hold

Cited from `cursor/m0-mic-hold-b19b` tip `a1654eb` (PR #22) and `cursor/m0-hold-guards-b19b` tip `89759b9` (PR #24). This branch does not replay `src/session.rs`. No acceptance box is checked. No Allow or Deny is recorded.

`open_microphone_spike` still opens only when `TACO_CAPTURE_SPIKE` is set. After `open_default_microphone` returns `Ok`, the stream stays up for `TACO_CAPTURE_SPIKE_HOLD_MS` milliseconds, parsed as `u64`. Unset or not a `u64` uses `1800000` (30 minutes).

The hold does not cover a CPAL wait inside open. CPAL may still block there until TCC answers. This hold only applies after open returns `Ok`. It does not press Allow or Deny.

PR #24 `microphone_spike_hold_follows_open_ok` reads `open_microphone_spike` and requires `TACO_CAPTURE_SPIKE_HOLD_MS` with default `1_800_000` after `open_default_microphone` returns `Ok`. `TACO_CAPTURE_SPIKE` still gates the open. No TCC is faked. M0-AC boxes stay unchecked.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
