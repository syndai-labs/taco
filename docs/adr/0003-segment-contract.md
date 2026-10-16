# ADR 0003: Immutable commits over bounded streaming utterances

Status: **Proposed**. Validation: M0 contract fixtures; M2 VAD; M3 commit tests;
M4 outcome accounting.

## Context and goals

Whisper produces hypotheses from finite audio windows. Repeated windows are not by
themselves a stable streaming transcript protocol. taco must show useful drafts,
commit source text once, and translate only the committed text while retaining a
finite amount of audio and linguistic context.

## Non-goals

No diarization, transcript database, retrospective commit corrections, arbitrary
session-length context, incremental translation of draft text, or automatic switching
between multiple source languages during a session.

## Decision

Use Silero VAD for speech activity and endpointing. Maintain bounded pre-roll and an
active utterance window. Show optional replaceable ASR drafts while the utterance is
open. Commit only after a silence endpoint, a forced maximum-duration boundary, or a
graceful stop flush. `stop_flush` commits the open utterance immediately and does
not wait out the silence timer. `max_duration` does not append trailing silence
past the window cap. Trailing silence is only part of a `silence` endpoint. This endpoint-based policy is deliberately simpler than exposing
token-level confidence as an irreversible stability promise.

An utterance transitions `idle → speech → finalizing → committed` or ends as
`discarded` with an explicit reason. The id assigned when the utterance opens is
the commit id. Drafts do not receive a second id. Each commit owns a half-open
sample interval `[start, end)`. Window composition, the 15-second cap, pre-roll,
and trailing silence are defined only in the [acceptance plan](../roadmap/acceptance.md).
Pre-roll and trailing endpoint silence are context. They are not part of the owned
interval, and they must not repeat samples already owned by another commit. v0.1
uses zero owned overlap on forced splits. A non-zero overlap is a recorded M0
decision plus a passing boundary fixture, not an upstream default an agent may turn
on. Upstream VAD that splits only at silence, or that overlaps segments by default,
does not satisfy the cap.

On a capture gap, discard the unfinished draft/utterance, record an untranscribed
interval, clear VAD/resampler state as appropriate, and start a new continuity epoch.
Never commit a sentence assembled from audio on both sides of unknown loss. Already
committed segments are not edited or removed by this reset. Empty or whitespace-only
ASR output is `empty_decode` on the untranscribed ledger, not a commit and not a
translation input. A scripted boundary fixture must not display the same boundary
word twice. Duplicate words in natural speech are a quality finding under the
acceptance rubric, not a second commit protocol.

## Event schema to freeze in M0

| Event | Required semantics |
| --- | --- |
| `Draft` | Session ID, continuity epoch, utterance ID, revision, sample range, bounded text. The utterance ID is the future commit ID. A newer revision replaces the old draft |
| `CommittedSegment` | That same ID, epoch, owned sample interval, source language, final source text, endpoint reason (`silence`, `max_duration`, or `stop_flush`), monotonic commit time |
| `TranslationOutcome` | Same ID; target language; one top-level outcome (`complete`, `failed`, `skipped_overload`, `timed_out`, `cancelled`); completion timestamp; never changes source text |
| `Discontinuity` | Source generation/epoch, loss range or unknown-duration flag, reason; no private audio/text |
| `SessionStatus` | Lifecycle state, source identity, language pair, model identities, bounded counters, actionable error code |

These are internal types, not a public JSON API. The CLI record format in
[ADR 0006](0006-session-and-terminal.md) is the v0.1 text rendering of the same
ledger. IDs never repeat within a session. The first terminal outcome wins. A retry
inside a worker cannot emit a second commit or replace that outcome. Source
generation identifies the selected capture instance; continuity epoch identifies
uninterrupted audio within it; a separate job token controls native work. A gap
invalidates unfinished ASR work from the old epoch. Translations of commits already
admitted remain valid across that gap. If that job is cancelled, it receives one
`cancelled` outcome and late results are ignored. A new session must not accept work
from a previous session because timestamps match. `skipped_overload` is only for a
commit refused because the translation queue was already full. Shutdown of admitted
work is `timed_out` or `cancelled`, not a skip. `failed` carries a detail of
`native_error`, `too_long`, `token_exhausted`, `empty_output`, or `bad_output`.

Every admitted commit stays in the counters after its UI row is evicted. Sample
accounting is separate from those counters: each captured sample in an epoch is in
exactly one of the live ring, a gap, one owned interval, an untranscribed interval,
or non-owned pre-roll or trailing context.

## Ordering, context, and caps

One final ASR job runs at a time on the ASR worker, and one translation job runs at
a time on the translation worker. A shared GPU permit may serialize those workers;
it must not queue a second job on a worker that already has one. Draft work is lower
priority and may be skipped. Translation consumes admitted commits in order; source
commits may appear before prior translations finish. The renderer associates results
by the single segment ID rather than by cursor position. Retain pending rows until
their terminal outcome; evict completed rows first. A bounded final-output queue
owns outcomes until delivered or explicitly failed.
Late duplicate results never resurrect evicted rows. The CLI writer either delivers
each final record or reports its documented output failure. Counters alone are not
evidence that the user received a subtitle.

Each text buffer has byte/token limits. Translation receives only the current commit
in v0.1; previous transcript context is not retained as a hidden chat history. Token
limits reserve space for instructions and output. If input cannot fit, mark the
segment as too long instead of silently truncating its meaning. If generation hits
the output cap before completion, emit an explicit failure, not a complete subtitle.
Native ASR decoder context is reset/bounded between utterances as validated in M0.

## Alternatives and consequences

Committing every partial hypothesis would yield low apparent latency at the cost of
immutable errors and duplicate translation. Waiting for arbitrarily long natural
silence would make latency and memory unbounded. Forced splits preserve finite
resource use but can hurt sentence quality, so they need a separate fixture stratum
and visible qualification results. Endpoint commit latency includes the endpoint
delay; it cannot be removed from performance reporting.

## Acceptance criteria

- [ ] Draft revisions can change repeatedly while only one final source commit is
  emitted and only that exact text is submitted for translation.
- [ ] Silence, short speech, long uninterrupted speech, repeated words, noise,
  timestamps near boundaries, forced splits, and stop flush have deterministic tests.
- [ ] No duplicate owned audio interval or segment ID is committed; intentional
  context overlap cannot duplicate displayed words in the boundary fixture suite.
- [ ] Gap/reset and cancellation reject stale results and account for discarded audio.
- [ ] A gap between source commit and translation completion preserves that valid
  translation; explicit job cancellation wins exactly once over a late completion.
- [ ] Every commit has exactly one terminal translation outcome by shutdown; bounded
  retention does not erase counters or keep text alive for the entire session.
