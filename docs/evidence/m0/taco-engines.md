# taco binary with static engines (2026-10-17)

Release `taco` on `cursor/m0-taco-engines-b19b`, built `--features native`
with the metal-worktree checkouts. This binary is `taco`, not the link probe.
M0-AC2 and M0-AC3 stay open. The two native cargo tests below did pass on
this Mac. The process printed `ac2=0 ac3=0`. A root completion audit still
has to decide whether that run meets the acceptance text.

## Host

| Field | Value |
| --- | --- |
| Chip | Apple M1 Pro |
| OS | macOS 27.0.1, build 26A434 |
| Arch | arm64 |
| rustc | 1.99.0 (`b940084d7`, 2026-09-28) |
| When | 2026-10-17 01:08 KST |

## Build

`TACO_LLAMA_SRC` and `TACO_WHISPER_SRC` pointed at
`~/workspace/syndai/taco-m0-metal/.cache/m0-engines`. `TACO_ORT_ROOT` pointed
at `~/workspace/syndai/taco-m0-metal/.cache/m0-onnx` (ONNX Runtime 1.31.0 CPU
static archives). `TACO_MODEL_DIR` pointed at
`~/workspace/syndai/taco-m0-pins-cache`. `cargo build --release --features native`
exited 0. The product is one Mach-O, 38261776 bytes.

```text
$ file target/release/taco
target/release/taco: Mach-O 64-bit executable arm64
```

No `.dylib`, `.so`, `.metal`, or `.metallib` sits next to that executable.
The linker warned that engine objects were built for macOS 14.2 while the
Rust driver linked at 11.0. The link still exited 0.

`session::open_inference` calls `engines::native_entrypoints`, which takes
the addresses of `init_role`, `infer_role`, `cancel_role`, `close_role`, and
`prove`. That keeps those C entry points in the Mach-O under `-dead_strip`.

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

Defined text symbols in this `taco` (`nm -g`):

```text
_OrtGetApiBase
_taco_engines_cancel
_taco_engines_close
_taco_engines_infer
_taco_engines_init
_taco_engines_linked
_taco_engines_prove
taco_engines_ffi::cancel_role
taco_engines_ffi::close_role
taco_engines_ffi::entrypoints
taco_engines_ffi::gpu_offload_linked
taco_engines_ffi::infer_role
taco_engines_ffi::init_role
taco_engines_ffi::prove
```

`_OrtGetApiBase` is defined, so the static ORT archives are inside the
Mach-O. Unbound `initialize`, `infer`, and `cancel` still return
`engine_not_linked`.

## Native tests through taco

Command, with the four environment variables above:

```text
cargo test --release --features native \
  --test t_m0_link_three_models --test t_m0_cancel_lifecycle -- --nocapture
```

Exit 0 at 2026-10-17 01:08 KST. Both tests call `session::prove_native`,
which calls `engines::prove_pins` and then `taco_engines_prove` inside the
test binary. They do not build or run `taco_link_probe`. They also do not
invoke `taco run`. Metal init printed `using embedded metal library`.

Binaries:

- `target/release/deps/t_m0_cancel_lifecycle-679034a0b74c66ed`
- `target/release/deps/t_m0_link_three_models-77e464305d2fa46a`

Pinned digests matched on both runs:

| Role | SHA-256 |
| --- | --- |
| vad `silero_vad.onnx` | `1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3` |
| asr `ggml-small.bin` | `1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b` |
| mt `Hy-MT2-1.8B-Q4_K_M.gguf` | `dc5f44fcf1fa496ee7ad725982c0c8c553a4de00259b53af84c4b89fb0c06699` |

Shared lines on both runs: `vad=fail` (whisper VAD, bad magic on the ONNX
pin), `vad_ort_session=ok version=1.31.0 inputs=3 outputs=2 ep=CPU`,
three 512-sample frames at probabilities 0.000592 / 0.000528 / 0.000592
with state L2 8.108584 / 11.574153 / 8.108584, `vad_ort_reset=1 match=1`,
`vad_ort=ok device=cpu`, `gpu_offload=1`, `gpu_device=MTL0`,
`asr=ok use_gpu=1 segments=1 text_bytes=21`,
`mt_load=0 n_gpu_layers=-1 offloaded=33/33`, `mt=ok text_bytes=34`,
`link_three=1 cancel_lifecycle=1 ac2=0 ac3=0`, `teardown=returned`.
`llama_decode` returned 2 on the aborted decode. That abort is the cancel.

Cancel test, 8.39s, `test result: ok`:

| Join | ms | reserve |
| --- | --- | --- |
| whisper decode | 166 | 2000, within |
| whisper init | 112 | 2000, within |
| mt init | 240 | 2000, within |
| mt decode | 9 | 2000, within |

Link test, 2.18s, shaders already warm, `test result: ok`:

| Join | ms | reserve |
| --- | --- | --- |
| whisper decode | 167 | 2000, within |
| whisper init | 56 | 2000, within |
| mt init | 233 | 2000, within |
| mt decode | 7 | 2000, within |

Each cargo test called `prove_native` once. No multi-cycle residency
measurement was taken.

## What this does not accept

- [ ] **M0-AC2:** ASR and MT ran in process on MTL0 with the pinned files, and Silero ran through static ORT. The VAD execution provider was CPU. The cargo tests are not `taco run`. Residency across repeated init and teardown was not measured. The process printed `ac2=0`.
- [ ] **M0-AC3:** Init and decode cancels joined inside 2000 ms and `teardown=returned` on one pass each. The catalog's late-completion and replacement-worker checks, and the residency return to baseline, were not recorded here. The process printed `ac3=0`.

Default `cargo test` (no `native` feature) passed on this host after the
entrypoint change. Linux still refuses the native feature: those tests panic
off macOS Apple Silicon. No TCC dialog was used.
