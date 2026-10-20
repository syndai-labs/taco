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

The contract freeze is `828bd48`. Measured pins are `5e9649c`
([pin-measurement.md](../evidence/m0/pin-measurement.md)). Early measurements are in
[macos-spike-results.md](../evidence/m0/macos-spike-results.md). The Info.plist
bind follow-up is in [native-impl.md](../evidence/m0/native-impl.md) (`3602000`).
The Metal link probe is [metal-link.md](../evidence/m0/metal-link.md), copied from
`cursor/m0-metal-link-b19b` tip `a8b2ebc` (PR #12). That tip is after `bc194a3`.
It links static ONNX Runtime 1.31.0 CPU archives into the probe and prints
`link_three=1`. Silero's execution provider is CPU. The probe is not `taco`.
The process-tap runs are [process-tap.md](../evidence/m0/process-tap.md), copied from
`cursor/m0-process-tap-b19b` tip `cd9a4c8` (PR #11). That tip includes the Terminal
HAL miss first recorded at `fcb3ae2`, the Chrome opens at `5958a71`, the `tccutil`
explanation at `d42010a`, and the named M2 input with the dialog no longer on
screen. License identifiers are
[license-inventory.md](../evidence/m0/license-inventory.md), copied from
`cursor/m0-whisper-spdx-b19b` tip `7ee17c8` (PR #18) on inventory `c04ab04`
(PR #15). Whisper small weights are MIT there. That commit's
`freeze::LICENSES` edit is not replayed here. R4 is
[r4-sampling.md](../evidence/m0/r4-sampling.md), copied from
`cursor/m0-r4-sampling-b19b` tip `1a52b8d` (PR #17), read together with metal
tip `a8b2ebc`. That note accepts no quantization or sampling arm. The OS
floor and launch parent are [os-floor.md](../evidence/m0/os-floor.md), copied
from `cursor/m0-ac4-os-host-b19b` tip `8ebd0fc` (PR #19). The floor is Partial
at macOS 27.0.1 (26A434). The alternate host is a launchd user agent, named
and not exercised. C2 stays open. That commit's `freeze` constants are not
replayed here. Engine adapters are PR #14 tip `05c0c73`. The macOS static-link scaffold is PR #16
tip `570364c`. On that tip, product `initialize`, `infer`, and `cancel` still refuse.
Silero and ORT are not in that hook. PR #21 tip `f2b1720`
([ort-macho-guards.md](../evidence/m0/ort-macho-guards.md)) refuses a shared
`libonnxruntime` dylib as a ship path. Documented ORT stays static archives and
system frameworks (ADR 0004). That tip's product CMake does not link ORT.
`T-M0-LINK-THREE-MODELS` and `T-M0-CANCEL-LIFECYCLE` still panic. Probe output
from `a8b2ebc` is not a pass. PR #23 tip `7f98ab1`
([taco-engines.md](../evidence/m0/taco-engines.md)) is product `taco`, one
arm64 Mach-O that statically links ONNX Runtime 1.31.0 CPU. `otool -L` is
system libraries only. `session::open_inference` keeps init, infer, cancel,
close, and prove. On that Mac, `T-M0-LINK-THREE-MODELS` and
`T-M0-CANCEL-LIFECYCLE` passed through `session::prove_native` and printed
`link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`. Silero's ORT provider is CPU.
Whisper VAD still rejects the ONNX (bad magic). ASR and MT used Metal. Joins
were inside 2 s. M0-AC2 and M0-AC3 stay open.
PR #22 tip `a1654eb` holds the microphone spike for `TACO_CAPTURE_SPIKE_HOLD_MS`
after open returns `Ok`. PR #24 tip `89759b9`
([mic-hold.md](../evidence/m0/mic-hold.md)) source-asserts that hold. Neither
records an Allow or a Deny. Native probe and tap sources stay on
those branches. This branch does not relink ggml, ORT, or CoreAudio.

PR #30 tip `e2ec751` ([taco-run-prove.md](../evidence/m0/taco-run-prove.md))
records the Mac `taco run` of the `af2af02` gate. `TACO_ENGINES_PROVE=1` on an
ad-hoc signed release `taco` printed `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`
and exited 0. Silero's provider was CPU (`vad_ort=ok device=cpu`). The signature
is ad-hoc, not Apple Development or Developer ID. That run called `prove_native`
once. PR #31 tip `c7e7c2e` ([mic-open-timeout.md](../evidence/m0/mic-open-timeout.md))
bounds `open_default_microphone` with `TACO_CAPTURE_SPIKE_OPEN_MS` (default
`180000` ms) and then fails closed as `capture_not_available`. It does not press
Allow or Deny. PR #32 tip `0972b2d`
([engines-residency.md](../evidence/m0/engines-residency.md)) ran 100
`prove_native` cycles in one process (pid 29510). `phys_footprint` after cycle
100 is 334972056 bytes, 252346632 above the cycle-1 teardown of 82625424, and
218792200 over the 33554432 tolerance. `ep=CPU`. The binary is the residency
test, not `taco run`. `MTLDevice.currentAllocatedSize` was not read. M0-AC2 and
M0-AC3 stay open because the VAD provider is CPU and the footprint is over
tolerance. This branch does not replay those sources.

PR #35 tip `7ae6d98` ([ac2-ac3-open.md](../evidence/m0/ac2-ac3-open.md))
closes the gap between `prove_native` and `taco run`. #30 tip `e2ec751`
reached prove from an ad-hoc signed `taco run` and still printed `ac2=0 ac3=0`.
CPU VAD stays open. The residency series is a Stop at #32 tip `a0e1614`, not a
fix. `whisper_free` retains the bytes. A Metal command-buffer flush missed the
band (6-cycle delta 98812072) and was reverted. The `0972b2d` numbers above
stand. M0-AC2 and M0-AC3 stay unchecked. PR #36 tip `81a2a54`
([mic-open-timeout.md](../evidence/m0/mic-open-timeout.md)) sets
`TACO_CAPTURE_SPIKE_OPEN_MS` default to `300000` (5 minutes). The #31 default
`180000` timed out before the operator reached the dialog. Timeout still fails
closed as `capture_not_available`. No Allow or Deny. PR #37 tip `d194012`
([vad-metal-ep.md](../evidence/m0/vad-metal-ep.md)) is a VAD CPU Stop. Silero
stays on `CPUExecutionProvider`. ORT 1.31.0 has no Metal execution provider. A
static CoreML link requested `MLComputeUnits=CPUAndGPU` and
`ModelFormat=MLProgram`. Every input stayed on `CPUExecutionProvider`.
`vad_ort_plan=missing`. `vad_ort=ok device=cpu`. That Mach-O is not the
three-model cargo test and not `taco run`. PR #38 tip `e7433c2`
([signed-install.md](../evidence/m0/signed-install.md)) is an R11 signed-install
Stop on macOS 27.0.1 (26A434). `Developer ID Application` is missing. Notary
profile `taco-notary` is missing (`notarytool` exit 69). `spctl --assess`
rejected the linker-signed binary and both ad-hoc copies (exit 3). Ad-hoc
`TeamIdentifier=not set`. An Apple Development re-sign did not finish. This
branch does not replay #35, #36, #37, or #38.

None of these rows accept M0. M0-AC1, M0-AC2, M0-AC3, and M0-AC4 stay open.
Milestone checkboxes stay unchecked. M1 and M5 stay blocked.

Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. Partial is not an accept. Checking either box requires [vad-metal-ep.md](../evidence/m0/vad-metal-ep.md) to drop `d194012` and [engines-residency.md](../evidence/m0/engines-residency.md) to drop `a0e1614`. R2 and R3 name the CPU VAD Stop. R5 names the residency Stop. Newer Stops still stand: #55 tip `0829a70` (residual phys_footprint is the process-lifetime Metal device in `ggml_backend_metal_reg`) and #57 tip `88eda06` (`EnableOnSubgraphs`; ORT 1.31 `GetOpBuilders` lacks `LSTM` and `If`; `device=` stays `cpu`). Prior Stops stay cited: #49 tip `37ff031` and #41 tip `a8e2eca`. This branch does not replay those Mac runs and adds no Metal measurement.

[mic-operator-timeouts.md](../evidence/m0/mic-operator-timeouts.md) records PR #51 tip `7c71d50`. TIMEOUT is not Don't Allow. Allow tip SHA: none. Deny tip SHA: none. Partial — operator Allow/Deny unproven; consequence AC1 open.

[ac4-partial.md](../evidence/m0/ac4-partial.md) records PR #54 tip `6a839de`. `SIGNED_INSTALL` stays `None`. `C2_BLOCKED` stays true. Charter accept is blocked: M0-AC4 cannot be checked while SIGNED_INSTALL=None or C2_BLOCKED stands. The launchd user-agent run does not close M0-AC4.

PR #50 tip `fdc201f` locks unset `TACO_CAPTURE_SPIKE_OPEN_MS` at `300000` and `TACO_CAPTURE_SPIKE_HOLD_MS` at `1_800_000`. This branch does not replay `src/capture/microphone.rs`.

M0-AC5 and M0-AC6 stay unchecked while the AC1 Partial and the AC4 Partial stand.

Host for the cited Mac runs: Apple M1 Pro, 32 GiB (34359738368 bytes), macOS
27.0.1 (26A434), `aarch64-apple-darwin`, rustc 1.99.0. PR #19 tip `8ebd0fc`
names that build as a Partial OS floor, not a support promise. C2 stays open.
PR #30 tip `e2ec751` and PR #32 tip `0972b2d` used that same host. `proves_c1` stays false. Neither is C2. The #30 signature is ad-hoc. PR #37 tip `d194012` used that same host. It does not name C2. PR #38 tip `e7433c2` measured signing on macOS 27.0.1 (26A434). It does not name C2.

| ID | Decision | Owner | Evidence on this host | Rejected alternative | Consequence |
| --- | --- | --- | --- | --- | --- |
| R1 | Stop. Chrome and system taps exited 0 without a dialog. The later Don't Allow/Allow dialog was unread, then gone, and no button was clicked. | macOS owner | Tip `cd9a4c8`, [process-tap.md](../evidence/m0/process-tap.md). `TACO_CAPTURE_SPIKE=1` `--source system`, `app:com.google.Chrome`, and `pid:688` exited 0. `tccd` logged a microphone preflight only (`authValue=2`, `authReason=2`), attributed to responsible process Cursor (`com.todesktop.230313mzl4w4u92`, `/Applications/Cursor.app`) with accessor `ai.syndai.taco`. No `AUTHREQ_PROMPTING`. `tccutil list` said Microphone granted, AudioCapture denied, and SystemAudioRecording denied for that Cursor bundle. `ai.syndai.taco` had no listed services. Those words are `tccutil`'s and are not a mapping of `authValue=2`. A prior Microphone record for the responsible app is consistent with a preflight that does not prompt. Exit 0 is not an AudioCapture authorization: the Chrome slice had no AudioCapture request, and a denied AudioCapture record did not become a dialog or a CLI deny. Later, `tccutil reset Microphone com.apple.Terminal` printed success and the next list still said denied. `osascript` `tell application "Terminal" to do script` of `taco run --source default` did not start `taco` and did not return. A dialog with Don't Allow and Allow was on screen. Its title was not read, so it is not a microphone or AudioCapture prompt. Neither button was pressed. The `osascript` process was killed. At 00:27 KST that dialog was not on screen. No `taco` process was running. No completed Terminal-launched `taco run` output was present. No Allow or Don't Allow click was observed. That absence is not an allow or a deny. `T-M0-SPIKE-SOURCES` stays `verdict=not_run`. PR #19 tip `8ebd0fc` sets the OS floor to Partial macOS 27.0.1 (26A434) and names a launchd user agent that was not exercised. That note records no Allow and no Deny, so it does not close this stop. PR #22 tip `a1654eb` and PR #24 tip `89759b9`, [mic-hold.md](../evidence/m0/mic-hold.md). After `open_default_microphone` returns `Ok`, the stream is held for `TACO_CAPTURE_SPIKE_HOLD_MS` (u64 milliseconds). Unset or not a u64 uses `1800000` (30 minutes). `TACO_CAPTURE_SPIKE` still gates the open. The hold does not cover a CPAL wait inside open. PR #24 `microphone_spike_hold_follows_open_ok` reads that source and requires default `1_800_000`. No TCC is faked. No Allow or Deny is recorded, so this stop stays open. PR #31 tip `c7e7c2e`, [mic-open-timeout.md](../evidence/m0/mic-open-timeout.md). `open_default_microphone` waits at most `TACO_CAPTURE_SPIKE_OPEN_MS`. Unset or not a u64 uses `180000` (3 minutes), then fails closed as `capture_not_available`. The timeout does not join the CPAL worker and does not press Allow or Deny. `TACO_CAPTURE_SPIKE` and `TACO_CAPTURE_SPIKE_HOLD_MS` are unchanged. No Allow or Deny is recorded, so this stop stays open. PR #36 tip `81a2a54`, [mic-open-timeout.md](../evidence/m0/mic-open-timeout.md). The unset default of `TACO_CAPTURE_SPIKE_OPEN_MS` is `300000` (5 minutes). The #31 default `180000` timed out before the operator reached the dialog (`OPEN_MS TIMEOUT`, pids 39380, 54200, and others). The variable still overrides the default. Timeout still returns `capture_not_available` and does not press Allow or Deny. `TACO_CAPTURE_SPIKE_HOLD_MS` is unchanged. No Allow or Deny is recorded, so this stop stays open. | Treat `tccutil` granted/denied, `authValue=2`, exit 0, or the dialog's disappearance as an operator Allow or Deny. Record the unread dialog as either prompt. | Blocks M0-AC1, M1, and M5 until microphone and audio-capture prompts are shown, with a pressed allow, a pressed deny, and observed cleanup, in both terminal hosts. |
| R2 | Partial. Whisper small and Hy-MT2 loaded on MTL0 in the probe and again in the PR #23 native cargo tests. Silero ran on static ORT CPU, not on Metal. The process printed `ac2=0`. PR #30 tip `e2ec751` is an ad-hoc Mac `taco run` that printed `ac2=0`. PR #32's 100-cycle footprint is over tolerance. PR #35 tip `7ae6d98` closes prove-versus-run and names the residency Stop at `a0e1614`. CPU VAD stays open. PR #37 tip `d194012` still prints `device=cpu`. | native runtime owner | Tip `a8b2ebc` (PR #12), [metal-link.md](../evidence/m0/metal-link.md). Same Mach-O. In-process SHA-256 matched the three pins. `ggml-small.bin` is type small, `use gpu = 1`, `MTL0 total size = 487.01 MB`, `whisper_full` returned 0 with `vad` false. `Hy-MT2-1.8B-Q4_K_M.gguf` is `hunyuan-dense`, `n_gpu_layers=-1`, `offloaded 33/33 layers to GPU`, `MTL0_Mapped` 1075.74 MiB plus `CPU_Mapped` 193.57 MiB, greedy smoke 34 bytes. `silero_vad.onnx` still fails `whisper_vad_init_with_params` (`invalid model data (bad magic)`). Static ORT 1.31.0 CPU then ran it: inputs `input` float `[-1,-1]`, `state` float `[2,-1,128]`, `sr` int64 scalar; outputs `output` float `[-1,1]`, `stateN` float `[-1,-1,-1]`. Frames of 512 at 16 kHz carried state `[2,1,128]` and the zero-state reset matched. `otool -L` is system frameworks only. The probe printed `link_three=1` `ac2=0` and exited 0. That flag is the probe predicate, not `T-M0-LINK-THREE-MODELS`. On that probe tip the catalog test still panics. The probe is not the `taco` binary. VAD's execution provider is CPU. PR #14 tip `05c0c73` adds fail-closed adapters. PR #16 tip `570364c` can link whisper.cpp and llama.cpp into `taco` when `TACO_LLAMA_SRC` and `TACO_WHISPER_SRC` are set; product `initialize`, `infer`, and `cancel` still refuse, and Silero and ORT are not in that hook. PR #23 tip `7f98ab1`, [taco-engines.md](../evidence/m0/taco-engines.md), is product `taco`, not the probe. It statically links ONNX Runtime 1.31.0 CPU and keeps init, infer, cancel, close, and prove. `otool -L` is system libraries only. `T-M0-LINK-THREE-MODELS` and `T-M0-CANCEL-LIFECYCLE` passed on that Mac through `session::prove_native` and printed `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`. Whisper VAD still reports bad magic. Silero's ORT provider is CPU. ASR and MT used Metal. Those cargo tests are not `taco run`. M0-AC2 stays open. PR 22836 stays an unmerged CPU STQ kernel and was not required for the GGUF load. PR #30 tip `e2ec751`, [taco-run-prove.md](../evidence/m0/taco-run-prove.md), records that run of the `af2af02` gate: ad-hoc signed release `taco`, `TACO_ENGINES_PROVE=1`, stdout `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`, exit 0. `vad_ort=ok device=cpu`. Whisper VAD still failed. ASR and MT used MTL0. Signature is ad-hoc, not Apple Development or Developer ID. One `prove_native` call. PR #32 tip `0972b2d`, [engines-residency.md](../evidence/m0/engines-residency.md): `phys_footprint` 82625424 after cycle 1 and 334972056 after cycle 100, 218792200 bytes over the 33554432 tolerance. `ep=CPU`. The binary is `t_m0_residency`, not `taco run`. M0-AC2 stays open. PR #35 tip `7ae6d98`, [ac2-ac3-open.md](../evidence/m0/ac2-ac3-open.md), closes prove reachable from `taco run` on the #30 ad-hoc run and leaves CPU VAD open. It records the residency series as a Stop at `a0e1614`, not a fix. PR #37 tip `d194012`, [vad-metal-ep.md](../evidence/m0/vad-metal-ep.md), linked CoreML and the session still assigned every input to `CPUExecutionProvider`. `vad_ort_plan=missing`. `vad_ort=ok device=cpu`. M0-AC2 stays open. | Subprocess `whisper-cli` / `llama-completion`, `-ngl 0`, a helper dylib, a ggml VAD substituted for the pinned ONNX, `whisper_full` batch VAD, another family's GGUF, the 1.25-bit and 2-bit repos, or `libonnxruntime.dylib`. | Blocks M0-AC2 while the run prints `ac2=0`. A CPU Silero provider does not pass. M1 and M5 stay blocked. Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. |
| R3 | Partial. Static ONNX Runtime 1.31.0 CPU ran incremental frames in the probe and in the PR #23 native cargo tests. Silero's provider is CPU. No Silero runtime is accepted. PR #37 tip `d194012` is a VAD CPU Stop: CoreML was linked and the session still assigned `CPUExecutionProvider`. | audio owner | Tip `a8b2ebc` (PR #12), [metal-link.md](../evidence/m0/metal-link.md). Digest matched `1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`. `whisper_vad_init_with_params` still rejects the ONNX before any frame loop. ORT session reports the shapes above, `device=cpu`, frame probs 0.000592 then 0.000528, `state_l2` 8.108584 then 11.574153, reset match 1. The release `libonnxruntime.1.31.0.dylib` was not linked. Missing `re2::RE2` symbols were closed with the build's `libre2.a`. Duplicate `libonnx_proto.a` symbols were closed by linking `libonnx.a` only. Forced 15-second split and zero owned overlap were not measured on `a8b2ebc`, and that probe did not package ORT into `taco`. PR #21 tip `f2b1720`, [ort-macho-guards.md](../evidence/m0/ort-macho-guards.md). A shared `libonnxruntime` dylib is not a ship path. Documented ORT stays static archives and system frameworks (ADR 0004). That tip's product CMake does not link ORT. `T-M0-LINK-THREE-MODELS` still panics. The `a8b2ebc` probe is not a pass. PR #23 tip `7f98ab1`, [taco-engines.md](../evidence/m0/taco-engines.md). Release `taco --features native` is one arm64 Mach-O, 38261776 bytes, statically linking ONNX Runtime 1.31.0 CPU archives. `otool -L` is system libraries only. `_OrtGetApiBase` is defined, and init, infer, cancel, close, and prove stay in the Mach-O. `session::prove_native` ran Silero frames on CPU (`vad_ort=ok device=cpu`) after Whisper VAD bad magic. The cargo tests are not `taco run`. Forced 15-second split and zero owned overlap were not measured. M0-AC2 stays open. PR #30 tip `e2ec751` printed `vad_ort=ok device=cpu` from that `taco run`. PR #32 tip `0972b2d` printed `ep=CPU` on every cycle. Forced 15-second split and zero owned overlap were not measured. No Silero runtime is accepted. M0-AC2 stays open. PR #35 tip `7ae6d98` leaves CPU VAD open. PR #37 tip `d194012`, [vad-metal-ep.md](../evidence/m0/vad-metal-ep.md). ORT 1.31.0 has no Metal execution provider. A static CoreML link requested `MLComputeUnits=CPUAndGPU` and `ModelFormat=MLProgram`. `vad_ort_provider` listed `CoreMLExecutionProvider` and `CPUExecutionProvider`. Every input was `CPUExecutionProvider`. `vad_ort_plan=missing`. `vad_ort=ok device=cpu`. Frame probabilities match the earlier CPU run (0.000592 then 0.000528). That Mach-O is not the three-model cargo test and not `taco run`. A hardware-inventory GPU line is not a VAD device. No Silero runtime is accepted. M0-AC2 stays open. | Batch `whisper_full` VAD, a hand-written model, a different VAD file, or a user-installed / sidecar ONNX dylib. | The probe and the PR #23 cargo tests ran frames on CPU ORT. Those tests are not `taco run`. M0 acceptance and M2's real VAD still wait on the forced split, owned overlap, and a run that does not print `ac2=0`. M1 stays blocked with the rest of M0. Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. |
| R4 | Stop. M0 accepts no quantization arm, no instruction winner, and no sampling arm. The pinned Q4_K_M file is the only main-repo quant that loaded on the probe. | inference owner | PR #17 tip `1a52b8d`, [r4-sampling.md](../evidence/m0/r4-sampling.md), and PR #12 tip `a8b2ebc`, [metal-link.md](../evidence/m0/metal-link.md). The R4 note was written on Linux and did not run Metal. It read the probe through `bc194a3`. Tip `a8b2ebc` is the later probe: same Q4_K_M load (`hunyuan-dense`, `n_gpu_layers=-1`, 33/33, `MTL0_Mapped` 1075.74 MiB, `CPU_Mapped` 193.57 MiB) and the same greedy smoke (`mt=ok text_bytes=34`). Pin sha256 `dc5f44fcf1fa496ee7ad725982c0c8c553a4de00259b53af84c4b89fb0c06699`. `quantizations_compared` is 0. No second main-repo quant was fetched. `EXPERIMENT.ran` is false. `SELECTED_SAMPLING` is none. Publisher temperature 0.7 was not executed. The seeded arm was not scored. `COMPARISON_WINNER` is none. The development corpus has 0 utterances and is unsealed. The 34-byte smoke is not a selection. Silero on `a8b2ebc` is CPU ORT in the probe, not a quant or sampling arm. PR #30 `e2ec751`, PR #31 `c7e7c2e`, and PR #32 `0972b2d` do not compare a quantization or execute a sampling arm. `SELECTED_SAMPLING` stays none. | Treat the Q4_K_M pin, `file_type=15`, the 34-byte smoke, or `link_three=1` as the joint-gate winner. Execute publisher temperature 0.7 with an empty development corpus. Switch to the 1.25-bit or 2-bit repos, a cleanup layer, or another model family. | Blocks the M0-AC4 quantization and sampling name, M0 acceptance, and M1's releasable bundle. Selected experiment fields stay empty. |
| R5 | Partial. Both engines' joins fit in 2 s in the probe and in the PR #23 native cargo tests. The process printed `ac3=0`. M0-AC3 stays open. PR #32's footprint did not return to the cycle-1 teardown. PR #35 tip `7ae6d98` records that miss as a Stop at `a0e1614`. | runtime owner | Tip `a8b2ebc` (PR #12), [metal-link.md](../evidence/m0/metal-link.md). After `native/patches/whisper-free-buffers-on-init-fail.patch`, the ORT run's Whisper decode returned -8 at hit 2, join 178 ms. Whisper init forced `n_dims` 99 after `MTL0 total size = 487.01 MB`, context null, join 111 ms, then `whisper_free`. Llama init join 240 ms at progress 0. Llama decode returned 2, join 8 ms. Reserve is 2000 ms. Process printed `teardown=returned` and exited 0 with `link_three=1`. No `GGML_ASSERT` on the residency set. The unpatched failure path exited 134. `ac3=0`. `t_m0_cancel_lifecycle` still panics. PR #16 tip `570364c` does not call cancel through the linked symbol. Shutdown arithmetic stays 8 s drain + 2 s join. The probe is not `taco`. PR #21 tip `f2b1720` keeps `T-M0-CANCEL-LIFECYCLE` panicking. The probe is not a pass. PR #23 tip `7f98ab1` links static ORT into product `taco` and keeps init, infer, cancel, close, and prove. `T-M0-CANCEL-LIFECYCLE` passed on that Mac through `session::prove_native`. Joins were inside 2000 ms and the process printed `cancel_lifecycle=1 ac3=0` and `teardown=returned`. Late completion, a replacement worker, and residency return were not recorded. Unbound `initialize`, `infer`, and `cancel` still return `engine_not_linked`. M0-AC3 stays open. On PR #23 that residency series was not recorded. PR #32 tip `0972b2d` later recorded it: cycle 100 is 252346632 bytes above cycle 1, outside the 33554432 band. Each cycle set `teardown=1` and printed `ac3=0`. Late completion and a replacement worker were not recorded. That process is the residency test. PR #30 tip `e2ec751` is one ad-hoc `taco run` with joins inside 2000 ms and `ac3=0`. It is not the 100-cycle series. M0-AC3 stays open. PR #35 tip `7ae6d98` names #32 tip `a0e1614` as a Stop, not a fix. `whisper_free` retains the bytes. A Metal command-buffer flush missed the band (6-cycle delta 98812072) and was reverted. That run does not meet `T-M0-LINK-THREE-MODELS` or the 100-cycle rule. M0-AC3 stays open. | `pthread_cancel`, a detached worker, a wrapper timeout, or `delete` of a context that still owns Metal buffers. | Blocks M0-AC3, M1, and M5. The native tests joined inside the reserve and printed `ac3=0`. Late completion, a replacement worker, and residency return were not recorded. Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. |
| R6 | Partial. The measured OS floor is macOS 27.0.1 (26A434). It is not a support promise. C2 is open. Neither corner is qualified. | release owner | PR #19 tip `8ebd0fc`, [os-floor.md](../evidence/m0/os-floor.md), branched from corners tip `f40f2f5`. The only measured build is Apple M1 Pro, 34359738368 bytes, macOS 27.0.1 (26A434). macOS 14.2 was not measured. `OS_FLOOR_PARTIAL` is true on that commit. This branch's `freeze::CORNERS` still says C1 is macOS 14.2; that crate edit is not replayed here. C2 has no chip, memory, or OS build. PR #30 tip `e2ec751` and PR #32 tip `0972b2d` measured this same Apple M1 Pro, 34359738368 bytes, macOS 27.0.1 (26A434). `proves_c1` stays false on #32. Neither tip names a second machine. C2 stays unnamed. PR #37 tip `d194012` measured Silero on this same Apple M1 Pro, macOS 27.0.1 (26A434). It does not name a second machine. C2 stays unnamed. | Treat this host as proof of a 16 GiB macOS 14.2 floor, assign build 26A434 to C2 without a second machine, or call 26A434 a qualified support promise. | C2 and the unconfirmed floor still block M0-AC4, the support claim, M1, and M5. |
| R7 | Closed table frozen: `en→zh` and `zh→en`. Experimental pairs empty. Quality is not qualified. | product/quality owner | `language::PAIRS` and the reject tests. No corpus was scored on this host. PR #30, #31, and #32 scored no corpus. The closed table is unchanged. | Auto-detect, identical tags, or the publisher language inventory. | The table is binding. Adequacy gates stay blocked. M1 still waits on M0 acceptance. |
| R8 | Proposed numbers are frozen in `fixtures/m0/limit-vectors.txt` and were not revised. The acceptance slope was not run. | audio/runtime/quality owners | No latency, memory, or slope measurement on this host. `T-M0-LIMIT-VECTORS` still fails if the fixture drifts from acceptance.md. PR #32 tip `0972b2d` records `phys_footprint` across 100 engine cycles, not an idle-hour slope. The delta is 252346632 bytes and fails the 32 MiB teardown band. That miss does not revise `fixtures/m0/limit-vectors.txt` or widen the 1 MiB/min slope. | Redefine `t_audio_end`, drop failures, or widen the slope past 1 MiB/min. | R8 stays open for a later measured revision. Quality gates stay blocked. |
| R9 | Partial. A failed app selection does not become system capture. Helper inclusion and isolation are unproven. The external input is named and not qualified. | macOS owner | Tip `cd9a4c8`, [process-tap.md](../evidence/m0/process-tap.md). `SelectionFailurePolicy::StopDoNotBroaden` held. Terminal was running (pid 42771) and was not a HAL client: `find_bundle` `osstatus=0` `count=29` `matches=0`; `find_pid` `object=0` `in_list=0`. Both selectors returned `capture_not_available`. `AudioHardwareCreateProcessTap` was not called for that miss. Chrome (`com.google.Chrome`, object=103, pid=688, `IsRunning=0`) was already on the list; `app:` and `pid:` exited 0. A one-shot list showed `com.google.Chrome.helper` as a different bundle. No helper was selected. No other-audio isolation fixture ran. `system_profiler SPAudioDataType` listed default input `MacBook Pro Microphone` and non-default `syndai-iphone-02 Microphone` (transport Unknown). The JSON record had no device uid, so none is stored. `EXTERNAL_INPUT.display_name` is `Some("syndai-iphone-02 Microphone")`. `qualified_in_m0` is false. PR #31 tip `c7e7c2e` bounds microphone open only. It does not select a helper or run an isolation fixture. | Treat the Terminal miss as a TCC deny, or broaden it to system capture. Treat the Chrome open as proof that helpers are included. Treat the iPhone mic name as an M0 qualification or as the default input. Invent a device uid. Drop the selected-app category. | Blocks M0-AC1 until allow and deny are taken, M5's selector proof, and M2 qualification of the reserved input. |
| R10 | Design frozen: the session takes verified local handles and has no fetch client. Inode proof is M1. | model owner | Digests are in [pin-measurement.md](../evidence/m0/pin-measurement.md) (`5e9649c`). `PINS_MEASURED` and `release_ready()` are true. `fetch_models` still returns `fetch_refused_unpinned`. Inode or handle proof of the load boundary was not run. The earlier `models_unpinned` refusal predates those pins. PR #30 tip `e2ec751` passed a model directory into `taco run`. It does not prove inode or handle loading. | A session URL or an HTTP client parameter. | M1 must prove the load boundary. M1 does not start before M0 is accepted. |
| R11 | Partial. SPDX and copyright lines are set on all five records from a fetched LICENSE. The permission text is not in the crate. Signed install is a Stop at PR #38 tip `e7433c2`: no `Developer ID Application` identity, notary profile `taco-notary` missing, and `spctl` rejected the ad-hoc Mach-O. | release owner | PR #18 tip `7ee17c8` on license inventory `c04ab04` (PR #15), [license-inventory.md](../evidence/m0/license-inventory.md). `silero-vad-weights` is MIT, notice `Copyright (c) 2020-present Silero Team`, from snakers4/silero-vad `1e261b0`. `hy-mt2-1.8b-gguf` is Apache-2.0, notice `Copyright (C) 2026 Tencent. All rights reserved.`, from Hugging Face `tencent/Hy-MT2-1.8B-GGUF` `LICENSE.txt` revision `a0c709d`. `whisper.cpp` `0c7be69` and `llama.cpp` `2a66703` are each MIT, notice `Copyright (c) 2023-2026 The ggml authors`. `whisper-small-weights` is MIT, notice `Copyright (c) 2022 OpenAI`, from openai/whisper `86098128` LICENSE. That commit's README says the code and model weights are under that license. The pin matches Hugging Face `ggerganov/whisper.cpp` `5359861` `ggml-small.bin` LFS oid and size. That tree has no LICENSE file. whisper.cpp `0c7be69` describes the file as a conversion of the OpenAI checkpoint. The conversion was not re-run. The card tag `license: mit` was not copied. Engine notices were not copied onto model rows. This branch does not replay `7ee17c8`'s `freeze::LICENSES` edit. No signed install. PR #30 tip `e2ec751` is an ad-hoc signature (`TeamIdentifier=not set`), not a signed install. SPDX rows are unchanged. PR #38 tip `e7433c2`, [signed-install.md](../evidence/m0/signed-install.md), measured that gap on macOS 27.0.1 (26A434), tree `607724e`. `security find-identity` listed one codesigning identity, `Apple Development: ops@syndai.com (CT93T5LALG)`. No `Developer ID Application` certificate. `codesign --sign "Developer ID Application"` printed `no identity found` and exited 1. `xcrun notarytool submit --keychain-profile taco-notary` printed `No Keychain password item found for profile: taco-notary` and exited 69. Ad-hoc `codesign -s - --identifier ai.syndai.taco` bound `Info.plist entries=5` with `TeamIdentifier=not set` (release `CDHash=434ca964d84bac7307406cd6532044347fa2792b`, debug `CDHash=82d87753911f3d623f726944bffdfe45c3c3a355`). `spctl --assess -vv --type execute` printed `rejected` and exited 3 for the linker-signed release binary and both ad-hoc copies. An Apple Development re-sign did not return and was stopped; that copy stayed linker-signed (`Info.plist=not bound`). Binaries were not executed. Nothing was installed. `freeze::SIGNED_INSTALL` stays `None` on that tip. This branch does not replay `scripts/sign-install.sh`. SPDX rows are unchanged. Weight distribution stays blocked. | Invent license text, or treat an engine license as the model license. | Blocks distribution of weights. |
| R12 | Stop. Not run. | inference/quality owners | No serialized-vs-concurrent measurement. The MTL0 load in metal-link.md (`a8b2ebc`) is not a timing run. PR #30's 9.35 s `taco run` and PR #32's per-cycle `elapsed_ms` (sum 209817) are not a serialized-versus-concurrent timing under shared GPU load. R12 stays not run. | A microbenchmark, a CPU timing, or that load presented as the joint gate. | Blocks M0 acceptance and M4's baseline. |
| R13 | Stop. Not run. | privacy/runtime owner | No sentinel file/log test and no process-attributed network observation is cited on PR #11 tip `cd9a4c8`, PR #12 tip `a8b2ebc`, PR #15 tip `c04ab04`, PR #17 tip `1a52b8d`, or PR #18 tip `7ee17c8`. PR #30 tip `e2ec751`, PR #31 tip `c7e7c2e`, and PR #32 tip `0972b2d` cite no sentinel file/log test and no process-attributed network observation. | Treat an absent log line as a privacy pass, or treat blocking the network as a pass. | Blocks M1's privacy proof and M7's final check. |
| R14 | Partial. The alternate TCC launch parent is named and was not exercised. C2 and devices stay open. | release/quality owners | PR #19 tip `8ebd0fc`, [os-floor.md](../evidence/m0/os-floor.md). The alternate host is a launchd user agent. `ALTERNATE_HOST_EXERCISED` is false on that commit. Cursor.app (`com.todesktop.230313mzl4w4u92`) is rejected. Apple Terminal was not a completed parent. C2's OS build is unnamed. No second GUI terminal was measured. The external-input slot below is named and not qualified. PR #32 tip `0972b2d` is the same measurement host, not C2, and not a second GUI terminal. `proves_c1` is false. PR #30 tip `e2ec751` is an ad-hoc `taco run` on that host, not a second machine and not the launchd agent. C2 stays open. PR #37 tip `d194012` is that same host, not C2, and not a second GUI terminal. | An aggregate "works on this Mac" verdict, treating the launchd name as a run, or treating the reserved mic as a qualified corner device. | Release cells stay not run. M1 and M5 stay blocked. |

## M2 external-input reservation (not an M0 qualification)

Tap tip `cd9a4c8` stores the name observed with `system_profiler SPAudioDataType`.
The default was `MacBook Pro Microphone`. The other was `syndai-iphone-02 Microphone`
(transport Unknown, not default, one input channel). The JSON record had no device
uid, so `EXTERNAL_INPUT` does not store one. `display_name` is
`Some("syndai-iphone-02 Microphone")`. `qualified_in_m0` stays false. At 00:27 KST
the Don't Allow / Allow dialog was not on screen and no click was observed.
M0-AC1 stays open.

| Field | Value |
| --- | --- |
| Display name | `syndai-iphone-02 Microphone` |
| Device UID | none stored (profiler JSON had no uid) |
| Device class | non-default input, transport Unknown |
| Default input on that host | `MacBook Pro Microphone` (not this reservation) |
| `qualified_in_m0` | false |
| Still open | M0-AC1 allow/deny/cleanup, and M2-AC1 qualification of this device |

## Acceptance criteria

- [ ] Every M0 decision has a concrete resolution before dependent milestone exit.
- [ ] Every remaining release-blocking row is resolved with evidence before M7 passes.
- [ ] Each change that affects a frozen contract names the invalidated evidence and
  completes the necessary requalification before the updated product is called done.
