# Operator microphone timeouts

Status: **Partial**. Operator Allow and operator Deny are unproven. M0-AC1 stays open. This note records `OPEN_MS TIMEOUT` exits. A timeout is the open wait ending. It is not a click. TIMEOUT is not Don't Allow.

Base is `cursor/m0-decision-tips6-b19b` tip `19303e8` (PR #48), which already cites Taco.app bundle tip `7fb5006` (PR #46). This branch does not replay the Mac binary, and it does not launch Taco.app. No button was pressed.

Allow tip SHA: none
Deny tip SHA: none

## What was measured

Earlier `OPEN_MS TIMEOUT` pids 39380 and 54200 are already in [mic-open-timeout.md](mic-open-timeout.md). They hit the `180000` window from PR #31 tip `c7e7c2e`, before PR #36 tip `81a2a54` raised the unset `TACO_CAPTURE_SPIKE_OPEN_MS` default to `300000`. This note does not give those two pids a new elapsed time.

The later exits below are operator-reported `OPEN_MS TIMEOUT` results. This Linux tree does not contain the Mac log lines, so it does not invent an elapsed millisecond count for them. On the open-timeout path that string fails closed as `capture_not_available`. It is not a click.

| Pid | Result | OPEN_MS | tccutil | Click |
| --- | --- | --- | --- | --- |
| 65422 | `OPEN_MS TIMEOUT` | no elapsed-ms line in this note | still denied | not a click |
| 71152 | `OPEN_MS TIMEOUT` | no elapsed-ms line in this note | still denied | not a click |
| 76134 | `OPEN_MS TIMEOUT` | no elapsed-ms line in this note | still denied | not a click |
| 82331 | `OPEN_MS TIMEOUT` | no elapsed-ms line in this note | still denied | not a click |
| 89220 | `OPEN_MS TIMEOUT` | no elapsed-ms line in this note | still denied | not a click |

Pid 76134 was `taco run --source default` at 01:55 KST and was gone at 01:59:48 KST on PR #44 tip `d78e7f8`. That note did not print `OPEN_MS TIMEOUT` and did not signal the pid. Pid 82331 was still that command when PR #46 tip `7fb5006` packed Taco.app, and that pack did not signal it. This Partial is the later report that both of those pids, with 65422, 71152, and 89220, ended `OPEN_MS TIMEOUT`.

`tccutil` is still denied for `ai.syndai.taco`. The last copied list, 01:59 KST on tip `d78e7f8`, printed `denied` for `tccutil list -s Microphone -b ai.syndai.taco` and for `tccutil list -s AudioCapture -b ai.syndai.taco`. `tccutil list -b ai.syndai.taco` was empty. A pair-query `denied` with no service row is the same output as a bundle that has never been asked. It is not an operator Deny. No later list in this note adds a service row.

Taco.app re-prompts are still being raised after pid 89220. The last watched pid was 97484. That pid is a live re-prompt, not a timeout recorded here, and not a click. Unset `OPEN_MS` on the packed app uses `300000`. No Allow. No Deny.

## Consequence

Partial — operator Allow/Deny unproven; consequence AC1 open.

Checking M0-AC1 requires this Partial to be replaced and both an Allow tip SHA and a Deny tip SHA to be written above. Neither SHA exists. A timeout, a disappeared dialog, or `tccutil` printing `denied` does not fill either line.

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the intended CLI artifact, including allow/deny and cleanup.
