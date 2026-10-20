# M0-AC2 and M0-AC3 stay unchecked

Cited from `cursor/m0-ac23-honesty2-b19b` tip `7ae6d98` (PR #35). This branch does not replay that note. No acceptance box is checked.

(a) Still open. Silero is the ORT CPU execution provider (`device=cpu` on the #30 `taco run`, `ep=CPU` on the #32 residency series).

(b) Closed for prove reachable from `taco run`. #30 tip `e2ec751` printed `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0` from an ad-hoc signed Mac `taco run` and exited 0. The signature is ad-hoc, not Developer ID. `ac2` and `ac3` stayed 0.

(c) Still blocks M0-AC2 and M0-AC3. #32 tip `a0e1614` is a Stop, not a fix. The 100-cycle `phys_footprint` delta is 252346632 bytes against a 33554432-byte tolerance. `whisper_free` retains the bytes. A Metal command-buffer flush missed the band (6-cycle delta 98812072) and was reverted. The earlier cite `0972b2d` recorded the same 100-cycle miss.

- [ ] **M0-AC2:** CPU VAD is open. Prove from `taco run` does not close this box. #57 tip `88eda06` does not close it. #59 tip `ed9ee10` does not close it. #60 tip `aab661c` does not close it. #63 tip `232ea78` does not close it. #64 tip `c9a6f6f` does not close it. #65 tip `04dda65` does not close it. #69 tip `2ff5c4f` does not close it. #70 tip `e06ea0e` does not close it. #72 tip `f16f0bb` does not close it. #73 tip `789c026` does not close it. #74 tip `30fd669` does not close it. #76 tip `2d9679f` does not close it.
- [ ] **M0-AC3:** The residency Stop at `a0e1614` is not a bounded teardown. #55 tip `0829a70` does not close it. #59 tip `ed9ee10` does not close it. #63 tip `232ea78` does not close it. #65 tip `04dda65` does not close it. #70 tip `e06ea0e` does not close it. #72 tip `f16f0bb` does not close it. #73 tip `789c026` does not close it. #74 tip `30fd669` does not close it. #76 tip `2d9679f` does not close it.

PR #41 tip `a8e2eca` freezes that consequence. This branch does not replay the charter guard. No new Metal or macOS evidence.

Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. Partial is not an accept. Checking either box requires [vad-metal-ep.md](vad-metal-ep.md) to drop `d194012` and [engines-residency.md](engines-residency.md) to drop `a0e1614`. R2 and R3 name the CPU VAD Stop. R5 names the residency Stop. Neither box above is a pass.

PR #49 tip `37ff031` ([engines-residency-retry.md](engines-residency-retry.md)) is a stronger Stop. `vm_deallocate` returned the 487014400-byte weight buffer and cycle 2 stayed 94699688 bytes above cycle 1. It does not drop `a0e1614`. The synchronize and the blit are not in this tree. No acceptance box is checked.

PR #55 tip `0829a70` ([engines-residency-rootcause.md](engines-residency-rootcause.md)) names the leftover mapping as the process-lifetime Metal device in `ggml_backend_metal_reg`. `llama_backend_free` does not destroy it. No purge. It does not drop `a0e1614`.

PR #57 tip `88eda06` ([vad-coreml-subgraph.md](vad-coreml-subgraph.md)) requests `EnableOnSubgraphs`. ORT 1.31 `GetOpBuilders` lacks `LSTM` and `If`. `device=` stays `cpu`. It does not close M0-AC2. No `device=metal`.

PR #58 tip `fb4246e` cites #55, #56, and #57. Those cites do not check a box.

PR #59 tip `ed9ee10` refreshes this consequence on top of #41 tip `a8e2eca`. The flip guard on that tip also locks `0829a70` and `88eda06`. Prior Stops stay cited: #37 tip `d194012`, #32 tip `a0e1614`, #49 tip `37ff031`, and #41 tip `a8e2eca`. Checking either box above fails on that tip while those SHAs stand. This branch does not replay that charter guard. No new Mac run. No Metal measurement. Neither box above is a pass.

PR #60 tip `aab661c` keeps the pin `1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`. No published Silero ONNX is a graph ORT 1.31 CoreML can take. `device=` stays `cpu`. It does not close M0-AC2. No `device=metal`. The note on that tip is `docs/evidence/m0/vad-silero-pin-stop.md`. This branch does not copy it and does not replay that charter guard.

PR #61 tip `f5d794a` cites #58 tip `fb4246e` and #59 tip `ed9ee10`. Those cites do not check a box.

PR #63 tip `232ea78` reads pinned llama.cpp `2a66703`. `g_devices` and `ggml_metal_device_get` keep the `MTLDevice` until process exit. `ggml_metal_device_free` from taco is a use-after-free. It does not drop `0829a70` or `a0e1614`. No new Mac run. No purge. The note on that tip is `docs/evidence/m0/metal-device-teardown.md`. This branch does not copy it and does not replay `native/taco_runtime.cpp`. Neither box above is a pass.

PR #64 tip `c9a6f6f` says an ORT upgrade is out of M0. `GetOpBuilders` blob `891704e8d9ca33411424c97ec7dc75ab1600540b` is the same on v1.31.0 and on `main` `a7a1172c`. It still has no `LSTM` or `If`. The pin does not change. `device=` stays `cpu`. It does not close M0-AC2. The note on that tip is `docs/evidence/m0/vad-ort-coreml-builders.md`. This branch does not copy it and does not replay `ort_coreml_upgrade_cannot_flip_ac2`.

PR #62 tip `8c24c99` does not change the AC2/AC3 Stop-SHA list. It does not close either box above.

PR #65 tip `04dda65` refreshes this consequence on top of #59 tip `ed9ee10`. The flip guard on that tip also locks `aab661c` and `232ea78`. The #55 lock at `0829a70` is unchanged. It cites `c9a6f6f` and does not copy `ort_coreml_upgrade_cannot_flip_ac2`. Checking either box above fails on that tip while `aab661c` and `232ea78` stand. This branch does not replay that charter guard. No new Mac run. No Metal measurement. Neither box above is a pass.

PR #66 tip `4011569` cites #60, #63, #64, and #65. Those cites do not check a box.

PR #67 tip `652614b` and PR #68 tip `a1c5c32` do not change the AC2/AC3 Stop-SHA list. They do not close either box above.

PR #69 tip `2ff5c4f` keeps the VAD family Silero. An alternate ONNX is not an M0 pin. The pin stays `1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3`. `device=` stays `cpu`. The guard on that tip is `alternate_vad_cannot_flip_ac2`. It does not copy `ac2_ac3_cannot_flip_while_stop_shas_stand`. It does not close M0-AC2. No `device=metal`. The note on that tip is `docs/evidence/m0/vad-alt-coreml-stop.md`. This branch does not copy it and does not replay that charter guard.

PR #70 tip `e06ea0e` reads pinned llama.cpp `2a66703` again. A refcount cannot free the process-lifetime Metal device between prove cycles. Freeing the struct at zero is the #63 use-after-free. It does not drop `232ea78` or `0829a70`. No new Mac run. No purge. The guard on that tip is `metal_refcount_stop_cites_63_and_55`. It does not read this note and does not copy `ac2_ac3_cannot_flip_while_stop_shas_stand`. The note on that tip is `docs/evidence/m0/metal-device-refcount.md`. This branch does not copy it and does not replay `native/taco_runtime.cpp`. Neither box above is a pass.

PR #71 tip `b94e2bb` cites #67 tip `652614b`, #68 tip `a1c5c32`, #69 tip `2ff5c4f`, and #70 tip `e06ea0e`. Those cites do not check a box.

PR #72 tip `f16f0bb` refreshes this consequence on top of #65 tip `04dda65`. The flip guard on that tip also locks #69 tip `2ff5c4f` and #70 tip `e06ea0e` into `ac2_ac3_cannot_flip_while_stop_shas_stand`. The locks at `0829a70` and `232ea78` stay. It cites `c9a6f6f` and does not copy `ort_coreml_upgrade_cannot_flip_ac2`. Checking either box above fails on that tip while `2ff5c4f` and `e06ea0e` stand. This branch does not replay that charter guard. No new Mac run. No Metal measurement. Neither box above is a pass.

PR #73 tip `789c026` cites #71 tip `b94e2bb` and #72 tip `f16f0bb`. Those cites do not check a box.

PR #74 tip `30fd669` is a Metal subprocess Stop stacked on #70 tip `e06ea0e`. A helper process violates single Mach-O, in-process Metal, and LINK-THREE. It cites #55 tip `0829a70`, #63 tip `232ea78`, and #70 tip `e06ea0e`. It does not drop them. No new Mac run. No purge. The guard on that tip is `metal_subprocess_stop_cites_55_63_and_70`. It does not read this note and does not replace `metal_refcount_stop_cites_63_and_55`. It is not `ac2_ac3_cannot_flip_while_stop_shas_stand`. The note on that tip is `docs/evidence/m0/metal-subprocess.md`. This branch does not copy it and does not replay that charter guard. Neither box above is a pass.

PR #75 tip `dc1b9be` does not change the AC2/AC3 Stop-SHA list. It does not close either box above.

PR #76 tip `2d9679f` refreshes this consequence on top of #72 tip `f16f0bb`. The flip guard on that tip also locks #74 tip `30fd669` into `ac2_ac3_cannot_flip_while_stop_shas_stand`. #74 owns `metal_subprocess_stop_cites_55_63_and_70`. That tip cites the Stop and does not copy that guard. The locks at `0829a70`, `232ea78`, `e06ea0e`, and `2ff5c4f` stay. Checking either box above fails on that tip while `30fd669` stands. This branch does not replay that charter guard. No new Mac run. No Metal measurement. Neither box above is a pass.

PR #77 tip `595dced` does not change the AC2/AC3 Stop-SHA list. It does not close either box above.

PR #78 tip `9a1d8f1` cites #73 through #77. Those cites do not check a box.

PR #79 tip `d04c224` does not change the AC2/AC3 Stop-SHA list. It does not close either box above.

PR #80 tip `9def38b` does not change the AC2/AC3 Stop-SHA list. It does not close either box above.

PR #81 tip `ca9bc50` does not change the AC2/AC3 Stop-SHA list. #51 tip `7c71d50` and #62 tip `8c24c99` stay. It does not close either box above.

PR #82 tip `637459f` does not change the AC2/AC3 Stop-SHA list. It does not close either box above.

PR #83 tip `86dd55d` cites #78 through #82. Those cites do not check a box.

PR #84 tip `fb48acc` does not change the AC2/AC3 Stop-SHA list. It does not close either box above.

PR #85 tip `576ac63` freezes both boxes as Partial, stacked on #76 tip `2d9679f`. It complements `ac2_ac3_cannot_flip_while_stop_shas_stand` and does not replace it. The note on that tip is `docs/evidence/m0/ac2-ac3-partial.md`. Status: Partial. The guard on that tip is `m0_ac2_ac3_partial_refuses_checked_boxes`. ORT CoreML `LSTM` and `If`, and a Metal device process-lifetime release, stay out of M0. This branch does not replay that guard and does not copy that note. Neither box above is a pass.

PR #86 tip `8938f5b` cites #83 through #85. Those cites do not check a box.

PR #87 tip `a40fad1` does not change the AC2/AC3 Stop-SHA list. It does not close either box above.

PR #88 tip `2766f79` does not change the AC2/AC3 Stop-SHA list. It does not close either box above.
