# M0 Metal link probe (2026-10-16)

Measured in worktree `~/workspace/syndai/taco-m0-metal` on
`cursor/m0-metal-link-b19b`, parent `5e9649c` (`cursor/m0-pin-apply-b19b`).
One static Mach-O links whisper.cpp and llama.cpp on llama.cpp's ggml with
Metal shaders embedded, and now also links ONNX Runtime 1.31.0 CPU archives.
The latest run hashed the three pins in process. Whisper small and Hy-MT2
loaded on MTL0. The pinned Silero ONNX still fails whisper.cpp's VAD magic
check. The same process then ran it through static ORT: incremental 512-sample
frames, carried state, and a reset that matched. `otool -L` stays on system
frameworks. The probe printed `link_three=1`, `teardown=returned`, and exited 0.
Silero's execution provider is CPU. This binary is not `taco`.
M0-AC2 and M0-AC3 stay open.

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
F32 (129), Q4_K (192), and Q6_K (33). No other type id is present. A later section records this file loading on
`2a66703`. The load did not need the unmerged STQ pull request.

Digests stay the ones in `pin-measurement.md`. A fresh `shasum -a 256` of
`~/workspace/syndai/taco-m0-pins-cache` matched those three lines. Later runs
of the probe open the three files and hash them again with CommonCrypto.

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

The pinned `silero_vad.onnx` matches the freeze digest. The probe calls
`whisper_vad_init_from_file_with_params`, and `whisper_vad_init_with_params`
rejects the file at the ggml magic check (`invalid model data (bad magic)`).
That is the VAD API, not `whisper_init` on the ONNX. The frame loop
(`whisper_vad_detect_speech_no_reset`) never runs. No other VAD file was
substituted. `whisper_full` in the ASR pass keeps `vad` false. `link_three=0`
because of that.

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

`cancel_lifecycle=1` on that run is only the llama pair. The next section is the Whisper cancel run.

## Whisper cancel (2026-10-17)

Same probe, after it learned Whisper init and decode cancel. Digests matched
again. ASR still used `vad=false`. Decode cancel is a second `whisper_full` on
the context that just transcribed. Init cancel is a later
`whisper_init_with_params` through a loader that, once abort is set and the
read is past 4 MiB, supplies `99` for the next 4-byte weight header.
`whisper_init_from_file_with_params` has no abort callback, and `read`'s
return count is ignored, so this is the checkpoint the loader can force.
There is no wrapper timeout and no `pthread_cancel`.

```text
vad=fail
gpu_offload=1
gpu_device=MTL0
asr=ok use_gpu=1 segments=1 text_bytes=21
whisper_cancel_decode=aborted rc=-8 hits=2 join_ms=204 reserve_ms=2000 within_reserve=1
whisper_cancel_init=aborted join_ms=115 poisoned=1 reserve_ms=2000 within_reserve=1
cancel_init=aborted join_ms=312 progress_milli=0 reserve_ms=2000 within_reserve=1
mt_load=0 n_gpu_layers=-1 offloaded=33/33
mt=ok text_bytes=34
cancel_decode=aborted rc=2 steps=2 join_ms=10 reserve_ms=2000 within_reserve=1
link_three=0 cancel_lifecycle=1 ac2=0 ac3=0
result=incomplete
```

The shell status after that process was 134, not 1. Stderr for the new
checkpoints:

```text
whisper_vad_init_from_file_with_params: loading VAD model from '.../silero_vad.onnx'
whisper_vad_init_with_params: invalid model data (bad magic)
whisper_full_with_state: failed to decode
whisper_model_load:         MTL0 total size =   487.01 MB
whisper_model_load: invalid n_dims 99 in model file (expected 0 <= n_dims <= 4)
whisper_init_with_params_no_state: failed to load model
ggml_metal_device.m:1041: GGML_ASSERT([rsets->data count] == 0) failed
```

