# M0-AC2 and M0-AC3 stay unchecked

Cited from `cursor/m0-ac23-honesty2-b19b` tip `7ae6d98` (PR #35). This branch does not replay that note. No acceptance box is checked.

(a) Still open. Silero is the ORT CPU execution provider (`device=cpu` on the #30 `taco run`, `ep=CPU` on the #32 residency series).

(b) Closed for prove reachable from `taco run`. #30 tip `e2ec751` printed `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0` from an ad-hoc signed Mac `taco run` and exited 0. The signature is ad-hoc, not Developer ID. `ac2` and `ac3` stayed 0.

(c) Still blocks M0-AC2 and M0-AC3. #32 tip `a0e1614` is a Stop, not a fix. The 100-cycle `phys_footprint` delta is 252346632 bytes against a 33554432-byte tolerance. `whisper_free` retains the bytes. A Metal command-buffer flush missed the band (6-cycle delta 98812072) and was reverted. The earlier cite `0972b2d` recorded the same 100-cycle miss.

- [ ] **M0-AC2:** CPU VAD is open. Prove from `taco run` does not close this box. #57 tip `88eda06` does not close it.
- [ ] **M0-AC3:** The residency Stop at `a0e1614` is not a bounded teardown. #55 tip `0829a70` does not close it.

Charter accept is blocked: M0-AC2 and M0-AC3 cannot be checked while the CPU VAD Stop at `d194012` and the residency Stop at `a0e1614` stand. A charter waiver records weight Partial on R2, R3, and R5. Partial is not an accept. Checking either box requires [vad-metal-ep.md](vad-metal-ep.md) to drop `d194012` and [engines-residency.md](engines-residency.md) to drop `a0e1614`. R2 and R3 name the CPU VAD Stop. R5 names the residency Stop. Neither box above is a pass.

Newer Stops still stand: #55 tip `0829a70` (residual phys_footprint is the process-lifetime Metal device in `ggml_backend_metal_reg`) and #57 tip `88eda06` (`EnableOnSubgraphs`; ORT 1.31 `GetOpBuilders` lacks `LSTM` and `If`; `device=` stays `cpu`). Prior Stops stay cited: #49 tip `37ff031` and #41 tip `a8e2eca`.

#49 tip `37ff031` freed the 487014400-byte weight buffer and cycle 2 stayed 94699688 bytes above cycle 1. It does not drop `a0e1614`. #41 tip `a8e2eca` is the prior consequence freeze. #55 tip `0829a70` names the leftover mapping as the process-lifetime device `ggml_backend_metal_reg` installs; `llama_backend_free` does not destroy it. No purge is in this tree. #57 tip `88eda06` sets provider option `EnableOnSubgraphs` on the product session. ORT 1.31.0 `GetOpBuilders` has no `LSTM` builder and no `If` builder, so `device=` stays `cpu`. This branch does not replay those Mac runs and adds no Metal measurement.
