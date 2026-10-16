# `taco run` prove gate

Cited from `cursor/m0-taco-run-prove-b19b` tip `e2ec751` (PR #30). The Mac binary was the `af2af02` gate. This branch does not replay `src/main.rs`. No acceptance box is checked.

`taco run` calls `session::prove_native` only when `TACO_ENGINES_PROVE=1`. On `Ok` it prints `link_three=<n> cancel_lifecycle=<n> ac2=0 ac3=0`. Any other value keeps the capture preflight. A missing model directory, an unbound build, and a non-macOS build refuse with `engine_not_linked`.

Linux `cargo test` on `af2af02` is fail-closed: `TACO_ENGINES_PROVE=1` exits 1 with `engine_not_linked` and no `link_three=` line. Unset and `=0` print `platform_unsupported`.

On this Mac, tip `e2ec751` records an ad-hoc signed release `taco` (`Signature=adhoc`, `TeamIdentifier=not set`, not Apple Development or Developer ID). `TACO_ENGINES_PROVE=1` exited 0 in 9.35 s and printed `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`. Silero's provider was CPU (`vad_ort=ok device=cpu`). Whisper VAD failed on the ONNX pin. ASR and MT used MTL0. Joins were inside 2000 ms. One `prove_native` call. That run did not measure catalog residency.

- [ ] **M0-AC2:** The ad-hoc `taco run` printed `ac2=0`. Silero's provider is CPU.
- [ ] **M0-AC3:** One pass joined inside 2000 ms and printed `ac3=0`. The 100-cycle footprint is PR #32 and is over tolerance.
