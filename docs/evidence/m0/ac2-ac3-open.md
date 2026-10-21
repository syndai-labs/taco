# M0-AC2 and M0-AC3 stay unchecked

Cited from `cursor/m0-ac23-honesty2-b19b` tip `7ae6d98` (PR #35). This branch does not replay that note. No acceptance box is checked.

(a) Still open. Silero is the ORT CPU execution provider (`device=cpu` on the #30 `taco run`, `ep=CPU` on the #32 residency series).

(b) Closed for prove reachable from `taco run`. #30 tip `e2ec751` printed `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0` from an ad-hoc signed Mac `taco run` and exited 0. The signature is ad-hoc, not Developer ID. `ac2` and `ac3` stayed 0.

(c) Still blocks M0-AC2 and M0-AC3. #32 tip `a0e1614` is a Stop, not a fix. The 100-cycle `phys_footprint` delta is 252346632 bytes against a 33554432-byte tolerance. `whisper_free` retains the bytes. A Metal command-buffer flush missed the band (6-cycle delta 98812072) and was reverted. The earlier cite `0972b2d` recorded the same 100-cycle miss.

- [ ] **M0-AC2:** CPU VAD is open. Prove from `taco run` does not close this box. #57 tip `88eda06` does not close it. #60 tip `aab661c` does not close it. #64 tip `c9a6f6f` does not close it. #69 tip `2ff5c4f` does not close it. #74 tip `30fd669` does not close it.
- [ ] **M0-AC3:** The residency Stop at `a0e1614` is not a bounded teardown. #55 tip `0829a70` does not close it. #63 tip `232ea78` does not close it. #70 tip `e06ea0e` does not close it. #74 tip `30fd669` does not close it.

Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. Partial is not an accept. Checking either box requires [vad-metal-ep.md](vad-metal-ep.md) to drop `d194012` and [engines-residency.md](engines-residency.md) to drop `a0e1614`. R2 and R3 name the CPU VAD Stop. R5 names the residency Stop. Neither box above is a pass.

Newer Stops still stand: #55 tip `0829a70` (residual phys_footprint is the process-lifetime Metal device in `ggml_backend_metal_reg`) and #57 tip `88eda06` (`EnableOnSubgraphs`; ORT 1.31 `GetOpBuilders` lacks `LSTM` and `If`; `device=` stays `cpu`). Prior Stops stay cited: #49 tip `37ff031` and #41 tip `a8e2eca`.

#49 tip `37ff031` freed the 487014400-byte weight buffer and cycle 2 stayed 94699688 bytes above cycle 1. It does not drop `a0e1614`. #41 tip `a8e2eca` is the prior consequence freeze. #55 tip `0829a70` names the leftover mapping as the process-lifetime device `ggml_backend_metal_reg` installs; `llama_backend_free` does not destroy it. No purge is in this tree. #57 tip `88eda06` sets provider option `EnableOnSubgraphs` on the product session. ORT 1.31.0 `GetOpBuilders` has no `LSTM` builder and no `If` builder, so `device=` stays `cpu`. This branch does not replay those Mac runs and adds no Metal measurement.

#60 tip `aab661c` is a Stop: the Silero pin stays, and no published Silero ONNX is CoreML-acceptable under ORT 1.31 (other files still contain LSTM, or a top-level If whose branch fails the Pad and Slice checks). #63 tip `232ea78` deepens the #55 Metal-device Stop: pinned llama.cpp has no safe teardown of that process-lifetime device without a use-after-free. #64 tip `c9a6f6f` is a complementary Stop: an ORT upgrade is out of M0, because `main` `GetOpBuilders` is still the v1.31.0 blob and still has no `LSTM` or `If`. This branch does not copy that guard. Checking M0-AC2 or M0-AC3 still fails while `aab661c` and `232ea78` stand. This note adds no Metal measurement.

#69 tip `2ff5c4f` is a Stop: the VAD stays Silero-only, and the charter forbids an alternate VAD pin. #70 tip `e06ea0e` deepens the #55 and #63 Metal-device Stops: a Metal device refcount free is still a use-after-free on prove cycle 2. `0829a70` and `232ea78` stay. #64 tip `c9a6f6f` still owns `ort_coreml_upgrade_cannot_flip_ac2`; this branch cites that Stop and does not copy that guard. Checking M0-AC2 or M0-AC3 still fails while `2ff5c4f` and `e06ea0e` stand. This note adds no Metal measurement.

#74 tip `30fd669` is a Stop: a Metal helper process violates single Mach-O, in-process Metal, and LINK-THREE, and it does not close residency. #74 owns `metal_subprocess_stop_cites_55_63_and_70`; this branch cites that Stop and does not copy that guard. `0829a70`, `232ea78`, `e06ea0e`, and `2ff5c4f` stay. Checking M0-AC2 or M0-AC3 still fails while `30fd669` stands. This note adds no Metal measurement.

PR #95 tip `0ee1065` does not change the AC2/AC3 Stop-SHA list. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. PR #96 tip `008cacb` and PR #97 tip `d0877e0` cite earlier tips and do not close either box. PR #98 tip `7f8ab75` does not print `device=metal`. PR #99 tip `e2b7b01` does not close M0-AC2 or M0-AC3. PR #100 tip `c32735f` and PR #101 tip `58f0fe5` do not change the Stop-SHA list. PR #102 tip `a0b6d95` vendors the `d194012` and `a0e1614` Stop notes and does not drop them. PR #103 tip `f5494b4` joins those tips onto #94 tip `fe08f47`. It cites `ac2_ac3_cannot_flip_while_stop_shas_stand` and does not replace it. ORT CoreML `LSTM` and `If`, and a Metal device process-lifetime release, stay out of M0. This cite does not replay those guards and does not add a Metal measurement. Neither box above is a pass.

PR #105 tip `e6fd0a9` does not change the AC2/AC3 Stop-SHA list. PR #106 tip `7f11ab8` does not close M0-AC2 or M0-AC3. PR #107 tip `eba8eb1` joins those harness guards onto #103 and does not print `device=metal`. PR #108 tip `32986ad` does not change the Stop-SHA list. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not replay those guards and does not add a Metal measurement. Neither box above is a pass.

PR #110 tip `022cec3` denies prefer-tip shorts and fulls, including tips23 tip `65e314f`, and the full hashes of five shorts already denied. Shorts whose fulls were not in that clone stay short-only. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` and does not add a Metal measurement. Neither box above is a pass.

PR #111 tip `35155fc` cites #110 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` and does not add a Metal measurement. Neither box above is a pass.

PR #112 tip `d66894e` denies #110 tip `022cec3` as an operator click. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #113 tip `7a5884e` cites #111 and #112 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #114 tip `237347f` denies #111 tip `35155fc`, #113 tip `7a5884e`, fifteen residual fulls, and `3602000` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` and does not add a Metal measurement. Neither box above is a pass.

PR #115 tip `8b82bbd` denies #112 tip `d66894e`, #114 tip `237347f`, and parent `0f14be3` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #116 tip `7fe1985` cites #113 and #114 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #117 tip `0a962b2` cites #115 and #116 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #118 tip `86e9393` denies #115 tip `8b82bbd`, #116 tip `7fe1985`, and #117 tip `0a962b2` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #119 tip `11ffe70` cites #117 and #118 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #120 tip `5c833d8` denies #118 tip `86e9393`, #119 tip `11ffe70`, and cited fetch pointer `4811926` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #121 tip `59deb5f` cites #119 and #120 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #122 tip `7b14d7b` denies #120 tip `5c833d8` and #121 tip `59deb5f` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #123 tip `a437589` cites #121 and #122 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #124 tip `6d85580` denies #122 tip `7b14d7b` and #123 tip `a437589` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #125 tip `4fa97cc` cites #123 and #124 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #126 tip `275f945` denies #124 tip `6d85580` and #125 tip `4fa97cc` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #127 tip `d6b0c1c` cites #125 and #126 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #128 tip `a2a810a` denies #126 tip `275f945` and #127 tip `d6b0c1c` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #129 tip `5701575` cites #127 and #128 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #130 tip `7157c5f` denies #128 tip `a2a810a` and #129 tip `5701575` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #131 tip `321274f` cites #129 and #130 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #132 tip `f24ded0` denies #130 tip `7157c5f` and #131 tip `321274f` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #133 tip `d543baf` cites #131 and #132 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #134 tip `a2ad5b2` denies #132 tip `f24ded0` and #133 tip `d543baf` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #135 tip `43397ff` cites #133 and #134 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #136 tip `5ab4f8f` denies #134 tip `a2ad5b2` and #135 tip `43397ff` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #137 tip `e101b38` cites #135 and #136 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #138 tip `55fa38f` denies #136 tip `5ab4f8f` and #137 tip `e101b38` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #139 tip `2ecb83f` cites #137 and #138 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #140 tip `0db1082` denies #138 tip `55fa38f` and #139 tip `2ecb83f` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #141 tip `7fbe6e2` cites #139 and #140 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #142 tip `7e308bf` denies #140 tip `0db1082` and #141 tip `7fbe6e2` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #143 tip `e97e74e` cites #141 and #142 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #144 tip `97e3827` denies #142 tip `7e308bf` and #143 tip `e97e74e` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #145 tip `46c6787` cites #143 and #144 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #146 tip `49a8fb6` denies #144 tip `97e3827` and #145 tip `46c6787` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #147 tip `20a7d0b` cites #145 and #146 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #148 tip `e7406cf` denies #146 tip `49a8fb6` and #147 tip `20a7d0b` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #149 tip `2c6668c` cites #147 and #148 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #150 tip `b0f6171` denies #148 tip `e7406cf` and #149 tip `2c6668c` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #151 tip `01557f6` cites #149 and #150 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #152 tip `f6fa173` denies #150 tip `b0f6171` and #151 tip `01557f6` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #153 tip `e3a64a6` cites #151 and #152 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #154 tip `0fb62fa` denies #152 tip `f6fa173` and #153 tip `e3a64a6` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #155 tip `42fbaa0` cites #153 and #154 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #156 tip `a2254fb` denies #154 tip `0fb62fa` and #155 tip `42fbaa0` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #157 tip `7025126` cites #155 and #156 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #158 tip `fb37980` denies #156 tip `a2254fb` and #157 tip `7025126` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #159 tip `13f0594` cites #157 and #158 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #160 tip `0c6f4f4` denies #158 tip `fb37980` and #159 tip `13f0594` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #161 tip `194d740` cites #159 and #160 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #162 tip `bb6decd` denies #160 tip `0c6f4f4` and #161 tip `194d740` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #163 tip `7d2e1dd` cites #161 and #162 in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #164 tip `32a7048` denies #162 tip `bb6decd` and #163 tip `7d2e1dd` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #166. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.


PR #165 tip `59289d6` cites #163 tip `7d2e1dd` and #164 tip `32a7048` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #166 tip `13c8ae4` denies #164 tip `32a7048` and #165 tip `59289d6` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #168. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.


PR #167 tip `1781f91` cites #165 tip `59289d6` and #166 tip `13c8ae4` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #168 tip `379dec0` denies #166 tip `13c8ae4` and #167 tip `1781f91` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #170. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.


PR #169 tip `e4098fb` cites #167 tip `1781f91` and #168 tip `379dec0` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #170 tip `91e57c3` denies #168 tip `379dec0` and #169 tip `e4098fb` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #172. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.


PR #171 tip `905bd66` cites #169 tip `e4098fb` and #170 tip `91e57c3` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #172 tip `87f529a` denies #170 tip `91e57c3` and #171 tip `905bd66` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #174. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.


PR #173 tip `ddd6e6c` cites #171 tip `905bd66` and #172 tip `87f529a` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #174 tip `61e2f4f` denies #172 tip `87f529a` and #173 tip `ddd6e6c` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #176. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.


PR #175 tip `215a7cf` cites #173 tip `ddd6e6c` and #174 tip `61e2f4f` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #176 tip `177c599` denies #174 tip `61e2f4f` and #175 tip `215a7cf` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #178. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.


PR #177 tip `74f7dc3` cites #175 tip `215a7cf` and #176 tip `177c599` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #178 tip `88a3dd3` denies #176 tip `177c599` and #177 tip `74f7dc3` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #179 tip `599ba9f` cites #177 tip `74f7dc3` and #178 tip `88a3dd3` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #180 tip `ce1484f` denies #178 tip `88a3dd3` and #179 tip `599ba9f` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #181 tip `b018c3a` cites #179 tip `599ba9f` and #180 tip `ce1484f` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #182 tip `f46c9f4` denies #180 tip `ce1484f` and #181 tip `b018c3a` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #183 tip `6c38349` cites #181 tip `b018c3a` and #182 tip `f46c9f4` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #184 tip `9d8de40` denies #182 tip `f46c9f4` and #183 tip `6c38349` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #185 tip `7b19eef` cites #183 tip `6c38349` and #184 tip `9d8de40` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #186 tip `dd6bd59` denies #184 tip `9d8de40` and #185 tip `7b19eef` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #187 tip `6c5879d` cites #185 tip `7b19eef` and #186 tip `dd6bd59` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #188 tip `c4c94ea` denies #186 tip `dd6bd59` and #187 tip `6c5879d` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #189 tip `d210690` cites #187 tip `6c5879d` and #188 tip `c4c94ea` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #190 tip `167af8a` denies #188 tip `c4c94ea` and #189 tip `d210690` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #191 tip `ed45598` cites #189 tip `d210690` and #190 tip `167af8a` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #192 tip `68cd5ff` denies #190 tip `167af8a` and #191 tip `ed45598` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #193 tip `b198178` cites #191 tip `ed45598` and #192 tip `68cd5ff` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #194 tip `c0e16d5` denies #192 tip `68cd5ff` and #193 tip `b198178` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #195 tip `7eb431d` cites #193 tip `b198178` and #194 tip `c0e16d5` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #196 tip `17026b5` denies #194 tip `c0e16d5` and #195 tip `7eb431d` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #198. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.


PR #197 tip `e95c9d9` cites #195 tip `7eb431d` and #196 tip `17026b5` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #198 tip `01d24de` denies #196 tip `17026b5` and #197 tip `e95c9d9` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #200. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #199 tip `726ba49` cites #197 tip `e95c9d9` and #198 tip `01d24de` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #200 tip `620d3c9` denies #198 tip `01d24de` and #199 tip `726ba49` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #202. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #201 tip `6c0e332` cites #199 tip `726ba49` and #200 tip `620d3c9` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #202 tip `d5b4799` denies #200 tip `620d3c9` and #201 tip `6c0e332` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #204. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #203 tip `53c4090` cites #201 tip `6c0e332` and #202 tip `d5b4799` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #204 tip `ee7fe26` denies #202 tip `d5b4799` and #203 tip `53c4090` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #206. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #205 tip `46086f9` cites #203 tip `53c4090` and #204 tip `ee7fe26` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #206 tip `5bf7567` denies #204 tip `ee7fe26` and #205 tip `46086f9` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #208. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #207 tip `4490817` cites #205 tip `46086f9` and #206 tip `5bf7567` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #208 tip `4a61474` denies #206 tip `5bf7567` and #207 tip `4490817` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #210. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #209 tip `3273e7e` cites #207 tip `4490817` and #208 tip `4a61474` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #210 tip `8f034b1` denies #208 tip `4a61474` and #209 tip `3273e7e` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #212. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #211 tip `b28affa` cites #209 tip `3273e7e` and #210 tip `8f034b1` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #212 tip `7aa8ace` denies #210 tip `8f034b1` and #211 tip `b28affa` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #214. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #213 tip `71df306` cites #211 tip `b28affa` and #212 tip `7aa8ace` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #214 tip `a148b7b` denies #212 tip `7aa8ace` and #213 tip `71df306` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. The playbook preferred this tip until #216. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #215 tip `16bdd11` cites #213 tip `71df306` and #214 tip `a148b7b` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #216 tip `5678c44` denies #214 tip `a148b7b` and #215 tip `16bdd11` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip was this tip until #218. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.


PR #217 tip `7a76a8f` cites #215 tip `16bdd11` and #216 tip `5678c44` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #218 tip `cca8648` denies #216 tip `5678c44` and #217 tip `7a76a8f` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip was this tip until #220. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass

PR #219 tip `e2f4adc` cites #217 tip `7a76a8f` and #218 tip `cca8648` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #220 tip `3b0d9dc` denies #218 tip `cca8648` and #219 tip `e2f4adc` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip was this tip until #222. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #221 tip `b9adeee` cites #219 tip `e2f4adc` and #220 tip `3b0d9dc` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #222 tip `d1c3fcc` denies #220 tip `3b0d9dc` and #221 tip `b9adeee` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip was this tip until #224. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #223 tip `6c025c1` cites #221 tip `b9adeee` and #222 tip `d1c3fcc` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #224 tip `c73de09` denies #222 tip `d1c3fcc` and #223 tip `6c025c1` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip was this tip until #226. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #225 tip `4d764af` cites #223 tip `6c025c1` and #224 tip `c73de09` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #226 tip `e0d77e5` denies #224 tip `c73de09` and #225 tip `4d764af` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip was this tip until #228. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #227 tip `45e2e96` cites #225 tip `4d764af` and #226 tip `e0d77e5` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #228 tip `e85a255` denies #226 tip `e0d77e5` and #227 tip `45e2e96` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip was this tip until #230. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #229 tip `de0834f` cites #227 tip `45e2e96` and #228 tip `e85a255` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #230 tip `d6cc06b` denies #228 tip `e85a255` and #229 tip `de0834f` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is this tip. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #231 tip `c43a567` cites #229 tip `de0834f` and #230 tip `d6cc06b` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #232 tip `b1f2f6c` denies #230 tip `d6cc06b` and #231 tip `c43a567` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is this tip. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.


PR #233 tip `e0d8e6a` cites #231 tip `c43a567` and #232 tip `b1f2f6c` in the decision register. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is #681 `cd04911`, not #679. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

PR #234 tip `7086111` denies #232 tip `b1f2f6c` and #233 tip `e0d8e6a` as operator clicks. It does not change the AC2/AC3 Stop-SHA list and does not print `device=metal`. Mac prefer tip is this tip. This cite does not replace it. It does not copy `tip_sha_denylist_rejects_non_clicks` or `tip_under_test_sha_is_denied_as_operator_click` and does not add a Metal measurement. Neither box above is a pass.

