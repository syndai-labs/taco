# M5 — System and selected-application audio

Status: **Not started**. Depends on: M4 and the M0 tap/distribution proof.
Accountable: macOS capture owner; reviewers: runtime and privacy owners.
Decision: [ADR 0002](../adr/0002-audio-capture.md).

## Goal

Let users select either the system output stream or a supported application source
and obtain the same bounded bilingual pipeline as microphone/input capture. Prove
selection isolation, permission behavior, and owned resource cleanup on real macOS.

## Non-goals

No source mixing, microphone-plus-system capture, virtual driver, output muting,
protected-content bypass, automatic broadening from app to system, or app restart
tracking that silently attaches to a new process instance.

## Work packages

1. Implement the system and application backends of the M2 capture trait. Reuse that
   trait's ring, clock, and gap path. Do not add a second resampler or commit
   pipeline. Carry over explicit lifetime ownership, capability discovery, selectors,
   and a private aggregate device that is not the default output.
2. Resolve application/helper-process selection semantics, distinguish ambiguity and
   unsupported sources, and pin the selected instance for the session.
3. Integrate channel/format/timestamp normalization and discontinuity handling through
   the existing pipeline. Avoid a separate system-audio resampling/commit implementation.
4. Test app exit, playback route change, sleep/wake, permission revoke, source silence,
   OS API errors, and repeated partial startup/teardown from the distributable binary.

## Tests to write first

Write these [catalog](test-catalog.md) tests. Fake CoreAudio covers rollback.
Licensed signals in two applications cover isolation. One application is
multi-process and one is single-process.

- `T-M5-REVERSE-CLEANUP`
- `T-M5-ISOLATION-SIGNALS`
- `T-M5-NO-BROADEN`
- `T-M5-PERMISSION-ONCE`
- `T-M5-ROUTE-AND-SLEEP`
- `T-M5-SILENCE-NOT-PERMISSION`
- `T-M5-CYCLES-100`
- `T-M5-SAME-CONTRACT`
- `T-M5-SMOKE-15`

## Acceptance criteria

- [ ] **M5-AC1:** System output and a selected application each produce bilingual
  output through the same segment contract as inputs, on both qualification corners.
  The 60-minute accuracy matrix remains M7. The smoke bar is M5-AC6.
- [ ] **M5-AC2:** Isolation fixtures prove source selection and helper-process behavior.
  Ambiguous/restarted/unsupported app selectors never capture broader audio silently.
- [ ] **M5-AC3:** Allow, deny, revoke, and recovery instructions work from the shipped
  identity in the terminal host the acceptance matrix assigns to that corner.
  Permission prompts are not retried in a loop.
- [ ] **M5-AC4:** App exit, route changes, source loss, and sleep/wake follow the defined
  stop/discontinuity policy with bounded cleanup and no crash. Silence is not falsely
  diagnosed as denial merely because no samples contain speech.
- [ ] **M5-AC5:** At least 100 start/stop and injected initialization-failure cycles
  leave no taco-owned aggregate devices/taps/listeners and do not mute, reroute, or
  remove unrelated audio devices.
- [ ] **M5-AC6:** `T-M5-SMOKE-15` meets the tap smoke gate for system and for
  application on every declared corner. These smokes do not replace the M7
  60-minute cells.

## Evidence and handoff

Deliver an explicit supported-source/OS matrix, permission runbook, isolation results,
cleanup inventory, and integrated smoke reports. Unsupported apps may be documented
only with a concrete limitation; the system and selected-app categories themselves
remain mandatory. M6 receives consistent source capabilities and actionable errors.
