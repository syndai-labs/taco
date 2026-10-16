# Implementation contract

This contract binds every agent that implements taco. It does not implement the
product, and it does not relax the [charter](../../CHARTER.md). The roadmap
status is **not started**. A written milestone is not an accepted milestone.

## Authority

When two statements disagree, the higher source wins. Fix the lower source; do
not pick the convenient reading.

1. [CHARTER.md](../../CHARTER.md)
2. Accepted ADRs, then proposed ADRs. **Proposed means binding but not yet
   confirmed by evidence.** It does not mean optional.
3. Numeric caps, corpora, and verdict rules in [acceptance.md](acceptance.md)
4. The milestone being implemented
5. This contract and the [test catalog](test-catalog.md)

An implementation that needs a lower source to beat a higher one stops. Record a
decision request in [risks-and-decisions.md](risks-and-decisions.md). Do not
"temporarily" ship the workaround.

## Before writing milestone code

1. Read the charter, this contract, that milestone, the ADRs it cites, the
   acceptance plan, and that milestone's catalog tests.
2. Confirm every dependency milestone is accepted by evidence, not merely
   written. M0 is not done because these documents exist.
3. Add the milestone's default-tier tests first. A missing type may fail
   compilation. Once the type exists, the test must fail because the behavior is
   absent, not because the test asserts `false`.
4. Implement only that milestone's work packages, behind the shared `taco`
   library types landed in M0.
5. Run the default tier before any native or manual tier.
6. Leave every acceptance checkbox unchecked. Attach evidence instead.

## Product shape that later milestones inherit

- One library and one user-facing binary, both named `taco`. The repository
  directory may be `cap`. Do not ship `taco-spike`, a second TUI binary, or a
  crate-per-stage workspace. A static library is allowed only when the M0 link
  record says symbol isolation requires it.
- M0 freezes shared types: selectors, events, outcomes, errors, and stage
  traits. Later milestones fill those types in. They do not invent a parallel
  session path.
- The capture trait has input, system, and application variants in M0/M2.
  M2 implements only the input backend. M5 implements taps behind the same
  trait and the same ring, clock, and gap path.
- CLI and TUI are renderers over one ledger. M2's status output is the early
  form of that renderer, not a prototype to discard.
- Session construction accepts verified local model handles and configuration.
  It has no URL, HTTP client, or fetch function. Only `taco models fetch` may
  use the network, and only as its own invocation.

## Closed vocabularies

Do not add a sibling concept under a new name.

| Concept | Rule |
| --- | --- |
| Source categories | Exactly four: built-in microphone, a distinct external input, system output, selected application. |
| Selector | `default`, `input:<device-uid>`, `system`, `app:<bundle-id>`, `pid:<pid>`. Names are labels, not selectors. |
| Language | One explicit pair per session from the M0 closed table. No auto-detect, no identical source and target, no pass-through of raw engine strings. |
| User tag | A pinned tag such as `en` or `zh`. The translation prompt receives the publisher's full language name, not the tag. |
| Experimental pairs | Empty unless M0 adds an explicit allowlist. Upstream language counts are not that list. |
| Segment identity | One id is born when the utterance opens and is the commit id. Drafts do not get a second id. |
| Top-level translation outcome | `complete`, `failed`, `skipped_overload`, `timed_out`, `cancelled`. Detail codes live inside `failed`. |
| Endpoint reason | `silence`, `max_duration`, `stop_flush`. |
| Session taint | Sticky for the process lifetime of the session. Returning from Degraded to Running does not clear it. |
| Exit code | Defined only in [ADR 0006](../adr/0006-session-and-terminal.md). |

`failed` details are `native_error`, `too_long`, `token_exhausted`,
`empty_output`, and `bad_output`. Empty ASR text is not a commit and is not one
of these outcomes. It is an untranscribed `empty_decode` interval.

The first terminal outcome wins. A commit already waiting in the translation queue
is `cancelled` or `timed_out` on shutdown, not `skipped_overload`.
`skipped_overload` means the queue was full and the commit was refused. A timeout
that later observes a native return stays `timed_out`.

## Behavior that is easy to "improve" and still wrong

- Stdin, files, and URLs are not audio sources. There is no config file.
- There is no `--insecure`, `--translate`, `--auto-detect`, model URL, dump-audio
  flag, or dump-transcript flag. A test-only observer keeps fixtures in memory
  or in the external harness, not in the release binary.
