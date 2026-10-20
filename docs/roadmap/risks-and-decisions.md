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
`cursor/m0-metal-link-b19b` tip `c0b2b22` (PR #12). The process-tap runs are
[process-tap.md](../evidence/m0/process-tap.md), copied from
`cursor/m0-process-tap-b19b` tip `d42010a` (PR #11). That tip includes the Terminal
HAL miss first recorded at `fcb3ae2`, the Chrome opens at `5958a71`, and the
`tccutil` explanation of why those opens skipped a dialog. Native probe and tap
sources stay on those branches. This branch does not relink ggml or CoreAudio.
The signed-install Stop is [signed-install.md](../evidence/m0/signed-install.md),
PR #38 tip `e7433c2`. This Mac has no Developer ID. `SIGNED_INSTALL` stays `None`.
`spctl` rejects ad-hoc. Weight distribution stays blocked.
The AC4 Partial freeze is [ac4-partial.md](../evidence/m0/ac4-partial.md).
M0-AC4 stays unchecked. Partial is not an accept.
Charter accept is blocked: M0-AC4 cannot be checked while SIGNED_INSTALL=None or C2_BLOCKED stands.
The launchd user-agent run does not close M0-AC4.
Cited gaps, not replayed onto this branch's corner strings: PR #38 tip `e7433c2`
(`SIGNED_INSTALL=None`), PR #42 tip `8794303` (those None guards), PR #27 tip
`ad8d418` (`C2_BLOCKED`), PR #43 tip `0c880fb` (adhoc launchd load/run/teardown),
PR #19 tip `8ebd0fc` (OS floor Partial at macOS 27.0.1).

None of these rows accept M0. M0-AC1, M0-AC2, M0-AC3, and M0-AC4 stay open. Milestone
checkboxes stay unchecked. M1 and M5 stay blocked.

Host for the cited Mac runs: Apple M1 Pro, 32 GiB (34359738368 bytes), macOS
27.0.1 (26A434), `aarch64-apple-darwin`, rustc 1.99.0. Not C1 (16 GiB, macOS 14.2).
Not a named C2.

| ID | Decision | Owner | Evidence on this host | Rejected alternative | Consequence |
| --- | --- | --- | --- | --- | --- |
| R1 | Stop. Chrome and system taps exited 0 without a dialog. A later Don't Allow/Allow dialog was not read and neither button was pressed. | macOS owner | Tip `d42010a`, [process-tap.md](../evidence/m0/process-tap.md). `TACO_CAPTURE_SPIKE=1` `--source system`, `app:com.google.Chrome`, and `pid:688` exited 0. `tccd` logged a microphone preflight only (`authValue=2`, `authReason=2`), attributed to responsible process Cursor (`com.todesktop.230313mzl4w4u92`, `/Applications/Cursor.app`) with accessor `ai.syndai.taco`. No `AUTHREQ_PROMPTING`. `tccutil list` said Microphone granted, AudioCapture denied, and SystemAudioRecording denied for that Cursor bundle. `ai.syndai.taco` had no listed services. Those words are `tccutil`'s and are not a mapping of `authValue=2`. A prior Microphone record for the responsible app is consistent with a preflight that does not prompt. Exit 0 is not an AudioCapture authorization: the Chrome slice had no AudioCapture request, and a denied AudioCapture record did not become a dialog or a CLI deny. Later, `tccutil reset Microphone com.apple.Terminal` printed success and the next list still said denied. `osascript` `tell application "Terminal" to do script` of `taco run --source default` did not start `taco` and did not return. A dialog with Don't Allow and Allow was on screen. Its title was not read, so it is not a microphone or AudioCapture prompt. Neither button was pressed. The `osascript` process was killed. No allow and no deny were taken. `T-M0-SPIKE-SOURCES` stays `verdict=not_run`. | Treat `tccutil` granted/denied, `authValue=2`, or exit 0 as an operator Allow or Deny. Record the unread dialog as either prompt. | Blocks M0-AC1, M1, and M5 until microphone and audio-capture prompts are shown, with a pressed allow, a pressed deny, and observed cleanup, in both terminal hosts. |
| R2 | Partial. Whisper small and Hy-MT2 loaded on MTL0 in the probe. Pinned Silero ONNX did not. | native runtime owner | Tip `c0b2b22`, [metal-link.md](../evidence/m0/metal-link.md). One static Mach-O. llama.cpp `2a66703` (ggml 0.27.0) and whisper.cpp `0c7be69` on that ggml; whisper's vendored ggml was not compiled. In-process SHA-256 matched the three pins. `ggml-small.bin` is type small, `use gpu = 1`, `MTL0 total size = 487.01 MB`, `whisper_full` returned 0. `Hy-MT2-1.8B-Q4_K_M.gguf` is `hunyuan-dense`, `n_gpu_layers=-1`, `offloaded 33/33 layers to GPU`, `MTL0_Mapped` 1075.74 MiB plus `CPU_Mapped` 193.57 MiB. Publisher PR 22836 stays an unmerged CPU STQ kernel and was not required for this load. `silero_vad.onnx` failed `invalid model data (bad magic)`. Probe stdout: `link_three=0` `ac2=0`. The probe is not the `taco` binary. `T-M0-LINK-THREE-MODELS` still panics. | Subprocess `whisper-cli` / `llama-completion`, `-ngl 0`, a helper process, a sidecar dylib, a ggml VAD substituted for the pinned ONNX, another family's GGUF, or the 1.25-bit and 2-bit repos. | Blocks M0-AC2 until all three pinned files load in `taco` on Metal. M1 and M5 stay blocked. |
| R3 | Stop. No Silero runtime is chosen. The pinned ONNX is rejected by the whisper VAD loader. | audio owner | Tip `c0b2b22`, [metal-link.md](../evidence/m0/metal-link.md). Digest matched `1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`. `whisper_vad_init_with_params` reported `invalid model data (bad magic)`. That loader expects a ggml VAD file. No other VAD file was substituted. No incremental Silero runtime was linked: frame shape, state reset, forced 15-second split, and zero owned overlap were not measured. Those bounds stay a contract, not a model result. | Batch `whisper_full` VAD, a hand-written model, a user-installed ONNX dylib, or swapping a ggml VAD in under the Silero pin. | Blocks M0 acceptance and M2's real VAD. M1 stays blocked with the rest of M0. |
| R4 | Stop. Not run. No quantization, instruction winner, or sampling arm is selected. | inference owner | `EXPERIMENT.ran` is false and every selected field is empty. No development-corpus comparison ran on this host. Baseline instruction remains the publisher English template; that is the ADR starting point, not a comparison result. Publisher temperature 0.7 is an arm, not the active sampler (`SELECTED_SAMPLING` is none). The greedy 8-token smoke in metal-link.md (`c0b2b22`, 34 bytes) is not an R4 selection. | Pick a quant or temperature without the development-corpus comparison. Do not treat the probe's greedy smoke as the winner. | Blocks M0 acceptance and M1's releasable bundle. |
| R5 | Partial. Llama init and decode joined inside 2 s. Whisper was not cancelled. | runtime owner | Tip `c0b2b22`, [metal-link.md](../evidence/m0/metal-link.md). Init abort at progress 0 made `llama_model_load_from_file` return null; join 222 ms. After a kept Metal context existed, `llama_decode` returned 2 and the worker joined in 9 ms. Stderr: `ggml_backend_sched_graph_compute_async failed with error 1`. Reserve is 2000 ms. Whisper had no cancel. Probe stdout: `cancel_lifecycle=1` `ac3=0`. That flag is only the llama pair. `t_m0_cancel_lifecycle` still panics. Shutdown arithmetic stays 8 s drain + 2 s join. The probe is not `taco`. | `pthread_cancel`, a detached worker, or a wrapper timeout. | Blocks M0-AC3, M1, and M5 until both engines cancel inside the reserve in `taco`. |
| R6 | Partial. Corners are named and unqualified. `C2_BLOCKED` is true. | release owner | This worker is M1 Pro, 32 GiB, macOS 27.0.1 (26A434). C1 stays "M1 class, 16 GiB, macOS 14.2" on this branch. C2 stays "a different Apple Silicon machine" with no OS build. Neither `qualified` flag is set. PR #27 tip `ad8d418` records the second-machine gap. This branch does not replay that C1 rename. | Call this Mac C1, call "macOS 27.0.1" the unnamed C2 build, or invent a second machine. | Dual-machine acceptance cannot close. M0-AC4 stays unchecked. Blocks the support claim, M1, and M5. |
| R7 | Closed table frozen: `en→zh` and `zh→en`. Experimental pairs empty. Quality is not qualified. | product/quality owner | `language::PAIRS` and the reject tests. No corpus was scored on this host. | Auto-detect, identical tags, or the publisher language inventory. | The table is binding. Adequacy gates stay blocked. M1 still waits on M0 acceptance. |
| R8 | Proposed numbers are frozen in `fixtures/m0/limit-vectors.txt` and were not revised. Measurement was not run. | audio/runtime/quality owners | No latency, memory, or slope measurement on this host. `T-M0-LIMIT-VECTORS` still fails if the fixture drifts from acceptance.md. | Redefine `t_audio_end`, drop failures, or widen the slope past 1 MiB/min. | R8 stays open for a later measured revision. Quality gates stay blocked. |
| R9 | Partial. A failed app selection does not become system capture. Helper inclusion and isolation are unproven. The external input is named and not qualified. | macOS owner | Tip `d42010a`, [process-tap.md](../evidence/m0/process-tap.md). `SelectionFailurePolicy::StopDoNotBroaden` held. Terminal was running (pid 42771) and was not a HAL client: `find_bundle` `osstatus=0` `count=29` `matches=0`; `find_pid` `object=0` `in_list=0`. Both selectors returned `capture_not_available`. `AudioHardwareCreateProcessTap` was not called for that miss. Chrome (`com.google.Chrome`, object=103, pid=688, `IsRunning=0`) was already on the list; `app:` and `pid:` exited 0. A one-shot list showed `com.google.Chrome.helper` as a different bundle. No helper was selected. No other-audio isolation fixture ran. `system_profiler SPAudioDataType` listed default input `MacBook Pro Microphone` and non-default `syndai-iphone-02 Microphone` (transport Unknown). `EXTERNAL_INPUT.display_name` is that second name. `device_uid` stays unset. `qualified_in_m0` is false. | Treat the Terminal miss as a TCC deny, or broaden it to system capture. Treat the Chrome open as proof that helpers are included. Treat the iPhone mic name as an M0 qualification or as the default input. Drop the selected-app category. | Blocks M0-AC1 until allow and deny are taken, M5's selector proof, and M2 qualification of the reserved input. |
| R10 | Design frozen: the session takes verified local handles and has no fetch client. Inode proof is M1. | model owner | Digests are in [pin-measurement.md](../evidence/m0/pin-measurement.md) (`5e9649c`). `PINS_MEASURED` and `release_ready()` are true. `fetch_models` still returns `fetch_refused_unpinned`. Inode or handle proof of the load boundary was not run. The earlier `models_unpinned` refusal predates those pins. | A session URL or an HTTP client parameter. | M1 must prove the load boundary. M1 does not start before M0 is accepted. |
| R11 | Stop. No Developer ID Application identity and no notary credential on this host. The SPDX inventory remains. `SIGNED_INSTALL` stays `None`. | release owner | [license-inventory.md](../evidence/m0/license-inventory.md). `silero-vad-weights` is MIT, notice `Copyright (c) 2020-present Silero Team`, from snakers4/silero-vad `1e261b036686cd0017d500ee96acd1c4ba572a9d` LICENSE. That tree's `silero_vad.onnx` sha256 matches the pin. `hy-mt2-1.8b-gguf` is Apache-2.0, notice `Copyright (C) 2026 Tencent. All rights reserved.`, from Hugging Face `tencent/Hy-MT2-1.8B-GGUF` `LICENSE.txt` revision `a0c709d9fac510f2c807aa3af52872340dc37a4a`. The terms from `Version 2.0, January 2004` match Apache-2.0. `whisper.cpp` `0c7be696371069e0baa2515ed7aa08d5a5473aa6` and `llama.cpp` `2a667032dc788c3a071d538e0b572e45a3f12731` are each MIT, notice `Copyright (c) 2023-2026 The ggml authors`, from that commit's root LICENSE. `whisper-small-weights` is MIT, notice `Copyright (c) 2022 OpenAI`, from openai/whisper `86098128c0b4f24f0e2aa2994de830614b474227` LICENSE. That commit's README says the code and model weights are under that license. The pin matches Hugging Face `ggerganov/whisper.cpp` `5359861c739e955e79d9a303bcbc70fb988958b1` `ggml-small.bin` LFS oid and size. That tree has no LICENSE file. whisper.cpp `0c7be69` downloads `small` from the pin URL and describes the file as a `convert-pt-to-ggml.py` conversion of the OpenAI checkpoint. The conversion was not re-run. The card tag `license: mit` was not copied. Engine notices were not copied onto model rows. Vendor and example LICENSE files stay out of these five records. Tip `e7433c2` (PR #38), 2026-10-17 on this host, [signed-install.md](../evidence/m0/signed-install.md). `security find-identity -v -p codesigning` listed only `Apple Development: ops@syndai.com (CT93T5LALG)`. `codesign --force --options runtime --timestamp --sign "Developer ID Application" --identifier ai.syndai.taco` printed `Developer ID Application: no identity found` and exited 1. `xcrun notarytool submit` with `--keychain-profile taco-notary` printed `No Keychain password item found for profile: taco-notary` and exited 69. Ad-hoc `codesign --force -s - --identifier ai.syndai.taco` bound `Info.plist entries=5` with `TeamIdentifier=not set`. Release `CDHash=434ca964d84bac7307406cd6532044347fa2792b`. Debug `CDHash=82d87753911f3d623f726944bffdfe45c3c3a355`. Each designated requirement is that cdhash alone. `spctl --assess -vv --type execute` printed `rejected` and exited 3. An Apple Development re-sign did not return and was stopped; that copy stayed linker-signed. No binary was installed. `scripts/sign-debug.sh` remains ad-hoc `codesign -s -`. The Apple Development re-sign in [macos-spike-results.md](../evidence/m0/macos-spike-results.md) (`CDHash=f5455d873156271820035ef4fe0ac7bbe2e8be07`, Info.plist not bound) is not that artifact. `SIGNED_INSTALL` is `None`. | Invent license text, treat an engine license as the model license, or treat ad-hoc codesign, a stopped Apple Development re-sign, or the spike re-sign as a notarized install. | Distribution of weights remains blocked. Gatekeeper rejects the Mach-O. The ad-hoc designated requirement is a per-build cdhash, so a fresh `taco run` has no stable install identity. Notarization cannot start. Charter invariant 4 and ADR 0005 keep weights outside the binary; the user fetches them only when a manifest inside a signed executable is the trust root. That artifact does not exist. |
| R12 | Stop. Not run. | inference/quality owners | No serialized-vs-concurrent measurement. The MTL0 load in metal-link.md (`c0b2b22`) is not a timing run. | A microbenchmark, a CPU timing, or that load presented as the joint gate. | Blocks M0 acceptance and M4's baseline. |
| R14 | Partial. The alternate host is a launchd user agent. The measured run is ad-hoc and does not close M0-AC4. | release/quality owners | PR #19 tip `8ebd0fc` names the launchd user agent and leaves it unexercised. The OS floor there is Partial at macOS 27.0.1 (26A434), not a support promise. PR #43 tip `0c880fb` loaded, ran, and removed `ai.syndai.taco.m0-launch-host`. `codesign` reported `Signature=adhoc` and `TeamIdentifier=not set`. That binary is not a signed install. C2 stays blocked. The external-input slot is named and not qualified. | Treat the launchd run as a signed install, as a TCC allow or deny, or as an M0-AC4 pass. | The launchd user-agent run does not close M0-AC4. Release cells stay not run. M1 and M5 stay blocked. |

## M2 external-input reservation (not an M0 qualification)

Tap tip `d42010a` observed two inputs with `system_profiler SPAudioDataType`.
The default was `MacBook Pro Microphone`. The other was `syndai-iphone-02 Microphone`
(transport Unknown, not default, one input channel). No device UID was listed.
`selector::EXTERNAL_INPUT.display_name` is that observed name. `qualified_in_m0`
stays false. M0-AC1 stays open: the name does not supply an allow, a deny, or cleanup.

| Field | Value |
| --- | --- |
| Display name | `syndai-iphone-02 Microphone` |
| Device UID | TBD (unset; no UID in the profiler listing) |
| Device class | non-default input, transport Unknown |
| Default input on that host | `MacBook Pro Microphone` (not this reservation) |
| `qualified_in_m0` | false |
| Still open | M0-AC1 allow/deny/cleanup, and M2-AC1 qualification of this device |

## Acceptance criteria

- [ ] Every M0 decision has a concrete resolution before dependent milestone exit.
- [ ] Every remaining release-blocking row is resolved with evidence before M7 passes.
- [ ] Each change that affects a frozen contract names the invalidated evidence and
  completes the necessary requalification before the updated product is called done.
