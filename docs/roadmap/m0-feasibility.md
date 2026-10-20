# M0 — Feasibility and executable contracts

Status: **Not accepted.** Shared `taco` types and default-tier tests are in the tree. Hardware spikes are recorded blockers in the decision register. Cited tips: process tap `cd9a4c8` (PR #11), metal probe `a8b2ebc` (PR #12, `link_three=1`, Silero on CPU ORT, probe is not `taco`), license inventory `c04ab04` (PR #15), adapters `05c0c73` (PR #14), engine bridge `570364c` (PR #16), R4 sampling Stop `1a52b8d` (PR #17), Whisper small SPDX `7ee17c8` (PR #18, R11 Partial), OS floor Partial macOS 27.0.1 and unexercised launchd host `8ebd0fc` (PR #19, C2 open), ORT Mach-O guards `f2b1720` (PR #21, shared dylib is not a ship path, LINK-THREE and CANCEL still panic), mic hold `a1654eb` (PR #22, `TACO_CAPTURE_SPIKE_HOLD_MS`, no Allow or Deny), product `taco` static ORT `7f98ab1` (PR #23, `otool -L` system-only, init/infer/cancel/close/prove kept, `T-M0-LINK-THREE-MODELS` and `T-M0-CANCEL-LIFECYCLE` passed via `session::prove_native` with `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`, Silero ORT CPU, Whisper VAD bad magic, ASR/MT Metal, joins under 2 s, AC2 and AC3 open), hold source assert `89759b9` (PR #24, default `1_800_000`, no TCC faked), `taco run` prove `e2ec751` (PR #30, `TACO_ENGINES_PROVE=1` on the `af2af02` gate, ad-hoc Mac run printed `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`, CPU Silero, not a Developer ID signature), mic open timeout `c7e7c2e` (PR #31, `TACO_CAPTURE_SPIKE_OPEN_MS` default `180000`, no Allow or Deny), engines residency `0972b2d` (PR #32, `phys_footprint` 218792200 bytes over the 32 MiB band, `ep=CPU`, residency test not `taco run`), AC2/AC3 honesty `7ae6d98` (PR #35, prove reachable from `taco run` is closed, residency Stop `a0e1614`, CPU VAD open), mic open default `81a2a54` (PR #36, `TACO_CAPTURE_SPIKE_OPEN_MS` default `300000`, no Allow or Deny), Silero CoreML `d194012` (PR #37, VAD CPU Stop, `CPUExecutionProvider`, `vad_ort_plan=missing`), signed install `e7433c2` (PR #38, R11 Stop, Developer ID missing, notary profile `taco-notary` missing, `spctl` rejected ad-hoc). M0-AC2 and M0-AC3 stay open. Acceptance checkboxes stay open. Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. Newer Stops still stand: #55 tip `0829a70` (residual phys_footprint is the process-lifetime Metal device in `ggml_backend_metal_reg`) and #57 tip `88eda06` (`EnableOnSubgraphs`; ORT 1.31 `GetOpBuilders` lacks `LSTM` and `If`; `device=` stays `cpu`). Prior Stops stay cited: #49 tip `37ff031` and #41 tip `a8e2eca`. This branch does not replay those Mac runs and adds no Metal measurement. Depends on: charter only.
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
  PR #51 tip `7c71d50`. TIMEOUT is not Don't Allow.
  Partial — operator Allow/Deny unproven; consequence AC1 open.
  [ac1-full-session.md](../evidence/m0/ac1-full-session.md) cites PR #77 tip `595dced`.
  Microphone tip SHAs live in [mic-operator-resume.md](../evidence/m0/mic-operator-resume.md).
  AudioCapture tip SHAs live in [audiocapture-measure.md](../evidence/m0/audiocapture-measure.md).
  Both pairs are none. TIMEOUT is not Deny.
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
  Status: Partial. Record: [ac4-partial.md](../evidence/m0/ac4-partial.md).
  Charter accept is blocked: M0-AC4 cannot be checked while SIGNED_INSTALL=None or C2_BLOCKED stands.
  The launchd user-agent run does not close M0-AC4.
- [ ] **M0-AC5:** Every M0 item in the [decision register](risks-and-decisions.md) has
  a recorded decision, owner, evidence, and consequence. ADRs are accepted only to the
  extent supported; unresolved feasibility explicitly blocks dependent milestones.
- [ ] **M0-AC6:** Shared library types, `T-M0-REJECT-INPUTS`, and `T-M0-LIMIT-VECTORS`
  match the ADRs and the acceptance plan. Stage traits can be faked independently.
  The test plan covers every source category, both UIs, and both corners.

M0-AC5 and M0-AC6 stay unchecked while the AC1 Partial and the AC4 Partial stand.

## Evidence and handoff

Deliver a feasibility report, pinned dependency/model manifest, command/event fixture
specification, benchmark comparison, and accepted/revised decisions. Future reports
may live in the repository's approved evidence location; this plan does not create
those runtime artifacts. M1 begins only after M0 is accepted, including a viable model bundle and both qualification corners.

If single-file permissions, native coexistence, bounded cancellation, or reasonable
combined performance fail, stop and revise the relevant ADR with measured evidence.
Do not conceal the failure by adding an inference server or dropping app capture.
