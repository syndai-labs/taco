# ADR 0007: Evidence-based quality and release qualification

Status: **Proposed**. Validation: M0 freezes the profile and corpus; M7 executes it
against the exact distributable artifact.

## Context and goals

The charter's done condition combines installation, model acquisition, source
selection, accurate bilingual subtitles, offline operation, a 60-minute session,
and finite memory. A short demo or successful compilation cannot establish it.
The primary metric starts at the end of spoken audio, not at translation invocation.

## Non-goals

No claim that all Apple Silicon hardware has the same performance, that all model
languages are equally accurate, that a finite soak proves all memory safety, or that
benchmark fixtures represent every accent/domain. No release based on an average
that hides a failing language pair or source mode.

## Decision

Use the [acceptance plan](../roadmap/acceptance.md) as the single source of numerical
budgets, fixture strata, the release matrix, and measurement rules. The initial
values are **proposed targets**, not measured performance. M0 must validate or
revise them in that file and in the decision register before dependent acceptance.
Never lower a gate silently, and never redefine speech-end or drop failures to make
a percentile pass. Development-corpus scores cannot stand in for the held-out M7
run. Freeze the final corners, bundle, and corpora before the release run.

Qualify the actual single executable and its exact model digests, engine revisions,
prompt and decoder settings, SDK, and the two qualification corners in the
acceptance plan. C2 does not prove C1, and the two corners are not a full
hardware-by-OS cross. The unmeasured proposal was an M1 with 16 GiB on macOS
14.2. That machine was not obtained. The OS floor is Partial at the measured
build macOS 27.0.1 (26A434). The 16 GiB memory proposal stays unmeasured. C1 is
the 32 GiB M1 Pro and is not qualified. A newer machine does not prove 14.2.

Measure from annotated utterance-end sample time mapped to a monotonic clock through
final subtitle presentation. Include capture buffering, VAD hangover, ASR queue and
final decode, translation wait/decode, and presentation. Instrument per-stage times
to explain regressions, but keep the end-to-end metric authoritative. Forced splits
have a separate boundary-latency distribution because their endpoint is synthetic.

Use aligned, licensed or consented fixtures for automated ASR scoring and bilingual
human review. Score translation both from reference source text (translation-only)
and actual committed ASR text (end-to-end) so ASR errors are not hidden. Report each
qualified direction separately. Unsupported/unqualified pairs must be identified in
the product; upstream language counts are not taco's quality guarantee.

Combine static resource bounds, deliberate stall/failure tests, repeated lifecycle
tests, and real 60-minute source/terminal runs. Observe attempted network activity with
network access available, and separately demonstrate an offline run. A blocked socket
is still an attempted connection and fails the session network invariant.

## Alternatives and consequences

Using upstream model benchmarks avoids work but says nothing about live segmentation,
quantization, prompts, capture paths, or terminal delays. A single long session catches
some leaks but misses permission/startup/teardown bugs. A broad OS/hardware promise
would be convenient marketing and unsupported evidence. This plan trades breadth for
an explicit qualification matrix while retaining all four charter source categories.

## Acceptance criteria

- [ ] The release report names exact artifacts, environment, fixture version, sample
  counts, denominators, exclusions, thresholds, and every pass/fail result.
- [ ] Normal-load accuracy, latency, and memory pass together with zero lost audio,
  skipped commits, missing translations, or hidden timeout failures.
- [ ] Every required source category and both terminal modes have real macOS evidence,
  not just synthetic adapters or compilation results.
- [ ] Offline/network and persistence observations cover errors and startup as well
  as steady state; signed-install tests cover a clean machine/user environment.
- [ ] Unrun or failed mandatory gates block release; no “provisional pass” substitutes
  for the charter's done condition.
