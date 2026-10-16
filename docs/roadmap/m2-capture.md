# M2 — Bounded microphone/input capture and speech windows

Status: **Not started**. Depends on: M0 and M1.
Accountable: audio owner; reviewer: runtime owner.
Decisions: [ADR 0001](../adr/0001-process-and-pipeline.md),
[0002](../adr/0002-audio-capture.md), [0003](../adr/0003-segment-contract.md).

## Goal

Select a microphone or another input device and convert its live audio into bounded,
timestamped speech windows. Demonstrate that stalled consumers cannot block native
capture and that gaps, device loss, and shutdown preserve correct ownership.

## Non-goals

No translated subtitles, system/app production adapter, user recording/replay, source
mixing, dynamic source switching, or automatic reconnect after device loss.

## Work packages

1. Implement the source-neutral capture trait with input, system, and application
   variants. Implement only the CPAL input backend in this milestone, plus listing,
   unambiguous selection, permission errors, callback ownership, and the
   preallocated ring.
2. Normalize supported native PCM into mono 16 kHz off the callback with explicit
   downmixing, bounded resampler state, sample-time mapping, and format validation.
3. Integrate the M0-selected Silero runtime with bounded pre-roll, endpoint delay,
   utterance cap, silence suppression, and gap resets.
4. Add a minimal CLI status view, a synthetic source/clock for tests, queue counters,
   and supervisor stop handling. This is infrastructure for the vertical slice, not
   a second temporary pipeline to be discarded in M6.

## Tests to write first

Write these [catalog](test-catalog.md) tests. Default-tier cases use fixtures and
fakes. Real devices are only the manual IDs.

- `T-M2-PIN-SOURCE`
- `T-M2-RESAMPLE-20MS`
- `T-M2-CALLBACK-WAITFREE`
- `T-M2-STALL-FRESH`
- `T-M2-FORCE-15S`
- `T-M2-SILENCE-NO-GROWTH`
- `T-M2-GAP-EPOCH`
- `T-M2-NO-STDIN-SOURCE`
- `T-M2-LIFECYCLE-100`
- `T-M2-FAKE-SOURCE`
- `T-M2-SLEEP-WAKE`
- `T-M2-NO-PERSIST`

## Acceptance criteria

- [ ] **M2-AC1:** A built-in microphone and a named non-default external input produce
  speech windows with the documented sample format and bounded timestamp error; duplicate
  device labels and default-device changes cannot silently change the selected source.
- [ ] **M2-AC2:** Instrumentation and callback code inspection show no allocation,
  blocking lock/channel wait, logging, resampling, or inference in the callback.
- [ ] **M2-AC3:** Forced consumer stalls keep memory within the shared limits, count
  dropped intervals, reset VAD/utterance continuity, and resume with fresh audio.
- [ ] **M2-AC4:** On the development timing fixtures, speech windows cover at least 98%
  of annotated speech duration and never exceed the model-window rules in the
  [acceptance plan](acceptance.md). Continuous input forces a `max_duration` boundary.
  Silence does not grow utterance storage. This milestone does not claim WER or
  translation adequacy.
- [ ] **M2-AC5:** Permission denial, device disappearance, sleep/wake, and partial
  startup return typed errors, prompt at most once, and do not use a freed callback.
  One hundred in-process cycles stay within the cycle tolerance and retain no
  capture resources.
- [ ] **M2-AC6:** `T-M2-FAKE-SOURCE` drives the same window path as CPAL. Stop works
  while data and status queues are saturated. Audio never enters logs or files.
  Real microphone and external-input evidence complements the fakes.

## Evidence and handoff

Deliver normalized audio/segment fixture results, timing error measurements, callback
audit, saturation/lifecycle tests, and named-device smoke reports. M3 consumes bounded
utterance windows and discontinuity events. No ASR quality claim is made yet.
