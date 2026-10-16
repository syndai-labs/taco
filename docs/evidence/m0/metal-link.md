# M0 Metal link probe (2026-10-16)

Measured in worktree `~/workspace/syndai/taco-m0-metal` on
`cursor/m0-metal-link-b19b`, parent `5e9649c` (`cursor/m0-pin-apply-b19b`).
One static Mach-O links whisper.cpp and llama.cpp on llama.cpp's ggml with
Metal shaders embedded. A later run of that probe opened the three pinned
files. Whisper small and Hy-MT2 loaded on MTL0. The pinned Silero ONNX was
rejected. Llama init and decode cancellation joined inside 2 s. M0-AC2 and
M0-AC3 stay open. This binary is not `taco`.

## Host

| Field | Value |
| --- | --- |
| Chip | Apple M1 Pro |
| Memory | 34359738368 bytes (32 GiB) |
| OS | macOS 27.0.1, build 26A434 |
| Arch | arm64 |
| rustc | 1.99.0 (`b940084d7`, 2026-09-28) |
| clang | Apple clang 21.0.0 (`clang-2100.3.34.2`) |
| cmake | 4.4.4 |
| When | 2026-10-17 00:02 KST |

This machine is not C1 and it is not a named C2.

## Engine commits

Shallow checkouts. Not vendored in git. `/.cache/` stays ignored.

| Tree | Commit | Note |
| --- | --- | --- |
| llama.cpp | `2a667032dc788c3a071d538e0b572e45a3f12731` | 2026-10-16, `2a66703`, ggml 0.27.0. cmake reports `llama.cpp version: 0.7.0-dev`. `src/models/hunyuan-dense.cpp` is in this tree. |
| whisper.cpp | `0c7be696371069e0baa2515ed7aa08d5a5473aa6` | 2026-10-12, `0c7be69`. cmake reports `whisper.cpp version: 1.9.5-dev`. Vendored ggml is 0.26.0 and was not compiled. |

Publisher card rechecked 2026-10-16 still says the GGUF depends on llama.cpp
PR 22836. `gh api repos/ggml-org/llama.cpp/pulls/22836` returned
`state=open`, `merged=false`, `merge_commit_sha=null`, head
`1e411d8f5a1e23525fa3265dfb4bd76265465397`, title
`ggml-cpu : add STQ1_0 ternary quantization with ARM NEON vec_dot kernel`.
That pull request is a CPU NEON kernel. It is not merged and it is not a Metal
backend.

The pinned `Hy-MT2-1.8B-Q4_K_M.gguf` header (GGUF v3, 354 tensors) has
`general.architecture=hunyuan-dense`, `general.file_type=15`, and tensor types
F32 (129), Q4_K (192), and Q6_K (33). No other type id is present. The file
was not loaded, so `2a66703` is a candidate that contains hunyuan-dense and
those quant types. It is not a demonstrated load of this GGUF.

Digests stay the ones in `pin-measurement.md`. A fresh `shasum -a 256` of
`~/workspace/syndai/taco-m0-pins-cache` matched those three lines. The probe
does not open the files.

## Build

`native/CMakeLists.txt` adds llama.cpp first, so the `ggml` target exists, then
adds whisper.cpp, which skips its own ggml. `BUILD_SHARED_LIBS=OFF`.
`GGML_METAL=ON` and `GGML_METAL_EMBED_LIBRARY=ON`. Examples, tools, server, and
app targets are off. Deployment target `14.2`.

```text
cmake -S native -B .cache/m0-link/build -DCMAKE_BUILD_TYPE=Release \
  -DLLAMA_SRC=$PWD/.cache/m0-engines/llama.cpp \
  -DWHISPER_SRC=$PWD/.cache/m0-engines/whisper.cpp
cmake --build .cache/m0-link/build -j8
```

Configure printed `ggml version: 0.27.0`, `ggml commit: 2a66703`,
`Including METAL backend`, then `whisper.cpp version: 1.9.5-dev`, then
`Configuring done`. The build log ends at `Built target taco_link_probe` and
`EXIT:0`. One `ggml-metal` static library is linked. There is no second ggml.

## Link-only probe

The first `taco_link_probe` only called `llama_supports_gpu_offload`. That run
did not open a model file.

```text
$ file .cache/m0-link/build/taco_link_probe
.cache/m0-link/build/taco_link_probe: Mach-O 64-bit executable arm64

$ otool -L .cache/m0-link/build/taco_link_probe
	/System/Library/Frameworks/Accelerate.framework/Versions/A/Accelerate
	/usr/lib/libSystem.B.dylib
	/System/Library/Frameworks/Foundation.framework/Versions/C/Foundation
	/System/Library/Frameworks/Metal.framework/Versions/A/Metal
	/System/Library/Frameworks/MetalKit.framework/Versions/A/MetalKit
	/usr/lib/libc++.1.dylib
	/System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
	/usr/lib/libobjc.A.dylib
```

No `.dylib`, `.so`, `.metal`, or `.metallib` sits next to that executable.
The first run compiled Metal libraries from embedded data in 4.390 sec. A
rerun of the same binary printed:

