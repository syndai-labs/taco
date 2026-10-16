# M4 — Local bilingual vertical slice

Status: **Not started**. Depends on: M3.
Accountable: translation owner; reviewers: quality and runtime owners.
Decisions: [ADR 0003](../adr/0003-segment-contract.md),
[0004](../adr/0004-local-inference.md), [0006](../adr/0006-session-and-terminal.md).

## Goal

Produce complete bilingual subtitles from a real input in one offline process.
Establish that the two resident inference engines can keep up together, and that
every committed segment has a visible, correctly associated translation outcome.

## Non-goals

No translation of drafts, chat interface, external context retrieval, TTS, unbounded
conversation memory, automatic target switching, or broad claims based on isolated
translation throughput. This milestone does not remove system/app capture from v0.1.

## Work packages

1. Implement one worker-owned Hy-MT2 context, per-segment prompt formatting and reset,
   supported-language validation, bounded input/output, and native cancellation.
2. Admit only immutable commits to a bounded FIFO. Define pending/complete/failed/
   skipped/timed-out/cancelled outcomes and preserve source text on translation failure.
3. Apply and measure the shared GPU scheduling policy with final ASR, optional drafts,
   and translation competing under realistic speech. Make degradation visible.
4. Present source/target associations through the shared state and minimal CLI, with
   final display timing, bounded output buffering, and cumulative outcome counters.

## Tests to write first

Write these [catalog](test-catalog.md) tests. Human and model scores use the
development corpus only.

- `T-M4-ONE-OUTCOME`
- `T-M4-EXACT-SOURCE-TEXT`
- `T-M4-NO-ROLLING-CONTEXT`
- `T-M4-TOO-LONG-FAILS`
- `T-M4-RAW-COMPLETION`
- `T-M4-LEDGER-EQUATION`
- `T-M4-OVERLOAD-SKIP`
- `T-M4-STUCK-WORKER-STOPS`
- `T-M4-PROMPT-IS-DATA`
- `T-M4-DEV-MT`
- `T-M4-SMOKE-15`
- `T-M4-FAKE-PIPELINE`

## Acceptance criteria

- [ ] **M4-AC1:** Every admitted source commit has exactly one terminal translation
  outcome with the same ID; draft changes trigger no translation and no final source
  text is rewritten. Shutdown reconciles all pending outcomes.
- [ ] **M4-AC2:** Prompt/context/token caps are enforced before native allocation;
  exhaustion and truncation cannot masquerade as complete translations. No per-session
  chat history accumulates inside the model context.
- [ ] **M4-AC3:** Each selected direction meets the development-corpus translation and
  end-to-end human gates. Reference-source and ASR-source scores are reported
  separately. Held-out labels are not used. Raw completions are scored; a cleanup
  layer cannot turn `bad_output` into `complete`.
- [ ] **M4-AC4:** `T-M4-SMOKE-15` meets the bilingual smoke gate. It does not claim
  the 60-minute slope, p99 at N≥200, or held-out adequacy. Both models stay loaded
  once, with no network activity and no content persistence.
- [ ] **M4-AC5:** Deliberate overload results in bounded visible failures, never callback
  blocking or unlimited queues. Removing overload restores fresh processing without
  replaying stale work or hiding the failure counters.
- [ ] **M4-AC6:** Slow output and native cancellation preserve stop responsiveness and
  terminal state. Measurements include display delivery, not just generated tokens.
- [ ] **M4-AC7:** `T-M4-FAKE-PIPELINE` produces the scripted bilingual ledger without
  CoreAudio or model files. Capture, ASR, translation, and view can each be replaced
  without changing outcome accounting.

## Evidence and handoff

Deliver the first end-to-end quality/latency/resource report, segment outcome
reconciliation, overload traces, and a reproducible bilingual demo. M5 can replace
the source adapter; M6 can replace presentation without changing inference contracts.
M7 still requires all source categories, both UIs, exact packaging, and 60-minute runs.
