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

The contract freeze is `828bd48`. Early measurements are in
[macos-spike-results.md](../evidence/m0/macos-spike-results.md). The Info.plist
bind follow-up is in [native-impl.md](../evidence/m0/native-impl.md) (`3602000`).
None of these rows accept M0. Milestone checkboxes stay open. M1 and M5 stay blocked.

Host: Apple M1 Pro, 32 GiB (34359738368 bytes), macOS 27.0.1 (26A434),
`aarch64-apple-darwin`, rustc 1.99.0. Not C1 (16 GiB, macOS 14.2). Not a named C2.

| ID | Decision | Owner | Evidence on this host | Rejected alternative | Consequence |
| --- | --- | --- | --- | --- | --- |
| R1 | Stop. Capture not reached. Packaging bind is fixed; digests are measured; session still lacks a capture backend. | macOS owner | At `234c26a`, `scripts/sign-debug.sh` yields `Info.plist entries=5`. Pin measurement recorded three digests (`pin-measurement.md`) and `PINS_MEASURED` is true. Bound `taco run` previously stopped at `models_unpinned` before pins; with pins, preflight should reach `capture_not_available` until CoreAudio/process-tap lands. No TCC prompt yet. `T-M0-SPIKE-SOURCES` remains `verdict=not_run`. | Treat linker-signed unbound, inspector `Pass`, `models_unpinned`, or `capture_not_available` as a TCC/capture result. | Blocks M0 acceptance, M1, and M5 until real capture evidence lands. |
| R2 | Digests measured; Metal link still not run. llama.cpp commit stays TBD. | native runtime owner | `docs/evidence/m0/pin-measurement.md` records sha256 for `silero_vad.onnx`, `ggml-small.bin`, and `Hy-MT2-1.8B-Q4_K_M.gguf`. `PINS_MEASURED` is true with those filenames. Files were hashed, not loaded. The Metal compiler is installed (32023.921) and was not used. `T-M0-LINK-THREE-MODELS` did not execute. | Subprocess `whisper-cli` / `llama-completion`, `-ngl 0`, a helper dylib, another family's GGUF, or the 1.25-bit and 2-bit repos. | Blocks M0-AC2 until in-process Metal load. M1/M5 still blocked. |
| R3 | Stop. Not run. | audio owner | No incremental Silero runtime was linked. Forced-split behavior is a contract (`max_duration`, zero owned overlap), not a model result. | Batch `whisper_full` VAD, a hand-written model, or a user-installed ONNX dylib. | Blocks M0 acceptance and M2's real VAD. M1 stays blocked with the rest of M0. |
| R4 | Stop. Not run. No quantization, instruction winner, or sampling arm is selected. | inference owner | `EXPERIMENT.ran` is false and every selected field is empty. No development-corpus comparison ran on this host. Baseline instruction remains the publisher English template; that is the ADR starting point, not a comparison result. Publisher temperature 0.7 is an arm, not the active sampler (`SELECTED_SAMPLING` is none). | Pick a quant or temperature without the development-corpus comparison. | Blocks M0 acceptance and M1's releasable bundle. |
| R5 | Stop. Not run. | runtime owner | `t_m0_cancel_lifecycle` panicked: `no Metal worker was cancelled or joined inside the shutdown reserve`. Shutdown arithmetic stays 8 s drain + 2 s join. | `pthread_cancel`, a detached worker, or a wrapper timeout. | Blocks M0 acceptance, M1, and M5. |
| R6 | Corners are named and unqualified. | release owner | This worker is M1 Pro, 32 GiB, macOS 27.0.1 (26A434). C1 stays "M1 class, 16 GiB, macOS 14.2". C2 stays "a different Apple Silicon machine" with no OS build. Neither `qualified` flag is set. | Call this Mac C1, or call "macOS 27.0.1" the unnamed C2 build without a decision. | Blocks the support claim, M1, and M5. |
| R7 | Closed table frozen: `en→zh` and `zh→en`. Experimental pairs empty. Quality is not qualified. | product/quality owner | `language::PAIRS` and the reject tests. No corpus was scored on this host. | Auto-detect, identical tags, or the publisher language inventory. | The table is binding. Adequacy gates stay blocked. M1 still waits on M0 acceptance. |
| R8 | Proposed numbers are frozen in `fixtures/m0/limit-vectors.txt` and were not revised. Measurement was not run. | audio/runtime/quality owners | No latency, memory, or slope measurement on this host. `T-M0-LIMIT-VECTORS` still fails if the fixture drifts from acceptance.md. | Redefine `t_audio_end`, drop failures, or widen the slope past 1 MiB/min. | R8 stays open for a later measured revision. Quality gates stay blocked. |
| R9 | Selector semantics frozen: a failed app selection does not become system capture. Helper attribution is unproven. | macOS owner | `app:com.apple.Terminal` exited `models_unpinned` before a tap. `SelectionFailurePolicy::StopDoNotBroaden`. External input device name is unset and `qualified_in_m0` is false. | Treat `models_unpinned` as an app-tap result, or name a device that was not reserved. | Blocks M5 and the M2 device reservation. |
| R10 | Design frozen: the session takes verified local handles and has no fetch client. Inode proof is M1. | model owner | `taco run` refused with `models_unpinned`. `release_ready()` is false. Pins stay TBD. | A session URL or an HTTP client parameter. | M1 must prove the load boundary. M1 does not start before M0 is accepted. |
| R11 | License records exist per model and per engine. SPDX fields are unset. | release owner | No new license text was measured. `LICENSES` keeps model subjects distinct from whisper.cpp and llama.cpp. | Invent license text, or treat an engine license as the model license. | Blocks distribution of weights. |
| R12 | Stop. Not run. | inference/quality owners | No serialized-vs-concurrent measurement. The installed Metal compiler was not a timing run. | A microbenchmark or a CPU timing presented as the joint gate. | Blocks M0 acceptance and M4's baseline. |
| R14 | Corners, terminal hosts, and devices are not provisioned. | release/quality owners | This shell on macOS 27.0.1 is a measurement host, not a qualified corner. C2 OS build and the alternate terminal host are unnamed. No second terminal host was exercised. | An aggregate "works on this Mac" verdict. | Release cells stay not run. M1 and M5 stay blocked. |

