# Operator microphone timeouts

Status: **Partial**. Operator Allow and operator Deny are unproven. M0-AC1 stays open.

Cited from PR #51 tip `7c71d50` (`7c71d509b404de536bb4a10b60c254d382824403`). This branch does not launch Taco.app and does not replay a Mac log. No button was pressed. A timeout is the open wait ending. It is not a click. TIMEOUT is not Don't Allow.

Allow tip SHA: none
Deny tip SHA: none

Pids 65422, 71152, 76134, 82331, and 89220 ended `OPEN_MS TIMEOUT`. `tccutil` for `ai.syndai.taco` was still denied. A pair-query `denied` with no service row is not an operator Deny. Unset `TACO_CAPTURE_SPIKE_OPEN_MS` is `300000` (PR #36 tip `81a2a54`, harness PR #50 tip `fdc201f`). This branch does not replay `src/capture/microphone.rs`.

Partial — operator Allow/Deny unproven; consequence AC1 open.

Checking M0-AC1 requires this Partial to be replaced and both an Allow tip SHA and a Deny tip SHA to be written above. A timeout, a disappeared dialog, or `tccutil` printing `denied` does not fill either line.

PR #77 tip `595dced` is still Partial. Its microphone and AudioCapture tip SHAs are none. TIMEOUT is not Deny. This note stays the #51 lock.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
