# M0 — Feasibility and executable contracts

Status: **Not accepted.** Shared `taco` types and default-tier tests are in the tree. Hardware spikes are recorded blockers in the decision register. Cited tips: process tap `cd9a4c8` (PR #11), metal probe `a8b2ebc` (PR #12, `link_three=1`, Silero on CPU ORT, probe is not `taco`), license inventory `c04ab04` (PR #15), adapters `05c0c73` (PR #14), engine bridge `570364c` (PR #16), R4 sampling Stop `1a52b8d` (PR #17), Whisper small SPDX `7ee17c8` (PR #18, R11 Partial), OS floor Partial macOS 27.0.1 and unexercised launchd host `8ebd0fc` (PR #19, C2 open), ORT Mach-O guards `f2b1720` (PR #21, shared dylib is not a ship path, LINK-THREE and CANCEL still panic), mic hold `a1654eb` (PR #22, `TACO_CAPTURE_SPIKE_HOLD_MS`, no Allow or Deny), product `taco` static ORT `7f98ab1` (PR #23, `otool -L` system-only, init/infer/cancel/close/prove kept, `T-M0-LINK-THREE-MODELS` and `T-M0-CANCEL-LIFECYCLE` passed via `session::prove_native` with `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`, Silero ORT CPU, Whisper VAD bad magic, ASR/MT Metal, joins under 2 s, AC2 and AC3 open), hold source assert `89759b9` (PR #24, default `1_800_000`, no TCC faked), `taco run` prove `e2ec751` (PR #30, `TACO_ENGINES_PROVE=1` on the `af2af02` gate, ad-hoc Mac run printed `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`, CPU Silero, not a Developer ID signature), mic open timeout `c7e7c2e` (PR #31, `TACO_CAPTURE_SPIKE_OPEN_MS` default `180000`, no Allow or Deny), engines residency `0972b2d` (PR #32, `phys_footprint` 218792200 bytes over the 32 MiB band, `ep=CPU`, residency test not `taco run`), AC2/AC3 honesty `7ae6d98` (PR #35, prove reachable from `taco run` is closed, residency Stop `a0e1614`, CPU VAD open), mic open default `81a2a54` (PR #36, `TACO_CAPTURE_SPIKE_OPEN_MS` default `300000`, no Allow or Deny), Silero CoreML `d194012` (PR #37, VAD CPU Stop, `CPUExecutionProvider`, `vad_ort_plan=missing`), signed install `e7433c2` (PR #38, R11 Stop, Developer ID missing, notary profile `taco-notary` missing, `spctl` rejected ad-hoc), stale OPEN_MS claim dropped `8c56c92` (PR #40, charter rejects regression to `180_000`, unset default stays `300000`, hold stays `1800000`, no Allow or Deny, no new Mac evidence), AC2/AC3 Stop consequence `a8e2eca` (PR #41, boxes stay unchecked while CPU VAD `d194012` and residency `a0e1614` stand), R11 `SIGNED_INSTALL=None` guard `8794303` (PR #42, no new Mac measurement), launchd host exercised `0c880fb` (PR #43, ad-hoc debug `taco notices`, not TCC), AudioCapture path `d78e7f8` (PR #44, no tap started, `AudioDeviceStart` gap, no Allow or Deny), Taco.app bundle `7fb5006` (PR #46, ad-hoc, not launched, AC1 open), residency retry Stop `37ff031` (PR #49, cycle 2 still 94699688 bytes over cycle 1 after `vm_deallocate`, synchronize and blit not in the tree, `a0e1614` stands, `ep=CPU`), AudioCapture start `ce5690f` (PR #47, newer than code `f9b08be`, `AudioDeviceStart` with open default `300000`, tap not started, no Allow or Deny), residency holder Stop `0829a70` (PR #55, process-lifetime `ggml_backend_metal_reg`, no new Mac run, no purge, AC2 and AC3 open), tips9 cite `a5619d8` (PR #56, cites #52/#53/#54, boxes unchecked), Silero CoreML subgraph Stop `88eda06` (PR #57, `EnableOnSubgraphs=1` on that tip, ORT 1.31 `GetOpBuilders` lacks `LSTM` and `If`, Linux `device=cpu`, no `device=metal`), tips10 cite `fb4246e` (PR #58, cites #55/#56/#57, boxes unchecked), AC2/AC3 consequence2 `ed9ee10` (PR #59, flip guard also locks #55 `0829a70` and #57 `88eda06`, prior #37 `d194012` / #32 `a0e1614` / #49 `37ff031` / #41 `a8e2eca` stay, boxes unchecked, charter guard not replayed), Silero pin Stay `aab661c` (PR #60, pin `1a153a22…` stays, no published ONNX is CoreML-takeable under ORT 1.31, `device=` stays `cpu`, boxes unchecked, charter guard not replayed), tips11 cite `f5d794a` (PR #61, cites #58 `fb4246e` and #59 `ed9ee10`, boxes unchecked), AC1/AC4/OPEN_MS flip guards `8c24c99` (PR #62, locks #51 `7c71d50` Partial and #54 `6a839de` Partial, OPEN_MS `300000` and HOLD `1_800_000`, AC5 and AC6 stay unchecked, charter guards not replayed), Metal device teardown Stop `232ea78` (PR #63, no safe `ggml_metal_device_free`, `g_devices` and `ggml_backend_metal_reg` live until process exit, use-after-free, boxes unchecked, no new flip guard), ORT CoreML builder Stop `c9a6f6f` (PR #64, `GetOpBuilders` blob `891704e8` is the same on v1.31.0 and main `a7a1172c`, no `LSTM` or `If`, upgrade out of M0, pin unchanged, `ort_coreml_upgrade_cannot_flip_ac2` not replayed), AC2/AC3 consequence3 `04dda65` (PR #65, flip guard also locks #60 `aab661c` and #63 `232ea78`, cites #64 `c9a6f6f` without copying that guard, #55 `0829a70` unchanged, boxes unchecked, charter guard not replayed), tips12 cite `4011569` (PR #66, cites #60 through #65, boxes unchecked), AudioCapture measure harness `652614b` (PR #67, Unmeasured, Allow and Deny tip SHAs none, TIMEOUT is not Deny, charter guard not replayed), mic Allow-then-Deny resume `a1c5c32` (PR #68, TIMEOUT is not Deny, no SecurityAgent on Linux, charter guard not replayed), Silero-only Stop `2ff5c4f` (PR #69, charter forbids an alternate VAD, `alternate_vad_cannot_flip_ac2` not replayed, `device=` stays `cpu`, boxes unchecked), Metal device refcount Stop `e06ea0e` (PR #70, refcount is still a use-after-free, `metal_refcount_stop_cites_63_and_55` not replayed, boxes unchecked), tips13 cite `b94e2bb` (PR #71, cites #67 through #70, boxes unchecked), AC2/AC3 consequence4 `f16f0bb` (PR #72, flip guard locks #69 `2ff5c4f` and #70 `e06ea0e` into `ac2_ac3_cannot_flip_while_stop_shas_stand`, #55 `0829a70` and #63 `232ea78` stay, cites #64 `c9a6f6f` without copying that guard, boxes unchecked, charter guard not replayed), tips14 cite `789c026` (PR #73, cites #71 `b94e2bb` and #72 `f16f0bb`, boxes unchecked), Metal subprocess Stop `30fd669` (PR #74, helper process violates single Mach-O, in-process Metal, and LINK-THREE, `metal_subprocess_stop_cites_55_63_and_70` not replayed, boxes unchecked), product process-tap Partial `dc1b9be` (PR #75, `system`/`app:`/`pid:` already on `taco run`, Linux tap success is Stop, `process_tap_product_path_refuses_linux_success` not replayed, Allow and Deny tip SHAs none, M0-AC1 unchecked), AC2/AC3 consequence5 `2d9679f` (PR #76, flip guard locks #74 `30fd669` into `ac2_ac3_cannot_flip_while_stop_shas_stand`, does not copy `metal_subprocess_stop_cites_55_63_and_70`, `0829a70` `232ea78` `e06ea0e` `2ff5c4f` stay, boxes unchecked, charter guard not replayed), AC1 full-session Partial `595dced` (PR #77, mic then AudioCapture, Mac offline, Allow and Deny tip SHAs none, `ac1_full_session_chains_pack_resume_then_audiocapture` not replayed, M0-AC1 unchecked), tips15 cite `9a1d8f1` (PR #78, cites #73 through #77, boxes unchecked), charter gap guards `d04c224` (PR #79, corpora unsealed, pin digests stay in the pin note, `taco models verify` fails closed, `pins_are_not_a_verify_pass_and_corpora_stay_unsealed` already on this tip, boxes unchecked), external-input lock `9def38b` (PR #80, `qualified_in_m0` stays false, `external_input_unqualified_refuses_ac1` not replayed, M0-AC1 unchecked), AC1 full-session flip `ca9bc50` (PR #81, locks M0-AC1 while #77 mic and AudioCapture tip SHAs stay none, `ac1_cannot_flip_while_full_session_tip_shas_are_none` not replayed, boxes unchecked), process-tap AC1 flip `637459f` (PR #82, locks M0-AC1 while process-tap Allow and Deny tip SHAs stay none, a Cleanup line when present, `process_tap_tips_refuse_ac1_flip_while_either_sha_is_none` not replayed, boxes unchecked), tips16 cite `86dd55d` (PR #83, cites #78 through #82, base #79 `d04c224`, boxes unchecked), process-tap measure `fb48acc` (PR #84, `scripts/m0-process-tap-measure.sh`, full session `after-pair` after AudioCapture only, Allow Deny and Cleanup tip SHAs none, exit 0 and Drop are not a Cleanup tip SHA, `process_tap_measure_keeps_tip_shas_none` not replayed, M0-AC1 unchecked), AC2/AC3 Partial freeze `576ac63` (PR #85, Status Partial, `m0_ac2_ac3_partial_refuses_checked_boxes` complements #76 and is not replayed, ORT CoreML LSTM/If and Metal device process-lifetime release stay out of M0, boxes unchecked), tips17 cite `8938f5b` (PR #86, cites #83 through #85, base #83 `86dd55d`, boxes unchecked), AC1 guards on the measure tip `a40fad1` (PR #87, joins `ac1_cannot_flip_while_full_session_tip_shas_are_none` and `process_tap_tips_refuse_ac1_flip_while_either_sha_is_none` onto #84, Cleanup tip SHA none refuses AC1, Allow Deny and Cleanup tip SHAs none, guards not replayed, M0-AC1 unchecked), external-input lock on the measure tip `2766f79` (PR #88, `qualified_in_m0` stays false, `external_input_unqualified_refuses_ac1` not replayed, no iPhone capture, M0-AC1 unchecked), tips18 cite `831d455` (PR #89, cites #86 through #88, base #86 `8938f5b`, boxes unchecked), AC1 Partial freeze `7e8f3c5` (PR #90, Status Partial, `m0_ac1_partial_refuses_checked_boxes` not replayed, Allow Deny and Cleanup tip SHAs none, TIMEOUT is not Deny, Mac still required, M0-AC1 unchecked), AC2/AC3 Partial on the measure tip `79cb73d` (PR #91, Status Partial, stacked on #88 `2766f79`, #85 `576ac63` is not an ancestor, `m0_ac2_ac3_partial_refuses_checked_boxes` not replayed, ORT CoreML LSTM/If and Metal device process-lifetime release stay out of M0, boxes unchecked), tips19 cite `6b165eb` (PR #92, cites #89 through #91, base #89 `831d455`, boxes unchecked), Cleanup destroy-trace `359375e` (PR #93, stacked on #88 `2766f79`, `m0_cleanup_hex_without_destroy_refuses_ac1` not replayed, Cleanup tip SHA none, `destroy_traced` stays no, M0-AC1 unchecked), AC1 and AC2/AC3 Partial join plus #93 destroy-trace `fe08f47` (PR #94, joins #90 `7e8f3c5` and #91 `79cb73d`, carries `m0_cleanup_hex_without_destroy_refuses_ac1`, guards not replayed, Allow Deny and Cleanup tip SHAs none, `destroy_traced` stays no, boxes unchecked). M0-AC2 and M0-AC3 stay open. Acceptance checkboxes stay open. Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. Depends on: charter only.
Accountable: technical lead; reviewers: capture, inference, and release owners.
Decisions: [ADR 0001](../adr/0001-process-and-pipeline.md),
[0002](../adr/0002-audio-capture.md), [0004](../adr/0004-local-inference.md),
[0007](../adr/0007-qualification.md).

## Goal

Establish that the exact core stack can produce bounded, offline bilingual output
inside the intended single-file macOS distribution. Resolve the expensive unknowns
before production pipeline and terminal work. Freeze contracts and an honest
acceptance plan so later milestones cannot move their own goalposts.

## Non-goals

No polished TUI, full downloader, production capture adapter, broad language claim,
long benchmark campaign, or declaration that v0.1 is already usable. Do not replace
a charter source category or model family with a quieter substitute. A successful
spike is feasibility evidence, not production completion. Contract types and
default-tier tests are in scope; they are not a second product.

## Work packages

1. **Distribution/capture spike.** Build a minimal arm64 command-line artifact using
   microphone input and process taps separately. Exercise the permission declaration,
   signing identity, terminal-host attribution, system output, selected application,
   and cleanup on the proposed minimum OS. Inspect dynamic dependencies and required
   Metal resources. Confirm the runtime needs no sidecar code, Python, Homebrew, or
   custom driver. Archive the exact artifact identity and observations.
2. **Combined inference spike.** Load VAD, multilingual Whisper small, and a main-repo
   Hy-MT2 1.8B GGUF in the same process. Recheck the publisher's llama.cpp kernel
   requirement and pin that commit. Exercise Metal with GPU layers loaded, not the
   card's CPU example, and exercise native cancellation. Pin linkage, embedded shader
   resources, bindings, toolchain, and model digests. Test incremental VAD state, not
   batch `whisper_full` segmentation and not upstream CLI defaults.
3. **Scheduling/quality experiment.** Seal the held-out split and the critical set
   before any comparison. On the development corpus only, compare serialized and
   concurrent inference, at most two quantizations, the publisher's two default
   instruction languages, and the publisher sampling arm against one seeded arm.
   Measure latency, memory, raw translation quality, and cancellation together.
   Pin one schedule, one quantization, one instruction, and one sampling arm.
   Name both qualification corners. Do not optimize a microbenchmark at the cost
   of missing subtitles.
4. **Contract freeze.** Land shared `taco` types for selectors, events, outcomes,
   errors, and stage traits, plus the limit-vector fixture. Finalize command grammar,
   the CLI record format, model manifest and license records, both qualification
   corners, the closed language table, and the development, held-out, and critical
   corpora. Markdown alone is not the freeze.

Timebox the distribution spike and the link spike to one reproducible candidate plus
one targeted revision of that candidate. The quality experiment is one pre-registered
comparison, not an open search: serialized versus one concurrent schedule, at most
two quantizations from the main GGUF repository, the publisher's two default
instruction languages, and the publisher sampling arm versus one seeded arm. If a
timeboxed spike fails twice, record the blocker and stop for a decision instead of
adding engines, servers, or source categories. Timeboxing does not waive the gate.

## Tests to write first

Write the M0 IDs in the [test catalog](test-catalog.md) and show the default-tier
tests failing before the contract types satisfy them. Do not rename the IDs.

- `T-M0-REJECT-INPUTS`
- `T-M0-LIMIT-VECTORS`
- `T-M0-LINK-THREE-MODELS`
- `T-M0-CANCEL-LIFECYCLE`
- `T-M0-ONE-MACHO`
- `T-M0-SPIKE-SOURCES`

## Acceptance criteria

- [ ] **M0-AC1:** A report proves microphone, system, and app tap feasibility from the
  intended CLI artifact, including allow/deny and cleanup. The external input category
  has a named device reserved for M2 qualification. Proposed minimum OS is confirmed
  or revised explicitly before any support promise is accepted.
  [mic-operator-timeouts.md](../evidence/m0/mic-operator-timeouts.md) is Partial.
  Repeated `OPEN_MS TIMEOUT` (pids 65422, 71152, 76134, 82331, 89220, and ongoing
  Taco.app re-prompts) left `tccutil` denied. TIMEOUT is not Don't Allow.
  Partial — operator Allow/Deny unproven; consequence AC1 open.
  PR #62 tip `8c24c99` locks this Partial. Allow tip SHA: none. Deny tip SHA: none. This branch does not replay that charter guard.
  PR #47 tip `ce5690f` did not start the tap. No AudioCapture Allow or Deny.
  PR #67 tip `652614b` is the AudioCapture measure harness. Status Unmeasured. Allow tip SHA: none. Deny tip SHA: none. TIMEOUT is not Deny. This branch does not replay `audiocapture_allow_and_deny_required_before_ac1`.
  PR #68 tip `a1c5c32` is the one-shot mic resume. TIMEOUT is not Deny. Allow tip SHA: none. Deny tip SHA: none. This branch does not replay `ac1_mac_resume_locks_timeout_and_both_outcomes`.
  PR #71 tip `b94e2bb` cites #67 and #68. Allow tip SHA: none. Deny tip SHA: none. This branch does not replay those charter guards.
  PR #73 tip `789c026` cites #71. Allow tip SHA: none. Deny tip SHA: none. This branch does not replay those charter guards.
  PR #75 tip `dc1b9be` is the product process-tap Partial. `system`, `app:`, and `pid:` already dispatch through `session::start_capture` (PR #11 tip `cd9a4c8`, ancestor of `a1c5c32`). CoreAudio and operator Allow/Deny stay Mac-only. Linux tap success is a Stop. Allow tip SHA: none. Deny tip SHA: none. This branch does not replay `process_tap_product_path_refuses_linux_success`.
  Partial — process taps are on taco run; Mac CoreAudio and operator Allow/Deny stay open; Linux tap success is Stop.
  PR #77 tip `595dced` chains mic resume then AudioCapture. Mac was offline. Allow tip SHA: none. Deny tip SHA: none. TIMEOUT is not Deny. This branch does not replay `ac1_full_session_chains_pack_resume_then_audiocapture`.
  PR #78 tip `9a1d8f1` cites #73 through #77. Allow tip SHA: none. Deny tip SHA: none.
  PR #80 tip `9def38b` refuses a checked M0-AC1 while `qualified_in_m0` stays false. No iPhone capture. No Allow. No Deny. This branch does not replay `external_input_unqualified_refuses_ac1`.
  PR #81 tip `ca9bc50` locks this Partial while mic and AudioCapture Allow and Deny tip SHAs stay `none`. TIMEOUT is not Deny. This branch does not replay `ac1_cannot_flip_while_full_session_tip_shas_are_none`.
  PR #82 tip `637459f` locks this Partial while process-tap Allow and Deny tip SHAs stay `none`. TIMEOUT is not Deny. Linux tap success stays Stop. This branch does not replay `process_tap_tips_refuse_ac1_flip_while_either_sha_is_none`.
  PR #83 tip `86dd55d` cites #78 through #82. Allow tip SHA: none. Deny tip SHA: none.
  PR #84 tip `fb48acc` is the process-tap measure. Allow tip SHA: none. Deny tip SHA: none. Cleanup tip SHA: none. The full session runs `after-pair` after AudioCapture only. Exit 0 is not a Cleanup tip SHA. Drop is not a Cleanup tip SHA. This branch does not replay `process_tap_measure_keeps_tip_shas_none`.
  PR #85 tip `576ac63` does not check M0-AC1.
  PR #86 tip `8938f5b` cites #83 through #85. Allow tip SHA: none. Deny tip SHA: none. Cleanup tip SHA: none.
  PR #87 tip `a40fad1` joins the #81 and #82 flip guards onto the #84 measure tip. Cleanup tip SHA `none` refuses M0-AC1. Allow tip SHA: none. Deny tip SHA: none. Cleanup tip SHA: none. This branch does not replay `ac1_cannot_flip_while_full_session_tip_shas_are_none` or `process_tap_tips_refuse_ac1_flip_while_either_sha_is_none`.
  PR #88 tip `2766f79` refuses a checked M0-AC1 while `qualified_in_m0` stays false. No iPhone capture. This branch does not replay `external_input_unqualified_refuses_ac1`.
  PR #89 tip `831d455` cites #86 through #88. Allow tip SHA: none. Deny tip SHA: none. Cleanup tip SHA: none.
  PR #90 tip `7e8f3c5` records Status Partial in `docs/evidence/m0/ac1-partial.md`. Allow tip SHA: none. Deny tip SHA: none. Cleanup tip SHA: none. TIMEOUT is not Deny. A Mac operator session is still required. Partial is not an accept. This branch does not replay `m0_ac1_partial_refuses_checked_boxes` and does not copy that note.
  PR #91 tip `79cb73d` does not check M0-AC1.
  PR #92 tip `6b165eb` cites #89 through #91. Allow tip SHA: none. Deny tip SHA: none. Cleanup tip SHA: none.
  PR #93 tip `359375e` refuses a checked M0-AC1 when a hex Cleanup tip SHA has no `destroy_traced: yes`. Cleanup tip SHA stays `none`. This branch does not replay `m0_cleanup_hex_without_destroy_refuses_ac1`.
  PR #94 tip `fe08f47` keeps Allow, Deny, and Cleanup tip SHAs `none` and refuses a hex Cleanup tip SHA without `destroy_traced: yes`. Status: Partial. TIMEOUT is not Deny. A Mac operator session is still required. Partial is not an accept. This branch does not replay `m0_ac1_partial_refuses_checked_boxes` or `m0_cleanup_hex_without_destroy_refuses_ac1`.
- [ ] **M0-AC2:** The combined native spike performs ASR and translation with the exact
  pinned artifacts, in process, with observed Metal GPU execution. A subprocess CLI
  or a CPU-only `-ngl 0` run does not pass.
  PR #49 tip `37ff031` is a stronger residency Stop and does not drop `a0e1614`. Silero stayed `ep=CPU`.
  PR #55 tip `0829a70` names the holder as `ggml_backend_metal_reg` and records no new Mac run. PR #57 tip `88eda06` keeps `device=cpu`. No `device=metal`.
  PR #59 tip `ed9ee10` locks `0829a70` and `88eda06`. `device=` stays `cpu`. This branch does not replay that charter guard.
  PR #60 tip `aab661c` keeps the pin. No published Silero ONNX is CoreML-takeable under ORT 1.31. `device=` stays `cpu`.
  PR #63 tip `232ea78` records no safe release of that Metal device. `llama_backend_free` does not destroy it.
  PR #64 tip `c9a6f6f`: ORT `main` `GetOpBuilders` is still the v1.31.0 blob. An upgrade is out of M0. `device=` stays `cpu`.
  PR #65 tip `04dda65` locks `aab661c` and `232ea78` and cites `c9a6f6f`. This branch does not replay that charter guard.
  PR #69 tip `2ff5c4f` keeps the VAD family Silero. An alternate ONNX is not a pin. `device=` stays `cpu`. This branch does not replay `alternate_vad_cannot_flip_ac2`.
  PR #70 tip `e06ea0e` records that a Metal device refcount is still a use-after-free. This branch does not replay `metal_refcount_stop_cites_63_and_55`.
  PR #71 tip `b94e2bb` cites #69 and #70. `device=` stays `cpu`.
  PR #72 tip `f16f0bb` locks `2ff5c4f` and `e06ea0e` into `ac2_ac3_cannot_flip_while_stop_shas_stand`. `0829a70` and `232ea78` stay. This branch does not replay that charter guard.
  PR #73 tip `789c026` cites #71 and #72. `device=` stays `cpu`.
  PR #74 tip `30fd669` records that a helper process leaves single Mach-O, in-process Metal, and LINK-THREE. This branch does not replay `metal_subprocess_stop_cites_55_63_and_70`.
  PR #76 tip `2d9679f` locks `30fd669` into `ac2_ac3_cannot_flip_while_stop_shas_stand`. `0829a70`, `232ea78`, `e06ea0e`, and `2ff5c4f` stay. This branch does not replay that charter guard and does not copy `metal_subprocess_stop_cites_55_63_and_70`.
  PR #78 tip `9a1d8f1` cites #73 through #77. `device=` stays `cpu`.
  PR #79 tip `d04c224` does not print `device=metal`. Not a Metal pass.
  PR #81 tip `ca9bc50` does not change the AC2/AC3 Stop-SHA list.
  PR #82 tip `637459f` does not change the AC2/AC3 Stop-SHA list.
  PR #83 tip `86dd55d` cites #78 through #82. `device=` stays `cpu`.
  PR #84 tip `fb48acc` does not change the AC2/AC3 Stop-SHA list.
  PR #85 tip `576ac63` records Status Partial in `docs/evidence/m0/ac2-ac3-partial.md`. It complements #76 and does not replace `ac2_ac3_cannot_flip_while_stop_shas_stand`. ORT CoreML `LSTM`/`If` and a Metal device process-lifetime release stay out of M0. This branch does not replay `m0_ac2_ac3_partial_refuses_checked_boxes` and does not copy that note.
  PR #86 tip `8938f5b` cites #83 through #85. `device=` stays `cpu`.
  PR #87 tip `a40fad1` does not change the AC2/AC3 Stop-SHA list.
  PR #88 tip `2766f79` does not change the AC2/AC3 Stop-SHA list.
  PR #89 tip `831d455` cites #86 through #88. `device=` stays `cpu`.
  PR #90 tip `7e8f3c5` does not change the AC2/AC3 Stop-SHA list.
  PR #91 tip `79cb73d` records Status Partial for M0-AC2 on the measure tip. #85 tip `576ac63` is not an ancestor of #88 tip `2766f79`. It cites #76 and does not replace `ac2_ac3_cannot_flip_while_stop_shas_stand`. ORT CoreML `LSTM`/`If` and a Metal device process-lifetime release stay out of M0. This branch does not replay `m0_ac2_ac3_partial_refuses_checked_boxes` and does not copy that note.
  PR #92 tip `6b165eb` cites #89 through #91. `device=` stays `cpu`.
  PR #93 tip `359375e` does not change the AC2/AC3 Stop-SHA list.
  PR #94 tip `fe08f47` keeps M0-AC2 Partial on the joined tip. It cites #76 and does not replace `ac2_ac3_cannot_flip_while_stop_shas_stand`. ORT CoreML `LSTM`/`If` and a Metal device process-lifetime release stay out of M0. This branch does not replay `m0_ac2_ac3_partial_refuses_checked_boxes` and does not copy that note.
- [ ] **M0-AC3:** Cancellation during initialization and active inference joins the
  worker inside the shutdown reserve. A wrapper timeout alone does not pass. Linkage,
  the required llama.cpp revision, and embedded shader packaging have a recorded
  resolution that still yields one Mach-O.
  PR #49 tip `37ff031`: `vm_deallocate` returned the weight buffer and cycle 2 stayed 94699688 bytes above cycle 1. Not a bounded teardown.
  PR #55 tip `0829a70`: `llama_backend_free` does not destroy that device. No purge. Not a 100-cycle pass.
  PR #59 tip `ed9ee10` locks `0829a70`. `llama_backend_free` does not destroy `ggml_backend_metal_reg`. Not a 100-cycle pass.
  PR #63 tip `232ea78`: `ggml_metal_device_free` from taco is a use-after-free. Not a 100-cycle pass.
  PR #65 tip `04dda65` locks `232ea78`. No purge. This branch does not replay that charter guard.
  PR #70 tip `e06ea0e`: a refcount cannot free that device between prove cycles. Not a 100-cycle pass. This branch does not replay that charter guard.
  PR #71 tip `b94e2bb` cites `e06ea0e`. Not a 100-cycle pass.
  PR #72 tip `f16f0bb` locks `e06ea0e` beside `2ff5c4f`. No purge. This branch does not replay that charter guard.
  PR #73 tip `789c026` cites `f16f0bb`. Not a 100-cycle pass.
  PR #74 tip `30fd669`: a helper does not nil `ggml_backend_metal_reg`. Not a 100-cycle pass. This branch does not replay that charter guard.
  PR #76 tip `2d9679f` locks `30fd669`. No purge. This branch does not replay that charter guard.
  PR #78 tip `9a1d8f1` cites `2d9679f`. Not a 100-cycle pass.
  PR #79 tip `d04c224` is not a 100-cycle pass.
  PR #81 tip `ca9bc50` does not change the AC2/AC3 Stop-SHA list.
  PR #82 tip `637459f` does not change the AC2/AC3 Stop-SHA list.
  PR #83 tip `86dd55d` cites `2d9679f`. Not a 100-cycle pass.
  PR #84 tip `fb48acc` does not change the AC2/AC3 Stop-SHA list.
  PR #85 tip `576ac63` keeps M0-AC3 Partial. Not a 100-cycle pass. No purge. This branch does not replay `m0_ac2_ac3_partial_refuses_checked_boxes`.
  PR #86 tip `8938f5b` cites `576ac63`. Not a 100-cycle pass.
  PR #87 tip `a40fad1` does not change the AC2/AC3 Stop-SHA list.
  PR #88 tip `2766f79` does not change the AC2/AC3 Stop-SHA list.
  PR #89 tip `831d455` cites `8938f5b`. Not a 100-cycle pass.
  PR #90 tip `7e8f3c5` does not change the AC2/AC3 Stop-SHA list.
  PR #91 tip `79cb73d` keeps M0-AC3 Partial on the measure tip. Not a 100-cycle pass. No purge. This branch does not replay `m0_ac2_ac3_partial_refuses_checked_boxes`.
  PR #92 tip `6b165eb` cites `831d455`. Not a 100-cycle pass.
  PR #93 tip `359375e` does not change the AC2/AC3 Stop-SHA list.
  PR #94 tip `fe08f47` keeps M0-AC3 Partial on the joined tip. Not a 100-cycle pass. No purge. This branch does not replay `m0_ac2_ac3_partial_refuses_checked_boxes`.
- [ ] **M0-AC4:** The [acceptance plan](acceptance.md) names both corners, OS builds,
  the selected quantization and sampling, corpus and license inventory, the closed
  language table, and reviewed numeric targets. No unresolved mandatory value remains.
  Status: Partial. The box stays unchecked. Record:
  [ac4-partial.md](../evidence/m0/ac4-partial.md).
  PR #54 tip `6a839de`. Charter accept is blocked: M0-AC4 cannot be checked while SIGNED_INSTALL=None or C2_BLOCKED stands.
  The launchd user-agent run does not close M0-AC4.
  PR #62 tip `8c24c99` locks this Partial. This branch does not replay that tip's `freeze.rs`.
  PR #79 tip `d04c224` keeps the three corpora unsealed at 0 utterances. Pin digests stay measured. `taco models verify` exits 1. This does not check M0-AC4.
  PR #78 tip `9a1d8f1`, PR #80 tip `9def38b`, PR #81 tip `ca9bc50`, PR #82 tip `637459f`, PR #83 tip `86dd55d`, PR #84 tip `fb48acc`, PR #85 tip `576ac63`, PR #86 tip `8938f5b`, PR #87 tip `a40fad1`, PR #88 tip `2766f79`, PR #89 tip `831d455`, PR #90 tip `7e8f3c5`, PR #91 tip `79cb73d`, PR #92 tip `6b165eb`, PR #93 tip `359375e`, and PR #94 tip `fe08f47` do not check M0-AC4.
- [ ] **M0-AC5:** Every M0 item in the [decision register](risks-and-decisions.md) has
  a recorded decision, owner, evidence, and consequence. ADRs are accepted only to the
  extent supported; unresolved feasibility explicitly blocks dependent milestones.
- [ ] **M0-AC6:** Shared library types, `T-M0-REJECT-INPUTS`, and `T-M0-LIMIT-VECTORS`
  match the ADRs and the acceptance plan. Stage traits can be faked independently.
  The test plan covers every source category, both UIs, and both corners.

M0-AC5 and M0-AC6 stay unchecked while the AC1 Partial and the AC4 Partial stand. PR #62 tip `8c24c99`.

## Evidence and handoff

Deliver a feasibility report, pinned dependency/model manifest, command/event fixture
specification, benchmark comparison, and accepted/revised decisions. Future reports
may live in the repository's approved evidence location; this plan does not create
those runtime artifacts. M1 begins only after M0 is accepted, including a viable model bundle and both qualification corners.

If single-file permissions, native coexistence, bounded cancellation, or reasonable
combined performance fail, stop and revise the relevant ADR with measured evidence.
Do not conceal the failure by adding an inference server or dropping app capture.
