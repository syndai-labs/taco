# taco binary with static engines (2026-10-17)

Cited from `cursor/m0-taco-engines-b19b` tip `de6d075` (PR #23). This branch does not relink. No acceptance box is checked.

Release `taco` on that tip, built `--features native` with the metal-worktree checkouts. This binary is `taco`, not the link probe. M0-AC2 and M0-AC3 stay open. `T-M0-LINK-THREE-MODELS` and `T-M0-CANCEL-LIFECYCLE` were not run. Both still fail closed: off macOS Apple Silicon they panic, and on that host they panic unless `prove_native` reports the predicate. That run was not waited on.

Host: Apple M1 Pro, macOS 27.0.1 build 26A434, arm64, rustc 1.99.0 (`b940084d7`, 2026-09-28), 2026-10-17 01:04 KST.

`TACO_ORT_ROOT` pointed at ONNX Runtime 1.31.0 CPU static archives. `cargo build --release --features native` exited 0. The product is one Mach-O, 38200496 bytes, `Mach-O 64-bit executable arm64`. No `.dylib`, `.so`, `.metal`, or `.metallib` sits next to that executable. The linker warned that engine objects were built for macOS 14.2 while the Rust driver linked at 11.0. The link still exited 0.

`otool -L` is system libraries only. No onnxruntime, whisper, llama, or ggml dynamic library. The listed dependencies are system frameworks (Accelerate, Foundation, Metal, MetalKit, CoreFoundation) and `/usr/lib` system libraries (`libc++`, `libobjc`, `libSystem`).

Defined text symbols in this `taco`: `_OrtGetApiBase`, `_taco_engines_init`, `_taco_engines_linked`. `libtaco_engines.a` also defines `_taco_engines_infer`, `_taco_engines_cancel`, `_taco_engines_close`, and `_taco_engines_prove`. This release `taco` does not. `main` calls `initialize` only when `--model-dir` is set, and the linker's dead strip dropped the unreferenced infer, cancel, and prove entry points. `_OrtGetApiBase` is defined, so the static ORT archives are inside the Mach-O. Unbound `initialize`, `infer`, and `cancel` still return `engine_not_linked`.

Silero's ORT provider is CPU. ASR and translation were not run in this binary. The three-pin test was not run. No worker was joined. The cancel test was not run.

- [ ] **M0-AC2:** ASR and translation were not run in this binary. Silero's ORT provider is CPU. The three-pin test was not run.
- [ ] **M0-AC3:** No worker was joined. The cancel test was not run.
