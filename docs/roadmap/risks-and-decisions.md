# Risks, unresolved decisions, and change rules

This is the implementation decision register. Entries are deliberately unresolved
until the named milestone produces evidence. They do not block completion of the
documentation plan; they block the relevant implementation or release gate.

## Goals

Expose assumptions before they become expensive architecture commitments. Every
choice has an accountable role, a decision deadline, an experiment, and a consequence
if the expected result does not hold.

## Non-goals

No speculative alternative stack catalog, automatic waiver process, invented staffing
or completion dates, or silent reduction of charter scope to suit a passing demo.

| ID | Decision or risk | Owner / deadline | Evidence and default direction | If unresolved or failed |
| --- | --- | --- | --- | --- |
| R1 | Can a single Mach-O receive both microphone and audio-capture TCC prompts in both terminal hosts? | macOS owner / M0 | Signed artifact, both usage strings embedded, allow/deny in the host assigned to each corner; proposed 14.2 floor | Stop dependent capture; revise packaging or the OS floor explicitly |
| R2 | Can whisper.cpp and the llama.cpp revision Hy-MT2 needs coexist in one Mach-O? | native runtime owner / M0 | Recheck the publisher kernel requirement, then same-process Metal load/teardown with shaders embedded | Fix linkage inside one binary; no helper process, sidecar dylib, or CPU-only fallback |
| R3 | Which Silero runtime supports incremental, bounded VAD without upstream CLI defaults? | audio owner / M0 | Frame shape, state reset, forced 15-second split, zero owned overlap, license, packaging | A maintained static runtime may be evaluated. Batch `whisper_full` VAD, a hand-written model, and a user-installed ONNX dylib do not pass |
| R4 | Which main-repo Hy-MT2 quantization, instruction language, and sampling arm meets the joint gates? | inference owner / M0 | Development-corpus comparison. Full language names. Live token cap, not the card's 4096-token document setting. Raw completions | Revise settings in the decision record. Do not switch to the 1.25-bit or 2-bit repos, a cleanup layer, or another model family |
| R5 | Can both native engines cancel and release ownership within the shutdown budget? | runtime owner / M0 | Real Metal cancellation under active decode and faults | Block backend choice; detached worker or future timeout is insufficient |
| R6 | Which two qualification corners are actually testable? | release owner / M0 | Proposed C1 is 14.2 on M1 with 16 GiB. C2 is a different Apple Silicon Mac and a named current OS build | Revise the corner in the open, or obtain the missing machine. A newer Mac does not prove C1 |
| R7 | Which source/target directions are qualified first? | product/quality owner / M0 | Proposed en→zh and zh→en, corpus rights, two bilingual reviewers, model-language intersection | Choose documented directions with real reviewers; do not claim all upstream languages |
| R8 | Which acceptance-plan numbers survive measurement, including the memory slope and forced-split overlap of zero? | audio/runtime/quality owners / M0 | Idle-hour noise, sustained speech, both scheduling arms. Upstream VAD defaults are not candidates unless a decision adopts one | Revise acceptance.md in the open. Do not redefine `t_audio_end`, drop failures, or widen the slope band past 1 MiB/min |
| R9 | Does app selection include helper processes reliably and isolate other audio? | macOS owner / M0 semantics, M5 full proof | Explicit process-instance identity, browser and simple-app isolation fixtures | Narrow selector semantics honestly; keep selected-app category mandatory |
| R10 | Does the model cache safely pass verified data to path-only native loaders? | model owner / M0 design, M1 proof | Handle loading or equivalent owned immutable artifact strategy, replacement tests | Block model readiness until integrity/load boundary is resolved |
| R11 | Are model/license/notices and clean distribution requirements satisfied? | release owner / M0 inventory, M7 artifact | Separate engine/model license records, required notices and signed install evidence | Block affected artifact distribution; do not infer model terms from engine license |
| R12 | Are latency and quality feasible under sustained speech and shared GPU load? | inference/quality owners / M0 baseline, M4 integrated proof | Both scheduling policies, long/noisy fixtures, zero-drop denominator | Optimize measured bottleneck or revise proposal explicitly; no hidden skips |
| R13 | Do logs/native errors or dependency startup leak content or attempt network? | privacy/runtime owner / M1, M7 final | Sentinel file/log tests, native logging control, process-attributed network observation | Remove behavior and rerun affected gates; blocking network alone is not a pass |
| R14 | Can full source/UI/OS release evidence be reproduced? | release/quality owners / M0 provision, M7 execute | Named machines/devices/apps/terminal hosts, fixture rights and measurement permissions | Mark cells not run and hold release; no aggregate “works on my Mac” verdict |

## Decision record format

For each resolved row, add date, responsible reviewer, exact chosen option, rejected
alternative and reason, artifact/experiment evidence, affected ADR and acceptance-plan values,
and remaining qualification work. A decision can accept the architecture while
leaving M7 qualification outstanding. Do not mark a risk resolved solely because its
proposed mitigation was implemented; inspect whether the experiment actually passed.

## Change and stop rules

- Changes to product scope, model family, source categories, runtime process shape,
  or privacy invariants require a deliberate charter/ADR decision, outside implicit
  implementation discretion. This documentation task leaves the charter untouched.
- Changes to decoder settings, endpointing, quantization, prompts, scheduling or
  resampling require affected quality/latency/resource tests again. Cache/network
  changes require P1/P3 again. Native capture/lifecycle changes require affected source,
  fault, and endurance cells again. Packaging changes require clean-install evidence.
- If exact-artifact identity changes during M7, label prior results with the old
  identity and review the dependency impact. The final candidate must have a coherent
  evidence set; results from unrelated binaries are not interchangeable.
