# taco binary with static engines (2026-10-17)

Release `taco` on `cursor/m0-taco-engines-b19b`, built `--features native`
with the metal-worktree checkouts. This binary is `taco`, not the link probe.
M0-AC2 and M0-AC3 stay open. `T-M0-LINK-THREE-MODELS` and
`T-M0-CANCEL-LIFECYCLE` were not run. Both still fail closed: off macOS
Apple Silicon they panic, and on this host they panic unless `prove_native`
reports the predicate. That run was not waited on.

## Host

| Field | Value |
| --- | --- |
| Chip | Apple M1 Pro |
| OS | macOS 27.0.1, build 26A434 |
| Arch | arm64 |
| rustc | 1.99.0 (`b940084d7`, 2026-09-28) |
| When | 2026-10-17 01:04 KST |

## Build

`TACO_LLAMA_SRC` and `TACO_WHISPER_SRC` pointed at
`~/workspace/syndai/taco-m0-metal/.cache/m0-engines`. `TACO_ORT_ROOT` pointed
at `~/workspace/syndai/taco-m0-metal/.cache/m0-onnx` (ONNX Runtime 1.31.0 CPU
static archives). `cargo build --release --features native` exited 0.
The product is one Mach-O, 38200496 bytes.

```text
$ file target/release/taco
target/release/taco: Mach-O 64-bit executable arm64
```

No `.dylib`, `.so`, `.metal`, or `.metallib` sits next to that executable.
The linker warned that engine objects were built for macOS 14.2 while the
Rust driver linked at 11.0. The link still exited 0.

## otool

System libraries only. No onnxruntime, whisper, llama, or ggml dynamic library.

```text
$ otool -L target/release/taco
	/System/Library/Frameworks/Accelerate.framework/Versions/A/Accelerate (compatibility version 1.0.0, current version 4.0.0)
	/System/Library/Frameworks/Foundation.framework/Versions/C/Foundation (compatibility version 300.0.0, current version 5027.0.69)
	/System/Library/Frameworks/Metal.framework/Versions/A/Metal (compatibility version 1.0.0, current version 382.5.3)
	/System/Library/Frameworks/MetalKit.framework/Versions/A/MetalKit (compatibility version 1.0.0, current version 174.4.0)
	/System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation (compatibility version 150.0.0, current version 5027.0.69)
	/usr/lib/libc++.1.dylib (compatibility version 1.0.0, current version 2200.27.0)
	/usr/lib/libobjc.A.dylib (compatibility version 1.0.0, current version 228.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1359.0.0)
```

## Symbols

Defined text symbols in this `taco`:

```text
_OrtGetApiBase
_taco_engines_init
_taco_engines_linked
taco_engines_ffi::gpu_offload_linked
taco_engines_ffi::init_role
```

`libtaco_engines.a` also defines `_taco_engines_infer`, `_taco_engines_cancel`,
`_taco_engines_close`, and `_taco_engines_prove`. This release `taco` does not.
`main` calls `initialize` only when `--model-dir` is set, and the linker's
dead strip dropped the unreferenced infer, cancel, and prove entry points.
`_OrtGetApiBase` is defined, so the static ORT archives are inside the
Mach-O. Unbound `initialize`, `infer`, and `cancel` still return
`engine_not_linked`.

## What this does not accept

- [ ] **M0-AC2:** ASR and translation were not run in this binary. Silero's ORT provider is CPU. The three-pin test was not run.
- [ ] **M0-AC3:** No worker was joined. The cancel test was not run.

Default `cargo test` (no `native` feature) passed on this host before this
note was added. Linux still refuses the native feature: those tests panic
off macOS Apple Silicon.
