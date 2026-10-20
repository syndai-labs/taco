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
The AC4 Partial freeze is [ac4-partial.md](../evidence/m0/ac4-partial.md).
M0-AC4 stays unchecked. Partial is not an accept.
Charter accept is blocked: M0-AC4 cannot be checked while SIGNED_INSTALL=None or C2_BLOCKED stands.
The launchd user-agent run does not close M0-AC4.
Cited gaps, not a new signed-install measurement and not a second machine: PR #38 tip `e7433c2`
(`SIGNED_INSTALL=None`), PR #42 tip `8794303` (None guards), PR #27 tip `ad8d418`
(`C2_BLOCKED`), PR #43 tip `0c880fb` (adhoc launchd), PR #19 tip `8ebd0fc` (OS floor
Partial at macOS 27.0.1).

Host: Apple M1 Pro, 32 GiB (34359738368 bytes), macOS 27.0.1 (26A434),
`aarch64-apple-darwin`, rustc 1.99.0. Not C1 (16 GiB, macOS 14.2). Not a named C2.

| ID | Decision | Owner | Evidence on this host | Rejected alternative | Consequence |
| --- | --- | --- | --- | --- | --- |
| R1 | Partial. Product `taco run` dispatches `system`, `app:`, and `pid:` (PR #11 tip `cd9a4c8`). CoreAudio open and operator Allow/Deny stay Mac-only. Linux tap success is Stop. | macOS owner | At `234c26a`, `scripts/sign-debug.sh` yields `Info.plist entries=5`. Pin measurement recorded three digests (`pin-measurement.md`) and `PINS_MEASURED` is true. Bound `taco run` previously stopped at `models_unpinned` before pins. The "lacks a capture backend" line is that pre-wiring measurement. [process-tap-product.md](../evidence/m0/process-tap-product.md): this tree's `taco run` calls `session::start_capture` for `system`, `app:`, and `pid:`. Mac evidence in [process-tap.md](../evidence/m0/process-tap.md): `system`, `app:com.google.Chrome`, and `pid:688` exited 0 with no dialog. Exit 0 is not Allow or Deny. A Terminal miss is CoreAudio, not a binding bug. This Linux tree does not compile `native/process_tap.m`. `T-M0-SPIKE-SOURCES` remains `verdict=not_run`. Partial — process taps are on taco run; Mac CoreAudio and operator Allow/Deny stay open; Linux tap success is Stop. | Treat linker-signed unbound, inspector `Pass`, `models_unpinned`, `capture_not_available`, a Linux `cargo test`, process exit, Drop, or the #11 exit 0 as an operator Allow, Deny, or Cleanup. | Blocks M0-AC1, M1, and M5 until a Mac operator Allow, a Mac operator Deny, and a separately traced Cleanup are recorded. Linux tap success does not close that stop. |
| R2 | Digests measured; Metal link still not run. llama.cpp commit stays TBD. | native runtime owner | `docs/evidence/m0/pin-measurement.md` records sha256 for `silero_vad.onnx`, `ggml-small.bin`, and `Hy-MT2-1.8B-Q4_K_M.gguf`. `PINS_MEASURED` is true with those filenames. Files were hashed, not loaded. The Metal compiler is installed (32023.921) and was not used. `T-M0-LINK-THREE-MODELS` did not execute. | Subprocess `whisper-cli` / `llama-completion`, `-ngl 0`, a helper dylib, another family's GGUF, or the 1.25-bit and 2-bit repos. | Blocks M0-AC2 until in-process Metal load. M1/M5 still blocked. |
| R3 | Stop. Not run. | audio owner | No incremental Silero runtime was linked. Forced-split behavior is a contract (`max_duration`, zero owned overlap), not a model result. | Batch `whisper_full` VAD, a hand-written model, or a user-installed ONNX dylib. | Blocks M0 acceptance and M2's real VAD. M1 stays blocked with the rest of M0. |
| R4 | Stop. Not run. No quantization, instruction winner, or sampling arm is selected. | inference owner | `EXPERIMENT.ran` is false and every selected field is empty. No development-corpus comparison ran on this host. Baseline instruction remains the publisher English template; that is the ADR starting point, not a comparison result. Publisher temperature 0.7 is an arm, not the active sampler (`SELECTED_SAMPLING` is none). | Pick a quant or temperature without the development-corpus comparison. | Blocks M0 acceptance and M1's releasable bundle. |
| R5 | Stop. Not run. | runtime owner | `t_m0_cancel_lifecycle` panicked: `no Metal worker was cancelled or joined inside the shutdown reserve`. Shutdown arithmetic stays 8 s drain + 2 s join. | `pthread_cancel`, a detached worker, or a wrapper timeout. | Blocks M0 acceptance, M1, and M5. |
| R6 | Corners are named and unqualified. | release owner | This worker is M1 Pro, 32 GiB, macOS 27.0.1 (26A434). C1 stays "M1 class, 16 GiB, macOS 14.2". C2 stays "a different Apple Silicon machine" with no OS build. Neither `qualified` flag is set. | Call this Mac C1, or call "macOS 27.0.1" the unnamed C2 build without a decision. | Blocks the support claim, M1, and M5. |
| R7 | Closed table frozen: `en→zh` and `zh→en`. Experimental pairs empty. Quality is not qualified. | product/quality owner | `language::PAIRS` and the reject tests. No corpus was scored on this host. | Auto-detect, identical tags, or the publisher language inventory. | The table is binding. Adequacy gates stay blocked. M1 still waits on M0 acceptance. |
| R8 | Proposed numbers are frozen in `fixtures/m0/limit-vectors.txt` and were not revised. Measurement was not run. | audio/runtime/quality owners | No latency, memory, or slope measurement on this host. `T-M0-LIMIT-VECTORS` still fails if the fixture drifts from acceptance.md. | Redefine `t_audio_end`, drop failures, or widen the slope past 1 MiB/min. | R8 stays open for a later measured revision. Quality gates stay blocked. |
| R9 | Selector semantics frozen: a failed app selection does not become system capture. Helper attribution is unproven. | macOS owner | The 2026-10-16 `app:com.apple.Terminal` run exited `models_unpinned` before a tap. On this tip `taco run` reaches `session::start_capture` for `app:` and `pid:` ([process-tap-product.md](../evidence/m0/process-tap-product.md)). A HAL miss stays not-found and does not become a system tap. `SelectionFailurePolicy::StopDoNotBroaden`. `EXTERNAL_INPUT.display_name` is `syndai-iphone-02 Microphone` and `qualified_in_m0` is false. | Treat `models_unpinned` as an app-tap result, turn a HAL miss into a system tap, or name a device that was not reserved. | Blocks M5 and the M2 device reservation. |
| R10 | Design frozen: the session takes verified local handles and has no fetch client. Inode proof is M1. | model owner | `taco run` refused with `models_unpinned`. `release_ready()` is false. Pins stay TBD. | A session URL or an HTTP client parameter. | M1 must prove the load boundary. M1 does not start before M0 is accepted. |
| R11 | License records exist per model and per engine. SPDX fields are unset. | release owner | No new license text was measured. `LICENSES` keeps model subjects distinct from whisper.cpp and llama.cpp. | Invent license text, or treat an engine license as the model license. | Blocks distribution of weights. |
| R12 | Stop. Not run. | inference/quality owners | No serialized-vs-concurrent measurement. The installed Metal compiler was not a timing run. | A microbenchmark or a CPU timing presented as the joint gate. | Blocks M0 acceptance and M4's baseline. |
| R14 | Corners, terminal hosts, and devices are not provisioned. | release/quality owners | This shell on macOS 27.0.1 is a measurement host, not a qualified corner. C2 OS build and the alternate terminal host are unnamed. No second terminal host was exercised. | An aggregate "works on this Mac" verdict. | Release cells stay not run. M1 and M5 stay blocked. |

## Process-tap product path

PR #11 tip `cd9a4c8` is an ancestor of stack base `a1c5c32`. Product-path freeze is #75 tip `dc1b9be` (`dc1b9bef6735d37315e4a32ef0893dd2ac20df11`). That SHA is not an Allow, Deny, or Cleanup tip. Product `taco run` dispatches `system`, `app:`, and `pid:` through `session::start_capture`. Mac CoreAudio evidence stays in [process-tap.md](../evidence/m0/process-tap.md). This host does not open a tap. Allow tip SHA: none. Deny tip SHA: none. Cleanup tip SHA: none. M0-AC1 stays unchecked while Allow, Deny, or Cleanup tip SHA is still none. Drop is not a Cleanup tip SHA.

Partial — process taps are on taco run; Mac CoreAudio and operator Allow/Deny stay open; Linux tap success is Stop.

[ac1-full-session.md](../evidence/m0/ac1-full-session.md) cites PR #77 tip `595dced` (`595dcedafa5e7f2dcc829670e41c462f8ca6dab1`). Microphone Allow/Deny tip SHAs in [mic-operator-resume.md](../evidence/m0/mic-operator-resume.md) and AudioCapture Allow/Deny tip SHAs in [audiocapture-measure.md](../evidence/m0/audiocapture-measure.md) are none. TIMEOUT is not Deny. Partial — mic and AudioCapture Allow/Deny tip SHAs are none; consequence AC1 open. This note does not record a Mac click.

## Acceptance criteria

- [ ] Every M0 decision has a concrete resolution before dependent milestone exit.
- [ ] Every remaining release-blocking row is resolved with evidence before M7 passes.
- [ ] Each change that affects a frozen contract names the invalidated evidence and
  completes the necessary requalification before the updated product is called done.

## AC2/AC3 Stop consequence

Cites #76 tip `2d9679f` and [ac2-ac3-open.md](../evidence/m0/ac2-ac3-open.md). `ac2_ac3_cannot_flip_while_stop_shas_stand` is on this tip unchanged. M0-AC2 and M0-AC3 stay unchecked.

Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. Partial is not an accept. Checking either box requires [vad-metal-ep.md](../evidence/m0/vad-metal-ep.md) to drop `d194012` and [engines-residency.md](../evidence/m0/engines-residency.md) to drop `a0e1614`. R2 and R3 name the CPU VAD Stop. R5 names the residency Stop. Neither box above is a pass.

Newer Stops still stand: #55 tip `0829a70` (residual phys_footprint is the process-lifetime Metal device in `ggml_backend_metal_reg`) and #57 tip `88eda06` (`EnableOnSubgraphs`; ORT 1.31 `GetOpBuilders` lacks `LSTM` and `If`; `device=` stays `cpu`). Prior Stops stay cited: #49 tip `37ff031` and #41 tip `a8e2eca`.

#49 tip `37ff031` freed the 487014400-byte weight buffer and cycle 2 stayed 94699688 bytes above cycle 1. It does not drop `a0e1614`. #41 tip `a8e2eca` is the prior consequence freeze. #55 tip `0829a70` names the leftover mapping as the process-lifetime device `ggml_backend_metal_reg` installs; `llama_backend_free` does not destroy it. No purge is in this tree. #57 tip `88eda06` sets provider option `EnableOnSubgraphs` on the product session. ORT 1.31.0 `GetOpBuilders` has no `LSTM` builder and no `If` builder, so `device=` stays `cpu`. This branch does not replay those Mac runs and adds no Metal measurement.

#60 tip `aab661c` is a Stop: the Silero pin stays, and no published Silero ONNX is CoreML-acceptable under ORT 1.31 (other files still contain LSTM, or a top-level If whose branch fails the Pad and Slice checks). #63 tip `232ea78` deepens the #55 Metal-device Stop: pinned llama.cpp has no safe teardown of that process-lifetime device without a use-after-free. #64 tip `c9a6f6f` is a complementary Stop: an ORT upgrade is out of M0, because `main` `GetOpBuilders` is still the v1.31.0 blob and still has no `LSTM` or `If`. This branch does not copy that guard. Checking M0-AC2 or M0-AC3 still fails while `aab661c` and `232ea78` stand. This note adds no Metal measurement.

#69 tip `2ff5c4f` is a Stop: the VAD stays Silero-only, and the charter forbids an alternate VAD pin. #70 tip `e06ea0e` deepens the #55 and #63 Metal-device Stops: a Metal device refcount free is still a use-after-free on prove cycle 2. `0829a70` and `232ea78` stay. #64 tip `c9a6f6f` still owns `ort_coreml_upgrade_cannot_flip_ac2`; this branch cites that Stop and does not copy that guard. Checking M0-AC2 or M0-AC3 still fails while `2ff5c4f` and `e06ea0e` stand. This note adds no Metal measurement.

#74 tip `30fd669` is a Stop: a Metal helper process violates single Mach-O, in-process Metal, and LINK-THREE, and it does not close residency. #74 owns `metal_subprocess_stop_cites_55_63_and_70`; this branch cites that Stop and does not copy that guard. `0829a70`, `232ea78`, `e06ea0e`, and `2ff5c4f` stay. Checking M0-AC2 or M0-AC3 still fails while `30fd669` stands. This note adds no Metal measurement.
