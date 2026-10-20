# M0 — Feasibility and executable contracts

Status: **Not accepted.** Shared `taco` types and default-tier tests are in the tree. Hardware spikes are recorded blockers in the decision register. Acceptance checkboxes stay open. Depends on: charter only.
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
  [process-tap-product.md](../evidence/m0/process-tap-product.md) is Partial.
  Product `taco run` dispatches `system`, `app:`, and `pid:` (PR #11 tip `cd9a4c8`,
  ancestor of stack base `a1c5c32`). Product-path freeze is #75 tip `dc1b9be`
  (`dc1b9bef6735d37315e4a32ef0893dd2ac20df11`). CoreAudio and operator Allow/Deny stay Mac-only.
  Linux tap success is a Stop.
  Partial — process taps are on taco run; Mac CoreAudio and operator Allow/Deny stay open; Linux tap success is Stop.
  Allow tip SHA, Deny tip SHA, and Cleanup tip SHA stay `none` in
  [process-tap-product.md](../evidence/m0/process-tap-product.md). Drop is not a Cleanup tip SHA.
  M0-AC1 stays unchecked while Allow, Deny, or Cleanup tip SHA is still `none`.
  [ac1-full-session.md](../evidence/m0/ac1-full-session.md) cites PR #77 tip `595dced`.
  Microphone and AudioCapture Allow/Deny tip SHAs are none. TIMEOUT is not Deny.
  Partial — mic and AudioCapture Allow/Deny tip SHAs are none; consequence AC1 open.
- [ ] **M0-AC2:** The combined native spike performs ASR and translation with the exact
  pinned artifacts, in process, with observed Metal GPU execution. A subprocess CLI
  or a CPU-only `-ngl 0` run does not pass.
- [ ] **M0-AC3:** Cancellation during initialization and active inference joins the
  worker inside the shutdown reserve. A wrapper timeout alone does not pass. Linkage,
  the required llama.cpp revision, and embedded shader packaging have a recorded
  resolution that still yields one Mach-O.
- [ ] **M0-AC4:** The [acceptance plan](acceptance.md) names both corners, OS builds,
  the selected quantization and sampling, corpus and license inventory, the closed
  language table, and reviewed numeric targets. No unresolved mandatory value remains.
- [ ] **M0-AC5:** Every M0 item in the [decision register](risks-and-decisions.md) has
  a recorded decision, owner, evidence, and consequence. ADRs are accepted only to the
  extent supported; unresolved feasibility explicitly blocks dependent milestones.
- [ ] **M0-AC6:** Shared library types, `T-M0-REJECT-INPUTS`, and `T-M0-LIMIT-VECTORS`
  match the ADRs and the acceptance plan. Stage traits can be faked independently.
  The test plan covers every source category, both UIs, and both corners.

## Evidence and handoff

Deliver a feasibility report, pinned dependency/model manifest, command/event fixture
specification, benchmark comparison, and accepted/revised decisions. Future reports
may live in the repository's approved evidence location; this plan does not create
those runtime artifacts. M1 begins only after M0 is accepted, including a viable model bundle and both qualification corners.

If single-file permissions, native coexistence, bounded cancellation, or reasonable
combined performance fail, stop and revise the relevant ADR with measured evidence.
Do not conceal the failure by adding an inference server or dropping app capture.

## AC2/AC3 Stop consequence

Cites #76 tip `2d9679f` and [ac2-ac3-open.md](../evidence/m0/ac2-ac3-open.md). `ac2_ac3_cannot_flip_while_stop_shas_stand` is on this tip unchanged. M0-AC2 and M0-AC3 stay unchecked.

Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. Partial is not an accept. Checking either box requires [vad-metal-ep.md](../evidence/m0/vad-metal-ep.md) to drop `d194012` and [engines-residency.md](../evidence/m0/engines-residency.md) to drop `a0e1614`. R2 and R3 name the CPU VAD Stop. R5 names the residency Stop. Neither box above is a pass.

Newer Stops still stand: #55 tip `0829a70` (residual phys_footprint is the process-lifetime Metal device in `ggml_backend_metal_reg`) and #57 tip `88eda06` (`EnableOnSubgraphs`; ORT 1.31 `GetOpBuilders` lacks `LSTM` and `If`; `device=` stays `cpu`). Prior Stops stay cited: #49 tip `37ff031` and #41 tip `a8e2eca`.

#49 tip `37ff031` freed the 487014400-byte weight buffer and cycle 2 stayed 94699688 bytes above cycle 1. It does not drop `a0e1614`. #41 tip `a8e2eca` is the prior consequence freeze. #55 tip `0829a70` names the leftover mapping as the process-lifetime device `ggml_backend_metal_reg` installs; `llama_backend_free` does not destroy it. No purge is in this tree. #57 tip `88eda06` sets provider option `EnableOnSubgraphs` on the product session. ORT 1.31.0 `GetOpBuilders` has no `LSTM` builder and no `If` builder, so `device=` stays `cpu`. This branch does not replay those Mac runs and adds no Metal measurement.

#60 tip `aab661c` is a Stop: the Silero pin stays, and no published Silero ONNX is CoreML-acceptable under ORT 1.31 (other files still contain LSTM, or a top-level If whose branch fails the Pad and Slice checks). #63 tip `232ea78` deepens the #55 Metal-device Stop: pinned llama.cpp has no safe teardown of that process-lifetime device without a use-after-free. #64 tip `c9a6f6f` is a complementary Stop: an ORT upgrade is out of M0, because `main` `GetOpBuilders` is still the v1.31.0 blob and still has no `LSTM` or `If`. This branch does not copy that guard. Checking M0-AC2 or M0-AC3 still fails while `aab661c` and `232ea78` stand. This note adds no Metal measurement.

#69 tip `2ff5c4f` is a Stop: the VAD stays Silero-only, and the charter forbids an alternate VAD pin. #70 tip `e06ea0e` deepens the #55 and #63 Metal-device Stops: a Metal device refcount free is still a use-after-free on prove cycle 2. `0829a70` and `232ea78` stay. #64 tip `c9a6f6f` still owns `ort_coreml_upgrade_cannot_flip_ac2`; this branch cites that Stop and does not copy that guard. Checking M0-AC2 or M0-AC3 still fails while `2ff5c4f` and `e06ea0e` stand. This note adds no Metal measurement.

#74 tip `30fd669` is a Stop: a Metal helper process violates single Mach-O, in-process Metal, and LINK-THREE, and it does not close residency. #74 owns `metal_subprocess_stop_cites_55_63_and_70`; this branch cites that Stop and does not copy that guard. `0829a70`, `232ea78`, `e06ea0e`, and `2ff5c4f` stay. Checking M0-AC2 or M0-AC3 still fails while `30fd669` stands. This note adds no Metal measurement.