Decode: `encoder_begin` marks the worker inside `whisper_full`. The abort flag
is set then. The callback's first hit is the end of the encoder graph and
returns false, so the call continues. The second hit is the end of the first
decoder graph and returns true. `whisper_full` returns -8. Join is 204 ms.
The callback is not consulted in the middle of a Metal graph; the in-flight
graph runs to that checkpoint.

Init: `use gpu = 1`. The load prints `MTL0 total size = 487.01 MB` and then
`invalid n_dims 99`. The context pointer is null. Join is 115 ms.
`whisper_init_with_params_no_state` in whisper.cpp `0c7be69` used `delete`
on that context. `whisper_free` is what releases `model.buffers`. The probe
build applies `native/patches/whisper-free-buffers-on-init-fail.patch` so the
failure path calls `whisper_free`. The four joins above are from the run
before that patch. Process exit on that run was 134:
`GGML_ASSERT([rsets->data count] == 0)` in `ggml_metal_device_free`.
`cancel_lifecycle=1` was those joins, not a released-ownership result.

## Teardown after whisper_free (2026-10-17)

Same probe, rebuilt with the patch. Joins stayed inside the reserve. Stdout
ended with `teardown=returned` and the shell status was 1. Stderr has no
`GGML_ASSERT` and no `ggml_abort`.

```text
vad=fail
gpu_offload=1
gpu_device=MTL0
asr=ok use_gpu=1 segments=1 text_bytes=21
whisper_cancel_decode=aborted rc=-8 hits=2 join_ms=177 reserve_ms=2000 within_reserve=1
whisper_cancel_init=aborted join_ms=102 poisoned=1 reserve_ms=2000 within_reserve=1
cancel_init=aborted join_ms=219 progress_milli=0 reserve_ms=2000 within_reserve=1
mt_load=0 n_gpu_layers=-1 offloaded=33/33
mt=ok text_bytes=34
cancel_decode=aborted rc=2 steps=2 join_ms=10 reserve_ms=2000 within_reserve=1
link_three=0 cancel_lifecycle=1 ac2=0 ac3=0
result=incomplete
teardown=returned
```

Exit status 1 is `result=incomplete`: the pinned ONNX still does not load, so
this is not a three-model pass. The aborted Whisper init still prints
`MTL0 total size = 487.01 MB` and then `invalid n_dims 99`. The context is
null, and the Metal residency set is empty at process exit. That is buffer
release inside the probe. It is not M0-AC3. `taco` does not link this engine,
and `t_m0_cancel_lifecycle` still panics.

## Static ORT Silero (2026-10-17)

ONNX Runtime v1.31.0, commit `402979ac1bef89dcd55add41eaa7277c407599eb`, was
built as static CPU archives under `.cache/m0-onnx`
(`onnxruntime_BUILD_SHARED_LIB=OFF`, telemetry off, ML ops off, deployment
target 14.2). The official `onnxruntime-osx-arm64-1.31.0.tgz` contains
`libonnxruntime.1.31.0.dylib` and was not linked. No `libonnxruntime.dylib`
sits beside the probe.

`native/CMakeLists.txt` force-loads those archives into `taco_link_probe`,
including `libre2.a` from the same ORT build. The first link failed with
`ld: symbol(s) not found for architecture arm64` on `re2::RE2::FullMatchN`,
`re2::RE2::RE2`, `re2::RE2::~RE2`, and `re2::RE2::Match`, referenced from
`libonnxruntime_providers.a` (`regex_full_match.cc.o`, `tokenizer.cc.o`).
`libre2.a` cleared that. Force-loading both `libonnx.a` and `libonnx_proto.a`
then failed with 695 duplicate `onnx::` symbols (`onnx-ml.pb.cc.o` in both).
`libonnx.a` already contains those objects, so `libonnx_proto.a` is not on the
link line. `libflatbuffers.a` is a normal archive, not `-force_load`, because
it shares FlatBufferBuilder symbols with `libonnxruntime_flatbuffers.a`.

