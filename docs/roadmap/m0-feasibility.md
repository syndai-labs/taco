# M0 — Feasibility and executable contracts

Status: **Not accepted.** Shared `taco` types and default-tier tests are in the tree. Hardware spikes are recorded blockers in the decision register. Acceptance checkboxes stay open. Depends on: charter only.

Cited tips: redundant sibling `0ee1065` (PR #95, on `cursor/m0-destroy-trace-on-partials-b19b`, same tree as #94 `fe08f47`, Mac prefer tip is #138 `55fa38f` not #136, guard not replayed, boxes unchecked), tips20 cite `008cacb` (PR #96, cites #92 `6b165eb` through #94 `fe08f47`, not an ancestor of this tip, boxes unchecked), tips21 cite `d0877e0` (PR #97, cites #95 `0ee1065` and #96 `008cacb`, not an ancestor of this tip, does not join #95 onto #94, boxes unchecked), pins on prefer `7f8ab75` (PR #98, `pins_are_not_a_verify_pass_and_corpora_stay_unsealed` already on this tip, not replayed, corpora unsealed, `device=` stays `cpu`, boxes unchecked), AC4 Partial `e2b7b01` (PR #99, `m0_ac4_cannot_be_checked_while_gaps_stand` already on this tip, not replayed, `SIGNED_INSTALL` stays None, boxes unchecked), destroy-prose refuse `c32735f` (PR #100, `m0_cleanup_prose_destroy_names_without_evidence_refuses_ac1` already on this tip, not replayed, Cleanup tip SHA none, `destroy_traced` stays no, boxes unchecked), tip-SHA denylist `58f0fe5` (PR #101, `tip_sha_denylist_rejects_non_clicks` and `m0_checked_box_variants_refuse` already on this tip, not replayed, Allow Deny and Cleanup tip SHAs none, boxes unchecked), prefer-tip evidence `a0b6d95` (PR #102, After-pair tip SHA none, Stop notes vendored, TIMEOUT is not Deny, boxes unchecked), prefer-tip join `f5494b4` (PR #103, joins #100 `c32735f`, #101 `58f0fe5`, #102 `a0b6d95`, #98 `7f8ab75`, and #99 `e2b7b01` onto #94 `fe08f47`, guards not replayed, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), tip-SHA denylist refresh `e6fd0a9` (PR #105, on `cursor/m0-tip-sha-denylist-refresh-b19b`, `tip_sha_denylist_rejects_non_clicks` refuses join-cited tokens, not an ancestor of this tip, not copied, boxes unchecked), After-pair early return `7f11ab8` (PR #106, on `cursor/m0-after-pair-early-return-b19b`, `process_tap_early_return_ok` requires After-pair, not copied, After-pair tip SHA none, boxes unchecked), prefer-tip harness join `eba8eb1` (PR #107, joins #105 `e6fd0a9` and #106 `7f11ab8` onto #103 `f5494b4`, guards not replayed, not the Mac prefer tip, boxes unchecked), residual tip-SHA denylist `32986ad` (PR #108, on `cursor/m0-tip-sha-denylist-residual-b19b`, full hashes of the eight residual shorts, star-checked M0-AC4 counts, playbook preferred this tip until #110, guards not copied, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), prefer-self tip-SHA denylist `022cec3` (PR #110, on `cursor/m0-tip-sha-denylist-prefer-self-b19b`, stacked on #108 `32986ad`, denies prefer-tip and join-head shorts and fulls including tips23 `65e314f`, five shorts already denied gain their full hashes, shorts without a local full stay short-only, `tip_sha_denylist_rejects_non_clicks` not copied, playbook preferred this tip until #112, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), tips24 cite `35155fc` (PR #111, on `cursor/m0-decision-tips24-b19b`, stacked on #109 `65e314f`, cites #110 `022cec3`, not an ancestor of #109, guard not copied, not the Mac prefer tip, boxes unchecked), self-guard tip-SHA denylist `d66894e` (PR #112, on `cursor/m0-tip-sha-denylist-self-guard-b19b`, stacked on #110 `022cec3`, denies #110 `022cec3` as an operator click, `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #111, playbook preferred this tip until #114, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), tips25 cite `7a5884e` (PR #113, on `cursor/m0-decision-tips25-b19b`, stacked on #111 `35155fc`, cites #111 `35155fc` and #112 `d66894e`, #112 not an ancestor of #111, guards not copied, not the Mac prefer tip, boxes unchecked), residual-fulls tip-SHA denylist `237347f` (PR #114, on `cursor/m0-tip-sha-denylist-residual-fulls-b19b`, stacked on #112 `d66894e`, denies #111 `35155fc`, #113 `7a5884e`, fifteen residual fulls, and `3602000`, `tip_sha_denylist_rejects_non_clicks` not copied, not an ancestor of #113, playbook preferred this tip until #115, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), deny-112 tip-SHA denylist `8b82bbd` (PR #115, on `cursor/m0-tip-sha-denylist-deny-112-b19b`, stacked on #114 `237347f`, denies #112 `d66894e`, #114 `237347f`, and parent `0f14be3`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #116, playbook preferred this tip until #118, tip self-accepts, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), tips26 cite `7fe1985` (PR #116, on `cursor/m0-decision-tips26-b19b`, stacked on #113 `7a5884e`, cites #113 `7a5884e` and #114 `237347f`, #114 not an ancestor of #113, guards not copied, not the Mac prefer tip, boxes unchecked), tips27 cite `0a962b2` (PR #117, on `cursor/m0-decision-tips27-b19b`, stacked on #116 `7fe1985`, cites #115 `8b82bbd` and #116 `7fe1985`, #115 not an ancestor of #116, guards not copied, not the Mac prefer tip, boxes unchecked), deny-115 tip-SHA denylist `86e9393` (PR #118, on `cursor/m0-tip-sha-denylist-deny-115-b19b`, stacked on #115 `8b82bbd`, denies #115 `8b82bbd`, #116 `7fe1985`, and #117 `0a962b2`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #117, playbook preferred this tip until #120, tip self-accepts, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), tips28 cite `11ffe70` (PR #119, on `cursor/m0-decision-tips28-b19b`, stacked on #117 `0a962b2`, cites #117 `0a962b2` and #118 `86e9393`, #118 not an ancestor of #117, guards not copied, not the Mac prefer tip, boxes unchecked), deny-118 tip-SHA denylist `5c833d8` (PR #120, on `cursor/m0-tip-sha-denylist-deny-118-b19b`, stacked on #118 `86e9393`, denies #118 `86e9393`, #119 `11ffe70`, and cited fetch pointer `4811926`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #119, playbook preferred this tip until #122, tip self-accepts, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), tips29 cite `59deb5f` (PR #121, on `cursor/m0-decision-tips29-b19b`, stacked on #119 `11ffe70`, cites #119 `11ffe70` and #120 `5c833d8`, #120 not an ancestor of #119, guards not copied, not the Mac prefer tip, boxes unchecked), deny-120 tip-SHA denylist `7b14d7b` (PR #122, on `cursor/m0-tip-sha-denylist-deny-120-b19b`, stacked on #120 `5c833d8`, denies #120 `5c833d8` and #121 `59deb5f`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #121, playbook preferred this tip until #124, tip self-accepts, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), tips30 cite `a437589` (PR #123, on `cursor/m0-decision-tips30-b19b`, stacked on #121 `59deb5f`, cites #121 `59deb5f` and #122 `7b14d7b`, #122 not an ancestor of #121, guards not copied, not the Mac prefer tip, boxes unchecked), deny-122 tip-SHA denylist `6d85580` (PR #124, on `cursor/m0-tip-sha-denylist-deny-122-b19b`, stacked on #122 `7b14d7b`, denies #122 `7b14d7b` and #123 `a437589`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #123, playbook preferred this tip until #126, tip self-accepts, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), tips31 cite `4fa97cc` (PR #125, on `cursor/m0-decision-tips31-b19b`, stacked on #123 `a437589`, cites #123 `a437589` and #124 `6d85580`, #124 not an ancestor of #123, guards not copied, not the Mac prefer tip, boxes unchecked), deny-124 tip-SHA denylist `275f945` (PR #126, on `cursor/m0-tip-sha-denylist-deny-124-b19b`, stacked on #124 `6d85580`, denies #124 `6d85580` and #125 `4fa97cc`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #125, playbook preferred this tip until #128, tip self-accepts, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), tips32 cite `d6b0c1c` (PR #127, on `cursor/m0-decision-tips32-b19b`, stacked on #125 `4fa97cc`, cites #125 `4fa97cc` and #126 `275f945`, #126 not an ancestor of #125, guards not copied, not the Mac prefer tip, boxes unchecked), deny-126 tip-SHA denylist `a2a810a` (PR #128, on `cursor/m0-tip-sha-denylist-deny-126-b19b`, stacked on #126 `275f945`, denies #126 `275f945` and #127 `d6b0c1c`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #127, playbook preferred this tip until #130, tip self-accepts, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked), tips33 cite `5701575` (PR #129, on `cursor/m0-decision-tips33-b19b`, stacked on #127 `d6b0c1c`, cites #127 `d6b0c1c` and #128 `a2a810a`, #128 not an ancestor of #127, guards not copied, not the Mac prefer tip, boxes unchecked), deny-128 tip-SHA denylist `7157c5f` (PR #130, on `cursor/m0-tip-sha-denylist-deny-128-b19b`, stacked on #128 `a2a810a`, denies #128 `a2a810a` and #129 `5701575`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #129, not the Mac prefer tip, boxes unchecked), tips34 cite `321274f` (PR #131, on `cursor/m0-decision-tips34-b19b`, stacked on #129 `5701575`, cites #129 `5701575` and #130 `7157c5f`, #130 not an ancestor of #129, guards not copied, not the Mac prefer tip, boxes unchecked), deny-130 tip-SHA denylist `f24ded0` (PR #132, on `cursor/m0-tip-sha-denylist-deny-130-b19b`, stacked on #130 `7157c5f`, denies #130 `7157c5f` and #131 `321274f`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #131, not the Mac prefer tip, boxes unchecked), tips35 cite `d543baf` (PR #133, on `cursor/m0-decision-tips35-b19b`, stacked on #131 `321274f`, cites #131 `321274f` and #132 `f24ded0`, #132 not an ancestor of #131, guards not copied, not the Mac prefer tip, boxes unchecked), deny-132 tip-SHA denylist `a2ad5b2` (PR #134, on `cursor/m0-tip-sha-denylist-deny-132-b19b`, stacked on #132 `f24ded0`, denies #132 `f24ded0` and #133 `d543baf`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #133, not the Mac prefer tip, boxes unchecked), tips36 cite `43397ff` (PR #135, on `cursor/m0-decision-tips36-b19b`, stacked on #133 `d543baf`, cites #133 `d543baf` and #134 `a2ad5b2`, #134 not an ancestor of #133, guards not copied, not the Mac prefer tip, boxes unchecked), deny-134 tip-SHA denylist `5ab4f8f` (PR #136, on `cursor/m0-tip-sha-denylist-deny-134-b19b`, stacked on #134 `a2ad5b2`, denies #134 `a2ad5b2` and #135 `43397ff`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #135, not the Mac prefer tip, boxes unchecked), tips37 cite `e101b38` (PR #137, on `cursor/m0-decision-tips37-b19b`, stacked on #135 `43397ff`, cites #135 `43397ff` and #136 `5ab4f8f`, #136 not an ancestor of #135, guards not copied, not the Mac prefer tip, boxes unchecked), deny-136 tip-SHA denylist `55fa38f` (PR #138, on `cursor/m0-tip-sha-denylist-deny-136-b19b`, stacked on #136 `5ab4f8f`, denies #136 `5ab4f8f` and #137 `e101b38`, `tip_sha_denylist_rejects_non_clicks` and `tip_under_test_sha_is_denied_as_operator_click` not copied, not an ancestor of #137, Mac prefer tip, this cite does not replace it, tip self-accepts, Allow Deny Cleanup and After-pair tip SHAs none, Linux is not a Metal pass, boxes unchecked).
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
  PR #95 tip `0ee1065`, PR #96 tip `008cacb`, PR #97 tip `d0877e0`, PR #98 tip `7f8ab75`, PR #99 tip `e2b7b01`, PR #100 tip `c32735f`, PR #101 tip `58f0fe5`, PR #102 tip `a0b6d95`, PR #103 tip `f5494b4`, PR #105 tip `e6fd0a9`, PR #106 tip `7f11ab8`, PR #107 tip `eba8eb1`, PR #108 tip `32986ad`, PR #110 tip `022cec3`, PR #111 tip `35155fc`, PR #112 tip `d66894e`, PR #113 tip `7a5884e`, PR #114 tip `237347f`, PR #115 tip `8b82bbd`, PR #116 tip `7fe1985`, PR #117 tip `0a962b2`, PR #118 tip `86e9393`, PR #119 tip `11ffe70`, PR #120 tip `5c833d8`, PR #121 tip `59deb5f`, PR #122 tip `7b14d7b`, PR #123 tip `a437589`, PR #124 tip `6d85580`, PR #125 tip `4fa97cc`, PR #126 tip `275f945`, PR #127 tip `d6b0c1c`, PR #128 tip `a2a810a`, PR #129 tip `5701575`, PR #130 tip `7157c5f`, PR #131 tip `321274f`, PR #132 tip `f24ded0`, PR #133 tip `d543baf`, PR #134 tip `a2ad5b2`, PR #135 tip `43397ff`, PR #136 tip `5ab4f8f`, PR #137 tip `e101b38`, and PR #138 tip `55fa38f` keep Allow, Deny, and Cleanup tip SHAs `none`. After-pair tip SHA stays `none`. `destroy_traced` stays `no`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. This cite does not copy `m0_cleanup_prose_destroy_names_without_evidence_refuses_ac1` or `m0_checked_box_variants_refuse` again.
- [ ] **M0-AC2:** The combined native spike performs ASR and translation with the exact
  pinned artifacts, in process, with observed Metal GPU execution. A subprocess CLI
  or a CPU-only `-ngl 0` run does not pass.
  PR #95 tip `0ee1065`, PR #96 tip `008cacb`, PR #97 tip `d0877e0`, PR #98 tip `7f8ab75`, PR #99 tip `e2b7b01`, PR #100 tip `c32735f`, PR #101 tip `58f0fe5`, PR #102 tip `a0b6d95`, PR #103 tip `f5494b4`, PR #105 tip `e6fd0a9`, PR #106 tip `7f11ab8`, PR #107 tip `eba8eb1`, PR #108 tip `32986ad`, PR #110 tip `022cec3`, PR #111 tip `35155fc`, PR #112 tip `d66894e`, PR #113 tip `7a5884e`, PR #114 tip `237347f`, PR #115 tip `8b82bbd`, PR #116 tip `7fe1985`, PR #117 tip `0a962b2`, PR #118 tip `86e9393`, PR #119 tip `11ffe70`, PR #120 tip `5c833d8`, PR #121 tip `59deb5f`, PR #122 tip `7b14d7b`, PR #123 tip `a437589`, PR #124 tip `6d85580`, PR #125 tip `4fa97cc`, PR #126 tip `275f945`, PR #127 tip `d6b0c1c`, PR #128 tip `a2a810a`, PR #129 tip `5701575`, PR #130 tip `7157c5f`, PR #131 tip `321274f`, PR #132 tip `f24ded0`, PR #133 tip `d543baf`, PR #134 tip `a2ad5b2`, PR #135 tip `43397ff`, PR #136 tip `5ab4f8f`, PR #137 tip `e101b38`, and PR #138 tip `55fa38f` do not print `device=metal`. `device=` stays `cpu`. ORT CoreML `LSTM` and `If`, and a Metal device process-lifetime release, stay out of M0.
- [ ] **M0-AC3:** Cancellation during initialization and active inference joins the
  worker inside the shutdown reserve. A wrapper timeout alone does not pass. Linkage,
  the required llama.cpp revision, and embedded shader packaging have a recorded
  resolution that still yields one Mach-O.
  PR #95 tip `0ee1065`, PR #96 tip `008cacb`, PR #97 tip `d0877e0`, PR #98 tip `7f8ab75`, PR #99 tip `e2b7b01`, PR #100 tip `c32735f`, PR #101 tip `58f0fe5`, PR #102 tip `a0b6d95`, PR #103 tip `f5494b4`, PR #105 tip `e6fd0a9`, PR #106 tip `7f11ab8`, PR #107 tip `eba8eb1`, PR #108 tip `32986ad`, PR #110 tip `022cec3`, PR #111 tip `35155fc`, PR #112 tip `d66894e`, PR #113 tip `7a5884e`, PR #114 tip `237347f`, PR #115 tip `8b82bbd`, PR #116 tip `7fe1985`, PR #117 tip `0a962b2`, PR #118 tip `86e9393`, PR #119 tip `11ffe70`, PR #120 tip `5c833d8`, PR #121 tip `59deb5f`, PR #122 tip `7b14d7b`, PR #123 tip `a437589`, PR #124 tip `6d85580`, PR #125 tip `4fa97cc`, PR #126 tip `275f945`, PR #127 tip `d6b0c1c`, PR #128 tip `a2a810a`, PR #129 tip `5701575`, PR #130 tip `7157c5f`, PR #131 tip `321274f`, PR #132 tip `f24ded0`, PR #133 tip `d543baf`, PR #134 tip `a2ad5b2`, PR #135 tip `43397ff`, PR #136 tip `5ab4f8f`, PR #137 tip `e101b38`, and PR #138 tip `55fa38f` are not a 100-cycle pass. This cite does not copy `m0_ac2_ac3_partial_refuses_checked_boxes` again.
- [ ] **M0-AC4:** The [acceptance plan](acceptance.md) names both corners, OS builds,
  the selected quantization and sampling, corpus and license inventory, the closed
  language table, and reviewed numeric targets. No unresolved mandatory value remains.
  Status: Partial. The box stays unchecked. Record:
  [ac4-partial.md](../evidence/m0/ac4-partial.md).
  Charter accept is blocked: M0-AC4 cannot be checked while SIGNED_INSTALL=None or C2_BLOCKED stands.
  The launchd user-agent run does not close M0-AC4.
  PR #99 tip `e2b7b01` freezes this Partial on the prefer tip. PR #95 tip `0ee1065`, PR #96 tip `008cacb`, PR #97 tip `d0877e0`, PR #98 tip `7f8ab75`, PR #100 tip `c32735f`, PR #101 tip `58f0fe5`, PR #102 tip `a0b6d95`, PR #103 tip `f5494b4`, PR #105 tip `e6fd0a9`, PR #106 tip `7f11ab8`, PR #107 tip `eba8eb1`, PR #108 tip `32986ad`, PR #110 tip `022cec3`, PR #111 tip `35155fc`, PR #112 tip `d66894e`, PR #113 tip `7a5884e`, PR #114 tip `237347f`, PR #115 tip `8b82bbd`, PR #116 tip `7fe1985`, PR #117 tip `0a962b2`, PR #118 tip `86e9393`, PR #119 tip `11ffe70`, PR #120 tip `5c833d8`, PR #121 tip `59deb5f`, PR #122 tip `7b14d7b`, PR #123 tip `a437589`, PR #124 tip `6d85580`, PR #125 tip `4fa97cc`, PR #126 tip `275f945`, PR #127 tip `d6b0c1c`, PR #128 tip `a2a810a`, PR #129 tip `5701575`, PR #130 tip `7157c5f`, PR #131 tip `321274f`, PR #132 tip `f24ded0`, PR #133 tip `d543baf`, PR #134 tip `a2ad5b2`, PR #135 tip `43397ff`, PR #136 tip `5ab4f8f`, PR #137 tip `e101b38`, and PR #138 tip `55fa38f` do not check M0-AC4. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it.
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

PR #95 tip `0ee1065` does not change the AC2/AC3 Stop-SHA list. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. PR #96 tip `008cacb` and PR #97 tip `d0877e0` cite earlier tips and do not close either box. PR #98 tip `7f8ab75` does not print `device=metal`. PR #99 tip `e2b7b01` does not close M0-AC2 or M0-AC3. PR #100 tip `c32735f` and PR #101 tip `58f0fe5` do not change the Stop-SHA list. PR #102 tip `a0b6d95` vendors the `d194012` and `a0e1614` Stop notes and does not drop them. PR #103 tip `f5494b4` joins those tips onto #94 tip `fe08f47`. It cites `ac2_ac3_cannot_flip_while_stop_shas_stand` and does not replace it. ORT CoreML `LSTM` and `If`, and a Metal device process-lifetime release, stay out of M0. This cite does not replay those guards and does not add a Metal measurement. Neither box above is a pass.

PR #105 tip `e6fd0a9` does not change the AC2/AC3 Stop-SHA list. PR #106 tip `7f11ab8` does not close M0-AC2 or M0-AC3. PR #107 tip `eba8eb1` joins those harness guards onto #103 and does not print `device=metal`. PR #108 tip `32986ad` does not change the Stop-SHA list. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not replay those guards and does not add a Metal measurement. Neither box above is a pass.

PR #110 tip `022cec3` denies prefer-tip shorts and fulls, including tips23 tip `65e314f`, and the full hashes of five shorts already denied. Shorts whose fulls were not in that clone stay short-only. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` and does not add a Metal measurement. Neither box above is a pass.

PR #111 tip `35155fc` cites #110 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` and does not add a Metal measurement. Neither box above is a pass.

PR #112 tip `d66894e` denies #110 tip `022cec3` as an operator click. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #113 tip `7a5884e` cites #111 and #112 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #114 tip `237347f` denies #111 tip `35155fc`, #113 tip `7a5884e`, fifteen residual fulls, and `3602000` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` and does not add a Metal measurement. Neither box above is a pass.

PR #115 tip `8b82bbd` denies #112 tip `d66894e`, #114 tip `237347f`, and parent `0f14be3` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #116 tip `7fe1985` cites #113 and #114 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #117 tip `0a962b2` cites #115 and #116 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #118 tip `86e9393` denies #115 tip `8b82bbd`, #116 tip `7fe1985`, and #117 tip `0a962b2` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #119 tip `11ffe70` cites #117 and #118 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #120 tip `5c833d8` denies #118 tip `86e9393`, #119 tip `11ffe70`, and cited fetch pointer `4811926` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #121 tip `59deb5f` cites #119 and #120 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #122 tip `7b14d7b` denies #120 tip `5c833d8` and #121 tip `59deb5f` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #123 tip `a437589` cites #121 and #122 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #124 tip `6d85580` denies #122 tip `7b14d7b` and #123 tip `a437589` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #125 tip `4fa97cc` cites #123 and #124 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #126 tip `275f945` denies #124 tip `6d85580` and #125 tip `4fa97cc` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #127 tip `d6b0c1c` cites #125 and #126 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #128 tip `a2a810a` denies #126 tip `275f945` and #127 tip `d6b0c1c` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #129 tip `5701575` cites #127 and #128 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #130 tip `7157c5f` denies #128 tip `a2a810a` and #129 tip `5701575` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #131 tip `321274f` cites #129 and #130 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #132 tip `f24ded0` denies #130 tip `7157c5f` and #131 tip `321274f` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #133 tip `d543baf` cites #131 and #132 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #134 tip `a2ad5b2` denies #132 tip `f24ded0` and #133 tip `d543baf` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #135 tip `43397ff` cites #133 and #134 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #136 tip `5ab4f8f` denies #134 tip `a2ad5b2` and #135 tip `43397ff` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #137 tip `e101b38` cites #135 and #136 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`, not #136. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #138 tip `55fa38f` denies #136 tip `5ab4f8f` and #137 tip `e101b38` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #138 `55fa38f`. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

