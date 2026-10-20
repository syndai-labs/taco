# M0-AC2 and M0-AC3 stay unchecked

Cited from `cursor/m0-ac23-honesty2-b19b` tip `7ae6d98` (PR #35). This branch does not replay that note. No acceptance box is checked.

(a) Still open. Silero is the ORT CPU execution provider (`device=cpu` on the #30 `taco run`, `ep=CPU` on the #32 residency series).

(b) Closed for prove reachable from `taco run`. #30 tip `e2ec751` printed `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0` from an ad-hoc signed Mac `taco run` and exited 0. The signature is ad-hoc, not Developer ID. `ac2` and `ac3` stayed 0.

(c) Still blocks M0-AC2 and M0-AC3. #32 tip `a0e1614` is a Stop, not a fix. The 100-cycle `phys_footprint` delta is 252346632 bytes against a 33554432-byte tolerance. `whisper_free` retains the bytes. A Metal command-buffer flush missed the band (6-cycle delta 98812072) and was reverted. The earlier cite `0972b2d` recorded the same 100-cycle miss.

- [ ] **M0-AC2:** CPU VAD is open. Prove from `taco run` does not close this box. #57 tip `88eda06` does not close it. #59 tip `ed9ee10` does not close it.
- [ ] **M0-AC3:** The residency Stop at `a0e1614` is not a bounded teardown. #55 tip `0829a70` does not close it. #59 tip `ed9ee10` does not close it.

PR #41 tip `a8e2eca` freezes that consequence. This branch does not replay the charter guard. No new Metal or macOS evidence.

Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. Partial is not an accept. Checking either box requires [vad-metal-ep.md](vad-metal-ep.md) to drop `d194012` and [engines-residency.md](engines-residency.md) to drop `a0e1614`. R2 and R3 name the CPU VAD Stop. R5 names the residency Stop. Neither box above is a pass.

PR #49 tip `37ff031` ([engines-residency-retry.md](engines-residency-retry.md)) is a stronger Stop. `vm_deallocate` returned the 487014400-byte weight buffer and cycle 2 stayed 94699688 bytes above cycle 1. It does not drop `a0e1614`. The synchronize and the blit are not in this tree. No acceptance box is checked.

PR #55 tip `0829a70` ([engines-residency-rootcause.md](engines-residency-rootcause.md)) names the leftover mapping as the process-lifetime Metal device in `ggml_backend_metal_reg`. `llama_backend_free` does not destroy it. No purge. It does not drop `a0e1614`.

PR #57 tip `88eda06` ([vad-coreml-subgraph.md](vad-coreml-subgraph.md)) requests `EnableOnSubgraphs`. ORT 1.31 `GetOpBuilders` lacks `LSTM` and `If`. `device=` stays `cpu`. It does not close M0-AC2. No `device=metal`.

PR #58 tip `fb4246e` cites #55, #56, and #57. Those cites do not check a box.

PR #59 tip `ed9ee10` refreshes this consequence on top of #41 tip `a8e2eca`. The flip guard on that tip also locks `0829a70` and `88eda06`. Prior Stops stay cited: #37 tip `d194012`, #32 tip `a0e1614`, #49 tip `37ff031`, and #41 tip `a8e2eca`. Checking either box above fails on that tip while those SHAs stand. This branch does not replay that charter guard. No new Mac run. No Metal measurement. Neither box above is a pass.