```text
ggml_metal_library_init: using embedded metal library
ggml_metal_library_compile_all: loaded 36 libraries from embedded data in 0.035 sec
ggml_metal_device_init: GPU name:   MTL0 (Apple M1 Pro)
ggml_metal_device_init: GPU family: MTLGPUFamilyApple7  (1007)
ggml_metal_device_init: has unified memory    = true
ggml_metal_device_init: recommendedMaxWorkingSetSize  = 26800.60 MB
taco_engines_linked gpu_offload=1
```

Exit status 0 on that first binary. `gpu_offload=1` there means the binary was
built with GPU offload. The load run below is the one that opened the pins.

## Pinned load (2026-10-17)

Same Mach-O, after `taco_link_probe` learned to take the three paths. Digests
were hashed in process with CommonCrypto before any load. `n_gpu_layers` was
`-1` (every layer). No `whisper-cli`, no `llama-completion`, no `-ngl 0`.

```text
$ .cache/m0-link/build/taco_link_probe \
    ~/workspace/syndai/taco-m0-pins-cache/silero_vad.onnx \
    ~/workspace/syndai/taco-m0-pins-cache/ggml-small.bin \
    ~/workspace/syndai/taco-m0-pins-cache/Hy-MT2-1.8B-Q4_K_M.gguf
digest role=vad ok=1 sha256=1a153a22f4509e292a94e67d6f9b85e8deb25b4988682b7e174c65279d8788e3
digest role=asr ok=1 sha256=1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b
digest role=mt ok=1 sha256=dc5f44fcf1fa496ee7ad725982c0c8c553a4de00259b53af84c4b89fb0c06699
vad=fail
gpu_offload=1
gpu_device=MTL0
asr=ok use_gpu=1 segments=1 text_bytes=21
cancel_init=aborted join_ms=222 progress_milli=0 reserve_ms=2000 within_reserve=1
mt_load=0 n_gpu_layers=-1 offloaded=33/33
mt=ok text_bytes=34
cancel_decode=aborted rc=2 steps=2 join_ms=9 reserve_ms=2000 within_reserve=1
link_three=0 cancel_lifecycle=1 ac2=0 ac3=0
result=incomplete
```

Exit status 1. Stderr lines from that process:

```text
whisper_vad_init_with_params: invalid model data (bad magic)
whisper_init_with_params_no_state: use gpu    = 1
whisper_model_load: type          = 3 (small)
whisper_model_load:         MTL0 total size =   487.01 MB
whisper_backend_init_gpu: using MTL0 backend
llama_model_loader: - kv   0:                       general.architecture str              = hunyuan-dense
load_tensors: offloaded 33/33 layers to GPU
load_tensors:   CPU_Mapped model buffer size =   193.57 MiB
load_tensors:  MTL0_Mapped model buffer size =  1075.74 MiB
graph_compute: ggml_backend_sched_graph_compute_async failed with error 1
```

`ggml-small.bin` is Whisper small. The weight buffer is on MTL0.
`whisper_full` on one second of a 440 Hz tone returned 0 (`translate` off,
language `en`, VAD inside that call off). `ggml_metal_free` follows
`whisper_free`.

`Hy-MT2-1.8B-Q4_K_M.gguf` loads as `hunyuan-dense`. The kept context reports
33/33 layers on GPU. 1075.74 MiB is `MTL0_Mapped` and 193.57 MiB is
`CPU_Mapped`. A greedy 8-token smoke produced 34 bytes. That sampler is not an
R4 selection. The file did not need the unmerged CPU STQ pull request to load
on this llama.cpp commit.

The pinned `silero_vad.onnx` matches the freeze digest and whisper.cpp rejects
it: bad magic. That loader expects a ggml VAD file. No other VAD file was
substituted. `link_three=0` because of that.

Cancellation, same process, reserve 2000 ms:

- Init: the worker calls `llama_model_load_from_file` with `n_gpu_layers=-1`.
  The main thread sets the abort flag once the worker has started. The first
  progress checkpoint is 0 and returns false. The load returns null. Join is
  222 ms. Layer-assignment logs still print `offloaded 33/33` before that
  checkpoint; the model pointer is not kept.
- Decode: after the kept model has already decoded, a second worker samples and
  calls `llama_decode`. Abort is set after the first token. `llama_decode`
  returns 2. Join is 9 ms. Stderr reports
  `ggml_backend_sched_graph_compute_async failed with error 1`.

`cancel_lifecycle=1` is only that llama pair. Whisper was not cancelled.
`T-M0-CANCEL-LIFECYCLE` is still the Rust panic. The probe is not `taco`.

## What this does not accept

- [ ] **M0-AC2:** Silero ONNX did not load. ASR and translation ran in the probe, not in `taco`.
- [ ] **M0-AC3:** Llama init and decode joined inside the reserve. Whisper did not. The Rust test did not pass.
- The `taco` binary from `cargo build` does not link these engines. `cargo test` does not build this probe.
- `T-M0-LINK-THREE-MODELS` and `T-M0-CANCEL-LIFECYCLE` stay fail-closed.
- A helper process, a sidecar dylib, and `-ngl 0` were not used, and they remain rejected.