## Residency partial (2026-10-17)

Stop. Not a pass. M0-AC2 and M0-AC3 stay unchecked.

The retry is in [engines-residency-retry.md](../evidence/m0/engines-residency-retry.md).
Eight-cycle probes still missed 33554432 bytes. `vm_deallocate` of every
mapping at least 8 MiB returned, including the 487014400-byte weight buffer,
and `phys_footprint` stayed about 90 MiB above the cycle-1 teardown. Those
patches were measured and are not in this tree.

One process ran 100 `prove_native` teardowns. `phys_footprint` after cycle 100
was 334972056 bytes, 252346632 above the cycle-1 teardown (82625424). The
catalog tolerance is 33554432. A command-buffer flush after Metal
`endResidency` was measured for 6 cycles and reverted: cycle 6 was still
98812072 bytes above cycle 1. The flush is not in the tree.

| Decision | Owner | Evidence | Rejected alternative | Consequence |
| --- | --- | --- | --- | --- |
| Stop. Repeated in-process teardown does not return `phys_footprint` to the cycle-1 baseline. | runtime owner | `docs/evidence/m0/engines-residency.md` and `docs/evidence/m0/engines-residency-retry.md`. The retry's vmtrace freed every `vm_allocate` of at least 8 MiB, including 487014400 bytes, and cycle 2 was still 94699688 bytes above cycle 1. Residency sets off, a pre-free synchronize, and a post-deallocate blit each missed 33554432 within 8 cycles. Those patches are not in the tree. | Call the 100-cycle run a pass, keep sessions warm across cycles, or keep a GPU wait that missed the same band. | `T-M0-LINK-THREE-MODELS` cycle sentence and the acceptance.md 100-cycle rule are unmet. Do not start M1 on the claim that engine teardown is residually bounded. |

## Acceptance criteria

- [ ] Every M0 decision has a concrete resolution before dependent milestone exit.
- [ ] Every remaining release-blocking row is resolved with evidence before M7 passes.
- [ ] Each change that affects a frozen contract names the invalidated evidence and
  completes the necessary requalification before the updated product is called done.
