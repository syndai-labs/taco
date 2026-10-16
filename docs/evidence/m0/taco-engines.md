# taco binary with static engines (2026-10-17)

Cited from `cursor/m0-taco-engines-b19b` tip `7f98ab1` (PR #23). This branch does not relink. No acceptance box is checked.

Release `taco` on that tip, built `--features native` with the metal-worktree checkouts. This binary is `taco`, not the link probe. `session::open_inference` names init, infer, cancel, close, and prove so dead-strip keeps them. M0-AC2 and M0-AC3 stay open. The two native cargo tests below passed on that Mac through `session::prove_native`. The process printed `link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`.

Host: Apple M1 Pro, macOS 27.0.1 build 26A434, arm64, rustc 1.99.0 (`b940084d7`, 2026-09-28), 2026-10-17 01:08 KST.

`TACO_ORT_ROOT` pointed at ONNX Runtime 1.31.0 CPU static archives. `cargo build --release --features native` exited 0. The product is one Mach-O, 38261776 bytes, `Mach-O 64-bit executable arm64`. No `.dylib`, `.so`, `.metal`, or `.metallib` sits next to that executable. The linker warned that engine objects were built for macOS 14.2 while the Rust driver linked at 11.0. The link still exited 0.

`otool -L` is system libraries only. No onnxruntime, whisper, llama, or ggml dynamic library. The listed dependencies are system frameworks (Accelerate, Foundation, Metal, MetalKit, CoreFoundation) and `/usr/lib` system libraries (`libc++`, `libobjc`, `libSystem`).

Defined text symbols in this `taco` include `_OrtGetApiBase`, `_taco_engines_init`, `_taco_engines_infer`, `_taco_engines_cancel`, `_taco_engines_close`, `_taco_engines_prove`, and `_taco_engines_linked`. Unbound `initialize`, `infer`, and `cancel` still return `engine_not_linked`.

`cargo test --release --features native --test t_m0_link_three_models --test t_m0_cancel_lifecycle` exited 0. Both tests call `session::prove_native`, then `taco_engines_prove` inside the test binary. They do not run `taco_link_probe` or `taco run`. Whisper VAD failed with bad magic on the ONNX pin. Silero then ran on static ORT CPU (`vad_ort=ok device=cpu`). ASR and MT used Metal (`gpu_device=MTL0`, `offloaded=33/33`). Joins were inside 2000 ms and the process printed `teardown=returned`. Each test called `prove_native` once. No multi-cycle residency measurement was taken. Off macOS Apple Silicon those tests still panic.

- [ ] **M0-AC2:** The cargo tests loaded the pins and printed `ac2=0`. Silero's ORT provider is CPU. The tests are not `taco run`. Residency across repeated init and teardown was not measured.
- [ ] **M0-AC3:** Joins fit in 2 s on one pass each and the process printed `ac3=0`. Late completion, a replacement worker, and residency return were not recorded.