- Do not add retries that retain unbounded audio or context, or that hide a broken
  native worker. Preserve the original failure, stop when required, and fix its cause.
- Taco operating limits change only in [acceptance.md](acceptance.md), in the same
  change as a decision-record entry. Do not keep a second tunable copy in code or
  in an ADR. Publisher figures cited as traps are not taco limits.
- The change that implements a milestone does not mark that milestone's checkboxes.
  A later review marks them when the linked evidence can be reproduced.

## M0 stop records (2026-10-16)

The contract freeze landed in the `taco` crate. The native spikes did not run.
This host is Linux x86_64, so a Mach-O, Metal, CoreAudio, or process-tap pass
would be fabricated. None of these rows accept M0. Milestone checkboxes stay
open. Dependent milestones M1 and M5 stay blocked until the named evidence
exists on Apple Silicon.

| ID | Decision | Owner | Evidence on this host | Rejected alternative | Consequence |
| --- | --- | --- | --- | --- | --- |
| R1 | Stop. Not run. | macOS owner | No signed arm64 artifact and no TCC prompt. `T-M0-SPIKE-SOURCES` and `T-M0-ONE-MACHO` record `not_run`. | Treat an ELF binary or a unit-test Mach-O fixture as the distribution spike. | Blocks M0 acceptance, M1, and M5. |
| R2 | Stop. Not run. Digests and the llama.cpp commit stay TBD. | native runtime owner | `PINS_MEASURED` is false. Families are Silero, multilingual Whisper small, and main-repo Hy-MT2 1.8B GGUF. `T-M0-LINK-THREE-MODELS` is native-only and fails closed. | Subprocess `whisper-cli` / `llama-completion`, `-ngl 0`, a helper dylib, or the 1.25-bit and 2-bit repos. | Blocks M0 acceptance, M1, and M5. |
| R3 | Stop. Not run. | audio owner | No incremental Silero runtime was linked. Forced-split behavior is a contract (`max_duration`, zero owned overlap), not a model result. | Batch `whisper_full` VAD, a hand-written model, or a user-installed ONNX dylib. | Blocks M0 acceptance and M2's real VAD. M1 stays blocked with the rest of M0. |
| R4 | Stop. Not run. No quantization, instruction winner, or sampling arm is selected. | inference owner | `EXPERIMENT.ran` is false and every selected field is empty. Baseline instruction remains the publisher English template; that is the ADR starting point, not a comparison result. Publisher temperature 0.7 is an arm, not the active sampler (`SELECTED_SAMPLING` is none). | Pick a quant or temperature without the development-corpus comparison. | Blocks M0 acceptance and M1's releasable bundle. |
| R5 | Stop. Not run. | runtime owner | `T-M0-CANCEL-LIFECYCLE` fails closed under `--features native`. Shutdown arithmetic is frozen at 8 s drain + 2 s join. | `pthread_cancel`, a detached worker, or a wrapper timeout. | Blocks M0 acceptance, M1, and M5. |
| R6 | Corners are named and unqualified. | release owner | C1 stays "M1 class, 16 GiB, macOS 14.2". C2 stays "a different Apple Silicon machine" with no OS build. Neither `qualified` flag is set. | Call this Linux VM a corner, or call "latest macOS" an OS build. | Blocks the support claim, M1, and M5. |
| R7 | Closed table frozen: `en→zh` and `zh→en`. Experimental pairs empty. Quality is not qualified. | product/quality owner | `language::PAIRS` and the reject tests. No corpus has been scored. | Auto-detect, identical tags, or the publisher language inventory. | The table is binding. Adequacy gates stay blocked. M1 still waits on M0 acceptance. |
| R8 | Proposed numbers are frozen in `fixtures/m0/limit-vectors.txt` and were not revised. Measurement was not run. | audio/runtime/quality owners | `T-M0-LIMIT-VECTORS` fails if the fixture drifts from acceptance.md. | Redefine `t_audio_end`, drop failures, or widen the slope past 1 MiB/min. | R8 stays open for a later measured revision. Quality gates stay blocked. |
| R9 | Selector semantics frozen: a failed app selection does not become system capture. Helper attribution is unproven. | macOS owner | `SelectionFailurePolicy::StopDoNotBroaden`. No process tap ran. External input device name is unset and `qualified_in_m0` is false. | Name a Linux device as the M2 external input. | Blocks M5 and the M2 device reservation. |
| R10 | Design frozen: the session takes verified local handles and has no fetch client. Inode proof is M1. | model owner | `preflight` / `fetch_models` refuse while pins are TBD. `release_ready()` is false. | A session URL or an HTTP client parameter. | M1 must prove the load boundary. M1 does not start before M0 is accepted. |
| R11 | License records exist per model and per engine. SPDX fields are unset. | release owner | `LICENSES` keeps model subjects distinct from whisper.cpp and llama.cpp. | Invent license text, or treat an engine license as the model license. | Blocks distribution of weights. |
| R12 | Stop. Not run. | inference/quality owners | No serialized-vs-concurrent measurement. | A microbenchmark or a CPU timing presented as the joint gate. | Blocks M0 acceptance and M4's baseline. |
| R14 | Corners, terminal hosts, and devices are not provisioned. | release/quality owners | C2 OS build and the alternate terminal host are unnamed. | An aggregate "works on this VM" verdict. | Release cells stay not run. M1 and M5 stay blocked. |

## Acceptance criteria

- [ ] Every M0 decision has a concrete resolution before dependent milestone exit.
- [ ] Every remaining release-blocking row is resolved with evidence before M7 passes.
- [ ] Each change that affects a frozen contract names the invalidated evidence and
  completes the necessary requalification before the updated product is called done.