- Do not persist audio, transcripts, prompts, or translations, including for
  debugging. Terminal output the user asked for is the exception.
- Do not translate drafts, rewrite a commit, keep a chat history, or send more
  than the current segment as translation context.
- Do not spawn a replacement inference worker when the resident worker is still
  alive. A native call that misses its deadline and does not return is fatal to
  the session.
- Do not detach threads, `pthread_cancel`, or `process::exit` to imitate
  cancellation. Use the engine abort callback and join.
- Capture callbacks do not allocate, log, resample, run inference, wait, or call
  Tokio. Publish into a preallocated wait-free ring or drop the frame and bump a
  wait-free loss counter. Loss counters remain readable when the status channel
  is full.
- Status history is a bounded ring. Dropping old status must not drop a commit,
  an outcome, or a loss count.
- Forced splits use zero owned-sample overlap. Upstream VAD defaults, including
  overlap and silence-only splitting, are not taco defaults.
- Model input follows the window rule in the acceptance plan. `t_audio_end` is
  the last spoken sample of the owned interval, not the end of endpoint silence.
- CLI does not enter raw mode and does not treat `q` as a command. `q` is TUI
  only. CLI stop is a signal.
- A recoverable skip or per-segment failure continues the session, taints it,
  and still fails a normal-load qualification cell. Do not stop the product on
  the first dropped buffer, and do not delete the drop from the release verdict.
- Device loss, permission revocation, route change for the selected source,
  sleep/wake, a stuck native worker, and an output stall past its deadline stop
  the session. They are not Degraded states.
- Permission prompts happen at most once per session attempt. Do not loop them.
- Build-time crate downloads are not session network activity. Runtime attempts
  are, even when the connection fails.

## Model-card traps

The Hy-MT2 card documents engines and settings that violate this product. Do not
copy them into the session path.

- No Python, transformers, vLLM, SGLang, Ollama, llama-server, `llama-completion`,
  or `whisper-cli` subprocess.
- The card's `-ngl 0` example is CPU. v0.1 inference must show Metal execution.
- The card's `max_tokens: 4096` is a document-translation setting. The live
  completion budget is the acceptance cap.
- Do not take the 1.25-bit or 2-bit sibling repositories as the way to pass the
  memory cap.
- Do not expose the publisher's full language list.
- Pin the default translation instruction and the chat template from the chosen
  model revision. Do not add a system prompt, terminology/style templates, or
  multi-turn history. The card says the model has no default system prompt.
- M0 rechecks the llama.cpp revision required by the chosen GGUF, including any
  publisher kernel patch, and records the commit. A patch is not permission to
  fork the architecture into a helper process.
- Sampling parameters are the M0 decision. Publisher temperature `0.7` is one
  experiment arm, not an implied default.
- A complete subtitle is the raw completion after pinned stop sequences, trimmed
  only of surrounding whitespace. Do not strip explanations to force a pass.

## Tests

Tiers:

| Tier | Command | May touch |
| --- | --- | --- |
| default | `cargo test` | Fakes, temp directories, local fake transports |
| native | `cargo test --features native` | Pinned models and Metal. Feature is off by default |
| manual | `scripts/qual/<test-id>` plus an evidence record | Devices, TCC, sleep, soaks, human review |

Default tests must pass on an Apple Silicon Mac without models, microphones,
accounts, or network. A test that needs any of those is in the wrong tier.

Do not weaken a catalog assertion, delete a test to go green, or `#[ignore]` a
default-tier test. Do not assert a limit by reading the implementation's own
constant. Exceed the cap and observe the specified refusal or degradation.
Imported limit vectors from `T-M0-LIMIT-VECTORS` are the allowed copy of the
acceptance numbers.

The same agent pass that writes the code does not check the milestone boxes.
Boxes flip only with linked evidence a reviewer can reproduce: command, binary
and model digests, logs, and the verdict. Missing evidence is **not run**, never
pass.

## Stop

Stop and write a decision request when a catalog test cannot pass without
breaking the charter, an ADR, or an acceptance gate. Dependent milestones wait.
In particular, do not introduce an inference server, a cloud engine, a virtual
audio driver, ScreenCaptureKit, or a dropped source category in order to finish
a milestone.