```text
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

No `.dylib`, `.so`, `.metal`, or `.metallib` sits next to the executable.
`nm` shows `_OrtGetApiBase` as a defined text symbol. The binary is a 41 MB
arm64 Mach-O. `taco_vad_ort` keeps the session `TypeInfo` alive before reading
shape; an earlier print used a temporary and reported rank 0 with garbage type
ids. That print was discarded.

Same three pins, same process. `whisper_vad_init_from_file_with_params` still
prints `vad=fail` (`invalid model data (bad magic)`). `link_three` uses
`taco_vad_ort`, not that return. `whisper_full` stays `vad` false. Element type
1 is float and 7 is int64 (`ONNXTensorElementDataType`). Dynamic dimensions
print as `-1`. `sr` rank 0 is a scalar.

```text
vad=fail
vad_ort_session=ok version=1.31.0 inputs=3 outputs=2 ep=CPU
vad_ort_in name=input type=1 rank=2 dims=-1,-1
vad_ort_in name=state type=1 rank=3 dims=2,-1,128
vad_ort_in name=sr type=7 rank=0 dims=
vad_ort_out name=output type=1 rank=2 dims=-1,1
vad_ort_out name=stateN type=1 rank=3 dims=-1,-1,-1
vad_ort_frame=0 samples=512 sr=16000 prob=0.000592 state_l2=8.108584
vad_ort_frame=1 samples=512 sr=16000 prob=0.000528 state_l2=11.574153
vad_ort_frame=2 samples=512 sr=16000 prob=0.000592 state_l2=8.108584
vad_ort_reset=1 match=1 carried_state=1 window=512 state=2x1x128
vad_ort=ok device=cpu
gpu_offload=1
gpu_device=MTL0
asr=ok use_gpu=1 segments=1 text_bytes=21
whisper_cancel_decode=aborted rc=-8 hits=2 join_ms=178 reserve_ms=2000 within_reserve=1
whisper_cancel_init=aborted join_ms=111 poisoned=1 reserve_ms=2000 within_reserve=1
cancel_init=aborted join_ms=240 progress_milli=0 reserve_ms=2000 within_reserve=1
mt_load=0 n_gpu_layers=-1 offloaded=33/33
mt=ok text_bytes=34
cancel_decode=aborted rc=2 steps=2 join_ms=8 reserve_ms=2000 within_reserve=1
link_three=1 cancel_lifecycle=1 ac2=0 ac3=0
result=ran
teardown=returned
```

Exit status 0. Frame 0 is 512 zeros with state `[2,1,128]` zeros. Frame 1 is a
440 Hz tone at amplitude 0.2 and copies `stateN` forward (`state_l2` moved
from 8.108584 to 11.574153). Frame 2 zeros audio and state again. Probability
and `state_l2` matched frame 0 within 1e-5 and 1e-4. One pass. The execution
provider is CPU. ASR and MT on this run are the same MTL0 results as the
pinned-load section (`use gpu = 1`, 33/33, text sizes 21 and 34). Stderr still
has the whisper bad-magic line, `invalid n_dims 99`, and `llama_decode` ret 2.
No `GGML_ASSERT`.

`link_three=1` is that probe predicate. It is not `T-M0-LINK-THREE-MODELS`,
and it is not three models on Metal. `ac2=0` and `ac3=0` are what the probe
prints. `cargo build` of `taco` does not link these engines or ORT.

## What this does not accept

- [ ] **M0-AC2:** The probe ran all three pins in one process. Silero was CPU ORT, not Metal. ASR and translation ran in the probe, not in `taco`.
- [ ] **M0-AC3:** Joins fit in 2 s, and the patched init failure still releases the MTL0 buffer (`teardown=returned`, exit 0, no residency-set assert). `taco` does not. The Rust test did not pass.
- The `taco` binary from `cargo build` does not link these engines or ORT. `cargo test` does not build this probe.
- `T-M0-LINK-THREE-MODELS` and `T-M0-CANCEL-LIFECYCLE` stay fail-closed.
- A helper process, a sidecar dylib, and `-ngl 0` were not used, and they remain rejected.
