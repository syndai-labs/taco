# M3 — Streaming source subtitles with immutable commits

Status: **Not started**. Depends on: M2.
Accountable: ASR owner; reviewers: audio and quality owners.
Decisions: [ADR 0003](../adr/0003-segment-contract.md),
[0004](../adr/0004-local-inference.md).

## Goal

Show revisable source drafts at the bounded cadence, and stable final source
subtitles from multilingual Whisper small. Turn bounded audio windows into a commit
stream whose text, single identity, ownership intervals, and order are safe for
translation.

## Non-goals

No target translation, diarization, speaker labels, retrospective correction of
commits, language auto-detection/switching, session transcript history, or a claim
that Whisper is a native incremental streaming model.

## Work packages

1. Implement worker-owned ASR with explicit source language, transcribe-only mode,
   native cancellation, bounded context, and the M0 scheduling policy.
2. Implement replaceable drafts and final decode at silence/forced/stop boundaries.
   Reserve capacity for final work; stale draft work cannot delay final commits.
3. Enforce unique segment IDs, immutable source text, sample ownership, gap resets,
   maximum text size, and bounded in-flight outcomes. Add source subtitles to the
   shared presentation state and minimal CLI.
4. Score ASR on the frozen corpus and inspect boundary failures separately from
   clean utterances. Tune only within reviewed limits and record all setting changes.

## Tests to write first

Write these [catalog](test-catalog.md) tests. Protocol cases use a fake ASR. Native
cases score the development corpus only and do not open held-out labels.

- `T-M3-ONE-COMMIT`
- `T-M3-DRAFTS-NOT-SUBMITTED`
- `T-M3-EMPTY-NOT-COMMIT`
- `T-M3-NO-OWNED-OVERLAP`
- `T-M3-STALE-EPOCH`
- `T-M3-QUEUE-CAP`
- `T-M3-SILENCE-10MIN`
- `T-M3-DEV-ASR`
- `T-M3-FAKE-ASR`

## Acceptance criteria

- [ ] **M3-AC1:** Draft revisions change only uncommitted text; every final segment has
  one ID, final text, language, owned sample interval, and endpoint reason. Repeated
  final callbacks cannot duplicate a commit.
- [ ] **M3-AC2:** Gaps discard unfinished hypotheses. Stop flush and forced boundaries
  do not overlap owned intervals. The scripted boundary fixture does not display a
  boundary word twice. Natural-speech duplication is scored later as quality, not
  waived here and not treated as a second commit.
- [ ] **M3-AC3:** Development-corpus ASR meets the frozen per-source-language gates,
  English by WER and Chinese by CER, including silence and continuous-speech
  coverage. Untranscribed annotated speech stays in the denominator. Held-out labels
  are not used.
- [ ] **M3-AC4:** Draft and final scheduling stays within memory/window/queue limits;
  a slow fake/model cannot block capture. Native work ends within the cancellation gate.
- [ ] **M3-AC5:** The M3 source smoke in the [acceptance plan](acceptance.md) shows
  progressive source subtitles and correct status on a real input. It is not the
  M7 release qualification.
- [ ] **M3-AC6:** `T-M3-FAKE-ASR` satisfies the same commit contract as the native
  adapter. Replacing the adapter does not change draft, commit, or discontinuity
  rules, and it does not submit drafts for translation.

## Evidence and handoff

Deliver commit-contract tests, annotated boundary examples, ASR score/error report,
model/settings identity, and input smoke results. M4 receives only immutable commits
and the shared outcome ledger, never an undocumented draft-to-translation shortcut.
